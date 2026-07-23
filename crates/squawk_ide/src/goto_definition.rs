use crate::db::{list_files, parse};
use crate::file::InFile;
use crate::location::{Location, LocationKind};
use crate::offsets::token_from_offset;
use crate::resolve;
use rowan::{TextRange, TextSize};
use salsa::Database as Db;
use smallvec::{SmallVec, smallvec};
use squawk_syntax::{
    SyntaxKind,
    ast::{self, AstNode},
};

pub fn goto_definition(db: &dyn Db, position: InFile<TextSize>) -> SmallVec<[Location; 1]> {
    let file = position.file_id;
    let Some(token) = token_from_offset(db, position) else {
        return smallvec![];
    };
    let Some(parent) = token.parent() else {
        return smallvec![];
    };

    // goto def on case exprs
    if (token.kind() == SyntaxKind::WHEN_KW && parent.kind() == SyntaxKind::WHEN_CLAUSE)
        || (token.kind() == SyntaxKind::ELSE_KW && parent.kind() == SyntaxKind::ELSE_CLAUSE)
        || (token.kind() == SyntaxKind::END_KW && parent.kind() == SyntaxKind::CASE_EXPR)
    {
        for parent in token.parent_ancestors() {
            if let Some(case_expr) = ast::CaseExpr::cast(parent)
                && let Some(case_token) = case_expr.case_token()
            {
                return smallvec![Location::new(
                    file,
                    case_token.text_range(),
                    LocationKind::CaseExpr
                )];
            }
        }
    }

    // goto def on COMMIT -> BEGIN/START TRANSACTION
    if ast::Commit::can_cast(parent.kind())
        && let Some(begin_range) = find_preceding_begin(db, position)
    {
        return smallvec![Location::new(file, begin_range, LocationKind::CommitBegin)];
    }

    // goto def on ROLLBACK -> BEGIN/START TRANSACTION
    if ast::Rollback::can_cast(parent.kind())
        && let Some(begin_range) = find_preceding_begin(db, position)
    {
        return smallvec![Location::new(file, begin_range, LocationKind::CommitBegin)];
    }

    // goto def on BEGIN/START TRANSACTION -> COMMIT or ROLLBACK
    if ast::Begin::can_cast(parent.kind())
        && let Some(end_range) = find_following_commit_or_rollback(db, position)
    {
        return smallvec![Location::new(file, end_range, LocationKind::CommitEnd)];
    }

    if let Some(name) = ast::Name::cast(parent.clone())
        && let Some(location) = Location::from_node(file, name.syntax())
    {
        return smallvec![location];
    }

    if (ast::AccessMethod::can_cast(parent.kind())
        || ast::Channel::can_cast(parent.kind())
        || ast::ColumnName::can_cast(parent.kind())
        || ast::ConstraintName::can_cast(parent.kind())
        || ast::CteName::can_cast(parent.kind())
        || ast::Cursor::can_cast(parent.kind())
        || ast::Database::can_cast(parent.kind())
        || ast::EventTrigger::can_cast(parent.kind())
        || ast::Extension::can_cast(parent.kind())
        || ast::ForeignDataWrapper::can_cast(parent.kind())
        || ast::JsonPathName::can_cast(parent.kind())
        || ast::Language::can_cast(parent.kind())
        || ast::ParamName::can_cast(parent.kind())
        || ast::Policy::can_cast(parent.kind())
        || ast::PreparedStatement::can_cast(parent.kind())
        || ast::Publication::can_cast(parent.kind())
        || ast::Role::can_cast(parent.kind())
        || ast::Rule::can_cast(parent.kind())
        || ast::Savepoint::can_cast(parent.kind())
        || ast::Schema::can_cast(parent.kind())
        || ast::Server::can_cast(parent.kind())
        || ast::Subscription::can_cast(parent.kind())
        || ast::TableAlias::can_cast(parent.kind())
        || ast::Tablespace::can_cast(parent.kind())
        || ast::TransitionRelationName::can_cast(parent.kind())
        || ast::Trigger::can_cast(parent.kind())
        || ast::Window::can_cast(parent.kind()))
        && let Some(location) = Location::from_node(file, &parent)
    {
        return smallvec![location];
    }

    if let Some(access_method_ref) = ast::AccessMethodRef::cast(parent.clone()) {
        for definition_file in list_files(db, file) {
            if let Some(locations) = resolve::resolve_access_method_ref(
                db,
                InFile::new(definition_file, &access_method_ref),
            ) {
                return locations;
            }
        }
    }

    if let Some(channel_ref) = ast::ChannelRef::cast(parent.clone()) {
        for definition_file in list_files(db, file) {
            if let Some(locations) =
                resolve::resolve_channel_ref(db, InFile::new(definition_file, &channel_ref))
            {
                return locations;
            }
        }
    }

    if let Some(cursor_ref) = ast::CursorRef::cast(parent.clone()) {
        for definition_file in list_files(db, file) {
            if let Some(locations) =
                resolve::resolve_cursor_ref(db, InFile::new(definition_file, &cursor_ref))
            {
                return locations;
            }
        }
    }

    if let Some(event_trigger_ref) = ast::EventTriggerRef::cast(parent.clone()) {
        for definition_file in list_files(db, file) {
            if let Some(locations) = resolve::resolve_event_trigger_ref(
                db,
                InFile::new(definition_file, &event_trigger_ref),
            ) {
                return locations;
            }
        }
    }

    if let Some(extension_ref) = ast::ExtensionRef::cast(parent.clone()) {
        for definition_file in list_files(db, file) {
            if let Some(locations) =
                resolve::resolve_extension_ref(db, InFile::new(definition_file, &extension_ref))
            {
                return locations;
            }
        }
    }

    if let Some(foreign_data_wrapper_ref) = ast::ForeignDataWrapperRef::cast(parent.clone()) {
        for definition_file in list_files(db, file) {
            if let Some(locations) = resolve::resolve_foreign_data_wrapper_ref(
                db,
                InFile::new(definition_file, &foreign_data_wrapper_ref),
            ) {
                return locations;
            }
        }
    }

    if let Some(json_path_name_ref) = ast::JsonPathNameRef::cast(parent.clone()) {
        return resolve::resolve_json_path_name_ref(file, &json_path_name_ref).unwrap_or_default();
    }

    if let Some(param_name_ref) = ast::ParamNameRef::cast(parent.clone()) {
        for definition_file in list_files(db, file) {
            if let Some(locations) =
                resolve::resolve_param_name_ref(db, InFile::new(definition_file, &param_name_ref))
            {
                return locations;
            }
        }
    }

    if let Some(column_name_ref) = ast::ColumnNameRef::cast(parent.clone()) {
        for definition_file in list_files(db, file) {
            if let Some(locations) =
                resolve::resolve_column_name_ref(db, InFile::new(definition_file, &column_name_ref))
            {
                return locations;
            }
        }
    }

    if let Some(database_ref) = ast::DatabaseRef::cast(parent.clone()) {
        for definition_file in list_files(db, file) {
            if let Some(locations) =
                resolve::resolve_database_ref(db, InFile::new(definition_file, &database_ref))
            {
                return locations;
            }
        }
    }

    if let Some(language_ref) = ast::LanguageRef::cast(parent.clone()) {
        for definition_file in list_files(db, file) {
            if let Some(locations) =
                resolve::resolve_language_ref(db, InFile::new(definition_file, &language_ref))
            {
                return locations;
            }
        }
    }

    if let Some(policy_ref) = ast::PolicyRef::cast(parent.clone()) {
        for definition_file in list_files(db, file) {
            if let Some(locations) =
                resolve::resolve_policy_ref(db, InFile::new(definition_file, &policy_ref))
            {
                return locations;
            }
        }
    }

    if let Some(publication_ref) = ast::PublicationRef::cast(parent.clone()) {
        for definition_file in list_files(db, file) {
            if let Some(locations) =
                resolve::resolve_publication_ref(db, InFile::new(definition_file, &publication_ref))
            {
                return locations;
            }
        }
    }

    if let Some(role_ref) = ast::RoleRef::cast(parent.clone()) {
        for definition_file in list_files(db, file) {
            if let Some(locations) =
                resolve::resolve_role_ref(db, InFile::new(definition_file, &role_ref))
            {
                return locations;
            }
        }
    }

    if let Some(rule_ref) = ast::RuleRef::cast(parent.clone()) {
        for definition_file in list_files(db, file) {
            if let Some(locations) =
                resolve::resolve_rule_ref(db, InFile::new(definition_file, &rule_ref))
            {
                return locations;
            }
        }
    }

    if let Some(schema_ref) = ast::SchemaRef::cast(parent.clone()) {
        for definition_file in list_files(db, file) {
            if let Some(locations) =
                resolve::resolve_schema_ref(db, InFile::new(definition_file, &schema_ref))
            {
                return locations;
            }
        }
    }

    if let Some(server_ref) = ast::ServerRef::cast(parent.clone()) {
        for definition_file in list_files(db, file) {
            if let Some(locations) =
                resolve::resolve_server_ref(db, InFile::new(definition_file, &server_ref))
            {
                return locations;
            }
        }
    }

    if let Some(statement_ref) = ast::PreparedStatementRef::cast(parent.clone()) {
        for definition_file in list_files(db, file) {
            if let Some(locations) = resolve::resolve_prepared_statement_ref(
                db,
                InFile::new(definition_file, &statement_ref),
            ) {
                return locations;
            }
        }
    }

    if let Some(savepoint_ref) = ast::SavepointRef::cast(parent.clone()) {
        for definition_file in list_files(db, file) {
            if let Some(locations) =
                resolve::resolve_savepoint_ref(db, InFile::new(definition_file, &savepoint_ref))
            {
                return locations;
            }
        }
    }

    if let Some(subscription_ref) = ast::SubscriptionRef::cast(parent.clone()) {
        for definition_file in list_files(db, file) {
            if let Some(locations) = resolve::resolve_subscription_ref(
                db,
                InFile::new(definition_file, &subscription_ref),
            ) {
                return locations;
            }
        }
    }

    if let Some(tablespace_ref) = ast::TablespaceRef::cast(parent.clone()) {
        for definition_file in list_files(db, file) {
            if let Some(locations) =
                resolve::resolve_tablespace_ref(db, InFile::new(definition_file, &tablespace_ref))
            {
                return locations;
            }
        }
    }

    if let Some(trigger_ref) = ast::TriggerRef::cast(parent.clone()) {
        for definition_file in list_files(db, file) {
            if let Some(locations) =
                resolve::resolve_trigger_ref(db, InFile::new(definition_file, &trigger_ref))
            {
                return locations;
            }
        }
    }

    if let Some(vertex_table_ref) = ast::VertexTableRef::cast(parent.clone()) {
        for definition_file in list_files(db, file) {
            if let Some(locations) = resolve::resolve_vertex_table_ref(
                db,
                InFile::new(definition_file, &vertex_table_ref),
            ) {
                return locations;
            }
        }
    }

    if let Some(window_ref) = ast::WindowRef::cast(parent.clone()) {
        return resolve::resolve_window_ref(file, &window_ref).unwrap_or_default();
    }

    if let Some(config_value_name) = ast::ConfigValueName::cast(parent.clone()) {
        for definition_file in list_files(db, file) {
            if let Some(locations) = resolve::resolve_config_value_name(
                db,
                InFile::new(definition_file, &config_value_name),
            ) {
                return locations;
            }
        }
    }

    if let Some(name_ref) = ast::NameRef::cast(parent.clone()) {
        for definition_file in list_files(db, file) {
            if let Some(locations) =
                // TODO: we shouldn't be wrapping name_ref like this since it's
                // a different file. Probably a bug.
                resolve::resolve_name_ref(db, InFile::new(definition_file, &name_ref))
            {
                return locations;
            }
        }
    }

    if let Some(literal) = ast::Literal::cast(parent.clone()) {
        for definition_file in list_files(db, file) {
            if let Some(locations) =
                resolve::resolve_literal(db, InFile::new(definition_file, &literal))
            {
                return locations;
            }
        }
    }

    if let Some(custom_op) = ast::CustomOp::cast(parent.clone()) {
        for definition_file in list_files(db, file) {
            if let Some(locations) =
                resolve::resolve_custom_op(db, InFile::new(definition_file, &custom_op))
            {
                return locations;
            }
        }
    }

    let type_node = ast::Type::cast(parent.clone()).or_else(|| {
        // special case if we're at the timezone clause inside a timezone type
        if ast::Timezone::can_cast(parent.kind()) {
            parent.parent().and_then(ast::Type::cast)
        } else {
            None
        }
    });
    if let Some(ty) = type_node {
        for definition_file in list_files(db, file) {
            if let Some(ptr) =
                // TODO: we shouldn't be wrapping name_ref like this since it's
                // a different file. Probably a bug.
                resolve::resolve_type_ptr_from_type(db, InFile::new(definition_file, &ty))
            {
                return smallvec![Location {
                    file: definition_file,
                    range: ptr.text_range(),
                    kind: LocationKind::Type,
                }];
            }
        }
    }

    smallvec![]
}

fn find_preceding_begin(db: &dyn Db, position: InFile<TextSize>) -> Option<TextRange> {
    let mut last_begin: Option<TextRange> = None;
    for stmt in parse(db, position.file_id).tree().stmts() {
        if let ast::Stmt::Begin(begin) = stmt {
            let range = begin.syntax().text_range();
            if range.end() <= position.value {
                last_begin = Some(range);
            }
        }
    }
    last_begin
}

fn find_following_commit_or_rollback(db: &dyn Db, position: InFile<TextSize>) -> Option<TextRange> {
    for stmt in parse(db, position.file_id).tree().stmts() {
        let range = match &stmt {
            ast::Stmt::Commit(commit) => commit.syntax().text_range(),
            ast::Stmt::Rollback(rollback) => rollback.syntax().text_range(),
            _ => continue,
        };
        if range.start() >= position.value {
            return Some(range);
        }
    }
    None
}

#[cfg(test)]
mod test {
    use crate::builtins::builtins_file;
    use crate::db::File;

    use crate::goto_definition::goto_definition;
    use crate::test_utils::Fixture;
    use annotate_snippets::{AnnotationKind, Level, Renderer, Snippet, renderer::DecorStyle};
    use insta::assert_snapshot;
    use rowan::TextRange;
    use rustc_hash::FxHashMap;

    #[must_use]
    #[track_caller]
    fn goto(sql: &str) -> String {
        goto_(sql).expect("should always find a definition")
    }

    #[track_caller]
    fn goto_(sql: &str) -> Option<String> {
        let fixture = Fixture::new(sql);
        // For go to def we want the previous character since we usually put the
        // marker after the item we're trying to go to def on.
        let marker = fixture.marker();
        let offset = marker.offset_before();
        let source_span = marker.range();
        let db = fixture.db();
        let current_file = offset.file_id;

        let results = goto_definition(db, offset);
        if results.is_empty() {
            return None;
        }

        let mut file_paths = FxHashMap::default();
        file_paths.insert(current_file, "current.sql");
        file_paths.insert(builtins_file(db), "builtins.sql");

        let mut dests_by_file: FxHashMap<File, Vec<(usize, TextRange)>> = FxHashMap::default();
        for (i, location) in results.iter().enumerate() {
            dests_by_file
                .entry(location.file)
                .or_default()
                .push((i + 2, location.range));
        }

        let multi_file = dests_by_file.len() > 1 || !dests_by_file.contains_key(&current_file);

        let mut snippet = Snippet::source(current_file.content(db).as_ref()).fold(true);
        if multi_file {
            snippet = snippet.path(*file_paths.get(&current_file).unwrap());
        }
        if let Some(current_dests) = dests_by_file.remove(&current_file) {
            snippet = annotate_destinations(snippet, current_dests);
        }
        snippet = snippet.annotation(AnnotationKind::Context.span(source_span).label("1. source"));

        let mut groups = vec![Level::INFO.primary_title("definition").element(snippet)];

        for (dest_file, dests) in dests_by_file {
            let path = file_paths.get(&dest_file).unwrap();
            let other_snippet = Snippet::source(dest_file.content(db).as_ref())
                .path(*path)
                .fold(true);
            let other_snippet = annotate_destinations(other_snippet, dests);
            groups.push(
                Level::INFO
                    .primary_title("definition")
                    .element(other_snippet),
            );
        }

        let renderer = Renderer::plain().decor_style(DecorStyle::Unicode);
        Some(
            renderer
                .render(&groups)
                .to_string()
                // hacky cleanup to make the text shorter
                .replace("info: definition", ""),
        )
    }

    fn goto_not_found(sql: &str) {
        assert!(goto_(sql).is_none(), "Should not find a definition");
    }

    fn annotate_destinations<'a>(
        mut snippet: Snippet<'a, annotate_snippets::Annotation<'a>>,
        destinations: Vec<(usize, TextRange)>,
    ) -> Snippet<'a, annotate_snippets::Annotation<'a>> {
        for (label_index, range) in destinations {
            snippet = snippet.annotation(
                AnnotationKind::Context
                    .span(range.into())
                    .label(format!("{label_index}. destination")),
            );
        }

        snippet
    }

    #[test]
    fn goto_case_when() {
        assert_snapshot!(goto("
select case when$0 x > 1 then 1 else 2 end;
"), @r"
          ╭▸ 
        2 │ select case when x > 1 then 1 else 2 end;
          │        ┬───    ─ 1. source
          │        │
          ╰╴       2. destination
        ");
    }

    #[test]
    fn goto_case_else() {
        assert_snapshot!(goto("
select case when x > 1 then 1 else$0 2 end;
"), @r"
          ╭▸ 
        2 │ select case when x > 1 then 1 else 2 end;
          ╰╴       ──── 2. destination       ─ 1. source
        ");
    }

    #[test]
    fn goto_case_end() {
        assert_snapshot!(goto("
select case when x > 1 then 1 else 2 end$0;
"), @r"
          ╭▸ 
        2 │ select case when x > 1 then 1 else 2 end;
          ╰╴       ──── 2. destination             ─ 1. source
        ");
    }

    #[test]
    fn goto_case_end_trailing_semi() {
        assert_snapshot!(goto("
select case when x > 1 then 1 else 2 end;$0
"), @r"
          ╭▸ 
        2 │ select case when x > 1 then 1 else 2 end;
          ╰╴       ──── 2. destination              ─ 1. source
        ");
    }

    #[test]
    fn goto_case_then_not_found() {
        goto_not_found(
            "
select case when x > 1 then$0 1 else 2 end;
",
        )
    }

    #[test]
    fn rollback_to_begin() {
        assert_snapshot!(goto(
            "
begin;
select 1;
rollback$0;
",
        ), @"
          ╭▸ 
        2 │ begin;
          │ ────── 2. destination
        3 │ select 1;
        4 │ rollback;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_table() {
        assert_snapshot!(goto("
create table t();
drop table t$0;
"), @r"
          ╭▸ 
        2 │ create table t();
          │              ─ 2. destination
        3 │ drop table t;
          ╰╴           ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_foreign_table() {
        assert_snapshot!(goto("
create foreign table t(a int) server s;
drop foreign table t$0;
"), @r"
          ╭▸ 
        2 │ create foreign table t(a int) server s;
          │                      ─ 2. destination
        3 │ drop foreign table t;
          ╰╴                   ─ 1. source
        ");
    }

    #[test]
    fn goto_definition_prefers_previous_token() {
        assert_snapshot!(goto("
create table t(a int);
select t.$0a from t;
"), @r"
          ╭▸ 
        2 │ create table t(a int);
          │              ─ 2. destination
        3 │ select t.a from t;
          ╰╴        ─ 1. source
        ");

        assert_snapshot!(goto("
create type ty as (a int, b int);
with t as (select '(1,2)'::ty c)
select (c)$0.a from t;
"), @r"
          ╭▸ 
        3 │ with t as (select '(1,2)'::ty c)
          │                               ─ 2. destination
        4 │ select (c).a from t;
          ╰╴         ─ 1. source
        ");
        assert_snapshot!(goto("
create function f() returns int as 'select 1' language sql;
select f($0);
"), @r"
          ╭▸ 
        2 │ create function f() returns int as 'select 1' language sql;
          │                 ─ 2. destination
        3 │ select f();
          ╰╴        ─ 1. source
        ");

        assert_snapshot!(goto("
with t as (select array[1,2,3]::int[] c)
select c[$01] from t;
"), @r"
          ╭▸ 
        2 │ with t as (select array[1,2,3]::int[] c)
          │                                       ─ 2. destination
        3 │ select c[1] from t;
          ╰╴        ─ 1. source
        ");

        assert_snapshot!(goto("
with t as (select array[1,2,3]::int[] c, 1 b)
select c[b]$0 from t;
"), @r"
          ╭▸ 
        2 │ with t as (select array[1,2,3]::int[] c, 1 b)
          │                                            ─ 2. destination
        3 │ select c[b] from t;
          ╰╴          ─ 1. source
        ");
    }

    #[test]
    fn goto_fetch_cursor() {
        assert_snapshot!(goto("
declare c scroll cursor for select * from t;
fetch forward 5 from c$0;
"), @r"
          ╭▸ 
        2 │ declare c scroll cursor for select * from t;
          │         ─ 2. destination
        3 │ fetch forward 5 from c;
          ╰╴                     ─ 1. source
        ");
    }

    #[test]
    fn goto_close_cursor() {
        assert_snapshot!(goto("
declare c scroll cursor for select * from t;
close c$0;
"), @r"
          ╭▸ 
        2 │ declare c scroll cursor for select * from t;
          │         ─ 2. destination
        3 │ close c;
          ╰╴      ─ 1. source
        ");
    }

    #[test]
    fn goto_move_cursor() {
        assert_snapshot!(goto("
declare c scroll cursor for select * from t;
move forward 10 from c$0;
"), @r"
          ╭▸ 
        2 │ declare c scroll cursor for select * from t;
          │         ─ 2. destination
        3 │ move forward 10 from c;
          ╰╴                     ─ 1. source
        ");
    }

    #[test]
    fn goto_execute_prepared_statement() {
        assert_snapshot!(goto("
prepare stmt as select 1;
execute stmt$0;
"), @r"
          ╭▸ 
        2 │ prepare stmt as select 1;
          │         ──── 2. destination
        3 │ execute stmt;
          ╰╴           ─ 1. source
        ");
    }

    #[test]
    fn goto_deallocate_prepared_statement() {
        assert_snapshot!(goto("
prepare stmt as select 1;
deallocate stmt$0;
"), @r"
          ╭▸ 
        2 │ prepare stmt as select 1;
          │         ──── 2. destination
        3 │ deallocate stmt;
          ╰╴              ─ 1. source
        ");
    }

    #[test]
    fn goto_notify_channel() {
        assert_snapshot!(goto("
listen updates;
notify updates$0;
"), @r"
          ╭▸ 
        2 │ listen updates;
          │        ─────── 2. destination
        3 │ notify updates;
          ╰╴             ─ 1. source
        ");
    }

    #[test]
    fn goto_unlisten_channel() {
        assert_snapshot!(goto("
listen updates;
unlisten updates$0;
"), @r"
          ╭▸ 
        2 │ listen updates;
          │        ─────── 2. destination
        3 │ unlisten updates;
          ╰╴               ─ 1. source
        ");
    }

    #[test]
    fn goto_rollback_to_savepoint() {
        assert_snapshot!(goto("
begin;
savepoint sp;
rollback to savepoint sp$0;
"), @"
          ╭▸ 
        3 │ savepoint sp;
          │           ── 2. destination
        4 │ rollback to savepoint sp;
          ╰╴                       ─ 1. source
        ");
    }

    #[test]
    fn goto_release_savepoint() {
        assert_snapshot!(goto("
begin;
savepoint sp;
release savepoint sp$0;
"), @"
          ╭▸ 
        3 │ savepoint sp;
          │           ── 2. destination
        4 │ release savepoint sp;
          ╰╴                   ─ 1. source
        ");
    }

    #[test]
    fn goto_delete_where_current_of_cursor() {
        assert_snapshot!(goto("
declare c scroll cursor for select * from t;
delete from t where current of c$0;
"), @r"
          ╭▸ 
        2 │ declare c scroll cursor for select * from t;
          │         ─ 2. destination
        3 │ delete from t where current of c;
          ╰╴                               ─ 1. source
        ");
    }

    #[test]
    fn goto_update_where_current_of_cursor() {
        assert_snapshot!(goto("
declare c scroll cursor for select * from t;
update t set a = a + 10 where current of c$0;
"), @r"
          ╭▸ 
        2 │ declare c scroll cursor for select * from t;
          │         ─ 2. destination
        3 │ update t set a = a + 10 where current of c;
          ╰╴                                         ─ 1. source
        ");
    }

    #[test]
    fn goto_with_table_star() {
        assert_snapshot!(goto("
with t as (select 1 a)
select t$0.* from t;
"), @r"
          ╭▸ 
        2 │ with t as (select 1 a)
          │      ─ 2. destination
        3 │ select t.* from t;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_shadowed_table_star_column_count_not_found() {
        goto_not_found(
            "
create table t(a int, b int);
with
  t as (
    select 1
  ),
  -- yy overrides y since there's only 1 column in the *
  u(x, yy) as (
    select *, 2 y, 3 z from t
  )
select y$0 from u;
",
        );
    }

    #[test]
    fn goto_cross_join_func_column() {
        assert_snapshot!(goto(r#"
with t(x) as (select $$[{"a":1,"b":2}]$$::json)
select * from t, json_to_recordset(x$0) as r(a int, b int);
"#), @r#"
          ╭▸ 
        2 │ with t(x) as (select $$[{"a":1,"b":2}]$$::json)
          │        ─ 2. destination
        3 │ select * from t, json_to_recordset(x) as r(a int, b int);
          ╰╴                                   ─ 1. source
        "#);
    }

    #[test]
    fn goto_cross_join_func_qualified_column_table() {
        assert_snapshot!(goto(r#"
with t(x) as (select $$[{"a":1,"b":2}]$$::json)
select * from t, json_to_recordset(t$0.x) as r(a int, b int);
"#), @r#"
          ╭▸ 
        2 │ with t(x) as (select $$[{"a":1,"b":2}]$$::json)
          │      ─ 2. destination
        3 │ select * from t, json_to_recordset(t.x) as r(a int, b int);
          ╰╴                                   ─ 1. source
        "#);
    }

    #[test]
    fn goto_cross_join_func_qualified_column_field() {
        assert_snapshot!(goto(r#"
with t(x) as (select $$[{"a":1,"b":2}]$$::json)
select * from t, json_to_recordset(t.x$0) as r(a int, b int);
"#), @r#"
          ╭▸ 
        2 │ with t(x) as (select $$[{"a":1,"b":2}]$$::json)
          │        ─ 2. destination
        3 │ select * from t, json_to_recordset(t.x) as r(a int, b int);
          ╰╴                                     ─ 1. source
        "#);
    }

    #[test]
    fn goto_lateral_values_alias_in_subquery() {
        assert_snapshot!(goto("
select u.n, x.val
from (values (1), (2)) u(n)
cross join lateral (select u$0.n * 10 as val) x;
"), @r"
          ╭▸ 
        3 │ from (values (1), (2)) u(n)
          │                        ─ 2. destination
        4 │ cross join lateral (select u.n * 10 as val) x;
          ╰╴                           ─ 1. source
        ");
    }

    #[test]
    fn goto_correlated_subquery_outer_column() {
        assert_snapshot!(goto("
create table foo (id int);
create table bar (fid int);
select * from bar b where exists (select 1 from foo where foo.id = b.fid$0);
"), @"
          ╭▸ 
        3 │ create table bar (fid int);
          │                   ─── 2. destination
        4 │ select * from bar b where exists (select 1 from foo where foo.id = b.fid);
          ╰╴                                                                       ─ 1. source
        ");
    }

    #[test]
    fn goto_update_set_correlated_subquery_column() {
        assert_snapshot!(goto("create table foo(a int, b int); update foo set a = (select b$0);"), @"
          ╭▸ 
        1 │ create table foo(a int, b int); update foo set a = (select b);
          ╰╴                        ─ 2. destination                   ─ 1. source
        ");
    }

    #[test]
    fn goto_delete_where_correlated_subquery_column() {
        assert_snapshot!(goto("create table foo(a int, b int); delete from foo where a = (select b$0);"), @"
          ╭▸ 
        1 │ create table foo(a int, b int); delete from foo where a = (select b);
          ╰╴                        ─ 2. destination                          ─ 1. source
        ");
    }

    #[test]
    fn goto_multi_level_nested_select_outer_column() {
        assert_snapshot!(goto("create table foo(a int); select (select (select a$0)) from foo;"), @"
          ╭▸ 
        1 │ create table foo(a int); select (select (select a)) from foo;
          ╰╴                 ─ 2. destination               ─ 1. source
        ");
    }

    #[test]
    fn goto_lateral_missing_not_found() {
        // Query 1 ERROR at Line 3: : ERROR:  invalid reference to FROM-clause entry for table "u"
        // LINE 3: cross join (select u.n * 10 as val) x;
        //                            ^
        // DETAIL:  There is an entry for table "u", but it cannot be referenced from this part of the query.
        // HINT:  To reference that table, you must mark this subquery with LATERAL.
        goto_not_found(
            "
select u.n, x.val
from (values (1), (2)) u(n)
cross join (select u$0.n * 10 as val) x;
",
        );
    }

    #[test]
    fn goto_lateral_deeply_nested_paren_expr_values_alias_in_subquery() {
        assert_snapshot!(goto("
select u.n, x.val
from (values (1), (2)) u(n)
cross join lateral ((((select u$0.n * 10 as val)))) x;
"), @r"
          ╭▸ 
        3 │ from (values (1), (2)) u(n)
          │                        ─ 2. destination
        4 │ cross join lateral ((((select u.n * 10 as val)))) x;
          ╰╴                              ─ 1. source
        ");
    }

    #[test]
    fn goto_lateral_deeply_nested_paren_expr_values_alias_column() {
        assert_snapshot!(goto("
select u.n, x.val$0
from (values (1), (2)) u(n)
cross join lateral ((((select u.n * 10 as val)))) x;
"), @r"
          ╭▸ 
        2 │ select u.n, x.val
          │                 ─ 1. source
        3 │ from (values (1), (2)) u(n)
        4 │ cross join lateral ((((select u.n * 10 as val)))) x;
          ╰╴                                          ─── 2. destination
        ");
    }

    #[test]
    fn goto_lateral_deeply_nested_paren_expr_missing_not_found() {
        // Query 1 ERROR at Line 3: : ERROR:  invalid reference to FROM-clause entry for table "u"
        // LINE 3: cross join ((((select u.n * 10 as val)))) x;
        //                               ^
        // DETAIL:  There is an entry for table "u", but it cannot be referenced from this part of the query.
        // HINT:  To reference that table, you must mark this subquery with LATERAL.
        goto_not_found(
            "
select u.n, x.val
from (values (1), (2)) u(n)
cross join ((((select u$0.n * 10 as val)))) x;
",
        );
    }

    #[test]
    fn goto_aliased_join_expr_qualified_column() {
        assert_snapshot!(goto("
create table t(a int, b int);
create table u(a int, c int);
select j.b$0 from (t join u using(a)) as j;
"), @"
          ╭▸ 
        2 │ create table t(a int, b int);
          │                       ─ 2. destination
        3 │ create table u(a int, c int);
        4 │ select j.b from (t join u using(a)) as j;
          ╰╴         ─ 1. source
        ");
    }

    #[test]
    fn goto_aliased_join_expr_qualified_merged_column() {
        assert_snapshot!(goto("
create table t(a int, b int);
create table u(a int, c int);
select j.a$0 from (t join u using(a)) as j;
"), @"
          ╭▸ 
        2 │ create table t(a int, b int);
          │                ─ 2. destination
        3 │ create table u(a int, c int);
        4 │ select j.a from (t join u using(a)) as j;
          ╰╴         ─ 1. source
        ");
    }

    #[test]
    fn goto_aliased_join_expr_qualified_right_column() {
        assert_snapshot!(goto("
create table t(a int, b int);
create table u(a int, c int);
select j.c$0 from (t join u using(a)) as j;
"), @"
          ╭▸ 
        3 │ create table u(a int, c int);
          │                       ─ 2. destination
        4 │ select j.c from (t join u using(a)) as j;
          ╰╴         ─ 1. source
        ");
    }

    #[test]
    fn goto_unaliased_paren_join_qualified_column_target_list() {
        assert_snapshot!(goto("
create table t (a int);
create table u (b int);
select t.a$0 from (t join u on t.a = u.b);
"), @"
          ╭▸ 
        2 │ create table t (a int);
          │                 ─ 2. destination
        3 │ create table u (b int);
        4 │ select t.a from (t join u on t.a = u.b);
          ╰╴         ─ 1. source
        ");
    }

    #[test]
    fn goto_unaliased_paren_join_qualified_column_where_clause() {
        assert_snapshot!(goto("
create table t (a int);
create table u (b int);
select 1 from (t join u on t.a = u.b) where t.a$0 = 1;
"), @"
          ╭▸ 
        2 │ create table t (a int);
          │                 ─ 2. destination
        3 │ create table u (b int);
        4 │ select 1 from (t join u on t.a = u.b) where t.a = 1;
          ╰╴                                              ─ 1. source
        ");
    }

    #[test]
    fn goto_unaliased_paren_join_qualified_column_own_on_clause() {
        assert_snapshot!(goto("
create table t (a int);
create table u (b int);
select 1 from (t join u on t.a$0 = u.b);
"), @"
          ╭▸ 
        2 │ create table t (a int);
          │                 ─ 2. destination
        3 │ create table u (b int);
        4 │ select 1 from (t join u on t.a = u.b);
          ╰╴                             ─ 1. source
        ");
    }

    #[test]
    fn goto_unaliased_paren_join_qualified_column_outer_on_clause() {
        assert_snapshot!(goto("
create table t (a int);
create table u (b int);
create table v (c int);
select 1 from (t join u on t.a = u.b) join v on t.a$0 = v.c;
"), @"
          ╭▸ 
        2 │ create table t (a int);
          │                 ─ 2. destination
          ‡
        5 │ select 1 from (t join u on t.a = u.b) join v on t.a = v.c;
          ╰╴                                                  ─ 1. source
        ");
    }

    #[test]
    fn goto_fully_wrapped_paren_join_qualified_column_left() {
        assert_snapshot!(goto("
create table t (a int);
create table u (b int);
create table v (c int);
select 1 from ((t join u on t.a = u.b) join v on v.c = t.a$0);
"), @"
          ╭▸ 
        2 │ create table t (a int);
          │                 ─ 2. destination
          ‡
        5 │ select 1 from ((t join u on t.a = u.b) join v on v.c = t.a);
          ╰╴                                                         ─ 1. source
        ");
    }

    #[test]
    fn goto_fully_wrapped_paren_join_qualified_column_right() {
        assert_snapshot!(goto("
create table t (a int);
create table u (b int);
create table v (c int);
select 1 from ((t join u on t.a = u.b) join v on v.c$0 = t.a);
"), @"
          ╭▸ 
        4 │ create table v (c int);
          │                 ─ 2. destination
        5 │ select 1 from ((t join u on t.a = u.b) join v on v.c = t.a);
          ╰╴                                                   ─ 1. source
        ");
    }

    #[test]
    fn goto_ambiguous_unqualified_column_comma_join() {
        assert_snapshot!(goto("
create table t(a int);
create table u(a int);
select a$0 from t, u;
"), @"
          ╭▸ 
        2 │ create table t(a int);
          │                ─ 2. destination
        3 │ create table u(a int);
          │                ─ 3. destination
        4 │ select a from t, u;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_join_using_output_column() {
        assert_snapshot!(goto("
create table t(a int, b int);
create table u(a int, c int);
select a$0 from t join u using(a);
"), @"
          ╭▸ 
        2 │ create table t(a int, b int);
          │                ─ 2. destination
        3 │ create table u(a int, c int);
          │                ─ 3. destination
        4 │ select a from t join u using(a);
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_natural_join_output_column() {
        assert_snapshot!(goto("
create table t(a int, b int);
create table u(a int, c int);
select a$0 from t natural join u;
"), @"
          ╭▸ 
        2 │ create table t(a int, b int);
          │                ─ 2. destination
        3 │ create table u(a int, c int);
          │                ─ 3. destination
        4 │ select a from t natural join u;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_lateral_cte_ref_after_lateral_not_found() {
        // c is defined after the lateral it isn't visible to the subquery
        // Query 1 ERROR at Line 10: : ERROR:  missing FROM-clause entry for table "c"
        // LINE 10:     where d.id = c.id
        //                           ^
        goto_not_found(
            "
with
  d as (select 1 id, 2 amount),
  c as (select 2 id)
select r.amount
from
  d,
  lateral (
    select d.amount
    from d
    where d.id = c$0.id
    limit 1
  ) r,
  c;
",
        );
    }

    #[test]
    fn goto_cte_forward_ref_not_found() {
        // b is defined after a, so a can't reference it in a non-recursive WITH
        // ERROR:  relation "b" does not exist
        goto_not_found(
            "
  with
    a as (select * from b$0),
    b as (select 1 x)
  select * from a;
",
        );
    }

    #[test]
    fn goto_cte_forward_ref_ignored() {
        assert_snapshot!(goto("
create table b(c int);
with
  a as (select c$0 from b),
  b as (select 1 c)
select c from a;
"), @"
          ╭▸ 
        2 │ create table b(c int);
          │                ─ 2. destination
        3 │ with
        4 │   a as (select c from b),
          ╰╴               ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_forward_ref_ignored_inside_table_query() {
        assert_snapshot!(goto("
create table b(c int);
with
  a as (table b),
  b as (select 1 c)
select c$0 from a;
"), @"
          ╭▸ 
        2 │ create table b(c int);
          │                ─ 2. destination
          ‡
        6 │ select c from a;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_forward_ref_ignored_inside_create_table_as_star() {
        assert_snapshot!(goto("
create table b(c int);
create table ct as
  with
    a as (select * from b),
    b as (select 1 c)
  select * from a;
select c$0 from ct;
"), @"
          ╭▸ 
        2 │ create table b(c int);
          │                ─ 2. destination
          ‡
        8 │ select c from ct;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_forward_ref_ignored_inside_create_table_as_table() {
        assert_snapshot!(goto("
create table b(c int);
create table made as
  with
    a as (table b),
    b as (select 1 c)
  table a;
select c$0 from made;
"), @"
          ╭▸ 
        2 │ create table b(c int);
          │                ─ 2. destination
          ‡
        8 │ select c from made;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_star_over_subquery_from_item() {
        assert_snapshot!(goto("
create table t(c int);
with
  a as (select * from (select c from t))
select c$0 from a;
"), @"
          ╭▸ 
        4 │   a as (select * from (select c from t))
          │                               ─ 2. destination
        5 │ select c from a;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_outer_cte_visible_inside_inner_with() {
        assert_snapshot!(goto("
with outer_cte as (select 1 c)
select * from (
  with inner_cte as (select 2 d)
  select c$0 from outer_cte
) s;
"), @"
          ╭▸ 
        2 │ with outer_cte as (select 1 c)
          │                             ─ 2. destination
          ‡
        5 │   select c from outer_cte
          ╰╴         ─ 1. source
        ");
    }

    #[test]
    fn goto_inner_cte_forward_ref_falls_back_to_outer_cte() {
        assert_snapshot!(goto("
with t as (select 1 c)
select * from (
  with
    x as (select c$0 from t),
    t as (select 2 c)
  select c from x
) s;
"), @"
          ╭▸ 
        2 │ with t as (select 1 c)
          │                     ─ 2. destination
          ‡
        5 │     x as (select c from t),
          ╰╴                 ─ 1. source
        ");
    }

    #[test]
    fn goto_recursive_inner_cte_forward_ref_shadows_outer_cte() {
        assert_snapshot!(goto("
with t as (select 1 c)
select * from (
  with recursive
    x as (select c$0 from t),
    t as (select 2 c)
  select c from x
) s;
"), @"
          ╭▸ 
        5 │     x as (select c from t),
          │                  ─ 1. source
        6 │     t as (select 2 c)
          ╰╴                   ─ 2. destination
        ");
    }

    #[test]
    fn goto_cte_forward_ref_ignored_for_qualified_star() {
        assert_snapshot!(goto("
create table b(c int);
with
  a as (select b.* from b),
  b as (select 1 c)
select c$0 from a;
"), @"
          ╭▸ 
        2 │ create table b(c int);
          │                ─ 2. destination
          ‡
        6 │ select c from a;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_forward_ref_ignored_for_star_column_count() {
        assert_snapshot!(goto("
create table b(x int, yy int);
with
  a(x, yy) as (select *, 2 y, 3 z from b),
  b as (select 1 only_col)
select y$0 from a;
"), @"
          ╭▸ 
        4 │   a(x, yy) as (select *, 2 y, 3 z from b),
          │                            ─ 2. destination
        5 │   b as (select 1 only_col)
        6 │ select y from a;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_forward_ref_ignored_inside_subquery_table() {
        assert_snapshot!(goto("
create table b(c int);
with
  a as (select * from (table b) q),
  b as (select 1 c)
select c$0 from a;
"), @"
          ╭▸ 
        2 │ create table b(c int);
          │                ─ 2. destination
          ‡
        6 │ select c from a;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_sequence() {
        assert_snapshot!(goto("
create sequence s;
drop sequence s$0;
"), @r"
          ╭▸ 
        2 │ create sequence s;
          │                 ─ 2. destination
        3 │ drop sequence s;
          ╰╴              ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_constraint() {
        assert_snapshot!(goto("
create table t(id int constraint id_positive check (id > 0));
alter table t drop constraint id_positive$0;
"), @"
          ╭▸ 
        2 │ create table t(id int constraint id_positive check (id > 0));
          │                                  ─────────── 2. destination
        3 │ alter table t drop constraint id_positive;
          ╰╴                                        ─ 1. source
        ");
    }

    #[test]
    fn goto_comment_on_constraint() {
        assert_snapshot!(goto("
create table t(id int constraint id_positive check (id > 0));
comment on constraint id_positive$0 on t is 'positive id';
"), @"
          ╭▸ 
        2 │ create table t(id int constraint id_positive check (id > 0));
          │                                  ─────────── 2. destination
        3 │ comment on constraint id_positive on t is 'positive id';
          ╰╴                                ─ 1. source
        ");
    }

    #[test]
    fn goto_comment_on_constraint_table() {
        assert_snapshot!(goto("
create table t(id int constraint id_positive check (id > 0));
comment on constraint id_positive on t$0 is 'positive id';
"), @"
          ╭▸ 
        2 │ create table t(id int constraint id_positive check (id > 0));
          │              ─ 2. destination
        3 │ comment on constraint id_positive on t is 'positive id';
          ╰╴                                     ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_constraint_with_same_name_on_multiple_tables() {
        assert_snapshot!(goto("
create table t(id int constraint id_positive check (id > 0));
create table u(id int constraint id_positive check (id > 0));
alter table u drop constraint id_positive$0;
"), @"
          ╭▸ 
        3 │ create table u(id int constraint id_positive check (id > 0));
          │                                  ─────────── 2. destination
        4 │ alter table u drop constraint id_positive;
          ╰╴                                        ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_table_add_constraint() {
        assert_snapshot!(goto("
create table t(id int);
alter table t add constraint id_positive check (id > 0);
comment on constraint id_positive$0 on t is 'positive id';
"), @"
          ╭▸ 
        3 │ alter table t add constraint id_positive check (id > 0);
          │                              ─────────── 2. destination
        4 │ comment on constraint id_positive on t is 'positive id';
          ╰╴                                ─ 1. source
        ");
    }

    #[test]
    fn goto_on_conflict_constraint() {
        assert_snapshot!(goto("
create table t(id int constraint t_id_key unique);
insert into t values (1) on conflict on constraint t_id_key$0 do nothing;
"), @"
          ╭▸ 
        2 │ create table t(id int constraint t_id_key unique);
          │                                  ──────── 2. destination
        3 │ insert into t values (1) on conflict on constraint t_id_key do nothing;
          ╰╴                                                          ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_trigger() {
        assert_snapshot!(goto("
create trigger tr before insert on t for each row execute function f();
drop trigger tr$0 on t;
"), @r"
          ╭▸ 
        2 │ create trigger tr before insert on t for each row execute function f();
          │                ── 2. destination
        3 │ drop trigger tr on t;
          ╰╴              ─ 1. source
        ");
    }

    #[test]
    fn goto_create_rule_table() {
        assert_snapshot!(goto("
create table t(a int);
create rule r as on select to t$0 do instead nothing;
"), @"
          ╭▸ 
        2 │ create table t(a int);
          │              ─ 2. destination
        3 │ create rule r as on select to t do instead nothing;
          ╰╴                              ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_rule() {
        assert_snapshot!(goto("
create table t(a int);
create rule r as on select to t do instead nothing;
drop rule r$0 on t;
"), @"
          ╭▸ 
        3 │ create rule r as on select to t do instead nothing;
          │             ─ 2. destination
        4 │ drop rule r on t;
          ╰╴          ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_rule() {
        assert_snapshot!(goto("
create table t(a int);
create rule r as on select to t do instead nothing;
alter rule r$0 on t rename to r2;
"), @"
          ╭▸ 
        3 │ create rule r as on select to t do instead nothing;
          │             ─ 2. destination
        4 │ alter rule r on t rename to r2;
          ╰╴           ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_table_enable_trigger() {
        assert_snapshot!(goto("
create table t(a int);
create function f() returns trigger language plpgsql as $$ begin return new; end $$;
create trigger tr before insert on t for each row execute function f();
alter table t enable trigger tr$0;
"), @"
          ╭▸ 
        4 │ create trigger tr before insert on t for each row execute function f();
          │                ── 2. destination
        5 │ alter table t enable trigger tr;
          ╰╴                              ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_table_disable_trigger() {
        assert_snapshot!(goto("
create table t(a int);
create function f() returns trigger language plpgsql as $$ begin return new; end $$;
create trigger tr before insert on t for each row execute function f();
alter table t disable trigger tr$0;
"), @"
          ╭▸ 
        4 │ create trigger tr before insert on t for each row execute function f();
          │                ── 2. destination
        5 │ alter table t disable trigger tr;
          ╰╴                               ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_table_enable_rule() {
        assert_snapshot!(goto("
create table t(a int);
create rule r as on insert to t do instead nothing;
alter table t enable rule r$0;
"), @"
          ╭▸ 
        3 │ create rule r as on insert to t do instead nothing;
          │             ─ 2. destination
        4 │ alter table t enable rule r;
          ╰╴                          ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_table_disable_rule() {
        assert_snapshot!(goto("
create table t(a int);
create rule r as on insert to t do instead nothing;
alter table t disable rule r$0;
"), @"
          ╭▸ 
        3 │ create rule r as on insert to t do instead nothing;
          │             ─ 2. destination
        4 │ alter table t disable rule r;
          ╰╴                           ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_policy() {
        assert_snapshot!(goto("
create table t(c int);
create table u(c int);
create policy p on t;
create policy p on u;
drop policy if exists p$0 on t;
"), @r"
          ╭▸ 
        4 │ create policy p on t;
          │               ─ 2. destination
        5 │ create policy p on u;
        6 │ drop policy if exists p on t;
          ╰╴                      ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_policy() {
        assert_snapshot!(goto("
create table t(c int);
create policy p on t;
alter policy p$0 on t
  with check (c > 1);
"), @r"
          ╭▸ 
        3 │ create policy p on t;
          │               ─ 2. destination
        4 │ alter policy p on t
          ╰╴             ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_policy_column() {
        assert_snapshot!(goto("
create table t(c int);
create policy p on t;
alter policy p on t
  with check (c$0 > 1);
"), @"
          ╭▸ 
        2 │ create table t(c int);
          │                ─ 2. destination
          ‡
        5 │   with check (c > 1);
          ╰╴              ─ 1. source
        ");
    }

    #[test]
    fn goto_create_policy_column() {
        assert_snapshot!(goto("
create table t(c int, d int);
create policy p on t
  with check (c$0 > d);
"), @r"
          ╭▸ 
        2 │ create table t(c int, d int);
          │                ─ 2. destination
        3 │ create policy p on t
        4 │   with check (c > d);
          ╰╴              ─ 1. source
        ");
    }

    #[test]
    fn goto_create_policy_using_column() {
        assert_snapshot!(goto("
create table t(c int, d int);
create policy p on t
  using (c$0 > d and 1 < 2);
"), @r"
          ╭▸ 
        2 │ create table t(c int, d int);
          │                ─ 2. destination
        3 │ create policy p on t
        4 │   using (c > d and 1 < 2);
          ╰╴         ─ 1. source
        ");
    }

    #[test]
    fn goto_create_policy_qualified_column_table() {
        assert_snapshot!(goto("
create table t(c int, d int);
create policy p on t
  with check (t$0.c > d);
"), @r"
          ╭▸ 
        2 │ create table t(c int, d int);
          │              ─ 2. destination
        3 │ create policy p on t
        4 │   with check (t.c > d);
          ╰╴              ─ 1. source
        ");
    }

    #[test]
    fn goto_create_policy_qualified_column() {
        assert_snapshot!(goto("
create table t(c int, d int);
create policy p on t
  with check (t.c$0 > d);
"), @r"
          ╭▸ 
        2 │ create table t(c int, d int);
          │                ─ 2. destination
        3 │ create policy p on t
        4 │   with check (t.c > d);
          ╰╴                ─ 1. source
        ");
    }

    #[test]
    fn goto_create_policy_field_style_function_call() {
        assert_snapshot!(goto("
create table t(c int);
create function x(t) returns int8
  as 'select 1'
  language sql;
create policy p on t
  with check (t.c > 1 and t.x$0 > 0);
"), @r"
          ╭▸ 
        3 │ create function x(t) returns int8
          │                 ─ 2. destination
          ‡
        7 │   with check (t.c > 1 and t.x > 0);
          ╰╴                            ─ 1. source
        ");
    }

    #[test]
    fn goto_function_param_in_begin_atomic_body() {
        assert_snapshot!(goto("
create function f(a int) returns int
begin atomic
  select a$0;
end;
"), @"
          ╭▸ 
        2 │ create function f(a int) returns int
          │                   ─ 2. destination
        3 │ begin atomic
        4 │   select a;
          ╰╴         ─ 1. source
        ");
    }

    #[test]
    fn goto_function_param_in_begin_atomic_predicate() {
        assert_snapshot!(goto("
create table t (id int);
create function f(x int) returns int begin atomic
  select id from t where id = x$0;
end;
"), @"
          ╭▸ 
        3 │ create function f(x int) returns int begin atomic
          │                   ─ 2. destination
        4 │   select id from t where id = x;
          ╰╴                              ─ 1. source
        ");
    }

    #[test]
    fn goto_function_param_in_sql_body_return_expr() {
        assert_snapshot!(goto("
create function f(x int) returns int language sql return x$0 + 1;
"), @"
          ╭▸ 
        2 │ create function f(x int) returns int language sql return x + 1;
          ╰╴                  ─ 2. destination                       ─ 1. source
        ");
    }

    #[test]
    fn goto_function_param_self_qualified_in_sql_body_return_expr() {
        assert_snapshot!(goto("
create function f(x int) returns int language sql return f.x$0 + 1;
"), @"
          ╭▸ 
        2 │ create function f(x int) returns int language sql return f.x + 1;
          ╰╴                  ─ 2. destination                         ─ 1. source
        ");
    }

    #[test]
    fn goto_function_param_bogus_qualified_in_sql_body_return_expr() {
        goto_not_found(
            "
create function f(x int) returns int language sql return bogus.x$0 + 1;
",
        );
    }

    #[test]
    fn goto_positional_param_in_sql_body_return_expr() {
        assert_snapshot!(goto("
create function f(x int) returns int language sql return $1$0 + 1;
"), @"
          ╭▸ 
        2 │ create function f(x int) returns int language sql return $1 + 1;
          ╰╴                  ─ 2. destination                        ─ 1. source
        ");
    }

    #[test]
    fn goto_positional_param_in_begin_atomic_body() {
        assert_snapshot!(goto("
create function f(x int) returns int language sql begin atomic
  select $1$0 + 1;
end;
"), @"
          ╭▸ 
        2 │ create function f(x int) returns int language sql begin atomic
          │                   ─ 2. destination
        3 │   select $1 + 1;
          ╰╴          ─ 1. source
        ");
    }

    #[test]
    fn goto_positional_param_unnamed_param() {
        assert_snapshot!(goto("
create function f(int) returns int language sql return $1$0;
"), @"
          ╭▸ 
        2 │ create function f(int) returns int language sql return $1;
          ╰╴                  ─── 2. destination                    ─ 1. source
        ");
    }

    #[test]
    fn goto_positional_param_second_of_two() {
        assert_snapshot!(goto("
create function f(int, text) returns int language sql return $2$0;
"), @"
          ╭▸ 
        2 │ create function f(int, text) returns int language sql return $2;
          ╰╴                       ──── 2. destination                    ─ 1. source
        ");
    }

    #[test]
    fn goto_function_param_in_later_param_default() {
        assert_snapshot!(goto("
create function f(a int default 1, b int default a$0) returns int
begin atomic select 1; end;
"), @"
          ╭▸ 
        2 │ create function f(a int default 1, b int default a) returns int
          ╰╴                  ─ 2. destination               ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_policy_qualified_column_table() {
        assert_snapshot!(goto("
create table t(c int, d int);
alter policy p on t
  with check (t$0.c > d);
"), @r"
          ╭▸ 
        2 │ create table t(c int, d int);
          │              ─ 2. destination
        3 │ alter policy p on t
        4 │   with check (t.c > d);
          ╰╴              ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_policy_qualified_column() {
        assert_snapshot!(goto("
create table t(c int, d int);
alter policy p on t
  with check (t.c$0 > d);
"), @r"
          ╭▸ 
        2 │ create table t(c int, d int);
          │                ─ 2. destination
        3 │ alter policy p on t
        4 │   with check (t.c > d);
          ╰╴                ─ 1. source
        ");
    }

    #[test]
    fn goto_builtin_now() {
        assert_snapshot!(goto("
-- include-builtins
select now$0();
"), @"
              ╭▸ current.sql:3:10
              │
            3 │ select now();
              │          ─ 1. source
              ╰╴

              ╭▸ builtins.sql:11089:28
              │
        11089 │ create function pg_catalog.now() returns timestamp with time zone
              ╰╴                           ─── 2. destination
        ");
    }

    #[test]
    fn goto_current_timestamp() {
        assert_snapshot!(goto("
-- include-builtins
select current_timestamp$0;
"), @"
              ╭▸ current.sql:3:24
              │
            3 │ select current_timestamp;
              │                        ─ 1. source
              ╰╴

              ╭▸ builtins.sql:11089:28
              │
        11089 │ create function pg_catalog.now() returns timestamp with time zone
              ╰╴                           ─── 2. destination
        ");
    }

    #[test]
    fn goto_current_user() {
        assert_snapshot!(goto("
create function pg_catalog.current_user() returns name
  language internal;
select current_user$0;
"), @"
          ╭▸ 
        2 │ create function pg_catalog.current_user() returns name
          │                            ──────────── 2. destination
        3 │   language internal;
        4 │ select current_user;
          ╰╴                  ─ 1. source
        "
        );
    }

    #[test]
    fn goto_user_keyword() {
        assert_snapshot!(goto("
create function pg_catalog.current_user() returns name
  language internal;
select user$0;
"), @"
          ╭▸ 
        2 │ create function pg_catalog.current_user() returns name
          │                            ──────────── 2. destination
        3 │   language internal;
        4 │ select user;
          ╰╴          ─ 1. source
        "
        );
    }

    #[test]
    fn goto_session_user() {
        assert_snapshot!(goto("
create function pg_catalog.session_user() returns name
  language internal;
select session_user$0;
"), @"
          ╭▸ 
        2 │ create function pg_catalog.session_user() returns name
          │                            ──────────── 2. destination
        3 │   language internal;
        4 │ select session_user;
          ╰╴                  ─ 1. source
        "
        );
    }

    #[test]
    fn goto_current_schema() {
        assert_snapshot!(goto("
create function current_schema() returns name
  language internal;
select current_schema$0;
"), @"
          ╭▸ 
        2 │ create function current_schema() returns name
          │                 ────────────── 2. destination
        3 │   language internal;
        4 │ select current_schema;
          ╰╴                    ─ 1. source
        "
        );
    }

    #[test]
    fn goto_current_timestamp_cte_column() {
        assert_snapshot!(goto("
with t as (select 1 current_timestamp)
select current_timestamp$0 from t;
"), @r"
          ╭▸ 
        2 │ with t as (select 1 current_timestamp)
          │                     ───────────────── 2. destination
        3 │ select current_timestamp from t;
          ╰╴                       ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_casing() {
        // postgres only folds ascii characters so Ä doesn't become ä
        goto_not_found(
            "
    with t as (select 1 Äpfel)
    select äpfel$0 from t;
    ",
        );
    }

    #[test]
    fn goto_cte_emoji() {
        assert_snapshot!(goto(
            "
    with t as (select 1 🦀)
    select 🦀$0 from t;
    "), @"
          ╭▸ 
        2 │     with t as (select 1 🦀)
          │                         ── 2. destination
        3 │     select 🦀 from t;
          ╰╴           ── 1. source
        ");
    }

    #[test]
    fn goto_current_timestamp_in_where() {
        assert_snapshot!(goto("
-- include-builtins
create table t(created_at timestamptz);
select * from t where current_timestamp$0 > t.created_at;
"), @"
              ╭▸ current.sql:4:39
              │
            4 │ select * from t where current_timestamp > t.created_at;
              │                                       ─ 1. source
              ╰╴

              ╭▸ builtins.sql:11089:28
              │
        11089 │ create function pg_catalog.now() returns timestamp with time zone
              ╰╴                           ─── 2. destination
        ");
    }

    #[test]
    fn goto_create_policy_schema_qualified_table() {
        assert_snapshot!(goto("
create schema foo;
create table foo.t(c int);
create policy p on foo.t
  with check (foo.t$0.c > 1);
"), @r"
          ╭▸ 
        3 │ create table foo.t(c int);
          │                  ─ 2. destination
        4 │ create policy p on foo.t
        5 │   with check (foo.t.c > 1);
          ╰╴                  ─ 1. source
        ");
    }

    #[test]
    fn goto_create_policy_unqualified_table_with_schema_on_table() {
        assert_snapshot!(goto("
create schema foo;
create table foo.t(c int);
create policy p on foo.t
  with check (t$0.c > 1);
"), @r"
          ╭▸ 
        3 │ create table foo.t(c int);
          │                  ─ 2. destination
        4 │ create policy p on foo.t
        5 │   with check (t.c > 1);
          ╰╴              ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_event_trigger() {
        assert_snapshot!(goto("
create event trigger et on ddl_command_start execute function f();
drop event trigger et$0;
"), @r"
          ╭▸ 
        2 │ create event trigger et on ddl_command_start execute function f();
          │                      ── 2. destination
        3 │ drop event trigger et;
          ╰╴                    ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_event_trigger() {
        assert_snapshot!(goto("
create event trigger et on ddl_command_start execute function f();
alter event trigger et$0 disable;
"), @r"
          ╭▸ 
        2 │ create event trigger et on ddl_command_start execute function f();
          │                      ── 2. destination
        3 │ alter event trigger et disable;
          ╰╴                     ─ 1. source
        ");
    }

    #[test]
    fn goto_create_event_trigger_function() {
        assert_snapshot!(goto("
create function f() returns event_trigger as 'select 1' language sql;
create event trigger et on ddl_command_start execute function f$0();
"), @r"
          ╭▸ 
        2 │ create function f() returns event_trigger as 'select 1' language sql;
          │                 ─ 2. destination
        3 │ create event trigger et on ddl_command_start execute function f();
          ╰╴                                                              ─ 1. source
        ");
    }

    #[test]
    fn goto_create_event_trigger_procedure() {
        assert_snapshot!(goto("
create procedure p() language sql as 'select 1';
create event trigger tr
  on ddl_command_end
  execute procedure p$0();
"), @r"
          ╭▸ 
        2 │ create procedure p() language sql as 'select 1';
          │                  ─ 2. destination
          ‡
        5 │   execute procedure p();
          ╰╴                    ─ 1. source
        ");
    }

    #[test]
    fn goto_create_trigger_function() {
        assert_snapshot!(goto("
create function f() returns trigger as 'select 1' language sql;
create trigger tr before insert on t for each row execute function f$0();
"), @r"
          ╭▸ 
        2 │ create function f() returns trigger as 'select 1' language sql;
          │                 ─ 2. destination
        3 │ create trigger tr before insert on t for each row execute function f();
          ╰╴                                                                   ─ 1. source
        ");
    }

    #[test]
    fn goto_create_trigger_procedure() {
        assert_snapshot!(goto("
create procedure a() language sql as 'select 1';
create trigger tr before truncate or delete or insert
on t
execute procedure a$0();
"), @r"
          ╭▸ 
        2 │ create procedure a() language sql as 'select 1';
          │                  ─ 2. destination
          ‡
        5 │ execute procedure a();
          ╰╴                  ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_trigger_table_specific() {
        assert_snapshot!(goto("
create table u(a int);
create trigger tr before truncate
on u
execute function noop();

create table t(b int);
create trigger tr before truncate
on t
execute function noop();

drop trigger tr$0 on t;
"), @r"
           ╭▸ 
         8 │ create trigger tr before truncate
           │                ── 2. destination
           ‡
        12 │ drop trigger tr on t;
           ╰╴              ─ 1. source
        ");
    }

    #[test]
    fn goto_create_trigger_table() {
        assert_snapshot!(goto("
create table t(b int);
create trigger tr before truncate
on t$0
execute function noop();
"), @r"
          ╭▸ 
        2 │ create table t(b int);
          │              ─ 2. destination
        3 │ create trigger tr before truncate
        4 │ on t
          ╰╴   ─ 1. source
        ");
    }

    #[test]
    fn goto_create_constraint_trigger_from_table() {
        assert_snapshot!(goto("
create table t(id int);
create table ref_t(id int);
create constraint trigger trg after insert on t from ref_t$0 for each row execute function f();
"), @"
          ╭▸ 
        3 │ create table ref_t(id int);
          │              ───── 2. destination
        4 │ create constraint trigger trg after insert on t from ref_t for each row execute function f();
          ╰╴                                                         ─ 1. source
        ");
    }

    #[test]
    fn goto_create_trigger_when_new_column() {
        assert_snapshot!(goto("
create table foo (id int);
create trigger tr before insert on foo for each row when (new.id$0 > 0) execute function f();
"), @"
          ╭▸ 
        2 │ create table foo (id int);
          │                   ── 2. destination
        3 │ create trigger tr before insert on foo for each row when (new.id > 0) execute function f();
          ╰╴                                                               ─ 1. source
        ");
    }

    #[test]
    fn goto_create_trigger_when_old_column() {
        assert_snapshot!(goto("
create table foo (id int);
create trigger tr after update on foo for each row when (old.id$0 > 0) execute function f();
"), @"
          ╭▸ 
        2 │ create table foo (id int);
          │                   ── 2. destination
        3 │ create trigger tr after update on foo for each row when (old.id > 0) execute function f();
          ╰╴                                                              ─ 1. source
        ");
    }

    #[test]
    fn goto_create_trigger_when_new_table() {
        assert_snapshot!(goto("
create table foo (id int);
create trigger tr before insert on foo for each row when (new$0.id > 0) execute function f();
"), @"
          ╭▸ 
        2 │ create table foo (id int);
          │              ─── 2. destination
        3 │ create trigger tr before insert on foo for each row when (new.id > 0) execute function f();
          ╰╴                                                            ─ 1. source
        ");
    }

    #[test]
    fn goto_create_rule_old_column() {
        assert_snapshot!(goto("
create table t(id int);
create rule r as on update to t where old.id$0 = new.id do instead nothing;
"), @"
          ╭▸ 
        2 │ create table t(id int);
          │                ── 2. destination
        3 │ create rule r as on update to t where old.id = new.id do instead nothing;
          ╰╴                                           ─ 1. source
        ");
    }

    #[test]
    fn goto_create_rule_new_column() {
        assert_snapshot!(goto("
create table t(id int);
create rule r as on update to t where old.id = new.id$0 do instead nothing;
"), @"
          ╭▸ 
        2 │ create table t(id int);
          │                ── 2. destination
        3 │ create rule r as on update to t where old.id = new.id do instead nothing;
          ╰╴                                                    ─ 1. source
        ");
    }

    #[test]
    fn goto_create_rule_old_table() {
        assert_snapshot!(goto("
create table t(id int);
create rule r as on update to t where old$0.id = new.id do instead nothing;
"), @"
          ╭▸ 
        2 │ create table t(id int);
          │              ─ 2. destination
        3 │ create rule r as on update to t where old.id = new.id do instead nothing;
          ╰╴                                        ─ 1. source
        ");
    }

    #[test]
    fn goto_create_trigger_update_of_column() {
        assert_snapshot!(goto("
create table t(id int, updated_at timestamptz);
create trigger tr before update of updated_at$0 on t for each row execute function f();
"), @"
          ╭▸ 
        2 │ create table t(id int, updated_at timestamptz);
          │                        ────────── 2. destination
        3 │ create trigger tr before update of updated_at on t for each row execute function f();
          ╰╴                                            ─ 1. source
        ");
    }

    #[test]
    fn goto_create_sequence_owned_by() {
        assert_snapshot!(goto("
create table t(c serial);
create sequence s
  owned by t.c$0;
"), @r"
          ╭▸ 
        2 │ create table t(c serial);
          │                ─ 2. destination
        3 │ create sequence s
        4 │   owned by t.c;
          ╰╴             ─ 1. source
        ");
    }

    #[test]
    fn goto_create_sequence_owned_by_table() {
        assert_snapshot!(goto("
create table t(c serial);
create sequence s
  owned by t$0.c;
"), @"
          ╭▸ 
        2 │ create table t(c serial);
          │              ─ 2. destination
        3 │ create sequence s
        4 │   owned by t.c;
          ╰╴           ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_sequence_owned_by_table() {
        assert_snapshot!(goto("
create table t(c serial);
create sequence s;
alter sequence s owned by t$0.c;
"), @"
          ╭▸ 
        2 │ create table t(c serial);
          │              ─ 2. destination
        3 │ create sequence s;
        4 │ alter sequence s owned by t.c;
          ╰╴                          ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_tablespace() {
        assert_snapshot!(goto("
create tablespace ts location '/tmp/ts';
drop tablespace ts$0;
"), @r"
          ╭▸ 
        2 │ create tablespace ts location '/tmp/ts';
          │                   ── 2. destination
        3 │ drop tablespace ts;
          ╰╴                 ─ 1. source
        ");
    }

    #[test]
    fn goto_create_table_tablespace() {
        assert_snapshot!(goto("
create tablespace bar location '/tmp/ts';
create table t (a int) tablespace b$0ar;
"), @r"
          ╭▸ 
        2 │ create tablespace bar location '/tmp/ts';
          │                   ─── 2. destination
        3 │ create table t (a int) tablespace bar;
          ╰╴                                  ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_database() {
        assert_snapshot!(goto("
create database mydb;
drop database my$0db;
"), @r"
          ╭▸ 
        2 │ create database mydb;
          │                 ──── 2. destination
        3 │ drop database mydb;
          ╰╴               ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_role() {
        assert_snapshot!(goto("
create role reader;
drop role read$0er;
"), @r"
          ╭▸ 
        2 │ create role reader;
          │             ────── 2. destination
        3 │ drop role reader;
          ╰╴             ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_role() {
        assert_snapshot!(goto("
create role reader;
alter role read$0er rename to writer;
"), @r"
          ╭▸ 
        2 │ create role reader;
          │             ────── 2. destination
        3 │ alter role reader rename to writer;
          ╰╴              ─ 1. source
        ");
    }

    #[test]
    fn goto_set_role() {
        assert_snapshot!(goto("
create role reader;
set role read$0er;
"), @r"
          ╭▸ 
        2 │ create role reader;
          │             ────── 2. destination
        3 │ set role reader;
          ╰╴            ─ 1. source
        ");
    }

    #[test]
    fn goto_create_tablespace_owner_role() {
        assert_snapshot!(goto("
create role reader;
create tablespace t owner read$0er location 'foo';
"), @r"
          ╭▸ 
        2 │ create role reader;
          │             ────── 2. destination
        3 │ create tablespace t owner reader location 'foo';
          ╰╴                             ─ 1. source
        ");
    }

    #[test]
    fn goto_role_definition_returns_self() {
        assert_snapshot!(goto("
create role read$0er;
"), @r"
          ╭▸ 
        2 │ create role reader;
          │             ┬──┬──
          │             │  │
          │             │  1. source
          ╰╴            2. destination
        ");
    }

    #[test]
    fn goto_drop_database_defined_after() {
        assert_snapshot!(goto("
drop database my$0db;
create database mydb;
"), @r"
          ╭▸ 
        2 │ drop database mydb;
          │                ─ 1. source
        3 │ create database mydb;
          ╰╴                ──── 2. destination
        ");
    }

    #[test]
    fn goto_database_definition_returns_self() {
        assert_snapshot!(goto("
create database my$0db;
"), @r"
          ╭▸ 
        2 │ create database mydb;
          │                 ┬┬──
          │                 ││
          │                 │1. source
          ╰╴                2. destination
        ");
    }

    #[test]
    fn goto_drop_server() {
        assert_snapshot!(goto("
create server myserver foreign data wrapper fdw;
drop server my$0server;
"), @r"
          ╭▸ 
        2 │ create server myserver foreign data wrapper fdw;
          │               ──────── 2. destination
        3 │ drop server myserver;
          ╰╴             ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_server_defined_after() {
        assert_snapshot!(goto("
drop server my$0server;
create server myserver foreign data wrapper fdw;
"), @r"
          ╭▸ 
        2 │ drop server myserver;
          │              ─ 1. source
        3 │ create server myserver foreign data wrapper fdw;
          ╰╴              ──────── 2. destination
        ");
    }

    #[test]
    fn goto_alter_server() {
        assert_snapshot!(goto("
create server myserver foreign data wrapper fdw;
alter server my$0server options (add foo 'bar');
"), @r"
          ╭▸ 
        2 │ create server myserver foreign data wrapper fdw;
          │               ──────── 2. destination
        3 │ alter server myserver options (add foo 'bar');
          ╰╴              ─ 1. source
        ");
    }

    #[test]
    fn goto_server_definition_returns_self() {
        assert_snapshot!(goto("
create server my$0server foreign data wrapper fdw;
"), @r"
          ╭▸ 
        2 │ create server myserver foreign data wrapper fdw;
          │               ┬┬──────
          │               ││
          │               │1. source
          ╰╴              2. destination
        ");
    }

    #[test]
    fn goto_drop_extension() {
        assert_snapshot!(goto("
create extension myext;
drop extension my$0ext;
"), @r"
          ╭▸ 
        2 │ create extension myext;
          │                  ───── 2. destination
        3 │ drop extension myext;
          ╰╴                ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_extension_defined_after() {
        assert_snapshot!(goto("
drop extension my$0ext;
create extension myext;
"), @r"
          ╭▸ 
        2 │ drop extension myext;
          │                 ─ 1. source
        3 │ create extension myext;
          ╰╴                 ───── 2. destination
        ");
    }

    #[test]
    fn goto_alter_extension() {
        assert_snapshot!(goto("
create extension myext;
alter extension my$0ext update to '2.0';
"), @r"
          ╭▸ 
        2 │ create extension myext;
          │                  ───── 2. destination
        3 │ alter extension myext update to '2.0';
          ╰╴                 ─ 1. source
        ");
    }

    #[test]
    fn goto_create_extension_with_schema() {
        assert_snapshot!(goto("
create schema ext_schema;
create extension hstore with schema ext_sche$0ma;
"), @"
          ╭▸ 
        2 │ create schema ext_schema;
          │               ────────── 2. destination
        3 │ create extension hstore with schema ext_schema;
          ╰╴                                           ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_extension_add_table() {
        assert_snapshot!(goto("
create extension e;
create table t(id int);
alter extension e add table t$0;
"), @"
          ╭▸ 
        3 │ create table t(id int);
          │              ─ 2. destination
        4 │ alter extension e add table t;
          ╰╴                            ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_extension_add_foreign_table() {
        assert_snapshot!(goto("
create extension e;
create foreign table t(id int) server s;
alter extension e add foreign table t$0;
"), @"
          ╭▸ 
        3 │ create foreign table t(id int) server s;
          │                      ─ 2. destination
        4 │ alter extension e add foreign table t;
          ╰╴                                    ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_default_privileges_in_schema() {
        assert_snapshot!(goto("
create schema myschema;
create role bob;
alter default privileges in schema myschema$0
  grant select on tables to bob;
"), @"
          ╭▸ 
        2 │ create schema myschema;
          │               ──────── 2. destination
        3 │ create role bob;
        4 │ alter default privileges in schema myschema
          ╰╴                                          ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_publication() {
        assert_snapshot!(goto("
create table t(id int);
create publication pub for table t;
alter publication pub$0 add table t;
"), @"
          ╭▸ 
        3 │ create publication pub for table t;
          │                    ─── 2. destination
        4 │ alter publication pub add table t;
          ╰╴                    ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_subscription() {
        assert_snapshot!(goto("
create subscription sub connection $$host=localhost$$ publication pub;
alter subscription sub$0 refresh publication;
"), @"
          ╭▸ 
        2 │ create subscription sub connection $$host=localhost$$ publication pub;
          │                     ─── 2. destination
        3 │ alter subscription sub refresh publication;
          ╰╴                     ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_language() {
        assert_snapshot!(goto("
create language plpythonu;
drop language plpythonu$0;
"), @"
          ╭▸ 
        2 │ create language plpythonu;
          │                 ───────── 2. destination
        3 │ drop language plpythonu;
          ╰╴                      ─ 1. source
        ");
    }

    #[test]
    fn goto_create_function_language_option() {
        assert_snapshot!(goto("
create language mylang;
create function f() returns int language mylang$0 as $$x$$;
"), @"
          ╭▸ 
        2 │ create language mylang;
          │                 ────── 2. destination
        3 │ create function f() returns int language mylang as $$x$$;
          ╰╴                                              ─ 1. source
        ");
    }

    #[test]
    fn goto_create_procedure_language_option() {
        assert_snapshot!(goto("
create language mylang;
create procedure p() language mylang$0 as $$x$$;
"), @"
          ╭▸ 
        2 │ create language mylang;
          │                 ────── 2. destination
        3 │ create procedure p() language mylang as $$x$$;
          ╰╴                                   ─ 1. source
        ");
    }

    #[test]
    fn goto_create_function_support_option() {
        assert_snapshot!(goto("
create function sf(internal) returns internal language c as $$x$$;
create function f(int) returns int language sql support sf$0 as $$select 1$$;
"), @"
          ╭▸ 
        2 │ create function sf(internal) returns internal language c as $$x$$;
          │                 ── 2. destination
        3 │ create function f(int) returns int language sql support sf as $$select 1$$;
          ╰╴                                                         ─ 1. source
        ");
    }

    #[test]
    fn goto_create_transform_language_option() {
        assert_snapshot!(goto("
create language mylang;
create type typ as (x int);
create transform for typ language mylang$0
  (from sql with function int4(typ));
"), @"
          ╭▸ 
        2 │ create language mylang;
          │                 ────── 2. destination
        3 │ create type typ as (x int);
        4 │ create transform for typ language mylang
          ╰╴                                       ─ 1. source
        ");
    }

    #[test]
    fn goto_collate_in_column() {
        assert_snapshot!(goto("
create collation mycoll (locale = 'C');
create table t(name text collate mycoll$0);
"), @"
          ╭▸ 
        2 │ create collation mycoll (locale = 'C');
          │                  ────── 2. destination
        3 │ create table t(name text collate mycoll);
          ╰╴                                      ─ 1. source
        ");
    }

    #[test]
    fn goto_collate_in_order_by() {
        assert_snapshot!(goto("
create collation c from \"C\";
create table t(a text);
select a from t order by a collate c$0;
"), @r#"
          ╭▸ 
        2 │ create collation c from "C";
          │                  ─ 2. destination
        3 │ create table t(a text);
        4 │ select a from t order by a collate c;
          ╰╴                                   ─ 1. source
        "#);
    }

    #[test]
    fn goto_collate_in_index_expr() {
        assert_snapshot!(goto("
create collation c from \"C\";
create table t(a text);
create index idx on t (a collate c$0);
"), @r#"
          ╭▸ 
        2 │ create collation c from "C";
          │                  ─ 2. destination
        3 │ create table t(a text);
        4 │ create index idx on t (a collate c);
          ╰╴                                 ─ 1. source
        "#);
    }

    #[test]
    fn goto_create_collation_from() {
        assert_snapshot!(goto("
create collation c1 (locale = 'C');
create collation c2 from c1$0;
"), @"
          ╭▸ 
        2 │ create collation c1 (locale = 'C');
          │                  ── 2. destination
        3 │ create collation c2 from c1;
          ╰╴                          ─ 1. source
        ");
    }

    #[test]
    fn goto_create_server_foreign_data_wrapper() {
        assert_snapshot!(goto("
create foreign data wrapper fdw;
create server srv foreign data wrapper fdw$0;
"), @"
          ╭▸ 
        2 │ create foreign data wrapper fdw;
          │                             ─── 2. destination
        3 │ create server srv foreign data wrapper fdw;
          ╰╴                                         ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_sequence() {
        assert_snapshot!(goto("
create sequence s;
alter sequence s$0 restart with 1;
"), @"
          ╭▸ 
        2 │ create sequence s;
          │                 ─ 2. destination
        3 │ alter sequence s restart with 1;
          ╰╴               ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_view() {
        assert_snapshot!(goto("
create view v as select 1 as id;
alter view v$0 rename to v2;
"), @"
          ╭▸ 
        2 │ create view v as select 1 as id;
          │             ─ 2. destination
        3 │ alter view v rename to v2;
          ╰╴           ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_materialized_view() {
        assert_snapshot!(goto("
create materialized view mv as select 1 as id;
alter materialized view mv$0 rename to mv2;
"), @"
          ╭▸ 
        2 │ create materialized view mv as select 1 as id;
          │                          ── 2. destination
        3 │ alter materialized view mv rename to mv2;
          ╰╴                         ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_type() {
        assert_snapshot!(goto("
create type address as (city text);
alter type address$0 add attribute zip text;
"), @"
          ╭▸ 
        2 │ create type address as (city text);
          │             ─────── 2. destination
        3 │ alter type address add attribute zip text;
          ╰╴                 ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_domain() {
        assert_snapshot!(goto("
create domain email as text;
alter domain email$0 set not null;
"), @"
          ╭▸ 
        2 │ create domain email as text;
          │               ───── 2. destination
        3 │ alter domain email set not null;
          ╰╴                 ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_function() {
        assert_snapshot!(goto("
create function f(a int) returns int language sql as $$ select a $$;
alter function f$0(int) owner to me;
"), @"
          ╭▸ 
        2 │ create function f(a int) returns int language sql as $$ select a $$;
          │                 ─ 2. destination
        3 │ alter function f(int) owner to me;
          ╰╴               ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_procedure() {
        assert_snapshot!(goto("
create procedure p(a int) language sql as $$ select 1 $$;
alter procedure p$0(int) rename to q;
"), @"
          ╭▸ 
        2 │ create procedure p(a int) language sql as $$ select 1 $$;
          │                  ─ 2. destination
        3 │ alter procedure p(int) rename to q;
          ╰╴                ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_routine() {
        assert_snapshot!(goto("
create function f() returns int language sql as $$ select 1 $$;
alter routine f$0 rename to g;
"), @"
          ╭▸ 
        2 │ create function f() returns int language sql as $$ select 1 $$;
          │                 ─ 2. destination
        3 │ alter routine f rename to g;
          ╰╴              ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_aggregate() {
        assert_snapshot!(goto("
create aggregate agg (int) (sfunc = f, stype = int8);
alter aggregate agg$0(int) rename to agg2;
"), @"
          ╭▸ 
        2 │ create aggregate agg (int) (sfunc = f, stype = int8);
          │                  ─── 2. destination
        3 │ alter aggregate agg(int) rename to agg2;
          ╰╴                  ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_index() {
        assert_snapshot!(goto("
create table t(id int);
create index idx on t(id);
alter index idx$0 rename to idx2;
"), @"
          ╭▸ 
        3 │ create index idx on t(id);
          │              ─── 2. destination
        4 │ alter index idx rename to idx2;
          ╰╴              ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_schema() {
        assert_snapshot!(goto("
create schema app;
alter schema app$0 rename to app2;
"), @"
          ╭▸ 
        2 │ create schema app;
          │               ─── 2. destination
        3 │ alter schema app rename to app2;
          ╰╴               ─ 1. source
        ");
    }

    #[test]
    fn goto_create_schema_element_unqualified_table_ref() {
        assert_snapshot!(goto("
create schema app
  create table users(id int)
  create view v as
    select id from users$0;
"), @"
          ╭▸ 
        3 │   create table users(id int)
          │                ───── 2. destination
        4 │   create view v as
        5 │     select id from users;
          ╰╴                       ─ 1. source
        ");
    }

    #[test]
    fn goto_create_schema_element_unqualified_column_ref() {
        assert_snapshot!(goto("
create schema app
  create table users(id int)
  create view v as
    select id$0 from users;
"), @"
          ╭▸ 
        3 │   create table users(id int)
          │                      ── 2. destination
        4 │   create view v as
        5 │     select id from users;
          ╰╴            ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_database() {
        assert_snapshot!(goto("
create database appdb;
alter database appdb$0 owner to alice;
"), @"
          ╭▸ 
        2 │ create database appdb;
          │                 ───── 2. destination
        3 │ alter database appdb owner to alice;
          ╰╴                   ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_tablespace() {
        assert_snapshot!(goto("
create tablespace fast location '/tmp/fast';
alter tablespace fast$0 rename to faster;
"), @"
          ╭▸ 
        2 │ create tablespace fast location '/tmp/fast';
          │                   ──── 2. destination
        3 │ alter tablespace fast rename to faster;
          ╰╴                    ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_trigger() {
        assert_snapshot!(goto("
create trigger trg before insert on t for each row execute function f();
alter trigger trg$0 on t rename to trg2;
"), @"
          ╭▸ 
        2 │ create trigger trg before insert on t for each row execute function f();
          │                ─── 2. destination
        3 │ alter trigger trg on t rename to trg2;
          ╰╴                ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_foreign_table() {
        assert_snapshot!(goto("
create foreign table ft(id int) server myserver;
alter foreign table ft$0 owner to alice;
"), @"
          ╭▸ 
        2 │ create foreign table ft(id int) server myserver;
          │                      ── 2. destination
        3 │ alter foreign table ft owner to alice;
          ╰╴                     ─ 1. source
        ");
    }

    #[test]
    fn goto_extension_definition_returns_self() {
        assert_snapshot!(goto("
create extension my$0ext;
"), @r"
          ╭▸ 
        2 │ create extension myext;
          │                  ┬┬───
          │                  ││
          │                  │1. source
          ╰╴                 2. destination
        ");
    }

    #[test]
    fn goto_drop_sequence_with_schema() {
        assert_snapshot!(goto("
create sequence foo.s;
drop sequence foo.s$0;
"), @r"
          ╭▸ 
        2 │ create sequence foo.s;
          │                     ─ 2. destination
        3 │ drop sequence foo.s;
          ╰╴                  ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_table_with_schema() {
        assert_snapshot!(goto("
create table public.t();
drop table t$0;
"), @r"
          ╭▸ 
        2 │ create table public.t();
          │                     ─ 2. destination
        3 │ drop table t;
          ╰╴           ─ 1. source
        ");

        assert_snapshot!(goto("
create table foo.t();
drop table foo.t$0;
"), @r"
          ╭▸ 
        2 │ create table foo.t();
          │                  ─ 2. destination
        3 │ drop table foo.t;
          ╰╴               ─ 1. source
        ");

        goto_not_found(
            "
-- defaults to public schema
create table t();
drop table foo.t$0;
",
        );
    }

    #[test]
    fn goto_drop_temp_table() {
        assert_snapshot!(goto("
create temp table t();
drop table t$0;
"), @r"
          ╭▸ 
        2 │ create temp table t();
          │                   ─ 2. destination
        3 │ drop table t;
          ╰╴           ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_temporary_table() {
        assert_snapshot!(goto("
create temporary table t();
drop table t$0;
"), @r"
          ╭▸ 
        2 │ create temporary table t();
          │                        ─ 2. destination
        3 │ drop table t;
          ╰╴           ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_temp_table_with_pg_temp_schema() {
        assert_snapshot!(goto("
create temp table t();
drop table pg_temp.t$0;
"), @r"
          ╭▸ 
        2 │ create temp table t();
          │                   ─ 2. destination
        3 │ drop table pg_temp.t;
          ╰╴                   ─ 1. source
        ");
    }

    #[test]
    fn goto_table_definition_returns_self() {
        assert_snapshot!(goto("
create table t$0(x bigint, y bigint);
"), @r"
          ╭▸ 
        2 │ create table t(x bigint, y bigint);
          │              ┬
          │              │
          │              2. destination
          ╰╴             1. source
        ");
    }

    #[test]
    fn goto_foreign_table_column() {
        assert_snapshot!(goto("
create foreign table ft(a int)
  server s;

select a$0 from ft;
"), @r"
          ╭▸ 
        2 │ create foreign table ft(a int)
          │                         ─ 2. destination
          ‡
        5 │ select a from ft;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_foreign_table_definition() {
        assert_snapshot!(goto("
create foreign table ft(a int)
  server s;

select a from ft$0;
"), @r"
          ╭▸ 
        2 │ create foreign table ft(a int)
          │                      ── 2. destination
          ‡
        5 │ select a from ft;
          ╰╴               ─ 1. source
        ");
    }

    #[test]
    fn goto_foreign_table_server_name() {
        assert_snapshot!(goto("
create server myserver foreign data wrapper fdw;
create foreign table ft(a int)
  server my$0server;
"), @r"
          ╭▸ 
        2 │ create server myserver foreign data wrapper fdw;
          │               ──────── 2. destination
        3 │ create foreign table ft(a int)
        4 │   server myserver;
          ╰╴          ─ 1. source
        ");
    }

    #[test]
    fn goto_foreign_table_server_name_defined_after() {
        assert_snapshot!(goto("
create foreign table ft(a int)
  server my$0server;
create server myserver foreign data wrapper fdw;
"), @r"
          ╭▸ 
        3 │   server myserver;
          │           ─ 1. source
        4 │ create server myserver foreign data wrapper fdw;
          ╰╴              ──────── 2. destination
        ");
    }

    #[test]
    fn goto_user_mapping_server_name() {
        assert_snapshot!(goto("
create server myserver foreign data wrapper fdw;
create user mapping for current_user server my$0server;
"), @r"
          ╭▸ 
        2 │ create server myserver foreign data wrapper fdw;
          │               ──────── 2. destination
        3 │ create user mapping for current_user server myserver;
          ╰╴                                             ─ 1. source
        ");
    }

    #[test]
    fn goto_foreign_key_references_table() {
        assert_snapshot!(goto("
create table foo(id int);
create table bar(
  id int,
  foo_id int,
  foreign key (foo_id) references foo$0(id)
);
"), @r"
          ╭▸ 
        2 │ create table foo(id int);
          │              ─── 2. destination
          ‡
        6 │   foreign key (foo_id) references foo(id)
          ╰╴                                    ─ 1. source
        ");
    }

    #[test]
    fn goto_foreign_key_on_delete_set_null_column() {
        assert_snapshot!(goto("
create table users (
  user_id integer not null,
  primary key (user_id)
);

create table posts (
  post_id integer not null,
  author_id integer,
  primary key (post_id),
  foreign key (author_id) references users on delete set null (author_id$0)
);
"), @r"
           ╭▸ 
         9 │   author_id integer,
           │   ───────── 2. destination
        10 │   primary key (post_id),
        11 │   foreign key (author_id) references users on delete set null (author_id)
           ╰╴                                                                       ─ 1. source
        ");
    }

    #[test]
    fn goto_references_constraint_table() {
        assert_snapshot!(goto("
create table t (
  id serial primary key
);

create table u (
  id serial primary key,
  t_id int references t$0
);
"), @r"
          ╭▸ 
        2 │ create table t (
          │              ─ 2. destination
          ‡
        8 │   t_id int references t
          ╰╴                      ─ 1. source
        ");
    }

    #[test]
    fn goto_references_constraint_column() {
        assert_snapshot!(goto("
create table t (
  id serial primary key
);

create table u (
  id serial primary key,
  t_id int references t(id$0)
);
"), @r"
          ╭▸ 
        3 │   id serial primary key
          │   ── 2. destination
          ‡
        8 │   t_id int references t(id)
          ╰╴                         ─ 1. source
        ");
    }

    #[test]
    fn goto_foreign_key_references_column() {
        assert_snapshot!(goto("
create table foo(id int);
create table bar(
  id int,
  foo_id int,
  foreign key (foo_id) references foo(id$0)
);
"), @r"
          ╭▸ 
        2 │ create table foo(id int);
          │                  ── 2. destination
          ‡
        6 │   foreign key (foo_id) references foo(id)
          ╰╴                                       ─ 1. source
        ");
    }

    #[test]
    fn goto_foreign_key_local_column() {
        assert_snapshot!(goto("
create table bar(
  id int,
  foo_id int,
  foreign key (foo_id$0) references foo(id)
);
"), @r"
          ╭▸ 
        4 │   foo_id int,
          │   ────── 2. destination
        5 │   foreign key (foo_id) references foo(id)
          ╰╴                    ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_table_foreign_key_local_column() {
        assert_snapshot!(goto("
create table t (
  id bigserial primary key
);

create table u (
  id bigserial primary key,
  t_id bigint
);

alter table u
  add constraint fooo_fkey
  foreign key (t_id$0) references t (id);
"), @r"
           ╭▸ 
         8 │   t_id bigint
           │   ──── 2. destination
           ‡
        13 │   foreign key (t_id) references t (id);
           ╰╴                  ─ 1. source
        ");
    }

    #[test]
    fn goto_check_constraint_column() {
        assert_snapshot!(goto("
create table t (
  b int check (b > 10),
  c int check (c$0 > 10) no inherit
);
"), @r"
          ╭▸ 
        4 │   c int check (c > 10) no inherit
          │   ┬            ─ 1. source
          │   │
          ╰╴  2. destination
        ");
    }

    #[test]
    fn goto_generated_column() {
        assert_snapshot!(goto("
create table t (
  a int,
  b int generated always as (
    a$0 * 2
  ) stored
);
"), @r"
          ╭▸ 
        3 │   a int,
          │   ─ 2. destination
        4 │   b int generated always as (
        5 │     a * 2
          ╰╴    ─ 1. source
        ");
    }

    #[test]
    fn goto_generated_column_function_call() {
        assert_snapshot!(goto("
create function pg_catalog.lower(text) returns text
  language internal;

create table articles (
  id serial primary key,
  title text not null,
  body text not null,
  title_lower text generated always as (
    lower$0(title)
  ) stored
);
"), @r"
           ╭▸ 
         2 │ create function pg_catalog.lower(text) returns text
           │                            ───── 2. destination
           ‡
        10 │     lower(title)
           ╰╴        ─ 1. source
        ");
    }

    #[test]
    fn goto_index_expr_function_call() {
        assert_snapshot!(goto("
create function lower(text) returns text language internal;
create table articles (
  id serial primary key,
  title text not null
);
create index on articles (lower$0(title));
"), @r"
          ╭▸ 
        2 │ create function lower(text) returns text language internal;
          │                 ───── 2. destination
          ‡
        7 │ create index on articles (lower(title));
          ╰╴                              ─ 1. source
        ");
    }

    #[test]
    fn goto_exclude_constraint_expr_function_call() {
        assert_snapshot!(goto("
create function lower(text) returns text language internal;
create table articles (
  title text not null,
  exclude using btree (lower$0(title) with =)
);
"), @r"
          ╭▸ 
        2 │ create function lower(text) returns text language internal;
          │                 ───── 2. destination
          ‡
        5 │   exclude using btree (lower(title) with =)
          ╰╴                           ─ 1. source
        ");
    }

    #[test]
    fn goto_partition_by_expr_function_call() {
        assert_snapshot!(goto("
create function lower(text) returns text language internal;
create table articles (
  id serial primary key,
  title text not null
) partition by range (lower$0(title));
"), @r"
          ╭▸ 
        2 │ create function lower(text) returns text language internal;
          │                 ───── 2. destination
          ‡
        6 │ ) partition by range (lower(title));
          ╰╴                          ─ 1. source
        ");
    }

    #[test]
    fn goto_table_check_constraint_column() {
        assert_snapshot!(goto("
create table t (
  a int,
  b text,
  check (a$0 > b)
);
"), @r"
          ╭▸ 
        3 │   a int,
          │   ─ 2. destination
        4 │   b text,
        5 │   check (a > b)
          ╰╴         ─ 1. source
        ");
    }

    #[test]
    fn goto_table_unique_constraint_column() {
        assert_snapshot!(goto("
create table t (
  a int,
  b text,
  unique (a$0)
);
"), @r"
          ╭▸ 
        3 │   a int,
          │   ─ 2. destination
        4 │   b text,
        5 │   unique (a)
          ╰╴          ─ 1. source
        ");
    }

    #[test]
    fn goto_table_primary_key_constraint_column() {
        assert_snapshot!(goto("
create table t (
  id bigint generated always as identity,
  inserted_at timestamptz not null default now(),
  primary key (id, inserted_at$0)
);
"), @r"
          ╭▸ 
        4 │   inserted_at timestamptz not null default now(),
          │   ─────────── 2. destination
        5 │   primary key (id, inserted_at)
          ╰╴                             ─ 1. source
        ");
    }

    #[test]
    fn goto_table_not_null_constraint_column() {
        assert_snapshot!(goto("
create table t (
  id integer,
  name text,
  not null name$0
);
"), @r"
          ╭▸ 
        4 │   name text,
          │   ──── 2. destination
        5 │   not null name
          ╰╴              ─ 1. source
        ");
    }

    #[test]
    fn goto_table_exclude_constraint_column() {
        assert_snapshot!(goto("
create table circles (
  c circle,
  exclude using gist (c$0 with &&)
);
"), @r"
          ╭▸ 
        3 │   c circle,
          │   ─ 2. destination
        4 │   exclude using gist (c with &&)
          ╰╴                      ─ 1. source
        ");
    }

    #[test]
    fn goto_table_exclude_constraint_include_column() {
        assert_snapshot!(goto("
create table t (
  a int,
  b text,
  exclude using btree ( a with > ) 
    include (a$0, b)
);
"), @r"
          ╭▸ 
        3 │   a int,
          │   ─ 2. destination
          ‡
        6 │     include (a, b)
          ╰╴             ─ 1. source
        ");
    }

    #[test]
    fn goto_table_exclude_constraint_where_column() {
        assert_snapshot!(goto("
create table t (
  a int,
  b text,
  exclude using btree ( a with > ) 
    where ( a$0 > 10 and b like '%foo' )
);
"), @r"
          ╭▸ 
        3 │   a int,
          │   ─ 2. destination
          ‡
        6 │     where ( a > 10 and b like '%foo' )
          ╰╴            ─ 1. source
        ");
    }

    #[test]
    fn goto_table_partition_by_column() {
        assert_snapshot!(goto("
create table t (
  id bigint generated always as identity,
  inserted_at timestamptz not null default now()
) partition by range (inserted_at$0);
"), @r"
          ╭▸ 
        4 │   inserted_at timestamptz not null default now()
          │   ─────────── 2. destination
        5 │ ) partition by range (inserted_at);
          ╰╴                                ─ 1. source
        ");
    }

    #[test]
    fn goto_table_partition_of_table() {
        assert_snapshot!(goto("
create table t ();
create table t_2026_01_02 partition of t$0
    for values from ('2026-01-02') to ('2026-01-03');
"), @r"
          ╭▸ 
        2 │ create table t ();
          │              ─ 2. destination
        3 │ create table t_2026_01_02 partition of t
          ╰╴                                       ─ 1. source
        ");
    }

    #[test]
    fn goto_table_partition_of_cycle() {
        goto_not_found(
            "
create table part1 partition of part2
    for values from ('2026-01-02') to ('2026-01-03');
create table part2 partition of part1
    for values from ('2026-01-02') to ('2026-01-03');
select a$0 from part2;
",
        );
    }

    #[test]
    fn goto_partition_table_column() {
        assert_snapshot!(goto("
create table part (
  a int,
  inserted_at timestamptz not null default now()
) partition by range (inserted_at);
create table part_2026_01_02 partition of part
    for values from ('2026-01-02') to ('2026-01-03');
select a$0 from part_2026_01_02;
"), @r"
          ╭▸ 
        3 │   a int,
          │   ─ 2. destination
          ‡
        8 │ select a from part_2026_01_02;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_partition_table_qualified_column() {
        assert_snapshot!(goto("
create table part (
  a int,
  inserted_at timestamptz not null default now()
) partition by range (inserted_at);
create table part_2026_01_02 partition of part
    for values from ('2026-01-02') to ('2026-01-03');
select part_2026_01_02.a$0 from part_2026_01_02;
"), @"
          ╭▸ 
        3 │   a int,
          │   ─ 2. destination
          ‡
        8 │ select part_2026_01_02.a from part_2026_01_02;
          ╰╴                       ─ 1. source
        ");
    }

    #[test]
    fn goto_partition_table_qualified_column_multi_level() {
        assert_snapshot!(goto("
create table p (a int) partition by list (a);
create table m partition of p for values in (1) partition by list (a);
create table c partition of m for values in (2);
select c.a$0 from c;
"), @"
          ╭▸ 
        2 │ create table p (a int) partition by list (a);
          │                 ─ 2. destination
          ‡
        5 │ select c.a from c;
          ╰╴         ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_index_attach_partition() {
        assert_snapshot!(goto("
create table t (
  inserted_at timestamptz not null default now()
) partition by range (inserted_at);
create table part partition of t
    for values from ('2026-01-02') to ('2026-01-03');
create index parent_idx on t (inserted_at);
create index child_idx on part (inserted_at);
alter index parent_idx attach partition child_$0idx;
"), @"
          ╭▸ 
        8 │ create index child_idx on part (inserted_at);
          │              ───────── 2. destination
        9 │ alter index parent_idx attach partition child_idx;
          ╰╴                                             ─ 1. source
        ");
    }

    #[test]
    fn goto_create_table_like_clause() {
        assert_snapshot!(goto("
create table large_data_table(a text);
create table t (
  a text,
  like large_data_table$0,
  b integer
);
"), @r"
          ╭▸ 
        2 │ create table large_data_table(a text);
          │              ──────────────── 2. destination
          ‡
        5 │   like large_data_table,
          ╰╴                      ─ 1. source
        ");
    }

    #[test]
    fn goto_create_table_like_view() {
        assert_snapshot!(goto("
create view v as select 1 a, 2 b;
create table t (like v);
select a$0 from t;
"), @"
          ╭▸ 
        2 │ create view v as select 1 a, 2 b;
          │                           ─ 2. destination
        3 │ create table t (like v);
        4 │ select a from t;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_view_select_star_column_gap() {
        assert_snapshot!(goto("
create table t(a int, b int);
create view v as select * from t;
select a$0 from v;
"), @"
          ╭▸ 
        2 │ create table t(a int, b int);
          │                ─ 2. destination
        3 │ create view v as select * from t;
        4 │ select a from v;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_view_table_query_column_gap() {
        assert_snapshot!(goto("
create table t(a int);
create view v as table t;
select a$0 from v;
"), @"
          ╭▸ 
        2 │ create table t(a int);
          │                ─ 2. destination
        3 │ create view v as table t;
        4 │ select a from v;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_view_values_query_column_gap() {
        assert_snapshot!(goto("
create view v as values (1, 2);
select column2$0 from v;
"), @"
          ╭▸ 
        2 │ create view v as values (1, 2);
          │                             ─ 2. destination
        3 │ select column2 from v;
          ╰╴             ─ 1. source
        ");
    }

    #[test]
    fn goto_view_compound_table_query_column() {
        assert_snapshot!(goto("
create table t(a int);
create view v as table t union table t;
select a$0 from v;
"), @"
          ╭▸ 
        2 │ create table t(a int);
          │                ─ 2. destination
        3 │ create view v as table t union table t;
        4 │ select a from v;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_view_compound_values_query_column() {
        assert_snapshot!(goto("
create view v as values (1, 2) union values (3, 4);
select column2$0 from v;
"), @"
          ╭▸ 
        2 │ create view v as values (1, 2) union values (3, 4);
          │                             ─ 2. destination
        3 │ select column2 from v;
          ╰╴             ─ 1. source
        ");
    }

    #[test]
    fn goto_view_table_query_column_count_gap() {
        assert_snapshot!(goto("
create table t(a int, b int);
create view v as table t;
select b$0 from (select * from v) u(a);
"), @"
          ╭▸ 
        2 │ create table t(a int, b int);
          │                       ─ 2. destination
        3 │ create view v as table t;
        4 │ select b from (select * from v) u(a);
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_view_values_query_column_count_gap() {
        assert_snapshot!(goto("
create view v as values (1, 2);
select column2$0 from (select * from v) u(a);
"), @"
          ╭▸ 
        2 │ create view v as values (1, 2);
          │                             ─ 2. destination
        3 │ select column2 from (select * from v) u(a);
          ╰╴             ─ 1. source
        ");
    }

    #[test]
    fn goto_values_partial_alias_remaining_column_gap() {
        assert_snapshot!(goto("
select column2$0 from (values (1, 2)) v(a);
"), @"
          ╭▸ 
        2 │ select column2 from (values (1, 2)) v(a);
          ╰╴             ─ 1. source        ─ 2. destination
        ");
    }

    #[test]
    fn goto_create_table_inherits() {
        assert_snapshot!(goto("
create table bar(a int);
create table t (a int)
inherits (foo.bar, bar$0, buzz);
"), @r"
          ╭▸ 
        2 │ create table bar(a int);
          │              ─── 2. destination
        3 │ create table t (a int)
        4 │ inherits (foo.bar, bar, buzz);
          ╰╴                     ─ 1. source
        ");
    }

    #[test]
    fn goto_create_table_inherits_builtin() {
        assert_snapshot!(goto("
-- include-builtins
create table t ()
inherits (information_schema.sql_features);
select feature_name$0 from t;
"), @"
            ╭▸ current.sql:5:19
            │
          5 │ select feature_name from t;
            │                   ─ 1. source
            ╰╴

            ╭▸ builtins.sql:437:3
            │
        437 │   feature_name information_schema.character_data,
            ╰╴  ──────────── 2. destination
        ");
    }

    #[test]
    fn goto_create_table_like_clause_columns() {
        assert_snapshot!(goto("
create table t(a int, b int);
create table u(like t, c int);
select a$0, c from u;
"), @r"
          ╭▸ 
        2 │ create table t(a int, b int);
          │                ─ 2. destination
        3 │ create table u(like t, c int);
        4 │ select a, c from u;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_create_table_like_clause_local_column() {
        assert_snapshot!(goto("
create table t(a int, b int);
create table u(like t, c int);
select a, c$0 from u;
"), @r"
          ╭▸ 
        3 │ create table u(like t, c int);
          │                        ─ 2. destination
        4 │ select a, c from u;
          ╰╴          ─ 1. source
        ");
    }

    #[test]
    fn goto_create_table_like_clause_multi() {
        assert_snapshot!(goto("
create table t(a int, b int);
create table u(x int, y int);
create table k(like t, like u, c int);
select y$0 from k;
"), @r"
          ╭▸ 
        3 │ create table u(x int, y int);
          │                       ─ 2. destination
        4 │ create table k(like t, like u, c int);
        5 │ select y from k;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_create_table_inherits_column() {
        assert_snapshot!(goto("
create table t (
  a int, b text
);
create table u (
  c int
) inherits (t);
select a$0 from u;
"), @r"
          ╭▸ 
        3 │   a int, b text
          │   ─ 2. destination
          ‡
        8 │ select a from u;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_create_table_inherits_local_column() {
        assert_snapshot!(goto("
create table t (
  a int, b text
);
create table u (
  c int
) inherits (t);
select c$0 from u;
"), @r"
          ╭▸ 
        6 │   c int
          │   ─ 2. destination
        7 │ ) inherits (t);
        8 │ select c from u;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_create_table_inherits_multiple_parents() {
        assert_snapshot!(goto("
create table t1 (
  a int
);
create table t2 (
  b text
);
create table u (
  c int
) inherits (t1, t2);
select b$0 from u;
"), @r"
           ╭▸ 
         6 │   b text
           │   ─ 2. destination
           ‡
        11 │ select b from u;
           ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_create_foreign_table_inherits_column() {
        assert_snapshot!(goto("
create server myserver foreign data wrapper postgres_fdw;
create table t (
  a int, b text
);
create foreign table u (
  c int
) inherits (t) server myserver;
select a$0 from u;
"), @r"
          ╭▸ 
        4 │   a int, b text
          │   ─ 2. destination
          ‡
        9 │ select a from u;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_temp_table_shadows_public() {
        // temp tables shadow public tables when no schema is specified
        assert_snapshot!(goto("
create table t();
create temp table t();
drop table t$0;
"), @"
          ╭▸ 
        3 │ create temp table t();
          │                   ─ 2. destination
        4 │ drop table t;
          ╰╴           ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_public_table_when_temp_exists() {
        // can still access public table explicitly
        assert_snapshot!(goto("
create table t();
create temp table t();
drop table public.t$0;
"), @r"
          ╭▸ 
        2 │ create table t();
          │              ─ 2. destination
        3 │ create temp table t();
        4 │ drop table public.t;
          ╰╴                  ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_table_defined_after() {
        assert_snapshot!(goto("
drop table t$0;
create table t();
"), @r"
          ╭▸ 
        2 │ drop table t;
          │            ─ 1. source
        3 │ create table t();
          ╰╴             ─ 2. destination
        ");
    }

    #[test]
    fn goto_drop_type() {
        assert_snapshot!(goto("
create type t as enum ('a', 'b');
drop type t$0;
"), @r"
          ╭▸ 
        2 │ create type t as enum ('a', 'b');
          │             ─ 2. destination
        3 │ drop type t;
          ╰╴          ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_type_with_schema() {
        assert_snapshot!(goto("
create type public.t as enum ('a', 'b');
drop type t$0;
"), @r"
          ╭▸ 
        2 │ create type public.t as enum ('a', 'b');
          │                    ─ 2. destination
        3 │ drop type t;
          ╰╴          ─ 1. source
        ");

        assert_snapshot!(goto("
create type foo.t as enum ('a', 'b');
drop type foo.t$0;
"), @r"
          ╭▸ 
        2 │ create type foo.t as enum ('a', 'b');
          │                 ─ 2. destination
        3 │ drop type foo.t;
          ╰╴              ─ 1. source
        ");

        goto_not_found(
            "
create type t as enum ('a', 'b');
drop type foo.t$0;
",
        );
    }

    #[test]
    fn goto_drop_type_defined_after() {
        assert_snapshot!(goto("
drop type t$0;
create type t as enum ('a', 'b');
"), @r"
          ╭▸ 
        2 │ drop type t;
          │           ─ 1. source
        3 │ create type t as enum ('a', 'b');
          ╰╴            ─ 2. destination
        ");
    }

    #[test]
    fn goto_drop_type_composite() {
        assert_snapshot!(goto("
create type person as (name text, age int);
drop type person$0;
"), @r"
          ╭▸ 
        2 │ create type person as (name text, age int);
          │             ────── 2. destination
        3 │ drop type person;
          ╰╴               ─ 1. source
        ");
    }

    #[test]
    fn goto_create_table_type_reference() {
        assert_snapshot!(goto("
create type person_info as (name text, email text);
create table users(id int, member person_info$0);
"), @"
          ╭▸ 
        2 │ create type person_info as (name text, email text);
          │             ─────────── 2. destination
        3 │ create table users(id int, member person_info);
          ╰╴                                            ─ 1. source
        ");
    }

    #[test]
    fn goto_function_param_table_type() {
        assert_snapshot!(goto("
create table t(a int, b int);
create function b(t$0) returns int as 'select 1' language sql;
"), @r"
          ╭▸ 
        2 │ create table t(a int, b int);
          │              ─ 2. destination
        3 │ create function b(t) returns int as 'select 1' language sql;
          ╰╴                  ─ 1. source
        ");
    }

    #[test]
    fn goto_function_param_time_type() {
        assert_snapshot!(goto("
create type timestamp;
create function f(timestamp$0 without time zone) returns text language internal;
"), @r"
          ╭▸ 
        2 │ create type timestamp;
          │             ───────── 2. destination
        3 │ create function f(timestamp without time zone) returns text language internal;
          ╰╴                          ─ 1. source
        ");
    }

    #[test]
    fn goto_function_param_time_type_no_timezone() {
        assert_snapshot!(goto("
create type time;
create function f(time$0) returns text language internal;
"), @r"
  ╭▸ 
2 │ create type time;
  │             ──── 2. destination
3 │ create function f(time) returns text language internal;
  ╰╴                     ─ 1. source
");
    }

    #[test]
    fn goto_create_table_type_reference_enum() {
        assert_snapshot!(goto("
create type mood as enum ('sad', 'ok', 'happy');
create table users(id int, mood mood$0);
"), @r"
          ╭▸ 
        2 │ create type mood as enum ('sad', 'ok', 'happy');
          │             ──── 2. destination
        3 │ create table users(id int, mood mood);
          ╰╴                                   ─ 1. source
        ");
    }

    #[test]
    fn goto_create_table_type_reference_range() {
        assert_snapshot!(goto("
create type int4_range as range (subtype = int4);
create table metrics(id int, span int4_range$0);
"), @r"
          ╭▸ 
        2 │ create type int4_range as range (subtype = int4);
          │             ────────── 2. destination
        3 │ create table metrics(id int, span int4_range);
          ╰╴                                           ─ 1. source
        ");
    }

    #[test]
    fn goto_create_table_type_reference_input_output() {
        assert_snapshot!(goto("
create type myint (input = myintin, output = myintout, like = int4);
create table data(id int, value myint$0);
"), @r"
          ╭▸ 
        2 │ create type myint (input = myintin, output = myintout, like = int4);
          │             ───── 2. destination
        3 │ create table data(id int, value myint);
          ╰╴                                    ─ 1. source
        ");
    }

    #[test]
    fn goto_composite_type_field() {
        assert_snapshot!(goto("
create type person_info as (name text, email text);
create table users(id int, member person_info);
select (member).name$0 from users;
"), @"
          ╭▸ 
        2 │ create type person_info as (name text, email text);
          │                             ──── 2. destination
        3 │ create table users(id int, member person_info);
        4 │ select (member).name from users;
          ╰╴                   ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_type_drop_attribute() {
        assert_snapshot!(goto("
create type address as (city text, zip text);
alter type address drop attribute city$0;
"), @"
          ╭▸ 
        2 │ create type address as (city text, zip text);
          │                         ──── 2. destination
        3 │ alter type address drop attribute city;
          ╰╴                                     ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_type_rename_attribute() {
        assert_snapshot!(goto("
create type address as (city text, zip text);
alter type address rename attribute city$0 to town;
"), @"
          ╭▸ 
        2 │ create type address as (city text, zip text);
          │                         ──── 2. destination
        3 │ alter type address rename attribute city to town;
          ╰╴                                       ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_type_alter_attribute() {
        assert_snapshot!(goto("
create type address as (city text, zip text);
alter type address alter attribute city$0 set data type varchar;
"), @"
          ╭▸ 
        2 │ create type address as (city text, zip text);
          │                         ──── 2. destination
        3 │ alter type address alter attribute city set data type varchar;
          ╰╴                                      ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_type_range() {
        assert_snapshot!(goto("
create type int4_range as range (subtype = int4);
drop type int4_range$0;
"), @r"
          ╭▸ 
        2 │ create type int4_range as range (subtype = int4);
          │             ────────── 2. destination
        3 │ drop type int4_range;
          ╰╴                   ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_domain() {
        assert_snapshot!(goto("
create domain posint as integer check (value > 0);
drop domain posint$0;
"), @r"
          ╭▸ 
        2 │ create domain posint as integer check (value > 0);
          │               ────── 2. destination
        3 │ drop domain posint;
          ╰╴                 ─ 1. source
        ");
    }

    #[test]
    fn goto_cast_to_domain() {
        assert_snapshot!(goto("
create domain posint as integer check (value > 0);
select 1::posint$0;
"), @r"
          ╭▸ 
        2 │ create domain posint as integer check (value > 0);
          │               ────── 2. destination
        3 │ select 1::posint;
          ╰╴               ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_type_domain() {
        assert_snapshot!(goto("
create domain posint as integer check (value > 0);
drop type posint$0;
"), @r"
          ╭▸ 
        2 │ create domain posint as integer check (value > 0);
          │               ────── 2. destination
        3 │ drop type posint;
          ╰╴               ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_view() {
        assert_snapshot!(goto("
create view v as select 1;
drop view v$0;
"), @r"
          ╭▸ 
        2 │ create view v as select 1;
          │             ─ 2. destination
        3 │ drop view v;
          ╰╴          ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_materialized_view() {
        assert_snapshot!(goto("
create materialized view v as select 1;
drop materialized view v$0;
"), @r"
          ╭▸ 
        2 │ create materialized view v as select 1;
          │                          ─ 2. destination
        3 │ drop materialized view v;
          ╰╴                       ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_view_with_schema() {
        assert_snapshot!(goto("
create view public.v as select 1;
drop view v$0;
"), @r"
          ╭▸ 
        2 │ create view public.v as select 1;
          │                    ─ 2. destination
        3 │ drop view v;
          ╰╴          ─ 1. source
        ");

        assert_snapshot!(goto("
create view foo.v as select 1;
drop view foo.v$0;
"), @r"
          ╭▸ 
        2 │ create view foo.v as select 1;
          │                 ─ 2. destination
        3 │ drop view foo.v;
          ╰╴              ─ 1. source
        ");

        goto_not_found(
            "
create view v as select 1;
drop view foo.v$0;
",
        );
    }

    #[test]
    fn goto_drop_temp_view() {
        assert_snapshot!(goto("
create temp view v as select 1;
drop view v$0;
"), @r"
          ╭▸ 
        2 │ create temp view v as select 1;
          │                  ─ 2. destination
        3 │ drop view v;
          ╰╴          ─ 1. source
        ");
    }

    #[test]
    fn goto_select_from_view() {
        assert_snapshot!(goto("
create view v as select 1;
select * from v$0;
"), @r"
          ╭▸ 
        2 │ create view v as select 1;
          │             ─ 2. destination
        3 │ select * from v;
          ╰╴              ─ 1. source
        ");
    }

    #[test]
    fn goto_select_from_materialized_view() {
        assert_snapshot!(goto("
create materialized view v as select 1;
select * from v$0;
"), @r"
          ╭▸ 
        2 │ create materialized view v as select 1;
          │                          ─ 2. destination
        3 │ select * from v;
          ╰╴              ─ 1. source
        ");
    }

    #[test]
    fn goto_select_from_view_with_schema() {
        assert_snapshot!(goto("
create view public.v as select 1;
select * from public.v$0;
"), @r"
          ╭▸ 
        2 │ create view public.v as select 1;
          │                    ─ 2. destination
        3 │ select * from public.v;
          ╰╴                     ─ 1. source
        ");
    }

    #[test]
    fn goto_view_column() {
        assert_snapshot!(goto("
create view v as select 1 as a;
select a$0 from v;
"), @r"
          ╭▸ 
        2 │ create view v as select 1 as a;
          │                              ─ 2. destination
        3 │ select a from v;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_view_column_qualified() {
        assert_snapshot!(goto("
create view v as select 1 as a;
select v.a$0 from v;
"), @r"
          ╭▸ 
        2 │ create view v as select 1 as a;
          │                              ─ 2. destination
        3 │ select v.a from v;
          ╰╴         ─ 1. source
        ");
    }

    #[test]
    fn goto_materialized_view_column_with_explicit_column_list() {
        assert_snapshot!(goto("
create materialized view mv (x, y) as select 1 as a, 2 as b;
select x$0 from mv;
"), @r"
          ╭▸ 
        2 │ create materialized view mv (x, y) as select 1 as a, 2 as b;
          │                              ─ 2. destination
        3 │ select x from mv;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_view_table_qualifier() {
        assert_snapshot!(goto("
create view v as select 1 id, 2 b;
select v$0.id from v;
"), @"
          ╭▸ 
        2 │ create view v as select 1 id, 2 b;
          │             ─ 2. destination
        3 │ select v.id from v;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_select_into_column() {
        assert_snapshot!(goto("
select 1 a into t;
select a$0 from t;
"), @"
          ╭▸ 
        2 │ select 1 a into t;
          │          ─ 2. destination
        3 │ select a from t;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_select_into_table() {
        assert_snapshot!(goto("
select 1 a into t;
select a from t$0;
"), @"
          ╭▸ 
        2 │ select 1 a into t;
          │                 ─ 2. destination
        3 │ select a from t;
          ╰╴              ─ 1. source
        ");
    }

    #[test]
    fn goto_select_into_select_star() {
        assert_snapshot!(goto("
create table t(a bigint);
select * into u from t;
select a$0 from u;
"), @"
          ╭▸ 
        2 │ create table t(a bigint);
          │                ─ 2. destination
        3 │ select * into u from t;
        4 │ select a from u;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_select_into_source_table() {
        assert_snapshot!(goto("
create table t(a int);
select a into u from t$0;
"), @"
          ╭▸ 
        2 │ create table t(a int);
          │              ─ 2. destination
        3 │ select a into u from t;
          ╰╴                     ─ 1. source
        ");
    }

    #[test]
    fn goto_select_into_target_list_column() {
        assert_snapshot!(goto("
create table t(a int);
select a$0 into u from t;
"), @"
          ╭▸ 
        2 │ create table t(a int);
          │                ─ 2. destination
        3 │ select a into u from t;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_select_into_where_column() {
        assert_snapshot!(goto("
create table t(a int);
select a into u from t where a$0 > 0;
"), @"
          ╭▸ 
        2 │ create table t(a int);
          │                ─ 2. destination
        3 │ select a into u from t where a > 0;
          ╰╴                             ─ 1. source
        ");
    }

    #[test]
    fn goto_select_into_qualified_column() {
        assert_snapshot!(goto("
create table t(a int);
select t.a$0 into u from t;
"), @"
          ╭▸ 
        2 │ create table t(a int);
          │                ─ 2. destination
        3 │ select t.a into u from t;
          ╰╴         ─ 1. source
        ");
    }

    #[test]
    fn goto_create_table_as_column() {
        assert_snapshot!(goto("
create table t as select 1 a;
select a$0 from t;
"), @r"
          ╭▸ 
        2 │ create table t as select 1 a;
          │                            ─ 2. destination
        3 │ select a from t;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_create_table_as_table() {
        assert_snapshot!(goto("
create table t(a bigint);
create table u as table t;
select a$0 from u;
"), @"
          ╭▸ 
        2 │ create table t(a bigint);
          │                ─ 2. destination
        3 │ create table u as table t;
        4 │ select a from u;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_create_table_as_select_star() {
        assert_snapshot!(goto("
create table t(a bigint);
create table u as select * from t;
select a$0 from u;
"), @"
          ╭▸ 
        2 │ create table t(a bigint);
          │                ─ 2. destination
        3 │ create table u as select * from t;
        4 │ select a from u;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_create_table_as_values() {
        assert_snapshot!(goto("
create table k as values (1, 2);
select column1$0 from k;
"), @"
          ╭▸ 
        2 │ create table k as values (1, 2);
          │                           ─ 2. destination
        3 │ select column1 from k;
          ╰╴             ─ 1. source
        ");
    }

    #[test]
    fn goto_create_table_as_compound_table_query_column() {
        assert_snapshot!(goto("
create table t(a int);
create table u as table t union table t;
select a$0 from u;
"), @"
          ╭▸ 
        2 │ create table t(a int);
          │                ─ 2. destination
        3 │ create table u as table t union table t;
        4 │ select a from u;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_create_table_as_compound_values_query_column() {
        assert_snapshot!(goto("
create table u as values (1, 2) union values (3, 4);
select column2$0 from u;
"), @"
          ╭▸ 
        2 │ create table u as values (1, 2) union values (3, 4);
          │                              ─ 2. destination
        3 │ select column2 from u;
          ╰╴             ─ 1. source
        ");
    }

    #[test]
    fn goto_create_table_as_paren_table_query_column() {
        assert_snapshot!(goto("
create table t(a int);
create table u as (table t);
select a$0 from u;
"), @"
          ╭▸ 
        2 │ create table t(a int);
          │                ─ 2. destination
        3 │ create table u as (table t);
        4 │ select a from u;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_create_table_as_paren_values_query_column() {
        assert_snapshot!(goto("
create table u as (values (1, 2));
select column2$0 from u;
"), @"
          ╭▸ 
        2 │ create table u as (values (1, 2));
          │                               ─ 2. destination
        3 │ select column2 from u;
          ╰╴             ─ 1. source
        ");
    }

    #[test]
    fn goto_create_table_as_values_column_count_gap() {
        assert_snapshot!(goto("
create table u as values (1, 2);
select column2$0 from (select * from u) x(a);
"), @"
          ╭▸ 
        2 │ create table u as values (1, 2);
          │                              ─ 2. destination
        3 │ select column2 from (select * from u) x(a);
          ╰╴             ─ 1. source
        ");
    }

    #[test]
    fn goto_select_from_create_table_as() {
        assert_snapshot!(goto("
create table t as select 1 a;
select a from t$0;
"), @r"
          ╭▸ 
        2 │ create table t as select 1 a;
          │              ─ 2. destination
        3 │ select a from t;
          ╰╴              ─ 1. source
        ");
    }

    #[test]
    fn goto_like_view_definition() {
        assert_snapshot!(goto("
create view v as select 1 a;
create table t (like v$0);
"), @"
          ╭▸ 
        2 │ create view v as select 1 a;
          │             ─ 2. destination
        3 │ create table t (like v);
          ╰╴                     ─ 1. source
        ");
    }

    #[test]
    fn goto_view_with_explicit_column_list() {
        assert_snapshot!(goto("
create view v(col1) as select 1;
select * from v$0;
"), @r"
          ╭▸ 
        2 │ create view v(col1) as select 1;
          │             ─ 2. destination
        3 │ select * from v;
          ╰╴              ─ 1. source
        ");
    }

    #[test]
    fn goto_view_column_with_explicit_column_list() {
        assert_snapshot!(goto("
    create view v(col1) as select 1;
    select col1$0 from v;
    "), @r"
          ╭▸ 
        2 │     create view v(col1) as select 1;
          │                   ──── 2. destination
        3 │     select col1 from v;
          ╰╴              ─ 1. source
        ");
    }

    #[test]
    fn goto_create_table_as_with_explicit_column_list() {
        assert_snapshot!(goto("
create table t (a int);
create table t2 (x) as select a from t;
select x$0 from t2;
"), @"
          ╭▸ 
        3 │ create table t2 (x) as select a from t;
          │                  ─ 2. destination
        4 │ select x from t2;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_create_table_as_explicit_column_list_shorter_than_select() {
        assert_snapshot!(goto("
create table t2 (x) as select 1 a, 2 b;
select b$0 from t2;
"), @"
          ╭▸ 
        2 │ create table t2 (x) as select 1 a, 2 b;
          │                                      ─ 2. destination
        3 │ select b from t2;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_create_table_as_explicit_column_list_shadows_select_column() {
        goto_not_found(
            "
create table t (a int);
create table t2 (x) as select a from t;
select a$0 from t2;
",
        );
    }

    #[test]
    fn goto_view_column_with_schema() {
        assert_snapshot!(goto("
create view public.v as select 1 as a;
select a$0 from public.v;
"), @r"
          ╭▸ 
        2 │ create view public.v as select 1 as a;
          │                                     ─ 2. destination
        3 │ select a from public.v;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_view_multiple_columns() {
        assert_snapshot!(goto("
create view v as select 1 as a, 2 as b;
select b$0 from v;
"), @r"
          ╭▸ 
        2 │ create view v as select 1 as a, 2 as b;
          │                                      ─ 2. destination
        3 │ select b from v;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_view_column_from_table() {
        assert_snapshot!(goto("
create table t(x int, y int);
create view v as select x, y from t;
select x$0 from v;
"), @r"
          ╭▸ 
        3 │ create view v as select x, y from t;
          │                         ─ 2. destination
        4 │ select x from v;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_view_column_with_table_preference() {
        assert_snapshot!(goto("
create table v(a int);
create view vw as select 1 as a;
select a$0 from v;
"), @r"
          ╭▸ 
        2 │ create table v(a int);
          │                ─ 2. destination
        3 │ create view vw as select 1 as a;
        4 │ select a from v;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_cast_operator() {
        assert_snapshot!(goto("
create type foo as enum ('a', 'b');
select x::foo$0;
"), @r"
          ╭▸ 
        2 │ create type foo as enum ('a', 'b');
          │             ─── 2. destination
        3 │ select x::foo;
          ╰╴            ─ 1. source
        ");
    }

    #[test]
    fn goto_cast_function() {
        assert_snapshot!(goto("
create type bar as enum ('x', 'y');
select cast(x as bar$0);
"), @r"
          ╭▸ 
        2 │ create type bar as enum ('x', 'y');
          │             ─── 2. destination
        3 │ select cast(x as bar);
          ╰╴                   ─ 1. source
        ");
    }

    #[test]
    fn goto_cast_with_schema() {
        assert_snapshot!(goto("
create type public.baz as enum ('m', 'n');
select x::public.baz$0;
"), @r"
          ╭▸ 
        2 │ create type public.baz as enum ('m', 'n');
          │                    ─── 2. destination
        3 │ select x::public.baz;
          ╰╴                   ─ 1. source
        ");
    }

    #[test]
    fn goto_cast_timestamp_without_time_zone() {
        assert_snapshot!(goto("
create type pg_catalog.timestamp;
select ''::timestamp without$0 time zone;
"), @r"
          ╭▸ 
        2 │ create type pg_catalog.timestamp;
          │                        ───────── 2. destination
        3 │ select ''::timestamp without time zone;
          ╰╴                           ─ 1. source
        ");
    }

    #[test]
    fn goto_cast_timestamp_with_time_zone() {
        assert_snapshot!(goto("
create type pg_catalog.timestamptz;
select ''::timestamp with$0 time zone;
"), @r"
          ╭▸ 
        2 │ create type pg_catalog.timestamptz;
          │                        ─────────── 2. destination
        3 │ select ''::timestamp with time zone;
          ╰╴                        ─ 1. source
        ");
    }

    #[test]
    fn goto_cast_multirange_type_from_range() {
        assert_snapshot!(goto("
create type floatrange as range (
  subtype = float8,
  subtype_diff = float8mi
);
select '{[1.234, 5.678]}'::floatmultirange$0;
"), @r"
          ╭▸ 
        2 │ create type floatrange as range (
          │             ────────── 2. destination
          ‡
        6 │ select '{[1.234, 5.678]}'::floatmultirange;
          ╰╴                                         ─ 1. source
        ");
    }

    #[test]
    fn goto_cast_multirange_special_type_name_string() {
        assert_snapshot!(goto("
create type floatrange as range (
  subtype = float8,
  subtype_diff = float8mi,
  multirange_type_name = 'floatmulirangething'
);
select '{[1.234, 5.678]}'::floatmulirangething$0;
"), @r"
          ╭▸ 
        2 │ create type floatrange as range (
          │             ────────── 2. destination
          ‡
        7 │ select '{[1.234, 5.678]}'::floatmulirangething;
          ╰╴                                             ─ 1. source
        ");
    }

    #[test]
    fn goto_cast_multirange_special_type_name_ident() {
        assert_snapshot!(goto("
create type floatrange as range (
  subtype = float8,
  subtype_diff = float8mi,
  multirange_type_name = floatrangemutirange
);
select '{[1.234, 5.678]}'::floatrangemutirange$0;
"), @r"
          ╭▸ 
        2 │ create type floatrange as range (
          │             ────────── 2. destination
          ‡
        7 │ select '{[1.234, 5.678]}'::floatrangemutirange;
          ╰╴                                             ─ 1. source
        ");
    }

    #[test]
    fn goto_cast_multirange_edge_case_type_from_range() {
        // make sure we're calculating the multirange correctly
        assert_snapshot!(goto("
create type floatrangerange as range (
  subtype = float8,
  subtype_diff = float8mi
);
select '{[1.234, 5.678]}'::floatmultirangerange$0;
"), @r"
          ╭▸ 
        2 │ create type floatrangerange as range (
          │             ─────────────── 2. destination
          ‡
        6 │ select '{[1.234, 5.678]}'::floatmultirangerange;
          ╰╴                                              ─ 1. source
        ");
    }

    #[test]
    fn goto_cast_boolean_falls_back_to_bool() {
        assert_snapshot!(goto("
create type pg_catalog.bool;
select '1'::boolean$0;
"), @"
          ╭▸ 
        2 │ create type pg_catalog.bool;
          │                        ──── 2. destination
        3 │ select '1'::boolean;
          ╰╴                  ─ 1. source
        ");
    }

    #[test]
    fn goto_cast_decimal_falls_back_to_numeric() {
        assert_snapshot!(goto("
create type pg_catalog.numeric;
select 1::decimal$0(10, 2);
"), @"
          ╭▸ 
        2 │ create type pg_catalog.numeric;
          │                        ─────── 2. destination
        3 │ select 1::decimal(10, 2);
          ╰╴                ─ 1. source
        ");
    }

    #[test]
    fn goto_cast_float_falls_back_to_float8() {
        assert_snapshot!(goto("
create type pg_catalog.float8;
select 1::float$0;
"), @"
          ╭▸ 
        2 │ create type pg_catalog.float8;
          │                        ────── 2. destination
        3 │ select 1::float;
          ╰╴              ─ 1. source
        ");
    }

    #[test]
    fn goto_cast_bigint_falls_back_to_int8() {
        assert_snapshot!(goto("
create type pg_catalog.int8;
select 1::bigint$0;
"), @r"
          ╭▸ 
        2 │ create type pg_catalog.int8;
          │                        ──── 2. destination
        3 │ select 1::bigint;
          ╰╴               ─ 1. source
        ");
    }

    #[test]
    fn goto_cast_real_falls_back_to_float4() {
        assert_snapshot!(goto("
create type pg_catalog.float4;
select 1::real$0;
"), @"
          ╭▸ 
        2 │ create type pg_catalog.float4;
          │                        ────── 2. destination
        3 │ select 1::real;
          ╰╴             ─ 1. source
        ");
    }

    #[test]
    fn goto_cast_bigint_prefers_user_type() {
        assert_snapshot!(goto("
create type bigint;
create type pg_catalog.int8;
select 1::bigint$0;
"), @r"
          ╭▸ 
        2 │ create type bigint;
          │             ────── 2. destination
        3 │ create type pg_catalog.int8;
        4 │ select 1::bigint;
          ╰╴               ─ 1. source
        ");
    }

    #[test]
    fn goto_cast_smallserial_falls_back_to_int2() {
        assert_snapshot!(goto("
create type pg_catalog.int2;
select 1::smallserial$0;
"), @r"
          ╭▸ 
        2 │ create type pg_catalog.int2;
          │                        ──── 2. destination
        3 │ select 1::smallserial;
          ╰╴                    ─ 1. source
        ");
    }

    #[test]
    fn goto_cast_serial2_falls_back_to_int2() {
        assert_snapshot!(goto("
create type pg_catalog.int2;
select 1::serial2$0;
"), @r"
          ╭▸ 
        2 │ create type pg_catalog.int2;
          │                        ──── 2. destination
        3 │ select 1::serial2;
          ╰╴                ─ 1. source
        ");
    }

    #[test]
    fn goto_cast_serial_falls_back_to_int4() {
        assert_snapshot!(goto("
create type pg_catalog.int4;
select 1::serial$0;
"), @r"
          ╭▸ 
        2 │ create type pg_catalog.int4;
          │                        ──── 2. destination
        3 │ select 1::serial;
          ╰╴               ─ 1. source
        ");
    }

    #[test]
    fn goto_cast_serial4_falls_back_to_int4() {
        assert_snapshot!(goto("
create type pg_catalog.int4;
select 1::serial4$0;
"), @r"
          ╭▸ 
        2 │ create type pg_catalog.int4;
          │                        ──── 2. destination
        3 │ select 1::serial4;
          ╰╴                ─ 1. source
        ");
    }

    #[test]
    fn goto_cast_bigserial_falls_back_to_int8() {
        assert_snapshot!(goto("
create type pg_catalog.int8;
select 1::bigserial$0;
"), @r"
          ╭▸ 
        2 │ create type pg_catalog.int8;
          │                        ──── 2. destination
        3 │ select 1::bigserial;
          ╰╴                  ─ 1. source
        ");
    }

    #[test]
    fn goto_cast_serial8_falls_back_to_int8() {
        assert_snapshot!(goto("
create type pg_catalog.int8;
select 1::serial8$0;
"), @r"
          ╭▸ 
        2 │ create type pg_catalog.int8;
          │                        ──── 2. destination
        3 │ select 1::serial8;
          ╰╴                ─ 1. source
        ");
    }

    #[test]
    fn goto_cast_int_falls_back_to_int4() {
        assert_snapshot!(goto("
create type pg_catalog.int4;
select 1::int$0;
"), @r"
          ╭▸ 
        2 │ create type pg_catalog.int4;
          │                        ──── 2. destination
        3 │ select 1::int;
          ╰╴            ─ 1. source
        ");
    }

    #[test]
    fn goto_cast_integer_falls_back_to_int4() {
        assert_snapshot!(goto("
create type pg_catalog.int4;
select 1::integer$0;
"), @r"
          ╭▸ 
        2 │ create type pg_catalog.int4;
          │                        ──── 2. destination
        3 │ select 1::integer;
          ╰╴                ─ 1. source
        ");
    }

    #[test]
    fn goto_cast_smallint_falls_back_to_int2() {
        assert_snapshot!(goto("
create type pg_catalog.int2;
select 1::smallint$0;
"), @r"
          ╭▸ 
        2 │ create type pg_catalog.int2;
          │                        ──── 2. destination
        3 │ select 1::smallint;
          ╰╴                 ─ 1. source
        ");
    }

    #[test]
    fn goto_cast_double_precision_falls_back_to_float8() {
        assert_snapshot!(goto("
create type pg_catalog.float8;
select '1'::double precision[]$0;
"), @r"
          ╭▸ 
        2 │ create type pg_catalog.float8;
          │                        ────── 2. destination
        3 │ select '1'::double precision[];
          ╰╴                             ─ 1. source
        ");
    }

    #[test]
    fn goto_cast_varchar_with_modifier() {
        assert_snapshot!(goto("
create type pg_catalog.varchar;
select '1'::varchar$0(1);
"), @r"
          ╭▸ 
        2 │ create type pg_catalog.varchar;
          │                        ─────── 2. destination
        3 │ select '1'::varchar(1);
          ╰╴                  ─ 1. source
        ");
    }

    #[test]
    fn goto_cast_composite_type() {
        assert_snapshot!(goto("
create type person_info as (name varchar(50), age int);
select ('Alice', 30)::person_info$0;
"), @r"
          ╭▸ 
        2 │ create type person_info as (name varchar(50), age int);
          │             ─────────── 2. destination
        3 │ select ('Alice', 30)::person_info;
          ╰╴                                ─ 1. source
        ");
    }

    #[test]
    fn goto_cast_composite_type_in_cte() {
        assert_snapshot!(goto("
create type person_info as (name varchar(50), age int);
with team as (
    select 1 as id, ('Alice', 30)::person_info$0 as member
)
select * from team;
"), @r"
          ╭▸ 
        2 │ create type person_info as (name varchar(50), age int);
          │             ─────────── 2. destination
        3 │ with team as (
        4 │     select 1 as id, ('Alice', 30)::person_info as member
          ╰╴                                             ─ 1. source
        ");
    }

    #[test]
    fn goto_composite_type_field_name() {
        assert_snapshot!(goto("
create type person_info as (name varchar(50), age int);
with team as (
    select 1 as id, ('Alice', 30)::person_info as member
)
select (member).name$0, (member).age from team;
"), @r"
          ╭▸ 
        2 │ create type person_info as (name varchar(50), age int);
          │                             ──── 2. destination
          ‡
        6 │ select (member).name, (member).age from team;
          ╰╴                   ─ 1. source
        ");
    }

    #[test]
    fn goto_composite_type_field_in_where() {
        assert_snapshot!(goto("
create type person_info as (name varchar(50), age int);
with team as (
    select 1 as id, ('Alice', 30)::person_info as member
    union all
    select 2, ('Bob', 25)::person_info
)
select (member).name, (member).age
from team
where (member).age$0 >= 18;
"), @r"
           ╭▸ 
         2 │ create type person_info as (name varchar(50), age int);
           │                                               ─── 2. destination
           ‡
        10 │ where (member).age >= 18;
           ╰╴                 ─ 1. source
        ");
    }

    #[test]
    fn goto_composite_type_field_base() {
        assert_snapshot!(goto("
create type person_info as (name varchar(50), age int);
with team as (
    select 1 as id, ('Alice', 30)::person_info as member
)
select (member$0).age from team;
"), @r"
          ╭▸ 
        4 │     select 1 as id, ('Alice', 30)::person_info as member
          │                                                   ────── 2. destination
        5 │ )
        6 │ select (member).age from team;
          ╰╴             ─ 1. source
        ");
    }

    #[test]
    fn goto_composite_type_field_nested_parens() {
        assert_snapshot!(goto("
create type person_info as (name varchar(50), age int);
with team as (
    select 1 as id, ('Alice', 30)::person_info as member
)
select ((((member))).name$0) from team;
"), @r"
          ╭▸ 
        2 │ create type person_info as (name varchar(50), age int);
          │                             ──── 2. destination
          ‡
        6 │ select ((((member))).name) from team;
          ╰╴                        ─ 1. source
        ");
    }

    #[test]
    fn goto_whole_row_field_access() {
        assert_snapshot!(goto("
create table t (a int);
select (t).a$0 from t;
"), @"
          ╭▸ 
        2 │ create table t (a int);
          │                 ─ 2. destination
        3 │ select (t).a from t;
          ╰╴           ─ 1. source
        ");
    }

    #[test]
    fn begin_to_rollback() {
        assert_snapshot!(goto(
            "
begin$0;
select 1;
rollback;
commit;
",
        ), @"
          ╭▸ 
        2 │ begin;
          │     ─ 1. source
        3 │ select 1;
        4 │ rollback;
          ╰╴───────── 2. destination
        ");
    }

    #[test]
    fn commit_to_begin() {
        assert_snapshot!(goto(
            "
begin;
select 1;
commit$0;
",
        ), @"
          ╭▸ 
        2 │ begin;
          │ ────── 2. destination
        3 │ select 1;
        4 │ commit;
          ╰╴     ─ 1. source
        ");
    }

    #[test]
    fn begin_to_commit() {
        assert_snapshot!(goto(
            "
begin$0;
select 1;
commit;
",
        ), @"
          ╭▸ 
        2 │ begin;
          │     ─ 1. source
        3 │ select 1;
        4 │ commit;
          ╰╴─────── 2. destination
        ");
    }

    #[test]
    fn commit_to_start_transaction() {
        assert_snapshot!(goto(
            "
start transaction;
select 1;
commit$0;
",
        ), @"
          ╭▸ 
        2 │ start transaction;
          │ ────────────────── 2. destination
        3 │ select 1;
        4 │ commit;
          ╰╴     ─ 1. source
        ");
    }

    #[test]
    fn start_transaction_to_commit() {
        assert_snapshot!(goto(
            "
start$0 transaction;
select 1;
commit;
",
        ), @"
          ╭▸ 
        2 │ start transaction;
          │     ─ 1. source
        3 │ select 1;
        4 │ commit;
          ╰╴─────── 2. destination
        ");
    }

    #[test]
    fn goto_with_search_path() {
        assert_snapshot!(goto(r#"
set search_path to "foo", public;
create table foo.t();
drop table t$0;
"#), @r"
          ╭▸ 
        3 │ create table foo.t();
          │                  ─ 2. destination
        4 │ drop table t;
          ╰╴           ─ 1. source
        ");
    }

    #[test]
    fn goto_with_search_path_and_unspecified_table() {
        assert_snapshot!(goto(r#"
set search_path to foo,bar;
create table t();
drop table foo.t$0;
"#), @r"
          ╭▸ 
        3 │ create table t();
          │              ─ 2. destination
        4 │ drop table foo.t;
          ╰╴               ─ 1. source
        ");
    }

    #[test]
    fn goto_with_search_path_via_set_config() {
        assert_snapshot!(goto(r#"
select set_config('search_path', 'foo, public', false);
create table foo.t();
drop table t$0;
"#), @"
          ╭▸ 
        3 │ create table foo.t();
          │                  ─ 2. destination
        4 │ drop table t;
          ╰╴           ─ 1. source
        ");
    }

    #[test]
    fn goto_with_search_path_via_set_config_unrelated_setting() {
        goto_not_found(
            r#"
select set_config('work_mem', '64MB', false);
create table foo.t();
drop table t$0;
"#,
        );
    }

    #[test]
    fn goto_with_search_path_via_set_config_user_defined_function() {
        goto_not_found(
            r#"
create function set_config(text, text, boolean) returns text as $$ select $2 $$ language sql;
select set_config('search_path', 'foo', false);
create table foo.t();
drop table t$0;
"#,
        );
    }

    #[test]
    fn goto_with_search_path_via_set_config_user_defined_function_outside_search_path() {
        assert_snapshot!(goto(r#"
create schema other;
create function other.set_config(text, text, boolean) returns text as $$ select $2 $$ language sql;
select set_config('search_path', 'foo', false);
create table foo.t();
drop table t$0;
"#), @"
          ╭▸ 
        5 │ create table foo.t();
          │                  ─ 2. destination
        6 │ drop table t;
          ╰╴           ─ 1. source
        ");
    }

    #[test]
    fn goto_with_search_path_via_set_config_pg_catalog_qualified() {
        assert_snapshot!(goto(r#"
select pg_catalog.set_config('search_path', 'foo', false);
create table foo.t();
drop table t$0;
"#), @"
          ╭▸ 
        3 │ create table foo.t();
          │                  ─ 2. destination
        4 │ drop table t;
          ╰╴           ─ 1. source
        ");
    }

    #[test]
    fn goto_with_search_path_via_set_config_non_pg_catalog_qualified() {
        goto_not_found(
            r#"
select public.set_config('search_path', 'foo', false);
create table foo.t();
drop table t$0;
"#,
        );
    }

    #[test]
    fn goto_column_not_in_cte_but_in_table() {
        // we shouldn't navigate up to the table of the same name
        goto_not_found(
            r"
create table t (c int);
with t as (select 1 a)
select c$0 from t;
",
        );
    }

    #[test]
    fn goto_with_search_path_empty() {
        goto_not_found(
            r#"
set search_path = '';
create table t();
drop table t$0;
"#,
        );
    }

    #[test]
    fn goto_with_search_path_like_variable() {
        // not actually search path
        goto_not_found(
            "
set bar.search_path to foo, public;
create table foo.t();
drop table t$0;
",
        )
    }

    #[test]
    fn goto_with_search_path_second_schema() {
        assert_snapshot!(goto("
set search_path to foo, bar, public;
create table bar.t();
drop table t$0;
"), @r"
          ╭▸ 
        3 │ create table bar.t();
          │                  ─ 2. destination
        4 │ drop table t;
          ╰╴           ─ 1. source
        ");
    }

    #[test]
    fn goto_with_search_path_skips_first() {
        assert_snapshot!(goto("
set search_path to foo, bar, public;
create table foo.t();
create table bar.t();
drop table t$0;
"), @r"
          ╭▸ 
        3 │ create table foo.t();
          │                  ─ 2. destination
        4 │ create table bar.t();
        5 │ drop table t;
          ╰╴           ─ 1. source
        ");
    }

    #[test]
    fn goto_without_search_path_uses_default() {
        assert_snapshot!(goto("
create table foo.t();
create table public.t();
drop table t$0;
"), @r"
          ╭▸ 
        3 │ create table public.t();
          │                     ─ 2. destination
        4 │ drop table t;
          ╰╴           ─ 1. source
        ");
    }

    #[test]
    fn goto_with_set_schema() {
        assert_snapshot!(goto("
set schema 'myschema';
create table myschema.t();
drop table t$0;
"), @r"
          ╭▸ 
        3 │ create table myschema.t();
          │                       ─ 2. destination
        4 │ drop table t;
          ╰╴           ─ 1. source
        ");
    }

    #[test]
    fn goto_with_set_schema_ignores_other_schemas() {
        assert_snapshot!(goto("
set schema 'myschema';
create table public.t();
create table myschema.t();
drop table t$0;
"), @r"
          ╭▸ 
        4 │ create table myschema.t();
          │                       ─ 2. destination
        5 │ drop table t;
          ╰╴           ─ 1. source
        ");
    }

    #[test]
    fn goto_search_path_schema_name() {
        assert_snapshot!(goto("
create schema app;
set search_path to app$0;
"), @"
          ╭▸ 
        2 │ create schema app;
          │               ─── 2. destination
        3 │ set search_path to app;
          ╰╴                     ─ 1. source
        ");
    }

    #[test]
    fn goto_search_path_schema_name_quoted() {
        assert_snapshot!(goto(r#"
create schema app;
set search_path to "app$0";
"#), @r#"
          ╭▸ 
        2 │ create schema app;
          │               ─── 2. destination
        3 │ set search_path to "app";
          ╰╴                      ─ 1. source
        "#);
    }

    #[test]
    fn goto_search_path_schema_name_string_literal() {
        assert_snapshot!(goto(r#"
create schema app;
set search_path to 'app$0';
"#), @"
          ╭▸ 
        2 │ create schema app;
          │               ─── 2. destination
        3 │ set search_path to 'app';
          ╰╴                      ─ 1. source
        ");
    }

    #[test]
    fn goto_search_path_schema_name_second_item() {
        assert_snapshot!(goto("
create schema app;
set search_path to public, app$0;
"), @"
          ╭▸ 
        2 │ create schema app;
          │               ─── 2. destination
        3 │ set search_path to public, app;
          ╰╴                             ─ 1. source
        ");
    }

    #[test]
    fn goto_search_path_schema_name_not_the_param_name() {
        goto_not_found(
            "
create schema search_path;
set search_path$0 to app;
",
        );
    }

    #[test]
    fn goto_alter_role_set_search_path() {
        assert_snapshot!(goto("
create schema app;
create role app;
create role r;
alter role r set search_path = app$0;
"), @"
          ╭▸ 
        2 │ create schema app;
          │               ─── 2. destination
          ‡
        5 │ alter role r set search_path = app;
          ╰╴                                 ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_database_set_search_path() {
        assert_snapshot!(goto("
create schema app;
alter database d set search_path = app$0;
"), @"
          ╭▸ 
        2 │ create schema app;
          │               ─── 2. destination
        3 │ alter database d set search_path = app;
          ╰╴                                     ─ 1. source
        ");
    }

    #[test]
    fn goto_create_function_set_search_path() {
        assert_snapshot!(goto("
create schema app;
create function f() returns int language sql as $$ select 1 $$ set search_path = app$0;
"), @"
          ╭▸ 
        2 │ create schema app;
          │               ─── 2. destination
        3 │ create function f() returns int language sql as $$ select 1 $$ set search_path = app;
          ╰╴                                                                                   ─ 1. source
        ");
    }

    #[test]
    fn goto_function_own_set_search_path_resolves_body_call() {
        assert_snapshot!(goto("
create schema bar;
create function bar.foo() returns int language sql begin atomic select 1; end;
create function caller() returns int language sql set search_path = bar begin atomic select foo$0(); end;
"), @"
          ╭▸ 
        3 │ create function bar.foo() returns int language sql begin atomic select 1; end;
          │                     ─── 2. destination
        4 │ create function caller() returns int language sql set search_path = bar begin atomic select foo(); end;
          ╰╴                                                                                              ─ 1. source
        ");
    }

    #[test]
    fn goto_function_own_set_search_path_does_not_leak_after_body() {
        assert_snapshot!(goto("
create schema bar;
create table bar.t(id int);
create table public.t(id int);
create function caller() returns int language sql set search_path = bar begin atomic select 1; end;
select * from t$0;
"), @"
          ╭▸ 
        4 │ create table public.t(id int);
          │                     ─ 2. destination
        5 │ create function caller() returns int language sql set search_path = bar begin atomic select 1; end;
        6 │ select * from t;
          ╰╴              ─ 1. source
        ");
    }

    #[test]
    fn goto_function_set_search_path_from_current_resolves_body_call() {
        assert_snapshot!(goto("
create schema app;
create function app.target() returns int language sql return 1;
set search_path to app;
create function caller() returns int language sql set search_path from current begin atomic select tar$0get(); end;
"), @"
          ╭▸ 
        3 │ create function app.target() returns int language sql return 1;
          │                     ────── 2. destination
        4 │ set search_path to app;
        5 │ create function caller() returns int language sql set search_path from current begin atomic select target(); end;
          ╰╴                                                                                                     ─ 1. source
        ");
    }

    #[test]
    fn goto_procedure_own_set_search_path_resolves_body_call() {
        assert_snapshot!(goto("
create schema bar;
create function bar.foo() returns int language sql begin atomic select 1; end;
create procedure caller() language sql set search_path = bar begin atomic select foo$0(); end;
"), @"
          ╭▸ 
        3 │ create function bar.foo() returns int language sql begin atomic select 1; end;
          │                     ─── 2. destination
        4 │ create procedure caller() language sql set search_path = bar begin atomic select foo(); end;
          ╰╴                                                                                   ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_function_set_search_path() {
        assert_snapshot!(goto("
create schema app;
create function f() returns int language sql as $$ select 1 $$;
alter function f() set search_path = app$0;
"), @"
          ╭▸ 
        2 │ create schema app;
          │               ─── 2. destination
        3 │ create function f() returns int language sql as $$ select 1 $$;
        4 │ alter function f() set search_path = app;
          ╰╴                                       ─ 1. source
        ");
    }

    #[test]
    fn goto_set_schema_literal() {
        assert_snapshot!(goto("
create schema app;
set schema 'app$0';
"), @"
          ╭▸ 
        2 │ create schema app;
          │               ─── 2. destination
        3 │ set schema 'app';
          ╰╴              ─ 1. source
        ");
    }

    #[test]
    fn goto_with_search_path_changed_twice() {
        assert_snapshot!(goto("
set search_path to foo;
create table foo.t();
set search_path to bar;
create table bar.t();
drop table t$0;
"), @r"
          ╭▸ 
        5 │ create table bar.t();
          │                  ─ 2. destination
        6 │ drop table t;
          ╰╴           ─ 1. source
        ");

        assert_snapshot!(goto("
set search_path to foo;
create table foo.t();
drop table t$0;
set search_path to bar;
create table bar.t();
drop table t;
"), @r"
          ╭▸ 
        3 │ create table foo.t();
          │                  ─ 2. destination
        4 │ drop table t;
          ╰╴           ─ 1. source
        ");
    }

    #[test]
    fn goto_with_empty_search_path() {
        goto_not_found(
            "
set search_path to '';
create table public.t();
drop table t$0;
",
        )
    }

    #[test]
    fn goto_with_search_path_uppercase() {
        assert_snapshot!(goto("
SET SEARCH_PATH TO foo;
create table foo.t();
drop table t$0;
"), @r"
          ╭▸ 
        3 │ create table foo.t();
          │                  ─ 2. destination
        4 │ drop table t;
          ╰╴           ─ 1. source
        ");
    }

    #[test]
    fn goto_table_stmt() {
        assert_snapshot!(goto("
create table t();
table t$0;
"), @r"
          ╭▸ 
        2 │ create table t();
          │              ─ 2. destination
        3 │ table t;
          ╰╴      ─ 1. source
        ");
    }

    #[test]
    fn goto_table_stmt_with_schema() {
        assert_snapshot!(goto("
create table public.t();
table public.t$0;
"), @r"
          ╭▸ 
        2 │ create table public.t();
          │                     ─ 2. destination
        3 │ table public.t;
          ╰╴             ─ 1. source
        ");
    }

    #[test]
    fn goto_table_stmt_with_search_path() {
        assert_snapshot!(goto("
set search_path to foo;
create table foo.t();
table t$0;
"), @r"
          ╭▸ 
        3 │ create table foo.t();
          │                  ─ 2. destination
        4 │ table t;
          ╰╴      ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_index() {
        assert_snapshot!(goto("
create index idx_name on t(x);
drop index idx_name$0;
"), @r"
          ╭▸ 
        2 │ create index idx_name on t(x);
          │              ──────── 2. destination
        3 │ drop index idx_name;
          ╰╴                  ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_index_with_schema() {
        assert_snapshot!(goto(r#"
set search_path to public;
create index idx_name on t(x);
drop index public.idx_name$0;
"#), @r"
          ╭▸ 
        3 │ create index idx_name on t(x);
          │              ──────── 2. destination
        4 │ drop index public.idx_name;
          ╰╴                         ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_index_defined_after() {
        assert_snapshot!(goto("
drop index idx_name$0;
create index idx_name on t(x);
"), @r"
          ╭▸ 
        2 │ drop index idx_name;
          │                   ─ 1. source
        3 │ create index idx_name on t(x);
          ╰╴             ──────── 2. destination
        ");
    }

    #[test]
    fn goto_index_definition_returns_self() {
        assert_snapshot!(goto("
create index idx_name$0 on t(x);
"), @r"
          ╭▸ 
        2 │ create index idx_name on t(x);
          │              ┬──────┬
          │              │      │
          │              │      1. source
          ╰╴             2. destination
        ");
    }

    #[test]
    fn goto_drop_index_with_search_path() {
        assert_snapshot!(goto(r#"
create index idx_name on t(x);
set search_path to bar;
create index idx_name on f(x);
set search_path to default;
drop index idx_name$0;
"#), @r"
          ╭▸ 
        2 │ create index idx_name on t(x);
          │              ──────── 2. destination
          ‡
        6 │ drop index idx_name;
          ╰╴                  ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_index_schema_qualified() {
        assert_snapshot!(goto("
create schema a;
create schema b;
create table a.t(id int);
create table b.t(id int);
create index idx on a.t(id);
create index idx on b.t(id);
drop index b.idx$0;
"), @"
          ╭▸ 
        7 │ create index idx on b.t(id);
          │              ─── 2. destination
        8 │ drop index b.idx;
          ╰╴               ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_index_multiple() {
        assert_snapshot!(goto("
create index idx1 on t(x);
create index idx2 on t(y);
drop index idx1, idx2$0;
"), @r"
          ╭▸ 
        3 │ create index idx2 on t(y);
          │              ──── 2. destination
        4 │ drop index idx1, idx2;
          ╰╴                    ─ 1. source
        ");
    }

    #[test]
    fn goto_create_index_table() {
        assert_snapshot!(goto("
create table users(id int);
create index idx_users on users$0(id);
"), @r"
          ╭▸ 
        2 │ create table users(id int);
          │              ───── 2. destination
        3 │ create index idx_users on users(id);
          ╰╴                              ─ 1. source
        ");
    }

    #[test]
    fn goto_create_index_table_with_schema() {
        assert_snapshot!(goto("
create table public.users(id int);
create index idx_users on public.users$0(id);
"), @r"
          ╭▸ 
        2 │ create table public.users(id int);
          │                     ───── 2. destination
        3 │ create index idx_users on public.users(id);
          ╰╴                                     ─ 1. source
        ");
    }

    #[test]
    fn goto_create_index_table_with_search_path() {
        assert_snapshot!(goto(r#"
set search_path to foo;
create table foo.users(id int);
create index idx_users on users$0(id);
"#), @r"
          ╭▸ 
        3 │ create table foo.users(id int);
          │                  ───── 2. destination
        4 │ create index idx_users on users(id);
          ╰╴                              ─ 1. source
        ");
    }

    #[test]
    fn goto_create_index_temp_table() {
        assert_snapshot!(goto("
create temp table users(id int);
create index idx_users on users$0(id);
"), @r"
          ╭▸ 
        2 │ create temp table users(id int);
          │                   ───── 2. destination
        3 │ create index idx_users on users(id);
          ╰╴                              ─ 1. source
        ");
    }

    #[test]
    fn goto_create_index_column() {
        assert_snapshot!(goto("
create table users(id int, email text);
create index idx_email on users(email$0);
"), @r"
          ╭▸ 
        2 │ create table users(id int, email text);
          │                            ───── 2. destination
        3 │ create index idx_email on users(email);
          ╰╴                                    ─ 1. source
        ");
    }

    #[test]
    fn goto_create_index_first_column() {
        assert_snapshot!(goto("
create table users(id int, email text);
create index idx_id on users(id$0);
"), @r"
          ╭▸ 
        2 │ create table users(id int, email text);
          │                    ── 2. destination
        3 │ create index idx_id on users(id);
          ╰╴                              ─ 1. source
        ");
    }

    #[test]
    fn goto_create_index_multiple_columns() {
        assert_snapshot!(goto("
create table users(id int, email text, name text);
create index idx_users on users(id, email$0, name);
"), @r"
          ╭▸ 
        2 │ create table users(id int, email text, name text);
          │                            ───── 2. destination
        3 │ create index idx_users on users(id, email, name);
          ╰╴                                        ─ 1. source
        ");
    }

    #[test]
    fn goto_create_index_column_with_schema() {
        assert_snapshot!(goto("
create table public.users(id int, email text);
create index idx_email on public.users(email$0);
"), @r"
          ╭▸ 
        2 │ create table public.users(id int, email text);
          │                                   ───── 2. destination
        3 │ create index idx_email on public.users(email);
          ╰╴                                           ─ 1. source
        ");
    }

    #[test]
    fn goto_create_index_column_temp_table() {
        assert_snapshot!(goto("
create temp table users(id int, email text);
create index idx_email on users(email$0);
"), @r"
          ╭▸ 
        2 │ create temp table users(id int, email text);
          │                                 ───── 2. destination
        3 │ create index idx_email on users(email);
          ╰╴                                    ─ 1. source
        ");
    }

    #[test]
    fn goto_create_index_include_column() {
        assert_snapshot!(goto("
create table users(id int, email text);
create index idx on users(id) include (email$0);
"), @r"
          ╭▸ 
        2 │ create table users(id int, email text);
          │                            ───── 2. destination
        3 │ create index idx on users(id) include (email);
          ╰╴                                           ─ 1. source
        ");
    }

    #[test]
    fn goto_create_index_where_column() {
        assert_snapshot!(goto("
create table users(id int, email text);
create index idx on users(id) where email$0 is not null;
"), @r"
          ╭▸ 
        2 │ create table users(id int, email text);
          │                            ───── 2. destination
        3 │ create index idx on users(id) where email is not null;
          ╰╴                                        ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_function() {
        assert_snapshot!(goto("
create function foo() returns int as $$ select 1 $$ language sql;
drop function foo$0();
"), @r"
          ╭▸ 
        2 │ create function foo() returns int as $$ select 1 $$ language sql;
          │                 ─── 2. destination
        3 │ drop function foo();
          ╰╴                ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_function_with_schema() {
        assert_snapshot!(goto("
set search_path to public;
create function foo() returns int as $$ select 1 $$ language sql;
drop function public.foo$0();
"), @r"
          ╭▸ 
        3 │ create function foo() returns int as $$ select 1 $$ language sql;
          │                 ─── 2. destination
        4 │ drop function public.foo();
          ╰╴                       ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_function_defined_after() {
        assert_snapshot!(goto("
drop function foo$0();
create function foo() returns int as $$ select 1 $$ language sql;
"), @r"
          ╭▸ 
        2 │ drop function foo();
          │                 ─ 1. source
        3 │ create function foo() returns int as $$ select 1 $$ language sql;
          ╰╴                ─── 2. destination
        ");
    }

    #[test]
    fn goto_function_definition_returns_self() {
        assert_snapshot!(goto("
create function foo$0() returns int as $$ select 1 $$ language sql;
"), @r"
          ╭▸ 
        2 │ create function foo() returns int as $$ select 1 $$ language sql;
          │                 ┬─┬
          │                 │ │
          │                 │ 1. source
          ╰╴                2. destination
        ");
    }

    #[test]
    fn goto_drop_function_with_search_path() {
        assert_snapshot!(goto("
create function foo() returns int as $$ select 1 $$ language sql;
set search_path to bar;
create function foo() returns int as $$ select 1 $$ language sql;
set search_path to default;
drop function foo$0();
"), @r"
          ╭▸ 
        2 │ create function foo() returns int as $$ select 1 $$ language sql;
          │                 ─── 2. destination
          ‡
        6 │ drop function foo();
          ╰╴                ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_function_multiple() {
        assert_snapshot!(goto("
create function foo() returns int as $$ select 1 $$ language sql;
create function bar() returns int as $$ select 1 $$ language sql;
drop function foo(), bar$0();
"), @r"
          ╭▸ 
        3 │ create function bar() returns int as $$ select 1 $$ language sql;
          │                 ─── 2. destination
        4 │ drop function foo(), bar();
          ╰╴                       ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_function_overloaded() {
        assert_snapshot!(goto("
create function add(complex) returns complex as $$ select null $$ language sql;
create function add(bigint) returns bigint as $$ select 1 $$ language sql;
drop function add$0(complex);
"), @r"
          ╭▸ 
        2 │ create function add(complex) returns complex as $$ select null $$ language sql;
          │                 ─── 2. destination
        3 │ create function add(bigint) returns bigint as $$ select 1 $$ language sql;
        4 │ drop function add(complex);
          ╰╴                ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_function_second_overload() {
        assert_snapshot!(goto("
create function add(complex) returns complex as $$ select null $$ language sql;
create function add(bigint) returns bigint as $$ select 1 $$ language sql;
drop function add$0(bigint);
"), @r"
          ╭▸ 
        3 │ create function add(bigint) returns bigint as $$ select 1 $$ language sql;
          │                 ─── 2. destination
        4 │ drop function add(bigint);
          ╰╴                ─ 1. source
        ");
    }

    #[test]
    fn goto_select_function_call() {
        assert_snapshot!(goto("
create function foo() returns int as $$ select 1 $$ language sql;
select foo$0();
"), @r"
          ╭▸ 
        2 │ create function foo() returns int as $$ select 1 $$ language sql;
          │                 ─── 2. destination
        3 │ select foo();
          ╰╴         ─ 1. source
        ");
    }

    #[test]
    fn goto_select_column_from_function_return_table() {
        assert_snapshot!(goto(r#"
create function dup(int) returns table(f1 int, f2 text)
  as ''
  language sql;

select f1$0 from dup(42);
"#), @r"
          ╭▸ 
        2 │ create function dup(int) returns table(f1 int, f2 text)
          │                                        ── 2. destination
          ‡
        6 │ select f1 from dup(42);
          ╰╴        ─ 1. source
        ");
    }

    #[test]
    fn goto_select_column_from_function_return_table_with_schema() {
        assert_snapshot!(goto(r#"
create function myschema.dup(int) returns table(f1 int, f2 text)
  as ''
  language sql;
create function otherschema.dup(int) returns table(f1 int, f2 text)
  as ''
  language sql;

select f1$0 from myschema.dup(42);
"#), @r"
          ╭▸ 
        2 │ create function myschema.dup(int) returns table(f1 int, f2 text)
          │                                                 ── 2. destination
          ‡
        9 │ select f1 from myschema.dup(42);
          ╰╴        ─ 1. source
        ");
    }

    #[test]
    fn goto_select_column_from_function_return_table_paren() {
        assert_snapshot!(goto(r#"
create function dup(int) returns table(f1 int, f2 text)
  as ''
  language sql;

select (dup(42)).f2$0;
"#), @r"
          ╭▸ 
        2 │ create function dup(int) returns table(f1 int, f2 text)
          │                                                ── 2. destination
          ‡
        6 │ select (dup(42)).f2;
          ╰╴                  ─ 1. source
        ");
    }

    #[test]
    fn goto_select_column_from_function_return_table_qualified() {
        assert_snapshot!(goto(r#"
create function dup(int) returns table(f1 int, f2 text)
  as ''
  language sql;

select dup.f1$0 from dup(42);
"#), @r"
          ╭▸ 
        2 │ create function dup(int) returns table(f1 int, f2 text)
          │                                        ── 2. destination
          ‡
        6 │ select dup.f1 from dup(42);
          ╰╴            ─ 1. source
        ");
    }

    #[test]
    fn goto_select_column_from_function_return_table_qualified_function_name() {
        assert_snapshot!(goto(r#"
create function dup(int) returns table(f1 int, f2 text)
  as ''
  language sql;

select dup$0.f1 from dup(42);
"#), @r"
          ╭▸ 
        2 │ create function dup(int) returns table(f1 int, f2 text)
          │                 ─── 2. destination
          ‡
        6 │ select dup.f1 from dup(42);
          ╰╴         ─ 1. source
        ");
    }

    #[test]
    fn goto_select_column_from_function_return_table_qualified_function_name_with_alias() {
        assert_snapshot!(goto(r#"
create function dup(int) returns table(f1 int, f2 text)
  as ''
  language sql;

select dup$0.f2 from dup(42) as dup;
"#), @r"
          ╭▸ 
        6 │ select dup.f2 from dup(42) as dup;
          ╰╴         ─ 1. source          ─── 2. destination
        ");
    }

    #[test]
    fn goto_select_column_from_function_return_table_alias_list() {
        assert_snapshot!(goto(r#"
create function dup(int) returns table(f1 int, f2 text)
  as ''
  language sql;

select a$0 from dup(42) t(a, b);
"#), @r"
          ╭▸ 
        6 │ select a from dup(42) t(a, b);
          ╰╴       ─ 1. source      ─ 2. destination
        ");
    }

    #[test]
    fn goto_select_column_from_function_return_table_alias_list_qualified_partial() {
        assert_snapshot!(goto(r#"
create function dup(int) returns table(f1 int, f2 text)
  as ''
  language sql;

select u.f2$0 from dup(42) as u(x);
"#), @r"
          ╭▸ 
        2 │ create function dup(int) returns table(f1 int, f2 text)
          │                                                ── 2. destination
          ‡
        6 │ select u.f2 from dup(42) as u(x);
          ╰╴          ─ 1. source
        ");
    }

    #[test]
    fn goto_select_column_from_function_return_table_alias_list_unqualified_partial() {
        assert_snapshot!(goto(r#"
create function dup(int) returns table(f1 int, f2 text)
  as ''
  language sql;

select f2$0 from dup(42) as u(x);
"#), @r"
          ╭▸ 
        2 │ create function dup(int) returns table(f1 int, f2 text)
          │                                                ── 2. destination
          ‡
        6 │ select f2 from dup(42) as u(x);
          ╰╴        ─ 1. source
        ");
    }

    #[test]
    fn goto_select_column_from_function_return_table_alias_list_unqualified_not_found() {
        goto_not_found(
            r#"
create function dup(int) returns table(f1 int, f2 text)
  as ''
  language sql;

select f2$0 from dup(42) as u(x, y);
"#,
        );
    }

    #[test]
    fn goto_select_column_from_function_returns_setof_table() {
        assert_snapshot!(goto("
create table users (id int, name text);
create function f() returns setof users
  language sql begin atomic select * from users; end;
select id$0 from f();
"), @"
          ╭▸ 
        2 │ create table users (id int, name text);
          │                     ── 2. destination
          ‡
        5 │ select id from f();
          ╰╴        ─ 1. source
        ");
    }

    #[test]
    fn goto_select_column_from_function_returns_setof_table_qualified() {
        assert_snapshot!(goto("
create table users (id int, name text);
create function f() returns setof users
  language sql begin atomic select * from users; end;
select f.id$0 from f();
"), @"
          ╭▸ 
        2 │ create table users (id int, name text);
          │                     ── 2. destination
          ‡
        5 │ select f.id from f();
          ╰╴          ─ 1. source
        ");
    }

    #[test]
    fn goto_select_column_from_function_returns_setof_composite_type() {
        assert_snapshot!(goto("
create type pt as (x int, y int);
create function f() returns setof pt language sql begin atomic select 1, 2; end;
select x$0 from f();
"), @"
          ╭▸ 
        2 │ create type pt as (x int, y int);
          │                    ─ 2. destination
        3 │ create function f() returns setof pt language sql begin atomic select 1, 2; end;
        4 │ select x from f();
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_select_column_from_scalar_setof_function_alias() {
        assert_snapshot!(goto("
create function nums() returns setof int language sql as $$ values (1) $$;
select n$0 from nums() as n;
"), @"
          ╭▸ 
        3 │ select n from nums() as n;
          ╰╴       ─ 1. source      ─ 2. destination
        ");
    }

    #[test]
    fn goto_select_column_from_scalar_setof_function_no_alias() {
        assert_snapshot!(goto("
create function nums() returns setof int language sql as $$ values (1) $$;
select nums$0 from nums();
"), @"
          ╭▸ 
        3 │ select nums from nums();
          │           ┬      ──── 2. destination
          │           │
          ╰╴          1. source
        ");
    }

    #[test]
    fn goto_select_column_from_scalar_setof_function_alias_with_column_list() {
        assert_snapshot!(goto("
create function nums() returns setof int language sql as $$ values (1) $$;
select x$0 from nums() as n(x);
"), @"
          ╭▸ 
        3 │ select x from nums() as n(x);
          ╰╴       ─ 1. source        ─ 2. destination
        ");
    }

    #[test]
    fn goto_select_column_from_function_out_param() {
        assert_snapshot!(goto("
create function f(out id int, out nm text) returns setof record
  language sql begin atomic select 1, 2; end;
select id$0 from f();
"), @"
          ╭▸ 
        2 │ create function f(out id int, out nm text) returns setof record
          │                       ── 2. destination
        3 │   language sql begin atomic select 1, 2; end;
        4 │ select id from f();
          ╰╴        ─ 1. source
        ");
    }

    #[test]
    fn goto_select_column_from_rows_from() {
        assert_snapshot!(goto("
create function f() returns table(a int) language sql begin atomic select 1; end;
select a$0 from rows from (f());
"), @"
          ╭▸ 
        2 │ create function f() returns table(a int) language sql begin atomic select 1; end;
          │                                   ─ 2. destination
        3 │ select a from rows from (f());
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_select_column_from_xmltable() {
        assert_snapshot!(goto("
create table t (x xml);
select b$0 from t, xmltable(
  '/r' passing x
  columns b int
);
"), @"
          ╭▸ 
        3 │ select b from t, xmltable(
          │        ─ 1. source
        4 │   '/r' passing x
        5 │   columns b int
          ╰╴          ─ 2. destination
        ");
    }

    #[test]
    fn goto_select_column_from_xmltable_aliased() {
        assert_snapshot!(goto("
create table t (x xml);
select xt.b$0 from t, xmltable(
  '/r' passing x
  columns b int
) as xt;
"), @"
          ╭▸ 
        3 │ select xt.b from t, xmltable(
          │           ─ 1. source
        4 │   '/r' passing x
        5 │   columns b int
          ╰╴          ─ 2. destination
        ");
    }

    #[test]
    fn goto_xmltable_passing_clause_qualified_column() {
        assert_snapshot!(goto("
create table t (x xml);
select 1 from t, xmltable(
  '/r' passing t.x$0
  columns b int
);
"), @"
          ╭▸ 
        2 │ create table t (x xml);
          │                 ─ 2. destination
        3 │ select 1 from t, xmltable(
        4 │   '/r' passing t.x
          ╰╴                 ─ 1. source
        ");
    }

    #[test]
    fn goto_select_column_from_json_table() {
        assert_snapshot!(goto("
create table t (j jsonb);
select b$0 from t, json_table(
  t.j, '$[*]'
  columns (b int path '$')
);
"), @"
          ╭▸ 
        3 │ select b from t, json_table(
          │        ─ 1. source
        4 │   t.j, '$[*]'
        5 │   columns (b int path '$')
          ╰╴           ─ 2. destination
        ");
    }

    #[test]
    fn goto_json_table_context_item_qualified_column() {
        assert_snapshot!(goto("
create table t (j jsonb);
select 1 from t, json_table(
  t.j$0, '$[*]'
  columns (b int path '$')
);
"), @"
          ╭▸ 
        2 │ create table t (j jsonb);
          │                 ─ 2. destination
        3 │ select 1 from t, json_table(
        4 │   t.j, '$[*]'
          ╰╴    ─ 1. source
        ");
    }

    #[test]
    fn goto_json_table_plan_root_path() {
        assert_snapshot!(goto("
select * from json_table(
  '{}'::jsonb, '$' as root
  columns (
    nested path '$.items[*]' as items columns (
      value text path '$'
    )
  )
  plan (ro$0ot outer items)
);
"), @"
          ╭▸ 
        3 │   '{}'::jsonb, '$' as root
          │                       ──── 2. destination
          ‡
        9 │   plan (root outer items)
          ╰╴         ─ 1. source
        ");
    }

    #[test]
    fn goto_json_table_plan_nested_path() {
        assert_snapshot!(goto("
select * from json_table(
  '{}'::jsonb, '$' as root
  columns (
    nested path '$.items[*]' as items columns (
      value text path '$'
    )
  )
  plan (root outer ite$0ms)
);
"), @"
          ╭▸ 
        5 │     nested path '$.items[*]' as items columns (
          │                                 ───── 2. destination
          ‡
        9 │   plan (root outer items)
          ╰╴                     ─ 1. source
        ");
    }

    #[test]
    fn goto_fn_call_column_from_cte() {
        assert_snapshot!(goto("
with cte as (select 1 as a)
select a$0(cte) from cte;
"), @"
          ╭▸ 
        2 │ with cte as (select 1 as a)
          │                          ─ 2. destination
        3 │ select a(cte) from cte;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_fn_call_column_from_view() {
        assert_snapshot!(goto("
create view v as select 1 as a;
select a$0(v) from v;
"), @"
          ╭▸ 
        2 │ create view v as select 1 as a;
          │                              ─ 2. destination
        3 │ select a(v) from v;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_select_aggregate_call() {
        assert_snapshot!(goto("
create aggregate foo(int) (
  sfunc = int4pl,
  stype = int,
  initcond = '0'
);

select foo$0(1);
"), @r"
          ╭▸ 
        2 │ create aggregate foo(int) (
          │                  ─── 2. destination
          ‡
        8 │ select foo(1);
          ╰╴         ─ 1. source
        ");
    }

    #[test]
    fn goto_create_aggregate_sfunc() {
        assert_snapshot!(goto("
create function pg_catalog.int8inc(bigint) returns bigint
  language internal;

create aggregate pg_catalog.count(*) (
  sfunc = int8inc$0,
  stype = bigint,
  combinefunc = int8pl,
  initcond = '0'
);
"), @r"
          ╭▸ 
        2 │ create function pg_catalog.int8inc(bigint) returns bigint
          │                            ─────── 2. destination
          ‡
        6 │   sfunc = int8inc,
          ╰╴                ─ 1. source
        "
        );
    }

    #[test]
    fn goto_create_aggregate_combinefunc() {
        assert_snapshot!(goto("
create function pg_catalog.int8pl(bigint, bigint) returns bigint
  language internal;

create aggregate pg_catalog.count(*) (
  sfunc = int8inc,
  stype = bigint,
  combinefunc = int8pl$0,
  initcond = '0'
);
"), @r"
          ╭▸ 
        2 │ create function pg_catalog.int8pl(bigint, bigint) returns bigint
          │                            ────── 2. destination
          ‡
        8 │   combinefunc = int8pl,
          ╰╴                     ─ 1. source
        "
        );
    }

    #[test]
    fn goto_default_constraint_function_call() {
        assert_snapshot!(goto("
create function f() returns int as 'select 1' language sql;
create table t(
  a int default f$0()
);
"), @r"
          ╭▸ 
        2 │ create function f() returns int as 'select 1' language sql;
          │                 ─ 2. destination
        3 │ create table t(
        4 │   a int default f()
          ╰╴                ─ 1. source
        ");
    }

    #[test]
    fn goto_select_function_call_with_schema() {
        assert_snapshot!(goto("
create function public.foo() returns int as $$ select 1 $$ language sql;
select public.foo$0();
"), @r"
          ╭▸ 
        2 │ create function public.foo() returns int as $$ select 1 $$ language sql;
          │                        ─── 2. destination
        3 │ select public.foo();
          ╰╴                ─ 1. source
        ");
    }

    #[test]
    fn goto_select_function_call_with_search_path() {
        assert_snapshot!(goto("
set search_path to myschema;
create function foo() returns int as $$ select 1 $$ language sql;
select myschema.foo$0();
"), @r"
          ╭▸ 
        3 │ create function foo() returns int as $$ select 1 $$ language sql;
          │                 ─── 2. destination
        4 │ select myschema.foo();
          ╰╴                  ─ 1. source
        ");
    }

    #[test]
    fn goto_function_call_style_column_access() {
        assert_snapshot!(goto("
create table t(a int, b int);
select a$0(t) from t;
"), @r"
          ╭▸ 
        2 │ create table t(a int, b int);
          │                ─ 2. destination
        3 │ select a(t) from t;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_function_call_style_column_access_with_function_precedence() {
        assert_snapshot!(goto("
create table t(a int, b int);
create function b(t) returns int as 'select 1' LANGUAGE sql;
select b$0(t) from t;
"), @r"
          ╭▸ 
        3 │ create function b(t) returns int as 'select 1' LANGUAGE sql;
          │                 ─ 2. destination
        4 │ select b(t) from t;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_function_call_style_column_access_table_arg() {
        assert_snapshot!(goto("
create table t(a int, b int);
select a(t$0) from t;
"), @r"
          ╭▸ 
        2 │ create table t(a int, b int);
          │              ─ 2. destination
        3 │ select a(t) from t;
          ╰╴         ─ 1. source
        ");
    }

    #[test]
    fn goto_function_call_style_column_access_table_arg_with_function() {
        assert_snapshot!(goto("
create table t(a int, b int);
create function b(t) returns int as 'select 1' LANGUAGE sql;
select b(t$0) from t;
"), @r"
          ╭▸ 
        2 │ create table t(a int, b int);
          │              ─ 2. destination
        3 │ create function b(t) returns int as 'select 1' LANGUAGE sql;
        4 │ select b(t) from t;
          ╰╴         ─ 1. source
        ");
    }

    #[test]
    fn goto_function_call_multiple_args_not_column_access() {
        goto_not_found(
            "
create table t(a int, b int);
select a$0(t, 1) from t;
",
        );
    }

    #[test]
    fn goto_function_call_nested() {
        assert_snapshot!(goto("
create function f() returns int8
  as 'select 1'
  language sql;
select format('foo%d', f$0());
"), @r"
          ╭▸ 
        2 │ create function f() returns int8
          │                 ─ 2. destination
          ‡
        5 │ select format('foo%d', f());
          ╰╴                       ─ 1. source
        ");
    }

    #[test]
    fn goto_field_style_function_call() {
        assert_snapshot!(goto("
create table t(a int);
create function b(t) returns int as 'select 1' language sql;
select t.b$0 from t;
"), @r"
          ╭▸ 
        3 │ create function b(t) returns int as 'select 1' language sql;
          │                 ─ 2. destination
        4 │ select t.b from t;
          ╰╴         ─ 1. source
        ");
    }

    #[test]
    fn goto_field_style_function_call_column_precedence() {
        assert_snapshot!(goto("
create table t(a int, b int);
create function b(t) returns int as 'select 1' language sql;
select t.b$0 from t;
"), @r"
          ╭▸ 
        2 │ create table t(a int, b int);
          │                       ─ 2. destination
        3 │ create function b(t) returns int as 'select 1' language sql;
        4 │ select t.b from t;
          ╰╴         ─ 1. source
        ");
    }

    #[test]
    fn goto_field_style_function_call_table_ref() {
        assert_snapshot!(goto("
create table t(a int);
create function b(t) returns int as 'select 1' language sql;
select t$0.b from t;
"), @r"
          ╭▸ 
        2 │ create table t(a int);
          │              ─ 2. destination
        3 │ create function b(t) returns int as 'select 1' language sql;
        4 │ select t.b from t;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_function_call_style_in_where() {
        assert_snapshot!(goto("
create table t(a int, b int);
select * from t where a$0(t) > 0;
"), @r"
          ╭▸ 
        2 │ create table t(a int, b int);
          │                ─ 2. destination
        3 │ select * from t where a(t) > 0;
          ╰╴                      ─ 1. source
        ");
    }

    #[test]
    fn goto_function_call_style_in_where_function_precedence() {
        assert_snapshot!(goto("
create table t(a int, b int);
create function b(t) returns int as 'select 1' language sql;
select * from t where b$0(t) > 0;
"), @r"
          ╭▸ 
        3 │ create function b(t) returns int as 'select 1' language sql;
          │                 ─ 2. destination
        4 │ select * from t where b(t) > 0;
          ╰╴                      ─ 1. source
        ");
    }

    #[test]
    fn goto_field_style_function_call_in_where() {
        assert_snapshot!(goto("
create table t(a int);
create function b(t) returns int as 'select 1' language sql;
select * from t where t.b$0 > 0;
"), @r"
          ╭▸ 
        3 │ create function b(t) returns int as 'select 1' language sql;
          │                 ─ 2. destination
        4 │ select * from t where t.b > 0;
          ╰╴                        ─ 1. source
        ");
    }

    #[test]
    fn goto_field_style_in_where_column_precedence() {
        assert_snapshot!(goto("
create table t(a int, b int);
create function b(t) returns int as 'select 1' language sql;
select * from t where t.b$0 > 0;
"), @r"
          ╭▸ 
        2 │ create table t(a int, b int);
          │                       ─ 2. destination
        3 │ create function b(t) returns int as 'select 1' language sql;
        4 │ select * from t where t.b > 0;
          ╰╴                        ─ 1. source
        ");
    }

    #[test]
    fn goto_function_call_style_table_arg_in_where() {
        assert_snapshot!(goto("
create table t(a int);
select * from t where a(t$0) > 2;
"), @r"
          ╭▸ 
        2 │ create table t(a int);
          │              ─ 2. destination
        3 │ select * from t where a(t) > 2;
          ╰╴                        ─ 1. source
        ");
    }

    #[test]
    fn goto_qualified_table_ref_in_where() {
        assert_snapshot!(goto("
create table t(a int);
create function b(t) returns int as 'select 1' language sql;
select * from t where t$0.b > 2;
"), @r"
          ╭▸ 
        2 │ create table t(a int);
          │              ─ 2. destination
        3 │ create function b(t) returns int as 'select 1' language sql;
        4 │ select * from t where t.b > 2;
          ╰╴                      ─ 1. source
        ");
    }

    #[test]
    fn goto_function_call_style_in_order_by() {
        assert_snapshot!(goto("
create table t(a int, b int);
create function b(t) returns int as 'select 1' language sql;
select * from t order by b$0(t);
"), @r"
          ╭▸ 
        3 │ create function b(t) returns int as 'select 1' language sql;
          │                 ─ 2. destination
        4 │ select * from t order by b(t);
          ╰╴                         ─ 1. source
        ");
    }

    #[test]
    fn goto_field_style_in_order_by() {
        assert_snapshot!(goto("
create table t(a int);
create function b(t) returns int as 'select 1' language sql;
select * from t order by t.b$0;
"), @r"
          ╭▸ 
        3 │ create function b(t) returns int as 'select 1' language sql;
          │                 ─ 2. destination
        4 │ select * from t order by t.b;
          ╰╴                           ─ 1. source
        ");
    }

    #[test]
    fn goto_function_call_style_in_group_by() {
        assert_snapshot!(goto("
create table t(a int, b int);
select * from t group by a$0(t);
"), @r"
          ╭▸ 
        2 │ create table t(a int, b int);
          │                ─ 2. destination
        3 │ select * from t group by a(t);
          ╰╴                         ─ 1. source
        ");
    }

    #[test]
    fn goto_field_style_in_group_by() {
        assert_snapshot!(goto("
create table t(a int);
create function b(t) returns int as 'select 1' language sql;
select * from t group by t.b$0;
"), @r"
          ╭▸ 
        3 │ create function b(t) returns int as 'select 1' language sql;
          │                 ─ 2. destination
        4 │ select * from t group by t.b;
          ╰╴                           ─ 1. source
        ");
    }

    #[test]
    fn goto_select_alias_order_by_column_name_conflict() {
        // If an ORDER BY expression is a simple name that matches both an
        // output column name and an input column name, ORDER BY will interpret
        // it as the output column name.
        assert_snapshot!(goto("
with t as (select 2 a)
select 1 a from t
order by a$0;
"), @"
          ╭▸ 
        3 │ select 1 a from t
          │          ─ 2. destination
        4 │ order by a;
          ╰╴         ─ 1. source
        ");
    }

    #[test]
    fn goto_select_alias_not_picked_window_order_by() {
        assert_snapshot!(goto("
with t as (select 4 a union select 2 a)
-- should go to the column def, not the alias
select 2 a, a, row_number() over (order by a$0) from t;
"), @"
          ╭▸ 
        2 │ with t as (select 4 a union select 2 a)
          │                     ─ 2. destination
        3 │ -- should go to the column def, not the alias
        4 │ select 2 a, a, row_number() over (order by a) from t;
          ╰╴                                           ─ 1. source
        ");
    }

    #[test]
    fn goto_select_alias_group_by_alias_func() {
        assert_snapshot!(goto("
with t as (select 'x'::text as name)
select lower(name) from t
group by lower$0;
"), @"
          ╭▸ 
        3 │ select lower(name) from t
          │        ───── 2. destination
        4 │ group by lower;
          ╰╴             ─ 1. source
        ");
    }

    #[test]
    fn goto_select_alias_order_by_alias_func() {
        assert_snapshot!(goto("
with t as (select 'x'::text as name)
select lower(name) from t
order by lower$0;
"), @"
          ╭▸ 
        3 │ select lower(name) from t
          │        ───── 2. destination
        4 │ order by lower;
          ╰╴             ─ 1. source
        ");
    }

    #[test]
    fn goto_select_alias_group_by_column_name_conflict() {
        // If a GROUP BY expression is a simple name that matches both output
        // column name and an input column name, GROUP BY will interpret it as
        // the input column name.
        assert_snapshot!(goto("
with t as (select 2 a)
select 1 a from t
group by a$0;
"), @"
          ╭▸ 
        2 │ with t as (select 2 a)
          │                     ─ 2. destination
        3 │ select 1 a from t
        4 │ group by a;
          ╰╴         ─ 1. source
        ");
    }

    #[test]
    fn goto_select_alias_in_group_by_with_cte() {
        assert_snapshot!(goto("
with t as (select 2 b)
select 1 a from t
group by a$0;
"), @"
          ╭▸ 
        3 │ select 1 a from t
          │          ─ 2. destination
        4 │ group by a;
          ╰╴         ─ 1. source
        ");
    }

    #[test]
    fn goto_select_alias_in_group_by_rollup() {
        assert_snapshot!(goto("
create table t (a int);
select a as x from t
group by rollup(x$0);
"), @"
          ╭▸ 
        3 │ select a as x from t
          │             ─ 2. destination
        4 │ group by rollup(x);
          ╰╴                ─ 1. source
        ");
    }

    #[test]
    fn goto_select_alias_in_group_by_cube() {
        assert_snapshot!(goto("
create table t (a int);
select a as x from t
group by cube(x$0);
"), @"
          ╭▸ 
        3 │ select a as x from t
          │             ─ 2. destination
        4 │ group by cube(x);
          ╰╴              ─ 1. source
        ");
    }

    #[test]
    fn goto_select_alias_in_group_by_grouping_sets() {
        assert_snapshot!(goto("
create table t (a int);
select a as x from t
group by grouping sets ((x$0));
"), @"
          ╭▸ 
        3 │ select a as x from t
          │             ─ 2. destination
        4 │ group by grouping sets ((x));
          ╰╴                         ─ 1. source
        ");
    }

    #[test]
    fn goto_select_alias_in_distinct_on() {
        assert_snapshot!(goto("
create table t (a int);
select distinct on (x$0) a as x from t;
"), @"
          ╭▸ 
        3 │ select distinct on (x) a as x from t;
          │                     ┬       ─ 2. destination
          │                     │
          ╰╴                    1. source
        ");
    }

    #[test]
    fn goto_select_alias_expr_in_order_by_not_found() {
        goto_not_found(
            "
with t as (select 2 b)
select 1 a from t
order by a$0 + 1
",
        );
    }

    #[test]
    fn goto_select_alias_expr_in_group_by_expr_not_found() {
        goto_not_found(
            "
with t as (select 2 b)
select 1 a from t
group by a$0 + 1
",
        );
    }

    #[test]
    fn goto_update_alias_hides_table_name() {
        goto_not_found(
            "
create table t(a int);
update t as u set a = t$0.a;
",
        );
    }

    #[test]
    fn goto_update_alias_hides_table_name_qualified_column() {
        goto_not_found(
            "
create table t(a int);
update t as u set a = t.a$0;
",
        );
    }

    #[test]
    fn goto_insert_alias_hides_table_name() {
        goto_not_found(
            "
create table t(a int);
insert into t as u values (1) returning t$0.a;
",
        );
    }

    #[test]
    fn goto_delete_alias_hides_table_name() {
        goto_not_found(
            "
create table t(a int);
delete from t as u where t$0.a = 1;
",
        );
    }

    #[test]
    fn goto_delete_alias_hides_table_name_qualified_column() {
        goto_not_found(
            "
create table t(a int);
delete from t as u where t.a$0 = 1;
",
        );
    }

    #[test]
    fn goto_merge_alias_hides_target_table_name() {
        goto_not_found(
            "
create table t(a int);
create table s(a int);
merge into t as u
  using s on t$0.a = s.a
  when matched then do nothing;
",
        );
    }

    #[test]
    fn goto_merge_using_alias_hides_table_name() {
        goto_not_found(
            "
create table t(a int);
create table s(a int);
merge into t
  using s as u on s$0.a = t.a
  when matched then do nothing;
",
        );
    }

    #[test]
    fn goto_merge_using_subquery_source_column() {
        assert_snapshot!(goto("
create table t(id int, val int);
merge into t
  using (select 1 as id, 2 as val) as s
    on t.id = s.id$0
  when not matched then
    insert (id, val) values (s.id, s.val);
"), @"
          ╭▸ 
        4 │   using (select 1 as id, 2 as val) as s
          │                      ── 2. destination
        5 │     on t.id = s.id
          ╰╴                 ─ 1. source
        ");
    }

    #[test]
    fn goto_select_alias_in_order_by_with_cte() {
        assert_snapshot!(goto("
with t as (select 2 b)
select 1 a from t
order by a$0;
"), @"
          ╭▸ 
        3 │ select 1 a from t
          │          ─ 2. destination
        4 │ order by a;
          ╰╴         ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_table() {
        assert_snapshot!(goto("
with x as (select 1 as a)
select a from x$0;
"), @r"
          ╭▸ 
        2 │ with x as (select 1 as a)
          │      ─ 2. destination
        3 │ select a from x;
          ╰╴              ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_shadows_table_in_from() {
        assert_snapshot!(goto("
create table x(a int);
with x as (select 1 a)
select a from x$0;
"), @"
          ╭▸ 
        3 │ with x as (select 1 a)
          │      ─ 2. destination
        4 │ select a from x;
          ╰╴              ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_shadows_view_column() {
        assert_snapshot!(goto("
create view x as select 1 a;
with x as (select 2 a)
select a$0 from x;
"), @"
          ╭▸ 
        3 │ with x as (select 2 a)
          │                     ─ 2. destination
        4 │ select a from x;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_column() {
        assert_snapshot!(goto("
with x as (select 1 as a)
select a$0 from x;
"), @r"
          ╭▸ 
        2 │ with x as (select 1 as a)
          │                        ─ 2. destination
        3 │ select a from x;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_qualified_column_prefers_cte_over_table() {
        assert_snapshot!(goto("
create table u(id int, b int);
with u as (select 1 id, 2 b)
select u.id$0 from u;
"), @r"
          ╭▸ 
        3 │ with u as (select 1 id, 2 b)
          │                     ── 2. destination
        4 │ select u.id from u;
          ╰╴          ─ 1. source
        ");
    }

    #[test]
    fn goto_qualified_table_ref_prefers_schema_qualified_from_item_over_cte() {
        assert_snapshot!(goto("
create schema s;
create table s.t(a int);
with t as (select 1 a)
select t$0.a from s.t;
"), @r"
          ╭▸ 
        3 │ create table s.t(a int);
          │                ─ 2. destination
        4 │ with t as (select 1 a)
        5 │ select t.a from s.t;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_subquery_qualified_column() {
        assert_snapshot!(goto("
select t.a$0 from (select 1 a) t;
"), @r"
          ╭▸ 
        2 │ select t.a from (select 1 a) t;
          ╰╴         ─ 1. source      ─ 2. destination
        ");
    }

    #[test]
    fn goto_cte_multiple_columns() {
        assert_snapshot!(goto("
with x as (select 1 as a, 2 as b)
select b$0 from x;
"), @r"
          ╭▸ 
        2 │ with x as (select 1 as a, 2 as b)
          │                                ─ 2. destination
        3 │ select b from x;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_nested() {
        assert_snapshot!(goto("
with x as (select 1 as a),
     y as (select a from x)
select a$0 from y;
"), @r"
          ╭▸ 
        3 │      y as (select a from x)
          │                   ─ 2. destination
        4 │ select a from y;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_unnamed_column() {
        assert_snapshot!(goto(r#"
with x as (select 1)
select "?column?"$0 from x;
"#), @r#"
          ╭▸ 
        2 │ with x as (select 1)
          │                   ─ 2. destination
        3 │ select "?column?" from x;
          ╰╴                ─ 1. source
        "#);
    }

    #[test]
    fn goto_cte_star_expansion() {
        assert_snapshot!(goto("
with t as (select 1 a),
     y as (select * from t)
select a$0 from y;
"), @r"
          ╭▸ 
        2 │ with t as (select 1 a),
          │                     ─ 2. destination
        3 │      y as (select * from t)
        4 │ select a from y;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_qualified_star_join_column() {
        assert_snapshot!(goto("
create table u(id int, b int);
create table t(id int, a int);

with k as (
    select u.* from t join u on a = b
)
select b$0 from k;
"), @r"
          ╭▸ 
        2 │ create table u(id int, b int);
          │                        ─ 2. destination
          ‡
        8 │ select b from k;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_qualified_star_join_column_with_partial_column_list() {
        assert_snapshot!(goto("
with
  u as (
    select 1 id, 2 b
  ),
  t as (
    select 1 id, 2 a
  ),
  k(x) as (
    select u.* from t join u on a = b
  )
select b$0 from k;
"), @r"
          ╭▸ 
        4 │     select 1 id, 2 b
          │                    ─ 2. destination
          ‡
       12 │ select b from k;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_reference_inside_cte() {
        assert_snapshot!(goto("
with t as (select 1 a),
     y as (select a$0 from t)
select a from y;
"), @r"
          ╭▸ 
        2 │ with t as (select 1 a),
          │                     ─ 2. destination
        3 │      y as (select a from t)
          ╰╴                  ─ 1. source
        ");
    }

    #[test]
    fn goto_recursive_cte_reference_inside_cte() {
        assert_snapshot!(goto("
with recursive nums as (
  select 1 as n
  union all
  select n + 1 from nums$0 where n < 5
)
select * from nums;
"), @r"
          ╭▸ 
        2 │ with recursive nums as (
          │                ──── 2. destination
          ‡
        5 │   select n + 1 from nums where n < 5
          ╰╴                       ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_search_clause_set_column() {
        assert_snapshot!(goto("
with recursive r as (select 1 as id)
  search depth first by id set ord
select ord$0 from r;
"), @"
          ╭▸ 
        3 │   search depth first by id set ord
          │                                ─── 2. destination
        4 │ select ord from r;
          ╰╴         ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_search_clause_set_column_qualified() {
        assert_snapshot!(goto("
with recursive r as (select 1 as id)
  search depth first by id set ord
select r.ord$0 from r;
"), @"
          ╭▸ 
        3 │   search depth first by id set ord
          │                                ─── 2. destination
        4 │ select r.ord from r;
          ╰╴           ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_cycle_clause_set_column() {
        assert_snapshot!(goto("
with recursive r as (select 1 as id)
  cycle id set is_cycle using path
select is_cycle$0 from r;
"), @"
          ╭▸ 
        3 │   cycle id set is_cycle using path
          │                ──────── 2. destination
        4 │ select is_cycle from r;
          ╰╴              ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_cycle_clause_path_column() {
        assert_snapshot!(goto("
with recursive r as (select 1 as id)
  cycle id set is_cycle using path
select path$0 from r;
"), @"
          ╭▸ 
        3 │   cycle id set is_cycle using path
          │                               ──── 2. destination
        4 │ select path from r;
          ╰╴          ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_with_column_list() {
        assert_snapshot!(goto("
with t(a) as (select 1)
select a$0 from t;
"), @r"
          ╭▸ 
        2 │ with t(a) as (select 1)
          │        ─ 2. destination
        3 │ select a from t;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_with_partial_column_list() {
        assert_snapshot!(goto("
with t(x) as (select 1 as a, 2 as b)
select b$0 from t;
"), @r"
          ╭▸ 
        2 │ with t(x) as (select 1 as a, 2 as b)
          │                                   ─ 2. destination
        3 │ select b from t;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_with_partial_column_list_renamed() {
        assert_snapshot!(goto("
with t(x) as (select 1 as a, 2 as b)
select x$0 from t;
"), @r"
          ╭▸ 
        2 │ with t(x) as (select 1 as a, 2 as b)
          │        ─ 2. destination
        3 │ select x from t;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_insert_returning_star_column() {
        assert_snapshot!(goto("
create table t(a int, b int);
with inserted as (
  insert into t values (1, 2), (3, 4)
  returning *
)
select a$0 from inserted;
"), @r"
          ╭▸ 
        2 │ create table t(a int, b int);
          │                ─ 2. destination
          ‡
        7 │ select a from inserted;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_returning_qualified_star_column_gap() {
        assert_snapshot!(goto("
create table t(a int, b int);
with changed as (
  insert into t values (1, 2)
  returning new.*
)
select a$0 from changed;
"), @"
          ╭▸ 
        2 │ create table t(a int, b int);
          │                ─ 2. destination
          ‡
        7 │ select a from changed;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_delete_returning_star_column() {
        assert_snapshot!(goto("
create table t(a int, b int);
with deleted as (
  delete from t
  returning *
)
select a$0 from deleted;
"), @r"
          ╭▸ 
        2 │ create table t(a int, b int);
          │                ─ 2. destination
          ‡
        7 │ select a from deleted;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_update_returning_star_column() {
        assert_snapshot!(goto("
create table t(a int, b int);
with updated as (
  update t set a = 42
  returning *
)
select a$0 from updated;
"), @r"
          ╭▸ 
        2 │ create table t(a int, b int);
          │                ─ 2. destination
          ‡
        7 │ select a from updated;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_update_returning_column_list_overwrites_column() {
        goto_not_found(
            "
create table t(a int, b int);
with updated(c) as (
  update t set a = 10
  returning a
)
select a$0 from updated;
",
        );
    }

    #[test]
    fn goto_cte_column_list_overwrites_column() {
        goto_not_found(
            "
with t(x) as (select 1 as a)
select a$0 from t;
",
        );
    }

    #[test]
    fn goto_cte_shadows_table() {
        assert_snapshot!(goto("
create table t(a int);
with t as (select a$0 from t)
select a from t;
"), @r"
          ╭▸ 
        2 │ create table t(a int);
          │                ─ 2. destination
        3 │ with t as (select a from t)
          ╰╴                  ─ 1. source
        ");
    }

    #[test]
    fn goto_subquery_column() {
        assert_snapshot!(goto("
select a$0 from (select 1 a);
"), @r"
          ╭▸ 
        2 │ select a from (select 1 a);
          ╰╴       ─ 1. source      ─ 2. destination
        ");
    }

    #[test]
    fn goto_subquery_star_partial_alias_masks_original_column_gap() {
        goto_not_found(
            "
create table t(a int, b int);
select a$0 from (select * from t) u(x);
",
        );
    }

    #[test]
    fn goto_subquery_column_with_as() {
        assert_snapshot!(goto("
select a$0 from (select 1 as a);
"), @r"
          ╭▸ 
        2 │ select a from (select 1 as a);
          ╰╴       ─ 1. source         ─ 2. destination
        ");
    }

    #[test]
    fn goto_subquery_compound_select_column() {
        assert_snapshot!(goto("
select c$0 from (select 1 c union select 2 c);
"), @r"
          ╭▸ 
        2 │ select c from (select 1 c union select 2 c);
          ╰╴       ─ 1. source      ─ 2. destination
        ");
    }

    #[test]
    fn goto_subquery_compound_table_query_column() {
        assert_snapshot!(goto("
create table t(a int);
select a$0 from (table t union table t) u;
"), @"
          ╭▸ 
        2 │ create table t(a int);
          │                ─ 2. destination
        3 │ select a from (table t union table t) u;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_subquery_table_query_whole_row_alias() {
        assert_snapshot!(goto("
create table t(a int);
select t$0 from (table t) t;
"), @"
          ╭▸ 
        3 │ select t from (table t) t;
          ╰╴       ─ 1. source      ─ 2. destination
        ");
    }

    #[test]
    fn goto_subquery_compound_table_query_whole_row_alias() {
        assert_snapshot!(goto("
create table t(a int);
select t$0 from (table t union table t) t;
"), @"
          ╭▸ 
        3 │ select t from (table t union table t) t;
          ╰╴       ─ 1. source                    ─ 2. destination
        ");
    }

    #[test]
    fn goto_subquery_compound_values_query_column() {
        assert_snapshot!(goto("
select column2$0 from (values (1, 2) union values (3, 4)) u;
"), @"
          ╭▸ 
        2 │ select column2 from (values (1, 2) union values (3, 4)) u;
          ╰╴             ─ 1. source        ─ 2. destination
        ");
    }

    #[test]
    fn goto_cte_compound_table_query_column() {
        assert_snapshot!(goto("
create table t(a int);
with u as (table t union table t)
select a$0 from u;
"), @"
          ╭▸ 
        2 │ create table t(a int);
          │                ─ 2. destination
        3 │ with u as (table t union table t)
        4 │ select a from u;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_compound_values_query_column() {
        assert_snapshot!(goto("
with u as (values (1, 2) union values (3, 4))
select column2$0 from u;
"), @"
          ╭▸ 
        2 │ with u as (values (1, 2) union values (3, 4))
          │                       ─ 2. destination
        3 │ select column2 from u;
          ╰╴             ─ 1. source
        ");
    }

    #[test]
    fn goto_subquery_compound_select_column_order_by() {
        assert_snapshot!(goto("
with t as (select 1 a)
select 2 a from t union select 1 order by a$0;
"), @"
          ╭▸ 
        3 │ select 2 a from t union select 1 order by a;
          ╰╴         ─ 2. destination                 ─ 1. source
        ");
    }

    #[test]
    fn goto_compound_table_query_column_order_by() {
        assert_snapshot!(goto("
create table t(a int);
table t union table t order by a$0;
"), @"
          ╭▸ 
        2 │ create table t(a int);
          │                ─ 2. destination
        3 │ table t union table t order by a;
          ╰╴                               ─ 1. source
        ");
    }

    #[test]
    fn goto_subquery_compound_select_column_with_nested_parens() {
        assert_snapshot!(goto("
with t as (
  select 1 as c
)
select c$0 from ((select * from t) union all (select * from t));
"), @r"
          ╭▸ 
        3 │   select 1 as c
          │               ─ 2. destination
        4 │ )
        5 │ select c from ((select * from t) union all (select * from t));
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_subquery_column_multiple_columns() {
        assert_snapshot!(goto("
select b$0 from (select 1 a, 2 b);
"), @r"
          ╭▸ 
        2 │ select b from (select 1 a, 2 b);
          ╰╴       ─ 1. source           ─ 2. destination
        ");
    }

    #[test]
    fn goto_subquery_column_nested_parens() {
        assert_snapshot!(goto("
select a$0 from ((select 1 a));
"), @r"
          ╭▸ 
        2 │ select a from ((select 1 a));
          ╰╴       ─ 1. source       ─ 2. destination
        ");
    }

    #[test]
    fn goto_subquery_column_star_table() {
        assert_snapshot!(goto("
create table foo.t(a int);
select a$0 from (select * from foo.t);
"), @r"
          ╭▸ 
        2 │ create table foo.t(a int);
          │                    ─ 2. destination
        3 │ select a from (select * from foo.t);
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_subquery_column_qualified_star_join() {
        assert_snapshot!(goto("
create table t(a int);
create table u(b int);
select b$0 from (select u.* from t join u on a = b);
"), @r"
          ╭▸ 
        3 │ create table u(b int);
          │                ─ 2. destination
        4 │ select b from (select u.* from t join u on a = b);
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_subquery_column_qualified_star_join_not_found() {
        goto_not_found(
            "
create table t(a int);
create table u(b int);
select a$0 from (select u.* from t join u on a = b);
",
        );
    }

    #[test]
    fn goto_subquery_column_alias_list() {
        assert_snapshot!(goto("
select c$0, t.c from (select 1) t(c);
"), @r"
          ╭▸ 
        2 │ select c, t.c from (select 1) t(c);
          ╰╴       ─ 1. source              ─ 2. destination
        ");
    }

    #[test]
    fn goto_subquery_column_alias_list_qualified() {
        assert_snapshot!(goto("
select t.c$0 from (select 1) t(c);
"), @r"
          ╭▸ 
        2 │ select t.c from (select 1) t(c);
          ╰╴         ─ 1. source         ─ 2. destination
        ");
    }

    #[test]
    fn goto_subquery_column_alias_list_multiple() {
        assert_snapshot!(goto("
select b$0 from (select 1, 2) t(a, b);
"), @r"
          ╭▸ 
        2 │ select b from (select 1, 2) t(a, b);
          ╰╴       ─ 1. source               ─ 2. destination
        ");
    }

    #[test]
    fn goto_cte_column_alias_list() {
        assert_snapshot!(goto("
with x as (select 1)
select c$0 from x t(c);
"), @r"
          ╭▸ 
        3 │ select c from x t(c);
          │        ┬          ─ 2. destination
          │        │
          ╰╴       1. source
        ");
    }

    #[test]
    fn goto_cte_column_alias_list_qualified() {
        assert_snapshot!(goto("
with x as (select 1)
select t.c$0 from x t(c);
"), @r"
          ╭▸ 
        3 │ select t.c from x t(c);
          │          ┬          ─ 2. destination
          │          │
          ╰╴         1. source
        ");
    }

    #[test]
    fn goto_cte_column_alias_list_multiple() {
        assert_snapshot!(goto("
with x as (select 1, 2)
select b$0 from x t(a, b);
"), @r"
          ╭▸ 
        3 │ select b from x t(a, b);
          ╰╴       ─ 1. source   ─ 2. destination
        ");
    }

    #[test]
    fn goto_values_column_alias_list() {
        assert_snapshot!(goto("
select c$0 from (values (1)) t(c);
"), @r"
          ╭▸ 
        2 │ select c from (values (1)) t(c);
          ╰╴       ─ 1. source           ─ 2. destination
        ");
    }

    #[test]
    fn goto_values_column_alias_list_qualified() {
        assert_snapshot!(goto("
select t.c$0 from (values (1)) t(c);
"), @r"
          ╭▸ 
        2 │ select t.c from (values (1)) t(c);
          ╰╴         ─ 1. source           ─ 2. destination
        ");
    }

    #[test]
    fn goto_values_column_alias_list_multiple() {
        assert_snapshot!(goto("
select b$0 from (values (1, 2)) t(a, b);
"), @r"
          ╭▸ 
        2 │ select b from (values (1, 2)) t(a, b);
          ╰╴       ─ 1. source                 ─ 2. destination
        ");
    }

    #[test]
    fn goto_values_column_alias_list_nested_parens() {
        assert_snapshot!(goto("
select n$0
from ((values (1), (2))) u(n);
"), @r"
          ╭▸ 
        2 │ select n
          │        ─ 1. source
        3 │ from ((values (1), (2))) u(n);
          ╰╴                           ─ 2. destination
        ");
    }

    #[test]
    fn goto_table_expr_column() {
        assert_snapshot!(goto("
create table t(a int, b int);
select a$0 from (table t);
"), @r"
          ╭▸ 
        2 │ create table t(a int, b int);
          │                ─ 2. destination
        3 │ select a from (table t);
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_table_expr_column_with_cte() {
        assert_snapshot!(goto("
with x as (select 1 a)
select a$0 from (table x);
"), @r"
          ╭▸ 
        2 │ with x as (select 1 a)
          │                     ─ 2. destination
        3 │ select a from (table x);
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_table_expr_cte_table() {
        assert_snapshot!(goto("
with t as (select 1 a, 2 b)
select * from (table t$0);
"), @r"
          ╭▸ 
        2 │ with t as (select 1 a, 2 b)
          │      ─ 2. destination
        3 │ select * from (table t);
          ╰╴                     ─ 1. source
        ");
    }

    #[test]
    fn goto_table_expr_partial_column_alias_list() {
        assert_snapshot!(goto("
with t as (select 1 a, 2 b)
select c, b$0 from (table t) u(c);
"), @r"
          ╭▸ 
        2 │ with t as (select 1 a, 2 b)
          │                          ─ 2. destination
        3 │ select c, b from (table t) u(c);
          ╰╴          ─ 1. source
        ");
    }

    #[test]
    fn goto_subquery_partial_column_alias_list() {
        assert_snapshot!(goto("
select x, b$0 from (select 1 a, 2 b) t(x);
"), @r"
          ╭▸ 
        2 │ select x, b from (select 1 a, 2 b) t(x);
          ╰╴          ─ 1. source           ─ 2. destination
        ");
    }

    #[test]
    fn goto_subquery_alias_with_column_list_table_ref() {
        assert_snapshot!(goto("
with t as (select 1 a, 2 b)
select z$0 from (select * from t) as z(x, y);
"), @"
          ╭▸ 
        3 │ select z from (select * from t) as z(x, y);
          ╰╴       ─ 1. source                 ─ 2. destination
        ");
    }

    #[test]
    fn goto_subquery_alias_with_column_list_table_ref_shadows_column() {
        assert_snapshot!(goto("
with t as (select 1 a, 2 b)
select z$0 from (select a as z, b from t) as z(x, y);
"), @"
          ╭▸ 
        3 │ select z from (select a as z, b from t) as z(x, y);
          ╰╴       ─ 1. source                         ─ 2. destination
        ");
    }

    #[test]
    fn goto_subquery_nested_paren_alias_with_column_list_table_ref() {
        assert_snapshot!(goto("
with t as (select 1 a, 2 b, 3 c)
select z$0 from ((select * from t)) as z(x, y);
"), @"
          ╭▸ 
        3 │ select z from ((select * from t)) as z(x, y);
          ╰╴       ─ 1. source                   ─ 2. destination
        ");
    }

    #[test]
    fn goto_table_expr_values_cte_partial_alias() {
        assert_snapshot!(goto("
with t as (values (1, 2), (3, 4))
select column2$0 from (table t) u(a);
"), @r"
          ╭▸ 
        2 │ with t as (values (1, 2), (3, 4))
          │                       ─ 2. destination
        3 │ select column2 from (table t) u(a);
          ╰╴             ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_with_table_expr() {
        assert_snapshot!(goto("
create table t(a int, b int);
with u as (table t)
select a$0 from u;
"), @r"
          ╭▸ 
        2 │ create table t(a int, b int);
          │                ─ 2. destination
        3 │ with u as (table t)
        4 │ select a from u;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_with_table_expr_nested() {
        assert_snapshot!(goto("
with t as (select 1 a, 2 b),
     u as (table t)
select b$0 from u;
"), @r"
          ╭▸ 
        2 │ with t as (select 1 a, 2 b),
          │                          ─ 2. destination
        3 │      u as (table t)
        4 │ select b from u;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_paren_table_query_column() {
        assert_snapshot!(goto("
create table t(a int);
with u as ((table t))
select a$0 from u;
"), @"
          ╭▸ 
        2 │ create table t(a int);
          │                ─ 2. destination
        3 │ with u as ((table t))
        4 │ select a from u;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_paren_values_query_column() {
        assert_snapshot!(goto("
with u as ((values (1, 2)))
select column2$0 from u;
"), @"
          ╭▸ 
        2 │ with u as ((values (1, 2)))
          │                        ─ 2. destination
        3 │ select column2 from u;
          ╰╴             ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_table_query_column_count_gap() {
        assert_snapshot!(goto("
create table t(a int, b int);
with u as (table t)
select b$0 from (select * from u) x(a);
"), @"
          ╭▸ 
        2 │ create table t(a int, b int);
          │                       ─ 2. destination
        3 │ with u as (table t)
        4 │ select b from (select * from u) x(a);
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_insert_table() {
        assert_snapshot!(goto("
create table users(id int, email text);
insert into users$0(id, email) values (1, 'test@example.com');
"), @r"
          ╭▸ 
        2 │ create table users(id int, email text);
          │              ───── 2. destination
        3 │ insert into users(id, email) values (1, 'test@example.com');
          ╰╴                ─ 1. source
        ");
    }

    #[test]
    fn goto_insert_table_with_schema() {
        assert_snapshot!(goto("
create table public.users(id int, email text);
insert into public.users$0(id, email) values (1, 'test@example.com');
"), @r"
          ╭▸ 
        2 │ create table public.users(id int, email text);
          │                     ───── 2. destination
        3 │ insert into public.users(id, email) values (1, 'test@example.com');
          ╰╴                       ─ 1. source
        ");
    }

    #[test]
    fn goto_insert_view() {
        assert_snapshot!(goto("
create table users as select 1 id, 'joe' name, 'joe@example.com' email, 'active' status;
create view active_users as
  select id, name, email
  from users
  where status = 'active';
insert into active_users$0 (name, email)
values ('Alice', 'alice@example.com');
"), @"
          ╭▸ 
        3 │ create view active_users as
          │             ──────────── 2. destination
          ‡
        7 │ insert into active_users (name, email)
          ╰╴                       ─ 1. source
        ");
    }

    #[test]
    fn goto_insert_column() {
        assert_snapshot!(goto("
create table users(id int, email text);
insert into users(id$0, email) values (1, 'test@example.com');
"), @r"
          ╭▸ 
        2 │ create table users(id int, email text);
          │                    ── 2. destination
        3 │ insert into users(id, email) values (1, 'test@example.com');
          ╰╴                   ─ 1. source
        ");
    }

    #[test]
    fn goto_insert_column_second() {
        assert_snapshot!(goto("
create table users(id int, email text);
insert into users(id, email$0) values (1, 'test@example.com');
"), @r"
          ╭▸ 
        2 │ create table users(id int, email text);
          │                            ───── 2. destination
        3 │ insert into users(id, email) values (1, 'test@example.com');
          ╰╴                          ─ 1. source
        ");
    }

    #[test]
    fn goto_insert_column_with_schema() {
        assert_snapshot!(goto("
create table public.users(id int, email text);
insert into public.users(email$0) values ('test@example.com');
"), @r"
          ╭▸ 
        2 │ create table public.users(id int, email text);
          │                                   ───── 2. destination
        3 │ insert into public.users(email) values ('test@example.com');
          ╰╴                             ─ 1. source
        ");
    }

    #[test]
    fn goto_insert_table_with_search_path() {
        assert_snapshot!(goto("
set search_path to foo;
create table foo.users(id int, email text);
insert into users$0(id, email) values (1, 'test@example.com');
"), @r"
          ╭▸ 
        3 │ create table foo.users(id int, email text);
          │                  ───── 2. destination
        4 │ insert into users(id, email) values (1, 'test@example.com');
          ╰╴                ─ 1. source
        ");
    }

    #[test]
    fn goto_insert_column_with_search_path() {
        assert_snapshot!(goto("
set search_path to myschema;
create table myschema.users(id int, email text, name text);
insert into users(email$0, name) values ('test@example.com', 'Test');
"), @r"
          ╭▸ 
        3 │ create table myschema.users(id int, email text, name text);
          │                                     ───── 2. destination
        4 │ insert into users(email, name) values ('test@example.com', 'Test');
          ╰╴                      ─ 1. source
        ");
    }

    #[test]
    fn goto_delete_table() {
        assert_snapshot!(goto("
create table users(id int, email text);
delete from users$0 where id = 1;
"), @r"
          ╭▸ 
        2 │ create table users(id int, email text);
          │              ───── 2. destination
        3 │ delete from users where id = 1;
          ╰╴                ─ 1. source
        ");
    }

    #[test]
    fn goto_delete_table_with_schema() {
        assert_snapshot!(goto("
create table public.users(id int, email text);
delete from public.users$0 where id = 1;
"), @r"
          ╭▸ 
        2 │ create table public.users(id int, email text);
          │                     ───── 2. destination
        3 │ delete from public.users where id = 1;
          ╰╴                       ─ 1. source
        ");
    }

    #[test]
    fn goto_delete_table_with_search_path() {
        assert_snapshot!(goto("
set search_path to foo;
create table foo.users(id int, email text);
delete from users$0 where id = 1;
"), @r"
          ╭▸ 
        3 │ create table foo.users(id int, email text);
          │                  ───── 2. destination
        4 │ delete from users where id = 1;
          ╰╴                ─ 1. source
        ");
    }

    #[test]
    fn goto_delete_temp_table() {
        assert_snapshot!(goto("
create temp table users(id int, email text);
delete from users$0 where id = 1;
"), @r"
          ╭▸ 
        2 │ create temp table users(id int, email text);
          │                   ───── 2. destination
        3 │ delete from users where id = 1;
          ╰╴                ─ 1. source
        ");
    }

    #[test]
    fn goto_delete_where_column() {
        assert_snapshot!(goto("
create table users(id int, email text);
delete from users where id$0 = 1;
"), @r"
          ╭▸ 
        2 │ create table users(id int, email text);
          │                    ── 2. destination
        3 │ delete from users where id = 1;
          ╰╴                         ─ 1. source
        ");
    }

    #[test]
    fn goto_delete_where_column_second() {
        assert_snapshot!(goto("
create table users(id int, email text);
delete from users where email$0 = 'test@example.com';
"), @r"
          ╭▸ 
        2 │ create table users(id int, email text);
          │                            ───── 2. destination
        3 │ delete from users where email = 'test@example.com';
          ╰╴                            ─ 1. source
        ");
    }

    #[test]
    fn goto_delete_where_column_with_schema() {
        assert_snapshot!(goto("
create table public.users(id int, email text, name text);
delete from public.users where name$0 = 'Test';
"), @r"
          ╭▸ 
        2 │ create table public.users(id int, email text, name text);
          │                                               ──── 2. destination
        3 │ delete from public.users where name = 'Test';
          ╰╴                                  ─ 1. source
        ");
    }

    #[test]
    fn goto_delete_where_column_with_search_path() {
        assert_snapshot!(goto("
set search_path to myschema;
create table myschema.users(id int, email text, active boolean);
delete from users where active$0 = true;
"), @r"
          ╭▸ 
        3 │ create table myschema.users(id int, email text, active boolean);
          │                                                 ────── 2. destination
        4 │ delete from users where active = true;
          ╰╴                             ─ 1. source
        ");
    }

    #[test]
    fn goto_delete_where_multiple_columns() {
        assert_snapshot!(goto("
create table users(id int, email text, active boolean);
delete from users where id$0 = 1 and active = true;
"), @r"
          ╭▸ 
        2 │ create table users(id int, email text, active boolean);
          │                    ── 2. destination
        3 │ delete from users where id = 1 and active = true;
          ╰╴                         ─ 1. source
        ");
    }

    #[test]
    fn goto_delete_using_table() {
        assert_snapshot!(goto("
create table t(id int, f_id int);
create table f(id int, name text);
delete from t using f$0 where f_id = f.id and f.name = 'foo';
"), @r"
          ╭▸ 
        3 │ create table f(id int, name text);
          │              ─ 2. destination
        4 │ delete from t using f where f_id = f.id and f.name = 'foo';
          ╰╴                    ─ 1. source
        ");
    }

    #[test]
    fn goto_delete_using_table_with_schema() {
        assert_snapshot!(goto("
create table t(id int, f_id int);
create table public.f(id int, name text);
delete from t using public.f$0 where f_id = f.id;
"), @r"
          ╭▸ 
        3 │ create table public.f(id int, name text);
          │                     ─ 2. destination
        4 │ delete from t using public.f where f_id = f.id;
          ╰╴                           ─ 1. source
        ");
    }

    #[test]
    fn goto_delete_using_column_in_where() {
        assert_snapshot!(goto("
create table t(id int, f_id int);
create table f(id int, name text);
delete from t using f where f_id = f.id$0 and f.name = 'foo';
"), @"
          ╭▸ 
        3 │ create table f(id int, name text);
          │                ── 2. destination
        4 │ delete from t using f where f_id = f.id and f.name = 'foo';
          ╰╴                                      ─ 1. source
        ");
    }

    #[test]
    fn goto_delete_using_source_alias_qualifier() {
        assert_snapshot!(goto("
create table target(id int);
create table src(y int);
delete from target using src s where s$0.y = target.id;
"), @"
          ╭▸ 
        4 │ delete from target using src s where s.y = target.id;
          │                              ┬       ─ 1. source
          │                              │
          ╰╴                             2. destination
        ");
    }

    #[test]
    fn goto_delete_using_source_alias_column() {
        assert_snapshot!(goto("
create table target(id int);
create table src(y int);
delete from target using src s where s.y$0 = target.id;
"), @"
          ╭▸ 
        3 │ create table src(y int);
          │                  ─ 2. destination
        4 │ delete from target using src s where s.y = target.id;
          ╰╴                                       ─ 1. source
        ");
    }

    #[test]
    fn goto_select_from_table() {
        assert_snapshot!(goto("
create table users(id int, email text);
select * from users$0;
"), @r"
          ╭▸ 
        2 │ create table users(id int, email text);
          │              ───── 2. destination
        3 │ select * from users;
          ╰╴                  ─ 1. source
        ");
    }

    #[test]
    fn goto_select_from_table_with_schema() {
        assert_snapshot!(goto("
create table public.users(id int, email text);
select * from public.users$0;
"), @r"
          ╭▸ 
        2 │ create table public.users(id int, email text);
          │                     ───── 2. destination
        3 │ select * from public.users;
          ╰╴                         ─ 1. source
        ");
    }

    #[test]
    fn goto_select_from_table_with_search_path() {
        assert_snapshot!(goto("
set search_path to foo;
create table foo.users(id int, email text);
select * from users$0;
"), @r"
          ╭▸ 
        3 │ create table foo.users(id int, email text);
          │                  ───── 2. destination
        4 │ select * from users;
          ╰╴                  ─ 1. source
        ");
    }

    #[test]
    fn goto_select_from_temp_table() {
        assert_snapshot!(goto("
create temp table users(id int, email text);
select * from users$0;
"), @r"
          ╭▸ 
        2 │ create temp table users(id int, email text);
          │                   ───── 2. destination
        3 │ select * from users;
          ╰╴                  ─ 1. source
        ");
    }

    #[test]
    fn goto_select_from_table_defined_after() {
        assert_snapshot!(goto("
select * from users$0;
create table users(id int, email text);
"), @r"
          ╭▸ 
        2 │ select * from users;
          │                   ─ 1. source
        3 │ create table users(id int, email text);
          ╰╴             ───── 2. destination
        ");
    }

    #[test]
    fn goto_select_column() {
        assert_snapshot!(goto("
create table users(id int, email text);
select id$0 from users;
"), @r"
          ╭▸ 
        2 │ create table users(id int, email text);
          │                    ── 2. destination
        3 │ select id from users;
          ╰╴        ─ 1. source
        ");
    }

    #[test]
    fn goto_select_column_second() {
        assert_snapshot!(goto("
create table users(id int, email text);
select id, email$0 from users;
"), @r"
          ╭▸ 
        2 │ create table users(id int, email text);
          │                            ───── 2. destination
        3 │ select id, email from users;
          ╰╴               ─ 1. source
        ");
    }

    #[test]
    fn goto_select_column_with_schema() {
        assert_snapshot!(goto("
create table public.users(id int, email text);
select email$0 from public.users;
"), @r"
          ╭▸ 
        2 │ create table public.users(id int, email text);
          │                                   ───── 2. destination
        3 │ select email from public.users;
          ╰╴           ─ 1. source
        ");
    }

    #[test]
    fn goto_select_column_with_search_path() {
        assert_snapshot!(goto("
set search_path to foo;
create table foo.users(id int, email text);
select id$0 from users;
"), @r"
          ╭▸ 
        3 │ create table foo.users(id int, email text);
          │                        ── 2. destination
        4 │ select id from users;
          ╰╴        ─ 1. source
        ");
    }

    #[test]
    fn goto_select_table_as_column() {
        assert_snapshot!(goto("
create table t(x bigint, y bigint);
select t$0 from t;
"), @r"
          ╭▸ 
        2 │ create table t(x bigint, y bigint);
          │              ─ 2. destination
        3 │ select t from t;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_select_table_star_expansion() {
        assert_snapshot!(goto("
create table t(id int, a int);
select t$0.* from t;
"), @r"
          ╭▸ 
        2 │ create table t(id int, a int);
          │              ─ 2. destination
        3 │ select t.* from t;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_select_table_as_column_with_schema() {
        assert_snapshot!(goto("
create table public.t(x bigint, y bigint);
select t$0 from public.t;
"), @r"
          ╭▸ 
        2 │ create table public.t(x bigint, y bigint);
          │                     ─ 2. destination
        3 │ select t from public.t;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_select_table_as_column_with_search_path() {
        assert_snapshot!(goto("
set search_path to foo;
create table foo.users(id int, email text);
select users$0 from users;
"), @r"
          ╭▸ 
        3 │ create table foo.users(id int, email text);
          │                  ───── 2. destination
        4 │ select users from users;
          ╰╴           ─ 1. source
        ");
    }

    #[test]
    fn goto_select_column_with_same_name_as_table() {
        assert_snapshot!(goto("
create table t(t int);
select t$0 from t;
"), @r"
          ╭▸ 
        2 │ create table t(t int);
          │                ─ 2. destination
        3 │ select t from t;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_select_view_name_from_view() {
        assert_snapshot!(goto("
create view boop as select 1 a;
select boop$0 from boop;
"), @"
          ╭▸ 
        2 │ create view boop as select 1 a;
          │             ──── 2. destination
        3 │ select boop from boop;
          ╰╴          ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_schema() {
        assert_snapshot!(goto("
create schema foo;
drop schema foo$0;
"), @r"
          ╭▸ 
        2 │ create schema foo;
          │               ─── 2. destination
        3 │ drop schema foo;
          ╰╴              ─ 1. source
        ");
    }

    #[test]
    fn goto_create_schema_authorization() {
        assert_snapshot!(goto("
create schema authorization foo$0;
"), @r"
          ╭▸ 
        2 │ create schema authorization foo;
          │                             ┬─┬
          │                             │ │
          │                             │ 1. source
          ╰╴                            2. destination
        ");
    }

    #[test]
    fn goto_drop_schema_authorization() {
        assert_snapshot!(goto("
create schema authorization foo;
drop schema foo$0;
"), @r"
          ╭▸ 
        2 │ create schema authorization foo;
          │                             ─── 2. destination
        3 │ drop schema foo;
          ╰╴              ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_schema_defined_after() {
        assert_snapshot!(goto("
drop schema foo$0;
create schema foo;
"), @r"
          ╭▸ 
        2 │ drop schema foo;
          │               ─ 1. source
        3 │ create schema foo;
          ╰╴              ─── 2. destination
        ");
    }

    #[test]
    fn goto_create_schema_embedded_table() {
        assert_snapshot!(goto("
create schema app create table users(id int);
select id from app.users$0;
"), @"
          ╭▸ 
        2 │ create schema app create table users(id int);
          │                                ───── 2. destination
        3 │ select id from app.users;
          ╰╴                       ─ 1. source
        ");
    }

    #[test]
    fn goto_create_schema_embedded_table_column() {
        assert_snapshot!(goto("
create schema app create table users(id int);
select id$0 from app.users;
"), @"
          ╭▸ 
        2 │ create schema app create table users(id int);
          │                                      ── 2. destination
        3 │ select id from app.users;
          ╰╴        ─ 1. source
        ");
    }

    #[test]
    fn goto_create_schema_embedded_view() {
        assert_snapshot!(goto("
create schema app create table users(id int) create view v as select 1;
select 1 from app.v$0;
"), @"
          ╭▸ 
        2 │ create schema app create table users(id int) create view v as select 1;
          │                                                          ─ 2. destination
        3 │ select 1 from app.v;
          ╰╴                  ─ 1. source
        ");
    }

    #[test]
    fn goto_schema_qualifier_in_table() {
        assert_snapshot!(goto("
create schema foo;
create table foo$0.t(a int);
"), @r"
          ╭▸ 
        2 │ create schema foo;
          │               ─── 2. destination
        3 │ create table foo.t(a int);
          ╰╴               ─ 1. source
        ");
    }

    #[test]
    fn goto_schema_qualifier_in_drop_table() {
        assert_snapshot!(goto("
create schema foo;
create table foo.t(a int);
drop table foo$0.t;
"), @r"
          ╭▸ 
        2 │ create schema foo;
          │               ─── 2. destination
        3 │ create table foo.t(a int);
        4 │ drop table foo.t;
          ╰╴             ─ 1. source
        ");
    }

    #[test]
    fn goto_schema_qualifier_multiple_schemas() {
        assert_snapshot!(goto("
create schema foo;
create schema bar;
create table bar$0.t(a int);
"), @r"
          ╭▸ 
        3 │ create schema bar;
          │               ─── 2. destination
        4 │ create table bar.t(a int);
          ╰╴               ─ 1. source
        ");
    }

    #[test]
    fn goto_schema_qualifier_in_function_call() {
        assert_snapshot!(goto(r#"
create schema foo;
create function foo.bar() returns int as $$ begin return 1; end; $$ language plpgsql;
select foo$0.bar();
"#), @r"
          ╭▸ 
        2 │ create schema foo;
          │               ─── 2. destination
        3 │ create function foo.bar() returns int as $$ begin return 1; end; $$ language plpgsql;
        4 │ select foo.bar();
          ╰╴         ─ 1. source
        ");
    }

    #[test]
    fn goto_schema_qualifier_in_function_call_from_clause() {
        assert_snapshot!(goto(r#"
create schema myschema;
create function myschema.get_data() returns table(id int) as $$ begin return query select 1; end; $$ language plpgsql;
select * from myschema$0.get_data();
"#), @r"
          ╭▸ 
        2 │ create schema myschema;
          │               ──────── 2. destination
        3 │ create function myschema.get_data() returns table(id int) as $$ begin return query select 1; end; $$ language plpgsql;
        4 │ select * from myschema.get_data();
          ╰╴                     ─ 1. source
        ");
    }

    #[test]
    fn goto_schema_qualifier_in_select_from() {
        assert_snapshot!(goto("
create schema foo;
create table foo.t(x int);
select x from foo$0.t;
"), @r"
          ╭▸ 
        2 │ create schema foo;
          │               ─── 2. destination
        3 │ create table foo.t(x int);
        4 │ select x from foo.t;
          ╰╴                ─ 1. source
        ");
    }

    #[test]
    fn goto_qualified_column_table() {
        assert_snapshot!(goto("
create table t(a int);
select t$0.a from t;
"), @r"
          ╭▸ 
        2 │ create table t(a int);
          │              ─ 2. destination
        3 │ select t.a from t;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_qualified_column_column() {
        assert_snapshot!(goto("
create table t(a int);
select t.a$0 from t;
"), @r"
          ╭▸ 
        2 │ create table t(a int);
          │                ─ 2. destination
        3 │ select t.a from t;
          ╰╴         ─ 1. source
        ");
    }

    #[test]
    fn goto_three_part_qualified_column_schema() {
        assert_snapshot!(goto("
create schema foo;
create table foo.t(a int);
select foo$0.t.a from t;
"), @r"
          ╭▸ 
        2 │ create schema foo;
          │               ─── 2. destination
        3 │ create table foo.t(a int);
        4 │ select foo.t.a from t;
          ╰╴         ─ 1. source
        ");
    }

    #[test]
    fn goto_three_part_qualified_column_table() {
        assert_snapshot!(goto("
create schema foo;
create table foo.t(a int);
select foo.t$0.a from t;
"), @r"
          ╭▸ 
        3 │ create table foo.t(a int);
          │                  ─ 2. destination
        4 │ select foo.t.a from t;
          ╰╴           ─ 1. source
        ");
    }

    #[test]
    fn goto_three_part_qualified_column_column() {
        assert_snapshot!(goto("
create schema foo;
create table foo.t(a int);
select foo.t.a$0 from t;
"), @r"
          ╭▸ 
        3 │ create table foo.t(a int);
          │                    ─ 2. destination
        4 │ select foo.t.a from t;
          ╰╴             ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_values_column1() {
        assert_snapshot!(goto("
with t as (
    values (1, 2), (3, 4)
)
select column1$0, column2 from t;
"), @r"
          ╭▸ 
        3 │     values (1, 2), (3, 4)
          │             ─ 2. destination
        4 │ )
        5 │ select column1, column2 from t;
          ╰╴             ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_values_column2() {
        assert_snapshot!(goto("
with t as (
    values (1, 2), (3, 4)
)
select column1, column2$0 from t;
"), @r"
          ╭▸ 
        3 │     values (1, 2), (3, 4)
          │                ─ 2. destination
        4 │ )
        5 │ select column1, column2 from t;
          ╰╴                      ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_values_single_column() {
        assert_snapshot!(goto("
with t as (
    values (1), (2), (3)
)
select column1$0 from t;
"), @r"
          ╭▸ 
        3 │     values (1), (2), (3)
          │             ─ 2. destination
        4 │ )
        5 │ select column1 from t;
          ╰╴             ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_values_multiple_rows() {
        assert_snapshot!(goto("
with t as (
    values
        (1, 2, 3),
        (4, 5, 6),
        (7, 8, 9)
)
select column3$0 from t;
"), @r"
          ╭▸ 
        4 │         (1, 2, 3),
          │                ─ 2. destination
          ‡
        8 │ select column3 from t;
          ╰╴             ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_values_uppercase_column_names() {
        assert_snapshot!(goto("
with t as (
    values (1, 2), (3, 4)
)
select COLUMN1$0, COLUMN2 from t;
"), @r"
          ╭▸ 
        3 │     values (1, 2), (3, 4)
          │             ─ 2. destination
        4 │ )
        5 │ select COLUMN1, COLUMN2 from t;
          ╰╴             ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_values_with_explicit_column_list_column1_not_found() {
        goto_not_found(
            "
with t(a, b) as (
    values (1, 2), (3, 4)
)
select column1$0 from t;
",
        );
    }

    #[test]
    fn goto_qualified_column_with_schema_in_from_table() {
        assert_snapshot!(goto("
create table foo.t(a int, b int);
select t$0.a from foo.t;
"), @r"
          ╭▸ 
        2 │ create table foo.t(a int, b int);
          │                  ─ 2. destination
        3 │ select t.a from foo.t;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_qualified_column_with_schema_in_from_column() {
        assert_snapshot!(goto("
create table foo.t(a int, b int);
select t.a$0 from foo.t;
"), @r"
          ╭▸ 
        2 │ create table foo.t(a int, b int);
          │                    ─ 2. destination
        3 │ select t.a from foo.t;
          ╰╴         ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_union_all_column() {
        assert_snapshot!(goto("
with t as (
    select 1 as a, 2 as b
    union all
    select 3, 4
)
select a$0, b from t;
"), @r"
          ╭▸ 
        3 │     select 1 as a, 2 as b
          │                 ─ 2. destination
          ‡
        7 │ select a, b from t;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_union_all_column_second() {
        assert_snapshot!(goto("
with t as (
    select 1 as a, 2 as b
    union all
    select 3, 4
)
select a, b$0 from t;
"), @r"
          ╭▸ 
        3 │     select 1 as a, 2 as b
          │                         ─ 2. destination
          ‡
        7 │ select a, b from t;
          ╰╴          ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_union_column() {
        assert_snapshot!(goto("
with t as (
    select 1 as a, 2 as b
    union
    select 3, 4
)
select a$0 from t;
"), @r"
          ╭▸ 
        3 │     select 1 as a, 2 as b
          │                 ─ 2. destination
          ‡
        7 │ select a from t;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_insert_returning_column() {
        assert_snapshot!(goto("
create table t(a int, b int);
with inserted as (
  insert into t values (1, 2), (3, 4)
  returning a, b
)
select a$0 from inserted;
"), @r"
          ╭▸ 
        5 │   returning a, b
          │             ─ 2. destination
        6 │ )
        7 │ select a from inserted;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_insert_returning_aliased_column() {
        assert_snapshot!(goto("
create table t(a int, b int);
with inserted as (
  insert into t values (1, 2), (3, 4)
  returning a as x
)
select x$0 from inserted;
"), @r"
          ╭▸ 
        5 │   returning a as x
          │                  ─ 2. destination
        6 │ )
        7 │ select x from inserted;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_aggregate() {
        assert_snapshot!(goto("
create aggregate myavg(int) (sfunc = int4_avg_accum, stype = _int8);
drop aggregate myavg$0(int);
"), @r"
          ╭▸ 
        2 │ create aggregate myavg(int) (sfunc = int4_avg_accum, stype = _int8);
          │                  ───── 2. destination
        3 │ drop aggregate myavg(int);
          ╰╴                   ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_aggregate_with_schema() {
        assert_snapshot!(goto("
set search_path to public;
create aggregate myavg(int) (sfunc = int4_avg_accum, stype = _int8);
drop aggregate public.myavg$0(int);
"), @r"
          ╭▸ 
        3 │ create aggregate myavg(int) (sfunc = int4_avg_accum, stype = _int8);
          │                  ───── 2. destination
        4 │ drop aggregate public.myavg(int);
          ╰╴                          ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_aggregate_defined_after() {
        assert_snapshot!(goto("
drop aggregate myavg$0(int);
create aggregate myavg(int) (sfunc = int4_avg_accum, stype = _int8);
"), @r"
          ╭▸ 
        2 │ drop aggregate myavg(int);
          │                    ─ 1. source
        3 │ create aggregate myavg(int) (sfunc = int4_avg_accum, stype = _int8);
          ╰╴                 ───── 2. destination
        ");
    }

    #[test]
    fn goto_aggregate_definition_returns_self() {
        assert_snapshot!(goto("
create aggregate myavg$0(int) (sfunc = int4_avg_accum, stype = _int8);
"), @r"
          ╭▸ 
        2 │ create aggregate myavg(int) (sfunc = int4_avg_accum, stype = _int8);
          │                  ┬───┬
          │                  │   │
          │                  │   1. source
          ╰╴                 2. destination
        ");
    }

    #[test]
    fn goto_drop_aggregate_with_search_path() {
        assert_snapshot!(goto("
create aggregate myavg(int) (sfunc = int4_avg_accum, stype = _int8);
set search_path to bar;
create aggregate myavg(int) (sfunc = int4_avg_accum, stype = _int8);
set search_path to default;
drop aggregate myavg$0(int);
"), @r"
          ╭▸ 
        2 │ create aggregate myavg(int) (sfunc = int4_avg_accum, stype = _int8);
          │                  ───── 2. destination
          ‡
        6 │ drop aggregate myavg(int);
          ╰╴                   ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_aggregate_multiple() {
        assert_snapshot!(goto("
create aggregate avg1(int) (sfunc = int4_avg_accum, stype = _int8);
create aggregate avg2(int) (sfunc = int4_avg_accum, stype = _int8);
drop aggregate avg1(int), avg2$0(int);
"), @r"
          ╭▸ 
        3 │ create aggregate avg2(int) (sfunc = int4_avg_accum, stype = _int8);
          │                  ──── 2. destination
        4 │ drop aggregate avg1(int), avg2(int);
          ╰╴                             ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_aggregate_overloaded() {
        assert_snapshot!(goto("
create aggregate sum(complex) (sfunc = complex_add, stype = complex, initcond = '(0,0)');
create aggregate sum(bigint) (sfunc = bigint_add, stype = bigint, initcond = '0');
drop aggregate sum$0(complex);
"), @r"
          ╭▸ 
        2 │ create aggregate sum(complex) (sfunc = complex_add, stype = complex, initcond = '(0,0)');
          │                  ─── 2. destination
        3 │ create aggregate sum(bigint) (sfunc = bigint_add, stype = bigint, initcond = '0');
        4 │ drop aggregate sum(complex);
          ╰╴                 ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_aggregate_second_overload() {
        assert_snapshot!(goto("
create aggregate sum(complex) (sfunc = complex_add, stype = complex, initcond = '(0,0)');
create aggregate sum(bigint) (sfunc = bigint_add, stype = bigint, initcond = '0');
drop aggregate sum$0(bigint);
"), @r"
          ╭▸ 
        3 │ create aggregate sum(bigint) (sfunc = bigint_add, stype = bigint, initcond = '0');
          │                  ─── 2. destination
        4 │ drop aggregate sum(bigint);
          ╰╴                 ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_routine_function() {
        assert_snapshot!(goto("
create function foo() returns int as $$ select 1 $$ language sql;
drop routine foo$0();
"), @r"
          ╭▸ 
        2 │ create function foo() returns int as $$ select 1 $$ language sql;
          │                 ─── 2. destination
        3 │ drop routine foo();
          ╰╴               ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_routine_aggregate() {
        assert_snapshot!(goto("
create aggregate myavg(int) (sfunc = int4_avg_accum, stype = _int8);
drop routine myavg$0(int);
"), @r"
          ╭▸ 
        2 │ create aggregate myavg(int) (sfunc = int4_avg_accum, stype = _int8);
          │                  ───── 2. destination
        3 │ drop routine myavg(int);
          ╰╴                 ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_routine_with_schema() {
        assert_snapshot!(goto("
set search_path to public;
create function foo() returns int as $$ select 1 $$ language sql;
drop routine public.foo$0();
"), @r"
          ╭▸ 
        3 │ create function foo() returns int as $$ select 1 $$ language sql;
          │                 ─── 2. destination
        4 │ drop routine public.foo();
          ╰╴                      ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_routine_defined_after() {
        assert_snapshot!(goto("
drop routine foo$0();
create function foo() returns int as $$ select 1 $$ language sql;
"), @r"
          ╭▸ 
        2 │ drop routine foo();
          │                ─ 1. source
        3 │ create function foo() returns int as $$ select 1 $$ language sql;
          ╰╴                ─── 2. destination
        ");
    }

    #[test]
    fn goto_drop_routine_with_search_path() {
        assert_snapshot!(goto("
create function foo() returns int as $$ select 1 $$ language sql;
set search_path to bar;
create function foo() returns int as $$ select 1 $$ language sql;
set search_path to default;
drop routine foo$0();
"), @r"
          ╭▸ 
        2 │ create function foo() returns int as $$ select 1 $$ language sql;
          │                 ─── 2. destination
          ‡
        6 │ drop routine foo();
          ╰╴               ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_routine_overloaded() {
        assert_snapshot!(goto("
create function add(complex) returns complex as $$ select null $$ language sql;
create function add(bigint) returns bigint as $$ select 1 $$ language sql;
drop routine add$0(complex);
"), @r"
          ╭▸ 
        2 │ create function add(complex) returns complex as $$ select null $$ language sql;
          │                 ─── 2. destination
        3 │ create function add(bigint) returns bigint as $$ select 1 $$ language sql;
        4 │ drop routine add(complex);
          ╰╴               ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_routine_second_overload() {
        assert_snapshot!(goto("
create function add(complex) returns complex as $$ select null $$ language sql;
create function add(bigint) returns bigint as $$ select 1 $$ language sql;
drop routine add$0(bigint);
"), @r"
          ╭▸ 
        3 │ create function add(bigint) returns bigint as $$ select 1 $$ language sql;
          │                 ─── 2. destination
        4 │ drop routine add(bigint);
          ╰╴               ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_routine_aggregate_overloaded() {
        assert_snapshot!(goto("
create aggregate sum(complex) (sfunc = complex_add, stype = complex, initcond = '(0,0)');
create aggregate sum(bigint) (sfunc = bigint_add, stype = bigint, initcond = '0');
drop routine sum$0(complex);
"), @r"
          ╭▸ 
        2 │ create aggregate sum(complex) (sfunc = complex_add, stype = complex, initcond = '(0,0)');
          │                  ─── 2. destination
        3 │ create aggregate sum(bigint) (sfunc = bigint_add, stype = bigint, initcond = '0');
        4 │ drop routine sum(complex);
          ╰╴               ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_routine_multiple() {
        assert_snapshot!(goto("
create function foo() returns int as $$ select 1 $$ language sql;
create function bar() returns int as $$ select 1 $$ language sql;
drop routine foo(), bar$0();
"), @r"
          ╭▸ 
        3 │ create function bar() returns int as $$ select 1 $$ language sql;
          │                 ─── 2. destination
        4 │ drop routine foo(), bar();
          ╰╴                      ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_procedure() {
        assert_snapshot!(goto("
create procedure foo() language sql as $$ select 1 $$;
drop procedure foo$0();
"), @r"
          ╭▸ 
        2 │ create procedure foo() language sql as $$ select 1 $$;
          │                  ─── 2. destination
        3 │ drop procedure foo();
          ╰╴                 ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_procedure_with_schema() {
        assert_snapshot!(goto("
set search_path to public;
create procedure foo() language sql as $$ select 1 $$;
drop procedure public.foo$0();
"), @r"
          ╭▸ 
        3 │ create procedure foo() language sql as $$ select 1 $$;
          │                  ─── 2. destination
        4 │ drop procedure public.foo();
          ╰╴                        ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_procedure_defined_after() {
        assert_snapshot!(goto("
drop procedure foo$0();
create procedure foo() language sql as $$ select 1 $$;
"), @r"
          ╭▸ 
        2 │ drop procedure foo();
          │                  ─ 1. source
        3 │ create procedure foo() language sql as $$ select 1 $$;
          ╰╴                 ─── 2. destination
        ");
    }

    #[test]
    fn goto_drop_procedure_with_search_path() {
        assert_snapshot!(goto("
create procedure foo() language sql as $$ select 1 $$;
set search_path to bar;
create procedure foo() language sql as $$ select 1 $$;
set search_path to default;
drop procedure foo$0();
"), @r"
          ╭▸ 
        2 │ create procedure foo() language sql as $$ select 1 $$;
          │                  ─── 2. destination
          ‡
        6 │ drop procedure foo();
          ╰╴                 ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_procedure_overloaded() {
        assert_snapshot!(goto("
create procedure add(complex) language sql as $$ select null $$;
create procedure add(bigint) language sql as $$ select 1 $$;
drop procedure add$0(complex);
"), @r"
          ╭▸ 
        2 │ create procedure add(complex) language sql as $$ select null $$;
          │                  ─── 2. destination
        3 │ create procedure add(bigint) language sql as $$ select 1 $$;
        4 │ drop procedure add(complex);
          ╰╴                 ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_procedure_second_overload() {
        assert_snapshot!(goto("
create procedure add(complex) language sql as $$ select null $$;
create procedure add(bigint) language sql as $$ select 1 $$;
drop procedure add$0(bigint);
"), @r"
          ╭▸ 
        3 │ create procedure add(bigint) language sql as $$ select 1 $$;
          │                  ─── 2. destination
        4 │ drop procedure add(bigint);
          ╰╴                 ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_procedure_multiple() {
        assert_snapshot!(goto("
create procedure foo() language sql as $$ select 1 $$;
create procedure bar() language sql as $$ select 1 $$;
drop procedure foo(), bar$0();
"), @r"
          ╭▸ 
        3 │ create procedure bar() language sql as $$ select 1 $$;
          │                  ─── 2. destination
        4 │ drop procedure foo(), bar();
          ╰╴                        ─ 1. source
        ");
    }

    #[test]
    fn goto_procedure_definition_returns_self() {
        assert_snapshot!(goto("
create procedure foo$0() language sql as $$ select 1 $$;
"), @r"
          ╭▸ 
        2 │ create procedure foo() language sql as $$ select 1 $$;
          │                  ┬─┬
          │                  │ │
          │                  │ 1. source
          ╰╴                 2. destination
        ");
    }

    #[test]
    fn goto_call_procedure() {
        assert_snapshot!(goto("
create procedure foo() language sql as $$ select 1 $$;
call foo$0();
"), @r"
          ╭▸ 
        2 │ create procedure foo() language sql as $$ select 1 $$;
          │                  ─── 2. destination
        3 │ call foo();
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_call_procedure_with_schema() {
        assert_snapshot!(goto("
create procedure public.foo() language sql as $$ select 1 $$;
call public.foo$0();
"), @r"
          ╭▸ 
        2 │ create procedure public.foo() language sql as $$ select 1 $$;
          │                         ─── 2. destination
        3 │ call public.foo();
          ╰╴              ─ 1. source
        ");
    }

    #[test]
    fn goto_call_procedure_with_search_path() {
        assert_snapshot!(goto("
set search_path to myschema;
create procedure foo() language sql as $$ select 1 $$;
call myschema.foo$0();
"), @r"
          ╭▸ 
        3 │ create procedure foo() language sql as $$ select 1 $$;
          │                  ─── 2. destination
        4 │ call myschema.foo();
          ╰╴                ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_routine_procedure() {
        assert_snapshot!(goto("
create procedure foo() language sql as $$ select 1 $$;
drop routine foo$0();
"), @r"
          ╭▸ 
        2 │ create procedure foo() language sql as $$ select 1 $$;
          │                  ─── 2. destination
        3 │ drop routine foo();
          ╰╴               ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_routine_prefers_function_over_procedure() {
        assert_snapshot!(goto("
create function foo() returns int as $$ select 1 $$ language sql;
create procedure foo() language sql as $$ select 1 $$;
drop routine foo$0();
"), @r"
          ╭▸ 
        2 │ create function foo() returns int as $$ select 1 $$ language sql;
          │                 ─── 2. destination
        3 │ create procedure foo() language sql as $$ select 1 $$;
        4 │ drop routine foo();
          ╰╴               ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_routine_prefers_aggregate_over_procedure() {
        assert_snapshot!(goto("
create aggregate foo(int) (sfunc = int4_avg_accum, stype = _int8);
create procedure foo(int) language sql as $$ select 1 $$;
drop routine foo$0(int);
"), @r"
          ╭▸ 
        2 │ create aggregate foo(int) (sfunc = int4_avg_accum, stype = _int8);
          │                  ─── 2. destination
        3 │ create procedure foo(int) language sql as $$ select 1 $$;
        4 │ drop routine foo(int);
          ╰╴               ─ 1. source
        ");
    }

    #[test]
    fn goto_table_alias_in_qualified_column() {
        assert_snapshot!(goto("
create table t(a int8, b text);
select f$0.a from t as f;
"), @r"
          ╭▸ 
        3 │ select f.a from t as f;
          ╰╴       ─ 1. source   ─ 2. destination
        ");
    }

    #[test]
    fn goto_column_through_table_alias() {
        assert_snapshot!(goto("
create table t(a int8, b text);
select f.a$0 from t as f;
"), @r"
          ╭▸ 
        2 │ create table t(a int8, b text);
          │                ─ 2. destination
        3 │ select f.a from t as f;
          ╰╴         ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_alias_renamed_column() {
        assert_snapshot!(goto("
with t as (select 1 a, 2 b)
select f.x$0 from t as f(x);
"), @r"
          ╭▸ 
        3 │ select f.x from t as f(x);
          ╰╴         ─ 1. source   ─ 2. destination
        ");
    }

    #[test]
    fn goto_cte_alias_unrenamed_column() {
        assert_snapshot!(goto("
with t as (select 1 a, 2 b)
select f.b$0 from t as f(x);
"), @r"
          ╭▸ 
        2 │ with t as (select 1 a, 2 b)
          │                          ─ 2. destination
        3 │ select f.b from t as f(x);
          ╰╴         ─ 1. source
        ");
    }

    #[test]
    fn goto_join_table() {
        assert_snapshot!(goto("
create table users(id int, email text);
create table messages(id int, user_id int, message text);
select * from users join messages$0 on users.id = messages.user_id;
"), @r"
          ╭▸ 
        3 │ create table messages(id int, user_id int, message text);
          │              ──────── 2. destination
        4 │ select * from users join messages on users.id = messages.user_id;
          ╰╴                                ─ 1. source
        ");
    }

    #[test]
    fn goto_join_qualified_column_from_joined_table() {
        assert_snapshot!(goto("
create table users(id int, email text);
create table messages(id int, user_id int, message text);
select messages.user_id$0 from users join messages on users.id = messages.user_id;
"), @r"
          ╭▸ 
        3 │ create table messages(id int, user_id int, message text);
          │                               ─────── 2. destination
        4 │ select messages.user_id from users join messages on users.id = messages.user_id;
          ╰╴                      ─ 1. source
        ");
    }

    #[test]
    fn goto_join_qualified_column_from_base_table() {
        assert_snapshot!(goto("
create table users(id int, email text);
create table messages(id int, user_id int, message text);
select users.id$0 from users join messages on users.id = messages.user_id;
"), @r"
          ╭▸ 
        2 │ create table users(id int, email text);
          │                    ── 2. destination
        3 │ create table messages(id int, user_id int, message text);
        4 │ select users.id from users join messages on users.id = messages.user_id;
          ╰╴              ─ 1. source
        ");
    }

    #[test]
    fn goto_join_multiple_joins() {
        assert_snapshot!(goto("
create table users(id int, name text);
create table messages(id int, user_id int, message text);
create table comments(id int, message_id int, text text);
select comments.text$0 from users
  join messages on users.id = messages.user_id
  join comments on messages.id = comments.message_id;
"), @r"
          ╭▸ 
        4 │ create table comments(id int, message_id int, text text);
          │                                               ──── 2. destination
        5 │ select comments.text from users
          ╰╴                   ─ 1. source
        ");
    }

    #[test]
    fn goto_join_with_aliases() {
        assert_snapshot!(goto("
create table users(id int, name text);
create table messages(id int, user_id int, message text);
select m.message$0 from users as u join messages as m on u.id = m.user_id;
"), @r"
          ╭▸ 
        3 │ create table messages(id int, user_id int, message text);
          │                                            ─────── 2. destination
        4 │ select m.message from users as u join messages as m on u.id = m.user_id;
          ╰╴               ─ 1. source
        ");
    }

    #[test]
    fn goto_alias_hides_table_name() {
        goto_not_found(
            "
create table t(a int);
select t$0.a from t as u;
",
        );
    }

    #[test]
    fn goto_join_unqualified_column() {
        assert_snapshot!(goto("
create table users(id int, email text);
create table messages(id int, user_id int, message text);
select message$0 from users join messages on users.id = messages.user_id;
"), @r"
          ╭▸ 
        3 │ create table messages(id int, user_id int, message text);
          │                                            ─────── 2. destination
        4 │ select message from users join messages on users.id = messages.user_id;
          ╰╴             ─ 1. source
        ");
    }

    #[test]
    fn goto_join_with_many_tables() {
        assert_snapshot!(goto("
create table users(id int, email text);
create table messages(id int, user_id int, message text);
create table logins(id int, user_id int, at timestamptz);
create table posts(id int, user_id int, post text);

select post$0 
  from users
    join messages 
      on users.id = messages.user_id
      join logins
        on users.id = logins.user_id
        join posts
          on users.id = posts.user_id
"), @r"
          ╭▸ 
        5 │ create table posts(id int, user_id int, post text);
          │                                         ──── 2. destination
        6 │
        7 │ select post 
          ╰╴          ─ 1. source
        ");
    }

    #[test]
    fn goto_join_with_schema() {
        assert_snapshot!(goto("
create schema foo;
create table foo.users(id int, email text);
create table foo.messages(id int, user_id int, message text);
select foo.messages.message$0 from foo.users join foo.messages on foo.users.id = foo.messages.user_id;
"), @r"
          ╭▸ 
        4 │ create table foo.messages(id int, user_id int, message text);
          │                                                ─────── 2. destination
        5 │ select foo.messages.message from foo.users join foo.messages on foo.users.id = foo.messages.user_id;
          ╰╴                          ─ 1. source
        ");
    }

    #[test]
    fn goto_join_left_join() {
        assert_snapshot!(goto("
create table users(id int, email text);
create table messages(id int, user_id int, message text);
select messages.message$0 from users left join messages on users.id = messages.user_id;
"), @r"
          ╭▸ 
        3 │ create table messages(id int, user_id int, message text);
          │                                            ─────── 2. destination
        4 │ select messages.message from users left join messages on users.id = messages.user_id;
          ╰╴                      ─ 1. source
        ");
    }

    #[test]
    fn goto_join_on_table_qualifier() {
        assert_snapshot!(goto("
create table t(a int);
create table u(a int);
select * from t join u on u$0.a = t.a;
"), @r"
          ╭▸ 
        3 │ create table u(a int);
          │              ─ 2. destination
        4 │ select * from t join u on u.a = t.a;
          ╰╴                          ─ 1. source
        ");
    }

    #[test]
    fn goto_join_on_column() {
        assert_snapshot!(goto("
create table t(a int);
create table u(a int);
select * from t join u on u.a$0 = t.a;
"), @r"
          ╭▸ 
        3 │ create table u(a int);
          │                ─ 2. destination
        4 │ select * from t join u on u.a = t.a;
          ╰╴                            ─ 1. source
        ");
    }

    #[test]
    fn goto_join_using_column() {
        assert_snapshot!(goto("
create table t(a int);
create table u(a int);
select * from t join u using (a$0);
"), @r"
          ╭▸ 
        2 │ create table t(a int);
          │                ─ 2. destination
        3 │ create table u(a int);
          │                ─ 3. destination
        4 │ select * from t join u using (a);
          ╰╴                              ─ 1. source
        ");
    }

    #[test]
    fn goto_join_using_alias_column() {
        assert_snapshot!(goto("
create table a(x int);
create table b(x int);
select j.x$0 from a join b using (x) as j;
"), @"
          ╭▸ 
        2 │ create table a(x int);
          │                ─ 2. destination
        3 │ create table b(x int);
          │                ─ 3. destination
        4 │ select j.x from a join b using (x) as j;
          ╰╴         ─ 1. source
        ");
    }

    #[test]
    fn goto_join_using_alias_table() {
        assert_snapshot!(goto("
create table a(x int);
create table b(x int);
select j$0.x from a join b using (x) as j;
"), @"
          ╭▸ 
        4 │ select j.x from a join b using (x) as j;
          ╰╴       ─ 1. source                    ─ 2. destination
        ");
    }

    #[test]
    fn goto_insert_select_cte_column() {
        assert_snapshot!(goto("
create table users(id int, email text);
with new_data as (
    select 1 as id, 'test@example.com' as email
)
insert into users (id, email)
select id$0, email from new_data;
"), @r"
          ╭▸ 
        4 │     select 1 as id, 'test@example.com' as email
          │                 ── 2. destination
          ‡
        7 │ select id, email from new_data;
          ╰╴        ─ 1. source
        ");
    }

    #[test]
    fn goto_insert_select_cte_column_second() {
        assert_snapshot!(goto("
create table users(id int, email text);
with new_data as (
    select 1 as id, 'test@example.com' as email
)
insert into users (id, email)
select id, email$0 from new_data;
"), @r"
          ╭▸ 
        4 │     select 1 as id, 'test@example.com' as email
          │                                           ───── 2. destination
          ‡
        7 │ select id, email from new_data;
          ╰╴               ─ 1. source
        ");
    }

    #[test]
    fn goto_insert_select_cte_table() {
        assert_snapshot!(goto("
create table users(id int, email text);
with new_data as (
    select 1 as id, 'test@example.com' as email
)
insert into users (id, email)
select id, email from new_data$0;
"), @r"
          ╭▸ 
        3 │ with new_data as (
          │      ──────── 2. destination
          ‡
        7 │ select id, email from new_data;
          ╰╴                             ─ 1. source
        ");
    }

    #[test]
    fn goto_delete_cte_column() {
        assert_snapshot!(goto("
create table users(id int, email text);
with old_data as (
    select 1 as id
)
delete from users where id in (select id$0 from old_data);
"), @r"
          ╭▸ 
        4 │     select 1 as id
          │                 ── 2. destination
        5 │ )
        6 │ delete from users where id in (select id from old_data);
          ╰╴                                       ─ 1. source
        ");
    }

    #[test]
    fn goto_update_table() {
        assert_snapshot!(goto("
create table users(id int, email text);
update users$0 set email = 'new@example.com';
"), @r"
          ╭▸ 
        2 │ create table users(id int, email text);
          │              ───── 2. destination
        3 │ update users set email = 'new@example.com';
          ╰╴           ─ 1. source
        ");
    }

    #[test]
    fn goto_update_table_with_schema() {
        assert_snapshot!(goto("
create table public.users(id int, email text);
update public.users$0 set email = 'new@example.com';
"), @r"
          ╭▸ 
        2 │ create table public.users(id int, email text);
          │                     ───── 2. destination
        3 │ update public.users set email = 'new@example.com';
          ╰╴                  ─ 1. source
        ");
    }

    #[test]
    fn goto_update_table_with_search_path() {
        assert_snapshot!(goto("
set search_path to foo;
create table foo.users(id int, email text);
update users$0 set email = 'new@example.com';
"), @r"
          ╭▸ 
        3 │ create table foo.users(id int, email text);
          │                  ───── 2. destination
        4 │ update users set email = 'new@example.com';
          ╰╴           ─ 1. source
        ");
    }

    #[test]
    fn goto_update_where_column() {
        assert_snapshot!(goto("
create table users(id int, email text);
update users set email = 'new@example.com' where id$0 = 1;
"), @r"
          ╭▸ 
        2 │ create table users(id int, email text);
          │                    ── 2. destination
        3 │ update users set email = 'new@example.com' where id = 1;
          ╰╴                                                  ─ 1. source
        ");
    }

    #[test]
    fn goto_update_where_column_with_schema() {
        assert_snapshot!(goto("
create table public.users(id int, email text);
update public.users set email = 'new@example.com' where id$0 = 1;
"), @r"
          ╭▸ 
        2 │ create table public.users(id int, email text);
          │                           ── 2. destination
        3 │ update public.users set email = 'new@example.com' where id = 1;
          ╰╴                                                         ─ 1. source
        ");
    }

    #[test]
    fn goto_update_where_column_with_search_path() {
        assert_snapshot!(goto("
set search_path to foo;
create table foo.users(id int, email text);
update users set email = 'new@example.com' where id$0 = 1;
"), @r"
          ╭▸ 
        3 │ create table foo.users(id int, email text);
          │                        ── 2. destination
        4 │ update users set email = 'new@example.com' where id = 1;
          ╰╴                                                  ─ 1. source
        ");
    }

    #[test]
    fn goto_update_set_column() {
        assert_snapshot!(goto("
create table users(id int, email text);
update users set email$0 = 'new@example.com' where id = 1;
"), @r"
          ╭▸ 
        2 │ create table users(id int, email text);
          │                            ───── 2. destination
        3 │ update users set email = 'new@example.com' where id = 1;
          ╰╴                     ─ 1. source
        ");
    }

    #[test]
    fn goto_update_set_column_with_schema() {
        assert_snapshot!(goto("
create table public.users(id int, email text);
update public.users set email$0 = 'new@example.com' where id = 1;
"), @r"
          ╭▸ 
        2 │ create table public.users(id int, email text);
          │                                   ───── 2. destination
        3 │ update public.users set email = 'new@example.com' where id = 1;
          ╰╴                            ─ 1. source
        ");
    }

    #[test]
    fn goto_update_set_column_with_search_path() {
        assert_snapshot!(goto("
set search_path to foo;
create table foo.users(id int, email text);
update users set email$0 = 'new@example.com' where id = 1;
"), @r"
          ╭▸ 
        3 │ create table foo.users(id int, email text);
          │                                ───── 2. destination
        4 │ update users set email = 'new@example.com' where id = 1;
          ╰╴                     ─ 1. source
        ");
    }

    #[test]
    fn goto_update_from_table() {
        assert_snapshot!(goto("
create table users(id int, email text);
create table messages(id int, user_id int, email text);
update users set email = messages.email from messages$0 where users.id = messages.user_id;
"), @r"
          ╭▸ 
        3 │ create table messages(id int, user_id int, email text);
          │              ──────── 2. destination
        4 │ update users set email = messages.email from messages where users.id = messages.user_id;
          ╰╴                                                    ─ 1. source
        ");
    }

    #[test]
    fn goto_update_from_table_qualifier_in_set() {
        assert_snapshot!(goto("
create table target(id int, x int);
create table src(id int, y int);
update target set x = src$0.y from src where src.id = target.id;
"), @"
          ╭▸ 
        3 │ create table src(id int, y int);
          │              ─── 2. destination
        4 │ update target set x = src.y from src where src.id = target.id;
          ╰╴                        ─ 1. source
        ");
    }

    #[test]
    fn goto_update_set_target_resolves_to_target_table() {
        assert_snapshot!(goto("
create table t (a int);
create table u (a int);
update t set a$0 = u.a from u;
"), @"
          ╭▸ 
        2 │ create table t (a int);
          │                 ─ 2. destination
        3 │ create table u (a int);
        4 │ update t set a = u.a from u;
          ╰╴             ─ 1. source
        ");
    }

    #[test]
    fn goto_update_set_target_tuple_resolves_to_target_table() {
        assert_snapshot!(goto("
create table t (a int, b int);
create table u (a int);
update t set (a$0, b) = (u.a, 1) from u;
"), @"
          ╭▸ 
        2 │ create table t (a int, b int);
          │                 ─ 2. destination
        3 │ create table u (a int);
        4 │ update t set (a, b) = (u.a, 1) from u;
          ╰╴              ─ 1. source
        ");
    }

    #[test]
    fn goto_update_from_table_with_schema() {
        assert_snapshot!(goto("
create table users(id int, email text);
create table public.messages(id int, user_id int, email text);
update users set email = messages.email from public.messages$0 where users.id = messages.user_id;
"), @r"
          ╭▸ 
        3 │ create table public.messages(id int, user_id int, email text);
          │                     ──────── 2. destination
        4 │ update users set email = messages.email from public.messages where users.id = messages.user_id;
          ╰╴                                                           ─ 1. source
        ");
    }

    #[test]
    fn goto_update_from_table_with_search_path() {
        assert_snapshot!(goto("
set search_path to foo;
create table users(id int, email text);
create table foo.messages(id int, user_id int, email text);
update users set email = messages.email from messages$0 where users.id = messages.user_id;
"), @r"
          ╭▸ 
        4 │ create table foo.messages(id int, user_id int, email text);
          │                  ──────── 2. destination
        5 │ update users set email = messages.email from messages where users.id = messages.user_id;
          ╰╴                                                    ─ 1. source
        ");
    }

    #[test]
    fn goto_update_with_cte_table() {
        assert_snapshot!(goto("
create table users(id int, email text);
with new_data as (
    select 1 as id, 'new@example.com' as email
)
update users set email = new_data.email from new_data$0 where users.id = new_data.id;
"), @r"
          ╭▸ 
        3 │ with new_data as (
          │      ──────── 2. destination
          ‡
        6 │ update users set email = new_data.email from new_data where users.id = new_data.id;
          ╰╴                                                    ─ 1. source
        ");
    }

    #[test]
    fn goto_update_with_cte_column_in_set() {
        assert_snapshot!(goto("
create table users(id int, email text);
with new_data as (
    select 1 as id, 'new@example.com' as email
)
update users set email = new_data.email$0 from new_data where users.id = new_data.id;
"), @r"
          ╭▸ 
        4 │     select 1 as id, 'new@example.com' as email
          │                                          ───── 2. destination
        5 │ )
        6 │ update users set email = new_data.email from new_data where users.id = new_data.id;
          ╰╴                                      ─ 1. source
        ");
    }

    #[test]
    fn goto_update_with_cte_column_in_where() {
        assert_snapshot!(goto("
create table users(id int, email text);
with new_data as (
    select 1 as id, 'new@example.com' as email
)
update users set email = new_data.email from new_data where new_data.id$0 = users.id;
"), @r"
          ╭▸ 
        4 │     select 1 as id, 'new@example.com' as email
          │                 ── 2. destination
        5 │ )
        6 │ update users set email = new_data.email from new_data where new_data.id = users.id;
          ╰╴                                                                      ─ 1. source
        ");
    }

    #[test]
    fn goto_update_with_cte_values() {
        assert_snapshot!(goto("
create table users(id int, email text);
with new_data as (
    values (1, 'new@example.com')
)
update users set email = new_data.column2$0 from new_data where users.id = new_data.column1;
"), @r"
          ╭▸ 
        4 │     values (1, 'new@example.com')
          │                ───────────────── 2. destination
        5 │ )
        6 │ update users set email = new_data.column2 from new_data where users.id = new_data.column1;
          ╰╴                                        ─ 1. source
        ");
    }

    #[test]
    fn goto_truncate_table() {
        assert_snapshot!(goto("
create table t();
truncate table t$0;
"), @r"
          ╭▸ 
        2 │ create table t();
          │              ─ 2. destination
        3 │ truncate table t;
          ╰╴               ─ 1. source
        ");
    }

    #[test]
    fn goto_truncate_table_without_table_keyword() {
        assert_snapshot!(goto("
create table t();
truncate t$0;
"), @r"
          ╭▸ 
        2 │ create table t();
          │              ─ 2. destination
        3 │ truncate t;
          ╰╴         ─ 1. source
        ");
    }

    #[test]
    fn goto_truncate_multiple_tables() {
        assert_snapshot!(goto("
create table t1();
create table t2();
truncate t1, t2$0;
"), @r"
          ╭▸ 
        3 │ create table t2();
          │              ── 2. destination
        4 │ truncate t1, t2;
          ╰╴              ─ 1. source
        ");
    }

    #[test]
    fn goto_lock_table() {
        assert_snapshot!(goto("
create table t();
lock table t$0;
"), @r"
          ╭▸ 
        2 │ create table t();
          │              ─ 2. destination
        3 │ lock table t;
          ╰╴           ─ 1. source
        ");
    }

    #[test]
    fn goto_lock_table_without_table_keyword() {
        assert_snapshot!(goto("
create table t();
lock t$0;
"), @r"
          ╭▸ 
        2 │ create table t();
          │              ─ 2. destination
        3 │ lock t;
          ╰╴     ─ 1. source
        ");
    }

    #[test]
    fn goto_lock_multiple_tables() {
        assert_snapshot!(goto("
create table t1();
create table t2();
lock t1, t2$0;
"), @r"
          ╭▸ 
        3 │ create table t2();
          │              ── 2. destination
        4 │ lock t1, t2;
          ╰╴          ─ 1. source
        ");
    }

    #[test]
    fn goto_vacuum_table() {
        assert_snapshot!(goto("
create table users(id int, email text);
vacuum users$0;
"), @r"
          ╭▸ 
        2 │ create table users(id int, email text);
          │              ───── 2. destination
        3 │ vacuum users;
          ╰╴           ─ 1. source
        ");
    }

    #[test]
    fn goto_vacuum_multiple_tables() {
        assert_snapshot!(goto("
create table t1();
create table t2();
vacuum t1, t2$0;
"), @r"
          ╭▸ 
        3 │ create table t2();
          │              ── 2. destination
        4 │ vacuum t1, t2;
          ╰╴            ─ 1. source
        ");
    }

    #[test]
    fn goto_vacuum_column() {
        assert_snapshot!(goto("
create table users(id int, email text);
vacuum users (id$0);
"), @"
          ╭▸ 
        2 │ create table users(id int, email text);
          │                    ── 2. destination
        3 │ vacuum users (id);
          ╰╴               ─ 1. source
        ");
    }

    #[test]
    fn goto_analyze_table() {
        assert_snapshot!(goto("
create table users(id int, email text);
analyze users$0;
"), @"
          ╭▸ 
        2 │ create table users(id int, email text);
          │              ───── 2. destination
        3 │ analyze users;
          ╰╴            ─ 1. source
        ");
    }

    #[test]
    fn goto_analyze_column() {
        assert_snapshot!(goto("
create table users(id int, email text);
analyze users (id$0);
"), @"
          ╭▸ 
        2 │ create table users(id int, email text);
          │                    ── 2. destination
        3 │ analyze users (id);
          ╰╴                ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_table() {
        assert_snapshot!(goto("
create table users(id int, email text);
alter table users$0 alter email set not null;
"), @r"
          ╭▸ 
        2 │ create table users(id int, email text);
          │              ───── 2. destination
        3 │ alter table users alter email set not null;
          ╰╴                ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_table_column() {
        assert_snapshot!(goto("
create table users(id int, email text);
alter table users alter email$0 set not null;
"), @r"
          ╭▸ 
        2 │ create table users(id int, email text);
          │                            ───── 2. destination
        3 │ alter table users alter email set not null;
          ╰╴                            ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_table_column_with_column_keyword() {
        assert_snapshot!(goto("
create table users(id int, email text);
alter table users alter column email$0 set not null;
"), @r"
          ╭▸ 
        2 │ create table users(id int, email text);
          │                            ───── 2. destination
        3 │ alter table users alter column email set not null;
          ╰╴                                   ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_table_rename_column() {
        assert_snapshot!(goto("
create table users(id int, email text);
alter table users rename column email$0 to email_address;
"), @"
          ╭▸ 
        2 │ create table users(id int, email text);
          │                            ───── 2. destination
        3 │ alter table users rename column email to email_address;
          ╰╴                                    ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_view_alter_column() {
        assert_snapshot!(goto("
create table t(a int);
create view v as select a from t;
alter view v alter column a$0 set default 1;
"), @"
          ╭▸ 
        3 │ create view v as select a from t;
          │                         ─ 2. destination
        4 │ alter view v alter column a set default 1;
          ╰╴                          ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_view_rename_column() {
        assert_snapshot!(goto("
create table t(a int);
create view v as select a from t;
alter view v rename column a$0 to b;
"), @"
          ╭▸ 
        3 │ create view v as select a from t;
          │                         ─ 2. destination
        4 │ alter view v rename column a to b;
          ╰╴                           ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_materialized_view_rename_column() {
        assert_snapshot!(goto("
create table t(a int);
create materialized view mv as select a from t;
alter materialized view mv rename column a$0 to b;
"), @"
          ╭▸ 
        3 │ create materialized view mv as select a from t;
          │                                       ─ 2. destination
        4 │ alter materialized view mv rename column a to b;
          ╰╴                                         ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_table_add_column() {
        assert_snapshot!(goto("
create table users(id int);
alter table users$0 add column email text;
"), @r"
          ╭▸ 
        2 │ create table users(id int);
          │              ───── 2. destination
        3 │ alter table users add column email text;
          ╰╴                ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_table_drop_column() {
        assert_snapshot!(goto("
create table users(id int, email text);
alter table users drop column email$0;
"), @r"
          ╭▸ 
        2 │ create table users(id int, email text);
          │                            ───── 2. destination
        3 │ alter table users drop column email;
          ╰╴                                  ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_table_drop_column_table_name() {
        assert_snapshot!(goto("
create table users(id int, email text);
alter table users$0 drop column email;
"), @r"
          ╭▸ 
        2 │ create table users(id int, email text);
          │              ───── 2. destination
        3 │ alter table users drop column email;
          ╰╴                ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_table_add_constraint_using_index() {
        assert_snapshot!(goto("
create table u(id int);
create index my_index on u (id);
alter table u add constraint uq unique using index my_in$0dex;
"), @r"
          ╭▸ 
        3 │ create index my_index on u (id);
          │              ──────── 2. destination
        4 │ alter table u add constraint uq unique using index my_index;
          ╰╴                                                       ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_table_owner_to_role() {
        assert_snapshot!(goto("
create role reader;
create table t(id int);
alter table t owner to read$0er;
"), @r"
          ╭▸ 
        2 │ create role reader;
          │             ────── 2. destination
        3 │ create table t(id int);
        4 │ alter table t owner to reader;
          ╰╴                          ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_table_set_tablespace() {
        assert_snapshot!(goto("
create tablespace ts location '/tmp/ts';
create table t(id int);
alter table t set tablespace t$0s;
"), @r"
          ╭▸ 
        2 │ create tablespace ts location '/tmp/ts';
          │                   ── 2. destination
        3 │ create table t(id int);
        4 │ alter table t set tablespace ts;
          ╰╴                             ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_table_all_in_tablespace() {
        assert_snapshot!(goto("
create tablespace ts location '/tmp/ts';
alter table all in tablespace t$0s set tablespace pg_default;
"), @"
          ╭▸ 
        2 │ create tablespace ts location '/tmp/ts';
          │                   ── 2. destination
        3 │ alter table all in tablespace ts set tablespace pg_default;
          ╰╴                              ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_materialized_view_all_in_tablespace() {
        assert_snapshot!(goto("
create tablespace ts location '/tmp/ts';
alter materialized view all in tablespace t$0s set tablespace pg_default;
"), @"
          ╭▸ 
        2 │ create tablespace ts location '/tmp/ts';
          │                   ── 2. destination
        3 │ alter materialized view all in tablespace ts set tablespace pg_default;
          ╰╴                                          ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_index_all_in_tablespace() {
        assert_snapshot!(goto("
create tablespace ts location '/tmp/ts';
alter index all in tablespace t$0s set tablespace pg_default;
"), @"
          ╭▸ 
        2 │ create tablespace ts location '/tmp/ts';
          │                   ── 2. destination
        3 │ alter index all in tablespace ts set tablespace pg_default;
          ╰╴                              ─ 1. source
        ");
    }

    #[test]
    fn goto_create_database_owner() {
        assert_snapshot!(goto("
create role r;
create database d owner r$0;
"), @"
          ╭▸ 
        2 │ create role r;
          │             ─ 2. destination
        3 │ create database d owner r;
          ╰╴                        ─ 1. source
        ");
    }

    #[test]
    fn goto_create_database_template() {
        assert_snapshot!(goto("
create database tmpl;
create database d template tmpl$0;
"), @"
          ╭▸ 
        2 │ create database tmpl;
          │                 ──── 2. destination
        3 │ create database d template tmpl;
          ╰╴                              ─ 1. source
        ");
    }

    #[test]
    fn goto_create_database_tablespace() {
        assert_snapshot!(goto("
create tablespace ts location '/tmp';
create database d tablespace ts$0;
"), @"
          ╭▸ 
        2 │ create tablespace ts location '/tmp';
          │                   ── 2. destination
        3 │ create database d tablespace ts;
          ╰╴                              ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_table_set_schema() {
        assert_snapshot!(goto("
create schema foo;
create table t(id int);
alter table t set schema fo$0o;
"), @r"
          ╭▸ 
        2 │ create schema foo;
          │               ─── 2. destination
        3 │ create table t(id int);
        4 │ alter table t set schema foo;
          ╰╴                          ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_table_attach_partition() {
        assert_snapshot!(goto("
create table parent (id int) partition by range (id);
create table child (id int);
alter table parent attach partition ch$0ild for values from (1) to (10);
"), @r"
          ╭▸ 
        3 │ create table child (id int);
          │              ───── 2. destination
        4 │ alter table parent attach partition child for values from (1) to (10);
          ╰╴                                     ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_table_detach_partition() {
        assert_snapshot!(goto("
create table parent (id int) partition by range (id);
create table child partition of parent for values from (1) to (10);
alter table parent detach partition ch$0ild;
"), @r"
          ╭▸ 
        3 │ create table child partition of parent for values from (1) to (10);
          │              ───── 2. destination
        4 │ alter table parent detach partition child;
          ╰╴                                     ─ 1. source
        ");
    }

    #[test]
    fn goto_comment_on_table() {
        assert_snapshot!(goto("
create table t(id int);
comment on table t$0 is '';
"), @r"
          ╭▸ 
        2 │ create table t(id int);
          │              ─ 2. destination
        3 │ comment on table t is '';
          ╰╴                 ─ 1. source
        ");
    }

    #[test]
    fn goto_comment_on_column() {
        assert_snapshot!(goto("
create table t(id int);
comment on column t.id$0 is '';
"), @"
          ╭▸ 
        2 │ create table t(id int);
          │                ── 2. destination
        3 │ comment on column t.id is '';
          ╰╴                     ─ 1. source
        ");
    }

    #[test]
    fn goto_comment_on_column_table_qualifier() {
        assert_snapshot!(goto("
create table t(id int);
comment on column t$0.id is '';
"), @"
          ╭▸ 
        2 │ create table t(id int);
          │              ─ 2. destination
        3 │ comment on column t.id is '';
          ╰╴                  ─ 1. source
        ");
    }

    #[test]
    fn goto_comment_on_column_composite_type_attribute() {
        assert_snapshot!(goto("
create type address as (city text, zip text);
comment on column address.city$0 is 'x';
"), @"
          ╭▸ 
        2 │ create type address as (city text, zip text);
          │                         ──── 2. destination
        3 │ comment on column address.city is 'x';
          ╰╴                             ─ 1. source
        ");
    }

    #[test]
    fn goto_comment_on_view() {
        assert_snapshot!(goto("
create view v as select 1;
comment on view v$0 is '';
"), @"
          ╭▸ 
        2 │ create view v as select 1;
          │             ─ 2. destination
        3 │ comment on view v is '';
          ╰╴                ─ 1. source
        ");
    }

    #[test]
    fn goto_comment_on_materialized_view() {
        assert_snapshot!(goto("
create materialized view mv as select 1;
comment on materialized view mv$0 is '';
"), @"
          ╭▸ 
        2 │ create materialized view mv as select 1;
          │                          ── 2. destination
        3 │ comment on materialized view mv is '';
          ╰╴                              ─ 1. source
        ");
    }

    #[test]
    fn goto_comment_on_sequence() {
        assert_snapshot!(goto("
create sequence s;
comment on sequence s$0 is '';
"), @"
          ╭▸ 
        2 │ create sequence s;
          │                 ─ 2. destination
        3 │ comment on sequence s is '';
          ╰╴                    ─ 1. source
        ");
    }

    #[test]
    fn goto_comment_on_type() {
        assert_snapshot!(goto("
create type t as (a int);
comment on type t$0 is '';
"), @"
          ╭▸ 
        2 │ create type t as (a int);
          │             ─ 2. destination
        3 │ comment on type t is '';
          ╰╴                ─ 1. source
        ");
    }

    #[test]
    fn goto_comment_on_function() {
        assert_snapshot!(goto("
create function f() returns int language sql as 'select 1';
comment on function f$0 is '';
"), @"
          ╭▸ 
        2 │ create function f() returns int language sql as 'select 1';
          │                 ─ 2. destination
        3 │ comment on function f is '';
          ╰╴                    ─ 1. source
        ");
    }

    #[test]
    fn goto_comment_on_index() {
        assert_snapshot!(goto("
create table foo(id int);
create index i on foo(id);
comment on index i$0 is '';
"), @"
          ╭▸ 
        3 │ create index i on foo(id);
          │              ─ 2. destination
        4 │ comment on index i is '';
          ╰╴                 ─ 1. source
        ");
    }

    #[test]
    fn goto_comment_on_trigger() {
        assert_snapshot!(goto("
create table t(a int);
create function f() returns trigger language plpgsql as $$
begin
  return new;
end
$$;
create trigger tr
  before insert on t
  for each row
  execute function f();
comment on trigger tr$0 on t is 'x';
"), @"
           ╭▸ 
         8 │ create trigger tr
           │                ── 2. destination
           ‡
        12 │ comment on trigger tr on t is 'x';
           ╰╴                    ─ 1. source
        ");
    }

    #[test]
    fn goto_comment_on_policy() {
        assert_snapshot!(goto("
create table t(a int);
create policy p on t using (a > 0);
comment on policy p$0 on t is 'x';
"), @"
          ╭▸ 
        3 │ create policy p on t using (a > 0);
          │               ─ 2. destination
        4 │ comment on policy p on t is 'x';
          ╰╴                  ─ 1. source
        ");
    }

    #[test]
    fn goto_comment_on_rule() {
        assert_snapshot!(goto("
create table t(a int);
create rule r as on select to t do instead nothing;
comment on rule r$0 on t is 'x';
"), @"
          ╭▸ 
        3 │ create rule r as on select to t do instead nothing;
          │             ─ 2. destination
        4 │ comment on rule r on t is 'x';
          ╰╴                ─ 1. source
        ");
    }

    #[test]
    fn goto_comment_on_publication() {
        assert_snapshot!(goto("
create publication pub;
comment on publication pub$0 is 'x';
"), @"
          ╭▸ 
        2 │ create publication pub;
          │                    ─── 2. destination
        3 │ comment on publication pub is 'x';
          ╰╴                         ─ 1. source
        ");
    }

    #[test]
    fn goto_comment_on_subscription() {
        assert_snapshot!(goto("
create subscription sub connection $$host=localhost$$ publication pub;
comment on subscription sub$0 is 'x';
"), @"
          ╭▸ 
        2 │ create subscription sub connection $$host=localhost$$ publication pub;
          │                     ─── 2. destination
        3 │ comment on subscription sub is 'x';
          ╰╴                          ─ 1. source
        ");
    }

    #[test]
    fn goto_comment_on_foreign_data_wrapper() {
        assert_snapshot!(goto("
create foreign data wrapper fdw;
comment on foreign data wrapper fdw$0 is 'x';
"), @"
          ╭▸ 
        2 │ create foreign data wrapper fdw;
          │                             ─── 2. destination
        3 │ comment on foreign data wrapper fdw is 'x';
          ╰╴                                  ─ 1. source
        ");
    }

    #[test]
    fn goto_comment_on_language() {
        assert_snapshot!(goto("
create language plfoo;
comment on language plfoo$0 is 'x';
"), @"
          ╭▸ 
        2 │ create language plfoo;
          │                 ───── 2. destination
        3 │ comment on language plfoo is 'x';
          ╰╴                        ─ 1. source
        ");
    }

    #[test]
    fn goto_comment_on_collation() {
        assert_snapshot!(goto("
create collation mycoll (locale = 'C');
comment on collation mycoll$0 is 'x';
"), @"
          ╭▸ 
        2 │ create collation mycoll (locale = 'C');
          │                  ────── 2. destination
        3 │ comment on collation mycoll is 'x';
          ╰╴                          ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_conversion() {
        assert_snapshot!(goto("
create conversion conv for 'UTF8' to 'LATIN1' from utf8_to_latin1;
drop conversion con$0v;
"), @"
          ╭▸ 
        2 │ create conversion conv for 'UTF8' to 'LATIN1' from utf8_to_latin1;
          │                   ──── 2. destination
        3 │ drop conversion conv;
          ╰╴                  ─ 1. source
        ");
    }

    #[test]
    fn goto_comment_on_conversion() {
        assert_snapshot!(goto("
create conversion conv for 'UTF8' to 'LATIN1' from utf8_to_latin1;
comment on conversion con$0v is 'x';
"), @"
          ╭▸ 
        2 │ create conversion conv for 'UTF8' to 'LATIN1' from utf8_to_latin1;
          │                   ──── 2. destination
        3 │ comment on conversion conv is 'x';
          ╰╴                        ─ 1. source
        ");
    }

    #[test]
    fn goto_create_conversion_from_function() {
        assert_snapshot!(goto("
create function my_conv(integer, integer, cstring, internal, integer) returns void language c as $$x$$;
create conversion my_conv_obj for 'UTF8' to 'LATIN1' from my_co$0nv;
"), @"
          ╭▸ 
        2 │ create function my_conv(integer, integer, cstring, internal, integer) returns void language c as $$x$$;
          │                 ─────── 2. destination
        3 │ create conversion my_conv_obj for 'UTF8' to 'LATIN1' from my_conv;
          ╰╴                                                              ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_text_search_dictionary() {
        assert_snapshot!(goto("
create text search dictionary english_stem (template = snowball, language = english);
drop text search dictionary english_st$0em;
"), @"
          ╭▸ 
        2 │ create text search dictionary english_stem (template = snowball, language = english);
          │                               ──────────── 2. destination
        3 │ drop text search dictionary english_stem;
          ╰╴                                     ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_text_search_dictionary() {
        assert_snapshot!(goto("
create text search dictionary english_stem (template = snowball, language = english);
alter text search dictionary english_st$0em rename to stemmer;
"), @"
          ╭▸ 
        2 │ create text search dictionary english_stem (template = snowball, language = english);
          │                               ──────────── 2. destination
        3 │ alter text search dictionary english_stem rename to stemmer;
          ╰╴                                      ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_text_search_configuration() {
        assert_snapshot!(goto("
create text search configuration my_config (parser = pg_catalog.default);
drop text search configuration my_conf$0ig;
"), @"
          ╭▸ 
        2 │ create text search configuration my_config (parser = pg_catalog.default);
          │                                  ───────── 2. destination
        3 │ drop text search configuration my_config;
          ╰╴                                     ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_text_search_configuration() {
        assert_snapshot!(goto("
create text search configuration my_config (parser = pg_catalog.default);
alter text search configuration my_conf$0ig rename to my_config2;
"), @"
          ╭▸ 
        2 │ create text search configuration my_config (parser = pg_catalog.default);
          │                                  ───────── 2. destination
        3 │ alter text search configuration my_config rename to my_config2;
          ╰╴                                      ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_text_search_parser() {
        assert_snapshot!(goto("
create text search parser my_parser (start = prsd_start, gettoken = prsd_nexttoken, end = prsd_end, lextypes = prsd_lextype);
drop text search parser my_pars$0er;
"), @"
          ╭▸ 
        2 │ create text search parser my_parser (start = prsd_start, gettoken = prsd_nexttoken, end = prsd_end, lextypes = prsd_lextype);
          │                           ───────── 2. destination
        3 │ drop text search parser my_parser;
          ╰╴                              ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_text_search_parser() {
        assert_snapshot!(goto("
create text search parser my_parser (start = prsd_start, gettoken = prsd_nexttoken, end = prsd_end, lextypes = prsd_lextype);
alter text search parser my_pars$0er rename to my_parser2;
"), @"
          ╭▸ 
        2 │ create text search parser my_parser (start = prsd_start, gettoken = prsd_nexttoken, end = prsd_end, lextypes = prsd_lextype);
          │                           ───────── 2. destination
        3 │ alter text search parser my_parser rename to my_parser2;
          ╰╴                               ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_text_search_template() {
        assert_snapshot!(goto("
create text search template my_template (init = dsimple_init, lexize = dsimple_lexize);
drop text search template my_temp$0late;
"), @"
          ╭▸ 
        2 │ create text search template my_template (init = dsimple_init, lexize = dsimple_lexize);
          │                             ─────────── 2. destination
        3 │ drop text search template my_template;
          ╰╴                                ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_text_search_template() {
        assert_snapshot!(goto("
create text search template my_template (init = dsimple_init, lexize = dsimple_lexize);
alter text search template my_temp$0late rename to my_template2;
"), @"
          ╭▸ 
        2 │ create text search template my_template (init = dsimple_init, lexize = dsimple_lexize);
          │                             ─────────── 2. destination
        3 │ alter text search template my_template rename to my_template2;
          ╰╴                                 ─ 1. source
        ");
    }

    #[test]
    fn goto_create_text_search_parser_function_option() {
        assert_snapshot!(goto("
create function start_fn(internal, int) returns internal language c as $$x$$;
create text search parser p (start = start_$0fn, gettoken = g, end = e, lextypes = l);
"), @"
          ╭▸ 
        2 │ create function start_fn(internal, int) returns internal language c as $$x$$;
          │                 ──────── 2. destination
        3 │ create text search parser p (start = start_fn, gettoken = g, end = e, lextypes = l);
          ╰╴                                          ─ 1. source
        ");
    }

    #[test]
    fn goto_create_text_search_template_function_option() {
        assert_snapshot!(goto("
create function init_fn(internal) returns internal language c as $$x$$;
create text search template t (init = init_$0fn, lexize = lex_fn);
"), @"
          ╭▸ 
        2 │ create function init_fn(internal) returns internal language c as $$x$$;
          │                 ─────── 2. destination
        3 │ create text search template t (init = init_fn, lexize = lex_fn);
          ╰╴                                          ─ 1. source
        ");
    }

    #[test]
    fn goto_create_text_search_configuration_parser_option() {
        assert_snapshot!(goto("
create text search parser my_parser (start = s, gettoken = g, end = e, lextypes = l);
create text search configuration cfg (parser = my_par$0ser);
"), @"
          ╭▸ 
        2 │ create text search parser my_parser (start = s, gettoken = g, end = e, lextypes = l);
          │                           ───────── 2. destination
        3 │ create text search configuration cfg (parser = my_parser);
          ╰╴                                                    ─ 1. source
        ");
    }

    #[test]
    fn goto_create_text_search_configuration_copy_option() {
        assert_snapshot!(goto("
create text search configuration src (parser = pg_catalog.default);
create text search configuration cfg (copy = sr$0c);
"), @"
          ╭▸ 
        2 │ create text search configuration src (parser = pg_catalog.default);
          │                                  ─── 2. destination
        3 │ create text search configuration cfg (copy = src);
          ╰╴                                              ─ 1. source
        ");
    }

    #[test]
    fn goto_create_text_search_dictionary_template_option() {
        assert_snapshot!(goto("
create text search template my_template (init = i, lexize = l);
create text search dictionary dict (template = my_temp$0late);
"), @"
          ╭▸ 
        2 │ create text search template my_template (init = i, lexize = l);
          │                             ─────────── 2. destination
        3 │ create text search dictionary dict (template = my_template);
          ╰╴                                                     ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_text_search_configuration_add_mapping_dictionary() {
        assert_snapshot!(goto("
create text search dictionary dict (template = pg_catalog.simple);
create text search configuration cfg (parser = pg_catalog.default);
alter text search configuration cfg add mapping for asciiword with dic$0t;
"), @"
          ╭▸ 
        2 │ create text search dictionary dict (template = pg_catalog.simple);
          │                               ──── 2. destination
        3 │ create text search configuration cfg (parser = pg_catalog.default);
        4 │ alter text search configuration cfg add mapping for asciiword with dict;
          ╰╴                                                                     ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_text_search_configuration_alter_mapping_with_dictionary() {
        assert_snapshot!(goto("
create text search dictionary d1 (template = pg_catalog.simple);
create text search configuration cfg (parser = pg_catalog.default);
alter text search configuration cfg alter mapping for asciiword with d$01;
"), @"
          ╭▸ 
        2 │ create text search dictionary d1 (template = pg_catalog.simple);
          │                               ── 2. destination
        3 │ create text search configuration cfg (parser = pg_catalog.default);
        4 │ alter text search configuration cfg alter mapping for asciiword with d1;
          ╰╴                                                                     ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_text_search_configuration_alter_mapping_replace_dictionary() {
        assert_snapshot!(goto("
create text search dictionary d1 (template = pg_catalog.simple);
create text search dictionary d2 (template = pg_catalog.simple);
create text search configuration cfg (parser = pg_catalog.default);
alter text search configuration cfg alter mapping replace d1 with d$02;
"), @"
          ╭▸ 
        3 │ create text search dictionary d2 (template = pg_catalog.simple);
          │                               ── 2. destination
        4 │ create text search configuration cfg (parser = pg_catalog.default);
        5 │ alter text search configuration cfg alter mapping replace d1 with d2;
          ╰╴                                                                  ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_access_method() {
        assert_snapshot!(goto("
create access method heap2 type table handler heap_tableam_handler;
drop access method hea$0p2;
"), @"
          ╭▸ 
        2 │ create access method heap2 type table handler heap_tableam_handler;
          │                      ───── 2. destination
        3 │ drop access method heap2;
          ╰╴                     ─ 1. source
        ");
    }

    #[test]
    fn goto_set_access_method() {
        assert_snapshot!(goto("
create access method heap2 type table handler heap_tableam_handler;
alter table t set access method hea$0p2;
"), @"
          ╭▸ 
        2 │ create access method heap2 type table handler heap_tableam_handler;
          │                      ───── 2. destination
        3 │ alter table t set access method heap2;
          ╰╴                                  ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_operator_family() {
        assert_snapshot!(goto("
create operator family my_family using btree;
drop operator family my_fami$0ly using btree;
"), @"
          ╭▸ 
        2 │ create operator family my_family using btree;
          │                        ───────── 2. destination
        3 │ drop operator family my_family using btree;
          ╰╴                           ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_operator_family() {
        assert_snapshot!(goto("
create operator family my_family using btree;
alter operator family my_fami$0ly using btree owner to someone;
"), @"
          ╭▸ 
        2 │ create operator family my_family using btree;
          │                        ───────── 2. destination
        3 │ alter operator family my_family using btree owner to someone;
          ╰╴                            ─ 1. source
        ");
    }

    #[test]
    fn goto_comment_on_operator_family() {
        assert_snapshot!(goto("
create operator family my_family using btree;
comment on operator family my_fami$0ly using btree is 'hi';
"), @"
          ╭▸ 
        2 │ create operator family my_family using btree;
          │                        ───────── 2. destination
        3 │ comment on operator family my_family using btree is 'hi';
          ╰╴                                 ─ 1. source
        ");
    }

    #[test]
    fn goto_comment_on_operator_class() {
        assert_snapshot!(goto("
create operator class my_opclass for type int using btree as operator 1 < (int, int);
comment on operator class my_opcla$0ss using btree is 'hi';
"), @"
          ╭▸ 
        2 │ create operator class my_opclass for type int using btree as operator 1 < (int, int);
          │                       ────────── 2. destination
        3 │ comment on operator class my_opclass using btree is 'hi';
          ╰╴                                 ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_operator_class() {
        assert_snapshot!(goto("
create operator class my_opclass for type int using btree as operator 1 < (int, int);
drop operator class my_opcla$0ss using btree;
"), @"
          ╭▸ 
        2 │ create operator class my_opclass for type int using btree as operator 1 < (int, int);
          │                       ────────── 2. destination
        3 │ drop operator class my_opclass using btree;
          ╰╴                           ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_operator_class() {
        assert_snapshot!(goto("
create operator class my_opclass for type int using btree as operator 1 < (int, int);
alter operator class my_opcla$0ss using btree owner to someone;
"), @"
          ╭▸ 
        2 │ create operator class my_opclass for type int using btree as operator 1 < (int, int);
          │                       ────────── 2. destination
        3 │ alter operator class my_opclass using btree owner to someone;
          ╰╴                            ─ 1. source
        ");
    }

    #[test]
    fn goto_create_index_using_access_method() {
        assert_snapshot!(goto("
create function my_handler(internal) returns index_am_handler language c as $$x$$;
create access method my_am type index handler my_handler;
create table t(id int);
create index on t using my_a$0m (id);
"), @"
          ╭▸ 
        3 │ create access method my_am type index handler my_handler;
          │                      ───── 2. destination
        4 │ create table t(id int);
        5 │ create index on t using my_am (id);
          ╰╴                           ─ 1. source
        ");
    }

    #[test]
    fn goto_create_table_using_access_method() {
        assert_snapshot!(goto("
create function my_handler(internal) returns table_am_handler language c as $$x$$;
create access method my_am type table handler my_handler;
create table t(id int) using my_a$0m;
"), @"
          ╭▸ 
        3 │ create access method my_am type table handler my_handler;
          │                      ───── 2. destination
        4 │ create table t(id int) using my_am;
          ╰╴                                ─ 1. source
        ");
    }

    #[test]
    fn goto_create_operator_family_using_access_method() {
        assert_snapshot!(goto("
create function my_handler(internal) returns index_am_handler language c as $$x$$;
create access method my_am type index handler my_handler;
create operator family fam using my_a$0m;
"), @"
          ╭▸ 
        3 │ create access method my_am type index handler my_handler;
          │                      ───── 2. destination
        4 │ create operator family fam using my_am;
          ╰╴                                    ─ 1. source
        ");
    }

    #[test]
    fn goto_create_operator_class_using_access_method() {
        assert_snapshot!(goto("
create function my_handler(internal) returns index_am_handler language c as $$x$$;
create access method my_am type index handler my_handler;
create operator class my_opclass for type int using my_a$0m as storage int;
"), @"
          ╭▸ 
        3 │ create access method my_am type index handler my_handler;
          │                      ───── 2. destination
        4 │ create operator class my_opclass for type int using my_am as storage int;
          ╰╴                                                       ─ 1. source
        ");
    }

    #[test]
    fn goto_create_operator_class_family() {
        assert_snapshot!(goto("
create function h(internal) returns index_am_handler language c as $$x$$;
create access method fam type index handler h;
create operator family fam using btree;
create operator class ops for type int using btree family fa$0m as operator 1 <;
"), @"
          ╭▸ 
        4 │ create operator family fam using btree;
          │                        ─── 2. destination
        5 │ create operator class ops for type int using btree family fam as operator 1 <;
          ╰╴                                                           ─ 1. source
        ");
    }

    #[test]
    fn goto_create_operator_class_for_order_by_family() {
        assert_snapshot!(goto("
create operator family sort_fam using btree;
create operator class ops for type int using gist as operator 1 <-> for order by sort_f$0am;
"), @"
          ╭▸ 
        2 │ create operator family sort_fam using btree;
          │                        ──────── 2. destination
        3 │ create operator class ops for type int using gist as operator 1 <-> for order by sort_fam;
          ╰╴                                                                                      ─ 1. source
        ");
    }

    #[test]
    fn goto_insert_on_conflict_operator_class() {
        assert_snapshot!(goto("
create operator class my_ops for type int using btree as operator 1 <;
create table t(a int);
insert into t values (1) on conflict (a my_o$0ps) do nothing;
"), @"
          ╭▸ 
        2 │ create operator class my_ops for type int using btree as operator 1 <;
          │                       ────── 2. destination
        3 │ create table t(a int);
        4 │ insert into t values (1) on conflict (a my_ops) do nothing;
          ╰╴                                           ─ 1. source
        ");
    }

    #[test]
    fn goto_create_index_operator_class() {
        assert_snapshot!(goto("
create operator class public.my_ops for type int using btree as operator 1 <, function 1 btint4cmp(int,int);
create table t(a int);
create index idx on t (a public.my_o$0ps);
"), @"
          ╭▸ 
        2 │ create operator class public.my_ops for type int using btree as operator 1 <, function 1 btint4cmp(int,int);
          │                              ────── 2. destination
        3 │ create table t(a int);
        4 │ create index idx on t (a public.my_ops);
          ╰╴                                   ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_operator_family_using_access_method() {
        assert_snapshot!(goto("
create function my_handler(internal) returns index_am_handler language c as $$x$$;
create access method my_am type index handler my_handler;
create operator family fam using my_am;
alter operator family fam using my_a$0m owner to someone;
"), @"
          ╭▸ 
        3 │ create access method my_am type index handler my_handler;
          │                      ───── 2. destination
        4 │ create operator family fam using my_am;
        5 │ alter operator family fam using my_am owner to someone;
          ╰╴                                   ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_operator_class_using_access_method() {
        assert_snapshot!(goto("
create function my_handler(internal) returns index_am_handler language c as $$x$$;
create access method my_am type index handler my_handler;
create operator class my_opclass for type int using my_am as storage int;
drop operator class my_opclass using my_a$0m;
"), @"
          ╭▸ 
        3 │ create access method my_am type index handler my_handler;
          │                      ───── 2. destination
        4 │ create operator class my_opclass for type int using my_am as storage int;
        5 │ drop operator class my_opclass using my_am;
          ╰╴                                        ─ 1. source
        ");
    }

    #[test]
    fn goto_operator_class_function_option() {
        assert_snapshot!(goto("
create function my_cmp(int, int) returns int language sql as $$select 0$$;
create operator class my_opclass for type int using btree as function 1 my_cm$0p(int, int);
"), @"
          ╭▸ 
        2 │ create function my_cmp(int, int) returns int language sql as $$select 0$$;
          │                 ────── 2. destination
        3 │ create operator class my_opclass for type int using btree as function 1 my_cmp(int, int);
          ╰╴                                                                            ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_operator_class_explicit_schema() {
        assert_snapshot!(goto("
create schema app;
create operator class app.my_ops for type int using btree as storage int;
drop operator class app.my_o$0ps using btree;
"), @"
          ╭▸ 
        3 │ create operator class app.my_ops for type int using btree as storage int;
          │                           ────── 2. destination
        4 │ drop operator class app.my_ops using btree;
          ╰╴                           ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_operator_class_wrong_explicit_schema_not_found() {
        goto_not_found(
            "
create schema app;
create operator class app.my_ops for type int using btree as storage int;
set search_path to app;
drop operator class public.my_o$0ps using btree;
",
        );
    }

    #[test]
    fn goto_drop_collation_explicit_schema() {
        assert_snapshot!(goto(r#"
create schema app;
create collation app.coll (locale = 'C');
drop collation app.co$0ll;
"#), @"
          ╭▸ 
        3 │ create collation app.coll (locale = 'C');
          │                      ──── 2. destination
        4 │ drop collation app.coll;
          ╰╴                    ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_text_search_configuration_explicit_schema() {
        assert_snapshot!(goto("
create schema app;
create text search configuration app.cfg (parser = pg_catalog.default);
drop text search configuration app.c$0fg;
"), @"
          ╭▸ 
        3 │ create text search configuration app.cfg (parser = pg_catalog.default);
          │                                      ─── 2. destination
        4 │ drop text search configuration app.cfg;
          ╰╴                                   ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_text_search_configuration_wrong_explicit_schema_not_found() {
        goto_not_found(
            "
create schema app;
create text search configuration app.cfg (parser = pg_catalog.default);
set search_path to app;
drop text search configuration public.c$0fg;
",
        );
    }

    #[test]
    fn goto_grant_table_explicit_schema() {
        assert_snapshot!(goto("
create schema app;
create table app.t(a int);
grant select on app.t$0 to public;
"), @"
          ╭▸ 
        3 │ create table app.t(a int);
          │                  ─ 2. destination
        4 │ grant select on app.t to public;
          ╰╴                    ─ 1. source
        ");
    }

    #[test]
    fn goto_grant_table_wrong_explicit_schema_not_found() {
        goto_not_found(
            "
create schema app;
create table app.t(a int);
set search_path to app;
grant select on public.t$0 to public;
",
        );
    }

    #[test]
    fn goto_security_label_table() {
        assert_snapshot!(goto("
create table foo(id int);
security label on table foo$0 is 'x';
"), @"
          ╭▸ 
        2 │ create table foo(id int);
          │              ─── 2. destination
        3 │ security label on table foo is 'x';
          ╰╴                          ─ 1. source
        ");
    }

    #[test]
    fn goto_security_label_column() {
        assert_snapshot!(goto("
create table foo(id int);
security label on column foo.id$0 is 'x';
"), @"
          ╭▸ 
        2 │ create table foo(id int);
          │                  ── 2. destination
        3 │ security label on column foo.id is 'x';
          ╰╴                              ─ 1. source
        ");
    }

    #[test]
    fn goto_security_label_column_table_qualifier() {
        assert_snapshot!(goto("
create table foo(id int);
security label on column foo$0.id is 'x';
"), @"
          ╭▸ 
        2 │ create table foo(id int);
          │              ─── 2. destination
        3 │ security label on column foo.id is 'x';
          ╰╴                           ─ 1. source
        ");
    }

    #[test]
    fn goto_security_label_view() {
        assert_snapshot!(goto("
create view v as select 1;
security label on view v$0 is 'x';
"), @"
          ╭▸ 
        2 │ create view v as select 1;
          │             ─ 2. destination
        3 │ security label on view v is 'x';
          ╰╴                       ─ 1. source
        ");
    }

    #[test]
    fn goto_security_label_type() {
        assert_snapshot!(goto("
create type t as (a int);
security label on type t$0 is 'x';
"), @"
          ╭▸ 
        2 │ create type t as (a int);
          │             ─ 2. destination
        3 │ security label on type t is 'x';
          ╰╴                       ─ 1. source
        ");
    }

    #[test]
    fn goto_security_label_function() {
        assert_snapshot!(goto("
create function f() returns int language sql as 'select 1';
security label on function f$0() is 'x';
"), @"
          ╭▸ 
        2 │ create function f() returns int language sql as 'select 1';
          │                 ─ 2. destination
        3 │ security label on function f() is 'x';
          ╰╴                           ─ 1. source
        ");
    }

    #[test]
    fn goto_security_label_provider_unresolved() {
        goto_not_found(
            "
create table foo(id int);
security label for prov$0 on table foo is 'x';
",
        );
    }

    #[test]
    fn goto_refresh_materialized_view() {
        assert_snapshot!(goto("
create materialized view mv as select 1;
refresh materialized view mv$0;
"), @r"
          ╭▸ 
        2 │ create materialized view mv as select 1;
          │                          ── 2. destination
        3 │ refresh materialized view mv;
          ╰╴                           ─ 1. source
        ");
    }

    #[test]
    fn goto_refresh_materialized_view_concurrently() {
        assert_snapshot!(goto("
create materialized view mv as select 1;
refresh materialized view concurrently mv$0;
"), @r"
          ╭▸ 
        2 │ create materialized view mv as select 1;
          │                          ── 2. destination
        3 │ refresh materialized view concurrently mv;
          ╰╴                                        ─ 1. source
        ");
    }

    #[test]
    fn goto_reindex_table() {
        assert_snapshot!(goto("
create table users(id int);
reindex table users$0;
"), @r"
          ╭▸ 
        2 │ create table users(id int);
          │              ───── 2. destination
        3 │ reindex table users;
          ╰╴                  ─ 1. source
        ");
    }

    #[test]
    fn goto_reindex_index() {
        assert_snapshot!(goto("
create table t(c int);
create index idx on t(c);
reindex index idx$0;
"), @r"
          ╭▸ 
        3 │ create index idx on t(c);
          │              ─── 2. destination
        4 │ reindex index idx;
          ╰╴                ─ 1. source
        ");
    }

    #[test]
    fn goto_cluster_table() {
        assert_snapshot!(goto("
create table foo(id int);
cluster foo$0;
"), @"
          ╭▸ 
        2 │ create table foo(id int);
          │              ─── 2. destination
        3 │ cluster foo;
          ╰╴          ─ 1. source
        ");
    }

    #[test]
    fn goto_cluster_using_index() {
        assert_snapshot!(goto("
create table foo(id int);
create index i on foo(id);
cluster foo using i$0;
"), @"
          ╭▸ 
        3 │ create index i on foo(id);
          │              ─ 2. destination
        4 │ cluster foo using i;
          ╰╴                  ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_table_cluster_on() {
        assert_snapshot!(goto("
create table t(a int);
create index idx on t(a);
alter table t cluster on idx$0;
"), @"
          ╭▸ 
        3 │ create index idx on t(a);
          │              ─── 2. destination
        4 │ alter table t cluster on idx;
          ╰╴                           ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_table_replica_identity_using_index() {
        assert_snapshot!(goto("
create table t(a int);
create unique index idx on t(a);
alter table t replica identity using index idx$0;
"), @"
          ╭▸ 
        3 │ create unique index idx on t(a);
          │                     ─── 2. destination
        4 │ alter table t replica identity using index idx;
          ╰╴                                             ─ 1. source
        ");
    }

    #[test]
    fn goto_copy_table() {
        assert_snapshot!(goto("
create table foo (id int);
copy foo$0 to stdout;
"), @"
          ╭▸ 
        2 │ create table foo (id int);
          │              ─── 2. destination
        3 │ copy foo to stdout;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_copy_column() {
        assert_snapshot!(goto("
create table foo (id int);
copy foo (id$0) to stdout;
"), @"
          ╭▸ 
        2 │ create table foo (id int);
          │                   ── 2. destination
        3 │ copy foo (id) to stdout;
          ╰╴           ─ 1. source
        ");
    }

    #[test]
    fn goto_select_exists_column() {
        assert_snapshot!(goto("
select exists$0 from (
  select exists(select 1)
);
"), @r"
          ╭▸ 
        2 │ select exists from (
          │             ─ 1. source
        3 │   select exists(select 1)
          ╰╴         ──────────────── 2. destination
        ");
    }

    #[test]
    fn goto_reindex_schema() {
        assert_snapshot!(goto("
create schema app;
reindex schema app$0;
"), @r"
          ╭▸ 
        2 │ create schema app;
          │               ─── 2. destination
        3 │ reindex schema app;
          ╰╴                 ─ 1. source
        ");
    }

    #[test]
    fn goto_reindex_database() {
        assert_snapshot!(goto("
create database appdb;
reindex database appdb$0;
"), @r"
          ╭▸ 
        2 │ create database appdb;
          │                 ───── 2. destination
        3 │ reindex database appdb;
          ╰╴                     ─ 1. source
        ");
    }

    #[test]
    fn goto_reindex_system() {
        assert_snapshot!(goto("
create database systemdb;
reindex system systemdb$0;
"), @r"
          ╭▸ 
        2 │ create database systemdb;
          │                 ──────── 2. destination
        3 │ reindex system systemdb;
          ╰╴                      ─ 1. source
        ");
    }

    #[test]
    fn goto_merge_returning_aliased_column() {
        assert_snapshot!(goto(
            "
create table t(a int, b int);
with u(x, y) as (
  select 1, 2
),
merged as (
  merge into t
    using u
      on t.a = u.x
  when matched then
    do nothing
  when not matched then
    do nothing
  returning a as x, b as y
)
select x$0 from merged;
",
        ), @r"
           ╭▸ 
        14 │   returning a as x, b as y
           │                  ─ 2. destination
        15 │ )
        16 │ select x from merged;
           ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_update_returning_column() {
        assert_snapshot!(goto("
create table t(a int, b int);
with updated(c) as (
  update t set a = 10
  returning a, b
)
select c, b$0 from updated;"
        ), @r"
          ╭▸ 
        5 │   returning a, b
          │                ─ 2. destination
        6 │ )
        7 │ select c, b from updated;
          ╰╴          ─ 1. source
        ");
    }

    #[test]
    fn goto_update_returning_column_to_table_def() {
        assert_snapshot!(goto("
create table t(a int, b int);
with updated(c) as (
  update t set a = 10
  returning a, b$0
)
select c, b from updated;"
        ), @r"
          ╭▸ 
        2 │ create table t(a int, b int);
          │                       ─ 2. destination
          ‡
        5 │   returning a, b
          ╰╴               ─ 1. source
        ");
    }

    #[test]
    fn goto_insert_returning_column_to_table_def() {
        assert_snapshot!(goto("
create table t(a int, b int);
with inserted as (
  insert into t values (1, 2)
  returning a$0, b
)
select a, b from inserted;"
        ), @r"
          ╭▸ 
        2 │ create table t(a int, b int);
          │                ─ 2. destination
          ‡
        5 │   returning a, b
          ╰╴            ─ 1. source
        ");
    }

    #[test]
    fn goto_delete_returning_column_to_table_def() {
        assert_snapshot!(goto("
create table t(a int, b int);
with deleted as (
  delete from t
  returning a, b$0
)
select a, b from deleted;"
        ), @r"
          ╭▸ 
        2 │ create table t(a int, b int);
          │                       ─ 2. destination
          ‡
        5 │   returning a, b
          ╰╴               ─ 1. source
        ");
    }

    #[test]
    fn goto_update_returning_qualified_star_table() {
        assert_snapshot!(goto("
create table t(a int, b int);
update t set a = 10
returning t$0.*;"
        ), @r"
          ╭▸ 
        2 │ create table t(a int, b int);
          │              ─ 2. destination
        3 │ update t set a = 10
        4 │ returning t.*;
          ╰╴          ─ 1. source
        ");
    }

    #[test]
    fn goto_insert_returning_qualified_star_table() {
        assert_snapshot!(goto("
create table t(a int, b int);
insert into t values (1, 2)
returning t$0.*;"
        ), @r"
          ╭▸ 
        2 │ create table t(a int, b int);
          │              ─ 2. destination
        3 │ insert into t values (1, 2)
        4 │ returning t.*;
          ╰╴          ─ 1. source
        ");
    }

    #[test]
    fn goto_delete_returning_qualified_star_table() {
        assert_snapshot!(goto("
create table t(a int, b int);
delete from t
returning t$0.*;"
        ), @r"
          ╭▸ 
        2 │ create table t(a int, b int);
          │              ─ 2. destination
        3 │ delete from t
        4 │ returning t.*;
          ╰╴          ─ 1. source
        ");
    }

    #[test]
    fn goto_update_alias_in_set_clause() {
        assert_snapshot!(goto("
create table t(a int, b int);
update t as f set f$0.a = 10;"
        ), @r"
          ╭▸ 
        3 │ update t as f set f.a = 10;
          │             ┬     ─ 1. source
          │             │
          ╰╴            2. destination
        ");
    }

    #[test]
    fn goto_update_alias_in_where_clause() {
        assert_snapshot!(goto("
create table t(a int, b int);
update t as f set a = 10 where f$0.b = 5;"
        ), @r"
          ╭▸ 
        3 │ update t as f set a = 10 where f.b = 5;
          ╰╴            ─ 2. destination   ─ 1. source
        ");
    }

    #[test]
    fn goto_update_alias_in_from_clause() {
        assert_snapshot!(goto("
create table t(a int, b int);
create table u(c int);
update t as f set a = 10 from u where f$0.b = u.c;"
        ), @r"
          ╭▸ 
        4 │ update t as f set a = 10 from u where f.b = u.c;
          ╰╴            ─ 2. destination          ─ 1. source
        ");
    }

    #[test]
    fn goto_insert_alias_in_on_conflict() {
        assert_snapshot!(goto("
create table t(a int primary key, b int);
insert into t as f values (1, 2) on conflict (f$0.a) do nothing;"
        ), @r"
          ╭▸ 
        3 │ insert into t as f values (1, 2) on conflict (f.a) do nothing;
          ╰╴                 ─ 2. destination             ─ 1. source
        ");
    }

    #[test]
    fn goto_insert_alias_in_returning() {
        assert_snapshot!(goto("
create table t(a int, b int);
insert into t as f values (1, 2) returning f$0.a;"
        ), @r"
          ╭▸ 
        3 │ insert into t as f values (1, 2) returning f.a;
          ╰╴                 ─ 2. destination          ─ 1. source
        ");
    }

    #[test]
    fn goto_insert_alias_returning_column() {
        assert_snapshot!(goto("
create table t(a int, b int);
insert into t as f values (1, 2) returning f.a$0;"
        ), @r"
          ╭▸ 
        2 │ create table t(a int, b int);
          │                ─ 2. destination
        3 │ insert into t as f values (1, 2) returning f.a;
          ╰╴                                             ─ 1. source
        ");
    }

    #[test]
    fn goto_insert_on_conflict_target_column() {
        assert_snapshot!(goto("
create table t(c text);
insert into t values ('c') on conflict (c$0) do nothing;"
        ), @r"
          ╭▸ 
        2 │ create table t(c text);
          │                ─ 2. destination
        3 │ insert into t values ('c') on conflict (c) do nothing;
          ╰╴                                        ─ 1. source
        ");
    }

    #[test]
    fn goto_insert_on_conflict_set_column() {
        assert_snapshot!(goto("
create table t(c text, d text);
insert into t values ('c', 'd') on conflict (c) do update set c$0 = excluded.c;"
        ), @r"
          ╭▸ 
        2 │ create table t(c text, d text);
          │                ─ 2. destination
        3 │ insert into t values ('c', 'd') on conflict (c) do update set c = excluded.c;
          ╰╴                                                              ─ 1. source
        ");
    }

    #[test]
    fn goto_insert_on_conflict_excluded_column() {
        assert_snapshot!(goto("
create table t(c text, d text);
insert into t values ('c', 'd') on conflict (c) do update set c = excluded.c$0;"
        ), @r"
          ╭▸ 
        2 │ create table t(c text, d text);
          │                ─ 2. destination
        3 │ insert into t values ('c', 'd') on conflict (c) do update set c = excluded.c;
          ╰╴                                                                           ─ 1. source
        ");
    }

    #[test]
    fn goto_insert_on_conflict_qualified_function() {
        assert_snapshot!(goto("
create function foo.lower(text) returns text
  language internal;
create table t(c text);
insert into t values ('c')
  on conflict (foo.lower$0(c))
    do nothing;"
        ), @r"
          ╭▸ 
        2 │ create function foo.lower(text) returns text
          │                     ───── 2. destination
          ‡
        6 │   on conflict (foo.lower(c))
          ╰╴                       ─ 1. source
        ");
    }

    #[test]
    fn goto_delete_from_alias() {
        assert_snapshot!(goto("
create table t(a int, b int);
delete from t as f where f$0.a = 10;"
        ), @r"
          ╭▸ 
        3 │ delete from t as f where f.a = 10;
          │                  ┬       ─ 1. source
          │                  │
          ╰╴                 2. destination
        ");
    }

    #[test]
    fn goto_delete_from_alias_column() {
        assert_snapshot!(goto("
create table t(a int, b int);
delete from t as f where f.a$0 = 10;"
        ), @r"
          ╭▸ 
        2 │ create table t(a int, b int);
          │                ─ 2. destination
        3 │ delete from t as f where f.a = 10;
          ╰╴                           ─ 1. source
        ");
    }

    #[test]
    fn goto_delete_from_alias_returning() {
        assert_snapshot!(goto("
create table t(a int, b int);
delete from t as f returning f$0.a"
        ), @r"
          ╭▸ 
        3 │ delete from t as f returning f.a
          │                  ┬           ─ 1. source
          │                  │
          ╰╴                 2. destination
        ");

        assert_snapshot!(goto("
create table t(a int, b int);
delete from t as f returning f.a$0"
        ), @r"
          ╭▸ 
        2 │ create table t(a int, b int);
          │                ─ 2. destination
        3 │ delete from t as f returning f.a
          ╰╴                               ─ 1. source
        ");
    }

    #[test]
    fn goto_merge_alias_on_table() {
        assert_snapshot!(goto("
create table t(a int, b int);
create table u(a int, b int);
merge into t as f
  using u on u.a = f$0.a
  when matched then do nothing;
"

        ), @r"
          ╭▸ 
        4 │ merge into t as f
          │                 ─ 2. destination
        5 │   using u on u.a = f.a
          ╰╴                   ─ 1. source
        ");
    }

    #[test]
    fn goto_merge_alias_on_column() {
        assert_snapshot!(goto("
create table t(a int, b int);
create table u(a int, b int);
merge into t as f
  using u on u.a = f.a$0
  when matched then do nothing;
"

        ), @r"
          ╭▸ 
        2 │ create table t(a int, b int);
          │                ─ 2. destination
          ‡
        5 │   using u on u.a = f.a
          ╰╴                     ─ 1. source
        ");
    }

    #[test]
    fn goto_merge_alias_returning() {
        assert_snapshot!(goto("
create table t(a int, b int);
create table u(a int, b int);
merge into t as f
  using u on u.a = f.a
  when matched then do nothing
  returning f$0.a;
"

        ), @r"
          ╭▸ 
        4 │ merge into t as f
          │                 ─ 2. destination
          ‡
        7 │   returning f.a;
          ╰╴            ─ 1. source
        ");

        assert_snapshot!(goto("
create table t(a int, b int);
create table u(a int, b int);
merge into t as f
  using u on u.a = f.a
  when matched then do nothing
  returning f.a$0;
"
        ), @r"
          ╭▸ 
        2 │ create table t(a int, b int);
          │                ─ 2. destination
          ‡
        7 │   returning f.a;
          ╰╴              ─ 1. source
        ");
    }

    #[test]
    fn goto_merge_using_table_in_when_clause() {
        assert_snapshot!(goto("
create table t(a int, b int);
create table u(a int, b int);
merge into t
  using u on true
  when matched and u$0.a = t.a
    then do nothing;
"
        ), @r"
          ╭▸ 
        3 │ create table u(a int, b int);
          │              ─ 2. destination
          ‡
        6 │   when matched and u.a = t.a
          ╰╴                   ─ 1. source
        ");
    }

    #[test]
    fn goto_merge_using_table_column_in_when_clause() {
        assert_snapshot!(goto("
create table t(a int, b int);
create table u(a int, b int);
merge into t
  using u on true
  when matched and u.a$0 = t.a
    then do nothing;
"
        ), @r"
          ╭▸ 
        3 │ create table u(a int, b int);
          │                ─ 2. destination
          ‡
        6 │   when matched and u.a = t.a
          ╰╴                     ─ 1. source
        ");
    }

    #[test]
    fn goto_merge_unqualified_column_target_table() {
        assert_snapshot!(goto("
create table x(a int, b int);
create table y(c int, d int);
merge into x
  using y
    on true
  when matched and a$0 = c
    then do nothing;
"
        ), @r"
          ╭▸ 
        2 │ create table x(a int, b int);
          │                ─ 2. destination
          ‡
        7 │   when matched and a = c
          ╰╴                   ─ 1. source
        ");
    }

    #[test]
    fn goto_merge_unqualified_column_source_table() {
        assert_snapshot!(goto("
create table x(a int, b int);
create table y(c int, d int);
merge into x
  using y
    on true
  when matched and a = c$0
    then do nothing;
"
        ), @r"
          ╭▸ 
        3 │ create table y(c int, d int);
          │                ─ 2. destination
          ‡
        7 │   when matched and a = c
          ╰╴                       ─ 1. source
        ");
    }

    #[test]
    fn goto_merge_update_set_target_column() {
        assert_snapshot!(goto("
create table target(id int, val int);
create table source(id int, val int);
merge into target using source on target.id = source.id
  when matched then update set val$0 = source.val;
"
        ), @"
          ╭▸ 
        2 │ create table target(id int, val int);
          │                             ─── 2. destination
          ‡
        5 │   when matched then update set val = source.val;
          ╰╴                                 ─ 1. source
        ");
    }

    #[test]
    fn goto_merge_update_set_source_expr_column() {
        assert_snapshot!(goto("
create table target(id int, val int);
create table source(id int, val int);
merge into target using source on target.id = source.id
  when matched then update set val = source.val$0;
"
        ), @"
          ╭▸ 
        3 │ create table source(id int, val int);
          │                             ─── 2. destination
        4 │ merge into target using source on target.id = source.id
        5 │   when matched then update set val = source.val;
          ╰╴                                              ─ 1. source
        ");
    }

    #[test]
    fn goto_merge_insert_column_list_target_column() {
        assert_snapshot!(goto("
create table target(id int, val int);
create table source(id int, val int);
merge into target using source on target.id = source.id
  when not matched then insert (val$0) values(source.val);
"
        ), @"
          ╭▸ 
        2 │ create table target(id int, val int);
          │                             ─── 2. destination
          ‡
        5 │   when not matched then insert (val) values(source.val);
          ╰╴                                  ─ 1. source
        ");
    }

    #[test]
    fn goto_merge_into_table() {
        assert_snapshot!(goto("
create table x(a int, b int);
create table y(c int, d int);
merge into x$0
  using y
    on true
  when matched and a = c
    then do nothing;
"
        ), @r"
          ╭▸ 
        2 │ create table x(a int, b int);
          │              ─ 2. destination
        3 │ create table y(c int, d int);
        4 │ merge into x
          ╰╴           ─ 1. source
        ");
    }

    #[test]
    fn goto_merge_using_clause_table() {
        assert_snapshot!(goto("
create table x(a int, b int);
create table y(c int, d int);
merge into x
  using y$0
    on true
  when matched and a = c
    then do nothing;
"
        ), @r"
          ╭▸ 
        3 │ create table y(c int, d int);
          │              ─ 2. destination
        4 │ merge into x
        5 │   using y
          ╰╴        ─ 1. source
        ");
    }

    #[test]
    fn goto_merge_using_clause_alias() {
        assert_snapshot!(goto("
create table x(a int, b int);
create table y(c int, d int);
merge into x as g
  using y as k
    on true
  when matched and a = k.c$0
    then do nothing;
"
        ), @r"
          ╭▸ 
        3 │ create table y(c int, d int);
          │                ─ 2. destination
          ‡
        7 │   when matched and a = k.c
          ╰╴                         ─ 1. source
        ");
    }

    #[test]
    fn goto_merge_on_clause_unqualified_source_column() {
        assert_snapshot!(goto("
create table x(a int, b int);
create table y(c int, d int);
merge into x as g
  using y as k
    on g.a = c$0 and a = c
  when matched and g.a = k.c
    then do nothing;
"
        ), @r"
          ╭▸ 
        3 │ create table y(c int, d int);
          │                ─ 2. destination
          ‡
        6 │     on g.a = c and a = c
          ╰╴             ─ 1. source
        ");
    }

    #[test]
    fn goto_merge_returning_old_table() {
        assert_snapshot!(goto("
create table x(a int, b int);
create table y(c int, d int);
merge into x as g
  using y as k
    on g.a = c and a = k.c
  when matched and g.a = k.c
    then do nothing
  returning old$0.a, new.a;
"
        ), @r"
          ╭▸ 
        2 │ create table x(a int, b int);
          │              ─ 2. destination
          ‡
        9 │   returning old.a, new.a;
          ╰╴              ─ 1. source
        ");
    }

    #[test]
    fn goto_merge_returning_old_column() {
        assert_snapshot!(goto("
create table x(a int, b int);
create table y(c int, d int);
merge into x as g
  using y as k
    on g.a = c and a = k.c
  when matched and g.a = k.c
    then do nothing
  returning old.a$0, new.a;
"
        ), @r"
          ╭▸ 
        2 │ create table x(a int, b int);
          │                ─ 2. destination
          ‡
        9 │   returning old.a, new.a;
          ╰╴                ─ 1. source
        ");
    }

    #[test]
    fn goto_merge_returning_new_table() {
        assert_snapshot!(goto("
create table x(a int, b int);
create table y(c int, d int);
merge into x as g
  using y as k
    on g.a = c and a = k.c
  when matched and g.a = k.c
    then do nothing
  returning old.a, new$0.a;
"
        ), @r"
          ╭▸ 
        2 │ create table x(a int, b int);
          │              ─ 2. destination
          ‡
        9 │   returning old.a, new.a;
          ╰╴                     ─ 1. source
        ");
    }

    #[test]
    fn goto_merge_returning_new_column() {
        assert_snapshot!(goto("
create table x(a int, b int);
create table y(c int, d int);
merge into x as g
  using y as k
    on g.a = c and a = k.c
  when matched and g.a = k.c
    then do nothing
  returning old.a, new.a$0;
"
        ), @r"
          ╭▸ 
        2 │ create table x(a int, b int);
          │                ─ 2. destination
          ‡
        9 │   returning old.a, new.a;
          ╰╴                       ─ 1. source
        ");
    }

    #[test]
    fn goto_merge_with_tables_named_old_new_old_table() {
        assert_snapshot!(goto("
create table old(a int, b int);
create table new(c int, d int);
merge into old
  using new
    on true
  when matched
    then do nothing
  returning old$0.a, new.d;
"
        ), @r"
          ╭▸ 
        2 │ create table old(a int, b int);
          │              ─── 2. destination
          ‡
        9 │   returning old.a, new.d;
          ╰╴              ─ 1. source
        ");
    }

    #[test]
    fn goto_merge_with_tables_named_old_new_old_column() {
        assert_snapshot!(goto("
create table old(a int, b int);
create table new(c int, d int);
merge into old
  using new
    on true
  when matched
    then do nothing
  returning old.a$0, new.d;
"
        ), @r"
          ╭▸ 
        2 │ create table old(a int, b int);
          │                  ─ 2. destination
          ‡
        9 │   returning old.a, new.d;
          ╰╴                ─ 1. source
        ");
    }

    #[test]
    fn goto_merge_with_tables_named_old_new_new_table() {
        assert_snapshot!(goto("
create table old(a int, b int);
create table new(c int, d int);
merge into old
  using new
    on true
  when matched
    then do nothing
  returning old.a, new$0.d;
"
        ), @r"
          ╭▸ 
        3 │ create table new(c int, d int);
          │              ─── 2. destination
          ‡
        9 │   returning old.a, new.d;
          ╰╴                     ─ 1. source
        ");
    }

    #[test]
    fn goto_merge_with_tables_named_old_new_new_column() {
        assert_snapshot!(goto("
create table old(a int, b int);
create table new(c int, d int);
merge into old
  using new
    on true
  when matched
    then do nothing
  returning old.a, new.d$0;
"
        ), @r"
          ╭▸ 
        3 │ create table new(c int, d int);
          │                         ─ 2. destination
          ‡
        9 │   returning old.a, new.d;
          ╰╴                       ─ 1. source
        ");
    }

    #[test]
    fn goto_merge_returning_with_aliases_before_table() {
        assert_snapshot!(goto("
create table x(a int, b int);
create table y(c int, d int);
merge into x
  using y on true
  when matched then do nothing
  returning
    with (old as before, new as after)
      before$0.a, after.a;
"
        ), @r"
          ╭▸ 
        8 │     with (old as before, new as after)
          │                  ────── 2. destination
        9 │       before.a, after.a;
          ╰╴           ─ 1. source
        ");
    }

    #[test]
    fn goto_merge_returning_with_aliases_before_column() {
        assert_snapshot!(goto("
create table x(a int, b int);
create table y(c int, d int);
merge into x
  using y on true
  when matched then do nothing
  returning
    with (old as before, new as after)
      before.a$0, after.a;
"
        ), @r"
          ╭▸ 
        2 │ create table x(a int, b int);
          │                ─ 2. destination
          ‡
        9 │       before.a, after.a;
          ╰╴             ─ 1. source
        ");
    }

    #[test]
    fn goto_merge_returning_with_aliases_after_table() {
        assert_snapshot!(goto("
create table x(a int, b int);
create table y(c int, d int);
merge into x
  using y on true
  when matched then do nothing
  returning
    with (old as before, new as after)
      before.a, after$0.a;
"
        ), @r"
          ╭▸ 
        8 │     with (old as before, new as after)
          │                                 ───── 2. destination
        9 │       before.a, after.a;
          ╰╴                    ─ 1. source
        ");
    }

    #[test]
    fn goto_merge_returning_with_aliases_after_column() {
        assert_snapshot!(goto("
create table x(a int, b int);
create table y(c int, d int);
merge into x
  using y on true
  when matched then do nothing
  returning
    with (old as before, new as after)
      before.a, after.a$0;
"
        ), @r"
          ╭▸ 
        2 │ create table x(a int, b int);
          │                ─ 2. destination
          ‡
        9 │       before.a, after.a;
          ╰╴                      ─ 1. source
        ");
    }

    #[test]
    fn goto_merge_when_not_matched_insert_values_qualified_column() {
        assert_snapshot!(goto("
create table inventory (
    product_id int,
    quantity int,
    updated_at timestamp
);
create table orders (
    id int,
    product_id int,
    qty int
);
merge into inventory as t
using orders as o
  on t.product_id = o.product_id
when matched then
  do nothing
when not matched then
  insert values (o$0.product_id, o.qty, now());
"
        ), @r"
           ╭▸ 
        13 │ using orders as o
           │                 ─ 2. destination
           ‡
        18 │   insert values (o.product_id, o.qty, now());
           ╰╴                 ─ 1. source
        ");
    }

    #[test]
    fn goto_merge_when_not_matched_insert_values_qualified_column_field() {
        assert_snapshot!(goto("
create table inventory (
    product_id int,
    quantity int,
    updated_at timestamp
);
create table orders (
    id int,
    product_id int,
    qty int
);
merge into inventory as t
using orders as o
  on t.product_id = o.product_id
when matched then
  do nothing
when not matched then
  insert values (o.product_id$0, o.qty, now());
"
        ), @r"
           ╭▸ 
         9 │     product_id int,
           │     ────────── 2. destination
           ‡
        18 │   insert values (o.product_id, o.qty, now());
           ╰╴                            ─ 1. source
        ");
    }

    #[test]
    fn goto_merge_when_not_matched_insert_values_unqualified_column() {
        assert_snapshot!(goto("
create table inventory (
    product_id int,
    quantity int
);
create table orders (
    product_id int,
    qty int
);
merge into inventory as t
using orders as o
  on t.product_id = o.product_id
when not matched then
  insert values (product_id$0, qty);
"
        ), @r"
           ╭▸ 
         7 │     product_id int,
           │     ────────── 2. destination
           ‡
        14 │   insert values (product_id, qty);
           ╰╴                          ─ 1. source
        ");
    }

    #[test]
    fn goto_insert_returning_old_table() {
        assert_snapshot!(goto("
create table t(a int, b int);
insert into t values (1, 2), (3, 4)
returning old$0.a, new.b;
"
        ), @r"
          ╭▸ 
        2 │ create table t(a int, b int);
          │              ─ 2. destination
        3 │ insert into t values (1, 2), (3, 4)
        4 │ returning old.a, new.b;
          ╰╴            ─ 1. source
        ");
    }

    #[test]
    fn goto_insert_returning_old_column() {
        assert_snapshot!(goto("
create table t(a int, b int);
insert into t values (1, 2), (3, 4)
returning old.a$0, new.b;
"
        ), @r"
          ╭▸ 
        2 │ create table t(a int, b int);
          │                ─ 2. destination
        3 │ insert into t values (1, 2), (3, 4)
        4 │ returning old.a, new.b;
          ╰╴              ─ 1. source
        ");
    }

    #[test]
    fn goto_insert_returning_new_table() {
        assert_snapshot!(goto("
create table t(a int, b int);
insert into t values (1, 2), (3, 4)
returning old.a, new$0.b;
"
        ), @r"
          ╭▸ 
        2 │ create table t(a int, b int);
          │              ─ 2. destination
        3 │ insert into t values (1, 2), (3, 4)
        4 │ returning old.a, new.b;
          ╰╴                   ─ 1. source
        ");
    }

    #[test]
    fn goto_insert_returning_new_column() {
        assert_snapshot!(goto("
create table t(a int, b int);
insert into t values (1, 2), (3, 4)
returning old.a, new.b$0;
"
        ), @r"
          ╭▸ 
        2 │ create table t(a int, b int);
          │                       ─ 2. destination
        3 │ insert into t values (1, 2), (3, 4)
        4 │ returning old.a, new.b;
          ╰╴                     ─ 1. source
        ");
    }

    #[test]
    fn goto_update_returning_old_table() {
        assert_snapshot!(goto("
create table t(a int, b int);
update t set a = 42
returning old$0.a, new.b;
"
        ), @r"
          ╭▸ 
        2 │ create table t(a int, b int);
          │              ─ 2. destination
        3 │ update t set a = 42
        4 │ returning old.a, new.b;
          ╰╴            ─ 1. source
        ");
    }

    #[test]
    fn goto_update_returning_old_column() {
        assert_snapshot!(goto("
create table t(a int, b int);
update t set a = 42
returning old.a$0, new.b;
"
        ), @r"
          ╭▸ 
        2 │ create table t(a int, b int);
          │                ─ 2. destination
        3 │ update t set a = 42
        4 │ returning old.a, new.b;
          ╰╴              ─ 1. source
        ");
    }

    #[test]
    fn goto_update_returning_new_table() {
        assert_snapshot!(goto("
create table t(a int, b int);
update t set a = 42
returning old.a, new$0.b;
"
        ), @r"
          ╭▸ 
        2 │ create table t(a int, b int);
          │              ─ 2. destination
        3 │ update t set a = 42
        4 │ returning old.a, new.b;
          ╰╴                   ─ 1. source
        ");
    }

    #[test]
    fn goto_update_returning_new_column() {
        assert_snapshot!(goto("
create table t(a int, b int);
update t set a = 42
returning old.a, new.b$0;
"
        ), @r"
          ╭▸ 
        2 │ create table t(a int, b int);
          │                       ─ 2. destination
        3 │ update t set a = 42
        4 │ returning old.a, new.b;
          ╰╴                     ─ 1. source
        ");
    }

    #[test]
    fn goto_delete_returning_old_table() {
        assert_snapshot!(goto("
create table t(a int, b int);
delete from t
returning old$0.a, new.b;
"
        ), @r"
          ╭▸ 
        2 │ create table t(a int, b int);
          │              ─ 2. destination
        3 │ delete from t
        4 │ returning old.a, new.b;
          ╰╴            ─ 1. source
        ");
    }

    #[test]
    fn goto_delete_returning_old_column() {
        assert_snapshot!(goto("
create table t(a int, b int);
delete from t
returning old.a$0, new.b;
"
        ), @r"
          ╭▸ 
        2 │ create table t(a int, b int);
          │                ─ 2. destination
        3 │ delete from t
        4 │ returning old.a, new.b;
          ╰╴              ─ 1. source
        ");
    }

    #[test]
    fn goto_delete_returning_new_table() {
        assert_snapshot!(goto("
create table t(a int, b int);
delete from t
returning old.a, new$0.b;
"
        ), @r"
          ╭▸ 
        2 │ create table t(a int, b int);
          │              ─ 2. destination
        3 │ delete from t
        4 │ returning old.a, new.b;
          ╰╴                   ─ 1. source
        ");
    }

    #[test]
    fn goto_delete_returning_new_column() {
        assert_snapshot!(goto("
create table t(a int, b int);
delete from t
returning old.a, new.b$0;
"
        ), @r"
          ╭▸ 
        2 │ create table t(a int, b int);
          │                       ─ 2. destination
        3 │ delete from t
        4 │ returning old.a, new.b;
          ╰╴                     ─ 1. source
        ");
    }

    #[test]
    fn goto_insert_as_old_alias() {
        assert_snapshot!(goto("
create table t(a int, b int);
insert into t as old values (1, 2)
returning old$0.a, new.a;
"
        ), @r"
          ╭▸ 
        3 │ insert into t as old values (1, 2)
          │                  ─── 2. destination
        4 │ returning old.a, new.a;
          ╰╴            ─ 1. source
        ");
    }

    #[test]
    fn goto_delete_as_old_alias() {
        assert_snapshot!(goto("
create table t(a int, b int);
delete from t as old
returning old$0.a, new.a;
"
        ), @r"
          ╭▸ 
        3 │ delete from t as old
          │                  ─── 2. destination
        4 │ returning old.a, new.a;
          ╰╴            ─ 1. source
        ");
    }

    #[test]
    fn goto_update_as_old_alias() {
        assert_snapshot!(goto("
create table t(a int, b int);
update t as old set a = 42
returning old$0.a, new.a;
"
        ), @r"
          ╭▸ 
        3 │ update t as old set a = 42
          │             ─── 2. destination
        4 │ returning old.a, new.a;
          ╰╴            ─ 1. source
        ");
    }

    #[test]
    fn goto_merge_returning_cte_column_unqualified() {
        assert_snapshot!(goto("
create table t(a int, b int);
with u(x, y) as (
  select 1, 2
)
merge into t
  using u on true
when matched then
  do nothing
when not matched then
  do nothing
returning x$0, u.y;
"
        ), @r"
           ╭▸ 
         3 │ with u(x, y) as (
           │        ─ 2. destination
           ‡
        12 │ returning x, u.y;
           ╰╴          ─ 1. source
        ");
    }

    #[test]
    fn goto_merge_returning_cte_column_qualified_table() {
        assert_snapshot!(goto("
create table t(a int, b int);
with u(x, y) as (
  select 1, 2
)
merge into t
  using u on true
when matched then
  do nothing
when not matched then
  do nothing
returning x, u$0.y;
"
        ), @r"
           ╭▸ 
         3 │ with u(x, y) as (
           │      ─ 2. destination
           ‡
        12 │ returning x, u.y;
           ╰╴             ─ 1. source
        ");
    }

    #[test]
    fn goto_merge_returning_cte_column_qualified_column() {
        assert_snapshot!(goto("
create table t(a int, b int);
with u(x, y) as (
  select 1, 2
)
merge into t
  using u on true
when matched then
  do nothing
when not matched then
  do nothing
returning x, u.y$0;
"
        ), @r"
           ╭▸ 
         3 │ with u(x, y) as (
           │           ─ 2. destination
           ‡
        12 │ returning x, u.y;
           ╰╴               ─ 1. source
        ");
    }

    #[test]
    fn goto_overlay_with_cte_column() {
        assert_snapshot!(goto("
with t as (
  select '1' a, '2' b, 3 start
)
select overlay(a placing b$0 from start) from t;
        "), @r"
          ╭▸ 
        3 │   select '1' a, '2' b, 3 start
          │                     ─ 2. destination
        4 │ )
        5 │ select overlay(a placing b from start) from t;
          ╰╴                         ─ 1. source
        ");
    }

    #[test]
    fn goto_overlay_with_cte_column_first_arg() {
        assert_snapshot!(goto("
with t as (
  select '1' a, '2' b, 3 start
)
select overlay(a$0 placing b from start) from t;
        "), @r"
          ╭▸ 
        3 │   select '1' a, '2' b, 3 start
          │              ─ 2. destination
        4 │ )
        5 │ select overlay(a placing b from start) from t;
          ╰╴               ─ 1. source
        ");
    }

    #[test]
    fn goto_overlay_with_cte_column_from_arg() {
        assert_snapshot!(goto("
with t as (
  select '1' a, '2' b, 3 start
)
select overlay(a placing b from start$0) from t;
        "), @r"
          ╭▸ 
        3 │   select '1' a, '2' b, 3 start
          │                          ───── 2. destination
        4 │ )
        5 │ select overlay(a placing b from start) from t;
          ╰╴                                    ─ 1. source
        ");
    }

    #[test]
    fn goto_named_arg_to_param() {
        assert_snapshot!(goto("
create function foo(bar_param int) returns int as 'select 1' language sql;
select foo(bar_param$0 := 5);
"), @r"
          ╭▸ 
        2 │ create function foo(bar_param int) returns int as 'select 1' language sql;
          │                     ───────── 2. destination
        3 │ select foo(bar_param := 5);
          ╰╴                   ─ 1. source
        ");
    }

    #[test]
    fn goto_named_arg_schema_qualified() {
        assert_snapshot!(goto("
create schema s;
create function s.foo(my_param int) returns int as 'select 1' language sql;
select s.foo(my_param$0 := 10);
"), @r"
          ╭▸ 
        3 │ create function s.foo(my_param int) returns int as 'select 1' language sql;
          │                       ──────── 2. destination
        4 │ select s.foo(my_param := 10);
          ╰╴                    ─ 1. source
        ");
    }

    #[test]
    fn goto_named_arg_multiple_params() {
        assert_snapshot!(goto("
create function foo(a int, b int, c int) returns int as 'select 1' language sql;
select foo(b$0 := 2, a := 1);
"), @r"
          ╭▸ 
        2 │ create function foo(a int, b int, c int) returns int as 'select 1' language sql;
          │                            ─ 2. destination
        3 │ select foo(b := 2, a := 1);
          ╰╴           ─ 1. source
        ");
    }

    #[test]
    fn goto_named_arg_procedure() {
        assert_snapshot!(goto("
create procedure proc(param_x int) as 'select 1' language sql;
call proc(param_x$0 := 42);
"), @r"
          ╭▸ 
        2 │ create procedure proc(param_x int) as 'select 1' language sql;
          │                       ─────── 2. destination
        3 │ call proc(param_x := 42);
          ╰╴                ─ 1. source
        ");
    }

    #[test]
    fn goto_named_arg_not_found_unnamed_param() {
        goto_not_found(
            "
create function foo(int) returns int as 'select 1' language sql;
select foo(bar$0 := 5);
",
        );
    }

    #[test]
    fn goto_named_arg_not_found_wrong_name() {
        goto_not_found(
            "
create function foo(correct_param int) returns int as 'select 1' language sql;
select foo(wrong_param$0 := 5);
",
        );
    }

    #[test]
    fn goto_operator_function_ref() {
        assert_snapshot!(goto("
create function pg_catalog.tsvector_concat(tsvector, tsvector) returns tsvector language internal;
create operator pg_catalog.|| (leftarg = tsvector, rightarg = tsvector, function = pg_catalog.tsvector_concat$0);
"), @r"
          ╭▸ 
        2 │ create function pg_catalog.tsvector_concat(tsvector, tsvector) returns tsvector language internal;
          │                            ─────────────── 2. destination
        3 │ create operator pg_catalog.|| (leftarg = tsvector, rightarg = tsvector, function = pg_catalog.tsvector_concat);
          ╰╴                                                                                                            ─ 1. source
        ");
    }

    #[test]
    fn goto_operator_procedure_ref() {
        assert_snapshot!(goto("
create function f(int, int) returns int language internal;
create operator ||| (leftarg = int, rightarg = int, procedure = f$0);
"), @r"
          ╭▸ 
        2 │ create function f(int, int) returns int language internal;
          │                 ─ 2. destination
        3 │ create operator ||| (leftarg = int, rightarg = int, procedure = f);
          ╰╴                                                                ─ 1. source
        ");
    }

    #[test]
    fn goto_operator_expr_usage() {
        assert_snapshot!(goto("
create operator === (leftarg = int, rightarg = int, function = int4eq);
select 1 ===$0 2;
"), @"
          ╭▸ 
        2 │ create operator === (leftarg = int, rightarg = int, function = int4eq);
          │                 ─── 2. destination
        3 │ select 1 === 2;
          ╰╴           ─ 1. source
        ");
    }

    #[test]
    fn goto_operator_explicit_operator_call() {
        assert_snapshot!(goto("
create operator === (leftarg = int, rightarg = int, function = int4eq);
select 1 operator(===$0) 2;
"), @"
          ╭▸ 
        2 │ create operator === (leftarg = int, rightarg = int, function = int4eq);
          │                 ─── 2. destination
        3 │ select 1 operator(===) 2;
          ╰╴                    ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_operator() {
        assert_snapshot!(goto("
create operator === (leftarg = int, rightarg = int, function = int4eq);
drop operator ===$0 (int, int);
"), @"
          ╭▸ 
        2 │ create operator === (leftarg = int, rightarg = int, function = int4eq);
          │                 ─── 2. destination
        3 │ drop operator === (int, int);
          ╰╴                ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_operator_set_schema() {
        assert_snapshot!(goto("
create operator === (leftarg = int, rightarg = int, function = int4eq);
alter operator ===$0 (int, int) set schema public;
"), @"
          ╭▸ 
        2 │ create operator === (leftarg = int, rightarg = int, function = int4eq);
          │                 ─── 2. destination
        3 │ alter operator === (int, int) set schema public;
          ╰╴                 ─ 1. source
        ");
    }

    #[test]
    fn goto_comment_on_operator() {
        assert_snapshot!(goto("
create operator === (leftarg = int, rightarg = int, function = int4eq);
comment on operator ===$0 (int, int) is 'x';
"), @"
          ╭▸ 
        2 │ create operator === (leftarg = int, rightarg = int, function = int4eq);
          │                 ─── 2. destination
        3 │ comment on operator === (int, int) is 'x';
          ╰╴                      ─ 1. source
        ");
    }

    #[test]
    fn goto_operator_class_operator_member() {
        assert_snapshot!(goto("
create operator === (leftarg = int, rightarg = int, function = int4eq);
create operator class c for type int using btree as operator 1 ===$0;
"), @"
          ╭▸ 
        2 │ create operator === (leftarg = int, rightarg = int, function = int4eq);
          │                 ─── 2. destination
        3 │ create operator class c for type int using btree as operator 1 ===;
          ╰╴                                                                 ─ 1. source
        ");
    }

    #[test]
    fn goto_operator_family_operator_member() {
        assert_snapshot!(goto("
create operator === (leftarg = int, rightarg = int, function = int4eq);
create operator family fam using btree;
alter operator family fam using btree add operator 1 ===$0 (int, int);
"), @"
          ╭▸ 
        2 │ create operator === (leftarg = int, rightarg = int, function = int4eq);
          │                 ─── 2. destination
        3 │ create operator family fam using btree;
        4 │ alter operator family fam using btree add operator 1 === (int, int);
          ╰╴                                                       ─ 1. source
        ");
    }

    #[test]
    fn goto_operator_exclude_constraint() {
        assert_snapshot!(goto("
create operator === (leftarg = int, rightarg = int, function = int4eq);
create table t (a int, exclude (a with ===$0));
"), @"
          ╭▸ 
        2 │ create operator === (leftarg = int, rightarg = int, function = int4eq);
          │                 ─── 2. destination
        3 │ create table t (a int, exclude (a with ===));
          ╰╴                                         ─ 1. source
        ");
    }

    #[test]
    fn goto_operator_commutator_option() {
        assert_snapshot!(goto("
create operator === (leftarg = int, rightarg = int, function = int4eq);
create operator ==== (leftarg = int, rightarg = int, function = int4eq, commutator = ===$0);
"), @"
          ╭▸ 
        2 │ create operator === (leftarg = int, rightarg = int, function = int4eq);
          │                 ─── 2. destination
        3 │ create operator ==== (leftarg = int, rightarg = int, function = int4eq, commutator = ===);
          ╰╴                                                                                       ─ 1. source
        ");
    }

    #[test]
    fn goto_operator_schema_qualified() {
        assert_snapshot!(goto("
create operator public.=== (leftarg = int, rightarg = int, function = int4eq);
drop operator public.===$0 (int, int);
"), @"
          ╭▸ 
        2 │ create operator public.=== (leftarg = int, rightarg = int, function = int4eq);
          │                 ────────── 2. destination
        3 │ drop operator public.=== (int, int);
          ╰╴                       ─ 1. source
        ");
    }

    #[test]
    fn goto_create_cast_function_ref() {
        assert_snapshot!(goto("
create type a as enum ('x');
create type b as enum ('x');
create function a_to_b(a) returns b language sql as $$ select 'x'::b $$;
create cast (a as b) with function a_to_b$0(a);
"), @"
          ╭▸ 
        4 │ create function a_to_b(a) returns b language sql as $$ select 'x'::b $$;
          │                 ────── 2. destination
        5 │ create cast (a as b) with function a_to_b(a);
          ╰╴                                        ─ 1. source
        ");
    }

    #[test]
    fn goto_create_type_range_subtype_diff_function_ref() {
        assert_snapshot!(goto("
create function int_diff(int, int) returns float8 language sql as $$ select 0::float8 $$;
create type int_range as range (subtype = int, subtype_diff = int_diff$0);
"), @"
          ╭▸ 
        2 │ create function int_diff(int, int) returns float8 language sql as $$ select 0::float8 $$;
          │                 ──────── 2. destination
        3 │ create type int_range as range (subtype = int, subtype_diff = int_diff);
          ╰╴                                                                     ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_window_partition_column_from_create_table_if_not_exists() {
        assert_snapshot!(goto("
create table t (
    id bigint primary key,
    group_col text not null,
    update_date date not null
);

with row_number_added as (
  select
    *,
    row_number() over (
      partition by group_col$0
      order by update_date desc
    ) as rn
  from t
)
select * from row_number_added
"), @"
           ╭▸ 
         4 │     group_col text not null,
           │     ───────── 2. destination
           ‡
        12 │       partition by group_col
           ╰╴                           ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_window_order_column_from_create_table_if_not_exists() {
        assert_snapshot!(goto("
create table t (
    id bigint primary key,
    group_col text not null,
    update_date date not null
);

with row_number_added as (
  select
    *,
    row_number() over (
      partition by group_col
      order by update_date$0 desc
    ) as rn
  from t
)
select * from row_number_added
"), @"
           ╭▸ 
         5 │     update_date date not null
           │     ─────────── 2. destination
           ‡
        13 │       order by update_date desc
           ╰╴                         ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_window_partition_function_call_from_create_table() {
        assert_snapshot!(goto("
create function length(text) returns int language internal;

create table t (
    id bigint primary key,
    group_col text not null,
    update_date date not null
);

with row_number_added as (
  select
    *,
    row_number() over (
      partition by length$0(group_col)
      order by update_date$0 desc
    ) as rn
  from t
)
select * from row_number_added
"), @"
           ╭▸ 
         2 │ create function length(text) returns int language internal;
           │                 ────── 2. destination
           ‡
        14 │       partition by length(group_col)
           ╰╴                        ─ 1. source
        ");
    }

    #[test]
    fn goto_select_window_def_reuse() {
        assert_snapshot!(goto("
create table tbl (
  id bigint primary key,
  group_col text not null,
  update_date date not null,
  value text
);
select
  id,
  group_col,
  row_number() over w as rn,
  lag(value) over w$0 as prev_value
from tbl
window w as (
  partition by group_col
  order by update_date desc
);
"), @r"
          ╭▸ 
       12 │   lag(value) over w as prev_value
          │                   ─ 1. source
       13 │ from tbl
       14 │ window w as (
          ╰╴       ─ 2. destination
        ");
    }

    #[test]
    fn goto_window_base_name_in_inline_over() {
        assert_snapshot!(goto("
create table t(a int);
select row_number() over (w1$0 order by a)
from t
window w1 as (partition by a);
"), @"
          ╭▸ 
        3 │ select row_number() over (w1 order by a)
          │                            ─ 1. source
        4 │ from t
        5 │ window w1 as (partition by a);
          ╰╴       ── 2. destination
        ");
    }

    #[test]
    fn goto_window_base_name_in_window_def() {
        assert_snapshot!(goto("
create table t(a int);
select row_number() over w2
from t
window w1 as (partition by a), w2 as (w1$0 order by a);
"), @"
          ╭▸ 
        5 │ window w1 as (partition by a), w2 as (w1 order by a);
          ╰╴       ── 2. destination               ─ 1. source
        ");
    }

    #[test]
    fn goto_cast_float_with_small_arg() {
        assert_snapshot!(goto("
create type pg_catalog.float4;
select '1'::float$0(8);
"), @"
          ╭▸ 
        2 │ create type pg_catalog.float4;
          │                        ────── 2. destination
        3 │ select '1'::float(8);
          ╰╴                ─ 1. source
        ");
    }

    #[test]
    fn goto_cast_float_with_large_arg() {
        assert_snapshot!(goto("
create type pg_catalog.float8;
select '1'::float$0(25);
"), @"
          ╭▸ 
        2 │ create type pg_catalog.float8;
          │                        ────── 2. destination
        3 │ select '1'::float(25);
          ╰╴                ─ 1. source
        ");
    }

    #[test]
    fn goto_cast_dec_with_modifier() {
        assert_snapshot!(goto("
create type pg_catalog.numeric;
select '10'::dec$0(10, 2);
"), @"
          ╭▸ 
        2 │ create type pg_catalog.numeric;
          │                        ─────── 2. destination
        3 │ select '10'::dec(10, 2);
          ╰╴               ─ 1. source
        ");
    }

    #[test]
    fn goto_cast_dec() {
        assert_snapshot!(goto("
create type pg_catalog.numeric;
select '10'::dec$0;
"), @"
          ╭▸ 
        2 │ create type pg_catalog.numeric;
          │                        ─────── 2. destination
        3 │ select '10'::dec;
          ╰╴               ─ 1. source
        ");
    }

    #[test]
    fn goto_create_property_graph() {
        assert_snapshot!(goto("
create table buzz.boo(a int, b int);
create property graph foo.bar
  vertex tables (buzz.boo$0 key (a, b) no properties)
  edge tables (foo.bar key (x, y)
    source key (a, b) references k (t, y)
    destination key (q, t) references a (r, j)
    properties all columns);
"), @"
          ╭▸ 
        2 │ create table buzz.boo(a int, b int);
          │                   ─── 2. destination
        3 │ create property graph foo.bar
        4 │   vertex tables (buzz.boo key (a, b) no properties)
          ╰╴                        ─ 1. source
        ");

        assert_snapshot!(goto("
create table foo.bar(x int, y int);
create property graph g
  vertex tables (boo key (a, b) no properties)
  edge tables (foo.bar$0 key (x, y)
    source key (a, b) references k (t, y)
    destination key (q, t) references a (r, j)
    properties all columns);
"), @"
          ╭▸ 
        2 │ create table foo.bar(x int, y int);
          │                  ─── 2. destination
          ‡
        5 │   edge tables (foo.bar key (x, y)
          ╰╴                     ─ 1. source
        ");
    }

    #[test]
    fn goto_create_property_graph_sources_table() {
        assert_snapshot!(goto("
create table v1 (
  id int8 primary key,
  name text
);

create table v2 (
  id int8 primary key,
  name text
);

create table v3 (
  id int8 primary key,
  name text
);

create table e1 (
  id int8 primary key,
  source_id int8 references v1,
  destination_id int8 references v2
);

create table e2 (
  id int8 primary key,
  source_id int8 references v1,
  destination_id int8 references v3
);

create property graph g1
  vertex tables (v1 as source_vertex, v2 as destination_vertex, v3)
  edge tables (
    e1 source source_vertex$0 destination destination_vertex,
    e2 source source_vertex destination v3);
"), @"
           ╭▸ 
        30 │   vertex tables (v1 as source_vertex, v2 as destination_vertex, v3)
           │                        ───────────── 2. destination
        31 │   edge tables (
        32 │     e1 source source_vertex destination destination_vertex,
           ╰╴                          ─ 1. source
        "
        );

        assert_snapshot!(goto("
create table v1 (
  id int8 primary key,
  name text
);

create table v2 (
  id int8 primary key,
  name text
);

create table v3 (
  id int8 primary key,
  name text
);

create table e1 (
  id int8 primary key,
  source_id int8 references v1,
  destination_id int8 references v2
);

create table e2 (
  id int8 primary key,
  source_id int8 references v1,
  destination_id int8 references v3
);

create property graph g1
  vertex tables (v1, v2, v3)
  edge tables (
    e1 source v1 destination v2,
    e2 source v1 destination v3$0);
"), @"
           ╭▸ 
        12 │ create table v3 (
           │              ── 2. destination
           ‡
        33 │     e2 source v1 destination v3);
           ╰╴                              ─ 1. source
        "
        );
    }

    #[test]
    fn goto_create_property_graph_references_table() {
        assert_snapshot!(goto("
create table v1 (id int8 primary key);
create table v2 (id int8 primary key);
create table e1 (
  id int8 primary key,
  source_id int8 references v1,
  destination_id int8 references v2
);

create property graph g1
  vertex tables (v1 as source_vertex, v2)
  edge tables (
    e1
      source key (source_id) references source_vertex$0 (id)
      destination key (destination_id) references v2 (id)
  );
"), @"
           ╭▸ 
        11 │   vertex tables (v1 as source_vertex, v2)
           │                        ───────────── 2. destination
           ‡
        14 │       source key (source_id) references source_vertex (id)
           ╰╴                                                    ─ 1. source
        "
        );
    }

    #[test]
    fn goto_create_property_graph_vertex_key_column() {
        assert_snapshot!(goto("
create table v1 (
  id int8 primary key,
  name text
);

create property graph g1
  vertex tables (v1 key (id$0));
"), @"
          ╭▸ 
        3 │   id int8 primary key,
          │   ── 2. destination
          ‡
        8 │   vertex tables (v1 key (id));
          ╰╴                          ─ 1. source
        ");
    }

    #[test]
    fn goto_create_property_graph_edge_source_key_column() {
        assert_snapshot!(goto("
create table v1 (id int8 primary key);
create table v2 (id int8 primary key);
create table e1 (
  id int8 primary key,
  source_id int8 references v1,
  destination_id int8 references v2
);

create property graph g1
  vertex tables (v1, v2)
  edge tables (
    e1 key (id)
      source key (source_id$0) references v1 (id)
      destination key (destination_id) references v2 (id));
"), @"
           ╭▸ 
         6 │   source_id int8 references v1,
           │   ───────── 2. destination
           ‡
        14 │       source key (source_id) references v1 (id)
           ╰╴                          ─ 1. source
        ");
    }

    #[test]
    fn goto_create_property_graph_edge_source_references_column() {
        assert_snapshot!(goto("
create table v1 (id int8 primary key);
create table v2 (id int8 primary key);
create table e1 (
  id int8 primary key,
  source_id int8 references v1,
  destination_id int8 references v2
);

create property graph g1
  vertex tables (v1 as source_vertex, v2)
  edge tables (
    e1 key (id)
      source key (source_id) references source_vertex (id$0)
      destination key (destination_id) references v2 (id));
"), @"
           ╭▸ 
         2 │ create table v1 (id int8 primary key);
           │                  ── 2. destination
           ‡
        14 │       source key (source_id) references source_vertex (id)
           ╰╴                                                        ─ 1. source
        ");
    }

    #[test]
    fn goto_create_property_graph_edge_destination_key_column() {
        assert_snapshot!(goto("
create table v1 (id int8 primary key);
create table v2 (id int8 primary key);
create table e1 (
  id int8 primary key,
  source_id int8 references v1,
  destination_id int8 references v2
);

create property graph g1
  vertex tables (v1, v2)
  edge tables (
    e1 key (id)
      source key (source_id) references v1 (id)
      destination key (destination_id$0) references v2 (id));
"), @"
           ╭▸ 
         7 │   destination_id int8 references v2
           │   ────────────── 2. destination
           ‡
        15 │       destination key (destination_id) references v2 (id));
           ╰╴                                    ─ 1. source
        ");
    }

    #[test]
    fn goto_create_property_graph_edge_destination_references_column() {
        assert_snapshot!(goto("
create table v1 (id int8 primary key);
create table v2 (id int8 primary key);
create table e1 (
  id int8 primary key,
  source_id int8 references v1,
  destination_id int8 references v2
);

create property graph g1
  vertex tables (v1, v2)
  edge tables (
    e1 key (id)
      source key (source_id) references v1 (id)
      destination key (destination_id) references v2 (id$0));
"), @"
           ╭▸ 
         3 │ create table v2 (id int8 primary key);
           │                  ── 2. destination
           ‡
        15 │       destination key (destination_id) references v2 (id));
           ╰╴                                                       ─ 1. source
        ");
    }

    #[test]
    fn goto_create_property_graph_vertex_properties_column() {
        assert_snapshot!(goto("
create table v1 (
  id int8 primary key,
  name text
);

create property graph g1
  vertex tables (v1 properties (id$0, name));
"), @"
          ╭▸ 
        3 │   id int8 primary key,
          │   ── 2. destination
          ‡
        8 │   vertex tables (v1 properties (id, name));
          ╰╴                                 ─ 1. source
        ");

        assert_snapshot!(goto("
create table v1 (
  id int8 primary key,
  name text
);

create property graph g1
  vertex tables (v1 properties (id, nam$0e));
"), @"
          ╭▸ 
        4 │   name text
          │   ──── 2. destination
          ‡
        8 │   vertex tables (v1 properties (id, name));
          ╰╴                                      ─ 1. source
        ");
    }

    #[test]
    fn goto_create_property_graph_edge_properties_column() {
        assert_snapshot!(goto("
create table v1 (id int8 primary key);
create table v2 (id int8 primary key);
create table e1 (
  id int8 primary key,
  source_id int8 references v1,
  destination_id int8 references v2
);

create property graph g1
  vertex tables (v1, v2)
  edge tables (
    e1
      source v1
      destination v2
      properties (id, source_id$0, destination_id));
"), @"
           ╭▸ 
         6 │   source_id int8 references v1,
           │   ───────── 2. destination
           ‡
        16 │       properties (id, source_id, destination_id));
           ╰╴                              ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_property_graph() {
        assert_snapshot!(goto("
create property graph foo.bar vertex tables (t key (a) no properties);
drop property graph foo.ba$0r;
"), @"
          ╭▸ 
        2 │ create property graph foo.bar vertex tables (t key (a) no properties);
          │                           ─── 2. destination
        3 │ drop property graph foo.bar;
          ╰╴                         ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_property_graph() {
        assert_snapshot!(goto("
create property graph foo.bar vertex tables (t key (a) no properties);
alter property graph foo.ba$0r rename to baz;
"), @"
          ╭▸ 
        2 │ create property graph foo.bar vertex tables (t key (a) no properties);
          │                           ─── 2. destination
        3 │ alter property graph foo.bar rename to baz;
          ╰╴                          ─ 1. source
        ");
    }

    #[test]
    fn goto_graph_table_fn() {
        assert_snapshot!(goto("
create property graph myshop vertex tables (t key (a) no properties);
select 1 from graph_table (myshop$0
  match (n is t)
  columns (1 as x));
"), @"
          ╭▸ 
        2 │ create property graph myshop vertex tables (t key (a) no properties);
          │                       ────── 2. destination
        3 │ select 1 from graph_table (myshop
          ╰╴                                ─ 1. source
        ");
    }

    #[test]
    fn goto_create_function_param_percent_type_column() {
        assert_snapshot!(goto("
create schema s;
create table s.t (a int, b text);
create function f(x s.t.a$0%type) returns s.t.b%type
  as $$ select 'hello'::text $$ language sql;
"), @"
          ╭▸ 
        3 │ create table s.t (a int, b text);
          │                   ─ 2. destination
        4 │ create function f(x s.t.a%type) returns s.t.b%type
          ╰╴                        ─ 1. source
        ");
    }

    #[test]
    fn goto_create_function_param_percent_type_table() {
        assert_snapshot!(goto("
create schema s;
create table s.t (a int, b text);
create function f(x s.t$0.a%type) returns s.t.b%type
  as $$ select 'hello'::text $$ language sql;
"), @"
          ╭▸ 
        3 │ create table s.t (a int, b text);
          │                ─ 2. destination
        4 │ create function f(x s.t.a%type) returns s.t.b%type
          ╰╴                      ─ 1. source
        ");
    }

    #[test]
    fn goto_create_function_param_percent_type_schema() {
        assert_snapshot!(goto("
create schema s;
create table s.t (a int, b text);
create function f(x s$0.t.a%type) returns s.t.b%type
  as $$ select 'hello'::text $$ language sql;
"), @"
          ╭▸ 
        2 │ create schema s;
          │               ─ 2. destination
        3 │ create table s.t (a int, b text);
        4 │ create function f(x s.t.a%type) returns s.t.b%type
          ╰╴                    ─ 1. source
        ");
    }

    #[test]
    fn goto_create_function_returns_percent_type_column() {
        assert_snapshot!(goto("
create schema s;
create table s.t (a int, b text);
create function f(x s.t.a%type) returns s.t.b$0%type
  as $$ select 'hello'::text $$ language sql;
"), @"
          ╭▸ 
        3 │ create table s.t (a int, b text);
          │                          ─ 2. destination
        4 │ create function f(x s.t.a%type) returns s.t.b%type
          ╰╴                                            ─ 1. source
        ");
    }

    #[test]
    fn goto_create_function_param_percent_type_two_part() {
        assert_snapshot!(goto("
create table t (a int, b text);
create function f(x t.a$0%type) returns t.b%type
  as $$ select 'hello'::text $$ language sql;
"), @"
          ╭▸ 
        2 │ create table t (a int, b text);
          │                 ─ 2. destination
        3 │ create function f(x t.a%type) returns t.b%type
          ╰╴                      ─ 1. source
        ");
    }

    #[test]
    fn goto_grant_table() {
        assert_snapshot!(goto("
create table foo (id int);
grant select on foo$0 to bob;
"), @"
          ╭▸ 
        2 │ create table foo (id int);
          │              ─── 2. destination
        3 │ grant select on foo to bob;
          ╰╴                  ─ 1. source
        ");
    }

    #[test]
    fn goto_grant_table_keyword() {
        assert_snapshot!(goto("
create table foo (id int);
grant select on table foo$0 to bob;
"), @"
          ╭▸ 
        2 │ create table foo (id int);
          │              ─── 2. destination
        3 │ grant select on table foo to bob;
          ╰╴                        ─ 1. source
        ");
    }

    #[test]
    fn goto_revoke_table() {
        assert_snapshot!(goto("
create table foo (id int);
revoke select on foo$0 from bob;
"), @"
          ╭▸ 
        2 │ create table foo (id int);
          │              ─── 2. destination
        3 │ revoke select on foo from bob;
          ╰╴                   ─ 1. source
        ");
    }

    #[test]
    fn goto_grant_column() {
        assert_snapshot!(goto("
create table foo (id int);
grant select (id$0) on foo to bob;
"), @"
          ╭▸ 
        2 │ create table foo (id int);
          │                   ── 2. destination
        3 │ grant select (id) on foo to bob;
          ╰╴               ─ 1. source
        ");
    }

    #[test]
    fn goto_grant_sequence() {
        assert_snapshot!(goto("
create sequence s;
grant usage on sequence s$0 to bob;
"), @"
          ╭▸ 
        2 │ create sequence s;
          │                 ─ 2. destination
        3 │ grant usage on sequence s to bob;
          ╰╴                        ─ 1. source
        ");
    }

    #[test]
    fn goto_grant_function() {
        assert_snapshot!(goto("
create function f() returns int language sql as 'select 1';
grant execute on function f$0 to bob;
"), @"
          ╭▸ 
        2 │ create function f() returns int language sql as 'select 1';
          │                 ─ 2. destination
        3 │ grant execute on function f to bob;
          ╰╴                          ─ 1. source
        ");
    }

    #[test]
    fn goto_grant_schema() {
        assert_snapshot!(goto("
create schema myschema;
grant usage on schema myschema$0 to bob;
"), @"
          ╭▸ 
        2 │ create schema myschema;
          │               ──────── 2. destination
        3 │ grant usage on schema myschema to bob;
          ╰╴                             ─ 1. source
        ");
    }

    #[test]
    fn goto_grant_view() {
        assert_snapshot!(goto("
create view v as select 1;
grant select on v$0 to bob;
"), @"
          ╭▸ 
        2 │ create view v as select 1;
          │             ─ 2. destination
        3 │ grant select on v to bob;
          ╰╴                ─ 1. source
        ");
    }

    #[test]
    fn goto_grant_domain() {
        assert_snapshot!(goto("
create domain d as int;
grant usage on domain d$0 to bob;
"), @"
          ╭▸ 
        2 │ create domain d as int;
          │               ─ 2. destination
        3 │ grant usage on domain d to bob;
          ╰╴                      ─ 1. source
        ");
    }

    #[test]
    fn goto_grant_language() {
        assert_snapshot!(goto("
create language mylang handler h;
grant usage on language mylang$0 to bob;
"), @"
          ╭▸ 
        2 │ create language mylang handler h;
          │                 ────── 2. destination
        3 │ grant usage on language mylang to bob;
          ╰╴                             ─ 1. source
        ");
    }

    #[test]
    fn goto_grant_foreign_server() {
        assert_snapshot!(goto("
create foreign data wrapper fdw;
create server s foreign data wrapper fdw;
grant usage on foreign server s$0 to bob;
"), @"
          ╭▸ 
        3 │ create server s foreign data wrapper fdw;
          │               ─ 2. destination
        4 │ grant usage on foreign server s to bob;
          ╰╴                              ─ 1. source
        ");
    }

    #[test]
    fn goto_grant_foreign_data_wrapper() {
        assert_snapshot!(goto("
create foreign data wrapper w;
grant usage on foreign data wrapper w$0 to bob;
"), @"
          ╭▸ 
        2 │ create foreign data wrapper w;
          │                             ─ 2. destination
        3 │ grant usage on foreign data wrapper w to bob;
          ╰╴                                    ─ 1. source
        ");
    }

    #[test]
    fn goto_grant_all_tables_in_schema() {
        assert_snapshot!(goto("
create schema sc;
grant select on all tables in schema sc$0 to bob;
"), @"
          ╭▸ 
        2 │ create schema sc;
          │               ── 2. destination
        3 │ grant select on all tables in schema sc to bob;
          ╰╴                                      ─ 1. source
        ");
    }

    #[test]
    fn goto_create_statistics_column() {
        assert_snapshot!(goto("
create table t(a int, b int);
create statistics st on a$0, b from t;
"), @"
          ╭▸ 
        2 │ create table t(a int, b int);
          │                ─ 2. destination
        3 │ create statistics st on a, b from t;
          ╰╴                        ─ 1. source
        ");
    }

    #[test]
    fn goto_create_statistics_table() {
        assert_snapshot!(goto("
create table t(a int, b int);
create statistics st on a, b from t$0;
"), @"
          ╭▸ 
        2 │ create table t(a int, b int);
          │              ─ 2. destination
        3 │ create statistics st on a, b from t;
          ╰╴                                  ─ 1. source
        ");
    }

    #[test]
    fn goto_create_statistics_schema_qualified_table() {
        assert_snapshot!(goto("
create schema s;
create table s.t(a int, b int);
create statistics st on a, b from s.t$0;
"), @"
          ╭▸ 
        3 │ create table s.t(a int, b int);
          │                ─ 2. destination
        4 │ create statistics st on a, b from s.t;
          ╰╴                                    ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_statistics() {
        assert_snapshot!(goto("
create table t(a int);
create statistics s on a from t;
drop statistics s$0;
"), @"
          ╭▸ 
        3 │ create statistics s on a from t;
          │                   ─ 2. destination
        4 │ drop statistics s;
          ╰╴                ─ 1. source
        ");
    }

    #[test]
    fn goto_alter_statistics() {
        assert_snapshot!(goto("
create table t(a int);
create statistics s on a from t;
alter statistics s$0 set statistics 100;
"), @"
          ╭▸ 
        3 │ create statistics s on a from t;
          │                   ─ 2. destination
        4 │ alter statistics s set statistics 100;
          ╰╴                 ─ 1. source
        ");
    }

    #[test]
    fn goto_comment_on_statistics() {
        assert_snapshot!(goto("
create table t(a int);
create statistics s on a from t;
comment on statistics s$0 is '';
"), @"
          ╭▸ 
        3 │ create statistics s on a from t;
          │                   ─ 2. destination
        4 │ comment on statistics s is '';
          ╰╴                      ─ 1. source
        ");
    }

    #[test]
    fn goto_create_publication_table() {
        assert_snapshot!(goto("
create table t(a int);
create publication pub for table t$0;
"), @"
          ╭▸ 
        2 │ create table t(a int);
          │              ─ 2. destination
        3 │ create publication pub for table t;
          ╰╴                                 ─ 1. source
        ");
    }

    #[test]
    fn goto_create_publication_column() {
        assert_snapshot!(goto("
create table t(a int, b int);
create publication pub for table t (a$0, b);
"), @"
          ╭▸ 
        2 │ create table t(a int, b int);
          │                ─ 2. destination
        3 │ create publication pub for table t (a, b);
          ╰╴                                    ─ 1. source
        ");
    }

    #[test]
    fn goto_create_publication_where_column() {
        assert_snapshot!(goto("
create table t(a int, b int);
create publication pub for table t where (a$0 > 1);
"), @"
          ╭▸ 
        2 │ create table t(a int, b int);
          │                ─ 2. destination
        3 │ create publication pub for table t where (a > 1);
          ╰╴                                          ─ 1. source
        ");
    }

    #[test]
    fn goto_count_star_filter_column() {
        assert_snapshot!(goto("
create table t (a int);
select count(*) filter (where a$0 > 0) from t;
"), @"
          ╭▸ 
        2 │ create table t (a int);
          │                 ─ 2. destination
        3 │ select count(*) filter (where a > 0) from t;
          ╰╴                              ─ 1. source
        ");
    }

    #[test]
    fn goto_with_ordinality_implicit_column() {
        assert_snapshot!(goto("
select ordinality$0 from unnest(array[1,2]) with ordinality;
"), @"
          ╭▸ 
        2 │ select ordinality from unnest(array[1,2]) with ordinality;
          ╰╴                ─ 1. source                    ────────── 2. destination
        ");
    }

    #[test]
    fn goto_with_ordinality_qualified_implicit_column() {
        assert_snapshot!(goto("
select u.ordinality$0 from unnest(array[1,2]) with ordinality as u;
"), @"
          ╭▸ 
        2 │ select u.ordinality from unnest(array[1,2]) with ordinality as u;
          ╰╴                  ─ 1. source                    ────────── 2. destination
        ");
    }

    #[test]
    fn goto_with_ordinality_explicit_alias_column() {
        assert_snapshot!(goto("
select o$0 from unnest(array[1,2]) with ordinality as u(x, o);
"), @"
          ╭▸ 
        2 │ select o from unnest(array[1,2]) with ordinality as u(x, o);
          ╰╴       ─ 1. source                                       ─ 2. destination
        ");
    }

    #[test]
    fn goto_rows_from_with_ordinality_implicit_column() {
        assert_snapshot!(goto("
select ordinality$0 from rows from (unnest(array[1,2])) with ordinality;
"), @"
          ╭▸ 
        2 │ select ordinality from rows from (unnest(array[1,2])) with ordinality;
          ╰╴                ─ 1. source                                ────────── 2. destination
        ");
    }
}
