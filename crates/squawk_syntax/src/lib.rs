// via https://github.com/rust-lang/rust-analyzer/blob/d8887c0758bbd2d5f752d5bd405d4491e90e7ed6/crates/syntax/src/lib.rs
//
// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:
//
// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

pub mod ast;
mod parsing;
pub mod syntax_error;
mod syntax_node;
mod token_text;
mod validation;

#[cfg(test)]
mod test;

use std::{marker::PhantomData, sync::Arc};

pub use squawk_parser::SyntaxKind;

use ast::AstNode;
use rowan::GreenNode;
use syntax_error::SyntaxError;
pub use syntax_node::{SyntaxNode, SyntaxToken};
pub use token_text::TokenText;

/// `Parse` is the result of the parsing: a syntax tree and a collection of
/// errors.
///
/// Note that we always produce a syntax tree, even for completely invalid
/// files.
#[derive(Debug, PartialEq, Eq)]
pub struct Parse<T> {
    green: GreenNode,
    errors: Option<Arc<[SyntaxError]>>,
    _ty: PhantomData<fn() -> T>,
}

impl<T> Clone for Parse<T> {
    fn clone(&self) -> Parse<T> {
        Parse {
            green: self.green.clone(),
            errors: self.errors.clone(),
            _ty: PhantomData,
        }
    }
}

impl<T> Parse<T> {
    fn new(green: GreenNode, errors: Vec<SyntaxError>) -> Parse<T> {
        Parse {
            green,
            errors: if errors.is_empty() {
                None
            } else {
                Some(errors.into())
            },
            _ty: PhantomData,
        }
    }

    pub fn syntax_node(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.green.clone())
    }

    pub fn errors(&self) -> Vec<SyntaxError> {
        let mut errors = if let Some(e) = self.errors.as_deref() {
            e.to_vec()
        } else {
            vec![]
        };
        validation::validate(&self.syntax_node(), &mut errors);
        errors
    }
}

impl<T: AstNode> Parse<T> {
    /// Converts this parse result into a parse result for an untyped syntax tree.
    pub fn to_syntax(self) -> Parse<SyntaxNode> {
        Parse {
            green: self.green,
            errors: self.errors,
            _ty: PhantomData,
        }
    }

    /// Gets the parsed syntax tree as a typed ast node.
    ///
    /// # Panics
    ///
    /// Panics if the root node cannot be casted into the typed ast node
    /// (e.g. if it's an `ERROR` node).
    pub fn tree(&self) -> T {
        T::cast(self.syntax_node()).unwrap()
    }

    /// Converts from `Parse<T>` to [`Result<T, Vec<SyntaxError>>`].
    pub fn ok(self) -> Result<T, Vec<SyntaxError>> {
        match self.errors() {
            errors if !errors.is_empty() => Err(errors),
            _ => Ok(self.tree()),
        }
    }
}

impl Parse<SyntaxNode> {
    pub fn cast<N: AstNode>(self) -> Option<Parse<N>> {
        if N::cast(self.syntax_node()).is_some() {
            Some(Parse {
                green: self.green,
                errors: self.errors,
                _ty: PhantomData,
            })
        } else {
            None
        }
    }
}

/// `SourceFile` represents a parse tree for a single SQL file.
pub use crate::ast::SourceFile;

impl SourceFile {
    pub fn parse(text: &str) -> Parse<SourceFile> {
        let (green, errors) = parsing::parse_text(text);
        let root = SyntaxNode::new_root(green.clone());

        assert_eq!(root.kind(), SyntaxKind::SOURCE_FILE);
        Parse::new(green, errors)
    }
}

/// Matches a `SyntaxNode` against an `ast` type.
///
/// # Example:
///
/// ```ignore
/// match_ast! {
///     match node {
///         ast::CallExpr(it) => { ... },
///         ast::MethodCallExpr(it) => { ... },
///         ast::MacroCall(it) => { ... },
///         _ => None,
///     }
/// }
/// ```
#[macro_export]
macro_rules! match_ast {
    (match $node:ident { $($tt:tt)* }) => { $crate::match_ast!(match ($node) { $($tt)* }) };

    (match ($node:expr) {
        $( $( $path:ident )::+ ($it:pat) => $res:expr, )*
        _ => $catch_all:expr $(,)?
    }) => {{
        $( if let Some($it) = $($path::)+cast($node.clone()) { $res } else )*
        { $catch_all }
    }};
}

/// This test does not assert anything and instead just shows off the crate's
/// API.
#[test]
fn api_walkthrough() {
    use ast::{HasModuleItem, SourceFile};
    use rowan::{Direction, NodeOrToken, SyntaxText, TextRange, WalkEvent};
    use std::fmt::Write;

    let source_code = "
        create function foo(p int8)
        returns int
        as 'select 1 + 1'
        language sql;
    ";
    // `SourceFile` is the main entry point.
    //
    // The `parse` method returns a `Parse` -- a pair of syntax tree and a list
    // of errors. That is, syntax tree is constructed even in presence of errors.
    let parse = SourceFile::parse(source_code);
    assert!(parse.errors().is_empty());

    // The `tree` method returns an owned syntax node of type `SourceFile`.
    // Owned nodes are cheap: inside, they are `Rc` handles to the underling data.
    let file: SourceFile = parse.tree();

    // `SourceFile` is the root of the syntax tree. We can iterate file's items.
    // Let's fetch the `foo` function.
    let mut func = None;
    for item in file.items() {
        match item {
            ast::Item::CreateFunc(f) => func = Some(f),
            _ => unreachable!(),
        }
    }
    let func: ast::CreateFunc = func.unwrap();

    // Each AST node has a bunch of getters for children. All getters return
    // `Option`s though, to account for incomplete code. Some getters are common
    // for several kinds of node. In this case, a trait like `ast::NameOwner`
    // usually exists. By convention, all ast types should be used with `ast::`
    // qualifier.
    let path: Option<ast::Path> = func.path();
    let name: ast::Name = path.unwrap().segment().unwrap().name().unwrap();
    assert_eq!(name.text(), "foo");

    // return
    let ret_type: Option<ast::RetType> = func.ret_type();
    let r_ty = &ret_type.unwrap().ty().unwrap();
    let type_: &ast::PathType = match &r_ty {
        ast::Type::PathType(r) => r,
        _ => unreachable!(),
    };
    let type_path: ast::Path = type_.path().unwrap();
    assert_eq!(type_path.syntax().to_string(), "int");

    // params
    let param_list: ast::ParamList = func.param_list().unwrap();
    let param: ast::Param = param_list.params().next().unwrap();

    let param_name: ast::Name = param.name().unwrap();
    assert_eq!(param_name.syntax().to_string(), "p");

    let param_ty: ast::Type = param.ty().unwrap();
    assert_eq!(param_ty.syntax().to_string(), "int8");

    let func_option_list: ast::FuncOptionList = func.option_list().unwrap();

    // Enums are used to group related ast nodes together, and can be used for
    // matching. However, because there are no public fields, it's possible to
    // match only the top level enum: that is the price we pay for increased API
    // flexibility
    let func_option = func_option_list.options().next().unwrap();
    let option: &ast::AsFuncOption = match &func_option {
        ast::FuncOption::AsFuncOption(o) => o,
        _ => unreachable!(),
    };
    let string_text: ast::Literal = option.strings().next().unwrap();
    assert_eq!(string_text.syntax().to_string(), "'select 1 + 1'");

    // Besides the "typed" AST API, there's an untyped CST one as well.
    // To switch from AST to CST, call `.syntax()` method:
    let func_option_syntax: &SyntaxNode = func_option.syntax();

    // Note how `expr` and `bin_expr` are in fact the same node underneath:
    assert!(func_option_syntax == option.syntax());

    // To go from CST to AST, `AstNode::cast` function is used:
    let _expr: ast::FuncOption = match ast::FuncOption::cast(func_option_syntax.clone()) {
        Some(e) => e,
        None => unreachable!(),
    };

    // The two properties each syntax node has is a `SyntaxKind`:
    assert_eq!(func_option_syntax.kind(), SyntaxKind::AS_FUNC_OPTION);

    // And text range:
    assert_eq!(
        func_option_syntax.text_range(),
        TextRange::new(65.into(), 82.into())
    );

    // You can get node's text as a `SyntaxText` object, which will traverse the
    // tree collecting token's text:
    let text: SyntaxText = func_option_syntax.text();
    assert_eq!(text.to_string(), "as 'select 1 + 1'");

    // There's a bunch of traversal methods on `SyntaxNode`:
    assert_eq!(
        func_option_syntax.parent().as_ref(),
        Some(func_option_list.syntax())
    );
    assert_eq!(
        param_list
            .syntax()
            .first_child_or_token()
            .map(|it| it.kind()),
        Some(SyntaxKind::L_PAREN)
    );
    assert_eq!(
        func_option_syntax
            .next_sibling_or_token()
            .map(|it| it.kind()),
        Some(SyntaxKind::WHITESPACE)
    );

    // As well as some iterator helpers:
    let f = func_option_syntax
        .ancestors()
        .find_map(ast::CreateFunc::cast);
    assert_eq!(f, Some(func));
    assert!(param
        .syntax()
        .siblings_with_tokens(Direction::Next)
        .any(|it| it.kind() == SyntaxKind::R_PAREN));
    assert_eq!(
        func_option_syntax.descendants_with_tokens().count(),
        5, // 5 tokens `1`, ` `, `+`, ` `, `1`
           // 2 child literal expressions: `1`, `1`
           // 1 the node itself: `1 + 1`
    );

    // There's also a `preorder` method with a more fine-grained iteration control:
    let mut buf = String::new();
    let mut indent = 0;
    for event in func_option_syntax.preorder_with_tokens() {
        match event {
            WalkEvent::Enter(node) => {
                let text = match &node {
                    NodeOrToken::Node(it) => it.text().to_string(),
                    NodeOrToken::Token(it) => it.text().to_owned(),
                };
                buf.write_fmt(format_args!(
                    "{:indent$}{:?} {:?}\n",
                    " ",
                    text,
                    node.kind(),
                    indent = indent
                ))
                .unwrap();
                indent += 2;
            }
            WalkEvent::Leave(_) => indent -= 2,
        }
    }
    assert_eq!(indent, 0);
    assert_eq!(
        buf.trim(),
        r#"
"as 'select 1 + 1'" AS_FUNC_OPTION
  "as" AS_KW
  " " WHITESPACE
  "'select 1 + 1'" LITERAL
    "'select 1 + 1'" STRING
    "#
        .trim()
    );

    // To recursively process the tree, there are three approaches:
    // 1. explicitly call getter methods on AST nodes.
    // 2. use descendants and `AstNode::cast`.
    // 3. use descendants and `match_ast!`.
    //
    // Here's how the first one looks like:
    let exprs_cast: Vec<String> = file
        .syntax()
        .descendants()
        .filter_map(ast::FuncOption::cast)
        .map(|expr| expr.syntax().text().to_string())
        .collect();

    // An alternative is to use a macro.
    let mut exprs_visit = Vec::new();
    for node in file.syntax().descendants() {
        match_ast! {
            match node {
                ast::FuncOption(it) => {
                    let res = it.syntax().text().to_string();
                    exprs_visit.push(res);
                },
                _ => (),
            }
        }
    }
    assert_eq!(exprs_cast, exprs_visit);
}
