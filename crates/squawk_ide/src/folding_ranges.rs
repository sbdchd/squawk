// via https://github.com/rust-lang/rust-analyzer/blob/8d75311400a108d7ffe17dc9c38182c566952e6e/crates/ide/src/folding_ranges.rs#L47
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

// NOTE: pretty much copied as is but simplfied a fair bit. I don't use folding
// much so not sure if this is optimal.

use rustc_hash::FxHashSet;

use rowan::{Direction, NodeOrToken, TextRange};
use salsa::Database as Db;
use squawk_syntax::SyntaxKind;
use squawk_syntax::ast::{self, AstNode, AstToken};

use crate::db::{File, parse};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FoldKind {
    ArgList,
    Array,
    Comment,
    FunctionCall,
    Join,
    List,
    Statement,
    Subquery,
    Tuple,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fold {
    pub range: TextRange,
    pub kind: FoldKind,
}

#[salsa::tracked]
pub fn folding_ranges(db: &dyn Db, file: File) -> Vec<Fold> {
    let parse = parse(db, file);

    let mut folds = vec![];
    let mut visited_comments = FxHashSet::default();

    for element in parse.tree().syntax().descendants_with_tokens() {
        match &element {
            NodeOrToken::Token(token) => {
                if let Some(comment) = ast::Comment::cast(token.clone())
                    && !visited_comments.contains(&comment)
                    && let Some(range) =
                        contiguous_range_for_comment(comment, &mut visited_comments)
                {
                    folds.push(Fold {
                        range,
                        kind: FoldKind::Comment,
                    });
                }
            }
            NodeOrToken::Node(node) => {
                if let Some(kind) = fold_kind(node.kind()) {
                    if !node.text().contains_char('\n') {
                        continue;
                    }
                    // skip any leading whitespace / comments
                    let start = node
                        .children_with_tokens()
                        .find(|e| match e {
                            NodeOrToken::Token(t) => {
                                let kind = t.kind();
                                kind != SyntaxKind::COMMENT && kind != SyntaxKind::WHITESPACE
                            }
                            NodeOrToken::Node(_) => true,
                        })
                        .map(|e| e.text_range().start())
                        .unwrap_or_else(|| node.text_range().start());
                    folds.push(Fold {
                        range: TextRange::new(start, node.text_range().end()),
                        kind,
                    });
                }
            }
        }
    }

    folds
}

fn fold_kind(kind: SyntaxKind) -> Option<FoldKind> {
    if ast::Stmt::can_cast(kind) {
        return Some(FoldKind::Statement);
    }

    match kind {
        SyntaxKind::ARG_LIST | SyntaxKind::TABLE_ARG_LIST | SyntaxKind::PARAM_LIST => {
            Some(FoldKind::ArgList)
        }
        SyntaxKind::ARRAY_EXPR => Some(FoldKind::Array),
        SyntaxKind::CALL_EXPR => Some(FoldKind::FunctionCall),
        SyntaxKind::JOIN => Some(FoldKind::Join),
        SyntaxKind::PAREN_SELECT => Some(FoldKind::Subquery),
        SyntaxKind::TUPLE_EXPR => Some(FoldKind::Tuple),
        SyntaxKind::WHEN_CLAUSE_LIST
        | SyntaxKind::ALTER_OPTION_LIST
        | SyntaxKind::ATTRIBUTE_LIST
        | SyntaxKind::BEGIN_FUNC_OPTION_LIST
        | SyntaxKind::COLUMN_LIST
        | SyntaxKind::CONFLICT_INDEX_ITEM_LIST
        | SyntaxKind::CONSTRAINT_EXCLUSION_LIST
        | SyntaxKind::COPY_OPTION_LIST
        | SyntaxKind::CREATE_DATABASE_OPTION_LIST
        | SyntaxKind::DROP_OP_CLASS_OPTION_LIST
        | SyntaxKind::FDW_OPTION_LIST
        | SyntaxKind::FUNCTION_SIG_LIST
        | SyntaxKind::FUNC_OPTION_LIST
        | SyntaxKind::GROUP_BY_LIST
        | SyntaxKind::JSON_TABLE_COLUMN_LIST
        | SyntaxKind::OPERATOR_CLASS_OPTION_LIST
        | SyntaxKind::OPTION_ITEM_LIST
        | SyntaxKind::OP_SIG_LIST
        | SyntaxKind::PARTITION_ITEM_LIST
        | SyntaxKind::PARTITION_LIST
        | SyntaxKind::RETURNING_OPTION_LIST
        | SyntaxKind::REVOKE_COMMAND_LIST
        | SyntaxKind::ROLE_OPTION_LIST
        | SyntaxKind::ROLE_REF_LIST
        | SyntaxKind::ROW_LIST
        | SyntaxKind::SEQUENCE_OPTION_LIST
        | SyntaxKind::SET_COLUMN_LIST
        | SyntaxKind::SET_EXPR_LIST
        | SyntaxKind::SET_OPTIONS_LIST
        | SyntaxKind::SORT_BY_LIST
        | SyntaxKind::TABLE_AND_COLUMNS_LIST
        | SyntaxKind::TABLE_LIST
        | SyntaxKind::TARGET_LIST
        | SyntaxKind::TRANSACTION_MODE_LIST
        | SyntaxKind::TRIGGER_EVENT_LIST
        | SyntaxKind::VACUUM_OPTION_LIST
        | SyntaxKind::VARIANT_LIST
        | SyntaxKind::XML_ATTRIBUTE_LIST
        | SyntaxKind::XML_COLUMN_OPTION_LIST
        | SyntaxKind::XML_NAMESPACE_LIST
        | SyntaxKind::XML_TABLE_COLUMN_LIST => Some(FoldKind::List),
        _ => None,
    }
}

fn contiguous_range_for_comment(
    first: ast::Comment,
    visited: &mut FxHashSet<ast::Comment>,
) -> Option<TextRange> {
    visited.insert(first.clone());

    // Only fold comments of the same flavor
    let group_kind = first.kind();
    if !group_kind.is_line() {
        return None;
    }

    let mut last = first.clone();
    for element in first.syntax().siblings_with_tokens(Direction::Next) {
        match element {
            NodeOrToken::Token(token) => {
                if let Some(ws) = ast::Whitespace::cast(token.clone())
                    && !ws.spans_multiple_lines()
                {
                    // Ignore whitespace without blank lines
                    continue;
                }
                if let Some(c) = ast::Comment::cast(token) {
                    visited.insert(c.clone());
                    last = c;
                    continue;
                }
                // The comment group ends because either:
                // * An element of a different kind was reached
                // * A comment of a different flavor was reached
                break;
            }
            NodeOrToken::Node(_) => break,
        }
    }

    if first != last {
        Some(TextRange::new(
            first.syntax().text_range().start(),
            last.syntax().text_range().end(),
        ))
    } else {
        // The group consists of only one element, therefore it cannot be folded
        None
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;

    use crate::db::{Database, File};

    use super::*;

    fn fold_kind_str(kind: &FoldKind) -> &'static str {
        match kind {
            FoldKind::ArgList => "arglist",
            FoldKind::Array => "array",
            FoldKind::Comment => "comment",
            FoldKind::FunctionCall => "function_call",
            FoldKind::Join => "join",
            FoldKind::List => "list",
            FoldKind::Statement => "statement",
            FoldKind::Subquery => "subquery",
            FoldKind::Tuple => "tuple",
        }
    }

    fn check(sql: &str) -> String {
        let db = Database::default();
        let file = File::new(&db, sql.to_string().into());
        let folds = folding_ranges(&db, file);

        if folds.is_empty() {
            return sql.to_string();
        }

        #[derive(PartialEq, Eq, PartialOrd, Ord)]
        struct Event<'a> {
            offset: usize,
            is_end: bool,
            kind: &'a str,
        }

        let mut events: Vec<Event<'_>> = vec![];
        for fold in &folds {
            let start: usize = fold.range.start().into();
            let end: usize = fold.range.end().into();
            let kind = fold_kind_str(&fold.kind);
            events.push(Event {
                offset: start,
                is_end: false,
                kind,
            });
            events.push(Event {
                offset: end,
                is_end: true,
                kind,
            });
        }
        events.sort();

        let mut output = String::new();
        let mut pos = 0usize;
        for event in &events {
            if event.offset > pos {
                output.push_str(&sql[pos..event.offset]);
                pos = event.offset;
            }
            if event.is_end {
                output.push_str("</fold>");
            } else {
                output.push_str(&format!("<fold {}>", event.kind));
            }
        }
        if pos < sql.len() {
            output.push_str(&sql[pos..]);
        }
        output
    }

    #[test]
    fn fold_create_table() {
        assert_snapshot!(check("
create table t (
  id int,
  name text
);"), @"
        <fold statement>create table t <fold arglist>(
          id int,
          name text
        )</fold></fold>;
        ");
    }

    #[test]
    fn fold_select() {
        assert_snapshot!(check("
select
  id,
  name
from t;"), @"
        <fold statement>select
          <fold list>id,
          name</fold>
        from t</fold>;
        ");
    }

    #[test]
    fn do_not_fold_single_line_comment() {
        assert_snapshot!(check("
-- a comment
select 1;"), @"
        -- a comment
        select 1;
        ");
    }

    #[test]
    fn fold_comments_does_not_apply_when_diff_comment_types() {
        assert_snapshot!(check("
/* first part */
-- second part
select 1;"), @"
        /* first part */
        -- second part
        select 1;
        ");
    }

    #[test]
    fn fold_comments_and_multi_statements() {
        assert_snapshot!(check("
-- this is

-- a comment
-- with some more
select a, b, 3
  from t
  where c > 10;"), @"
        -- this is

        <fold comment>-- a comment
        -- with some more</fold>
        <fold statement>select a, b, 3
          from t
          where c > 10</fold>;
        ");
    }

    #[test]
    fn fold_comments_does_not_apply_when_whitespace_between() {
        assert_snapshot!(check("
-- this is

-- a comment
-- with some more
select 1;"), @"
        -- this is

        <fold comment>-- a comment
        -- with some more</fold>
        select 1;
        ");
    }

    #[test]
    fn fold_multiline_comments() {
        assert_snapshot!(check("
-- this is
-- a comment
select 1;"), @"
        <fold comment>-- this is
        -- a comment</fold>
        select 1;
        ");
    }

    #[test]
    fn fold_single_line_no_fold() {
        assert_snapshot!(check("select 1;"), @"select 1;");
    }

    #[test]
    fn fold_subquery() {
        assert_snapshot!(check("
select * from (
  select id from t
);"), @"
        <fold statement>select * from <fold statement>(
          select id from t
        )</fold></fold>;
        ");
    }

    #[test]
    fn fold_case_when() {
        assert_snapshot!(check("
select
  case
    when x = 1 then 'a'
    when x = 2 then 'b'
  end
from t;"), @"
        <fold statement>select
          <fold list>case
            <fold list>when x = 1 then 'a'
            when x = 2 then 'b'</fold>
          end</fold>
        from t</fold>;
        ");
    }

    #[test]
    fn fold_join() {
        assert_snapshot!(check("
select *
from a
join b
  on a.id = b.id;"), @"
        <fold statement>select *
        from a
        <fold join>join b
          on a.id = b.id</fold></fold>;
        ");
    }

    #[test]
    fn fold_array_literal() {
        assert_snapshot!(check("
select * from t where
  x = any(array[
    1,
    2,
    3
  ]);"), @"
        <fold statement>select * from t where
          x = <fold function_call>any(<fold array>array[
            1,
            2,
            3
          ]</fold>)</fold></fold>;
        ");
    }

    #[test]
    fn fold_tuple_literal() {
        assert_snapshot!(check("
select (
  1,
  2,
  3
);"), @"
        <fold statement>select <fold list><fold tuple>(
          1,
          2,
          3
        )</fold></fold></fold>;
        ");
    }

    #[test]
    fn fold_tuple_bin_expr() {
        assert_snapshot!(check("
select * from x
  where z in (
    1,
    2,
    3,
    4,
    5
  );
"), @"
        <fold statement>select * from x
          where z in <fold tuple>(
            1,
            2,
            3,
            4,
            5
          )</fold></fold>;
        ");
    }

    #[test]
    fn fold_function_call() {
        assert_snapshot!(check("
select coalesce(
  a,
  b,
  c
);"), @"
        <fold statement>select <fold function_call><fold list>coalesce<fold arglist>(
          a,
          b,
          c
        )</fold></fold></fold></fold>;
        ");
    }

    #[test]
    fn fold_create_enum() {
        assert_snapshot!(check("
create type status as enum (
  'active',
  'inactive'
);"), @"
        <fold statement>create type status as enum <fold list>(
          'active',
          'inactive'
        )</fold></fold>;
        ");
    }

    #[test]
    fn fold_insert_values() {
        assert_snapshot!(check("
insert into t (id, name)
values
  (1, 'a'),
  (2, 'b');"), @"
        <fold statement>insert into t (id, name)
        <fold statement>values
          <fold list>(1, 'a'),
          (2, 'b')</fold></fold></fold>;
        ");
    }

    #[test]
    fn no_fold_single_line_create_table() {
        assert_snapshot!(check("create table t (id int);"), @"create table t (id int);");
    }

    #[test]
    fn list_variants() {
        let unhandled_list_kinds: Vec<SyntaxKind> = (0..SyntaxKind::__LAST as u16)
            .map(SyntaxKind::from)
            .filter(|kind| format!("{:?}", kind).ends_with("_LIST"))
            .filter(|kind| fold_kind(*kind).is_none())
            .collect();

        assert_eq!(
            unhandled_list_kinds,
            vec![],
            "All _LIST SyntaxKind variants should be handled in fold_kind"
        );
    }
}
