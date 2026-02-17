use crate::builtins::BUILTINS_SQL;
use crate::goto_definition::FileId;
use crate::resolve;
use crate::symbols::Name;
use crate::{binder, goto_definition};
use rowan::{TextRange, TextSize};
use squawk_syntax::ast::{self, AstNode};

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
    // Need this to be an Option because we can still inlay hints when we don't
    // have the destination.
    // For example: `insert into t(a, b) values (1, 2)`
    pub target: Option<TextRange>,
    // TODO: combine with the target range above
    pub file: Option<FileId>,
}

pub fn inlay_hints(file: &ast::SourceFile) -> Vec<InlayHint> {
    let mut hints = vec![];
    for node in file.syntax().descendants() {
        if let Some(call_expr) = ast::CallExpr::cast(node.clone()) {
            inlay_hint_call_expr(&mut hints, file, call_expr);
        } else if let Some(insert) = ast::Insert::cast(node) {
            inlay_hint_insert(&mut hints, file, insert);
        }
    }
    hints
}

fn inlay_hint_call_expr(
    hints: &mut Vec<InlayHint>,
    file: &ast::SourceFile,
    call_expr: ast::CallExpr,
) -> Option<()> {
    let arg_list = call_expr.arg_list()?;
    let expr = call_expr.expr()?;

    let name_ref = if let Some(name_ref) = ast::NameRef::cast(expr.syntax().clone()) {
        name_ref
    } else {
        ast::FieldExpr::cast(expr.syntax().clone())?.field()?
    };

    let location = goto_definition::goto_definition(file, name_ref.syntax().text_range().start())
        .into_iter()
        .next()?;

    let file = match location.file {
        goto_definition::FileId::Current => file,
        goto_definition::FileId::Builtins => &ast::SourceFile::parse(BUILTINS_SQL).tree(),
    };

    let function_name_node = file.syntax().covering_element(location.range);

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
                    file: Some(location.file),
                });
            }
        }
    };

    Some(())
}

fn inlay_hint_insert(
    hints: &mut Vec<InlayHint>,
    file: &ast::SourceFile,
    insert: ast::Insert,
) -> Option<()> {
    let name_start = insert
        .path()?
        .segment()?
        .name_ref()?
        .syntax()
        .text_range()
        .start();
    // We need to support the table definition not being found since we can
    // still provide inlay hints when a column list is provided
    let location = goto_definition::goto_definition(file, name_start)
        .into_iter()
        .next();

    let file = match location.as_ref().map(|x| x.file) {
        Some(goto_definition::FileId::Current) | None => file,
        Some(goto_definition::FileId::Builtins) => &ast::SourceFile::parse(BUILTINS_SQL).tree(),
    };

    let create_table = {
        let range = location.as_ref().map(|x| x.range);

        range.and_then(|range| {
            file.syntax()
                .covering_element(range)
                .ancestors()
                .find_map(ast::CreateTableLike::cast)
        })
    };

    let binder = binder::bind(file);

    let columns = if let Some(column_list) = insert.column_list() {
        // `insert into t(a, b, c) values (1, 2, 3)`
        column_list
            .columns()
            .filter_map(|col| {
                let col_name = resolve::extract_column_name(&col)?;
                let target = create_table
                    .as_ref()
                    .and_then(|x| {
                        resolve::find_column_in_create_table(&binder, file.syntax(), x, &col_name)
                    })
                    .map(|x| x.text_range());
                Some((col_name, target, location.as_ref().map(|x| x.file)))
            })
            .collect()
    } else {
        // `insert into t values (1, 2, 3)`
        resolve::collect_columns_from_create_table(&binder, file.syntax(), &create_table?)
            .into_iter()
            .map(|(col_name, ptr)| {
                let target = ptr.map(|p| p.to_node(file.syntax()).text_range());
                (col_name, target, location.as_ref().map(|x| x.file))
            })
            .collect()
    };

    let Some(values) = insert.values() else {
        // `insert into t select 1, 2;`
        return inlay_hint_insert_select(hints, columns, insert.stmt()?);
    };
    // `insert into t values (1, 2);`
    for row in values.row_list()?.rows() {
        for ((column_name, target, file_id), expr) in columns.iter().zip(row.exprs()) {
            let expr_start = expr.syntax().text_range().start();
            hints.push(InlayHint {
                position: expr_start,
                label: format!("{}: ", column_name),
                kind: InlayHintKind::Parameter,
                target: *target,
                file: *file_id,
            });
        }
    }

    Some(())
}

fn inlay_hint_insert_select(
    hints: &mut Vec<InlayHint>,
    columns: Vec<(Name, Option<TextRange>, Option<FileId>)>,
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

    for ((column_name, target, file_id), target_expr) in columns.iter().zip(target_list.targets()) {
        let expr = target_expr.expr()?;
        let expr_start = expr.syntax().text_range().start();
        hints.push(InlayHint {
            position: expr_start,
            label: format!("{}: ", column_name),
            kind: InlayHintKind::Parameter,
            target: *target,
            file: *file_id,
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
    fn builtin_function() {
        assert_snapshot!(check_inlay_hints("
select json_strip_nulls('[1, null]', true);
"), @r"
        inlay hints:
          ╭▸ 
        2 │ select json_strip_nulls(target: '[1, null]', strip_in_arrays: true);
          ╰╴                        ────────             ─────────────────
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
    fn insert_table_inherits_select() {
        assert_snapshot!(check_inlay_hints("
create table t (a int, b int);
create table u (c int) inherits (t);
insert into u select 1, 2, 3;
"), @r"
        inlay hints:
          ╭▸ 
        4 │ insert into u select a: 1, b: 2, c: 3;
          ╰╴                     ───   ───   ───
        ");
    }

    #[test]
    fn insert_table_like_select() {
        assert_snapshot!(check_inlay_hints("
create table x (a int, b int);
create table y (c int, like x);
insert into y select 1, 2, 3;
"), @r"
        inlay hints:
          ╭▸ 
        4 │ insert into y select c: 1, a: 2, b: 3;
          ╰╴                     ───   ───   ───
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
