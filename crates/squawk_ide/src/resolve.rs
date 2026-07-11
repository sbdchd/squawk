use crate::{ast_nav, db::list_files};
use smallvec::{SmallVec, smallvec};
use squawk_syntax::{
    SyntaxKind, SyntaxNode, SyntaxNodePtr,
    ast::{self, AstNode},
};

use crate::binder::ResolvedSchemas;
use crate::column_name::ColumnName;
use crate::db::File;
use crate::file::InFile;
use crate::location::{Location, LocationKind};
use crate::name::{self, Name, Schema};
use crate::symbols::SymbolKind;
use crate::{
    classify::{NameRefClass, classify_name_ref},
    db::{bind, parse},
};
use salsa::Database as Db;

/// Resolves a name reference to its definition(s).
///
/// Most of the time returns one result, but can return two definitions
/// in the case of:
///
/// ```sql
/// select * from t join u using (col);
/// ```
///
/// since `col` is defined in both `t` and `u`.
pub(crate) fn resolve_name_ref(
    db: &dyn Db,
    name_ref: InFile<&ast::NameRef>,
) -> Option<SmallVec<[Location; 1]>> {
    let file = name_ref.file_id;
    let name_ref = name_ref.value;
    let binder = bind(db, file);
    let context = classify_name_ref(name_ref.syntax())?;

    match context {
        NameRefClass::Table => {
            let (schema, table_name) = name::schema_and_table_name(name_ref)?;
            let position = name_ref.syntax().text_range().start();

            if schema.is_none()
                && let Some(cte_ptr) = resolve_cte_table(name_ref, &table_name)
            {
                return Some(smallvec![Location::new(
                    file,
                    cte_ptr.text_range(),
                    LocationKind::Table
                )]);
            }

            let schemas = binder.resolved_schemas(position, schema.as_ref());
            let ptr = resolve_table_name_ptr(db, &table_name, &schemas, file)?;
            Some(smallvec![Location::new(
                file,
                ptr.text_range(),
                LocationKind::Table
            )])
        }
        NameRefClass::InsertTable => {
            let (schema, relation_name) = name::schema_and_table_name(name_ref)?;
            let position = name_ref.syntax().text_range().start();
            let schemas = binder.resolved_schemas(position, schema.as_ref());

            if let Some(ptr) = resolve_table_name_ptr(db, &relation_name, &schemas, file) {
                return Some(smallvec![Location::new(
                    file,
                    ptr.text_range(),
                    LocationKind::Table
                )]);
            }

            let ptr = resolve_view_name_ptr(db, &relation_name, &schemas, file)?;
            Some(smallvec![Location::new(
                file,
                ptr.text_range(),
                LocationKind::View
            )])
        }
        NameRefClass::NamedArgParameter => {
            let (schema, function_name) = name_ref.syntax().ancestors().find_map(|a| {
                if let Some(call_expr) = ast::CallExpr::cast(a.clone()) {
                    name::schema_and_func_name(&call_expr)
                } else if let Some(call) = ast::Call::cast(a) {
                    name::schema_and_name_path(&call.path()?)
                } else {
                    None
                }
            })?;
            let param_name = Name::from_node(name_ref);
            let position = name_ref.syntax().text_range().start();
            let schemas = binder.resolved_schemas(position, schema.as_ref());

            // TODO: this should be one lookup
            let function_ptr = binder
                .lookup_with(&function_name, SymbolKind::Function, &schemas)
                .or_else(|| binder.lookup_with(&function_name, SymbolKind::Procedure, &schemas))
                .or_else(|| binder.lookup_with(&function_name, SymbolKind::Aggregate, &schemas))?;

            let param_ptr =
                find_param_in_func_def(db, InFile::new(file, function_ptr), &param_name)?;
            Some(smallvec![Location::new(
                file,
                param_ptr.text_range(),
                LocationKind::NamedArgParameter
            )])
        }
        NameRefClass::ParamDefault => resolve_enclosing_function_param(InFile::new(file, name_ref)),
        NameRefClass::Cursor => {
            let cursor_name = &Name::from_node(name_ref);
            let ptr = binder.lookup(cursor_name, SymbolKind::Cursor)?;
            Some(smallvec![Location::new(
                file,
                ptr.text_range(),
                LocationKind::Cursor
            )])
        }
        NameRefClass::PreparedStatement => {
            let statement_name = &Name::from_node(name_ref);
            let binder = bind(db, file);
            let ptr = binder.lookup(statement_name, SymbolKind::PreparedStatement)?;
            Some(smallvec![Location::new(
                file,
                ptr.text_range(),
                LocationKind::PreparedStatement
            )])
        }
        NameRefClass::Channel => {
            let channel_name = &Name::from_node(name_ref);
            let binder = bind(db, file);
            let ptr = binder.lookup(channel_name, SymbolKind::Channel)?;
            Some(smallvec![Location::new(
                file,
                ptr.text_range(),
                LocationKind::Channel
            )])
        }
        NameRefClass::FromTable => {
            let (schema, table_name) = name::schema_and_name(name_ref);
            let position = name_ref.syntax().text_range().start();
            let schemas = binder.resolved_schemas(position, schema.as_ref());
            let (ptr, kind) = resolve_table_like(db, Some(name_ref), &table_name, &schemas, file)?;
            Some(smallvec![Location::new(file, ptr.text_range(), kind)])
        }
        NameRefClass::Index => {
            let (schema, index_name) = name::schema_and_table_name(name_ref)?;
            let position = name_ref.syntax().text_range().start();
            let schemas = binder.resolved_schemas(position, schema.as_ref());
            let ptr = binder.lookup_with(&index_name, SymbolKind::Index, &schemas)?;
            Some(smallvec![Location::new(
                file,
                ptr.text_range(),
                LocationKind::Index
            )])
        }
        NameRefClass::Type => {
            let (schema, type_name) = name::schema_and_table_name(name_ref)?;
            let type_name = resolve_float_precision(name_ref, type_name);
            let position = name_ref.syntax().text_range().start();
            let schemas = binder.resolved_schemas(position, schema.as_ref());
            let ptr = resolve_type_name_ptr(db, &type_name, &schemas, file)?;
            Some(smallvec![Location::new(
                file,
                ptr.text_range(),
                LocationKind::Type
            )])
        }
        NameRefClass::View => {
            let (schema, view_name) = name::schema_and_table_name(name_ref)?;
            let position = name_ref.syntax().text_range().start();
            let schemas = binder.resolved_schemas(position, schema.as_ref());
            let ptr = resolve_view_name_ptr(db, &view_name, &schemas, file)?;
            Some(smallvec![Location::new(
                file,
                ptr.text_range(),
                LocationKind::View
            )])
        }
        NameRefClass::Window => {
            let window_name = Name::from_node(name_ref);
            let select = name_ref.syntax().ancestors().find_map(ast::Select::cast)?;
            for window_def in select.window_clause()?.window_defs() {
                if let Some(name) = window_def.name()
                    && Name::from_node(&name) == window_name
                {
                    return Some(smallvec![Location::new(
                        file,
                        name.syntax().text_range(),
                        LocationKind::Window
                    )]);
                }
            }
            None
        }
        NameRefClass::Sequence => {
            let (schema, sequence_name) = name::schema_and_table_name(name_ref)?;
            let position = name_ref.syntax().text_range().start();
            let schemas = binder.resolved_schemas(position, schema.as_ref());
            let ptr = binder.lookup_with(&sequence_name, SymbolKind::Sequence, &schemas)?;
            Some(smallvec![Location::new(
                file,
                ptr.text_range(),
                LocationKind::Sequence
            )])
        }
        NameRefClass::Trigger => {
            let (mut schema, trigger_name, on_table_path) =
                name_ref.syntax().ancestors().find_map(|a| {
                    if let Some(drop_trigger) = ast::DropTrigger::cast(a.clone()) {
                        let (schema, trigger_name) =
                            name::schema_and_name_path(&drop_trigger.path()?)?;
                        Some((
                            schema,
                            trigger_name,
                            drop_trigger.on_table().and_then(|on_table| on_table.path()),
                        ))
                    } else if let Some(alter_trigger) = ast::AlterTrigger::cast(a.clone()) {
                        Some((
                            None,
                            Name::from_node(name_ref),
                            alter_trigger
                                .on_table()
                                .and_then(|on_table| on_table.path()),
                        ))
                    } else {
                        ast::AlterTable::cast(a).map(|alter_table| {
                            (
                                None,
                                Name::from_node(name_ref),
                                alter_table
                                    .relation_name()
                                    .and_then(|relation| relation.path()),
                            )
                        })
                    }
                })?;
            let on_table_path = on_table_path?;
            if schema.is_none() {
                schema = name::schema_name(&on_table_path);
            }
            let table_name = name::table_name(&on_table_path)?;
            let position = name_ref.syntax().text_range().start();
            let schemas = binder.resolved_schemas(position, schema.as_ref());
            let ptr = binder.lookup_with_table(
                &trigger_name,
                SymbolKind::Trigger,
                &schemas,
                &Some(table_name),
            )?;
            Some(smallvec![Location::new(
                file,
                ptr.text_range(),
                LocationKind::Trigger
            )])
        }
        NameRefClass::TriggerEventColumn => {
            let column_name = Name::from_node(name_ref);
            let create_trigger = name_ref
                .syntax()
                .ancestors()
                .find_map(ast::CreateTrigger::cast)?;
            let path = create_trigger.on_table()?.path()?;
            resolve_column_for_path(db, InFile::new(file, &path), column_name)
        }
        NameRefClass::TriggerWhenColumn => {
            let column_name = Name::from_node(name_ref);
            let create_trigger = name_ref
                .syntax()
                .ancestors()
                .find_map(ast::CreateTrigger::cast)?;
            let path = create_trigger.on_table()?.path()?;
            resolve_column_for_path(db, InFile::new(file, &path), column_name)
        }
        NameRefClass::TriggerWhenColumnTable => {
            let create_trigger = name_ref
                .syntax()
                .ancestors()
                .find_map(ast::CreateTrigger::cast)?;
            let path = create_trigger.on_table()?.path()?;
            let (schema, table_name) = name::schema_and_name_path(&path)?;
            let position = name_ref.syntax().text_range().start();
            let schemas = binder.resolved_schemas(position, schema.as_ref());
            let ptr = resolve_table_name_ptr(db, &table_name, &schemas, file)?;
            Some(smallvec![Location::new(
                file,
                ptr.text_range(),
                LocationKind::Table
            )])
        }
        NameRefClass::Policy => {
            let (policy_name, on_table) = name_ref.syntax().ancestors().find_map(|a| {
                if let Some(policy) = ast::DropPolicy::cast(a.clone()) {
                    Some((policy.name_ref(), policy.on_table()))
                } else {
                    ast::AlterPolicy::cast(a).map(|policy| (policy.name_ref(), policy.on_table()))
                }
            })?;
            let policy_name = Name::from_node(&policy_name?);
            let on_table_path = on_table.and_then(|on_table| on_table.path())?;
            let (schema, table_name) = name::schema_and_name_path(&on_table_path)?;
            let position = name_ref.syntax().text_range().start();
            let schemas = binder.resolved_schemas(position, schema.as_ref());
            let ptr = binder.lookup_with_table(
                &policy_name,
                SymbolKind::Policy,
                &schemas,
                &Some(table_name),
            )?;
            Some(smallvec![Location::new(
                file,
                ptr.text_range(),
                LocationKind::Policy
            )])
        }
        NameRefClass::Rule => {
            let (rule_name, on_table_path) = name_ref.syntax().ancestors().find_map(|a| {
                if let Some(drop_rule) = ast::DropRule::cast(a.clone()) {
                    Some((
                        drop_rule.name_ref(),
                        drop_rule.on_table().and_then(|on_table| on_table.path()),
                    ))
                } else if let Some(alter_rule) = ast::AlterRule::cast(a.clone()) {
                    Some((
                        alter_rule.name_ref(),
                        alter_rule.on_table().and_then(|on_table| on_table.path()),
                    ))
                } else {
                    ast::AlterTable::cast(a).map(|alter_table| {
                        (
                            Some(name_ref.clone()),
                            alter_table
                                .relation_name()
                                .and_then(|relation| relation.path()),
                        )
                    })
                }
            })?;
            let rule_name = Name::from_node(&rule_name?);
            let on_table_path = on_table_path?;
            let (schema, table_name) = name::schema_and_name_path(&on_table_path)?;
            let position = name_ref.syntax().text_range().start();
            let schemas = binder.resolved_schemas(position, schema.as_ref());
            let ptr = binder.lookup_with_table(
                &rule_name,
                SymbolKind::Rule,
                &schemas,
                &Some(table_name),
            )?;
            Some(smallvec![Location::new(
                file,
                ptr.text_range(),
                LocationKind::Rule
            )])
        }
        NameRefClass::RulePseudoColumn => {
            let column_name = Name::from_node(name_ref);
            let create_rule = name_ref
                .syntax()
                .ancestors()
                .find_map(ast::CreateRule::cast)?;
            let path = create_rule.rule_on()?.path()?;
            resolve_column_for_path(db, InFile::new(file, &path), column_name)
        }
        NameRefClass::RulePseudoColumnTable => {
            let create_rule = name_ref
                .syntax()
                .ancestors()
                .find_map(ast::CreateRule::cast)?;
            let path = create_rule.rule_on()?.path()?;
            let (schema, table_name) = name::schema_and_name_path(&path)?;
            let position = name_ref.syntax().text_range().start();
            let schemas = binder.resolved_schemas(position, schema.as_ref());
            let ptr = resolve_table_name_ptr(db, &table_name, &schemas, file)?;
            Some(smallvec![Location::new(
                file,
                ptr.text_range(),
                LocationKind::Table
            )])
        }
        NameRefClass::EventTrigger => {
            let event_trigger_name = Name::from_node(name_ref);
            let binder = bind(db, file);
            let ptr = binder.lookup(&event_trigger_name, SymbolKind::EventTrigger)?;
            Some(smallvec![Location::new(
                file,
                ptr.text_range(),
                LocationKind::EventTrigger
            )])
        }
        NameRefClass::PropertyGraph => {
            let path = name_ref.syntax().ancestors().find_map(ast::Path::cast)?;
            let (schema, property_graph_name) = name::schema_and_name_path(&path)?;
            let position = name_ref.syntax().text_range().start();
            let schemas = binder.resolved_schemas(position, schema.as_ref());
            let ptr =
                binder.lookup_with(&property_graph_name, SymbolKind::PropertyGraph, &schemas)?;
            Some(smallvec![Location::new(
                file,
                ptr.text_range(),
                LocationKind::PropertyGraph
            )])
        }
        NameRefClass::Database => {
            let database_name = Name::from_node(name_ref);
            let binder = bind(db, file);
            let ptr = binder.lookup(&database_name, SymbolKind::Database)?;
            Some(smallvec![Location::new(
                file,
                ptr.text_range(),
                LocationKind::Database
            )])
        }
        NameRefClass::Server => {
            let server_name = Name::from_node(name_ref);
            let binder = bind(db, file);
            let ptr = binder.lookup(&server_name, SymbolKind::Server)?;
            Some(smallvec![Location::new(
                file,
                ptr.text_range(),
                LocationKind::Server
            )])
        }
        NameRefClass::Extension => {
            let extension_name = Name::from_node(name_ref);
            let binder = bind(db, file);
            let ptr = binder.lookup(&extension_name, SymbolKind::Extension)?;
            Some(smallvec![Location::new(
                file,
                ptr.text_range(),
                LocationKind::Extension
            )])
        }
        NameRefClass::ForeignDataWrapper => {
            let fdw_name = Name::from_node(name_ref);
            let binder = bind(db, file);
            let ptr = binder.lookup(&fdw_name, SymbolKind::ForeignDataWrapper)?;
            Some(smallvec![Location::new(
                file,
                ptr.text_range(),
                LocationKind::ForeignDataWrapper
            )])
        }
        NameRefClass::Publication => {
            let publication_name = Name::from_node(name_ref);
            let binder = bind(db, file);
            let ptr = binder.lookup(&publication_name, SymbolKind::Publication)?;
            Some(smallvec![Location::new(
                file,
                ptr.text_range(),
                LocationKind::Publication
            )])
        }
        NameRefClass::Subscription => {
            let subscription_name = Name::from_node(name_ref);
            let binder = bind(db, file);
            let ptr = binder.lookup(&subscription_name, SymbolKind::Subscription)?;
            Some(smallvec![Location::new(
                file,
                ptr.text_range(),
                LocationKind::Subscription
            )])
        }
        NameRefClass::Language => {
            let language_name = Name::from_node(name_ref);
            let binder = bind(db, file);
            let ptr = binder.lookup(&language_name, SymbolKind::Language)?;
            Some(smallvec![Location::new(
                file,
                ptr.text_range(),
                LocationKind::Language
            )])
        }
        NameRefClass::Collation => {
            let (schema, collation_name) = name::schema_and_name(name_ref);
            let position = name_ref.syntax().text_range().start();
            let binder = bind(db, file);
            let schemas = binder.resolved_schemas(position, schema.as_ref());
            let ptr = binder.lookup_with(&collation_name, SymbolKind::Collation, &schemas)?;
            Some(smallvec![Location::new(
                file,
                ptr.text_range(),
                LocationKind::Collation
            )])
        }
        NameRefClass::Role => {
            let role_name = Name::from_node(name_ref);
            let binder = bind(db, file);
            let ptr = binder.lookup(&role_name, SymbolKind::Role)?;
            Some(smallvec![Location::new(
                file,
                ptr.text_range(),
                LocationKind::Role
            )])
        }
        NameRefClass::QualifiedColumn => {
            let path = name_ref.syntax().ancestors().find_map(ast::Path::cast)?;
            let column_name = Name::from_node(name_ref);
            let table_path = path.qualifier()?;
            resolve_column_for_path(db, InFile::new(file, &table_path), column_name)
        }
        NameRefClass::TableAndColumnsColumn => {
            let column_name = Name::from_node(name_ref);
            let table_and_columns = name_ref
                .syntax()
                .ancestors()
                .find_map(ast::TableAndColumns::cast)?;
            let path = table_and_columns.relation_name()?.path()?;
            resolve_column_for_path(db, InFile::new(file, &path), column_name)
        }
        NameRefClass::Tablespace => {
            let tablespace_name = Name::from_node(name_ref);
            let binder = bind(db, file);
            let ptr = binder.lookup(&tablespace_name, SymbolKind::Tablespace)?;
            Some(smallvec![Location::new(
                file,
                ptr.text_range(),
                LocationKind::Tablespace
            )])
        }
        NameRefClass::ForeignKeyTable => {
            let path = name_ref.syntax().ancestors().find_map(ast::Path::cast)?;
            let (schema, table_name) = name::schema_and_name_path(&path)?;
            let position = name_ref.syntax().text_range().start();
            let schemas = binder.resolved_schemas(position, schema.as_ref());
            let ptr = resolve_table_name_ptr(db, &table_name, &schemas, file)?;
            Some(smallvec![Location::new(
                file,
                ptr.text_range(),
                LocationKind::Table
            )])
        }
        NameRefClass::ForeignKeyColumn => {
            // TODO: the ast is too flat here
            let path = if let Some(foreign_key) = name_ref
                .syntax()
                .ancestors()
                .find_map(ast::ForeignKeyConstraint::cast)
            {
                foreign_key.path()?
            } else if let Some(references_constraint) = name_ref
                .syntax()
                .ancestors()
                .find_map(ast::ReferencesConstraint::cast)
            {
                references_constraint.table()?
            } else {
                return None;
            };
            let column_name = Name::from_node(name_ref);
            resolve_column_for_path(db, InFile::new(file, &path), column_name)
        }
        NameRefClass::Constraint => resolve_constraint(db, InFile::new(file, name_ref)),
        NameRefClass::ConstraintColumn => {
            let column_name = Name::from_node(name_ref);
            for ancestor in name_ref.syntax().ancestors() {
                if let Some(create_table) = ast::CreateTableLike::cast(ancestor.clone()) {
                    return find_column_in_create_table(
                        db,
                        InFile::new(file, &create_table),
                        &column_name,
                    );
                }
                if let Some(alter_table) = ast::AlterTable::cast(ancestor) {
                    let table_path = alter_table.relation_name()?.path()?;
                    return resolve_column_for_path(
                        db,
                        InFile::new(file, &table_path),
                        column_name,
                    );
                }
            }
            None
        }
        NameRefClass::CopyColumn => {
            let column_name = Name::from_node(name_ref);
            let copy = name_ref.syntax().ancestors().find_map(ast::Copy::cast)?;
            let path = copy.copy_table()?.path()?;
            resolve_column_for_path(db, InFile::new(file, &path), column_name)
        }
        NameRefClass::StatisticsColumn => {
            let column_name = Name::from_node(name_ref);
            let create_statistics = name_ref
                .syntax()
                .ancestors()
                .find_map(ast::CreateStatistics::cast)?;
            let path = create_statistics.from_table()?.path()?;
            resolve_column_for_path(db, InFile::new(file, &path), column_name)
        }
        NameRefClass::PublicationColumn => {
            let column_name = Name::from_node(name_ref);
            let publication_object = name_ref
                .syntax()
                .ancestors()
                .find_map(ast::PublicationObject::cast)?;
            let path = publication_object.path()?;
            resolve_column_for_path(db, InFile::new(file, &path), column_name)
        }
        NameRefClass::PolicyColumn => {
            let on_table_path = name_ref.syntax().ancestors().find_map(|n| {
                if let Some(create_policy) = ast::CreatePolicy::cast(n.clone()) {
                    create_policy.on_table()?.path()
                } else if let Some(alter_policy) = ast::AlterPolicy::cast(n) {
                    alter_policy.on_table()?.path()
                } else {
                    None
                }
            })?;
            let column_name = Name::from_node(name_ref);
            resolve_column_for_path(db, InFile::new(file, &on_table_path), column_name)
        }
        NameRefClass::PolicyQualifiedColumnTable => {
            let on_table_path = name_ref.syntax().ancestors().find_map(|n| {
                if let Some(create_policy) = ast::CreatePolicy::cast(n.clone()) {
                    create_policy.on_table()?.path()
                } else if let Some(alter_policy) = ast::AlterPolicy::cast(n) {
                    alter_policy.on_table()?.path()
                } else {
                    None
                }
            })?;
            let (schema, table_name) = name::schema_and_name_path(&on_table_path)?;
            let position = name_ref.syntax().text_range().start();
            let schemas = binder.resolved_schemas(position, schema.as_ref());
            let ptr = resolve_table_name_ptr(db, &table_name, &schemas, file)?;
            Some(smallvec![Location::new(
                file,
                ptr.text_range(),
                LocationKind::Table
            )])
        }
        NameRefClass::LikeTable => {
            let like_clause = name_ref
                .syntax()
                .ancestors()
                .find_map(ast::LikeClause::cast)?;
            let path = like_clause.path()?;
            let (schema, table_name) = name::schema_and_name_path(&path)?;
            let position = name_ref.syntax().text_range().start();
            let schemas = binder.resolved_schemas(position, schema.as_ref());
            if let Some(ptr) = resolve_table_name_ptr(db, &table_name, &schemas, file) {
                return Some(smallvec![Location::new(
                    file,
                    ptr.text_range(),
                    LocationKind::Table
                )]);
            }
            if let Some(ptr) = resolve_view_name_ptr(db, &table_name, &schemas, file) {
                return Some(smallvec![Location::new(
                    file,
                    ptr.text_range(),
                    LocationKind::View
                )]);
            }
            None
        }
        NameRefClass::Function => {
            let function_sig = name_ref
                .syntax()
                .ancestors()
                .find_map(ast::HasParamList::cast)?;
            let path = function_sig.path()?;
            let (schema, function_name) = name::schema_and_name_path(&path)?;
            let params = param_signature(&function_sig);
            let position = name_ref.syntax().text_range().start();
            let schemas = binder.resolved_schemas(position, schema.as_ref());
            resolve_function(db, &function_name, &schemas, params.as_deref(), file)
        }
        NameRefClass::Aggregate => {
            let aggregate = name_ref
                .syntax()
                .ancestors()
                .find_map(ast::HasParamList::cast)?;
            let path = aggregate.path()?;
            let (schema, aggregate_name) = name::schema_and_name_path(&path)?;
            let params = param_signature(&aggregate);
            let position = name_ref.syntax().text_range().start();
            let schemas = binder.resolved_schemas(position, schema.as_ref());
            resolve_aggregate(db, &aggregate_name, &schemas, params.as_deref(), file)
        }
        NameRefClass::Procedure => {
            let function_sig = name_ref
                .syntax()
                .ancestors()
                .find_map(ast::HasParamList::cast)?;
            let path = function_sig.path()?;
            let (schema, procedure_name) = name::schema_and_name_path(&path)?;
            let params = param_signature(&function_sig);
            let position = name_ref.syntax().text_range().start();
            let schemas = binder.resolved_schemas(position, schema.as_ref());
            resolve_procedure(db, &procedure_name, &schemas, params.as_deref(), file)
        }
        NameRefClass::Routine => {
            let function_sig = name_ref
                .syntax()
                .ancestors()
                .find_map(ast::HasParamList::cast)?;
            let path = function_sig.path()?;
            let (schema, routine_name) = name::schema_and_name_path(&path)?;
            let params = param_signature(&function_sig);
            let position = name_ref.syntax().text_range().start();
            let schemas = binder.resolved_schemas(position, schema.as_ref());

            if let Some(results) =
                resolve_function(db, &routine_name, &schemas, params.as_deref(), file)
            {
                return Some(results);
            }

            if let Some(results) =
                resolve_aggregate(db, &routine_name, &schemas, params.as_deref(), file)
            {
                return Some(results);
            }

            resolve_procedure(db, &routine_name, &schemas, params.as_deref(), file)
        }
        NameRefClass::CallProcedure => {
            let call = name_ref.syntax().ancestors().find_map(ast::Call::cast)?;
            let path = call.path()?;
            let (schema, procedure_name) = name::schema_and_name_path(&path)?;
            let position = name_ref.syntax().text_range().start();
            let schemas = binder.resolved_schemas(position, schema.as_ref());
            resolve_procedure(db, &procedure_name, &schemas, None, file)
        }
        NameRefClass::Schema => {
            let schema_name = Name::from_node(name_ref);
            let binder = bind(db, file);
            let ptr = binder.lookup(&schema_name, SymbolKind::Schema)?;
            Some(smallvec![Location::new(
                file,
                ptr.text_range(),
                LocationKind::Schema
            )])
        }
        NameRefClass::PrivilegeObjectTable => {
            let (schema, table_name) = name::schema_and_name(name_ref);
            let position = name_ref.syntax().text_range().start();
            let schemas = binder.resolved_schemas(position, schema.as_ref());
            let (ptr, kind) = resolve_table_like(db, Some(name_ref), &table_name, &schemas, file)?;
            Some(smallvec![Location::new(file, ptr.text_range(), kind)])
        }
        NameRefClass::PrivilegeColumn => {
            let column_name = Name::from_node(name_ref);
            let privilege_objects = name_ref.syntax().ancestors().find_map(|a| {
                ast::Grant::cast(a.clone())
                    .and_then(|grant| grant.privilege_objects())
                    .or_else(|| ast::Revoke::cast(a).and_then(|revoke| revoke.privilege_objects()))
            })?;
            let path = privilege_objects
                .syntax()
                .children()
                .find_map(ast::Path::cast)?;
            resolve_column_for_path(db, InFile::new(file, &path), column_name)
        }
        NameRefClass::FunctionCall => {
            let (schema, function_name) = name::schema_and_name(name_ref);
            let position = name_ref.syntax().text_range().start();
            let schemas = binder.resolved_schemas(position, schema.as_ref());
            resolve_function(db, &function_name, &schemas, None, file)
        }
        NameRefClass::ProcedureCall => {
            let (schema, procedure_name) = name::schema_and_name(name_ref);
            let position = name_ref.syntax().text_range().start();
            let schemas = binder.resolved_schemas(position, schema.as_ref());
            resolve_procedure(db, &procedure_name, &schemas, None, file)
        }
        NameRefClass::FunctionName => {
            let path_type = name_ref
                .syntax()
                .ancestors()
                .find_map(ast::PathType::cast)?;
            let path = path_type.path()?;
            let (schema, function_name) = name::schema_and_name_path(&path)?;
            let position = name_ref.syntax().text_range().start();
            let schemas = binder.resolved_schemas(position, schema.as_ref());
            resolve_function(db, &function_name, &schemas, None, file)
        }
        NameRefClass::SelectFunctionCall => {
            let (schema, function_name) = name::schema_and_name(name_ref);
            let position = name_ref.syntax().text_range().start();
            let schemas = binder.resolved_schemas(position, schema.as_ref());

            // functions take precedence
            if let Some(results) = resolve_function(db, &function_name, &schemas, None, file) {
                return Some(results);
            }

            // aggregates take precedence over function-call-style column access
            if let Some(results) = resolve_aggregate(db, &function_name, &schemas, None, file) {
                return Some(results);
            }

            // if no function found, check if this is function-call-style column access
            // ```sql
            // create table t(a int, b int);
            // select a(t) from t;
            // ```
            if schema.is_none()
                && let Some(ptr) = resolve_fn_call_column(db, InFile::new(file, name_ref))
            {
                return Some(ptr);
            }

            None
        }
        NameRefClass::CreateIndexColumn => {
            resolve_create_index_column_ptr(db, InFile::new(file, name_ref))
        }
        NameRefClass::SelectColumn => resolve_select_column_ptr(db, InFile::new(file, name_ref)),
        NameRefClass::SelectGroupByAliasOrColumn => {
            resolve_select_group_by_alias_or_column_ptr(db, InFile::new(file, name_ref))
        }
        NameRefClass::SelectOrderByAliasOrColumn => {
            resolve_select_order_by_alias_or_column_ptr(db, InFile::new(file, name_ref))
        }
        NameRefClass::SelectQualifiedColumnTable => {
            resolve_select_qualified_column_table_name_ptr(db, InFile::new(file, name_ref))
        }
        NameRefClass::SelectQualifiedColumn => {
            resolve_select_qualified_column_ptr(db, InFile::new(file, name_ref))
        }
        NameRefClass::CompositeTypeField => {
            resolve_composite_type_field_ptr(db, InFile::new(file, name_ref))
        }
        NameRefClass::InsertColumn => {
            let column_name = Name::from_node(name_ref);
            let insert = name_ref.syntax().ancestors().find_map(ast::Insert::cast)?;
            let path = insert.path()?;
            resolve_column_for_path(db, InFile::new(file, &path), column_name)
        }
        NameRefClass::InsertQualifiedColumnTable => {
            let insert = name_ref.syntax().ancestors().find_map(ast::Insert::cast)?;
            let path = insert.path()?;
            resolve_table_in_returning_clause(
                db,
                InFile::new(file, name_ref),
                insert.alias(),
                &path,
                insert.returning_clause(),
            )
        }
        NameRefClass::DeleteColumn => resolve_delete_column_ptr(db, InFile::new(file, name_ref)),
        NameRefClass::DeleteQualifiedColumnTable => {
            resolve_delete_table_name_ptr(db, InFile::new(file, name_ref))
        }
        NameRefClass::UpdateColumn => resolve_update_column_ptr(db, InFile::new(file, name_ref)),
        NameRefClass::UpdateQualifiedColumnTable => {
            resolve_update_table_name_ptr(db, InFile::new(file, name_ref))
        }
        NameRefClass::MergeColumn => resolve_merge_column_ptr(db, InFile::new(file, name_ref)),
        NameRefClass::MergeQualifiedColumnTable => {
            resolve_merge_table_name_ptr(db, InFile::new(file, name_ref))
        }
        NameRefClass::JoinUsingColumn => {
            let join_expr = name_ref
                .syntax()
                .ancestors()
                .find_map(ast::JoinExpr::cast)?;

            let mut results: SmallVec<[Location; 1]> = SmallVec::new();
            for from_item in ast_nav::iter_join_expr(&join_expr) {
                if let Some(locations) =
                    resolve_from_item_column_ptr(db, InFile::new(file, &from_item), name_ref)
                {
                    results.extend(locations);
                }
            }
            (!results.is_empty()).then_some(results)
        }
        NameRefClass::PropertyGraphColumn => {
            resolve_property_graph_column_ptr(db, InFile::new(file, name_ref))
        }
        NameRefClass::AlterColumn => {
            let column_name = Name::from_node(name_ref);
            let alter_table = name_ref
                .syntax()
                .ancestors()
                .find_map(ast::AlterTable::cast)?;
            let table_path = alter_table.relation_name()?.path()?;
            resolve_column_for_path(db, InFile::new(file, &table_path), column_name)
        }
    }
    .or_else(|| resolve_special_keyword_as_function(db, InFile::new(file, name_ref)))
}

fn resolve_table_name_ptr(
    db: &dyn Db,
    table_name: &Name,
    schemas: &ResolvedSchemas,
    file: File,
) -> Option<SyntaxNodePtr> {
    bind(db, file).lookup_with(table_name, SymbolKind::Table, schemas)
}

fn resolve_type_name_ptr(
    db: &dyn Db,
    type_name: &Name,
    schemas: &ResolvedSchemas,
    file: File,
) -> Option<SyntaxNodePtr> {
    let binder = bind(db, file);
    if let Some(ptr) = binder.lookup_with(type_name, SymbolKind::Type, schemas) {
        return Some(ptr);
    }
    // We only want to fallback from bigint to int8 and similar when
    // unqualified.
    if schemas.unqualified()
        && let Some(fallback_name) = fallback_type_alias(type_name)
    {
        return binder.lookup_with(&fallback_name, SymbolKind::Type, schemas);
    }
    None
}

pub(crate) fn resolve_type_ptr_from_type(
    db: &dyn Db,
    ty: InFile<&ast::Type>,
) -> Option<SyntaxNodePtr> {
    let position = ty.value.syntax().text_range().start();
    let (schema, type_name) = name::schema_and_type_name(ty.value)?;
    let schemas = bind(db, ty.file_id).resolved_schemas(position, schema.as_ref());
    resolve_type_name_ptr(db, &type_name, &schemas, ty.file_id)
}

fn fallback_type_alias(type_name: &Name) -> Option<Name> {
    match type_name.0.as_str() {
        "bigint" | "bigserial" | "serial8" => Some(Name::from_string("int8")),
        "boolean" => Some(Name::from_string("bool")),
        "dec" | "decimal" => Some(Name::from_string("numeric")),
        "float" => Some(Name::from_string("float8")),
        "int" | "integer" | "serial" | "serial4" => Some(Name::from_string("int4")),
        "real" => Some(Name::from_string("float4")),
        "smallint" | "smallserial" | "serial2" => Some(Name::from_string("int2")),
        _ => None,
    }
}

fn resolve_float_precision(name_ref: &ast::NameRef, type_name: Name) -> Name {
    if type_name.0.as_str() == "float"
        && let Some(ast::Expr::Literal(lit)) = name_ref
            .syntax()
            .ancestors()
            .find_map(ast::PathType::cast)
            .and_then(|x| x.arg_list()?.args().next()?.expr())
    {
        let precision: u32 = lit.syntax().text().to_string().parse().unwrap_or(0);
        return Name::from_string(if precision <= 24 { "float4" } else { "float8" });
    }
    type_name
}

fn resolve_view_name_ptr(
    db: &dyn Db,
    view_name: &Name,
    schemas: &ResolvedSchemas,
    file: File,
) -> Option<SyntaxNodePtr> {
    bind(db, file).lookup_with(view_name, SymbolKind::View, schemas)
}

pub(crate) fn resolve_table_like(
    db: &dyn Db,
    name_ref: Option<&ast::NameRef>,
    table_name: &Name,
    schemas: &ResolvedSchemas,
    file: File,
) -> Option<(SyntaxNodePtr, LocationKind)> {
    if schemas.unqualified()
        && let Some(name_ref) = name_ref
        && let Some(cte_ptr) = resolve_cte_table(name_ref, table_name)
    {
        return Some((cte_ptr, LocationKind::Table));
    }

    if let Some(view_name_ptr) = resolve_view_name_ptr(db, table_name, schemas, file) {
        return Some((view_name_ptr, LocationKind::View));
    }

    if let Some(table_name_ptr) = resolve_table_name_ptr(db, table_name, schemas, file) {
        return Some((table_name_ptr, LocationKind::Table));
    }

    None
}

fn resolve_constraint(
    db: &dyn Db,
    name_ref: InFile<&ast::NameRef>,
) -> Option<SmallVec<[Location; 1]>> {
    let file = name_ref.file_id;
    let name_ref = name_ref.value;
    let constraint_name = Name::from_node(name_ref);
    let position = name_ref.syntax().text_range().start();
    let (schema, owner_name) = constraint_owner(name_ref)?;
    let binder = bind(db, file);
    let schemas = binder.resolved_schemas(position, schema.as_ref());
    let ptr = match owner_name {
        Some(owner_name) => binder.lookup_with_table(
            &constraint_name,
            SymbolKind::Constraint,
            &schemas,
            &Some(owner_name),
        ),
        None => binder.lookup_with(&constraint_name, SymbolKind::Constraint, &schemas),
    }?;

    Some(smallvec![Location::new(
        file,
        ptr.text_range(),
        LocationKind::Constraint,
    )])
}

fn constraint_owner(name_ref: &ast::NameRef) -> Option<(Option<Schema>, Option<Name>)> {
    let mut fallback_schema = None;

    for ancestor in name_ref.syntax().ancestors() {
        if let Some(path) = ast::Path::cast(ancestor.clone()) {
            fallback_schema = name::schema_name(&path);
        }

        if let Some(alter_table) = ast::AlterTable::cast(ancestor.clone()) {
            let path = alter_table.relation_name()?.path()?;
            let (schema, table_name) = name::schema_and_name_path(&path)?;
            return Some((schema, Some(table_name)));
        }

        if let Some(alter_domain) = ast::AlterDomain::cast(ancestor.clone()) {
            let (schema, domain_name) = name::schema_and_name_path(&alter_domain.path()?)?;
            return Some((schema, Some(domain_name)));
        }

        if let Some(comment_constraint) = ast::ObjectConstraint::cast(ancestor.clone()) {
            let (schema, owner_name) = name::schema_and_name_path(&comment_constraint.path()?)?;
            return Some((schema, Some(owner_name)));
        }

        if let Some(insert) = ast::Insert::cast(ancestor) {
            let (schema, table_name) = name::schema_and_name_path(&insert.path()?)?;
            return Some((schema, Some(table_name)));
        }
    }

    Some((fallback_schema, None))
}

fn resolve_for_kind_with_params(
    db: &dyn Db,
    name: &Name,
    schemas: &ResolvedSchemas,
    params: Option<&[Name]>,
    file: File,
    kind: SymbolKind,
) -> Option<SyntaxNodePtr> {
    bind(db, file).lookup_with_params(name, kind, schemas, params)
}

// some keywords behave as functions
fn resolve_special_keyword_as_function(
    db: &dyn Db,
    name_ref: InFile<&ast::NameRef>,
) -> Option<SmallVec<[Location; 1]>> {
    let function_name = name_ref
        .value
        .syntax()
        .first_child_or_token()
        .and_then(|t| match t.kind() {
            SyntaxKind::CURRENT_SCHEMA_KW => Some("current_schema"),
            SyntaxKind::CURRENT_TIMESTAMP_KW => Some("now"),
            SyntaxKind::CURRENT_USER_KW | SyntaxKind::USER_KW => Some("current_user"),
            SyntaxKind::SESSION_USER_KW => Some("session_user"),
            _ => None,
        })?;
    let function_name = Name::from_string(function_name);
    let position = name_ref.value.syntax().text_range().start();
    let schemas = bind(db, name_ref.file_id).resolved_schemas(position, None);
    resolve_function(db, &function_name, &schemas, None, name_ref.file_id)
}

fn resolve_function(
    db: &dyn Db,
    function_name: &Name,
    schemas: &ResolvedSchemas,
    params: Option<&[Name]>,
    file: File,
) -> Option<SmallVec<[Location; 1]>> {
    let ptr = resolve_for_kind_with_params(
        db,
        function_name,
        schemas,
        params,
        file,
        SymbolKind::Function,
    )?;
    Some(smallvec![Location::new(
        file,
        ptr.text_range(),
        LocationKind::Function
    )])
}

fn resolve_aggregate(
    db: &dyn Db,
    aggregate_name: &Name,
    schemas: &ResolvedSchemas,
    params: Option<&[Name]>,
    file: File,
) -> Option<SmallVec<[Location; 1]>> {
    let ptr = resolve_for_kind_with_params(
        db,
        aggregate_name,
        schemas,
        params,
        file,
        SymbolKind::Aggregate,
    )?;
    Some(smallvec![Location::new(
        file,
        ptr.text_range(),
        LocationKind::Aggregate
    )])
}

fn resolve_procedure(
    db: &dyn Db,
    procedure_name: &Name,
    schemas: &ResolvedSchemas,
    params: Option<&[Name]>,
    file: File,
) -> Option<SmallVec<[Location; 1]>> {
    let ptr = resolve_for_kind_with_params(
        db,
        procedure_name,
        schemas,
        params,
        file,
        SymbolKind::Procedure,
    )?;
    Some(smallvec![Location::new(
        file,
        ptr.text_range(),
        LocationKind::Procedure
    )])
}

fn resolve_create_index_column_ptr(
    db: &dyn Db,
    column_name_ref: InFile<&ast::NameRef>,
) -> Option<SmallVec<[Location; 1]>> {
    let file = column_name_ref.file_id;
    let column_name_ref = column_name_ref.value;
    let column_name = Name::from_node(column_name_ref);

    let create_index = column_name_ref
        .syntax()
        .ancestors()
        .find_map(ast::CreateIndex::cast)?;
    let path = create_index.relation_name()?.path()?;

    resolve_column_for_path(db, InFile::new(file, &path), column_name)
}

fn resolve_property_graph_column_ptr(
    db: &dyn Db,
    column_name_ref: InFile<&ast::NameRef>,
) -> Option<SmallVec<[Location; 1]>> {
    let file = column_name_ref.file_id;
    let column_name_ref = column_name_ref.value;
    let column_name = Name::from_node(column_name_ref);
    let parent = column_name_ref.syntax().parent()?;

    if let Some(column_ref) = ast::ColumnRef::cast(parent.clone())
        && let Some(column_list) = ast::ColumnRefList::cast(column_ref.syntax().parent()?)
    {
        if let Some(references_table) = ast::ReferencesTable::cast(column_list.syntax().parent()?) {
            let table_name = Name::from_node(&references_table.name_ref()?);
            let position = column_name_ref.syntax().text_range().start();
            let schemas = bind(db, file).resolved_schemas(position, None);
            return resolve_column_for_table(db, &table_name, &schemas, &column_name, file);
        } else if let Some(edge_table_def) = column_list
            .syntax()
            .ancestors()
            .find_map(ast::EdgeTableDef::cast)
        {
            return resolve_column_for_path(
                db,
                InFile::new(file, &edge_table_def.path()?),
                column_name,
            );
        } else if let Some(vertex_table_def) =
            ast::VertexTableDef::cast(column_list.syntax().parent()?)
        {
            return resolve_column_for_path(
                db,
                InFile::new(file, &vertex_table_def.path()?),
                column_name,
            );
        }
    } else if let Some(expr_as_name) = ast::ExprAsName::cast(parent)
        && let Some(expr_as_name_list) = ast::ExprAsNameList::cast(expr_as_name.syntax().parent()?)
        && let Some(properties) = ast::Properties::cast(expr_as_name_list.syntax().parent()?)
    {
        let parent = properties.syntax().parent()?;
        if let Some(edge) = ast::EdgeTableDef::cast(parent.clone()) {
            return resolve_column_for_path(db, InFile::new(file, &edge.path()?), column_name);
        } else if let Some(vertex) = ast::VertexTableDef::cast(parent) {
            return resolve_column_for_path(db, InFile::new(file, &vertex.path()?), column_name);
        }
    }

    None
}

fn resolve_column_for_path(
    db: &dyn Db,
    path: InFile<&ast::Path>,
    column_name: Name,
) -> Option<SmallVec<[Location; 1]>> {
    let file = path.file_id;
    let path = path.value;
    let (schema, table_name) = name::schema_and_name_path(path)?;
    let position = path.syntax().text_range().start();
    let schemas = bind(db, file).resolved_schemas(position, schema.as_ref());
    resolve_column_for_table(db, &table_name, &schemas, &column_name, file)
}

fn resolve_select_qualified_column_table_name_ptr(
    db: &dyn Db,
    table_name_ref: InFile<&ast::NameRef>,
) -> Option<SmallVec<[Location; 1]>> {
    let file = table_name_ref.file_id;
    let table_name_ref = table_name_ref.value;
    let table_name = Name::from_node(table_name_ref);

    let field_expr = table_name_ref
        .syntax()
        .parent()
        .and_then(ast::FieldExpr::cast)?;

    let explicit_schema = if field_expr
        .field()
        .is_some_and(|f| f.syntax() == table_name_ref.syntax())
        && field_expr.star_token().is_none()
        && let Some(ast::Expr::NameRef(schema_name_ref)) = field_expr.base()
    {
        // `foo.bar` where table_name_ref is `bar`
        Some(Schema(Name::from_node(&schema_name_ref)))
    } else if let Some(ast::Expr::FieldExpr(inner_field_expr)) = field_expr.base()
        && let Some(ast::Expr::NameRef(schema_name_ref)) = inner_field_expr.base()
    {
        // `foo.buzz.bar` where table_name_ref is `buzz`
        Some(Schema(Name::from_node(&schema_name_ref)))
    } else {
        None
    };

    let from_item = find_from_item_for_select_qualified_name_ref(table_name_ref, &table_name)?;

    if let Some(alias_name) = from_item.alias().and_then(|a| a.name())
        && Name::from_node(&alias_name) == table_name
    {
        return Some(smallvec![Location::new(
            file,
            alias_name.syntax().text_range(),
            LocationKind::Table
        )]);
    }

    if let ast::FromItem::FunctionFromItem(func) = &from_item
        && let Some(call_expr) = func.call_expr()
        && let Some((function_schema, function_name)) = name::schema_and_func_name(&call_expr)
        && function_name == table_name
        && function_schema == explicit_schema
    {
        let position = table_name_ref.syntax().text_range().start();
        let schemas = bind(db, file).resolved_schemas(position, function_schema.as_ref());
        return resolve_function(db, &function_name, &schemas, None, file);
    }

    let (schema, table_name) = if let Some(schema) = explicit_schema {
        (Some(schema), table_name)
    } else {
        name::schema_and_table_from_from_item(&from_item)?
    };
    let position = table_name_ref.syntax().text_range().start();
    let schemas = bind(db, file).resolved_schemas(position, schema.as_ref());
    let (ptr, kind) = resolve_table_like(db, Some(table_name_ref), &table_name, &schemas, file)?;
    Some(smallvec![Location::new(file, ptr.text_range(), kind)])
}

enum ReturningClauseMatch {
    ReturningAlias(ast::Name),
    TableAlias(ast::Name),
    PseudoTable,
    Table,
}

fn match_table_in_returning_clause(
    table_name: &Name,
    stmt_table_name: &Name,
    alias: Option<&ast::Alias>,
    returning_clause: Option<&ast::ReturningClause>,
) -> Option<ReturningClauseMatch> {
    // Check `returning with (old as alias, new as alias)`
    if let Some(option_list) = returning_clause.and_then(|x| x.returning_option_list()) {
        for option in option_list.returning_options() {
            if let Some(name) = option.name()
                && Name::from_node(&name) == *table_name
            {
                return Some(ReturningClauseMatch::ReturningAlias(name));
            }
        }
    }

    let alias_name = alias.and_then(|x| x.name());
    if let Some(alias_name) = &alias_name
        && Name::from_node(alias_name) == *table_name
    {
        return Some(ReturningClauseMatch::TableAlias(alias_name.clone()));
    }

    let old_name = Name::from_string("old");
    let new_name = Name::from_string("new");
    if *table_name == old_name || *table_name == new_name {
        return Some(ReturningClauseMatch::PseudoTable);
    }

    if alias_name.is_none() && *stmt_table_name == *table_name {
        return Some(ReturningClauseMatch::Table);
    }

    None
}

fn resolve_select_qualified_column_ptr(
    db: &dyn Db,
    column_name_ref: InFile<&ast::NameRef>,
) -> Option<SmallVec<[Location; 1]>> {
    let file = column_name_ref.file_id;
    let column_name_ref = column_name_ref.value;
    let column_name = Name::from_node(column_name_ref);

    let field_expr = column_name_ref
        .syntax()
        .parent()
        .and_then(ast::FieldExpr::cast)?;

    let (explicit_schema, column_table_name) = name::schema_and_table_from_field_expr(&field_expr)?;

    let position = column_name_ref.syntax().text_range().start();

    let (schema, mut table_name) = if let Some(schema) = explicit_schema {
        (Some(schema), column_table_name)
    } else {
        match ast_nav::node_parent_query(column_name_ref.syntax())? {
            ast_nav::ParentQuery::Select(_select) => {
                let from_item = find_from_item_for_select_qualified_name_ref(
                    column_name_ref,
                    &column_table_name,
                )?;

                if let ast::FromItem::FunctionFromItem(func) = &from_item
                    && let Some(call_expr) = func.call_expr()
                    && let Some(ptr) = resolve_column_from_call_expr_return_table(
                        db,
                        InFile::new(file, &call_expr),
                        column_name_ref,
                        &column_name,
                        0,
                    )
                {
                    return Some(ptr);
                }

                // `from t as u`
                // `from t as u(a, b, c)`
                if let Some(alias) = from_item.alias()
                    && let Some(alias_name) = alias.name()
                    && Name::from_node(&alias_name) == column_table_name
                {
                    if let ast::FromItem::ParenFromItem(paren) = &from_item {
                        if let Some(paren_select) = paren.paren_select() {
                            return resolve_subquery_column_ptr(
                                db,
                                InFile::new(file, &paren_select),
                                column_name_ref,
                                &column_name,
                                Some(&alias),
                            );
                        }

                        if let Some(paren_expr) = paren.paren_expr() {
                            return resolve_column_from_paren_expr(
                                db,
                                InFile::new(file, &paren_expr),
                                column_name_ref,
                                &column_name,
                            );
                        }
                    }

                    // `from t as u(a, b, c)`
                    if let Some(column_list) = alias.column_list() {
                        for column in column_list.columns() {
                            if let Some(col_name) = column.name()
                                && Name::from_node(&col_name) == column_name
                            {
                                return Some(smallvec![Location::new(
                                    file,
                                    col_name.syntax().text_range(),
                                    LocationKind::Column
                                )]);
                            }
                        }

                        // ```sql
                        // create table t(a int, b int);
                        // select b from t as u(x);
                        //        ^
                        // ```
                        if let ast::FromItem::RelationFromItem(relation) = &from_item
                            && let Some(name_ref_node) = relation.name_ref()
                        {
                            let cte_name = Name::from_node(&name_ref_node);
                            return resolve_cte_column(
                                db,
                                InFile::new(file, column_name_ref),
                                &cte_name,
                                &column_name,
                            );
                        }
                    }
                }
                name::schema_and_table_from_from_item(&from_item)?
            }
            ast_nav::ParentQuery::Update(update) => {
                let path = update.relation_name()?.path()?;
                name::schema_and_name_path(&path)?
            }
            ast_nav::ParentQuery::Delete(delete) => {
                let path = delete.relation_name()?.path()?;
                name::schema_and_name_path(&path)?
            }
            ast_nav::ParentQuery::Insert(insert) => {
                let path = insert.path()?;
                name::schema_and_name_path(&path)?
            }
            ast_nav::ParentQuery::Merge(merge) => {
                // When the qualifier refers to the USING source (by alias or by
                // relation name), resolve the column against that source. This
                // handles subquery and VALUES sources, where the qualifier is not
                // a real table name.
                if let Some(using_on) = merge.using_on_clause()
                    && let Some(from_item) = using_on.from_item()
                {
                    let matches_source =
                        if let Some(alias_name) = from_item.alias().and_then(|x| x.name()) {
                            Name::from_node(&alias_name) == column_table_name
                        } else if let Some((_, item_name)) =
                            name::schema_and_table_from_from_item(&from_item)
                        {
                            item_name == column_table_name
                        } else {
                            false
                        };

                    if matches_source {
                        return resolve_from_item_column_ptr(
                            db,
                            InFile::new(file, &from_item),
                            column_name_ref,
                        );
                    }
                }

                let path = merge.relation_name()?.path()?;
                name::schema_and_name_path(&path)?
            }
        }
    };

    if schema.is_none() {
        if resolve_cte_table(column_name_ref, &table_name).is_some() {
            if let Some(cte_column_ptr) = resolve_cte_column(
                db,
                InFile::new(file, column_name_ref),
                &table_name,
                &column_name,
            ) {
                return Some(cte_column_ptr);
            }
            return None;
        }
        if let Some(alias_table_name) = resolve_merge_alias(column_name_ref, &table_name) {
            table_name = alias_table_name;
        }
    }

    let schemas = bind(db, file).resolved_schemas(position, schema.as_ref());
    resolve_column_for_table(db, &table_name, &schemas, &column_name, file)
}

fn resolve_column_for_table(
    db: &dyn Db,
    table_name: &Name,
    schemas: &ResolvedSchemas,
    column_name: &Name,
    origin_file: File,
) -> Option<SmallVec<[Location; 1]>> {
    let resolved = resolve_table_name(db, table_name, schemas, origin_file)?;
    let file = resolved.file_id;
    match resolved.value {
        ResolvedTableName::View(create_view) => {
            if let Some(ptr) = find_column_in_create_view_like(db, file, &create_view, column_name)
            {
                return Some(ptr);
            }
            return resolve_function(db, column_name, schemas, None, file);
        }
        ResolvedTableName::Table(create_table_like) => {
            // 1. Try to find a matching column (columns take precedence)
            if let Some(ptr) =
                find_column_in_create_table(db, InFile::new(file, &create_table_like), column_name)
            {
                return Some(ptr);
            }
            // 2. No column found, check for field-style function call
            // e.g., select t.b from t where b is a function that takes t as an argument
            return resolve_function(db, column_name, schemas, None, file);
        }
        ResolvedTableName::TableAs(create_table_as) => {
            if let Some(ptr) =
                find_column_in_create_table_as(db, InFile::new(file, &create_table_as), column_name)
            {
                return Some(ptr);
            }
            return resolve_function(db, column_name, schemas, None, file);
        }
        ResolvedTableName::SelectInto(select_into) => {
            if let Some(ptr) = find_column_in_select_into(db, file, &select_into, column_name) {
                return Some(ptr);
            }
            return resolve_function(db, column_name, schemas, None, file);
        }
    }
}

pub(crate) enum ResolvedTableName {
    Table(ast::CreateTableLike),
    TableAs(ast::CreateTableAs),
    SelectInto(ast::SelectInto),
    View(ast::CreateViewLike),
}
// TODO: basically goto def for a table name, we should refactor into an ast_nav
// + goto def setup
pub(crate) fn resolve_table_name(
    db: &dyn Db,
    table_name: &Name,
    schemas: &ResolvedSchemas,
    origin_file: File,
) -> Option<InFile<ResolvedTableName>> {
    use ResolvedTableName::*;
    for resolved_schema in schemas.list() {
        // A little clunky
        let single = ResolvedSchemas::from_single(resolved_schema.clone());
        for file in list_files(db, origin_file) {
            let Some((ptr, kind)) = resolve_table_like(db, None, table_name, &single, file) else {
                continue;
            };
            let tree = parse(db, file).tree();
            let node = ptr.to_node(tree.syntax());
            match kind {
                LocationKind::Table => {
                    if let Some(create_table) =
                        node.ancestors().find_map(ast::CreateTableLike::cast)
                    {
                        return Some(InFile::new(file, Table(create_table)));
                    }
                    if let Some(create_table_as) =
                        node.ancestors().find_map(ast::CreateTableAs::cast)
                    {
                        return Some(InFile::new(file, TableAs(create_table_as)));
                    }
                    if let Some(select_into) = node.ancestors().find_map(ast::SelectInto::cast) {
                        return Some(InFile::new(file, SelectInto(select_into)));
                    }
                }
                LocationKind::View => {
                    if let Some(view) = node.ancestors().find_map(ast::CreateViewLike::cast) {
                        return Some(InFile::new(file, View(view)));
                    }
                }
                _ => (),
            }
        }
    }
    None
}

fn resolve_merge_alias(name_ref: &ast::NameRef, table_name: &Name) -> Option<Name> {
    let from_item = name_ref.syntax().ancestors().find_map(|x| {
        ast::Merge::cast(x)?
            .using_on_clause()
            .and_then(|c| c.from_item())
    })?;
    if let Some(alias_name) = from_item.alias().and_then(|x| x.name())
        && Name::from_node(&alias_name) == *table_name
        && let ast::FromItem::RelationFromItem(relation) = &from_item
    {
        let table_name = Name::from_node(&relation.name_ref()?);
        return Some(table_name);
    }
    None
}

fn resolve_from_item_column_ptr(
    db: &dyn Db,
    from_item: InFile<&ast::FromItem>,
    column_name_ref: &ast::NameRef,
) -> Option<SmallVec<[Location; 1]>> {
    let column_name = Name::from_node(column_name_ref);
    resolve_from_item_column_by_name(db, from_item, column_name_ref, &column_name)
}

fn resolve_from_item_column_by_name(
    db: &dyn Db,
    from_item: InFile<&ast::FromItem>,
    scope_name_ref: &ast::NameRef,
    column_name: &Name,
) -> Option<SmallVec<[Location; 1]>> {
    resolve_from_item_column_by_name_after_index(db, from_item, scope_name_ref, column_name, 0)
}

fn resolve_from_item_column_by_name_after_index(
    db: &dyn Db,
    from_item: InFile<&ast::FromItem>,
    scope_name_ref: &ast::NameRef,
    column_name: &Name,
    skip_column_count: usize,
) -> Option<SmallVec<[Location; 1]>> {
    let file = from_item.file_id;
    let from_item = from_item.value;
    let original_skip = skip_column_count;

    if let ast::FromItem::ParenFromItem(paren) = from_item
        && let Some(paren_select) = paren.paren_select()
    {
        let alias = from_item.alias();
        if let Some(ptr) = resolve_subquery_column_ptr_with_skip(
            db,
            InFile::new(file, &paren_select),
            scope_name_ref,
            column_name,
            alias.as_ref(),
            skip_column_count,
        ) {
            return Some(ptr);
        }
        if original_skip == 0
            && let Some(alias_name) = alias.and_then(|x| x.name())
            && Name::from_node(&alias_name) == *column_name
        {
            return Some(smallvec![Location::new(
                file,
                alias_name.syntax().text_range(),
                LocationKind::Table
            )]);
        }
        return None;
    }

    if let ast::FromItem::ParenFromItem(paren) = from_item
        && let Some(paren_expr) = paren.paren_expr()
    {
        let (alias_len, alias_column) = resolve_column_list_column(
            file,
            from_item.alias().and_then(|x| x.column_list()),
            column_name,
            skip_column_count,
        );
        if let Some(alias_column) = alias_column {
            return Some(alias_column);
        }
        let skip_column_count = skip_column_count.max(alias_len);

        if let Some(ptr) = resolve_column_from_paren_expr_with_skip(
            db,
            InFile::new(file, &paren_expr),
            scope_name_ref,
            column_name,
            skip_column_count,
        ) {
            return Some(ptr);
        }
        if original_skip == 0
            && let Some(alias_name) = from_item.alias().and_then(|x| x.name())
            && Name::from_node(&alias_name) == *column_name
        {
            return Some(smallvec![Location::new(
                file,
                alias_name.syntax().text_range(),
                LocationKind::Table
            )]);
        }
        return None;
    }

    if let Some(select_variant) = from_item
        .syntax()
        .children()
        .find_map(ast::SelectVariant::cast)
    {
        return resolve_column_from_select_variant_with_skip(
            db,
            file,
            select_variant,
            scope_name_ref,
            column_name,
            skip_column_count,
        );
    }

    let (alias_len, alias_column) = resolve_column_list_column(
        file,
        from_item.alias().and_then(|x| x.column_list()),
        column_name,
        skip_column_count,
    );
    if let Some(alias_column) = alias_column {
        return Some(alias_column);
    }
    let skip_column_count = skip_column_count.max(alias_len);

    if let ast::FromItem::FunctionFromItem(func) = from_item
        && let Some(call_expr) = func.call_expr()
        && let Some(ptr) = resolve_column_from_call_expr_return_table(
            db,
            InFile::new(file, &call_expr),
            scope_name_ref,
            column_name,
            skip_column_count,
        )
    {
        return Some(ptr);
    }

    if let ast::FromItem::RowsFromItem(rows_from) = from_item {
        for call_expr in rows_from.call_exprs() {
            if let Some(ptr) = resolve_column_from_call_expr_return_table(
                db,
                InFile::new(file, &call_expr),
                scope_name_ref,
                column_name,
                skip_column_count,
            ) {
                return Some(ptr);
            }
        }
    }

    let (schema, table_name) = name::schema_and_table_from_from_item(from_item)?;
    let scope_name_ref = relation_name_ref_from_from_item(from_item)?;

    if let Some(ptr) = resolve_column_from_table_or_view_or_cte_impl(
        db,
        InFile::new(file, &scope_name_ref),
        &table_name,
        schema.as_ref(),
        column_name,
        0,
        skip_column_count,
    ) {
        return Some(ptr);
    }

    if original_skip == 0
        && let Some(alias_name) = from_item.alias().and_then(|x| x.name())
        && Name::from_node(&alias_name) == *column_name
    {
        return Some(smallvec![Location::new(
            file,
            alias_name.syntax().text_range(),
            LocationKind::Table
        )]);
    }

    None
}

fn resolve_column_from_table_or_view_or_cte(
    db: &dyn Db,
    name_ref: InFile<&ast::NameRef>,
    table_name: &Name,
    schema: Option<&Schema>,
    column_name: &Name,
) -> Option<SmallVec<[Location; 1]>> {
    resolve_column_from_table_or_view_or_cte_impl(
        db,
        name_ref,
        table_name,
        schema,
        column_name,
        0,
        0,
    )
}

fn resolve_column_from_table_or_view_or_cte_impl(
    db: &dyn Db,
    name_ref: InFile<&ast::NameRef>,
    table_name: &Name,
    schema: Option<&Schema>,
    column_name: &Name,
    depth: u32,
    skip_column_count: usize,
) -> Option<SmallVec<[Location; 1]>> {
    resolve_column_from_table_or_view_or_cte_impl_inner(
        db,
        name_ref,
        table_name,
        schema,
        column_name,
        depth,
        skip_column_count,
        true,
    )
}

fn resolve_column_from_table_or_view_or_cte_column_only(
    db: &dyn Db,
    name_ref: InFile<&ast::NameRef>,
    table_name: &Name,
    schema: Option<&Schema>,
    column_name: &Name,
    skip_column_count: usize,
) -> Option<SmallVec<[Location; 1]>> {
    resolve_column_from_table_or_view_or_cte_impl_inner(
        db,
        name_ref,
        table_name,
        schema,
        column_name,
        0,
        skip_column_count,
        false,
    )
}

fn resolve_column_from_table_or_view_or_cte_impl_inner(
    db: &dyn Db,
    name_ref: InFile<&ast::NameRef>,
    table_name: &Name,
    schema: Option<&Schema>,
    column_name: &Name,
    depth: u32,
    skip_column_count: usize,
    allow_whole_row_fallback: bool,
) -> Option<SmallVec<[Location; 1]>> {
    let file = name_ref.file_id;
    let name_ref = name_ref.value;
    if depth > 40 {
        log::info!("max resolve depth reached, probably in a cycle");
        return None;
    }

    let position = name_ref.syntax().text_range().start();
    let schemas = bind(db, file).resolved_schemas(position, schema);
    let (table_like_ptr, kind) =
        resolve_table_like(db, Some(name_ref), table_name, &schemas, file)?;

    match kind {
        LocationKind::Table => {
            if schemas.unqualified() && resolve_cte_table(name_ref, table_name).is_some() {
                return resolve_cte_column_with_skip_impl(
                    db,
                    InFile::new(file, name_ref),
                    table_name,
                    column_name,
                    skip_column_count,
                    allow_whole_row_fallback,
                );
            }

            let tree = parse(db, file).tree();
            let root = tree.syntax();
            let node = table_like_ptr.to_node(root);

            if let Some(create_table) = node.ancestors().find_map(ast::CreateTableLike::cast) {
                if let Some(cols) = find_column_in_create_table_impl(
                    db,
                    InFile::new(file, &create_table),
                    column_name,
                    0,
                    skip_column_count,
                ) {
                    return Some(cols);
                }

                // check if this is a partitioned table
                if skip_column_count == 0
                    && let Some(parent_path) = ast::CreateTable::cast(create_table.syntax().clone())
                        .and_then(|x| x.partition_of())
                        .and_then(|x| x.path())
                {
                    let (parent_schema, parent_table_name) =
                        name::schema_and_name_path(&parent_path)?;
                    return resolve_column_from_table_or_view_or_cte_impl_inner(
                        db,
                        InFile::new(file, name_ref),
                        &parent_table_name,
                        parent_schema.as_ref(),
                        column_name,
                        depth + 1,
                        skip_column_count,
                        allow_whole_row_fallback,
                    );
                }

                // For example, in:
                // ```sql
                // create table t(a int);
                // select t from t;
                // ```
                if allow_whole_row_fallback && skip_column_count == 0 && column_name == table_name {
                    return Some(smallvec![Location::new(
                        file,
                        table_like_ptr.text_range(),
                        LocationKind::Table
                    )]);
                }
            }

            if let Some(create_table_as) = node.ancestors().find_map(ast::CreateTableAs::cast) {
                if let Some(cols) = find_column_in_create_table_as_with_skip(
                    db,
                    InFile::new(file, &create_table_as),
                    column_name,
                    skip_column_count,
                ) {
                    return Some(cols);
                }

                if allow_whole_row_fallback && skip_column_count == 0 && column_name == table_name {
                    return Some(smallvec![Location::new(
                        file,
                        table_like_ptr.text_range(),
                        LocationKind::Table
                    )]);
                }
            }

            if let Some(select_into) = node.ancestors().find_map(ast::SelectInto::cast) {
                if let Some(cols) = find_column_in_select_into_with_skip(
                    db,
                    file,
                    &select_into,
                    column_name,
                    skip_column_count,
                ) {
                    return Some(cols);
                }

                if allow_whole_row_fallback && skip_column_count == 0 && column_name == table_name {
                    return Some(smallvec![Location::new(
                        file,
                        table_like_ptr.text_range(),
                        LocationKind::Table
                    )]);
                }
            }

            None
        }
        LocationKind::View => {
            let tree = parse(db, file).tree();
            let root = tree.syntax();
            let node = table_like_ptr.to_node(root);

            if let Some(create_view) = node.ancestors().find_map(ast::CreateViewLike::cast) {
                if let Some(cols) = find_column_in_create_view_like_with_skip(
                    db,
                    file,
                    &create_view,
                    column_name,
                    skip_column_count,
                ) {
                    return Some(cols);
                }

                if allow_whole_row_fallback && skip_column_count == 0 && column_name == table_name {
                    return Some(smallvec![Location::new(
                        file,
                        table_like_ptr.text_range(),
                        LocationKind::View
                    )]);
                }
            }

            None
        }
        _ => None,
    }
}

fn resolve_from_item_for_cte_star(
    db: &dyn Db,
    from_item: InFile<&ast::FromItem>,
    name_ref: &ast::NameRef,
    cte_name: &Name,
    column_name: &Name,
    skip_column_count: usize,
) -> Option<SmallVec<[Location; 1]>> {
    let file = from_item.file_id;
    if let Some((schema, table_name)) = name::schema_and_table_from_from_item(from_item.value)
        && table_name == *cte_name
    {
        let scope_name_ref = relation_name_ref_from_from_item(from_item.value)?;
        return resolve_column_from_table_or_view_or_cte_impl(
            db,
            InFile::new(file, &scope_name_ref),
            &table_name,
            schema.as_ref(),
            column_name,
            0,
            skip_column_count,
        );
    }

    resolve_from_item_column_by_name_after_index(
        db,
        from_item,
        name_ref,
        column_name,
        skip_column_count,
    )
}

fn resolve_select_column_ptr(
    db: &dyn Db,
    column_name_ref: InFile<&ast::NameRef>,
) -> Option<SmallVec<[Location; 1]>> {
    let file = column_name_ref.file_id;
    let column_name_ref = column_name_ref.value;

    // Walk up through enclosing selects so a column in a correlated subquery
    // resolves against an outer query's from clause:
    // `select (select (select a)) from foo`
    for select in column_name_ref
        .syntax()
        .ancestors()
        .filter_map(ast::Select::cast)
    {
        let Some(from_clause) = select.from_clause() else {
            continue;
        };
        // In the case of ambiguous columns, we'll have multiple matches.
        // They're an error, but we'll report that elsewhere.
        let mut results: SmallVec<[Location; 1]> = SmallVec::new();
        for from_item in ast_nav::iter_from_clause(&from_clause) {
            if let Some(column_ptr) =
                resolve_from_item_column_ptr(db, InFile::new(file, &from_item), column_name_ref)
            {
                results.extend(column_ptr);
            }
        }
        if !results.is_empty() {
            return Some(results);
        }
    }

    // A correlated subquery can reference the target relation of an enclosing
    // DML statement, e.g. `update foo set a = (select b)` where `b` is `foo.b`
    let in_file = InFile::new(file, column_name_ref);
    for ancestor in column_name_ref.syntax().ancestors() {
        match ancestor.kind() {
            SyntaxKind::UPDATE => return resolve_update_column_ptr(db, in_file),
            SyntaxKind::DELETE => return resolve_delete_column_ptr(db, in_file),
            SyntaxKind::MERGE => return resolve_merge_column_ptr(db, in_file),
            _ => (),
        }
    }

    resolve_enclosing_function_param(in_file)
}

fn resolve_enclosing_function_param(
    name_ref: InFile<&ast::NameRef>,
) -> Option<SmallVec<[Location; 1]>> {
    let file = name_ref.file_id;
    let name_ref = name_ref.value;
    let param_name = Name::from_node(name_ref);

    for ancestor in name_ref.syntax().ancestors() {
        let Some(has_param_list) = ast::HasParamList::cast(ancestor) else {
            continue;
        };
        let Some(param_list) = has_param_list.param_list() else {
            continue;
        };
        for param in param_list.params() {
            if let Some(name) = param.name()
                && Name::from_node(&name) == param_name
            {
                return Some(smallvec![Location::new(
                    file,
                    name.syntax().text_range(),
                    LocationKind::NamedArgParameter
                )]);
            }
        }
    }

    None
}

fn resolve_select_group_by_alias_or_column_ptr(
    db: &dyn Db,
    column_name_ref: InFile<&ast::NameRef>,
) -> Option<SmallVec<[Location; 1]>> {
    if let Some(ptr) = resolve_select_column_ptr(db, column_name_ref) {
        return Some(ptr);
    }
    resolve_select_target_alias_ptr(column_name_ref)
}

fn resolve_select_order_by_alias_or_column_ptr(
    db: &dyn Db,
    column_name_ref: InFile<&ast::NameRef>,
) -> Option<SmallVec<[Location; 1]>> {
    if let Some(compound_select) = compound_select_for_order_by_name_ref(column_name_ref.value) {
        return resolve_compound_select_order_by_column_ptr(db, column_name_ref, &compound_select);
    }

    if let Some(ptr) = resolve_select_target_alias_ptr(column_name_ref) {
        return Some(ptr);
    }
    resolve_select_column_ptr(db, column_name_ref)
}

fn compound_select_for_order_by_name_ref(name_ref: &ast::NameRef) -> Option<ast::CompoundSelect> {
    let order_by_clause = name_ref
        .syntax()
        .ancestors()
        .find_map(ast::OrderByClause::cast)?;
    order_by_clause
        .syntax()
        .parent()
        .and_then(ast::CompoundSelect::cast)
}

fn resolve_compound_select_order_by_column_ptr(
    db: &dyn Db,
    column_name_ref: InFile<&ast::NameRef>,
    compound_select: &ast::CompoundSelect,
) -> Option<SmallVec<[Location; 1]>> {
    let file = column_name_ref.file_id;
    let column_name_ref = column_name_ref.value;
    let column_name = Name::from_node(column_name_ref);
    resolve_column_from_select_variant_with_skip(
        db,
        file,
        compound_select.lhs()?,
        column_name_ref,
        &column_name,
        0,
    )
}

fn resolve_column_from_select_variant_with_skip(
    db: &dyn Db,
    file: File,
    select_variant: ast::SelectVariant,
    name_ref: &ast::NameRef,
    column_name: &Name,
    skip_column_count: usize,
) -> Option<SmallVec<[Location; 1]>> {
    match select_variant {
        ast::SelectVariant::CompoundSelect(compound_select) => {
            resolve_column_from_select_variant_with_skip(
                db,
                file,
                compound_select.lhs()?,
                name_ref,
                column_name,
                skip_column_count,
            )
        }
        ast::SelectVariant::ParenSelect(paren_select) => resolve_subquery_column_ptr_with_skip(
            db,
            InFile::new(file, &paren_select),
            name_ref,
            column_name,
            None,
            skip_column_count,
        ),
        ast::SelectVariant::Select(select) => resolve_column_from_select_targets(
            db,
            InFile::new(file, &select),
            name_ref,
            column_name,
            skip_column_count,
        ),
        ast::SelectVariant::SelectInto(select_into) => find_column_in_select_into_with_skip(
            db,
            file,
            &select_into,
            column_name,
            skip_column_count,
        ),
        ast::SelectVariant::Table(table) => resolve_column_from_table_query_with_skip(
            db,
            file,
            &table,
            column_name,
            skip_column_count,
        ),
        ast::SelectVariant::Values(values) => {
            resolve_values_column_after_index(file, &values, column_name, skip_column_count)
        }
    }
}

fn resolve_column_from_table_query_with_skip(
    db: &dyn Db,
    file: File,
    table: &ast::Table,
    column_name: &Name,
    skip_column_count: usize,
) -> Option<SmallVec<[Location; 1]>> {
    let path = table.relation_name()?.path()?;
    let (schema, table_name) = name::schema_and_name_path(&path)?;
    let table_name_ref = relation_name_ref_from_table(table)?;

    resolve_column_from_table_or_view_or_cte_column_only(
        db,
        InFile::new(file, &table_name_ref),
        &table_name,
        schema.as_ref(),
        column_name,
        skip_column_count,
    )
}

fn resolve_select_target_alias_ptr(
    column_name_ref: InFile<&ast::NameRef>,
) -> Option<SmallVec<[Location; 1]>> {
    let file = column_name_ref.file_id;
    let column_name_ref = column_name_ref.value;
    let select = column_name_ref
        .syntax()
        .ancestors()
        .find_map(ast::Select::cast)?;
    let column_name = Name::from_node(column_name_ref);
    let target_list = select.select_clause()?.target_list()?;
    for target in target_list.targets() {
        if let Some((target_name, node)) = ColumnName::from_target(target)
            && let Some(target_name) = target_name.to_string()
            && Name::from_string(target_name) == column_name
        {
            return Some(smallvec![Location::new(
                file,
                node.text_range(),
                LocationKind::Column,
            )]);
        }
    }
    None
}

fn resolve_fn_call_column(
    db: &dyn Db,
    name_ref: InFile<&ast::NameRef>,
) -> Option<SmallVec<[Location; 1]>> {
    let file = name_ref.file_id;
    let name_ref = name_ref.value;
    let column_name = Name::from_node(name_ref);

    // function call syntax for columns is only valid if there is one argument
    let call_expr = name_ref
        .syntax()
        .ancestors()
        .find_map(ast::CallExpr::cast)?;
    let arg_count = call_expr.arg_list()?.args().count();
    if arg_count != 1 {
        return None;
    }

    let select = name_ref.syntax().ancestors().find_map(ast::Select::cast)?;
    let from_clause = select.from_clause()?;

    for from_item in ast_nav::iter_from_clause(&from_clause) {
        if let Some((schema, table_name)) = name::schema_and_table_from_from_item(&from_item)
            && let Some(scope_name_ref) = relation_name_ref_from_from_item(&from_item)
            && let Some(result) = resolve_column_from_table_or_view_or_cte(
                db,
                InFile::new(file, &scope_name_ref),
                &table_name,
                schema.as_ref(),
                &column_name,
            )
        {
            return Some(result);
        }
    }

    None
}

fn is_from_item_match(from_item: &ast::FromItem, qualifier: &Name) -> bool {
    if let Some(alias_name) = from_item.alias().and_then(|a| a.name()) {
        return Name::from_node(&alias_name) == *qualifier;
    }

    match from_item {
        ast::FromItem::FunctionFromItem(func) => func
            .call_expr()
            .and_then(|x| name::schema_and_func_name(&x))
            .is_some_and(|(_schema, function_name)| function_name == *qualifier),
        ast::FromItem::RelationFromItem(relation) => {
            if let Some(name_ref) = relation.name_ref() {
                Name::from_node(&name_ref) == *qualifier
            } else if let Some(field) = relation.field_expr().and_then(|x| x.field()) {
                Name::from_node(&field) == *qualifier
            } else {
                false
            }
        }
        _ => false,
    }
}

pub(crate) fn find_from_item_in_from_clause(
    from_clause: &ast::FromClause,
    qualifier: &Name,
) -> Option<ast::FromItem> {
    ast_nav::iter_from_clause(from_clause)
        .find(|from_item| is_from_item_match(from_item, qualifier))
}

fn find_from_item_for_select_qualified_name_ref(
    name_ref: &ast::NameRef,
    table_name: &Name,
) -> Option<ast::FromItem> {
    let select = name_ref.syntax().ancestors().find_map(ast::Select::cast)?;

    if let Some(from_clause) = select.from_clause()
        && let Some(from_item) = find_from_item_in_from_clause(&from_clause, table_name)
    {
        return Some(from_item);
    }

    if let Some(lateral_from_item) = name_ref.syntax().ancestors().find_map(|ancestor| {
        ast::FromItem::cast(ancestor).filter(|from_item| {
            from_item
                .syntax()
                .children_with_tokens()
                .any(|it| it.kind() == SyntaxKind::LATERAL_KW)
        })
    }) {
        let lateral_start = lateral_from_item.syntax().text_range().start();

        for ancestor in lateral_from_item.syntax().ancestors() {
            if let Some(from_clause) = ast::Select::cast(ancestor).and_then(|x| x.from_clause())
                && let Some(outer_from_item) = ast_nav::iter_from_clause(&from_clause)
                    .filter(|item| item.syntax().text_range().start() < lateral_start)
                    .find(|item| is_from_item_match(item, table_name))
            {
                return Some(outer_from_item);
            }
        }
    }

    let inner_select_start = select.syntax().text_range().start();
    for ancestor in select.syntax().ancestors().skip(1) {
        if let Some(outer_from_clause) = ast::Select::cast(ancestor).and_then(|x| x.from_clause()) {
            if outer_from_clause
                .syntax()
                .text_range()
                .contains(inner_select_start)
            {
                continue;
            }
            if let Some(from_item) = find_from_item_in_from_clause(&outer_from_clause, table_name) {
                return Some(from_item);
            }
        }
    }

    None
}

pub(crate) fn find_column_in_create_table(
    db: &dyn Db,
    create_table: InFile<&impl ast::HasCreateTable>,
    column_name: &Name,
) -> Option<SmallVec<[Location; 1]>> {
    find_column_in_create_table_impl(db, create_table, column_name, 0, 0)
}

fn find_column_in_create_table_impl(
    db: &dyn Db,
    create_table: InFile<&impl ast::HasCreateTable>,
    column_name: &Name,
    depth: usize,
    skip_column_count: usize,
) -> Option<SmallVec<[Location; 1]>> {
    let file = create_table.file_id;
    let create_table = create_table.value;
    if depth > 40 {
        log::info!("max depth reached, probably in a cycle");
        return None;
    }

    let mut column_index = 0usize;
    for arg in ast_nav::create_table_args(create_table) {
        match arg {
            ast_nav::CreateTableArg::Inherits(path) => {
                if skip_column_count == 0 {
                    let (schema, table_name) = name::schema_and_name_path(&path)?;
                    let position = path.syntax().text_range().start();
                    let schemas = bind(db, file).resolved_schemas(position, schema.as_ref());
                    if let Some(resolved) = resolve_table_name(db, &table_name, &schemas, file)
                        && let Some(ptr) =
                            find_column_in_resolved(db, resolved, column_name, depth + 1)
                    {
                        return Some(ptr);
                    }
                }
            }
            ast_nav::CreateTableArg::Column(column) => {
                if column_index >= skip_column_count
                    && let Some(name) = column.name()
                    && Name::from_node(&name) == *column_name
                {
                    return Some(smallvec![Location::new(
                        file,
                        name.syntax().text_range(),
                        LocationKind::Column
                    )]);
                }
                column_index = column_index.saturating_add(1);
            }
            ast_nav::CreateTableArg::LikeClause(like_clause) => {
                if skip_column_count == 0 {
                    let path = like_clause.path()?;
                    let (schema, table_name) = name::schema_and_name_path(&path)?;
                    let position = path.syntax().text_range().start();
                    let schemas = bind(db, file).resolved_schemas(position, schema.as_ref());
                    if let Some(resolved) = resolve_table_name(db, &table_name, &schemas, file)
                        && let Some(ptr) =
                            find_column_in_resolved(db, resolved, column_name, depth + 1)
                    {
                        return Some(ptr);
                    }
                }
            }
            ast_nav::CreateTableArg::TableConstraint(_) => (),
        }
    }

    None
}

fn find_column_in_resolved(
    db: &dyn Db,
    resolved: InFile<ResolvedTableName>,
    column_name: &Name,
    depth: usize,
) -> Option<SmallVec<[Location; 1]>> {
    let file = resolved.file_id;
    match resolved.value {
        ResolvedTableName::Table(parent_table) => find_column_in_create_table_impl(
            db,
            InFile::new(file, &parent_table),
            column_name,
            depth,
            0,
        ),
        ResolvedTableName::TableAs(create_table_as) => {
            find_column_in_create_table_as(db, InFile::new(file, &create_table_as), column_name)
        }
        ResolvedTableName::SelectInto(select_into) => {
            find_column_in_select_into(db, file, &select_into, column_name)
        }
        ResolvedTableName::View(create_view) => {
            find_column_in_create_view_like(db, file, &create_view, column_name)
        }
    }
}

fn resolve_column_list_column(
    file: File,
    column_list: Option<ast::ColumnList>,
    column_name: &Name,
    skip_column_count: usize,
) -> (usize, Option<SmallVec<[Location; 1]>>) {
    let Some(column_list) = column_list else {
        return (0, None);
    };

    let mut column_list_len = 0usize;
    for (idx, column) in column_list.columns().enumerate() {
        column_list_len = idx + 1;
        if idx >= skip_column_count
            && let Some(col_name) = column.name()
            && Name::from_node(&col_name) == *column_name
        {
            return (
                column_list_len,
                Some(smallvec![Location::new(
                    file,
                    col_name.syntax().text_range(),
                    LocationKind::Column
                )]),
            );
        }
    }

    (column_list_len, None)
}

fn resolve_values_column_after_index(
    file: File,
    values: &ast::Values,
    column_name: &Name,
    skip_column_count: usize,
) -> Option<SmallVec<[Location; 1]>> {
    for (idx, (col_name, expr)) in ast_nav::iter_values_columns(values).enumerate() {
        if idx >= skip_column_count && col_name == *column_name {
            return Some(smallvec![Location::new(
                file,
                expr.syntax().text_range(),
                LocationKind::Column
            )]);
        }
    }
    None
}

fn resolve_column_from_targets(
    db: &dyn Db,
    file: File,
    target_list: &ast::TargetList,
    from_clause: Option<&ast::FromClause>,
    column_name: &Name,
    skip_column_count: usize,
    mut resolve_star: impl FnMut(
        InFile<&ast::FromClause>,
        &Name,
        usize,
    ) -> Option<SmallVec<[Location; 1]>>,
    mut resolve_qualified_star: impl FnMut(
        InFile<&ast::FromClause>,
        &Name,
        &Name,
        usize,
    ) -> Option<SmallVec<[Location; 1]>>,
) -> Option<SmallVec<[Location; 1]>> {
    let mut column_index = 0usize;

    for target in target_list.targets() {
        let target_column_count = from_clause
            .and_then(|from_clause| {
                count_columns_for_target(db, InFile::new(file, &target), from_clause)
            })
            .unwrap_or(1);
        let column_list_end = column_index.saturating_add(target_column_count);
        if column_list_end <= skip_column_count {
            column_index = column_list_end;
            continue;
        }

        let target_skip = skip_column_count.saturating_sub(column_index);
        if let Some((col_name, node)) = ColumnName::from_target(target.clone()) {
            if let Some(col_name_str) = col_name.to_string()
                && column_index >= skip_column_count
                && Name::from_string(col_name_str) == *column_name
            {
                return Some(smallvec![Location::new(
                    file,
                    node.text_range(),
                    LocationKind::Column
                )]);
            }

            if matches!(col_name, ColumnName::Star)
                && let Some(from_clause) = from_clause
                && let Some(result) =
                    resolve_star(InFile::new(file, from_clause), column_name, target_skip)
            {
                return Some(result);
            }
        }

        if let Some(ast::Expr::FieldExpr(field_expr)) = target.expr()
            && let Some(table_name) = qualified_star_table_name(&field_expr)
            && let Some(from_clause) = from_clause
            && let Some(result) = resolve_qualified_star(
                InFile::new(file, from_clause),
                column_name,
                &table_name,
                target_skip,
            )
        {
            return Some(result);
        }

        column_index = column_list_end;
    }

    None
}

fn find_column_in_target_list_with_skip(
    db: &dyn Db,
    file: File,
    target_list: &ast::TargetList,
    from_clause: Option<&ast::FromClause>,
    column_name: &Name,
    skip_column_count: usize,
) -> Option<SmallVec<[Location; 1]>> {
    resolve_column_from_targets(
        db,
        file,
        target_list,
        from_clause,
        column_name,
        skip_column_count,
        |from_clause, column_name, skip_column_count| {
            find_column_in_from_clause_with_skip(db, from_clause, column_name, skip_column_count)
        },
        |from_clause, column_name, table_name, skip_column_count| {
            find_column_in_qualified_from_clause_with_skip(
                db,
                from_clause,
                column_name,
                table_name,
                skip_column_count,
            )
        },
    )
}

fn find_column_in_select_variant_with_skip(
    db: &dyn Db,
    file: File,
    select_variant: ast::SelectVariant,
    column_name: &Name,
    skip_column_count: usize,
) -> Option<SmallVec<[Location; 1]>> {
    match select_variant {
        ast::SelectVariant::CompoundSelect(compound_select) => {
            find_column_in_select_variant_with_skip(
                db,
                file,
                compound_select.lhs()?,
                column_name,
                skip_column_count,
            )
        }
        ast::SelectVariant::ParenSelect(paren_select) => find_column_in_select_variant_with_skip(
            db,
            file,
            paren_select.select()?,
            column_name,
            skip_column_count,
        ),
        ast::SelectVariant::Select(select) => {
            let target_list = select.select_clause()?.target_list()?;
            let from_clause = select.from_clause();
            find_column_in_target_list_with_skip(
                db,
                file,
                &target_list,
                from_clause.as_ref(),
                column_name,
                skip_column_count,
            )
        }
        ast::SelectVariant::SelectInto(select_into) => find_column_in_select_into_with_skip(
            db,
            file,
            &select_into,
            column_name,
            skip_column_count,
        ),
        ast::SelectVariant::Table(table) => resolve_column_from_table_query_with_skip(
            db,
            file,
            &table,
            column_name,
            skip_column_count,
        ),
        ast::SelectVariant::Values(values) => {
            resolve_values_column_after_index(file, &values, column_name, skip_column_count)
        }
    }
}

// TODO: this is similar to the CTE funcs, maybe we can simplify
fn find_column_in_create_view_like(
    db: &dyn Db,
    file: File,
    create_view: &ast::CreateViewLike,
    column_name: &Name,
) -> Option<SmallVec<[Location; 1]>> {
    find_column_in_create_view_like_with_skip(db, file, create_view, column_name, 0)
}

fn find_column_in_create_view_like_with_skip(
    db: &dyn Db,
    file: File,
    create_view: &ast::CreateViewLike,
    column_name: &Name,
    skip_column_count: usize,
) -> Option<SmallVec<[Location; 1]>> {
    let (column_list_len, alias_column) = resolve_column_list_column(
        file,
        create_view.column_list(),
        column_name,
        skip_column_count,
    );
    if let Some(alias_column) = alias_column {
        return Some(alias_column);
    }
    let skip_column_count = skip_column_count.max(column_list_len);

    find_column_in_select_variant_with_skip(
        db,
        file,
        create_view.query()?,
        column_name,
        skip_column_count,
    )
}

fn find_column_in_create_table_as(
    db: &dyn Db,
    create_table_as: InFile<&ast::CreateTableAs>,
    column_name: &Name,
) -> Option<SmallVec<[Location; 1]>> {
    find_column_in_create_table_as_with_skip(db, create_table_as, column_name, 0)
}

fn find_column_in_create_table_as_with_skip(
    db: &dyn Db,
    create_table_as: InFile<&ast::CreateTableAs>,
    column_name: &Name,
    skip_column_count: usize,
) -> Option<SmallVec<[Location; 1]>> {
    let file = create_table_as.file_id;
    let create_table_as = create_table_as.value;
    find_column_in_select_variant_with_skip(
        db,
        file,
        create_table_as.query()?.select_variant()?,
        column_name,
        skip_column_count,
    )
}

fn find_column_in_from_clause_with_skip(
    db: &dyn Db,
    from_clause: InFile<&ast::FromClause>,
    column_name: &Name,
    skip_column_count: usize,
) -> Option<SmallVec<[Location; 1]>> {
    let file = from_clause.file_id;
    let from_clause = from_clause.value;
    let mut column_index = 0usize;
    for from_item in ast_nav::iter_from_clause(from_clause) {
        let item_skip = skip_column_count.saturating_sub(column_index);
        if let Some(count) = count_columns_for_from_item(db, InFile::new(file, &from_item)) {
            if item_skip >= count {
                column_index = column_index.saturating_add(count);
                continue;
            }
        }

        let Some((schema, table_name)) = name::schema_and_table_from_from_item(&from_item) else {
            continue;
        };
        let Some(relation_name_ref) = relation_name_ref_from_from_item(&from_item) else {
            continue;
        };
        if let Some(result) = resolve_column_from_table_or_view_or_cte_impl(
            db,
            InFile::new(file, &relation_name_ref),
            &table_name,
            schema.as_ref(),
            column_name,
            0,
            item_skip,
        ) {
            return Some(result);
        }
    }
    None
}

fn find_column_in_qualified_from_clause_with_skip(
    db: &dyn Db,
    from_clause: InFile<&ast::FromClause>,
    column_name: &Name,
    table_name: &Name,
    skip_column_count: usize,
) -> Option<SmallVec<[Location; 1]>> {
    let file = from_clause.file_id;
    let from_item = find_from_item_in_from_clause(from_clause.value, table_name)?;
    let (schema, table_name) = name::schema_and_table_from_from_item(&from_item)?;
    let relation_name_ref = relation_name_ref_from_from_item(&from_item)?;
    resolve_column_from_table_or_view_or_cte_impl(
        db,
        InFile::new(file, &relation_name_ref),
        &table_name,
        schema.as_ref(),
        column_name,
        0,
        skip_column_count,
    )
}

fn find_column_in_select_into(
    db: &dyn Db,
    file: File,
    select_into: &ast::SelectInto,
    column_name: &Name,
) -> Option<SmallVec<[Location; 1]>> {
    find_column_in_select_into_with_skip(db, file, select_into, column_name, 0)
}

fn find_column_in_select_into_with_skip(
    db: &dyn Db,
    file: File,
    select_into: &ast::SelectInto,
    column_name: &Name,
    skip_column_count: usize,
) -> Option<SmallVec<[Location; 1]>> {
    let from_clause = select_into.from_clause();
    let target_list = select_into.select_clause()?.target_list()?;
    find_column_in_target_list_with_skip(
        db,
        file,
        &target_list,
        from_clause.as_ref(),
        column_name,
        skip_column_count,
    )
}

fn relation_name_ref_from_table(table: &ast::Table) -> Option<ast::NameRef> {
    table.relation_name()?.path()?.segment()?.name_ref()
}

fn relation_name_ref_from_from_item(from_item: &ast::FromItem) -> Option<ast::NameRef> {
    let ast::FromItem::RelationFromItem(relation) = from_item else {
        return None;
    };
    relation.name_ref().or_else(|| {
        relation
            .field_expr()
            .and_then(|field_expr| field_expr.field())
    })
}

fn resolve_cte_table(name_ref: &ast::NameRef, cte_name: &Name) -> Option<SyntaxNodePtr> {
    let with_table = ast_nav::find_cte_with_table(name_ref, cte_name)?;
    Some(SyntaxNodePtr::new(with_table.name()?.syntax()))
}

fn count_columns_for_path(db: &dyn Db, path: InFile<&ast::Path>) -> Option<usize> {
    let file = path.file_id;
    let path = path.value;
    let (schema, table_name) = name::schema_and_name_path(path)?;
    let position = path.syntax().text_range().start();
    let schemas = bind(db, file).resolved_schemas(position, schema.as_ref());
    count_columns_for_table_name(db, &table_name, &schemas, file, None)
}

fn count_columns_for_table_name(
    db: &dyn Db,
    table_name: &Name,
    schemas: &ResolvedSchemas,
    file: File,
    name_ref: Option<&ast::NameRef>,
) -> Option<usize> {
    let tree = parse(db, file).tree();
    let root = tree.syntax();
    let (table_like_ptr, kind) = resolve_table_like(db, name_ref, table_name, schemas, file)?;

    match kind {
        LocationKind::Table => {
            if schemas.unqualified()
                && let Some(name_ref) = name_ref
                && let Some(with_table) = ast_nav::find_cte_with_table(name_ref, table_name)
            {
                return count_columns_for_with_table(db, file, with_table);
            }

            let table_like_node = table_like_ptr.to_node(root);
            if let Some(create_table) = table_like_node
                .ancestors()
                .find_map(ast::CreateTableLike::cast)
            {
                let mut count: usize = 0;
                if let Some(args) = create_table.table_arg_list() {
                    for arg in args.args() {
                        if matches!(arg, ast::TableArg::Column(_)) {
                            count = count.saturating_add(1);
                        }
                    }
                }
                return Some(count);
            }

            if let Some(create_table_as) = table_like_node
                .ancestors()
                .find_map(ast::CreateTableAs::cast)
            {
                return count_columns_for_select_variant(
                    db,
                    file,
                    &create_table_as.query()?.select_variant()?,
                );
            }

            if let Some(select_into) = table_like_node.ancestors().find_map(ast::SelectInto::cast) {
                if let Some(target_list) = select_into.select_clause().and_then(|c| c.target_list())
                {
                    return Some(target_list.targets().count());
                }
            }

            None
        }
        LocationKind::View => {
            let table_like_node = table_like_ptr.to_node(root);
            let create_view = table_like_node
                .ancestors()
                .find_map(ast::CreateViewLike::cast)?;
            if let Some(column_list) = create_view.column_list() {
                return Some(column_list.columns().count());
            }

            count_columns_for_select_variant(db, file, &create_view.query()?)
        }
        _ => None,
    }
}

fn count_columns_for_with_table(
    db: &dyn Db,
    file: File,
    with_table: ast::WithTable,
) -> Option<usize> {
    if let Some(column_list) = with_table.column_list() {
        return Some(column_list.columns().count());
    }

    count_columns_for_with_query(db, file, with_table.query()?)
}

fn count_columns_for_with_query(db: &dyn Db, file: File, query: ast::WithQuery) -> Option<usize> {
    match query {
        ast::WithQuery::CompoundSelect(compound_select) => {
            count_columns_for_select_variant(db, file, &compound_select.lhs()?)
        }
        ast::WithQuery::ParenSelect(paren_select) => {
            count_columns_for_select_variant(db, file, &paren_select.select()?)
        }
        ast::WithQuery::Select(select) => Some(count_columns_for_target_list(
            &select.select_clause()?.target_list()?,
        )),
        ast::WithQuery::Table(table) => count_columns_for_table_query(db, file, &table),
        ast::WithQuery::Values(values) => count_columns_for_values(&values),
        ast::WithQuery::Delete(_)
        | ast::WithQuery::Insert(_)
        | ast::WithQuery::Merge(_)
        | ast::WithQuery::Update(_) => None,
    }
}

fn count_columns_for_select_variant(
    db: &dyn Db,
    file: File,
    select_variant: &ast::SelectVariant,
) -> Option<usize> {
    match select_variant {
        ast::SelectVariant::CompoundSelect(compound_select) => {
            count_columns_for_select_variant(db, file, &compound_select.lhs()?)
        }
        ast::SelectVariant::ParenSelect(paren_select) => {
            count_columns_for_select_variant(db, file, &paren_select.select()?)
        }
        ast::SelectVariant::Select(select) => Some(count_columns_for_target_list(
            &select.select_clause()?.target_list()?,
        )),
        ast::SelectVariant::SelectInto(select_into) => Some(count_columns_for_target_list(
            &select_into.select_clause()?.target_list()?,
        )),
        ast::SelectVariant::Table(table) => count_columns_for_table_query(db, file, table),
        ast::SelectVariant::Values(values) => count_columns_for_values(values),
    }
}

fn count_columns_for_target_list(target_list: &ast::TargetList) -> usize {
    target_list.targets().count()
}

fn count_columns_for_table_query(db: &dyn Db, file: File, table: &ast::Table) -> Option<usize> {
    let path = table.relation_name()?.path()?;
    let (schema, table_name) = name::schema_and_name_path(&path)?;
    let table_name_ref = relation_name_ref_from_table(table)?;
    let position = table_name_ref.syntax().text_range().start();
    let schemas = bind(db, file).resolved_schemas(position, schema.as_ref());
    count_columns_for_table_name(db, &table_name, &schemas, file, Some(&table_name_ref))
}

fn count_columns_for_values(values: &ast::Values) -> Option<usize> {
    values
        .row_list()
        .and_then(|x| x.rows().next())
        .map(|row| row.exprs().count())
}

fn resolve_cte_column(
    db: &dyn Db,
    name_ref: InFile<&ast::NameRef>,
    cte_name: &Name,
    column_name: &Name,
) -> Option<SmallVec<[Location; 1]>> {
    resolve_cte_column_with_skip(db, name_ref, cte_name, column_name, 0)
}

fn resolve_cte_column_with_skip(
    db: &dyn Db,
    name_ref: InFile<&ast::NameRef>,
    cte_name: &Name,
    column_name: &Name,
    skip_column_count: usize,
) -> Option<SmallVec<[Location; 1]>> {
    resolve_cte_column_with_skip_impl(db, name_ref, cte_name, column_name, skip_column_count, true)
}

fn resolve_cte_column_with_skip_impl(
    db: &dyn Db,
    name_ref: InFile<&ast::NameRef>,
    cte_name: &Name,
    column_name: &Name,
    skip_column_count: usize,
    allow_whole_row_fallback: bool,
) -> Option<SmallVec<[Location; 1]>> {
    let file = name_ref.file_id;
    let name_ref = name_ref.value;
    let with_table = ast_nav::find_cte_with_table(name_ref, cte_name)?;

    let (column_list_len, alias_column) = resolve_column_list_column(
        file,
        with_table.column_list(),
        column_name,
        skip_column_count,
    );
    if let Some(alias_column) = alias_column {
        return Some(alias_column);
    }
    let skip_column_count = skip_column_count.max(column_list_len);

    let query = with_table.query()?;

    if let Some(column) = column_in_with_query(
        InFile::new(file, &query),
        db,
        column_name,
        skip_column_count,
    ) {
        return Some(column);
    }

    if let Some(column) = resolve_cte_column_from_with_query_with_skip(
        db,
        file,
        query,
        name_ref,
        cte_name,
        column_name,
        skip_column_count,
    ) {
        return Some(column);
    }

    if allow_whole_row_fallback
        && skip_column_count == 0
        && column_name == cte_name
        && let Some(name_node) = with_table.name()
    {
        return Some(smallvec![Location::new(
            file,
            name_node.syntax().text_range(),
            LocationKind::Table
        )]);
    }

    None
}

fn resolve_cte_column_from_with_query_with_skip(
    db: &dyn Db,
    file: File,
    query: ast::WithQuery,
    name_ref: &ast::NameRef,
    cte_name: &Name,
    column_name: &Name,
    skip_column_count: usize,
) -> Option<SmallVec<[Location; 1]>> {
    match query {
        ast::WithQuery::CompoundSelect(compound_select) => {
            resolve_cte_column_from_select_variant_with_skip(
                db,
                file,
                compound_select.lhs()?,
                name_ref,
                cte_name,
                column_name,
                skip_column_count,
            )
        }
        ast::WithQuery::ParenSelect(paren_select) => {
            resolve_cte_column_from_select_variant_with_skip(
                db,
                file,
                paren_select.select()?,
                name_ref,
                cte_name,
                column_name,
                skip_column_count,
            )
        }
        ast::WithQuery::Select(select) => resolve_cte_column_from_select_with_skip(
            db,
            file,
            &select,
            name_ref,
            cte_name,
            column_name,
            skip_column_count,
        ),
        ast::WithQuery::Table(table) => resolve_column_from_table_query_with_skip(
            db,
            file,
            &table,
            column_name,
            skip_column_count,
        ),
        ast::WithQuery::Values(values) => {
            resolve_values_column_after_index(file, &values, column_name, skip_column_count)
        }
        ast::WithQuery::Delete(_)
        | ast::WithQuery::Insert(_)
        | ast::WithQuery::Merge(_)
        | ast::WithQuery::Update(_) => None,
    }
}

fn resolve_cte_column_from_select_variant_with_skip(
    db: &dyn Db,
    file: File,
    select_variant: ast::SelectVariant,
    name_ref: &ast::NameRef,
    cte_name: &Name,
    column_name: &Name,
    skip_column_count: usize,
) -> Option<SmallVec<[Location; 1]>> {
    match select_variant {
        ast::SelectVariant::CompoundSelect(compound_select) => {
            resolve_cte_column_from_select_variant_with_skip(
                db,
                file,
                compound_select.lhs()?,
                name_ref,
                cte_name,
                column_name,
                skip_column_count,
            )
        }
        ast::SelectVariant::ParenSelect(paren_select) => {
            resolve_cte_column_from_select_variant_with_skip(
                db,
                file,
                paren_select.select()?,
                name_ref,
                cte_name,
                column_name,
                skip_column_count,
            )
        }
        ast::SelectVariant::Select(select) => resolve_cte_column_from_select_with_skip(
            db,
            file,
            &select,
            name_ref,
            cte_name,
            column_name,
            skip_column_count,
        ),
        ast::SelectVariant::SelectInto(select_into) => find_column_in_select_into_with_skip(
            db,
            file,
            &select_into,
            column_name,
            skip_column_count,
        ),
        ast::SelectVariant::Table(table) => resolve_column_from_table_query_with_skip(
            db,
            file,
            &table,
            column_name,
            skip_column_count,
        ),
        ast::SelectVariant::Values(values) => {
            resolve_values_column_after_index(file, &values, column_name, skip_column_count)
        }
    }
}

fn resolve_cte_column_from_select_with_skip(
    db: &dyn Db,
    file: File,
    select: &ast::Select,
    name_ref: &ast::NameRef,
    cte_name: &Name,
    column_name: &Name,
    skip_column_count: usize,
) -> Option<SmallVec<[Location; 1]>> {
    let target_list = select.select_clause()?.target_list()?;
    let from_clause = select.from_clause();

    resolve_column_from_targets(
        db,
        file,
        &target_list,
        from_clause.as_ref(),
        column_name,
        skip_column_count,
        |from_clause, column_name, skip_column_count| {
            resolve_from_clause_for_cte_star(
                db,
                from_clause,
                name_ref,
                cte_name,
                column_name,
                skip_column_count,
            )
        },
        |from_clause, column_name, table_name, skip_column_count| {
            resolve_qualified_star_in_from_clause(
                db,
                from_clause,
                name_ref,
                cte_name,
                column_name,
                table_name,
                skip_column_count,
            )
        },
    )
}

fn column_in_with_query(
    query: InFile<&ast::WithQuery>,
    db: &dyn Db,
    column_name: &Name,
    column_list_len: usize,
) -> Option<SmallVec<[Location; 1]>> {
    let file = query.file_id;
    let query = query.value;
    let (returning_clause, alias, path) = match query {
        ast::WithQuery::Delete(delete) => (
            delete.returning_clause(),
            delete.alias(),
            delete.relation_name()?.path()?,
        ),
        ast::WithQuery::Insert(insert) => {
            (insert.returning_clause(), insert.alias(), insert.path()?)
        }
        ast::WithQuery::Merge(merge) => (
            merge.returning_clause(),
            merge.alias(),
            merge.relation_name()?.path()?,
        ),
        ast::WithQuery::Update(update) => (
            update.returning_clause(),
            update.alias(),
            update.relation_name()?.path()?,
        ),
        ast::WithQuery::Select(_)
        | ast::WithQuery::CompoundSelect(_)
        | ast::WithQuery::Table(_)
        | ast::WithQuery::Values(_)
        | ast::WithQuery::ParenSelect(_) => return None,
    };
    let returning_clause = returning_clause?;
    let (_, stmt_table_name) = name::schema_and_name_path(&path)?;

    let mut column_index: usize = 0;
    for target in returning_clause.target_list()?.targets() {
        let target_kind =
            returning_target_kind(&target, &stmt_table_name, alias.as_ref(), &returning_clause);
        let target_column_count = match target_kind {
            ReturningTargetKind::Column => 1,
            ReturningTargetKind::QualifiedStar | ReturningTargetKind::UnqualifiedStar => {
                count_columns_for_path(db, InFile::new(file, &path)).unwrap_or(1)
            }
        };
        let column_list_end = column_index.saturating_add(target_column_count);

        if column_list_end <= column_list_len {
            column_index = column_list_end;
            continue;
        }

        if let Some((col_name, node)) = ColumnName::from_target(target) {
            if let Some(col_name_str) = col_name.to_string()
                && Name::from_string(col_name_str) == *column_name
            {
                return Some(smallvec![Location::new(
                    file,
                    node.text_range(),
                    LocationKind::Column
                )]);
            }
            if matches!(col_name, ColumnName::Star)
                && let Some(ptr) =
                    resolve_column_for_path(db, InFile::new(file, &path), column_name.clone())
            {
                return Some(ptr);
            }
        }
        if matches!(target_kind, ReturningTargetKind::QualifiedStar)
            && let Some(ptr) =
                resolve_column_for_path(db, InFile::new(file, &path), column_name.clone())
        {
            return Some(ptr);
        }
        column_index = column_list_end;
    }

    None
}

#[derive(Clone, Copy)]
enum ReturningTargetKind {
    Column,
    QualifiedStar,
    UnqualifiedStar,
}

fn returning_target_kind(
    target: &ast::Target,
    stmt_table_name: &Name,
    alias: Option<&ast::Alias>,
    returning_clause: &ast::ReturningClause,
) -> ReturningTargetKind {
    if target.star_token().is_some() {
        return ReturningTargetKind::UnqualifiedStar;
    }

    if let Some(ast::Expr::FieldExpr(field_expr)) = target.expr()
        && let Some(table_name) = qualified_star_table_name(&field_expr)
        && match_table_in_returning_clause(
            &table_name,
            stmt_table_name,
            alias,
            Some(returning_clause),
        )
        .is_some()
    {
        return ReturningTargetKind::QualifiedStar;
    }

    ReturningTargetKind::Column
}

fn resolve_subquery_column_ptr(
    db: &dyn Db,
    paren_select: InFile<&ast::ParenSelect>,
    name_ref: &ast::NameRef,
    column_name: &Name,
    alias: Option<&ast::Alias>,
) -> Option<SmallVec<[Location; 1]>> {
    resolve_subquery_column_ptr_with_skip(db, paren_select, name_ref, column_name, alias, 0)
}

fn resolve_subquery_column_ptr_with_skip(
    db: &dyn Db,
    paren_select: InFile<&ast::ParenSelect>,
    name_ref: &ast::NameRef,
    column_name: &Name,
    alias: Option<&ast::Alias>,
    skip_column_count: usize,
) -> Option<SmallVec<[Location; 1]>> {
    let file = paren_select.file_id;
    let paren_select = paren_select.value;
    let select_variant = paren_select.select()?;

    let (column_list_len, alias_column) = resolve_column_list_column(
        file,
        alias.and_then(|x| x.column_list()),
        column_name,
        skip_column_count,
    );
    if let Some(alias_column) = alias_column {
        return Some(alias_column);
    }
    let skip_column_count = skip_column_count.max(column_list_len);

    resolve_column_from_select_variant_with_skip(
        db,
        file,
        select_variant,
        name_ref,
        column_name,
        skip_column_count,
    )
}

fn resolve_column_from_select_targets(
    db: &dyn Db,
    select: InFile<&ast::Select>,
    name_ref: &ast::NameRef,
    column_name: &Name,
    skip_column_count: usize,
) -> Option<SmallVec<[Location; 1]>> {
    let file = select.file_id;
    let select = select.value;
    let target_list = select.select_clause()?.target_list()?;
    let from_clause = select.from_clause();

    resolve_column_from_targets(
        db,
        file,
        &target_list,
        from_clause.as_ref(),
        column_name,
        skip_column_count,
        |from_clause, column_name, skip_column_count| {
            resolve_from_clause_column_after_index(
                db,
                from_clause,
                name_ref,
                column_name,
                skip_column_count,
            )
        },
        |from_clause, column_name, table_name, skip_column_count| {
            let file = from_clause.file_id;
            let from_item = find_from_item_in_from_clause(from_clause.value, table_name)?;
            resolve_from_item_column_by_name_after_index(
                db,
                InFile::new(file, &from_item),
                name_ref,
                column_name,
                skip_column_count,
            )
        },
    )
}

pub(crate) fn qualified_star_table_name(field_expr: &ast::FieldExpr) -> Option<Name> {
    field_expr.star_token()?;

    match field_expr.base()? {
        ast::Expr::NameRef(name_ref) => Some(Name::from_node(&name_ref)),
        ast::Expr::FieldExpr(inner_field_expr) => {
            let field = inner_field_expr.field()?;
            Some(Name::from_node(&field))
        }
        _ => None,
    }
}

// TODO: I don't think we should be passing in file here and using it, we want
// to use the content from the updated file with the completion marker
pub(crate) fn table_ptrs_from_clause(
    db: &dyn Db,
    from_clause: InFile<&ast::FromClause>,
) -> Vec<SyntaxNodePtr> {
    let file = from_clause.file_id;
    let from_clause = from_clause.value;
    let mut results = vec![];

    for from_item in ast_nav::iter_from_clause(from_clause) {
        if let Some(alias) = from_item.alias()
            && alias.column_list().is_some()
        {
            results.push(SyntaxNodePtr::new(alias.syntax()));
            continue;
        }

        if let ast::FromItem::ParenFromItem(paren) = &from_item
            && let Some(paren_select) = paren.paren_select()
        {
            results.push(SyntaxNodePtr::new(paren_select.syntax()));
            continue;
        }

        let Some((schema, table_name)) = name::schema_and_table_from_from_item(&from_item) else {
            continue;
        };

        let name_ref = match &from_item {
            ast::FromItem::RelationFromItem(relation) => relation.name_ref(),
            _ => None,
        };
        let position = from_item.syntax().text_range().start();
        let schemas = bind(db, file).resolved_schemas(position, schema.as_ref());
        if let Some((table_like_ptr, _kind)) =
            resolve_table_like(db, name_ref.as_ref(), &table_name, &schemas, file)
        {
            results.push(table_like_ptr);
        }
    }

    results
}

pub(crate) fn table_ptr_from_from_item(
    db: &dyn Db,
    from_item: InFile<&ast::FromItem>,
) -> Option<SyntaxNodePtr> {
    let file = from_item.file_id;
    let from_item = from_item.value;
    if let ast::FromItem::ParenFromItem(paren) = from_item {
        if let Some(paren_select) = paren.paren_select() {
            return Some(SyntaxNodePtr::new(paren_select.syntax()));
        }
        if let Some(paren_expr) = paren.paren_expr() {
            return table_ptr_from_paren_expr(db, InFile::new(file, &paren_expr));
        }
    }

    let (schema, table_name) = name::schema_and_table_from_from_item(from_item)?;
    let name_ref = match from_item {
        ast::FromItem::RelationFromItem(relation) => relation.name_ref(),
        _ => None,
    };
    let position = from_item.syntax().text_range().start();
    let schemas = bind(db, file).resolved_schemas(position, schema.as_ref());

    resolve_table_like(db, name_ref.as_ref(), &table_name, &schemas, file)
        .map(|(table_like_ptr, _kind)| table_like_ptr)
}

fn table_ptr_from_paren_expr(
    db: &dyn Db,
    paren_expr: InFile<&ast::ParenExpr>,
) -> Option<SyntaxNodePtr> {
    let file = paren_expr.file_id;
    let paren_expr = paren_expr.value;
    if let Some(from_item) = paren_expr.from_item() {
        return table_ptr_from_from_item(db, InFile::new(file, &from_item));
    }
    if let Some(ast::Expr::ParenExpr(inner)) = paren_expr.expr() {
        return table_ptr_from_paren_expr(db, InFile::new(file, &inner));
    }
    None
}

fn count_columns_for_target(
    db: &dyn Db,
    target: InFile<&ast::Target>,
    from_clause: &ast::FromClause,
) -> Option<usize> {
    let file = target.file_id;
    let target = target.value;
    if target.star_token().is_some() {
        return count_columns_for_from_clause(db, InFile::new(file, from_clause));
    }

    if let Some(ast::Expr::FieldExpr(field_expr)) = target.expr()
        && let Some(table_name) = qualified_star_table_name(&field_expr)
        && let Some(from_item) = find_from_item_in_from_clause(from_clause, &table_name)
    {
        return count_columns_for_from_item(db, InFile::new(file, &from_item));
    }

    Some(1)
}

fn count_columns_for_from_clause(
    db: &dyn Db,
    from_clause: InFile<&ast::FromClause>,
) -> Option<usize> {
    let file = from_clause.file_id;
    let from_clause = from_clause.value;
    let mut total: usize = 0;
    let mut found = false;

    for from_item in ast_nav::iter_from_clause(from_clause) {
        if let Some(count) = count_columns_for_from_item(db, InFile::new(file, &from_item)) {
            total = total.saturating_add(count);
            found = true;
        }
    }

    found.then_some(total)
}

fn count_columns_for_from_item(db: &dyn Db, from_item: InFile<&ast::FromItem>) -> Option<usize> {
    let file = from_item.file_id;
    let from_item = from_item.value;
    let (schema, table_name) = name::schema_and_table_from_from_item(from_item)?;
    let scope_name_ref = relation_name_ref_from_from_item(from_item)?;
    let position = scope_name_ref.syntax().text_range().start();
    let schemas = bind(db, file).resolved_schemas(position, schema.as_ref());
    count_columns_for_table_name(db, &table_name, &schemas, file, Some(&scope_name_ref))
}

fn resolve_from_clause_column_after_index(
    db: &dyn Db,
    from_clause: InFile<&ast::FromClause>,
    name_ref: &ast::NameRef,
    column_name: &Name,
    skip_column_count: usize,
) -> Option<SmallVec<[Location; 1]>> {
    let file = from_clause.file_id;
    let mut column_index = 0usize;
    for from_item in ast_nav::iter_from_clause(from_clause.value) {
        let item_skip = skip_column_count.saturating_sub(column_index);
        if let Some(count) = count_columns_for_from_item(db, InFile::new(file, &from_item)) {
            if item_skip >= count {
                column_index = column_index.saturating_add(count);
                continue;
            }
        }

        if let Some(result) = resolve_from_item_column_by_name_after_index(
            db,
            InFile::new(file, &from_item),
            name_ref,
            column_name,
            item_skip,
        ) {
            return Some(result);
        }
    }

    None
}

fn resolve_from_clause_for_cte_star(
    db: &dyn Db,
    from_clause: InFile<&ast::FromClause>,
    name_ref: &ast::NameRef,
    cte_name: &Name,
    column_name: &Name,
    skip_column_count: usize,
) -> Option<SmallVec<[Location; 1]>> {
    let file = from_clause.file_id;
    let mut column_index = 0usize;
    for from_item in ast_nav::iter_from_clause(from_clause.value) {
        let item_skip = skip_column_count.saturating_sub(column_index);
        if let Some(count) = count_columns_for_from_item(db, InFile::new(file, &from_item)) {
            if item_skip >= count {
                column_index = column_index.saturating_add(count);
                continue;
            }
        }

        if let Some(result) = resolve_from_item_for_cte_star(
            db,
            InFile::new(file, &from_item),
            name_ref,
            cte_name,
            column_name,
            item_skip,
        ) {
            return Some(result);
        }
    }

    None
}

fn resolve_qualified_star_in_from_clause(
    db: &dyn Db,
    from_clause: InFile<&ast::FromClause>,
    name_ref: &ast::NameRef,
    cte_name: &Name,
    column_name: &Name,
    table_name: &Name,
    skip_column_count: usize,
) -> Option<SmallVec<[Location; 1]>> {
    let file = from_clause.file_id;
    let from_item = find_from_item_in_from_clause(from_clause.value, table_name)?;
    resolve_from_item_for_cte_star(
        db,
        InFile::new(file, &from_item),
        name_ref,
        cte_name,
        column_name,
        skip_column_count,
    )
}

fn resolve_column_from_paren_expr(
    db: &dyn Db,
    paren_expr: InFile<&ast::ParenExpr>,
    name_ref: &ast::NameRef,
    column_name: &Name,
) -> Option<SmallVec<[Location; 1]>> {
    resolve_column_from_paren_expr_with_skip(db, paren_expr, name_ref, column_name, 0)
}

fn resolve_column_from_paren_expr_with_skip(
    db: &dyn Db,
    paren_expr: InFile<&ast::ParenExpr>,
    name_ref: &ast::NameRef,
    column_name: &Name,
    skip_column_count: usize,
) -> Option<SmallVec<[Location; 1]>> {
    let file = paren_expr.file_id;
    let paren_expr = paren_expr.value;
    if let Some(select) = paren_expr.select() {
        return resolve_column_from_select_targets(
            db,
            InFile::new(file, &select),
            name_ref,
            column_name,
            skip_column_count,
        );
    }

    if let Some(ast::Expr::CallExpr(call_expr)) = paren_expr.expr()
        && let Some(result) = resolve_column_from_call_expr_return_table(
            db,
            InFile::new(file, &call_expr),
            name_ref,
            column_name,
            skip_column_count,
        )
    {
        return Some(result);
    }

    if let Some(ast::Expr::ParenExpr(paren_expr)) = paren_expr.expr() {
        return resolve_column_from_paren_expr_with_skip(
            db,
            InFile::new(file, &paren_expr),
            name_ref,
            column_name,
            skip_column_count,
        );
    }

    if let Some(from_item) = paren_expr.from_item() {
        return resolve_from_item_column_by_name_after_index(
            db,
            InFile::new(file, &from_item),
            name_ref,
            column_name,
            skip_column_count,
        );
    }

    if let Some(join_expr) = paren_expr.join_expr() {
        for from_item in ast_nav::iter_join_expr(&join_expr) {
            if let Some(ptr) = resolve_from_item_column_by_name_after_index(
                db,
                InFile::new(file, &from_item),
                name_ref,
                column_name,
                skip_column_count,
            ) {
                return Some(ptr);
            }
        }
    }

    None
}

fn resolve_column_from_call_expr_return_table(
    db: &dyn Db,
    call_expr: InFile<&ast::CallExpr>,
    name_ref: &ast::NameRef,
    column_name: &Name,
    min_index: usize,
) -> Option<SmallVec<[Location; 1]>> {
    let file = call_expr.file_id;
    let call_expr = call_expr.value;
    let position = name_ref.syntax().text_range().start();
    let (schema, function_name) = name::schema_and_func_name(call_expr)?;
    let schemas = bind(db, file).resolved_schemas(position, schema.as_ref());
    let function_locs = resolve_function(db, &function_name, &schemas, None, file)?;
    let function_node = function_locs.first()?.to_node(db)?;
    let create_function = function_node
        .ancestors()
        .find_map(ast::CreateFunction::cast)?;

    // `returns table(col ...)`
    if let Some(table_arg_list) = create_function.ret_type().and_then(|r| r.table_arg_list()) {
        let mut index = 0usize;
        for arg in table_arg_list.args() {
            if let ast::TableArg::Column(column) = arg {
                if let Some(name) = column.name()
                    && Name::from_node(&name) == *column_name
                    && index >= min_index
                {
                    return Some(smallvec![Location::new(
                        file,
                        name.syntax().text_range(),
                        LocationKind::Column
                    )]);
                }
                index += 1;
            }
        }
    }

    // `out` / `inout` parameters define the result columns
    if let Some(param_list) = create_function.param_list() {
        let mut index = 0usize;
        for param in param_list.params() {
            if !matches!(
                param.mode(),
                Some(ast::ParamMode::ParamInOut(_) | ast::ParamMode::ParamOut(_))
            ) {
                continue;
            }
            if let Some(name) = param.name()
                && Name::from_node(&name) == *column_name
                && index >= min_index
            {
                return Some(smallvec![Location::new(
                    file,
                    name.syntax().text_range(),
                    LocationKind::Column
                )]);
            }
            index += 1;
        }
    }

    // `returns setof <table>` or `returns setof <composite type>`
    if let Some(ast::Type::PathType(path_type)) = create_function.ret_type().and_then(|r| r.ty())
        && let Some(path) = path_type.path()
    {
        if let Some(ptr) =
            resolve_column_for_path(db, InFile::new(file, &path), column_name.clone())
        {
            return Some(ptr);
        }
        if let Some(ptr) =
            resolve_composite_type_field_for_path(db, InFile::new(file, &path), column_name)
        {
            return Some(ptr);
        }
    }

    None
}

pub(crate) fn resolve_table_info(
    db: &dyn Db,
    path: InFile<&ast::Path>,
) -> Option<(Schema, String)> {
    resolve_symbol_info(db, path, SymbolKind::Table)
}

pub(crate) fn resolve_function_info(
    db: &dyn Db,
    path: InFile<&ast::Path>,
) -> Option<(Schema, String)> {
    resolve_symbol_info(db, path, SymbolKind::Function)
}

pub(crate) fn resolve_aggregate_info(
    db: &dyn Db,
    path: InFile<&ast::Path>,
) -> Option<(Schema, String)> {
    resolve_symbol_info(db, path, SymbolKind::Aggregate)
}

pub(crate) fn resolve_procedure_info(
    db: &dyn Db,
    path: InFile<&ast::Path>,
) -> Option<(Schema, String)> {
    resolve_symbol_info(db, path, SymbolKind::Procedure)
}

pub(crate) fn resolve_type_info(db: &dyn Db, path: InFile<&ast::Path>) -> Option<(Schema, String)> {
    resolve_symbol_info(db, path, SymbolKind::Type)
}

pub(crate) fn resolve_property_graph_info(
    db: &dyn Db,
    path: InFile<&ast::Path>,
) -> Option<(Schema, String)> {
    resolve_symbol_info(db, path, SymbolKind::PropertyGraph)
}

pub(crate) fn resolve_view_info(db: &dyn Db, path: InFile<&ast::Path>) -> Option<(Schema, String)> {
    resolve_symbol_info(db, path, SymbolKind::View)
}

pub(crate) fn resolve_sequence_info(
    db: &dyn Db,
    path: InFile<&ast::Path>,
) -> Option<(Schema, String)> {
    resolve_symbol_info(db, path, SymbolKind::Sequence)
}

fn resolve_symbol_info(
    db: &dyn Db,
    path: InFile<&ast::Path>,
    kind: SymbolKind,
) -> Option<(Schema, String)> {
    let binder = bind(db, path.file_id);
    let name = name::table_name(path.value)?;
    let schema = name::schema_name(path.value);
    let position = path.value.syntax().text_range().start();
    let schemas = binder.resolved_schemas(position, schema.as_ref());
    binder.lookup_info(&name, kind, &schemas)
}

fn param_signature(node: &ast::HasParamList) -> Option<Vec<Name>> {
    let mut params = vec![];
    for param in node.param_list()?.params() {
        if let Some(ast::Type::PathType(path_type)) = param.ty()
            && let Some(name_ref) = path_type
                .path()
                .and_then(|x| x.segment())
                .and_then(|x| x.name_ref())
        {
            params.push(Name::from_node(&name_ref));
        }
    }
    (!params.is_empty()).then_some(params)
}

fn resolve_composite_type_field_ptr(
    db: &dyn Db,
    field_name_ref: InFile<&ast::NameRef>,
) -> Option<SmallVec<[Location; 1]>> {
    let file = field_name_ref.file_id;
    let field_name_ref = field_name_ref.value;
    let tree = parse(db, file).tree();
    let root = tree.syntax();
    let field_name = Name::from_node(field_name_ref);
    let field_expr = field_name_ref
        .syntax()
        .parent()
        .and_then(ast::FieldExpr::cast)?;
    let base = field_expr.base()?;

    if let ast::Expr::ParenExpr(ref paren_expr) = base
        && let Some(result) = resolve_column_from_paren_expr(
            db,
            InFile::new(file, paren_expr),
            field_name_ref,
            &field_name,
        )
    {
        return Some(result);
    }

    let base_name_ref = ast_nav::unwrap_paren_expr(base.clone()).find_map(|e| match e {
        ast::Expr::NameRef(nr) => Some(nr),
        _ => None,
    })?;

    let column_locs = resolve_select_column_ptr(db, InFile::new(file, &base_name_ref))?;
    let column_node = column_locs.first()?.to_node(db)?;

    let (schema, type_name) = resolve_composite_type_from_column_node(&column_node)
        .or_else(|| resolve_composite_type_from_cast_node(&column_node))?;

    let position = field_name_ref.syntax().text_range().start();
    let schemas = bind(db, file).resolved_schemas(position, schema.as_ref());
    let type_name_ptr = resolve_type_name_ptr(db, &type_name, &schemas, file)?;
    let type_node = type_name_ptr.to_node(root);

    composite_type_field_location(file, &type_node, &field_name)
}

fn resolve_composite_type_field_for_path(
    db: &dyn Db,
    path: InFile<&ast::Path>,
    field_name: &Name,
) -> Option<SmallVec<[Location; 1]>> {
    let file = path.file_id;
    let path = path.value;
    let (schema, type_name) = name::schema_and_name_path(path)?;
    let position = path.syntax().text_range().start();
    let schemas = bind(db, file).resolved_schemas(position, schema.as_ref());
    let type_name_ptr = resolve_type_name_ptr(db, &type_name, &schemas, file)?;
    let tree = parse(db, file).tree();
    let type_node = type_name_ptr.to_node(tree.syntax());
    composite_type_field_location(file, &type_node, field_name)
}

fn composite_type_field_location(
    file: File,
    type_node: &SyntaxNode,
    field_name: &Name,
) -> Option<SmallVec<[Location; 1]>> {
    let create_type = type_node.ancestors().find_map(ast::CreateType::cast)?;
    let ast::CreateTypeKind::CompositeType(composite) = create_type.kind()? else {
        return None;
    };
    for column in composite.column_list()?.columns() {
        if let Some(col_name) = column.name()
            && Name::from_node(&col_name) == *field_name
        {
            return Some(smallvec![Location::new(
                file,
                col_name.syntax().text_range(),
                LocationKind::Column
            )]);
        }
    }

    None
}

fn resolve_composite_type_from_column_node(
    column_node: &SyntaxNode,
) -> Option<(Option<Schema>, Name)> {
    let column = column_node.ancestors().find_map(ast::Column::cast)?;
    let ty = column.ty()?;
    name::schema_and_type_name(&ty)
}

fn resolve_composite_type_from_cast_node(
    column_node: &SyntaxNode,
) -> Option<(Option<Schema>, Name)> {
    let target = column_node.ancestors().find_map(ast::Target::cast)?;
    let ast::Expr::CastExpr(cast_expr) = target.expr()? else {
        return None;
    };
    let ty = cast_expr.ty()?;
    name::schema_and_type_name(&ty)
}

fn resolve_update_table_name_ptr(
    db: &dyn Db,
    table_name_ref: InFile<&ast::NameRef>,
) -> Option<SmallVec<[Location; 1]>> {
    let file = table_name_ref.file_id;
    let table_name_ref = table_name_ref.value;
    let table_name = Name::from_node(table_name_ref);
    let update = table_name_ref
        .syntax()
        .ancestors()
        .find_map(ast::Update::cast)?;

    if let Some(from_clause) = update.from_clause() {
        for from_item in ast_nav::iter_from_clause(&from_clause) {
            if let Some(result) = resolve_from_item_table_name_ptr(
                db,
                InFile::new(file, table_name_ref),
                &from_item,
                &table_name,
            ) {
                return Some(result);
            }
        }
    }

    let path = update.relation_name()?.path()?;
    resolve_table_in_returning_clause(
        db,
        InFile::new(file, table_name_ref),
        update.alias(),
        &path,
        update.returning_clause(),
    )
}

fn resolve_from_item_table_name_ptr(
    db: &dyn Db,
    table_name_ref: InFile<&ast::NameRef>,
    from_item: &ast::FromItem,
    table_name: &Name,
) -> Option<SmallVec<[Location; 1]>> {
    let file = table_name_ref.file_id;
    let table_name_ref = table_name_ref.value;

    if let Some(alias_name) = from_item.alias().and_then(|x| x.name()) {
        if Name::from_node(&alias_name) == *table_name {
            return Some(smallvec![Location::new(
                file,
                alias_name.syntax().text_range(),
                LocationKind::Table,
            )]);
        }
        return None;
    }

    if let ast::FromItem::FunctionFromItem(func) = from_item
        && let Some(call_expr) = func.call_expr()
        && let Some((function_schema, function_name)) = name::schema_and_func_name(&call_expr)
        && function_name == *table_name
    {
        let position = table_name_ref.syntax().text_range().start();
        let schemas = bind(db, file).resolved_schemas(position, function_schema.as_ref());
        return resolve_function(db, &function_name, &schemas, None, file);
    }

    let (schema, item_name) = name::schema_and_table_from_from_item(from_item)?;
    if item_name != *table_name {
        return None;
    }

    let position = table_name_ref.syntax().text_range().start();
    let schemas = bind(db, file).resolved_schemas(position, schema.as_ref());
    let (ptr, kind) = resolve_table_like(db, Some(table_name_ref), &item_name, &schemas, file)?;
    Some(smallvec![Location::new(file, ptr.text_range(), kind)])
}

fn resolve_update_column_ptr(
    db: &dyn Db,
    column_name_ref: InFile<&ast::NameRef>,
) -> Option<SmallVec<[Location; 1]>> {
    let file = column_name_ref.file_id;
    let column_name_ref = column_name_ref.value;
    let column_name = Name::from_node(column_name_ref);
    let update = column_name_ref
        .syntax()
        .ancestors()
        .find_map(ast::Update::cast)?;

    // The left-hand side of `set` is always a target-table column, so `from`
    // tables are only considered for right-hand side expressions and predicates.
    let mut in_set_clause = false;
    let mut in_set_expr = false;
    for ancestor in column_name_ref.syntax().ancestors() {
        if ast::SetClause::can_cast(ancestor.kind()) {
            in_set_clause = true;
        }
        if ast::SetExpr::can_cast(ancestor.kind()) {
            in_set_expr = true;
        }
    }
    let is_set_target = in_set_clause && !in_set_expr;

    // `update t set a = b from u`
    if !is_set_target && let Some(from_clause) = update.from_clause() {
        for from_item in ast_nav::iter_from_clause(&from_clause) {
            if let Some(result) =
                resolve_from_item_column_ptr(db, InFile::new(file, &from_item), column_name_ref)
            {
                return Some(result);
            }
        }
    }

    // `update t set a = b`
    let path = update.relation_name()?.path()?;
    resolve_column_for_path(db, InFile::new(file, &path), column_name)
}

fn resolve_delete_column_ptr(
    db: &dyn Db,
    column_name_ref: InFile<&ast::NameRef>,
) -> Option<SmallVec<[Location; 1]>> {
    let file = column_name_ref.file_id;
    let column_name_ref = column_name_ref.value;
    let column_name = Name::from_node(column_name_ref);
    let delete = column_name_ref
        .syntax()
        .ancestors()
        .find_map(ast::Delete::cast)?;

    if let Some(using_clause) = delete.using_clause() {
        for from_item in using_clause.from_items() {
            if let Some(ptr) =
                resolve_from_item_column_ptr(db, InFile::new(file, &from_item), column_name_ref)
            {
                return Some(ptr);
            }
        }
    }

    let path = delete.relation_name()?.path()?;
    resolve_column_for_path(db, InFile::new(file, &path), column_name)
}

fn resolve_delete_table_name_ptr(
    db: &dyn Db,
    table_name_ref: InFile<&ast::NameRef>,
) -> Option<SmallVec<[Location; 1]>> {
    let file = table_name_ref.file_id;
    let table_name_ref = table_name_ref.value;
    let table_name = Name::from_node(table_name_ref);
    let delete = table_name_ref
        .syntax()
        .ancestors()
        .find_map(ast::Delete::cast)?;

    if let Some(using_clause) = delete.using_clause() {
        for from_item in using_clause.from_items() {
            if let Some(alias_name) = from_item.alias().and_then(|x| x.name()) {
                if Name::from_node(&alias_name) == table_name {
                    return Some(smallvec![Location::new(
                        file,
                        alias_name.syntax().text_range(),
                        LocationKind::Table
                    )]);
                }
            } else if let ast::FromItem::RelationFromItem(relation) = &from_item
                && let Some(item_name_ref) = relation.name_ref()
            {
                let item_name = Name::from_node(&item_name_ref);
                if item_name == table_name {
                    let position = table_name_ref.syntax().text_range().start();
                    let schemas = bind(db, file).resolved_schemas(position, None);
                    let (ptr, kind) =
                        resolve_table_like(db, Some(table_name_ref), &item_name, &schemas, file)?;
                    return Some(smallvec![Location::new(file, ptr.text_range(), kind)]);
                }
            }
        }
    }

    let path = delete.relation_name()?.path()?;
    resolve_table_in_returning_clause(
        db,
        InFile::new(file, table_name_ref),
        delete.alias(),
        &path,
        delete.returning_clause(),
    )
}

fn resolve_merge_column_ptr(
    db: &dyn Db,
    column_name_ref: InFile<&ast::NameRef>,
) -> Option<SmallVec<[Location; 1]>> {
    let file = column_name_ref.file_id;
    let column_name_ref = column_name_ref.value;
    let column_name = Name::from_node(column_name_ref);
    let merge = column_name_ref
        .syntax()
        .ancestors()
        .find_map(ast::Merge::cast)?;

    // Try resolving in source table first
    if let Some(from_item) = merge.using_on_clause().and_then(|x| x.from_item())
        && let Some(ptr) =
            resolve_from_item_column_ptr(db, InFile::new(file, &from_item), column_name_ref)
    {
        return Some(ptr);
    }

    let path = merge.relation_name()?.path()?;
    resolve_column_for_path(db, InFile::new(file, &path), column_name)
}

// TODO: I think we could use trait(s) here to simplify this and have the
// callers pass in the stmt instead of the fields.
fn resolve_table_in_returning_clause(
    db: &dyn Db,
    table_name_ref: InFile<&ast::NameRef>,
    alias: Option<ast::Alias>,
    path: &ast::Path,
    returning_clause: Option<ast::ReturningClause>,
) -> Option<SmallVec<[Location; 1]>> {
    let file = table_name_ref.file_id;
    let table_name_ref = table_name_ref.value;
    let table_name = Name::from_node(table_name_ref);
    let (schema, stmt_table_name) = name::schema_and_name_path(path)?;

    let matched = match_table_in_returning_clause(
        &table_name,
        &stmt_table_name,
        alias.as_ref(),
        returning_clause.as_ref(),
    )?;

    let position = table_name_ref.syntax().text_range().start();
    let schemas = bind(db, file).resolved_schemas(position, schema.as_ref());

    match matched {
        ReturningClauseMatch::ReturningAlias(name) => Some(smallvec![Location::new(
            file,
            name.syntax().text_range(),
            LocationKind::Table
        )]),
        ReturningClauseMatch::TableAlias(alias_name) => Some(smallvec![Location::new(
            file,
            alias_name.syntax().text_range(),
            LocationKind::Table
        )]),
        ReturningClauseMatch::PseudoTable => {
            let ptr = resolve_table_name_ptr(db, &stmt_table_name, &schemas, file)?;
            Some(smallvec![Location::new(
                file,
                ptr.text_range(),
                LocationKind::Table
            )])
        }
        ReturningClauseMatch::Table => {
            let ptr = resolve_table_name_ptr(db, &table_name, &schemas, file)?;
            Some(smallvec![Location::new(
                file,
                ptr.text_range(),
                LocationKind::Table
            )])
        }
    }
}

fn resolve_merge_table_name_ptr(
    db: &dyn Db,
    table_name_ref: InFile<&ast::NameRef>,
) -> Option<SmallVec<[Location; 1]>> {
    let file = table_name_ref.file_id;
    let table_name_ref = table_name_ref.value;
    let table_name = Name::from_node(table_name_ref);
    let merge = table_name_ref
        .syntax()
        .ancestors()
        .find_map(ast::Merge::cast)?;

    let path = merge.relation_name()?.path()?;

    // Check USING clause for the source table - MERGE-specific.
    // A source alias hides the underlying table name.
    if let Some(from_item) = merge.using_on_clause().and_then(|x| x.from_item()) {
        if let Some(alias_name) = from_item.alias().and_then(|x| x.name()) {
            if Name::from_node(&alias_name) == table_name {
                return Some(smallvec![Location::new(
                    file,
                    alias_name.syntax().text_range(),
                    LocationKind::Table
                )]);
            }
        } else if let ast::FromItem::RelationFromItem(relation) = &from_item
            && let Some(item_name_ref) = relation.name_ref()
        {
            let item_name = Name::from_node(&item_name_ref);
            if item_name == table_name {
                let position = table_name_ref.syntax().text_range().start();
                let schemas = bind(db, file).resolved_schemas(position, None);
                let (ptr, kind) =
                    resolve_table_like(db, Some(table_name_ref), &item_name, &schemas, file)?;
                return Some(smallvec![Location::new(file, ptr.text_range(), kind)]);
            }
        }
    }

    resolve_table_in_returning_clause(
        db,
        InFile::new(file, table_name_ref),
        merge.alias(),
        &path,
        merge.returning_clause(),
    )
}

fn find_param_in_func_def(
    db: &dyn Db,
    function_ptr: InFile<SyntaxNodePtr>,
    param_name: &Name,
) -> Option<SyntaxNodePtr> {
    let file = function_ptr.file_id;
    let function_ptr = function_ptr.value;
    let tree = parse(db, file).tree();
    let root = tree.syntax();
    let function_node = function_ptr.to_node(root);

    let param_list = function_node.ancestors().find_map(|a| {
        if let Some(create_func) = ast::CreateFunction::cast(a.clone()) {
            create_func.param_list()
        } else if let Some(create_proc) = ast::CreateProcedure::cast(a.clone()) {
            create_proc.param_list()
        } else if let Some(create_aggregate) = ast::CreateAggregate::cast(a) {
            create_aggregate.param_list()
        } else {
            None
        }
    })?;

    for param in param_list.params() {
        if let Some(name) = param.name()
            && Name::from_node(&name) == *param_name
        {
            return Some(SyntaxNodePtr::new(name.syntax()));
        }
    }

    None
}
