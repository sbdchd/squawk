use crate::binder;
use crate::binder::Binder;
use crate::resolve;
use crate::symbols::Name;
use rowan::{TextRange, TextSize};
use squawk_syntax::{
    SyntaxNode,
    ast::{self, AstNode},
};

/// `VSCode` has some theming options based on these types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InlayHintKind {
    Type,
    Parameter,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InlayHint {
    pub position: TextSize,
    pub label: String,
    pub kind: InlayHintKind,
    pub target: Option<TextRange>,
}

pub fn inlay_hints(file: &ast::SourceFile) -> Vec<InlayHint> {
    let mut hints = vec![];
    let binder = binder::bind(file);
    let root = file.syntax();

    for node in root.descendants() {
        if let Some(call_expr) = ast::CallExpr::cast(node.clone()) {
            inlay_hint_call_expr(&mut hints, root, &binder, call_expr);
        } else if let Some(insert) = ast::Insert::cast(node) {
            inlay_hint_insert(&mut hints, root, &binder, insert);
        }
    }

    hints
}

fn inlay_hint_call_expr(
    hints: &mut Vec<InlayHint>,
    root: &SyntaxNode,
    binder: &Binder,
    call_expr: ast::CallExpr,
) -> Option<()> {
    let arg_list = call_expr.arg_list()?;
    let expr = call_expr.expr()?;

    let name_ref = if let Some(name_ref) = ast::NameRef::cast(expr.syntax().clone()) {
        name_ref
    } else {
        ast::FieldExpr::cast(expr.syntax().clone())?.field()?
    };

    let function_ptr = resolve::resolve_name_ref_ptrs(binder, root, &name_ref)?
        .into_iter()
        .next()?;

    let function_name_node = function_ptr.to_node(root);

    if let Some(create_function) = function_name_node
        .ancestors()
        .find_map(ast::CreateFunction::cast)
        && let Some(param_list) = create_function.param_list()
    {
        for (param, arg) in param_list.params().zip(arg_list.args()) {
            if let Some(param_name) = param.name() {
                let arg_start = arg.syntax().text_range().start();
                let target = Some(param_name.syntax().text_range());
                hints.push(InlayHint {
                    position: arg_start,
                    label: format!("{}: ", param_name.syntax().text()),
                    kind: InlayHintKind::Parameter,
                    target,
                });
            }
        }
    };

    Some(())
}

fn inlay_hint_insert(
    hints: &mut Vec<InlayHint>,
    root: &SyntaxNode,
    binder: &Binder,
    insert: ast::Insert,
) -> Option<()> {
    let create_table = resolve::resolve_insert_create_table(root, binder, &insert);

    let columns = if let Some(column_list) = insert.column_list() {
        // `insert into t(a, b, c) values (1, 2, 3)`
        column_list
            .columns()
            .filter_map(|col| {
                let col_name = resolve::extract_column_name(&col)?;
                let target = create_table
                    .as_ref()
                    .and_then(|x| resolve::find_column_in_create_table(binder, root, x, &col_name))
                    .map(|x| x.text_range());
                Some((col_name, target))
            })
            .collect()
    } else {
        // `insert into t values (1, 2, 3)`
        create_table?
            .table_arg_list()?
            .args()
            .filter_map(|arg| {
                if let ast::TableArg::Column(column) = arg
                    && let Some(name) = column.name()
                {
                    let col_name = Name::from_node(&name);
                    let target = Some(name.syntax().text_range());
                    Some((col_name, target))
                } else {
                    None
                }
            })
            .collect()
    };

    let Some(values) = insert.values() else {
        return inlay_hint_insert_select(hints, columns, insert.stmt()?);
    };
    let row_list = values.row_list()?;

    for row in row_list.rows() {
        for ((column_name, target), expr) in columns.iter().zip(row.exprs()) {
            let expr_start = expr.syntax().text_range().start();
            hints.push(InlayHint {
                position: expr_start,
                label: format!("{}: ", column_name),
                kind: InlayHintKind::Parameter,
                target: *target,
            });
        }
    }

    Some(())
}

fn inlay_hint_insert_select(
    hints: &mut Vec<InlayHint>,
    columns: Vec<(Name, Option<TextRange>)>,
    stmt: ast::Stmt,
) -> Option<()> {
    let target_list = match stmt {
        ast::Stmt::Select(select) => select.select_clause()?.target_list(),
        ast::Stmt::SelectInto(select_into) => select_into.select_clause()?.target_list(),
        ast::Stmt::ParenSelect(paren_select) => {
            target_list_from_select_variant(paren_select.select()?)
        }
        _ => None,
    }?;

    for ((column_name, target), target_expr) in columns.iter().zip(target_list.targets()) {
        let expr = target_expr.expr()?;
        let expr_start = expr.syntax().text_range().start();
        hints.push(InlayHint {
            position: expr_start,
            label: format!("{}: ", column_name),
            kind: InlayHintKind::Parameter,
            target: *target,
        });
    }

    Some(())
}

fn target_list_from_select_variant(select: ast::SelectVariant) -> Option<ast::TargetList> {
    let mut current = select;
    for _ in 0..100 {
        match current {
            ast::SelectVariant::Select(select) => {
                return select.select_clause()?.target_list();
            }
            ast::SelectVariant::SelectInto(select_into) => {
                return select_into.select_clause()?.target_list();
            }
            ast::SelectVariant::ParenSelect(paren_select) => {
                current = paren_select.select()?;
            }
            _ => return None,
        }
    }
    None
}

#[cfg(test)]
mod test {
    use crate::inlay_hints::inlay_hints;
    use annotate_snippets::{AnnotationKind, Level, Renderer, Snippet, renderer::DecorStyle};
    use insta::assert_snapshot;
    use squawk_syntax::ast;

    #[track_caller]
    fn check_inlay_hints(sql: &str) -> String {
        let parse = ast::SourceFile::parse(sql);
        assert_eq!(parse.errors(), vec![]);
        let file: ast::SourceFile = parse.tree();

        let hints = inlay_hints(&file);

        if hints.is_empty() {
            return String::new();
        }

        let mut modified_sql = sql.to_string();
        let mut insertions: Vec<(usize, String)> = hints
            .iter()
            .map(|hint| {
                let offset: usize = hint.position.into();
                (offset, hint.label.clone())
            })
            .collect();

        insertions.sort_by(|a, b| b.0.cmp(&a.0));

        for (offset, label) in &insertions {
            modified_sql.insert_str(*offset, label);
        }

        let mut annotations = vec![];
        let mut cumulative_offset = 0;

        insertions.reverse();
        for (original_offset, label) in insertions {
            let new_offset = original_offset + cumulative_offset;
            annotations.push((new_offset, label.len()));
            cumulative_offset += label.len();
        }

        let mut snippet = Snippet::source(&modified_sql).fold(true);

        for (offset, len) in annotations {
            snippet = snippet.annotation(AnnotationKind::Context.span(offset..offset + len));
        }

        let group = Level::INFO.primary_title("inlay hints").element(snippet);

        let renderer = Renderer::plain().decor_style(DecorStyle::Unicode);
        renderer
            .render(&[group])
            .to_string()
            .replace("info: inlay hints", "inlay hints:")
    }

    #[test]
    fn single_param() {
        assert_snapshot!(check_inlay_hints("
create function foo(a int) returns int as 'select $$1' language sql;
select foo(1);
"), @r"
        inlay hints:
          ╭▸ 
        3 │ select foo(a: 1);
          ╰╴           ───
        ");
    }

    #[test]
    fn multiple_params() {
        assert_snapshot!(check_inlay_hints("
create function add(a int, b int) returns int as 'select $$1 + $$2' language sql;
select add(1, 2);
"), @r"
        inlay hints:
          ╭▸ 
        3 │ select add(a: 1, b: 2);
          ╰╴           ───   ───
        ");
    }

    #[test]
    fn no_params() {
        assert_snapshot!(check_inlay_hints("
create function foo() returns int as 'select 1' language sql;
select foo();
"), @"");
    }

    #[test]
    fn with_schema() {
        assert_snapshot!(check_inlay_hints("
create function public.foo(x int) returns int as 'select $$1' language sql;
select public.foo(42);
"), @r"
        inlay hints:
          ╭▸ 
        3 │ select public.foo(x: 42);
          ╰╴                  ───
        ");
    }

    #[test]
    fn with_search_path() {
        assert_snapshot!(check_inlay_hints(r#"
set search_path to myschema;
create function foo(val int) returns int as 'select $$1' language sql;
select foo(100);
"#), @r"
        inlay hints:
          ╭▸ 
        4 │ select foo(val: 100);
          ╰╴           ─────
        ");
    }

    #[test]
    fn multiple_calls() {
        assert_snapshot!(check_inlay_hints("
create function inc(n int) returns int as 'select $$1 + 1' language sql;
select inc(1), inc(2);
"), @r"
        inlay hints:
          ╭▸ 
        3 │ select inc(n: 1), inc(n: 2);
          ╰╴           ───        ───
        ");
    }

    #[test]
    fn more_args_than_params() {
        assert_snapshot!(check_inlay_hints("
create function foo(a int) returns int as 'select $$1' language sql;
select foo(1, 2);
"), @r"
        inlay hints:
          ╭▸ 
        3 │ select foo(a: 1, 2);
          ╰╴           ───
        ");
    }

    #[test]
    fn insert_with_column_list() {
        assert_snapshot!(check_inlay_hints("
create table t (column_a int, column_b int, column_c text);
insert into t (column_a, column_c) values (1, 'foo');
"), @r"
        inlay hints:
          ╭▸ 
        3 │ insert into t (column_a, column_c) values (column_a: 1, column_c: 'foo');
          ╰╴                                           ──────────   ──────────
        ");
    }

    #[test]
    fn insert_without_column_list() {
        assert_snapshot!(check_inlay_hints("
create table t (column_a int, column_b int, column_c text);
insert into t values (1, 2, 'foo');
"), @r"
        inlay hints:
          ╭▸ 
        3 │ insert into t values (column_a: 1, column_b: 2, column_c: 'foo');
          ╰╴                      ──────────   ──────────   ──────────
        ");
    }

    #[test]
    fn insert_multiple_rows() {
        assert_snapshot!(check_inlay_hints("
create table t (x int, y int);
insert into t values (1, 2), (3, 4);
"), @r"
        inlay hints:
          ╭▸ 
        3 │ insert into t values (x: 1, y: 2), (x: 3, y: 4);
          ╰╴                      ───   ───     ───   ───
        ");
    }

    #[test]
    fn insert_no_create_table() {
        assert_snapshot!(check_inlay_hints("
insert into t (a, b) values (1, 2);
"), @r"
        inlay hints:
          ╭▸ 
        2 │ insert into t (a, b) values (a: 1, b: 2);
          ╰╴                             ───   ───
        ");
    }

    #[test]
    fn insert_more_values_than_columns() {
        assert_snapshot!(check_inlay_hints("
create table t (a int, b int);
insert into t values (1, 2, 3);
"), @r"
        inlay hints:
          ╭▸ 
        3 │ insert into t values (a: 1, b: 2, 3);
          ╰╴                      ───   ───
        ");
    }

    #[test]
    fn insert_select() {
        assert_snapshot!(check_inlay_hints("
create table t (a int, b int);
insert into t select 1, 2;
"), @r"
        inlay hints:
          ╭▸ 
        3 │ insert into t select a: 1, b: 2;
          ╰╴                     ───   ───
        ");
    }
}
