use rustc_hash::{FxHashMap, FxHashSet};
use squawk_syntax::{
    Parse, SourceFile, SyntaxNode,
    ast::{self, AstNode, NameLike},
};

use crate::{Linter, Rule, Violation};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Action {
    Delete,
    Update,
}

impl Action {
    fn sql(self) -> &'static str {
        match self {
            Action::Delete => "ON DELETE SET NULL",
            Action::Update => "ON UPDATE SET NULL",
        }
    }

    fn consequence(self) -> &'static str {
        match self {
            Action::Delete => "deleting a referenced row will fail",
            Action::Update => "updating a referenced key will fail",
        }
    }
}

#[derive(Debug)]
struct SetNullAction {
    action: Action,
    columns: Vec<String>,
    node: SyntaxNode,
}

#[derive(Debug)]
struct SetNullForeignKey {
    name: Option<String>,
    actions: Vec<SetNullAction>,
}

#[derive(Debug, Default)]
struct TableState {
    not_null_columns: FxHashSet<String>,
    foreign_keys: Vec<SetNullForeignKey>,
}

const HELP: &str =
    "Use `CASCADE`, `RESTRICT`, or `NO ACTION` instead of `SET NULL`, or make the column nullable.";

pub(crate) fn ban_set_null_on_not_null_column(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    let mut tables: FxHashMap<Vec<String>, TableState> = FxHashMap::default();

    for stmt in parse.tree().stmts() {
        match stmt {
            ast::Stmt::CreateTable(create_table) => {
                let Some(table) = create_table
                    .table_name()
                    .and_then(|table_name| table_name.path())
                    .map(|path| path_key(&path))
                else {
                    continue;
                };
                let state = tables.entry(table).or_default();
                *state = TableState::default();
                let Some(table_arg_list) = create_table.table_arg_list() else {
                    continue;
                };
                let mut foreign_keys = vec![];
                for arg in table_arg_list.args() {
                    match arg {
                        ast::TableArg::Column(column) => {
                            let Some(column_name) = column.name().map(|name| name.text()) else {
                                continue;
                            };
                            for clause in column.clauses() {
                                let ast::ColumnClause::ColumnConstraint(constraint) = clause else {
                                    continue;
                                };
                                match constraint {
                                    ast::ColumnConstraint::NotNullConstraint(_)
                                    | ast::ColumnConstraint::PrimaryKeyConstraint(_) => {
                                        state.not_null_columns.insert(column_name.clone());
                                    }
                                    ast::ColumnConstraint::ReferencesConstraint(references) => {
                                        foreign_keys.push(references_foreign_key(
                                            &references,
                                            &column_name,
                                        ));
                                    }
                                    _ => (),
                                }
                            }
                        }
                        ast::TableArg::TableConstraint(constraint) => match constraint {
                            ast::TableConstraint::PrimaryKeyConstraint(primary_key) => {
                                state
                                    .not_null_columns
                                    .extend(primary_key_columns(&primary_key));
                            }
                            ast::TableConstraint::NotNullConstraint(not_null) => {
                                if let Some(column) = not_null.column_name_ref() {
                                    state.not_null_columns.insert(column.text());
                                }
                            }
                            ast::TableConstraint::ForeignKeyConstraint(foreign_key) => {
                                foreign_keys.push(foreign_key_constraint(&foreign_key));
                            }
                            _ => (),
                        },
                        ast::TableArg::LikeClause(_) => (),
                    }
                }
                for foreign_key in foreign_keys {
                    add_foreign_key(ctx, state, foreign_key);
                }
            }
            ast::Stmt::AlterTable(alter_table) => {
                let Some(table) = alter_table
                    .table_relation_name()
                    .and_then(|relation| relation.table_name_ref())
                    .and_then(|table| table.path_ref())
                    .map(|path| path_ref_key(&path))
                else {
                    continue;
                };
                let state = tables.entry(table).or_default();
                for action in alter_table.actions() {
                    match action {
                        ast::AlterTableAction::AlterColumn(alter_column) => {
                            let Some(column) = alter_column.column_name_ref().map(|c| c.text())
                            else {
                                continue;
                            };
                            match alter_column.option() {
                                Some(ast::AlterColumnOption::SetNotNull(_)) => {
                                    set_not_null(ctx, state, column, alter_column.syntax());
                                }
                                Some(ast::AlterColumnOption::DropNotNull(_)) => {
                                    state.not_null_columns.remove(&column);
                                }
                                _ => (),
                            }
                        }
                        ast::AlterTableAction::AddColumn(add_column) => {
                            let Some(column) = add_column.column_name().map(|c| c.text()) else {
                                continue;
                            };
                            let mut foreign_keys = vec![];
                            for constraint in add_column.constraints() {
                                match constraint {
                                    ast::Constraint::NotNullConstraint(_)
                                    | ast::Constraint::PrimaryKeyConstraint(_) => {
                                        state.not_null_columns.insert(column.clone());
                                    }
                                    ast::Constraint::ReferencesConstraint(references) => {
                                        foreign_keys
                                            .push(references_foreign_key(&references, &column));
                                    }
                                    _ => (),
                                }
                            }
                            for foreign_key in foreign_keys {
                                add_foreign_key(ctx, state, foreign_key);
                            }
                        }
                        ast::AlterTableAction::AddConstraint(add_constraint) => {
                            match add_constraint.constraint() {
                                Some(ast::Constraint::ForeignKeyConstraint(foreign_key)) => {
                                    add_foreign_key(
                                        ctx,
                                        state,
                                        foreign_key_constraint(&foreign_key),
                                    );
                                }
                                Some(ast::Constraint::PrimaryKeyConstraint(primary_key)) => {
                                    for column in primary_key_columns(&primary_key) {
                                        set_not_null(ctx, state, column, primary_key.syntax());
                                    }
                                }
                                Some(ast::Constraint::NotNullConstraint(not_null)) => {
                                    if let Some(column) = not_null.column_name_ref() {
                                        set_not_null(ctx, state, column.text(), not_null.syntax());
                                    }
                                }
                                _ => (),
                            }
                        }
                        ast::AlterTableAction::DropConstraint(drop_constraint) => {
                            if let Some(name) = drop_constraint
                                .constraint_name_ref()
                                .and_then(|name| name.path_ref())
                                .and_then(|path| path.segment())
                                .map(|segment| segment.text())
                            {
                                state
                                    .foreign_keys
                                    .retain(|fk| fk.name.as_deref() != Some(name.as_str()));
                            }
                        }
                        _ => (),
                    }
                }
            }
            ast::Stmt::DropTable(drop_table) => {
                for table in drop_table.table_name_refs() {
                    if let Some(path) = table.path_ref() {
                        tables.remove(&path_ref_key(&path));
                    }
                }
            }
            _ => (),
        }
    }
}

fn path_key(path: &ast::Path) -> Vec<String> {
    let mut key: Vec<String> = path
        .segment()
        .map(|segment| segment.text())
        .into_iter()
        .collect();
    if let Some(qualifier) = path.qualifier() {
        key.extend(path_ref_key(&qualifier));
    }
    key
}

fn path_ref_key(path: &ast::PathRef) -> Vec<String> {
    std::iter::successors(Some(path.clone()), |path| path.qualifier())
        .filter_map(|path| path.segment())
        .map(|segment| segment.text())
        .collect()
}

fn primary_key_columns(primary_key: &ast::PrimaryKeyConstraint) -> Vec<String> {
    primary_key
        .index_parameters()
        .and_then(|params| params.column_list())
        .map(|list| list.column_name_refs().map(|c| c.text()).collect())
        .unwrap_or_default()
}

fn set_null_actions(
    actions: impl Iterator<Item = ast::ReferentialAction>,
    fk_columns: &[String],
) -> Vec<SetNullAction> {
    actions
        .filter_map(|referential_action| {
            let (action, ref_action) = match &referential_action {
                ast::ReferentialAction::OnDeleteAction(on_delete) => {
                    (Action::Delete, on_delete.ref_action()?)
                }
                ast::ReferentialAction::OnUpdateAction(on_update) => {
                    (Action::Update, on_update.ref_action()?)
                }
            };
            let ast::RefAction::SetNullColumns(set_null) = ref_action else {
                return None;
            };
            // `SET NULL (col, ...)` only nulls the listed columns.
            let columns = match set_null.column_ref_list() {
                Some(list) => list.column_name_refs().map(|c| c.text()).collect(),
                None => fk_columns.to_vec(),
            };
            Some(SetNullAction {
                action,
                columns,
                node: referential_action.syntax().clone(),
            })
        })
        .collect()
}

fn constraint_name(clause: Option<ast::ConstraintNameClause>) -> Option<String> {
    clause
        .and_then(|clause| clause.constraint_name())
        .map(|name| name.text())
}

fn references_foreign_key(
    references: &ast::ReferencesConstraint,
    column: &str,
) -> SetNullForeignKey {
    SetNullForeignKey {
        name: constraint_name(references.constraint_name_clause()),
        actions: set_null_actions(references.referential_actions(), &[column.to_string()]),
    }
}

fn foreign_key_constraint(foreign_key: &ast::ForeignKeyConstraint) -> SetNullForeignKey {
    let fk_columns: Vec<String> = foreign_key
        .from_columns()
        .map(|list| list.column_name_refs().map(|c| c.text()).collect())
        .unwrap_or_default();
    SetNullForeignKey {
        name: constraint_name(foreign_key.constraint_name_clause()),
        actions: set_null_actions(foreign_key.referential_actions(), &fk_columns),
    }
}

fn format_columns(columns: &[&String]) -> String {
    columns
        .iter()
        .map(|column| format!("`{column}`"))
        .collect::<Vec<_>>()
        .join(", ")
}

fn add_foreign_key(ctx: &mut Linter, state: &mut TableState, foreign_key: SetNullForeignKey) {
    if foreign_key.actions.is_empty() {
        return;
    }
    for set_null in &foreign_key.actions {
        let not_null: Vec<&String> = set_null
            .columns
            .iter()
            .filter(|column| state.not_null_columns.contains(*column))
            .collect();
        if not_null.is_empty() {
            continue;
        }
        let pronoun = if not_null.len() == 1 {
            "the column is"
        } else {
            "the columns are"
        };
        ctx.report(
            Violation::for_node(
                Rule::BanSetNullOnNotNullColumn,
                format!(
                    "`{}` would set {} to `NULL`, but {pronoun} `NOT NULL`, so {}.",
                    set_null.action.sql(),
                    format_columns(&not_null),
                    set_null.action.consequence(),
                ),
                &set_null.node,
            )
            .help(HELP),
        );
    }
    state.foreign_keys.push(foreign_key);
}

fn set_not_null(ctx: &mut Linter, state: &mut TableState, column: String, node: &SyntaxNode) {
    let mut reported: Vec<Action> = vec![];
    for foreign_key in &state.foreign_keys {
        for set_null in &foreign_key.actions {
            if reported.contains(&set_null.action) || !set_null.columns.contains(&column) {
                continue;
            }
            reported.push(set_null.action);
            ctx.report(
                Violation::for_node(
                    Rule::BanSetNullOnNotNullColumn,
                    format!(
                        "Column `{column}` has a foreign key with `{}`, so making it `NOT NULL` means {}.",
                        set_null.action.sql(),
                        set_null.action.consequence(),
                    ),
                    node,
                )
                .help(HELP),
            );
        }
    }
    state.not_null_columns.insert(column);
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::{
        Rule,
        test_utils::{lint_errors, lint_ok},
    };

    #[test]
    fn create_table_foreign_key_err() {
        let sql = r#"
CREATE TABLE messages (
    id uuid PRIMARY KEY,
    conversation_id uuid NOT NULL,
    CONSTRAINT fk_messages_conversation_id
        FOREIGN KEY (conversation_id)
        REFERENCES conversations(id) ON DELETE SET NULL
);
"#;
        assert_snapshot!(lint_errors(sql, Rule::BanSetNullOnNotNullColumn), @"
        warning[ban-set-null-on-not-null-column]: `ON DELETE SET NULL` would set `conversation_id` to `NULL`, but the column is `NOT NULL`, so deleting a referenced row will fail.
          ╭▸ 
        7 │         REFERENCES conversations(id) ON DELETE SET NULL
          │                                      ━━━━━━━━━━━━━━━━━━
          │
          ╰ help: Use `CASCADE`, `RESTRICT`, or `NO ACTION` instead of `SET NULL`, or make the column nullable.
        ");
    }

    #[test]
    fn create_table_column_references_err() {
        let sql = r#"
create table messages (
    id uuid primary key,
    conversation_id uuid not null references conversations(id) on delete set null
);
"#;
        assert_snapshot!(lint_errors(sql, Rule::BanSetNullOnNotNullColumn), @"
        warning[ban-set-null-on-not-null-column]: `ON DELETE SET NULL` would set `conversation_id` to `NULL`, but the column is `NOT NULL`, so deleting a referenced row will fail.
          ╭▸ 
        4 │     conversation_id uuid not null references conversations(id) on delete set null
          │                                                                ━━━━━━━━━━━━━━━━━━
          │
          ╰ help: Use `CASCADE`, `RESTRICT`, or `NO ACTION` instead of `SET NULL`, or make the column nullable.
        ");
    }

    #[test]
    fn create_table_on_update_set_null_err() {
        let sql = r#"
create table messages (
    conversation_id uuid not null references conversations(id) on update set null
);
"#;
        assert_snapshot!(lint_errors(sql, Rule::BanSetNullOnNotNullColumn), @"
        warning[ban-set-null-on-not-null-column]: `ON UPDATE SET NULL` would set `conversation_id` to `NULL`, but the column is `NOT NULL`, so updating a referenced key will fail.
          ╭▸ 
        3 │     conversation_id uuid not null references conversations(id) on update set null
          │                                                                ━━━━━━━━━━━━━━━━━━
          │
          ╰ help: Use `CASCADE`, `RESTRICT`, or `NO ACTION` instead of `SET NULL`, or make the column nullable.
        ");
    }

    #[test]
    fn create_table_primary_key_column_err() {
        let sql = r#"
create table tag_post (
    tag_id bigint references tag(id) on delete set null,
    post_id bigint references post(id) on delete cascade,
    primary key (tag_id, post_id)
);
"#;
        assert_snapshot!(lint_errors(sql, Rule::BanSetNullOnNotNullColumn), @"
        warning[ban-set-null-on-not-null-column]: `ON DELETE SET NULL` would set `tag_id` to `NULL`, but the column is `NOT NULL`, so deleting a referenced row will fail.
          ╭▸ 
        3 │     tag_id bigint references tag(id) on delete set null,
          │                                      ━━━━━━━━━━━━━━━━━━
          │
          ╰ help: Use `CASCADE`, `RESTRICT`, or `NO ACTION` instead of `SET NULL`, or make the column nullable.
        ");
    }

    #[test]
    fn create_table_quoted_names_err() {
        let sql = r#"
create table "messages" (
    "conversation_id" uuid not null,
    foreign key (conversation_id) references "conversations" ("id") on delete set null
);
"#;
        assert_snapshot!(lint_errors(sql, Rule::BanSetNullOnNotNullColumn), @r#"
        warning[ban-set-null-on-not-null-column]: `ON DELETE SET NULL` would set `conversation_id` to `NULL`, but the column is `NOT NULL`, so deleting a referenced row will fail.
          ╭▸ 
        4 │     foreign key (conversation_id) references "conversations" ("id") on delete set null
          │                                                                     ━━━━━━━━━━━━━━━━━━
          │
          ╰ help: Use `CASCADE`, `RESTRICT`, or `NO ACTION` instead of `SET NULL`, or make the column nullable.
        "#);
    }

    #[test]
    fn set_null_column_list_err() {
        let sql = r#"
create table messages (
    tenant_id uuid not null,
    conversation_id uuid,
    foreign key (tenant_id, conversation_id)
        references conversations (tenant_id, id) on delete set null (tenant_id)
);
"#;
        assert_snapshot!(lint_errors(sql, Rule::BanSetNullOnNotNullColumn), @"
        warning[ban-set-null-on-not-null-column]: `ON DELETE SET NULL` would set `tenant_id` to `NULL`, but the column is `NOT NULL`, so deleting a referenced row will fail.
          ╭▸ 
        6 │         references conversations (tenant_id, id) on delete set null (tenant_id)
          │                                                  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
          │
          ╰ help: Use `CASCADE`, `RESTRICT`, or `NO ACTION` instead of `SET NULL`, or make the column nullable.
        ");
    }

    #[test]
    fn alter_table_add_constraint_err() {
        let sql = r#"
create table messages (id uuid primary key, conversation_id uuid not null);
alter table messages add constraint fk_conversation
    foreign key (conversation_id) references conversations (id) on delete set null;
"#;
        assert_snapshot!(lint_errors(sql, Rule::BanSetNullOnNotNullColumn), @"
        warning[ban-set-null-on-not-null-column]: `ON DELETE SET NULL` would set `conversation_id` to `NULL`, but the column is `NOT NULL`, so deleting a referenced row will fail.
          ╭▸ 
        4 │     foreign key (conversation_id) references conversations (id) on delete set null;
          │                                                                 ━━━━━━━━━━━━━━━━━━
          │
          ╰ help: Use `CASCADE`, `RESTRICT`, or `NO ACTION` instead of `SET NULL`, or make the column nullable.
        ");
    }

    #[test]
    fn alter_table_add_column_err() {
        let sql = r#"
alter table messages add column conversation_id uuid not null
    references conversations (id) on delete set null;
"#;
        assert_snapshot!(lint_errors(sql, Rule::BanSetNullOnNotNullColumn), @"
        warning[ban-set-null-on-not-null-column]: `ON DELETE SET NULL` would set `conversation_id` to `NULL`, but the column is `NOT NULL`, so deleting a referenced row will fail.
          ╭▸ 
        3 │     references conversations (id) on delete set null;
          │                                   ━━━━━━━━━━━━━━━━━━
          │
          ╰ help: Use `CASCADE`, `RESTRICT`, or `NO ACTION` instead of `SET NULL`, or make the column nullable.
        ");
    }

    #[test]
    fn set_not_null_after_foreign_key_err() {
        let sql = r#"
create table messages (
    id uuid primary key,
    conversation_id uuid references conversations (id) on delete set null
);
alter table messages alter column conversation_id set not null;
"#;
        assert_snapshot!(lint_errors(sql, Rule::BanSetNullOnNotNullColumn), @"
        warning[ban-set-null-on-not-null-column]: Column `conversation_id` has a foreign key with `ON DELETE SET NULL`, so making it `NOT NULL` means deleting a referenced row will fail.
          ╭▸ 
        6 │ alter table messages alter column conversation_id set not null;
          │                      ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
          │
          ╰ help: Use `CASCADE`, `RESTRICT`, or `NO ACTION` instead of `SET NULL`, or make the column nullable.
        ");
    }

    #[test]
    fn set_not_null_before_foreign_key_err() {
        let sql = r#"
alter table messages alter column conversation_id set not null;
alter table messages add constraint fk_conversation
    foreign key (conversation_id) references conversations (id) on delete set null not valid;
"#;
        assert_snapshot!(lint_errors(sql, Rule::BanSetNullOnNotNullColumn), @"
        warning[ban-set-null-on-not-null-column]: `ON DELETE SET NULL` would set `conversation_id` to `NULL`, but the column is `NOT NULL`, so deleting a referenced row will fail.
          ╭▸ 
        4 │     foreign key (conversation_id) references conversations (id) on delete set null not valid;
          │                                                                 ━━━━━━━━━━━━━━━━━━
          │
          ╰ help: Use `CASCADE`, `RESTRICT`, or `NO ACTION` instead of `SET NULL`, or make the column nullable.
        ");
    }

    #[test]
    fn nullable_column_ok() {
        let sql = r#"
create table messages (
    id uuid primary key,
    conversation_id uuid,
    constraint fk_messages_conversation_id
        foreign key (conversation_id)
        references conversations(id) on delete set null
);
create table replies (
    message_id uuid null references messages(id) on delete set null
);
"#;
        lint_ok(sql, Rule::BanSetNullOnNotNullColumn);
    }

    #[test]
    fn not_null_with_other_actions_ok() {
        let sql = r#"
create table messages (
    a uuid not null references t(id) on delete cascade,
    b uuid not null references t(id) on delete restrict on update cascade,
    c uuid not null references t(id) on delete no action,
    d uuid not null references t(id) on delete set default,
    e uuid not null references t(id)
);
"#;
        lint_ok(sql, Rule::BanSetNullOnNotNullColumn);
    }

    #[test]
    fn set_null_column_list_excludes_not_null_ok() {
        let sql = r#"
create table messages (
    tenant_id uuid not null,
    conversation_id uuid,
    foreign key (tenant_id, conversation_id)
        references conversations (tenant_id, id) on delete set null (conversation_id)
);
"#;
        lint_ok(sql, Rule::BanSetNullOnNotNullColumn);
    }

    #[test]
    fn drop_not_null_ok() {
        let sql = r#"
create table messages (conversation_id uuid not null);
alter table messages alter column conversation_id drop not null;
alter table messages add foreign key (conversation_id) references conversations (id) on delete set null;
"#;
        lint_ok(sql, Rule::BanSetNullOnNotNullColumn);
    }

    #[test]
    fn drop_constraint_before_set_not_null_ok() {
        let sql = r#"
create table messages (
    conversation_id uuid,
    constraint fk_conversation foreign key (conversation_id)
        references conversations (id) on delete set null
);
alter table messages drop constraint fk_conversation;
alter table messages alter column conversation_id set not null;
"#;
        lint_ok(sql, Rule::BanSetNullOnNotNullColumn);
    }

    #[test]
    fn different_table_ok() {
        let sql = r#"
create table a (conversation_id uuid not null);
create table b (conversation_id uuid references conversations (id) on delete set null);
alter table c alter column conversation_id set not null;
"#;
        lint_ok(sql, Rule::BanSetNullOnNotNullColumn);
    }

    #[test]
    fn recreated_table_ok() {
        let sql = r#"
create table messages (conversation_id uuid not null);
drop table messages;
create table messages (
    conversation_id uuid references conversations (id) on delete set null
);
"#;
        lint_ok(sql, Rule::BanSetNullOnNotNullColumn);
    }
}
