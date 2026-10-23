use std::ops::Range;

use annotate_snippets::{AnnotationKind, Level, Renderer, Snippet, renderer::DecorStyle};
use insta::assert_snapshot;
use squawk_syntax::SourceFile;

use crate::{
    builtins::BUILTINS_SQL,
    extensions::BundledExtension,
    file::{FileId, InFile},
    inlay_hints::inlay_hints_with_index as inlay_hints,
    semantic_index_builder::SemanticIndexBuilder,
};

#[must_use]
#[track_caller]
fn check_inlay_hints(sql: &str) -> String {
    let parse = SourceFile::parse(sql);
    assert_eq!(parse.errors(), vec![]);
    let tree = parse.tree();
    let db = crate::db::Database::default();
    let file = InFile::new(crate::db::File::new(&db, sql.into()), &tree);
    let mut builder = SemanticIndexBuilder::new();
    builder.add_file(file);
    let index = builder.build();
    let hints = inlay_hints(&index, file);
    if hints.is_empty() {
        return String::new();
    }

    let mut order: Vec<usize> = (0..hints.len()).collect();
    order.sort_by_key(|&i| hints[i].position);
    let mut modified_sql =
        String::with_capacity(sql.len() + hints.iter().map(|hint| hint.label.len()).sum::<usize>());
    modified_sql.push_str(sql);
    let mut label_ranges: Vec<Range<usize>> = vec![0..0; hints.len()];
    let mut shift = 0;
    for i in order {
        let hint = &hints[i];
        let position = usize::from(hint.position) + shift;
        modified_sql.insert_str(position, &hint.label);
        label_ranges[i] = position..position + hint.label.len();
        shift += hint.label.len();
    }

    let mut labels = Snippet::source(&modified_sql).fold(true);
    for (i, range) in label_ranges.into_iter().enumerate() {
        labels = labels.annotation(
            AnnotationKind::Context
                .span(range)
                .label(format!("{}. label", i + 1)),
        );
    }
    let mut groups = vec![Level::INFO.primary_title("labels").element(labels)];

    let mut file_ids: Vec<FileId> = vec![];
    for target in hints.iter().filter_map(|hint| hint.target) {
        if !file_ids.contains(&target.file_id) {
            file_ids.push(target.file_id);
        }
    }
    for file_id in file_ids {
        let (source, path) = if file_id == FileId::Builtins {
            (BUILTINS_SQL, "builtins.sql")
        } else if let Some(extension) = BundledExtension::from_file(file_id) {
            (extension.sql(), extension.path())
        } else {
            (sql, "file0.sql")
        };
        let mut snippet = Snippet::source(source).path(path).fold(true);
        for (i, hint) in hints.iter().enumerate() {
            if let Some(target) = hint.target
                && target.file_id == file_id
            {
                snippet = snippet.annotation(
                    AnnotationKind::Context
                        .span(target.value.start().into()..target.value.end().into())
                        .label(format!("{}. target", i + 1)),
                );
            }
        }
        groups.push(Level::INFO.primary_title("targets").element(snippet));
    }

    Renderer::plain()
        .decor_style(DecorStyle::Unicode)
        .render(&groups)
        .replace("info: labels", "labels:")
        .replace("info: targets", "targets:")
}

#[test]
fn single_param() {
    assert_snapshot!(check_inlay_hints("
create function foo(a int) returns int as 'select $1' language sql;
select foo(1);
"), @"
    labels:
      ╭▸ 
    3 │ select foo(a: 1);
      │            ─── 1. label
      ╰╴
    targets:
      ╭▸ file0.sql:2:21
      │
    2 │ create function foo(a int) returns int as 'select $1' language sql;
      ╰╴                    ─ 1. target
    ");
}

#[test]
fn multiple_params() {
    assert_snapshot!(check_inlay_hints("
create function add(a int, b int) returns int as 'select $1 + $2' language sql;
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
      ╭▸ file0.sql:2:21
      │
    2 │ create function add(a int, b int) returns int as 'select $1 + $2' language sql;
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
create function public.foo(x int) returns int as 'select $1' language sql;
select public.foo(42);
"), @"
    labels:
      ╭▸ 
    3 │ select public.foo(x: 42);
      │                   ─── 1. label
      ╰╴
    targets:
      ╭▸ file0.sql:2:28
      │
    2 │ create function public.foo(x int) returns int as 'select $1' language sql;
      ╰╴                           ─ 1. target
    ");
}

#[test]
fn with_search_path() {
    assert_snapshot!(check_inlay_hints(r#"
set search_path to myschema;
create function foo(val int) returns int as 'select $1' language sql;
select foo(100);
"#), @"
    labels:
      ╭▸ 
    4 │ select foo(val: 100);
      │            ───── 1. label
      ╰╴
    targets:
      ╭▸ file0.sql:3:21
      │
    3 │ create function foo(val int) returns int as 'select $1' language sql;
      ╰╴                    ─── 1. target
    ");
}

#[test]
fn multiple_calls() {
    assert_snapshot!(check_inlay_hints("
create function inc(n int) returns int as 'select $1 + 1' language sql;
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
      ╭▸ file0.sql:2:21
      │
    2 │ create function inc(n int) returns int as 'select $1 + 1' language sql;
      │                     ┬
      │                     │
      │                     1. target
      ╰╴                    2. target
    ");
}

#[test]
fn more_args_than_params() {
    assert_snapshot!(check_inlay_hints("
create function foo(a int) returns int as 'select $1' language sql;
select foo(1, 2);
"), @"");
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
          ╭▸ builtins.sql:16230:3
          │
    16230 │   target json,
          │   ────── 1. target
    16231 │   strip_in_arrays boolean default false
          ╰╴  ─────────────── 2. target
    ");
}

#[test]
fn insert_without_definition() {
    assert_snapshot!(check_inlay_hints("
insert into missing(a, b) values (1, 2);
"), @"
    labels:
      ╭▸ 
    2 │ insert into missing(a, b) values (a: 1, b: 2);
      │                                   ┬──   ─── 2. label
      │                                   │
      ╰╴                                  1. label
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
      ╭▸ file0.sql:2:17
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
      ╭▸ file0.sql:2:17
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
      ╭▸ file0.sql:2:17
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
      ╭▸ file0.sql:2:17
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
      ╭▸ file0.sql:2:17
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
        ╭▸ builtins.sql:481:3
        │
    481 │   feature_id information_schema.character_data,
        │   ────────── 1. target
    482 │   feature_name information_schema.character_data,
        │   ──────────── 2. target
    483 │   sub_feature_id information_schema.character_data,
        │   ────────────── 3. target
    484 │   sub_feature_name information_schema.character_data,
        │   ──────────────── 4. target
    485 │   is_supported information_schema.yes_or_no,
        │   ──────────── 5. target
    486 │   is_verified_by information_schema.character_data,
        │   ────────────── 6. target
    487 │   comments information_schema.character_data
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
      ╭▸ file0.sql:2:33
      │
    2 │ create table parent as select 1 a, 'x'::text b;
      │                                 ─ 1. target  ─ 2. target
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
      ╭▸ file0.sql:2:20
      │
    2 │ create table base (a int, b text);
      │                    ┬      ─ 2. target
      │                    │
      │                    1. target
    3 │ create table parent as select * from base;
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
      ╭▸ file0.sql:2:17
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
      ╭▸ file0.sql:2:17
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
        ╭▸ builtins.sql:481:3
        │
    481 │   feature_id information_schema.character_data,
        │   ────────── 1. target
    482 │   feature_name information_schema.character_data,
        │   ──────────── 2. target
    483 │   sub_feature_id information_schema.character_data,
        │   ────────────── 3. target
    484 │   sub_feature_name information_schema.character_data,
        │   ──────────────── 4. target
    485 │   is_supported information_schema.yes_or_no,
        │   ──────────── 5. target
    486 │   is_verified_by information_schema.character_data,
        │   ────────────── 6. target
    487 │   comments information_schema.character_data
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
      │                           1. label
      ╰╴
    targets:
      ╭▸ file0.sql:2:10
      │
    2 │ select 1 a, 'x'::text b into parent;
      ╰╴         ─ 1. target  ─ 2. target
    ");
}

#[test]
fn named_args() {
    assert_snapshot!(check_inlay_hints("
create function foo(a int, b int) returns int as 'select $1' language sql;
select foo(1, b => 2);
"), @"
    labels:
      ╭▸ 
    3 │ select foo(a: 1, b => 2);
      │            ─── 1. label
      ╰╴
    targets:
      ╭▸ file0.sql:2:21
      │
    2 │ create function foo(a int, b int) returns int as 'select $1' language sql;
      ╰╴                    ─ 1. target
    ");
}

#[test]
fn out_params() {
    assert_snapshot!(check_inlay_hints("
create function foo(out x int, a int, out y int, b int) as 'select 1, 2' language sql;
select foo(1, 2);
"), @"
    labels:
      ╭▸ 
    3 │ select foo(a: 1, b: 2);
      │            ┬──   ─── 2. label
      │            │
      │            1. label
      ╰╴
    targets:
      ╭▸ file0.sql:2:32
      │
    2 │ create function foo(out x int, a int, out y int, b int) as 'select 1, 2' language sql;
      ╰╴                               ─ 1. target       ─ 2. target
    ");
}

#[test]
fn call_procedure() {
    assert_snapshot!(check_inlay_hints("
create procedure p(a int, out b int) language sql as 'select 1';
call p(1, null);
"), @"
    labels:
      ╭▸ 
    3 │ call p(a: 1, b: null);
      │        ┬──   ─── 2. label
      │        │
      │        1. label
      ╰╴
    targets:
      ╭▸ file0.sql:2:20
      │
    2 │ create procedure p(a int, out b int) language sql as 'select 1';
      │                    ┬          ─ 2. target
      │                    │
      ╰╴                   1. target
    ");
}

#[test]
fn insert_with_cte() {
    assert_snapshot!(check_inlay_hints("
create table t (a int, b int);
with x as (select 1) insert into t values (1, 2);
"), @"
    labels:
      ╭▸ 
    3 │ with x as (select 1) insert into t values (a: 1, b: 2);
      │                                            ┬──   ─── 2. label
      │                                            │
      │                                            1. label
      ╰╴
    targets:
      ╭▸ file0.sql:2:17
      │
    2 │ create table t (a int, b int);
      │                 ┬      ─ 2. target
      │                 │
      ╰╴                1. target
    ");
}
