use squawk_diagnostics::Edit;
use squawk_rowan::TextSize;
use squawk_sema::file::InFile;
use squawk_sema::ir::{Item, Resolution};
use squawk_syntax::ast::{self, AstNode};
use squawk_syntax::quote::quote_ident;

use squawk_ide_db::offsets::token_from_offset;
use squawk_ide_db::{Db, semantic_index};

use super::{ActionKind, CodeAction};

pub(super) fn qualify_column(
    db: &dyn Db,
    position: InFile<TextSize>,
    actions: &mut Vec<CodeAction>,
) -> Option<()> {
    let token = token_from_offset(db, position)?;
    let name_ref = token.parent().and_then(ast::NameRef::cast)?;

    let parent = name_ref.syntax().parent()?;
    if ast::PartitionItem::can_cast(parent.kind())
        || ast::ConflictIndexItem::can_cast(parent.kind())
    {
        return None;
    }
    if let Some(field_expr) = ast::FieldExpr::cast(parent)
        && field_expr.field().as_ref() == Some(&name_ref)
    {
        return None;
    }

    let index = semantic_index(db, position.file_id);
    let range = name_ref.syntax().text_range();
    let Item::Reference(reference) = index.item_at_range(position.with_value(range))?.value.item
    else {
        return None;
    };
    let Resolution::Column {
        source: Some(source),
        ..
    } = index.references[reference].resolution
    else {
        return None;
    };
    let source_name = index.sources[source].name?;
    let qualifier = format!("{}.", quote_ident(index.names.resolve(source_name)));

    actions.push(CodeAction {
        title: "Qualify column".to_owned(),
        edits: vec![Edit::insert(qualifier, range.start())],
        kind: ActionKind::RefactorRewrite,
    });

    Some(())
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::test_utils::{apply_code_action, code_action_not_applicable};

    use super::qualify_column;

    #[test]
    fn qualify_column_simple() {
        assert_snapshot!(apply_code_action(
            qualify_column,
            "
create table t (
  a bigint
);

select a$0 from t;
"),
            @"
        create table t (
          a bigint
        );

        select t.a from t;
        "
        );
    }

    #[test]
    fn qualify_column_alias() {
        assert_snapshot!(apply_code_action(
            qualify_column,
            "
create table t (
  a bigint
);

select a$0 from t as u;
"),
            @"
        create table t (
          a bigint
        );

        select u.a from t as u;
        "
        );
    }

    #[test]
    fn qualify_column_schema_qualified_table() {
        assert_snapshot!(apply_code_action(
            qualify_column,
            "
create table public.t (
  a bigint
);

select a$0 from public.t;
"),
            @"
        create table public.t (
          a bigint
        );

        select t.a from public.t;
        "
        );
    }

    #[test]
    fn qualify_column_quoted_alias() {
        assert_snapshot!(apply_code_action(
            qualify_column,
            r#"
create table t (
  a bigint
);

select a$0 from t as "Foo";
"#),
            @r#"
        create table t (
          a bigint
        );

        select "Foo".a from t as "Foo";
        "#
        );
    }

    #[test]
    fn qualify_column_where() {
        assert_snapshot!(apply_code_action(
            qualify_column,
            "
create table t (
  a bigint
);

select 1 from t where a$0 > 1;
"),
            @"
        create table t (
          a bigint
        );

        select 1 from t where t.a > 1;
        "
        );
    }

    #[test]
    fn qualify_column_join() {
        assert_snapshot!(apply_code_action(
            qualify_column,
            "
create table t (
  a bigint
);
create table u (
  b bigint
);

select b$0 from t join u on true;
"),
            @"
        create table t (
          a bigint
        );
        create table u (
          b bigint
        );

        select u.b from t join u on true;
        "
        );
    }

    #[test]
    fn qualify_column_subquery() {
        assert_snapshot!(apply_code_action(
            qualify_column,
            "
create table t (
  a bigint
);

select a$0 from (select a from t) as s;
"),
            @"
        create table t (
          a bigint
        );

        select s.a from (select a from t) as s;
        "
        );
    }

    #[test]
    fn qualify_column_cte() {
        assert_snapshot!(apply_code_action(
            qualify_column,
            "
create table t (
  a bigint
);

with c as (select a from t)
select a$0 from c;
"),
            @"
        create table t (
          a bigint
        );

        with c as (select a from t)
        select c.a from c;
        "
        );
    }

    #[test]
    fn qualify_column_not_applicable_already_qualified() {
        assert!(code_action_not_applicable(
            qualify_column,
            "
create table t (
  a bigint
);

select t.a$0 from t;
"
        ));
    }

    #[test]
    fn qualify_column_not_applicable_table() {
        assert!(code_action_not_applicable(
            qualify_column,
            "
create table t (
  a bigint
);

select a from t$0;
"
        ));
    }

    #[test]
    fn qualify_column_not_applicable_unresolved() {
        assert!(code_action_not_applicable(
            qualify_column,
            "select a$0 from t;"
        ));
    }

    #[test]
    fn qualify_column_not_applicable_using_join() {
        assert!(code_action_not_applicable(
            qualify_column,
            "
create table t (
  a bigint
);
create table u (
  a bigint
);

select a$0 from t join u using (a);
"
        ));
    }

    #[test]
    fn qualify_column_not_applicable_insert_column_list() {
        assert!(code_action_not_applicable(
            qualify_column,
            "
create table t (
  a bigint
);

insert into t (a$0) values (1);
"
        ));
    }

    #[test]
    fn qualify_column_not_applicable_update_set() {
        assert!(code_action_not_applicable(
            qualify_column,
            "
create table t (
  a bigint
);

update t set a$0 = 1;
"
        ));
    }

    #[test]
    fn qualify_column_index_where() {
        assert_snapshot!(apply_code_action(
            qualify_column,
            "
create table t (
  a bigint
);

create index on t (a) where a$0 > 1;
"),
            @"
        create table t (
          a bigint
        );

        create index on t (a) where t.a > 1;
        "
        );
    }

    #[test]
    fn qualify_column_returning() {
        assert_snapshot!(apply_code_action(
            qualify_column,
            "
create table t (
  a bigint
);

delete from t returning a$0;
"),
            @"
        create table t (
          a bigint
        );

        delete from t returning t.a;
        "
        );
    }

    #[test]
    fn qualify_column_not_applicable_on_conflict() {
        assert!(code_action_not_applicable(
            qualify_column,
            "
create table t (
  a bigint
);

insert into t values (1) on conflict (a$0) do nothing;
"
        ));
    }

    #[test]
    fn qualify_column_not_applicable_create_index() {
        assert!(code_action_not_applicable(
            qualify_column,
            "
create table t (
  a bigint
);

create index on t (a$0);
"
        ));
    }
}
