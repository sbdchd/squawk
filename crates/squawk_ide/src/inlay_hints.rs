use crate::collect;
use crate::db::{File, parse};
use crate::file::InFile;
use crate::goto_definition;
use crate::resolve;
use crate::symbols::Name;
use rowan::{TextRange, TextSize};
use salsa::Database as Db;
use squawk_syntax::ast::{self, AstNode};

/// `VSCode` has some theming options based on these types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InlayHintKind {
    Type,
    Parameter,
}

#[derive(Clone, PartialEq, Eq)]
pub struct InlayHint {
    pub position: TextSize,
    pub label: String,
    pub kind: InlayHintKind,
    // Optional because we can still emit hints without a destination,
    // e.g. `insert into t(a, b) values (1, 2)` with no matching table.
    pub target: Option<InFile<TextRange>>,
}

#[salsa::tracked]
pub fn inlay_hints(db: &dyn Db, file: File) -> Vec<InlayHint> {
    let mut hints = vec![];
    for node in parse(db, file).tree().syntax().descendants() {
        if let Some(call_expr) = ast::CallExpr::cast(node.clone()) {
            inlay_hint_call_expr(db, &mut hints, file, call_expr);
        } else if let Some(insert) = ast::Insert::cast(node) {
            inlay_hint_insert(db, &mut hints, file, insert);
        }
    }
    hints
}

fn inlay_hint_call_expr(
    db: &dyn Db,
    hints: &mut Vec<InlayHint>,
    file_id: File,
    call_expr: ast::CallExpr,
) -> Option<()> {
    let arg_list = call_expr.arg_list()?;
    let expr = call_expr.expr()?;

    let name_ref = if let Some(name_ref) = ast::NameRef::cast(expr.syntax().clone()) {
        name_ref
    } else {
        ast::FieldExpr::cast(expr.syntax().clone())?.field()?
    };

    let location = goto_definition::goto_definition(
        db,
        InFile::new(file_id, name_ref.syntax().text_range().start()),
    )
    .into_iter()
    .next()?;

    let def_file = parse(db, location.file).tree();

    let function_name_node = def_file.syntax().covering_element(location.range);

    if let Some(create_function) = function_name_node
        .ancestors()
        .find_map(ast::CreateFunction::cast)
        && let Some(param_list) = create_function.param_list()
    {
        for (param, arg) in param_list.params().zip(arg_list.args()) {
            if let Some(param_name) = param.name() {
                let arg_start = arg.syntax().text_range().start();
                let target = Some(InFile::new(location.file, param_name.syntax().text_range()));
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
    db: &dyn Db,
    hints: &mut Vec<InlayHint>,
    file_id: File,
    insert: ast::Insert,
) -> Option<()> {
    let name_start = insert
        .path_ref()?
        .segment()?
        .name_ref()?
        .syntax()
        .text_range()
        .start();
    // We need to support the table definition not being found since we can
    // still provide inlay hints when a column list is provided
    let location = goto_definition::goto_definition(db, InFile::new(file_id, name_start))
        .into_iter()
        .next();

    let def_file = location.as_ref().map(|loc| loc.file).unwrap_or(file_id);
    let def_tree = parse(db, def_file).tree();

    let create_table = location.as_ref().and_then(|loc| {
        def_tree
            .syntax()
            .covering_element(loc.range)
            .ancestors()
            .find_map(ast::CreateTableLike::cast)
    });

    let columns: Vec<(Name, Option<InFile<TextRange>>)> =
        if let Some(column_list) = insert.column_ref_list() {
            // `insert into t(a, b, c) values (1, 2, 3)`
            column_list
                .column_refs()
                .filter_map(|col| {
                    let col_name = col.name_ref().map(|x| Name::from_node(&x))?;
                    let target = create_table
                        .as_ref()
                        .and_then(|x| {
                            resolve::find_column_in_create_table(
                                db,
                                InFile::new(def_file, x),
                                &col_name,
                            )
                        })
                        .and_then(|x| x.into_iter().next())
                        .map(|x| InFile::new(x.file, x.range));
                    Some((col_name, target))
                })
                .collect()
        } else {
            // `insert into t values (1, 2, 3)`
            collect::columns_from_create_table(db, def_file, &create_table?)
                .into_iter()
                .map(|(col_name, ptr)| {
                    let target = ptr.map(|ptr| InFile::new(ptr.file_id, ptr.value.text_range()));
                    (col_name, target)
                })
                .collect()
        };

    inlay_hint_insert_select(hints, columns, insert.select_variant()?)
}

fn inlay_hint_insert_select(
    hints: &mut Vec<InlayHint>,
    columns: Vec<(Name, Option<InFile<TextRange>>)>,
    select_variant: ast::SelectVariant,
) -> Option<()> {
    if let ast::SelectVariant::Values(values) = &select_variant {
        // `insert into t values (1, 2);`
        for row in values.row_list()?.rows() {
            for ((column_name, target), expr) in columns.iter().zip(row.exprs()) {
                let expr_start = expr.syntax().text_range().start();
                hints.push(InlayHint {
                    position: expr_start,
                    label: format!("{column_name}: "),
                    kind: InlayHintKind::Parameter,
                    target: *target,
                });
            }
        }
        return Some(());
    }

    // `insert into t select 1, 2;`
    let target_list = select_variant.target_list()?;
    for ((column_name, target), target_expr) in columns.iter().zip(target_list.targets()) {
        let expr = target_expr.expr()?;
        let expr_start = expr.syntax().text_range().start();
        hints.push(InlayHint {
            position: expr_start,
            label: format!("{column_name}: "),
            kind: InlayHintKind::Parameter,
            target: *target,
        });
    }

    Some(())
}

#[cfg(test)]
mod test {
    use crate::builtins::builtins_file;
    use crate::db::{Database, File};
    use crate::inlay_hints::{InlayHint, inlay_hints};
    use annotate_snippets::{AnnotationKind, Level, Renderer, Snippet, renderer::DecorStyle};
    use insta::assert_snapshot;
    use rustc_hash::FxHashMap;
    use std::ops::Range;

    #[must_use]
    #[track_caller]
    fn check_inlay_hints(sql: &str) -> String {
        let db = Database::default();
        let file = File::new(&db, sql.to_string().into());

        assert_eq!(crate::db::parse(&db, file).errors(), vec![]);

        let hints = inlay_hints(&db, file);

        if hints.is_empty() {
            return String::new();
        }

        let mut modified_sql = sql.to_string();
        let mut indexed: Vec<(usize, &InlayHint)> = hints.iter().enumerate().collect();
        indexed.sort_by_key(|(_, h)| h.position);

        let mut label_annotations: Vec<Range<usize>> = vec![0..0; hints.len()];
        let mut cumulative = 0;
        for (i, hint) in &indexed {
            let pos: usize = hint.position.into();
            let new_pos = pos + cumulative;
            modified_sql.insert_str(new_pos, &hint.label);
            label_annotations[*i] = new_pos..new_pos + hint.label.len();
            cumulative += hint.label.len();
        }

        let mut targets_by_file: FxHashMap<File, Vec<(usize, Range<usize>)>> = FxHashMap::default();
        for (i, hint) in hints.iter().enumerate() {
            if let Some(target) = &hint.target {
                let start: usize = target.value.start().into();
                let end: usize = target.value.end().into();
                targets_by_file
                    .entry(target.file_id)
                    .or_default()
                    .push((i + 1, start..end));
            }
        }

        let mut file_paths: FxHashMap<File, &'static str> = FxHashMap::default();
        file_paths.insert(file, "current.sql");
        file_paths.insert(builtins_file(&db), "builtins.sql");

        let mut labels_snippet = Snippet::source(&modified_sql).fold(true);
        for (i, range) in label_annotations.into_iter().enumerate() {
            labels_snippet = labels_snippet.annotation(
                AnnotationKind::Context
                    .span(range)
                    .label(format!("{}. label", i + 1)),
            );
        }

        let mut groups = vec![Level::INFO.primary_title("labels").element(labels_snippet)];

        let mut target_entries = targets_by_file.into_iter().collect::<Vec<_>>();
        target_entries.sort_by_key(|(_, targets)| {
            targets.iter().map(|(i, _)| *i).min().unwrap_or(usize::MAX)
        });

        let target_contents = target_entries
            .into_iter()
            .map(|(f, targets)| {
                let path = *file_paths.get(&f).unwrap();
                (f.content(&db).clone(), path, targets)
            })
            .collect::<Vec<_>>();

        for (content, path, targets) in &target_contents {
            let mut snippet = Snippet::source(content.as_ref()).fold(true).path(*path);
            for (i, range) in targets {
                snippet = snippet.annotation(
                    AnnotationKind::Context
                        .span(range.clone())
                        .label(format!("{i}. target")),
                );
            }
            groups.push(Level::INFO.primary_title("targets").element(snippet));
        }

        let renderer = Renderer::plain().decor_style(DecorStyle::Unicode);
        renderer
            .render(&groups)
            .to_string()
            .replace("info: labels", "labels:")
            .replace("info: targets", "targets:")
    }

    #[test]
    fn single_param() {
        assert_snapshot!(check_inlay_hints("
create function foo(a int) returns int as 'select $$1' language sql;
select foo(1);
"), @"
        labels:
          ╭▸ 
        3 │ select foo(a: 1);
          │            ─── 1. label
          ╰╴
        targets:
          ╭▸ current.sql:2:21
          │
        2 │ create function foo(a int) returns int as 'select $$1' language sql;
          ╰╴                    ─ 1. target
        ");
    }

    #[test]
    fn multiple_params() {
        assert_snapshot!(check_inlay_hints("
create function add(a int, b int) returns int as 'select $$1 + $$2' language sql;
select add(1, 2);
"), @"
        labels:
          ╭▸ 
        3 │ select add(a: 1, b: 2);
          │            ┬──   ─── 2. label
          │            │
          │            1. label
          ╰╴
        targets:
          ╭▸ current.sql:2:21
          │
        2 │ create function add(a int, b int) returns int as 'select $$1 + $$2' language sql;
          │                     ┬      ─ 2. target
          │                     │
          ╰╴                    1. target
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
"), @"
        labels:
          ╭▸ 
        3 │ select public.foo(x: 42);
          │                   ─── 1. label
          ╰╴
        targets:
          ╭▸ current.sql:2:28
          │
        2 │ create function public.foo(x int) returns int as 'select $$1' language sql;
          ╰╴                           ─ 1. target
        ");
    }

    #[test]
    fn with_search_path() {
        assert_snapshot!(check_inlay_hints(r#"
set search_path to myschema;
create function foo(val int) returns int as 'select $$1' language sql;
select foo(100);
"#), @"
        labels:
          ╭▸ 
        4 │ select foo(val: 100);
          │            ───── 1. label
          ╰╴
        targets:
          ╭▸ current.sql:3:21
          │
        3 │ create function foo(val int) returns int as 'select $$1' language sql;
          ╰╴                    ─── 1. target
        ");
    }

    #[test]
    fn multiple_calls() {
        assert_snapshot!(check_inlay_hints("
create function inc(n int) returns int as 'select $$1 + 1' language sql;
select inc(1), inc(2);
"), @"
        labels:
          ╭▸ 
        3 │ select inc(n: 1), inc(n: 2);
          │            ┬──        ─── 2. label
          │            │
          │            1. label
          ╰╴
        targets:
          ╭▸ current.sql:2:21
          │
        2 │ create function inc(n int) returns int as 'select $$1 + 1' language sql;
          │                     ┬
          │                     │
          │                     1. target
          ╰╴                    2. target
        ");
    }

    #[test]
    fn more_args_than_params() {
        assert_snapshot!(check_inlay_hints("
create function foo(a int) returns int as 'select $$1' language sql;
select foo(1, 2);
"), @"
        labels:
          ╭▸ 
        3 │ select foo(a: 1, 2);
          │            ─── 1. label
          ╰╴
        targets:
          ╭▸ current.sql:2:21
          │
        2 │ create function foo(a int) returns int as 'select $$1' language sql;
          ╰╴                    ─ 1. target
        ");
    }

    #[test]
    fn builtin_function() {
        assert_snapshot!(check_inlay_hints("
select json_strip_nulls('[1, null]', true);
"), @"
        labels:
             ╭▸ 
           2 │ select json_strip_nulls(target: '[1, null]', strip_in_arrays: true);
             │                         ──────── 1. label    ───────────────── 2. label
             ╰╴
        targets:
             ╭▸ builtins.sql:9239:45
             │
        9239 │ create function pg_catalog.json_strip_nulls(target json, strip_in_arrays boolean DEFAULT false) returns json
             │                                             ┬─────       ─────────────── 2. target
             │                                             │
             ╰╴                                            1. target
        ");
    }

    #[test]
    fn insert_with_column_list() {
        assert_snapshot!(check_inlay_hints("
create table t (column_a int, column_b int, column_c text);
insert into t (column_a, column_c) values (1, 'foo');
"), @"
        labels:
          ╭▸ 
        3 │ insert into t (column_a, column_c) values (column_a: 1, column_c: 'foo');
          │                                            ┬─────────   ────────── 2. label
          │                                            │
          │                                            1. label
          ╰╴
        targets:
          ╭▸ current.sql:2:17
          │
        2 │ create table t (column_a int, column_b int, column_c text);
          ╰╴                ──────── 1. target          ──────── 2. target
        ");
    }

    #[test]
    fn insert_without_column_list() {
        assert_snapshot!(check_inlay_hints("
create table t (column_a int, column_b int, column_c text);
insert into t values (1, 2, 'foo');
"), @"
        labels:
          ╭▸ 
        3 │ insert into t values (column_a: 1, column_b: 2, column_c: 'foo');
          │                       ┬─────────   ┬─────────   ────────── 3. label
          │                       │            │
          │                       │            2. label
          │                       1. label
          ╰╴
        targets:
          ╭▸ current.sql:2:17
          │
        2 │ create table t (column_a int, column_b int, column_c text);
          │                 ┬───────      ┬───────      ──────── 3. target
          │                 │             │
          │                 │             2. target
          ╰╴                1. target
        ");
    }

    #[test]
    fn insert_multiple_rows() {
        assert_snapshot!(check_inlay_hints("
create table t (x int, y int);
insert into t values (1, 2), (3, 4);
"), @"
        labels:
          ╭▸ 
        3 │ insert into t values (x: 1, y: 2), (x: 3, y: 4);
          │                       ┬──   ┬──     ┬──   ─── 4. label
          │                       │     │       │
          │                       │     │       3. label
          │                       │     2. label
          │                       1. label
          ╰╴
        targets:
          ╭▸ current.sql:2:17
          │
        2 │ create table t (x int, y int);
          │                 ┬      ┬
          │                 │      │
          │                 │      2. target
          │                 │      4. target
          │                 1. target
          ╰╴                3. target
        ");
    }

    #[test]
    fn insert_no_create_table() {
        assert_snapshot!(check_inlay_hints("
insert into t (a, b) values (1, 2);
"), @"
        labels:
          ╭▸ 
        2 │ insert into t (a, b) values (a: 1, b: 2);
          │                              ┬──   ─── 2. label
          │                              │
          ╰╴                             1. label
        ");
    }

    #[test]
    fn insert_more_values_than_columns() {
        assert_snapshot!(check_inlay_hints("
create table t (a int, b int);
insert into t values (1, 2, 3);
"), @"
        labels:
          ╭▸ 
        3 │ insert into t values (a: 1, b: 2, 3);
          │                       ┬──   ─── 2. label
          │                       │
          │                       1. label
          ╰╴
        targets:
          ╭▸ current.sql:2:17
          │
        2 │ create table t (a int, b int);
          │                 ┬      ─ 2. target
          │                 │
          ╰╴                1. target
        ");
    }

    #[test]
    fn insert_table_inherits_select() {
        assert_snapshot!(check_inlay_hints("
create table t (a int, b int);
create table u (c int) inherits (t);
insert into u select 1, 2, 3;
"), @"
        labels:
          ╭▸ 
        4 │ insert into u select a: 1, b: 2, c: 3;
          │                      ┬──   ┬──   ─── 3. label
          │                      │     │
          │                      │     2. label
          │                      1. label
          ╰╴
        targets:
          ╭▸ current.sql:2:17
          │
        2 │ create table t (a int, b int);
          │                 ┬      ─ 2. target
          │                 │
          │                 1. target
        3 │ create table u (c int) inherits (t);
          ╰╴                ─ 3. target
        ");
    }

    #[test]
    fn insert_table_inherits_builtin_values() {
        assert_snapshot!(check_inlay_hints("
create table t ()
inherits (information_schema.sql_features);
insert into t values (1, 2, 3, 4, 5, 6, 7);
"), @"
        labels:
            ╭▸ 
          4 │ …ues (feature_id: 1, feature_name: 2, sub_feature_id: 3, sub_feature_name: 4, is_supported: 5, is_verified_by: 6, comments: 7);
            │       ┬───────────   ┬─────────────   ┬───────────────   ┬─────────────────   ┬─────────────   ┬───────────────   ────────── 7. label
            │       │              │                │                  │                    │                │
            │       │              │                │                  │                    │                6. label
            │       │              │                │                  │                    5. label
            │       │              │                │                  4. label
            │       │              │                3. label
            │       │              2. label
            │       1. label
            ╰╴
        targets:
            ╭▸ builtins.sql:436:3
            │
        436 │   feature_id information_schema.character_data,
            │   ────────── 1. target
        437 │   feature_name information_schema.character_data,
            │   ──────────── 2. target
        438 │   sub_feature_id information_schema.character_data,
            │   ────────────── 3. target
        439 │   sub_feature_name information_schema.character_data,
            │   ──────────────── 4. target
        440 │   is_supported information_schema.yes_or_no,
            │   ──────────── 5. target
        441 │   is_verified_by information_schema.character_data,
            │   ────────────── 6. target
        442 │   comments information_schema.character_data
            ╰╴  ──────── 7. target
        ");
    }

    #[test]
    fn insert_table_inherits_create_table_as_values() {
        assert_snapshot!(check_inlay_hints("
create table parent as select 1 a, 'x'::text b;
create table child (c int) inherits (parent);
insert into child values (1, 2, 3);
"), @"
        labels:
          ╭▸ 
        4 │ insert into child values (a: 1, b: 2, c: 3);
          │                           ┬──   ┬──   ─── 3. label
          │                           │     │
          │                           │     2. label
          │                           1. label
          ╰╴
        targets:
          ╭▸ current.sql:3:21
          │
        3 │ create table child (c int) inherits (parent);
          ╰╴                    ─ 3. target
        ");
    }

    #[test]
    fn insert_table_inherits_create_table_as_select_star() {
        assert_snapshot!(check_inlay_hints("
create table base (a int, b text);
create table parent as select * from base;
create table child (c int) inherits (parent);
insert into child values (1, 2, 3);
"), @"
        labels:
          ╭▸ 
        5 │ insert into child values (a: 1, b: 2, c: 3);
          │                           ┬──   ┬──   ─── 3. label
          │                           │     │
          │                           │     2. label
          │                           1. label
          ╰╴
        targets:
          ╭▸ current.sql:4:21
          │
        4 │ create table child (c int) inherits (parent);
          ╰╴                    ─ 3. target
        ");
    }

    #[test]
    fn insert_table_like_select() {
        assert_snapshot!(check_inlay_hints("
create table x (a int, b int);
create table y (c int, like x);
insert into y select 1, 2, 3;
"), @"
        labels:
          ╭▸ 
        4 │ insert into y select c: 1, a: 2, b: 3;
          │                      ┬──   ┬──   ─── 3. label
          │                      │     │
          │                      │     2. label
          │                      1. label
          ╰╴
        targets:
          ╭▸ current.sql:2:17
          │
        2 │ create table x (a int, b int);
          │                 ┬      ─ 3. target
          │                 │
          │                 2. target
        3 │ create table y (c int, like x);
          ╰╴                ─ 1. target
        ");
    }

    #[test]
    fn insert_select() {
        assert_snapshot!(check_inlay_hints("
create table t (a int, b int);
insert into t select 1, 2;
"), @"
        labels:
          ╭▸ 
        3 │ insert into t select a: 1, b: 2;
          │                      ┬──   ─── 2. label
          │                      │
          │                      1. label
          ╰╴
        targets:
          ╭▸ current.sql:2:17
          │
        2 │ create table t (a int, b int);
          │                 ┬      ─ 2. target
          │                 │
          ╰╴                1. target
        ");
    }

    #[test]
    fn insert_table_like_builtin_values() {
        assert_snapshot!(check_inlay_hints("
create table t (like information_schema.sql_features);
insert into t values (1, 2, 3, 4, 5, 6, 7);
"), @"
        labels:
            ╭▸ 
          3 │ …ues (feature_id: 1, feature_name: 2, sub_feature_id: 3, sub_feature_name: 4, is_supported: 5, is_verified_by: 6, comments: 7);
            │       ┬───────────   ┬─────────────   ┬───────────────   ┬─────────────────   ┬─────────────   ┬───────────────   ────────── 7. label
            │       │              │                │                  │                    │                │
            │       │              │                │                  │                    │                6. label
            │       │              │                │                  │                    5. label
            │       │              │                │                  4. label
            │       │              │                3. label
            │       │              2. label
            │       1. label
            ╰╴
        targets:
            ╭▸ builtins.sql:436:3
            │
        436 │   feature_id information_schema.character_data,
            │   ────────── 1. target
        437 │   feature_name information_schema.character_data,
            │   ──────────── 2. target
        438 │   sub_feature_id information_schema.character_data,
            │   ────────────── 3. target
        439 │   sub_feature_name information_schema.character_data,
            │   ──────────────── 4. target
        440 │   is_supported information_schema.yes_or_no,
            │   ──────────── 5. target
        441 │   is_verified_by information_schema.character_data,
            │   ────────────── 6. target
        442 │   comments information_schema.character_data
            ╰╴  ──────── 7. target
        ");
    }

    #[test]
    fn insert_table_like_select_into_values() {
        assert_snapshot!(check_inlay_hints("
select 1 a, 'x'::text b into parent;
create table child (like parent);
insert into child values (1, 2);
"), @"
        labels:
          ╭▸ 
        4 │ insert into child values (a: 1, b: 2);
          │                           ┬──   ─── 2. label
          │                           │
          ╰╴                          1. label
        ");
    }
}
