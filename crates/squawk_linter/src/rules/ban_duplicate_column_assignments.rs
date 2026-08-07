use rustc_hash::FxHashMap;
use squawk_syntax::{
    Parse, SourceFile, SyntaxKind, SyntaxNode,
    ast::{self, AstNode, NameLike},
    column_name::ColumnName,
};

use rowan::TextRange;

use crate::{Edit, Fix, Linter, Rule, Violation};

use super::identifier_too_long::MAX_IDENT_BYTES;

#[derive(Debug, Eq, Hash, PartialEq)]
struct Name(String);

impl Name {
    fn from_node(node: &impl NameLike) -> Self {
        Self::from_string(node.text())
    }

    fn from_string(mut text: String) -> Self {
        text.truncate(text.floor_char_boundary(MAX_IDENT_BYTES));
        Self(text)
    }

    fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug)]
struct Assignment {
    column: ast::ColumnNameRef,
    is_partial: bool,
    set_column: ast::SetColumn,
}

#[derive(Debug)]
struct InsertAssignment {
    column: ast::ColumnNameRef,
    is_partial: bool,
}

pub(crate) fn ban_duplicate_column_assignments(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    for node in parse.tree().syntax().descendants() {
        if let Some(set_clause) = ast::SetClause::cast(node.clone()) {
            check_set_clause(ctx, &set_clause);
        } else if let Some(insert) = ast::Insert::cast(node.clone())
            && let Some(column_target_list) = insert.column_target_list()
        {
            check_insert_column_target_list(ctx, &column_target_list);
        } else if let Some(merge_insert) = ast::MergeInsert::cast(node.clone())
            && let Some(column_target_list) = merge_insert.column_target_list()
        {
            check_insert_column_target_list(ctx, &column_target_list);
        } else if let Some(create_table) = ast::CreateTableLike::cast(node.clone())
            && let Some(table_arg_list) = create_table.table_arg_list()
        {
            check_table_arg_list(ctx, &table_arg_list);
        } else if let Some(create_table_as) = ast::CreateTableAs::cast(node.clone())
            && let Some(table_arg_list) = create_table_as.table_arg_list()
        {
            check_table_arg_list(ctx, &table_arg_list);
        } else if let Some(create_view) = ast::CreateViewLike::cast(node.clone()) {
            if let Some(column_list) = create_view.column_list() {
                check_defined_columns(ctx, column_list.column_names());
            } else if let Some(target_list) =
                create_view.query().and_then(|query| query.target_list())
            {
                check_target_aliases(ctx, &target_list);
            }
        } else if let Some(select_into) = ast::SelectInto::cast(node)
            && let Some(target_list) = select_into
                .select_clause()
                .and_then(|select_clause| select_clause.target_list())
        {
            check_target_aliases(ctx, &target_list);
        }
    }
}

fn check_table_arg_list(ctx: &mut Linter, table_arg_list: &ast::TableArgList) {
    check_defined_columns(
        ctx,
        table_arg_list.args().filter_map(|arg| match arg {
            ast::TableArg::Column(column) => column.name(),
            _ => None,
        }),
    );
}

fn check_target_aliases(ctx: &mut Linter, target_list: &ast::TargetList) {
    check_defined_nodes(
        ctx,
        target_list.targets().filter_map(|target| {
            let (name, node) = ColumnName::from_target(target)?;
            Some((Name::from_string(name.to_string()?), node))
        }),
    );
}

fn check_defined_columns(ctx: &mut Linter, columns: impl Iterator<Item = ast::ColumnName>) {
    check_defined_nodes(
        ctx,
        columns.map(|column| (Name::from_node(&column), column.syntax().clone())),
    );
}

fn check_defined_nodes(ctx: &mut Linter, columns: impl Iterator<Item = (Name, SyntaxNode)>) {
    let mut defined_columns: FxHashMap<Name, Vec<SyntaxNode>> = FxHashMap::default();

    for (name, column) in columns {
        defined_columns.entry(name).or_default().push(column);
    }

    for (name, columns) in defined_columns {
        if columns.len() < 2 {
            continue;
        }

        for column in columns {
            ctx.report(Violation::for_node(
                Rule::BanDuplicateColumnAssignments,
                format!("Column `{}` is specified more than once.", name.as_str()),
                &column,
            ));
        }
    }
}

fn check_insert_column_target_list(ctx: &mut Linter, column_target_list: &ast::ColumnTargetList) {
    let mut assigned_columns: FxHashMap<Name, Vec<InsertAssignment>> = FxHashMap::default();

    for target in column_target_list.column_targets() {
        let Some(column) = target.name() else {
            continue;
        };
        assigned_columns
            .entry(Name::from_node(&column))
            .or_default()
            .push(InsertAssignment {
                column,
                is_partial: target.accessors().next().is_some(),
            });
    }

    for (name, assignments) in assigned_columns {
        if assignments.len() < 2 || assignments.iter().all(|assignment| assignment.is_partial) {
            continue;
        }

        for assignment in assignments {
            ctx.report(Violation::for_node(
                Rule::BanDuplicateColumnAssignments,
                format!(
                    "Multiple assignments to the same column `{}`.",
                    name.as_str()
                ),
                assignment.column.syntax(),
            ));
        }
    }
}

fn check_set_clause(ctx: &mut Linter, set_clause: &ast::SetClause) {
    let Some(set_column_list) = set_clause.set_column_list() else {
        return;
    };
    let set_columns = set_column_list.set_columns().collect::<Vec<_>>();
    let mut assigned_columns: FxHashMap<Name, Vec<Assignment>> = FxHashMap::default();

    for set_column in &set_columns {
        match set_column {
            ast::SetColumn::SetMultipleColumns(set_multiple_columns) => {
                let Some(column_target_list) = set_multiple_columns.column_target_list() else {
                    continue;
                };
                for target in column_target_list.column_targets() {
                    add_assignment(&mut assigned_columns, set_column.clone(), &target);
                }
            }
            ast::SetColumn::SetSingleColumn(set_single_column) => {
                if let Some(target) = set_single_column.column_target() {
                    add_assignment(&mut assigned_columns, set_column.clone(), &target);
                }
            }
        }
    }

    for (name, assignments) in assigned_columns {
        if assignments.len() < 2 || assignments.iter().all(|assignment| assignment.is_partial) {
            continue;
        }
        let mut fix = create_fix(&name, &assignments, &set_columns);
        let last_index = assignments.len() - 1;

        for (index, assignment) in assignments.iter().enumerate() {
            let violation = Violation::for_node(
                Rule::BanDuplicateColumnAssignments,
                format!(
                    "Multiple assignments to the same column `{}`.",
                    name.as_str()
                ),
                assignment.column.syntax(),
            );
            ctx.report(if index == last_index {
                violation.fix(fix.take())
            } else {
                violation
            });
        }
    }
}

fn add_assignment(
    assigned_columns: &mut FxHashMap<Name, Vec<Assignment>>,
    set_column: ast::SetColumn,
    target: &ast::ColumnTarget,
) {
    let Some(column) = target.name() else {
        return;
    };
    let name = Name::from_node(&column);
    assigned_columns.entry(name).or_default().push(Assignment {
        column,
        is_partial: target.accessors().next().is_some(),
        set_column,
    });
}

fn create_fix(
    name: &Name,
    assignments: &[Assignment],
    set_columns: &[ast::SetColumn],
) -> Option<Fix> {
    let mut edits = Vec::with_capacity(assignments.len() - 1);

    for assignment in &assignments[..assignments.len() - 1] {
        if !matches!(assignment.set_column, ast::SetColumn::SetSingleColumn(_)) {
            return None;
        }
        let index = set_columns
            .iter()
            .position(|set_column| set_column.syntax() == assignment.set_column.syntax())?;
        let next_set_column = set_columns.get(index + 1)?;
        let next_start = next_set_column.syntax().text_range().start();
        let mut end = next_start;
        let mut seen_comma = false;
        let mut token = assignment.set_column.syntax().last_token()?.next_token();

        while let Some(current) = token
            && current.text_range().start() < next_start
        {
            if seen_comma && current.kind() == SyntaxKind::COMMENT {
                end = current.text_range().start();
                break;
            }
            seen_comma |= current.kind() == SyntaxKind::COMMA;
            token = current.next_token();
        }

        edits.push(Edit::delete(TextRange::new(
            assignment.set_column.syntax().text_range().start(),
            end,
        )));
    }

    Some(Fix::new(
        format!("Delete all but the last assignment to `{}`", name.as_str()),
        edits,
    ))
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::{
        Rule,
        test_utils::{fix_sql, lint_errors, lint_ok},
    };

    fn lint(sql: &str) -> String {
        lint_errors(sql, Rule::BanDuplicateColumnAssignments)
    }

    #[test]
    fn duplicate_assignment_err() {
        let sql = r#"
drop table k;
create table k (
  a int,
  b int,
  c int
);
update k
set
  a = 1,
  a = 1,
  b = 2,
  c = 3;
"#;
        assert_snapshot!(lint(sql), @"
        warning[ban-duplicate-column-assignments]: Multiple assignments to the same column `a`.
           ╭▸ 
        10 │   a = 1,
           ╰╴  ━
        warning[ban-duplicate-column-assignments]: Multiple assignments to the same column `a`.
           ╭▸ 
        11 │   a = 1,
           │   ━
           ╭╴
        10 -   a = 1,
           ╰╴
        ");
    }

    #[test]
    fn fix_deletes_all_but_last_assignment() {
        let sql = r#"
update k
set
  a = 1,
  b = 2,
  a = 3,
  a = 4,
  c = 5;
"#;
        assert_snapshot!(fix_sql(sql, Rule::BanDuplicateColumnAssignments), @"
        update k
        set
          b = 2,
          a = 4,
          c = 5;
        ");
    }

    #[test]
    fn fix_preserves_comments_between_assignments() {
        let sql = r#"
update k
set
  a = 1 /* some comment */,
  -- another comment
  b = 2,
  a = 3;
"#;
        assert_snapshot!(fix_sql(sql, Rule::BanDuplicateColumnAssignments), @"
        update k
        set
          -- another comment
          b = 2,
          a = 3;
        ");
    }

    #[test]
    fn duplicate_multiple_column_assignment_err() {
        let sql = r#"
update k
set
  (a, b) = (1, 2),
  a = 3;
"#;
        assert_snapshot!(lint(sql), @"
        warning[ban-duplicate-column-assignments]: Multiple assignments to the same column `a`.
          ╭▸ 
        4 │   (a, b) = (1, 2),
          ╰╴   ━
        warning[ban-duplicate-column-assignments]: Multiple assignments to the same column `a`.
          ╭▸ 
        5 │   a = 3;
          ╰╴  ━
        ");
    }

    #[test]
    fn whole_and_partial_assignment_err() {
        let sql = r#"
create type k_record as (
  x int,
  y int
);
create table k (
  a k_record,
  b int,
  c int
);
update k
set
  a = row(1, 2),
  a.x = 1;
"#;
        assert_snapshot!(lint(sql), @"
        warning[ban-duplicate-column-assignments]: Multiple assignments to the same column `a`.
           ╭▸ 
        13 │   a = row(1, 2),
           ╰╴  ━
        warning[ban-duplicate-column-assignments]: Multiple assignments to the same column `a`.
           ╭▸ 
        14 │   a.x = 1;
           │   ━
           ╭╴
        13 -   a = row(1, 2),
           ╰╴
        ");
    }

    #[test]
    fn names_use_postgres_case_rules() {
        let sql = r#"
update k
set
  a = 1,
  A = 2;
update k
set
  "a" = 1,
  a = 2;
"#;
        assert_snapshot!(lint(sql), @r#"
        warning[ban-duplicate-column-assignments]: Multiple assignments to the same column `a`.
          ╭▸ 
        4 │   a = 1,
          ╰╴  ━
        warning[ban-duplicate-column-assignments]: Multiple assignments to the same column `a`.
          ╭▸ 
        5 │   A = 2;
          │   ━
          ╭╴
        4 -   a = 1,
          ╰╴
        warning[ban-duplicate-column-assignments]: Multiple assignments to the same column `a`.
          ╭▸ 
        8 │   "a" = 1,
          ╰╴  ━━━
        warning[ban-duplicate-column-assignments]: Multiple assignments to the same column `a`.
          ╭▸ 
        9 │   a = 2;
          │   ━
          ╭╴
        8 -   "a" = 1,
          ╰╴
        "#);
    }

    #[test]
    fn names_use_postgres_truncation_rules() {
        let prefix = "a".repeat(63);
        let sql = format!("update k set {prefix}x = 1, {prefix}y = 2;");

        assert_snapshot!(lint(&sql), @"
        warning[ban-duplicate-column-assignments]: Multiple assignments to the same column `aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa`.
          ╭▸ 
        1 │ update k set aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaax = 1, aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa…
          ╰╴             ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
        warning[ban-duplicate-column-assignments]: Multiple assignments to the same column `aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa`.
          ╭▸ 
        1 │ …aaaaaaaaaaaaaaaaaaaaaaaaaaaax = 1, aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaay = 2;
          │                                     ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
          ╭╴
        1 - update k set aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaax = 1, aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaay = 2;
        1 + update k set aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaay = 2;
          ╰╴
        ");
    }

    #[test]
    fn insert_err() {
        let sql = r#"
insert into t (a, a)
values (1, 2);
"#;
        assert_snapshot!(lint(sql), @"
        warning[ban-duplicate-column-assignments]: Multiple assignments to the same column `a`.
          ╭▸ 
        2 │ insert into t (a, a)
          ╰╴               ━
        warning[ban-duplicate-column-assignments]: Multiple assignments to the same column `a`.
          ╭▸ 
        2 │ insert into t (a, a)
          ╰╴                  ━
        ");
    }

    #[test]
    fn merge_insert_err() {
        let sql = r#"
merge into t
using s on t.id = s.id
when not matched then
  insert (a, a)
  values (1, 2);
"#;
        assert_snapshot!(lint(sql), @"
        warning[ban-duplicate-column-assignments]: Multiple assignments to the same column `a`.
          ╭▸ 
        5 │   insert (a, a)
          ╰╴          ━
        warning[ban-duplicate-column-assignments]: Multiple assignments to the same column `a`.
          ╭▸ 
        5 │   insert (a, a)
          ╰╴             ━
        ");
    }

    #[test]
    fn create_table_err() {
        let sql = r#"
create table t (
  a int,
  a text
);
"#;
        assert_snapshot!(lint(sql), @"
        warning[ban-duplicate-column-assignments]: Column `a` is specified more than once.
          ╭▸ 
        3 │   a int,
          ╰╴  ━
        warning[ban-duplicate-column-assignments]: Column `a` is specified more than once.
          ╭▸ 
        4 │   a text
          ╰╴  ━
        ");
    }

    #[test]
    fn create_foreign_table_err() {
        let sql = r#"
create foreign table t (
  a int,
  a text
) server s;
"#;
        assert_snapshot!(lint(sql), @"
        warning[ban-duplicate-column-assignments]: Column `a` is specified more than once.
          ╭▸ 
        3 │   a int,
          ╰╴  ━
        warning[ban-duplicate-column-assignments]: Column `a` is specified more than once.
          ╭▸ 
        4 │   a text
          ╰╴  ━
        ");
    }

    #[test]
    fn create_view_err() {
        let sql = r#"
create view v (a, a) as
select 1, 2;
"#;
        assert_snapshot!(lint(sql), @"
        warning[ban-duplicate-column-assignments]: Column `a` is specified more than once.
          ╭▸ 
        2 │ create view v (a, a) as
          ╰╴               ━
        warning[ban-duplicate-column-assignments]: Column `a` is specified more than once.
          ╭▸ 
        2 │ create view v (a, a) as
          ╰╴                  ━
        ");
    }

    #[test]
    fn create_view_with_duplicate_target_aliases_err() {
        let sql = r#"
create view v as
select 1 a, 2 a;
"#;
        assert_snapshot!(lint(sql), @"
        warning[ban-duplicate-column-assignments]: Column `a` is specified more than once.
          ╭▸ 
        3 │ select 1 a, 2 a;
          ╰╴         ━
        warning[ban-duplicate-column-assignments]: Column `a` is specified more than once.
          ╭▸ 
        3 │ select 1 a, 2 a;
          ╰╴              ━
        ");
    }

    #[test]
    fn create_view_with_duplicate_inferred_names_err() {
        let sql = r#"
create view v as
select 1, 2;
"#;
        assert_snapshot!(lint(sql), @"
        warning[ban-duplicate-column-assignments]: Column `?column?` is specified more than once.
          ╭▸ 
        3 │ select 1, 2;
          ╰╴       ━
        warning[ban-duplicate-column-assignments]: Column `?column?` is specified more than once.
          ╭▸ 
        3 │ select 1, 2;
          ╰╴          ━
        ");
    }

    #[test]
    fn select_into_err() {
        let sql = "select 1 a, 2 a into z;";
        assert_snapshot!(lint(sql), @"
        warning[ban-duplicate-column-assignments]: Column `a` is specified more than once.
          ╭▸ 
        1 │ select 1 a, 2 a into z;
          ╰╴         ━
        warning[ban-duplicate-column-assignments]: Column `a` is specified more than once.
          ╭▸ 
        1 │ select 1 a, 2 a into z;
          ╰╴              ━
        ");
    }

    #[test]
    fn conflict_update_err() {
        let sql = r#"
insert into k
values (1)
on conflict (id)
do update
set
  a = 1,
  a = 2;
"#;
        assert_snapshot!(lint(sql), @"
        warning[ban-duplicate-column-assignments]: Multiple assignments to the same column `a`.
          ╭▸ 
        7 │   a = 1,
          ╰╴  ━
        warning[ban-duplicate-column-assignments]: Multiple assignments to the same column `a`.
          ╭▸ 
        8 │   a = 2;
          │   ━
          ╭╴
        7 -   a = 1,
          ╰╴
        ");
    }

    #[test]
    fn merge_update_err() {
        let sql = r#"
merge into k
using j on k.id = j.id
when matched then
  update
  set
    a = 1,
    a = 2;
"#;
        assert_snapshot!(lint(sql), @"
        warning[ban-duplicate-column-assignments]: Multiple assignments to the same column `a`.
          ╭▸ 
        7 │     a = 1,
          ╰╴    ━
        warning[ban-duplicate-column-assignments]: Multiple assignments to the same column `a`.
          ╭▸ 
        8 │     a = 2;
          │     ━
          ╭╴
        7 -     a = 1,
          ╰╴
        ");
    }

    #[test]
    fn distinct_and_partial_assignments_ok() {
        let sql = r#"
update k
set
  a = 1,
  b = 2;
update k
set
  a.x = 1,
  a.y = 2;
update k
set
  a[1] = 1,
  a[2] = 2;
update k
set
  "A" = 1,
  A = 2;
insert into k (a, b)
values (1, 2);
create table t (
  a int,
  b int
);
create foreign table ft (
  a int,
  b int
) server s;
create view v (a, b) as
select 1 a, 2 a;
create view inferred_v as
select 1 a, 2 b;
select 1 a, 2 b into z;
insert into k (a.x, a.y)
values (1, 2);
merge into k
using j on k.id = j.id
when not matched then
  insert (a.x, a.y)
  values (1, 2);
"#;
        lint_ok(sql, Rule::BanDuplicateColumnAssignments);
    }
}
