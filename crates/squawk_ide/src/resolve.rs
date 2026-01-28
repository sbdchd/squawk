use rowan::TextSize;
use smallvec::{SmallVec, smallvec};
use squawk_syntax::{
    SyntaxNode, SyntaxNodePtr,
    ast::{self, AstNode, SelectVariant},
};

use crate::binder::Binder;
use crate::classify::{NameRefClass, classify_name_ref};
use crate::column_name::ColumnName;
use crate::infer::{Type, infer_type_from_expr, infer_type_from_ty};
pub(crate) use crate::symbols::Schema;
use crate::symbols::{Name, SymbolKind};

pub(crate) fn resolve_name_ref_ptrs(
    binder: &Binder,
    root: &SyntaxNode,
    name_ref: &ast::NameRef,
) -> Option<SmallVec<[SyntaxNodePtr; 1]>> {
    let context = classify_name_ref(name_ref)?;

    match context {
        NameRefClass::Table => {
            let path = find_containing_path(name_ref)?;
            let (table_name, schema) = extract_table_schema_from_path(&path)?;
            let position = name_ref.syntax().text_range().start();
            resolve_table_name_ptr(binder, &table_name, &schema, position).map(|ptr| smallvec![ptr])
        }
        NameRefClass::NamedArgParameter => {
            let (function_name, schema) = find_func_call_from_named_arg(name_ref)?;
            let param_name = Name::from_node(name_ref);
            let position = name_ref.syntax().text_range().start();

            // TODO: this should be one lookup
            let function_ptr = binder
                .lookup_with(&function_name, SymbolKind::Function, position, &schema)
                .or_else(|| {
                    binder.lookup_with(&function_name, SymbolKind::Procedure, position, &schema)
                })
                .or_else(|| {
                    binder.lookup_with(&function_name, SymbolKind::Aggregate, position, &schema)
                })?;

            let param_ptr = find_param_in_func_def(root, function_ptr, &param_name)?;
            Some(smallvec![param_ptr])
        }
        NameRefClass::Cursor => {
            let cursor_name = Name::from_node(name_ref);
            binder
                .lookup(&cursor_name, SymbolKind::Cursor)
                .map(|ptr| smallvec![ptr])
        }
        NameRefClass::PreparedStatement => {
            let statement_name = Name::from_node(name_ref);
            resolve_prepared_statement_name_ptr(binder, &statement_name).map(|ptr| smallvec![ptr])
        }
        NameRefClass::Channel => {
            let channel_name = Name::from_node(name_ref);
            resolve_channel_name_ptr(binder, &channel_name).map(|ptr| smallvec![ptr])
        }
        NameRefClass::FromTable => {
            let table_name = Name::from_node(name_ref);
            let schema = if let Some(parent) = name_ref.syntax().parent()
                && let Some(field_expr) = ast::FieldExpr::cast(parent)
                && let Some(base) = field_expr.base()
                && let Some(schema_name_ref) = ast::NameRef::cast(base.syntax().clone())
            {
                Some(Schema(Name::from_node(&schema_name_ref)))
            } else {
                None
            };

            if schema.is_none()
                && let Some(cte_ptr) = resolve_cte_table(name_ref, &table_name)
            {
                return Some(smallvec![cte_ptr]);
            }

            let position = name_ref.syntax().text_range().start();

            if let Some(table_name_ptr) =
                resolve_table_name_ptr(binder, &table_name, &schema, position)
            {
                return Some(smallvec![table_name_ptr]);
            }

            resolve_view_name_ptr(binder, &table_name, &schema, position).map(|ptr| smallvec![ptr])
        }
        NameRefClass::Index => {
            let path = find_containing_path(name_ref)?;
            let (index_name, schema) = extract_table_schema_from_path(&path)?;
            let position = name_ref.syntax().text_range().start();
            resolve_index_name_ptr(binder, &index_name, &schema, position).map(|ptr| smallvec![ptr])
        }
        NameRefClass::Type => {
            let (type_name, schema) = if let Some(parent) = name_ref.syntax().parent()
                && let Some(field_expr) = ast::FieldExpr::cast(parent)
                && field_expr
                    .field()
                    .is_some_and(|field| field.syntax() == name_ref.syntax())
            {
                let type_name = Name::from_node(name_ref);
                let schema = if let Some(base) = field_expr.base()
                    && let ast::Expr::NameRef(schema_name_ref) = base
                {
                    Some(Schema(Name::from_node(&schema_name_ref)))
                } else {
                    None
                };
                (type_name, schema)
            } else {
                let path = find_containing_path(name_ref)?;
                let (type_name, schema) = extract_table_schema_from_path(&path)?;
                (type_name, schema)
            };
            let position = name_ref.syntax().text_range().start();
            resolve_type_name_ptr(binder, &type_name, &schema, position).map(|ptr| smallvec![ptr])
        }
        NameRefClass::View => {
            let path = find_containing_path(name_ref)?;
            let (view_name, schema) = extract_table_schema_from_path(&path)?;
            let position = name_ref.syntax().text_range().start();
            resolve_view_name_ptr(binder, &view_name, &schema, position).map(|ptr| smallvec![ptr])
        }
        NameRefClass::Sequence => {
            let path = find_containing_path(name_ref)?;
            let (sequence_name, schema) = extract_table_schema_from_path(&path)?;
            let position = name_ref.syntax().text_range().start();
            resolve_sequence_name_ptr(binder, &sequence_name, &schema, position)
                .map(|ptr| smallvec![ptr])
        }
        NameRefClass::Trigger => {
            let drop_trigger = name_ref
                .syntax()
                .ancestors()
                .find_map(ast::DropTrigger::cast)?;
            let path = drop_trigger.path()?;
            let (trigger_name, mut schema) = extract_table_schema_from_path(&path)?;
            let on_table_path = drop_trigger
                .on_table()
                .and_then(|on_table| on_table.path())?;
            if schema.is_none() {
                schema = extract_schema_name(&on_table_path);
            }
            let table_name = extract_table_name(&on_table_path)?;
            let position = name_ref.syntax().text_range().start();
            resolve_trigger_name_ptr(binder, &trigger_name, &schema, position, Some(table_name))
                .map(|ptr| smallvec![ptr])
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
            let (table_name, schema) = extract_table_schema_from_path(&on_table_path)?;
            let position = name_ref.syntax().text_range().start();
            resolve_policy_name_ptr(binder, &policy_name, &schema, position, table_name)
                .map(|ptr| smallvec![ptr])
        }
        NameRefClass::EventTrigger => {
            let event_trigger_name = Name::from_node(name_ref);
            resolve_event_trigger_name_ptr(binder, &event_trigger_name).map(|ptr| smallvec![ptr])
        }
        NameRefClass::Database => {
            let database_name = Name::from_node(name_ref);
            resolve_database_name_ptr(binder, &database_name).map(|ptr| smallvec![ptr])
        }
        NameRefClass::Server => {
            let server_name = Name::from_node(name_ref);
            resolve_server_name_ptr(binder, &server_name).map(|ptr| smallvec![ptr])
        }
        NameRefClass::Extension => {
            let extension_name = Name::from_node(name_ref);
            resolve_extension_name_ptr(binder, &extension_name).map(|ptr| smallvec![ptr])
        }
        NameRefClass::Role => {
            let role_name = Name::from_node(name_ref);
            resolve_role_name_ptr(binder, &role_name).map(|ptr| smallvec![ptr])
        }
        NameRefClass::QualifiedColumn => {
            let path = name_ref.syntax().ancestors().find_map(ast::Path::cast)?;
            let column_name = Name::from_node(name_ref);
            let table_path = path.qualifier()?;
            resolve_column_for_path(binder, root, &table_path, column_name)
                .map(|ptr| smallvec![ptr])
        }
        NameRefClass::Tablespace => {
            let tablespace_name = Name::from_node(name_ref);
            resolve_tablespace_name_ptr(binder, &tablespace_name).map(|ptr| smallvec![ptr])
        }
        NameRefClass::ForeignKeyTable => {
            let path = name_ref.syntax().ancestors().find_map(ast::Path::cast)?;
            let (table_name, schema) = extract_table_schema_from_path(&path)?;
            let position = name_ref.syntax().text_range().start();
            resolve_table_name_ptr(binder, &table_name, &schema, position).map(|ptr| smallvec![ptr])
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
            resolve_column_for_path(binder, root, &path, column_name).map(|ptr| smallvec![ptr])
        }
        NameRefClass::ConstraintColumn => {
            let create_table = name_ref
                .syntax()
                .ancestors()
                .find_map(ast::CreateTableLike::cast)?;
            let column_name = Name::from_node(name_ref);
            find_column_in_create_table(binder, root, &create_table, &column_name)
                .map(|ptr| smallvec![ptr])
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
            resolve_column_for_path(binder, root, &on_table_path, column_name)
                .map(|ptr| smallvec![ptr])
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
            let (table_name, schema) = extract_table_schema_from_path(&on_table_path)?;
            let position = name_ref.syntax().text_range().start();
            resolve_table_name_ptr(binder, &table_name, &schema, position).map(|ptr| smallvec![ptr])
        }
        NameRefClass::LikeTable => {
            let like_clause = name_ref
                .syntax()
                .ancestors()
                .find_map(ast::LikeClause::cast)?;
            let path = like_clause.path()?;
            let (table_name, schema) = extract_table_schema_from_path(&path)?;
            let position = name_ref.syntax().text_range().start();
            resolve_table_name_ptr(binder, &table_name, &schema, position).map(|ptr| smallvec![ptr])
        }
        NameRefClass::Function => {
            let function_sig = name_ref
                .syntax()
                .ancestors()
                .find_map(ast::FunctionSig::cast)?;
            let path = function_sig.path()?;
            let (function_name, schema) = extract_table_schema_from_path(&path)?;
            let params = extract_param_signature(&function_sig);
            let position = name_ref.syntax().text_range().start();
            resolve_function(binder, &function_name, &schema, params.as_deref(), position)
                .map(|ptr| smallvec![ptr])
        }
        NameRefClass::Aggregate => {
            let aggregate = name_ref
                .syntax()
                .ancestors()
                .find_map(ast::Aggregate::cast)?;
            let path = aggregate.path()?;
            let (aggregate_name, schema) = extract_table_schema_from_path(&path)?;
            let params = extract_param_signature(&aggregate);
            let position = name_ref.syntax().text_range().start();
            resolve_aggregate(
                binder,
                &aggregate_name,
                &schema,
                params.as_deref(),
                position,
            )
            .map(|ptr| smallvec![ptr])
        }
        NameRefClass::Procedure => {
            let function_sig = name_ref
                .syntax()
                .ancestors()
                .find_map(ast::FunctionSig::cast)?;
            let path = function_sig.path()?;
            let (procedure_name, schema) = extract_table_schema_from_path(&path)?;
            let params = extract_param_signature(&function_sig);
            let position = name_ref.syntax().text_range().start();
            resolve_procedure(
                binder,
                &procedure_name,
                &schema,
                params.as_deref(),
                position,
            )
            .map(|ptr| smallvec![ptr])
        }
        NameRefClass::Routine => {
            let function_sig = name_ref
                .syntax()
                .ancestors()
                .find_map(ast::FunctionSig::cast)?;
            let path = function_sig.path()?;
            let (routine_name, schema) = extract_table_schema_from_path(&path)?;
            let params = extract_param_signature(&function_sig);
            let position = name_ref.syntax().text_range().start();

            if let Some(ptr) =
                resolve_function(binder, &routine_name, &schema, params.as_deref(), position)
            {
                return Some(smallvec![ptr]);
            }

            if let Some(ptr) =
                resolve_aggregate(binder, &routine_name, &schema, params.as_deref(), position)
            {
                return Some(smallvec![ptr]);
            }

            resolve_procedure(binder, &routine_name, &schema, params.as_deref(), position)
                .map(|ptr| smallvec![ptr])
        }
        NameRefClass::CallProcedure => {
            let call = name_ref.syntax().ancestors().find_map(ast::Call::cast)?;
            let path = call.path()?;
            let (procedure_name, schema) = extract_table_schema_from_path(&path)?;
            let position = name_ref.syntax().text_range().start();
            resolve_procedure(binder, &procedure_name, &schema, None, position)
                .map(|ptr| smallvec![ptr])
        }
        NameRefClass::Schema => {
            let schema_name = Name::from_node(name_ref);
            resolve_schema(binder, &schema_name).map(|ptr| smallvec![ptr])
        }
        NameRefClass::FunctionCall => {
            let schema = if let Some(parent_node) = name_ref.syntax().parent()
                && let Some(field_expr) = ast::FieldExpr::cast(parent_node)
            {
                let base = field_expr.base()?;
                let schema_name_ref = ast::NameRef::cast(base.syntax().clone())?;
                Some(Schema(Name::from_node(&schema_name_ref)))
            } else {
                None
            };
            let function_name = Name::from_node(name_ref);
            let position = name_ref.syntax().text_range().start();
            resolve_function(binder, &function_name, &schema, None, position)
                .map(|ptr| smallvec![ptr])
        }
        NameRefClass::ProcedureCall => {
            let schema = if let Some(parent_node) = name_ref.syntax().parent()
                && let Some(field_expr) = ast::FieldExpr::cast(parent_node)
            {
                let base = field_expr.base()?;
                let schema_name_ref = ast::NameRef::cast(base.syntax().clone())?;
                Some(Schema(Name::from_node(&schema_name_ref)))
            } else {
                None
            };
            let procedure_name = Name::from_node(name_ref);
            let position = name_ref.syntax().text_range().start();

            resolve_procedure(binder, &procedure_name, &schema, None, position)
                .map(|ptr| smallvec![ptr])
        }
        NameRefClass::FunctionName => {
            let path_type = name_ref
                .syntax()
                .ancestors()
                .find_map(ast::PathType::cast)?;
            let path = path_type.path()?;
            let (function_name, schema) = extract_table_schema_from_path(&path)?;
            let position = name_ref.syntax().text_range().start();
            resolve_function(binder, &function_name, &schema, None, position)
                .map(|ptr| smallvec![ptr])
        }
        NameRefClass::SelectFunctionCall => {
            let schema = if let Some(parent_node) = name_ref.syntax().parent()
                && let Some(field_expr) = ast::FieldExpr::cast(parent_node)
            {
                let base = field_expr.base()?;
                let schema_name_ref = ast::NameRef::cast(base.syntax().clone())?;
                Some(Schema(Name::from_node(&schema_name_ref)))
            } else {
                None
            };
            let function_name = Name::from_node(name_ref);
            let position = name_ref.syntax().text_range().start();

            // functions take precedence
            if let Some(ptr) = resolve_function(binder, &function_name, &schema, None, position) {
                return Some(smallvec![ptr]);
            }

            // aggregates take precedence over function-call-style column access
            if let Some(ptr) = resolve_aggregate(binder, &function_name, &schema, None, position) {
                return Some(smallvec![ptr]);
            }

            // if no function found, check if this is function-call-style column access
            // ```sql
            // create table t(a int, b int);
            // select a(t) from t;
            // ```
            if schema.is_none()
                && let Some(ptr) = resolve_fn_call_column(binder, root, name_ref)
            {
                return Some(smallvec![ptr]);
            }

            None
        }
        NameRefClass::CreateIndexColumn => {
            resolve_create_index_column_ptr(binder, root, name_ref).map(|ptr| smallvec![ptr])
        }
        NameRefClass::SelectColumn => {
            resolve_select_column_ptr(binder, root, name_ref).map(|ptr| smallvec![ptr])
        }
        NameRefClass::SelectQualifiedColumnTable => {
            resolve_select_qualified_column_table_name_ptr(binder, name_ref)
                .map(|ptr| smallvec![ptr])
        }
        NameRefClass::SelectQualifiedColumn => {
            resolve_select_qualified_column_ptr(binder, root, name_ref).map(|ptr| smallvec![ptr])
        }
        NameRefClass::CompositeTypeField => {
            resolve_composite_type_field_ptr(binder, root, name_ref).map(|ptr| smallvec![ptr])
        }
        NameRefClass::InsertColumn => {
            resolve_insert_column_ptr(binder, root, name_ref).map(|ptr| smallvec![ptr])
        }
        NameRefClass::InsertQualifiedColumnTable => {
            resolve_insert_table_name_ptr(binder, name_ref).map(|ptr| smallvec![ptr])
        }
        NameRefClass::DeleteColumn => {
            resolve_delete_column_ptr(binder, root, name_ref).map(|ptr| smallvec![ptr])
        }
        NameRefClass::DeleteQualifiedColumnTable => {
            resolve_delete_table_name_ptr(binder, name_ref).map(|ptr| smallvec![ptr])
        }
        NameRefClass::UpdateColumn => {
            resolve_update_column_ptr(binder, root, name_ref).map(|ptr| smallvec![ptr])
        }
        NameRefClass::UpdateQualifiedColumnTable => {
            resolve_update_table_name_ptr(binder, name_ref).map(|ptr| smallvec![ptr])
        }
        NameRefClass::MergeColumn => {
            resolve_merge_column_ptr(binder, root, name_ref).map(|ptr| smallvec![ptr])
        }
        NameRefClass::MergeQualifiedColumnTable => {
            resolve_merge_table_name_ptr(binder, name_ref).map(|ptr| smallvec![ptr])
        }
        NameRefClass::JoinUsingColumn => resolve_join_using_columns(binder, root, name_ref),
        NameRefClass::AlterColumn => {
            let column_name = Name::from_node(name_ref);
            let alter_table = name_ref
                .syntax()
                .ancestors()
                .find_map(ast::AlterTable::cast)?;
            let table_path = alter_table.relation_name()?.path()?;
            resolve_column_for_path(binder, root, &table_path, column_name)
                .map(|ptr| smallvec![ptr])
        }
    }
}

fn resolve_table_name_ptr(
    binder: &Binder,
    table_name: &Name,
    schema: &Option<Schema>,
    position: TextSize,
) -> Option<SyntaxNodePtr> {
    binder.lookup_with(table_name, SymbolKind::Table, position, schema)
}

fn resolve_index_name_ptr(
    binder: &Binder,
    index_name: &Name,
    schema: &Option<Schema>,
    position: TextSize,
) -> Option<SyntaxNodePtr> {
    binder.lookup_with(index_name, SymbolKind::Index, position, schema)
}

fn resolve_type_name_ptr(
    binder: &Binder,
    type_name: &Name,
    schema: &Option<Schema>,
    position: TextSize,
) -> Option<SyntaxNodePtr> {
    if let Some(ptr) = binder.lookup_with(type_name, SymbolKind::Type, position, schema) {
        return Some(ptr);
    }

    if schema.is_none()
        && let Some(fallback_name) = fallback_type_alias(type_name)
    {
        return binder.lookup_with(&fallback_name, SymbolKind::Type, position, &None);
    }

    None
}

pub(crate) fn resolve_type_ptr_from_type(
    binder: &Binder,
    ty: &ast::Type,
    position: TextSize,
) -> Option<SyntaxNodePtr> {
    let (type_name, schema) = type_name_and_schema_from_type(ty)?;
    resolve_type_name_ptr(binder, &type_name, &schema, position)
}

fn type_name_and_schema_from_type(ty: &ast::Type) -> Option<(Name, Option<Schema>)> {
    match ty {
        ast::Type::ArrayType(array_type) => {
            let inner = array_type.ty()?;
            type_name_and_schema_from_type(&inner)
        }
        ast::Type::BitType(bit_type) => {
            let name = if bit_type.varying_token().is_some() {
                "varbit"
            } else {
                "bit"
            };
            Some((Name::from_string(name), None))
        }
        ast::Type::IntervalType(_) => Some((Name::from_string("interval"), None)),
        ast::Type::PathType(path_type) => {
            let path = path_type.path()?;
            extract_table_schema_from_path(&path)
        }
        ast::Type::ExprType(expr_type) => {
            let expr = expr_type.expr()?;
            if let ast::Expr::FieldExpr(field_expr) = expr
                && let Some(field) = field_expr.field()
                && let Some(ast::Expr::NameRef(schema_name_ref)) = field_expr.base()
            {
                let type_name = Name::from_node(&field);
                let schema = Some(Schema(Name::from_node(&schema_name_ref)));
                Some((type_name, schema))
            } else {
                None
            }
        }
        ast::Type::CharType(char_type) => {
            let name = if char_type.varchar_token().is_some() || char_type.varying_token().is_some()
            {
                "varchar"
            } else {
                "bpchar"
            };
            Some((Name::from_string(name), None))
        }
        ast::Type::DoubleType(_) => Some((Name::from_string("float8"), None)),
        ast::Type::TimeType(time_type) => {
            let mut name = if time_type.timestamp_token().is_some() {
                "timestamp".to_string()
            } else {
                "time".to_string()
            };
            if let Some(ast::Timezone::WithTimezone(_)) = time_type.timezone() {
                name.push_str("tz");
            }
            Some((Name::from_string(name), None))
        }
        ast::Type::PercentType(_) => None,
    }
}

fn fallback_type_alias(type_name: &Name) -> Option<Name> {
    match type_name.0.as_str() {
        "bigint" | "bigserial" | "serial8" => Some(Name::from_string("int8")),
        "int" | "integer" | "serial" | "serial4" => Some(Name::from_string("int4")),
        "smallint" | "smallserial" | "serial2" => Some(Name::from_string("int2")),
        _ => None,
    }
}

fn resolve_view_name_ptr(
    binder: &Binder,
    view_name: &Name,
    schema: &Option<Schema>,
    position: TextSize,
) -> Option<SyntaxNodePtr> {
    binder.lookup_with(view_name, SymbolKind::View, position, schema)
}

fn resolve_sequence_name_ptr(
    binder: &Binder,
    sequence_name: &Name,
    schema: &Option<Schema>,
    position: TextSize,
) -> Option<SyntaxNodePtr> {
    binder.lookup_with(sequence_name, SymbolKind::Sequence, position, schema)
}

fn resolve_trigger_name_ptr(
    binder: &Binder,
    trigger_name: &Name,
    schema: &Option<Schema>,
    position: TextSize,
    table: Option<Name>,
) -> Option<SyntaxNodePtr> {
    binder.lookup_with_table(trigger_name, SymbolKind::Trigger, position, schema, &table)
}

fn resolve_policy_name_ptr(
    binder: &Binder,
    policy_name: &Name,
    schema: &Option<Schema>,
    position: TextSize,
    table: Name,
) -> Option<SyntaxNodePtr> {
    binder.lookup_with_table(
        policy_name,
        SymbolKind::Policy,
        position,
        schema,
        &Some(table),
    )
}

fn resolve_event_trigger_name_ptr(
    binder: &Binder,
    event_trigger_name: &Name,
) -> Option<SyntaxNodePtr> {
    binder.lookup(event_trigger_name, SymbolKind::EventTrigger)
}

fn resolve_tablespace_name_ptr(binder: &Binder, tablespace_name: &Name) -> Option<SyntaxNodePtr> {
    binder.lookup(tablespace_name, SymbolKind::Tablespace)
}

fn resolve_prepared_statement_name_ptr(
    binder: &Binder,
    statement_name: &Name,
) -> Option<SyntaxNodePtr> {
    binder.lookup(statement_name, SymbolKind::PreparedStatement)
}

fn resolve_channel_name_ptr(binder: &Binder, channel_name: &Name) -> Option<SyntaxNodePtr> {
    binder.lookup(channel_name, SymbolKind::Channel)
}

fn resolve_database_name_ptr(binder: &Binder, database_name: &Name) -> Option<SyntaxNodePtr> {
    binder.lookup(database_name, SymbolKind::Database)
}

fn resolve_server_name_ptr(binder: &Binder, server_name: &Name) -> Option<SyntaxNodePtr> {
    binder.lookup(server_name, SymbolKind::Server)
}

fn resolve_extension_name_ptr(binder: &Binder, extension_name: &Name) -> Option<SyntaxNodePtr> {
    binder.lookup(extension_name, SymbolKind::Extension)
}

fn resolve_role_name_ptr(binder: &Binder, role_name: &Name) -> Option<SyntaxNodePtr> {
    binder.lookup(role_name, SymbolKind::Role)
}

fn resolve_for_kind_with_params(
    binder: &Binder,
    name: &Name,
    schema: &Option<Schema>,
    params: Option<&[Name]>,
    position: TextSize,
    kind: SymbolKind,
) -> Option<SyntaxNodePtr> {
    binder.lookup_with_params(name, kind, position, schema, params)
}

fn resolve_function(
    binder: &Binder,
    function_name: &Name,
    schema: &Option<Schema>,
    params: Option<&[Name]>,
    position: TextSize,
) -> Option<SyntaxNodePtr> {
    resolve_for_kind_with_params(
        binder,
        function_name,
        schema,
        params,
        position,
        SymbolKind::Function,
    )
}

fn resolve_aggregate(
    binder: &Binder,
    aggregate_name: &Name,
    schema: &Option<Schema>,
    params: Option<&[Name]>,
    position: TextSize,
) -> Option<SyntaxNodePtr> {
    resolve_for_kind_with_params(
        binder,
        aggregate_name,
        schema,
        params,
        position,
        SymbolKind::Aggregate,
    )
}

fn resolve_procedure(
    binder: &Binder,
    procedure_name: &Name,
    schema: &Option<Schema>,
    params: Option<&[Name]>,
    position: TextSize,
) -> Option<SyntaxNodePtr> {
    resolve_for_kind_with_params(
        binder,
        procedure_name,
        schema,
        params,
        position,
        SymbolKind::Procedure,
    )
}

fn resolve_schema(binder: &Binder, schema_name: &Name) -> Option<SyntaxNodePtr> {
    binder.lookup(schema_name, SymbolKind::Schema)
}

fn resolve_create_index_column_ptr(
    binder: &Binder,
    root: &SyntaxNode,
    column_name_ref: &ast::NameRef,
) -> Option<SyntaxNodePtr> {
    let column_name = Name::from_node(column_name_ref);

    let create_index = column_name_ref
        .syntax()
        .ancestors()
        .find_map(ast::CreateIndex::cast)?;
    let path = create_index.relation_name()?.path()?;

    resolve_column_for_path(binder, root, &path, column_name)
}

fn resolve_column_for_path(
    binder: &Binder,
    root: &SyntaxNode,
    path: &ast::Path,
    column_name: Name,
) -> Option<SyntaxNodePtr> {
    let (table_name, schema) = extract_table_schema_from_path(path)?;
    let position = path.syntax().text_range().start();

    resolve_column_for_table(binder, root, &table_name, &schema, &column_name, position)
}

fn resolve_insert_column_ptr(
    binder: &Binder,
    root: &SyntaxNode,
    column_name_ref: &ast::NameRef,
) -> Option<SyntaxNodePtr> {
    let column_name = Name::from_node(column_name_ref);

    let insert = column_name_ref
        .syntax()
        .ancestors()
        .find_map(ast::Insert::cast)?;
    let path = insert.path()?;

    resolve_column_for_path(binder, root, &path, column_name)
}

fn resolve_select_qualified_column_table_name_ptr(
    binder: &Binder,
    table_name_ref: &ast::NameRef,
) -> Option<SyntaxNodePtr> {
    let table_name = Name::from_node(table_name_ref);

    let field_expr = table_name_ref
        .syntax()
        .parent()
        .and_then(ast::FieldExpr::cast)?;

    let explicit_schema = if field_expr
        .field()
        .is_some_and(|f| f.syntax() == table_name_ref.syntax())
        && field_expr.star_token().is_none()
    {
        // if we're at the field `bar` in `foo.bar`
        if let ast::Expr::NameRef(schema_name_ref) = field_expr.base()? {
            Some(Schema(Name::from_node(&schema_name_ref)))
        } else {
            None
        }
    } else if let Some(base) = field_expr.base()
        && let ast::Expr::FieldExpr(inner_field_expr) = base
        && let Some(inner_base) = inner_field_expr.base()
        && let ast::Expr::NameRef(schema_name_ref) = inner_base
    {
        // if we're at the field `foo` in `foo.buzz.bar`
        Some(Schema(Name::from_node(&schema_name_ref)))
    } else {
        None
    };

    if let Some(schema) = explicit_schema {
        let position = table_name_ref.syntax().text_range().start();
        return resolve_table_name_ptr(binder, &table_name, &Some(schema), position);
    }

    let select = table_name_ref
        .syntax()
        .ancestors()
        .find_map(ast::Select::cast)?;
    let from_clause = select.from_clause()?;
    let from_item = find_from_item_in_from_clause(&from_clause, &table_name)?;

    if let Some(alias_name) = from_item.alias().and_then(|a| a.name())
        && Name::from_node(&alias_name) == table_name
    {
        return Some(SyntaxNodePtr::new(alias_name.syntax()));
    }

    let (table_name, schema) = if let Some(name_ref_node) = from_item.name_ref() {
        if let Some(cte_ptr) = resolve_cte_table(table_name_ref, &table_name) {
            return Some(cte_ptr);
        }

        // `from foo`
        let from_table_name = Name::from_node(&name_ref_node);
        if from_table_name == table_name {
            (from_table_name, None)
        } else {
            return None;
        }
    } else {
        // `from bar.foo`
        let from_field_expr = from_item.field_expr()?;
        let from_table_name = Name::from_node(&from_field_expr.field()?);
        if from_table_name != table_name {
            return None;
        }
        let ast::Expr::NameRef(schema_name_ref) = from_field_expr.base()? else {
            return None;
        };
        let schema = Schema(Name::from_node(&schema_name_ref));
        (from_table_name, Some(schema))
    };

    let position = table_name_ref.syntax().text_range().start();
    resolve_table_name_ptr(binder, &table_name, &schema, position)
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
    alias: Option<ast::Alias>,
    returning_clause: Option<ast::ReturningClause>,
) -> Option<ReturningClauseMatch> {
    // Check `returning with (old as alias, new as alias)`
    if let Some(returning_clause) = returning_clause
        && let Some(option_list) = returning_clause.returning_option_list()
    {
        for option in option_list.returning_options() {
            if let Some(name) = option.name()
                && Name::from_node(&name) == *table_name
            {
                return Some(ReturningClauseMatch::ReturningAlias(name));
            }
        }
    }

    if let Some(alias) = alias
        && let Some(alias_name) = alias.name()
        && Name::from_node(&alias_name) == *table_name
    {
        return Some(ReturningClauseMatch::TableAlias(alias_name));
    }

    let old_name = Name::from_string("old");
    let new_name = Name::from_string("new");
    if *table_name == old_name || *table_name == new_name {
        return Some(ReturningClauseMatch::PseudoTable);
    }

    if *stmt_table_name == *table_name {
        return Some(ReturningClauseMatch::Table);
    }

    None
}

fn extract_table_schema_from_path(path: &ast::Path) -> Option<(Name, Option<Schema>)> {
    Some((extract_table_name(path)?, extract_schema_name(path)))
}

fn resolve_select_qualified_column_ptr(
    binder: &Binder,
    root: &SyntaxNode,
    column_name_ref: &ast::NameRef,
) -> Option<SyntaxNodePtr> {
    let column_name = Name::from_node(column_name_ref);

    let field_expr = column_name_ref
        .syntax()
        .parent()
        .and_then(ast::FieldExpr::cast)?;

    let (column_table_name, explicit_schema) =
    // if we're at `base` in `base.field`
    if let Some(base) = field_expr.base()
        && let ast::Expr::NameRef(table_name_ref) = base
    {
        (Name::from_node(&table_name_ref), None)
    // we have `foo.bar.buzz`
    } else if let Some(base) = field_expr.base()
        && let ast::Expr::FieldExpr(inner_field_expr) = base
        && let Some(table_field) = inner_field_expr.field()
        && let Some(inner_base) = inner_field_expr.base()
        && let ast::Expr::NameRef(schema_name_ref) = inner_base
    {
        (
            Name::from_node(&table_field),
            Some(Schema(Name::from_node(&schema_name_ref))),
        )
    } else {
        return None;
    };

    let position = column_name_ref.syntax().text_range().start();

    let (mut table_name, schema) = if let Some(schema) = explicit_schema {
        (column_table_name, Some(schema))
    } else if let Some(merge) = column_name_ref
        .syntax()
        .ancestors()
        .find_map(ast::Merge::cast)
    {
        let found_in_using = if let Some(using_on) = merge.using_on_clause()
            && let Some(from_item) = using_on.from_item()
        {
            if let Some(item_name_ref) = from_item.name_ref()
                && let item_name = Name::from_node(&item_name_ref)
                && item_name == column_table_name
            {
                Some((item_name, None))
            } else if let Some(alias) = from_item.alias()
                && let Some(alias_name) = alias.name()
                && let alias_name = Name::from_node(&alias_name)
                && alias_name == column_table_name
            {
                Some((alias_name, None))
            } else {
                None
            }
        } else {
            None
        };

        if let Some(result) = found_in_using {
            result
        } else {
            let path = merge.relation_name()?.path()?;
            extract_table_schema_from_path(&path)?
        }
    } else if let Some(insert) = column_name_ref
        .syntax()
        .ancestors()
        .find_map(ast::Insert::cast)
    {
        let path = insert.path()?;
        extract_table_schema_from_path(&path)?
    } else if let Some(update) = column_name_ref
        .syntax()
        .ancestors()
        .find_map(ast::Update::cast)
    {
        let path = update.relation_name()?.path()?;
        extract_table_schema_from_path(&path)?
    } else if let Some(delete) = column_name_ref
        .syntax()
        .ancestors()
        .find_map(ast::Delete::cast)
    {
        let path = delete.relation_name()?.path()?;
        extract_table_schema_from_path(&path)?
    } else {
        let select = column_name_ref
            .syntax()
            .ancestors()
            .find_map(ast::Select::cast)?;
        let from_clause = select.from_clause()?;
        let from_item = find_from_item_in_from_clause(&from_clause, &column_table_name)?;

        // `from t as u`
        // `from t as u(a, b, c)`
        if let Some(alias) = from_item.alias()
            && let Some(alias_name) = alias.name()
            && Name::from_node(&alias_name) == column_table_name
        {
            if let Some(paren_select) = from_item.paren_select() {
                return resolve_subquery_column_ptr(
                    binder,
                    root,
                    &paren_select,
                    column_name_ref,
                    &column_name,
                    Some(&alias),
                );
            }

            // `from t as u(a, b, c)`
            if let Some(column_list) = alias.column_list() {
                for column in column_list.columns() {
                    if let Some(col_name) = column.name()
                        && Name::from_node(&col_name) == column_name
                    {
                        return Some(SyntaxNodePtr::new(col_name.syntax()));
                    }
                }

                // ```sql
                // create table t(a int, b int);
                // select b from t as u(x);
                //        ^
                // ```
                if let Some(name_ref_node) = from_item.name_ref() {
                    let cte_name = Name::from_node(&name_ref_node);
                    return resolve_cte_column(
                        binder,
                        root,
                        column_name_ref,
                        &cte_name,
                        &column_name,
                    );
                }
            }

            // `from t as u`
            if let Some(name_ref_node) = from_item.name_ref() {
                (Name::from_node(&name_ref_node), None)
            // `from foo.t as u`
            } else if let Some(from_field_expr) = from_item.field_expr() {
                let table_name = Name::from_node(&from_field_expr.field()?);
                let ast::Expr::NameRef(schema_name_ref) = from_field_expr.base()? else {
                    return None;
                };
                let schema = Schema(Name::from_node(&schema_name_ref));
                (table_name, Some(schema))
            } else {
                return None;
            }
        } else if let Some(name_ref_node) = from_item.name_ref() {
            // `from bar`
            let from_table_name = Name::from_node(&name_ref_node);
            if from_table_name == column_table_name {
                (from_table_name, None)
            } else {
                return None;
            }
        } else {
            // `from foo.bar`
            let from_field_expr = from_item.field_expr()?;
            let from_table_name = Name::from_node(&from_field_expr.field()?);
            if from_table_name != column_table_name {
                return None;
            }
            let ast::Expr::NameRef(schema_name_ref) = from_field_expr.base()? else {
                return None;
            };
            let schema = Schema(Name::from_node(&schema_name_ref));
            (from_table_name, Some(schema))
        }
    };

    if schema.is_none() {
        if resolve_cte_table(column_name_ref, &table_name).is_some() {
            if let Some(cte_column_ptr) =
                resolve_cte_column(binder, root, column_name_ref, &table_name, &column_name)
            {
                return Some(cte_column_ptr);
            }
            return None;
        }
        if let Some(alias_table_name) = resolve_alias(column_name_ref, &table_name) {
            table_name = alias_table_name;
        }
    }

    resolve_column_for_table(binder, root, &table_name, &schema, &column_name, position)
}

fn resolve_column_for_table(
    binder: &Binder,
    root: &SyntaxNode,
    table_name: &Name,
    schema: &Option<Schema>,
    column_name: &Name,
    position: TextSize,
) -> Option<SyntaxNodePtr> {
    let resolved = resolve_table_name(binder, root, table_name, schema, position)?;
    match resolved {
        ResolvedTableName::View(create_view) => {
            if let Some(ptr) = find_column_in_create_view(&create_view, column_name) {
                return Some(ptr);
            }
            return resolve_function(binder, column_name, schema, None, position);
        }
        ResolvedTableName::Table(create_table_like) => {
            // 1. Try to find a matching column (columns take precedence)
            if let Some(ptr) =
                find_column_in_create_table(binder, root, &create_table_like, column_name)
            {
                return Some(ptr);
            }
            // 2. No column found, check for field-style function call
            // e.g., select t.b from t where b is a function that takes t as an argument
            return resolve_function(binder, column_name, schema, None, position);
        }
        ResolvedTableName::TableAs(create_table_as) => {
            if let Some(ptr) = find_column_in_create_table_as(&create_table_as, column_name) {
                return Some(ptr);
            }
            return resolve_function(binder, column_name, schema, None, position);
        }
    }
}

enum ResolvedTableName {
    View(ast::CreateView),
    Table(ast::CreateTableLike),
    TableAs(ast::CreateTableAs),
}
fn resolve_table_name(
    binder: &Binder,
    root: &SyntaxNode,
    table_name: &Name,
    schema: &Option<Schema>,
    position: TextSize,
) -> Option<ResolvedTableName> {
    use ResolvedTableName::*;
    if let Some(table_name_ptr) = resolve_table_name_ptr(binder, table_name, schema, position) {
        let table_name_node = table_name_ptr.to_node(root);
        if let Some(create_table) = table_name_node
            .ancestors()
            .find_map(ast::CreateTableLike::cast)
        {
            return Some(Table(create_table));
        }
        if let Some(create_table_as) = table_name_node
            .ancestors()
            .find_map(ast::CreateTableAs::cast)
        {
            return Some(TableAs(create_table_as));
        }
    }

    if let Some(view_name_ptr) = resolve_view_name_ptr(binder, table_name, schema, position) {
        let view_name_node = view_name_ptr.to_node(root);
        if let Some(create_view) = view_name_node.ancestors().find_map(ast::CreateView::cast) {
            return Some(View(create_view));
        }
    }
    None
}

fn resolve_alias(name_ref: &ast::NameRef, table_name: &Name) -> Option<Name> {
    let from_item = find_parent_alias_from_item(name_ref.syntax())?;
    if let Some(alias) = from_item.alias()
        && let Some(alias_name) = alias.name()
        && Name::from_node(&alias_name) == *table_name
    {
        let table_name = Name::from_node(&from_item.name_ref()?);
        return Some(table_name);
    }
    None
}

fn find_parent_alias_from_item(syntax: &SyntaxNode) -> Option<ast::FromItem> {
    for a in syntax.ancestors() {
        if let Some(merge) = ast::Merge::cast(a)
            && let Some(from_item) = merge.using_on_clause().and_then(|c| c.from_item())
        {
            return Some(from_item);
        }
    }
    None
}

fn resolve_from_item_column_ptr(
    binder: &Binder,
    root: &SyntaxNode,
    from_item: &ast::FromItem,
    column_name_ref: &ast::NameRef,
) -> Option<SyntaxNodePtr> {
    let column_name = Name::from_node(column_name_ref);
    if let Some(paren_select) = from_item.paren_select() {
        let alias = from_item.alias();
        return resolve_subquery_column_ptr(
            binder,
            root,
            &paren_select,
            column_name_ref,
            &column_name,
            alias.as_ref(),
        );
    }

    if let Some(paren_expr) = from_item.paren_expr() {
        return resolve_column_from_paren_expr(
            binder,
            root,
            &paren_expr,
            column_name_ref,
            &column_name,
        );
    }

    if let Some(alias) = from_item.alias()
        && let Some(column_list) = alias.column_list()
    {
        for col in column_list.columns() {
            if let Some(col_name) = col.name()
                && Name::from_node(&col_name) == column_name
            {
                return Some(SyntaxNodePtr::new(col_name.syntax()));
            }
        }
    }

    let (table_name, schema) = table_and_schema_from_from_item(from_item)?;

    if schema.is_none() && resolve_cte_table(column_name_ref, &table_name).is_some() {
        if let Some(cte_column_ptr) =
            resolve_cte_column(binder, root, column_name_ref, &table_name, &column_name)
        {
            return Some(cte_column_ptr);
        }
        return None;
    }

    resolve_column_from_table_or_view(
        binder,
        root,
        column_name_ref,
        &table_name,
        &schema,
        &column_name,
    )
}

fn resolve_column_from_table_or_view(
    binder: &Binder,
    root: &SyntaxNode,
    name_ref: &ast::NameRef,
    table_name: &Name,
    schema: &Option<Schema>,
    column_name: &Name,
) -> Option<SyntaxNodePtr> {
    resolve_column_from_table_or_view_impl(
        binder,
        root,
        name_ref,
        table_name,
        schema,
        column_name,
        0,
    )
}

fn resolve_column_from_table_or_view_impl(
    binder: &Binder,
    root: &SyntaxNode,
    name_ref: &ast::NameRef,
    table_name: &Name,
    schema: &Option<Schema>,
    column_name: &Name,
    depth: u32,
) -> Option<SyntaxNodePtr> {
    if depth > 40 {
        log::info!("max resolve depth reached, probably in a cycle");
        return None;
    }

    let position = name_ref.syntax().text_range().start();

    if let Some(table_name_ptr) = resolve_table_name_ptr(binder, table_name, schema, position) {
        let table_name_node = table_name_ptr.to_node(root);

        if let Some(create_table) = table_name_node
            .ancestors()
            .find_map(ast::CreateTableLike::cast)
        {
            // 1. try to find a matching column
            if let Some(ptr) = find_column_in_create_table(binder, root, &create_table, column_name)
            {
                return Some(ptr);
            }

            // 2. No column found, check if this is a partitioned table
            if let Some(create_table_node) = ast::CreateTable::cast(create_table.syntax().clone())
                && let Some(partition_of) = create_table_node.partition_of()
                && let Some(parent_path) = partition_of.path()
            {
                let (parent_table_name, parent_schema) =
                    extract_table_schema_from_path(&parent_path)?;
                return resolve_column_from_table_or_view_impl(
                    binder,
                    root,
                    name_ref,
                    &parent_table_name,
                    &parent_schema,
                    column_name,
                    depth + 1,
                );
            }

            // 3. No column found, check if the name matches the table name.
            // For example, in:
            // ```sql
            // create table t(a int);
            // select t from t;
            // ```
            if column_name == table_name {
                return Some(table_name_ptr);
            }
        }

        if let Some(create_table_as) = table_name_node
            .ancestors()
            .find_map(ast::CreateTableAs::cast)
        {
            if let Some(ptr) = find_column_in_create_table_as(&create_table_as, column_name) {
                return Some(ptr);
            }

            if column_name == table_name {
                return Some(table_name_ptr);
            }
        }
    }

    // ditto as above but with view
    if let Some(view_name_ptr) = resolve_view_name_ptr(binder, table_name, schema, position) {
        let view_name_node = view_name_ptr.to_node(root);

        if let Some(create_view) = view_name_node.ancestors().find_map(ast::CreateView::cast) {
            if let Some(ptr) = find_column_in_create_view(&create_view, column_name) {
                return Some(ptr);
            }

            if column_name == table_name {
                return Some(view_name_ptr);
            }
        }
    }

    None
}

fn resolve_from_item_for_cte_star(
    binder: &Binder,
    root: &SyntaxNode,
    from_item: &ast::FromItem,
    name_ref: &ast::NameRef,
    cte_name: &Name,
    column_name: &Name,
) -> Option<SyntaxNodePtr> {
    if let Some((table_name, schema)) = table_and_schema_from_from_item(from_item)
        && table_name == *cte_name
    {
        return resolve_column_from_table_or_view(
            binder,
            root,
            name_ref,
            &table_name,
            &schema,
            column_name,
        );
    }

    resolve_from_item_column_ptr(binder, root, from_item, name_ref)
}

fn resolve_from_join_expr<F>(join_expr: &ast::JoinExpr, try_resolve: &F) -> Option<SyntaxNodePtr>
where
    F: Fn(&ast::FromItem) -> Option<SyntaxNodePtr>,
{
    if let Some(nested_join) = join_expr.join_expr()
        && let Some(result) = resolve_from_join_expr(&nested_join, try_resolve)
    {
        return Some(result);
    }
    if let Some(from_item) = join_expr.from_item()
        && let Some(result) = try_resolve(&from_item)
    {
        return Some(result);
    }
    if let Some(join) = join_expr.join()
        && let Some(from_item) = join.from_item()
        && let Some(result) = try_resolve(&from_item)
    {
        return Some(result);
    }
    None
}

fn resolve_select_column_ptr(
    binder: &Binder,
    root: &SyntaxNode,
    column_name_ref: &ast::NameRef,
) -> Option<SyntaxNodePtr> {
    let select = column_name_ref
        .syntax()
        .ancestors()
        .find_map(ast::Select::cast)?;
    let from_clause = select.from_clause()?;

    for from_item in from_clause.from_items() {
        if let Some(column_ptr) =
            resolve_from_item_column_ptr(binder, root, &from_item, column_name_ref)
        {
            return Some(column_ptr);
        }
    }

    for join_expr in from_clause.join_exprs() {
        if let Some(column_ptr) =
            resolve_from_join_expr(&join_expr, &|from_item: &ast::FromItem| {
                resolve_from_item_column_ptr(binder, root, from_item, column_name_ref)
            })
        {
            return Some(column_ptr);
        }
    }

    None
}

fn resolve_delete_column_ptr(
    binder: &Binder,
    root: &SyntaxNode,
    column_name_ref: &ast::NameRef,
) -> Option<SyntaxNodePtr> {
    let column_name = Name::from_node(column_name_ref);

    let delete = column_name_ref
        .syntax()
        .ancestors()
        .find_map(ast::Delete::cast)?;
    let path = delete.relation_name()?.path()?;

    resolve_column_for_path(binder, root, &path, column_name)
}

fn resolve_join_using_columns(
    binder: &Binder,
    root: &SyntaxNode,
    name_ref: &ast::NameRef,
) -> Option<SmallVec<[SyntaxNodePtr; 1]>> {
    let join_expr = name_ref
        .syntax()
        .ancestors()
        .find_map(ast::JoinExpr::cast)?;

    let mut results: SmallVec<[SyntaxNodePtr; 1]> = SmallVec::new();

    collect_from_join_expr(&join_expr, &mut results, &|from_item: &ast::FromItem| {
        resolve_from_item_column_ptr(binder, root, from_item, name_ref)
    });

    (!results.is_empty()).then_some(results)
}

fn collect_from_join_expr<F>(
    join_expr: &ast::JoinExpr,
    results: &mut SmallVec<[SyntaxNodePtr; 1]>,
    try_resolve: &F,
) where
    F: Fn(&ast::FromItem) -> Option<SyntaxNodePtr>,
{
    if let Some(nested_join) = join_expr.join_expr() {
        collect_from_join_expr(&nested_join, results, try_resolve);
    }
    if let Some(from_item) = join_expr.from_item()
        && let Some(result) = try_resolve(&from_item)
    {
        results.push(result);
    }
    if let Some(join) = join_expr.join()
        && let Some(from_item) = join.from_item()
    {
        if let Some(result) = try_resolve(&from_item) {
            results.push(result);
        }
    }
}

fn resolve_update_column_ptr(
    binder: &Binder,
    root: &SyntaxNode,
    column_name_ref: &ast::NameRef,
) -> Option<SyntaxNodePtr> {
    let column_name = Name::from_node(column_name_ref);

    let update = column_name_ref
        .syntax()
        .ancestors()
        .find_map(ast::Update::cast)?;

    // `update t set a = b from u`
    if let Some(from_clause) = update.from_clause() {
        for from_item in from_clause.from_items() {
            if let Some(result) =
                resolve_from_item_column_ptr(binder, root, &from_item, column_name_ref)
            {
                return Some(result);
            }
        }

        for join_expr in from_clause.join_exprs() {
            if let Some(result) =
                resolve_from_join_expr(&join_expr, &|from_item: &ast::FromItem| {
                    resolve_from_item_column_ptr(binder, root, from_item, column_name_ref)
                })
            {
                return Some(result);
            }
        }
    }

    // `update t set a = b`
    let path = update.relation_name()?.path()?;

    resolve_column_for_path(binder, root, &path, column_name)
}

fn resolve_fn_call_column(
    binder: &Binder,
    root: &SyntaxNode,
    name_ref: &ast::NameRef,
) -> Option<SyntaxNodePtr> {
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

    for from_item in from_clause.from_items() {
        if let Some(result) =
            resolve_from_item_for_fn_call_column(binder, root, &from_item, &column_name, name_ref)
        {
            return Some(result);
        }
    }

    for join_expr in from_clause.join_exprs() {
        if let Some(result) = resolve_from_join_expr(&join_expr, &|from_item: &ast::FromItem| {
            resolve_from_item_for_fn_call_column(binder, root, from_item, &column_name, name_ref)
        }) {
            return Some(result);
        }
    }

    None
}

fn resolve_from_item_for_fn_call_column(
    binder: &Binder,
    root: &SyntaxNode,
    from_item: &ast::FromItem,
    column_name: &Name,
    name_ref: &ast::NameRef,
) -> Option<SyntaxNodePtr> {
    let (table_name, schema) = table_and_schema_from_from_item(from_item)?;

    let position = name_ref.syntax().text_range().start();
    let table_ptr = resolve_table_name_ptr(binder, &table_name, &schema, position)?;

    let table_name_node = table_ptr.to_node(root);
    let create_table = table_name_node
        .ancestors()
        .find_map(ast::CreateTableLike::cast)?;

    find_column_in_create_table(binder, root, &create_table, column_name)
}

fn table_and_schema_from_from_item(from_item: &ast::FromItem) -> Option<(Name, Option<Schema>)> {
    if let Some(name_ref_node) = from_item.name_ref() {
        return Some((Name::from_node(&name_ref_node), None));
    }

    let field_expr = from_item.field_expr()?;
    let table_name = Name::from_node(&field_expr.field()?);
    let ast::Expr::NameRef(schema_name_ref) = field_expr.base()? else {
        return None;
    };
    let schema = Schema(Name::from_node(&schema_name_ref));
    Some((table_name, Some(schema)))
}

fn is_from_item_match(from_item: &ast::FromItem, qualifier: &Name) -> bool {
    if let Some(alias_name) = from_item.alias().and_then(|a| a.name())
        && Name::from_node(&alias_name) == *qualifier
    {
        return true;
    }

    if let Some(name_ref) = from_item.name_ref()
        && Name::from_node(&name_ref) == *qualifier
    {
        return true;
    }

    if let Some(field_expr) = from_item.field_expr()
        && let Some(field) = field_expr.field()
        && Name::from_node(&field) == *qualifier
    {
        return true;
    }

    false
}

fn find_from_item_in_join_expr(
    join_expr: &ast::JoinExpr,
    qualifier: &Name,
) -> Option<ast::FromItem> {
    if let Some(nested_join_expr) = join_expr.join_expr()
        && let Some(found) = find_from_item_in_join_expr(&nested_join_expr, qualifier)
    {
        return Some(found);
    }

    if let Some(from_item) = join_expr.from_item()
        && is_from_item_match(&from_item, qualifier)
    {
        return Some(from_item);
    }

    if let Some(join) = join_expr.join()
        && let Some(from_item) = join.from_item()
        && is_from_item_match(&from_item, qualifier)
    {
        return Some(from_item);
    }

    None
}

pub(crate) fn find_from_item_in_from_clause(
    from_clause: &ast::FromClause,
    qualifier: &Name,
) -> Option<ast::FromItem> {
    for from_item in from_clause.from_items() {
        if is_from_item_match(&from_item, qualifier) {
            return Some(from_item);
        }
    }

    for join_expr in from_clause.join_exprs() {
        if let Some(found) = find_from_item_in_join_expr(&join_expr, qualifier) {
            return Some(found);
        }
    }

    None
}

fn find_containing_path(name_ref: &ast::NameRef) -> Option<ast::Path> {
    for ancestor in name_ref.syntax().ancestors() {
        if let Some(path) = ast::Path::cast(ancestor) {
            return Some(path);
        }
    }
    None
}

pub(crate) fn extract_table_name(path: &ast::Path) -> Option<Name> {
    let segment = path.segment()?;
    let name_ref = segment.name_ref()?;
    Some(Name::from_node(&name_ref))
}

pub(crate) fn extract_schema_name(path: &ast::Path) -> Option<Schema> {
    path.qualifier()
        .and_then(|q| q.segment())
        .and_then(|s| s.name_ref())
        .map(|name_ref| Schema(Name::from_node(&name_ref)))
}

pub(crate) fn extract_column_name(col: &ast::Column) -> Option<Name> {
    let name = if let Some(name_ref) = col.name_ref() {
        Name::from_node(&name_ref)
    } else {
        let name = col.name()?;
        Name::from_node(&name)
    };
    Some(name)
}

pub(crate) fn find_column_in_create_table(
    binder: &Binder,
    root: &SyntaxNode,
    create_table: &impl ast::HasCreateTable,
    column_name: &Name,
) -> Option<SyntaxNodePtr> {
    find_column_in_create_table_impl(binder, root, create_table, column_name, 0)
}

fn find_column_in_create_table_impl(
    binder: &Binder,
    root: &SyntaxNode,
    create_table: &impl ast::HasCreateTable,
    column_name: &Name,
    depth: usize,
) -> Option<SyntaxNodePtr> {
    if depth > 40 {
        log::info!("max depth reached, probably in a cycle");
        return None;
    }

    for arg in create_table.table_arg_list()?.args() {
        match &arg {
            ast::TableArg::Column(column) => {
                if let Some(name) = column.name()
                    && Name::from_node(&name) == *column_name
                {
                    return Some(SyntaxNodePtr::new(name.syntax()));
                }
            }
            ast::TableArg::LikeClause(like_clause) => {
                let path = like_clause.path()?;
                let (table_name, schema) = extract_table_schema_from_path(&path)?;
                let position = path.syntax().text_range().start();

                if let Some(resolved) =
                    resolve_table_name(binder, root, &table_name, &schema, position)
                {
                    if let Some(ptr) = match resolved {
                        ResolvedTableName::Table(source_table) => find_column_in_create_table_impl(
                            binder,
                            root,
                            &source_table,
                            column_name,
                            depth + 1,
                        ),
                        ResolvedTableName::TableAs(create_table_as) => {
                            find_column_in_create_table_as(&create_table_as, column_name)
                        }
                        ResolvedTableName::View(_) => None,
                    } {
                        return Some(ptr);
                    }
                }
            }
            ast::TableArg::TableConstraint(_) => (),
        }
    }

    if let Some(inherits) = create_table.inherits() {
        for path in inherits.paths() {
            let (table_name, schema) = extract_table_schema_from_path(&path)?;
            let position = path.syntax().text_range().start();

            if let Some(resolved) = resolve_table_name(binder, root, &table_name, &schema, position)
            {
                if let Some(ptr) = match resolved {
                    ResolvedTableName::Table(parent_table) => find_column_in_create_table_impl(
                        binder,
                        root,
                        &parent_table,
                        column_name,
                        depth + 1,
                    ),
                    ResolvedTableName::TableAs(create_table_as) => {
                        find_column_in_create_table_as(&create_table_as, column_name)
                    }
                    ResolvedTableName::View(_) => None,
                } {
                    return Some(ptr);
                }
            }
        }
    }

    None
}

// TODO: this is similar to the CTE funcs, maybe we can simplify
fn find_column_in_create_view(
    create_view: &ast::CreateView,
    column_name: &Name,
) -> Option<SyntaxNodePtr> {
    let column_list_len = if let Some(column_list) = create_view.column_list() {
        for column in column_list.columns() {
            if let Some(col_name) = column.name()
                && Name::from_node(&col_name) == *column_name
            {
                return Some(SyntaxNodePtr::new(col_name.syntax()));
            }
        }
        column_list.columns().count()
    } else {
        0
    };

    let select = resolve_select_clause(create_view.query()?)?;

    let select_clause = select.select_clause()?;
    let target_list = select_clause.target_list()?;

    for (idx, target) in target_list.targets().enumerate() {
        if idx < column_list_len {
            continue;
        }

        if let Some((col_name, node)) = ColumnName::from_target(target.clone()) {
            if let Some(col_name_str) = col_name.to_string()
                && Name::from_string(col_name_str) == *column_name
            {
                return Some(SyntaxNodePtr::new(&node));
            }
        }
    }

    None
}

fn find_column_in_create_table_as(
    create_table_as: &ast::CreateTableAs,
    column_name: &Name,
) -> Option<SyntaxNodePtr> {
    let select = resolve_select_clause(create_table_as.query()?)?;

    for target in select.select_clause()?.target_list()?.targets() {
        if let Some((col_name, node)) = ColumnName::from_target(target.clone()) {
            if let Some(col_name_str) = col_name.to_string()
                && Name::from_string(col_name_str) == *column_name
            {
                return Some(SyntaxNodePtr::new(&node));
            }
        }
    }

    None
}

fn resolve_cte_table(name_ref: &ast::NameRef, cte_name: &Name) -> Option<SyntaxNodePtr> {
    let with_clause = find_parent_with_clause(name_ref.syntax())?;
    for with_table in with_clause.with_tables() {
        if let Some(name) = with_table.name()
            && Name::from_node(&name) == *cte_name
        {
            if with_table
                .syntax()
                .text_range()
                .contains_range(name_ref.syntax().text_range())
            {
                continue;
            }
            return Some(SyntaxNodePtr::new(name.syntax()));
        }
    }

    None
}

fn find_parent_with_clause(node: &SyntaxNode) -> Option<ast::WithClause> {
    node.ancestors().find_map(|x| {
        if let Some(query) = ast::WithQuery::cast(x) {
            query.with_clause()
        } else {
            None
        }
    })
}

fn count_columns_for_path(binder: &Binder, root: &SyntaxNode, path: &ast::Path) -> Option<usize> {
    let (table_name, schema) = extract_table_schema_from_path(path)?;
    let position = path.syntax().text_range().start();

    count_columns_for_table_name(binder, root, &table_name, &schema, position)
}

fn count_columns_for_table_name(
    binder: &Binder,
    root: &SyntaxNode,
    table_name: &Name,
    schema: &Option<Schema>,
    position: TextSize,
) -> Option<usize> {
    if let Some(table_name_ptr) = resolve_table_name_ptr(binder, table_name, schema, position) {
        let table_name_node = table_name_ptr.to_node(root);

        if let Some(create_table) = table_name_node
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

        if let Some(create_table_as) = table_name_node
            .ancestors()
            .find_map(ast::CreateTableAs::cast)
        {
            let select = resolve_select_clause(create_table_as.query()?)?;

            if let Some(target_list) = select.select_clause().and_then(|c| c.target_list()) {
                return Some(target_list.targets().count());
            }
        }
    }

    if let Some(view_name_ptr) = resolve_view_name_ptr(binder, table_name, schema, position) {
        let view_name_node = view_name_ptr.to_node(root);

        if let Some(create_view) = view_name_node.ancestors().find_map(ast::CreateView::cast) {
            if let Some(column_list) = create_view.column_list() {
                return Some(column_list.columns().count());
            }

            let select = resolve_select_clause(create_view.query()?)?;

            if let Some(target_list) = select.select_clause().and_then(|c| c.target_list()) {
                // This is not quite right if there's a `*` in the view definition.
                // It becomes recursive.
                // For now, let's assume simple views.
                return Some(target_list.targets().count());
            }
        }
    }
    None
}

fn resolve_select_clause(query: SelectVariant) -> Option<ast::Select> {
    match query {
        ast::SelectVariant::Select(s) => Some(s),
        ast::SelectVariant::ParenSelect(ps) => match ps.select()? {
            ast::SelectVariant::Select(s) => Some(s),
            _ => return None,
        },
        _ => return None,
    }
}

fn resolve_cte_column(
    binder: &Binder,
    root: &SyntaxNode,
    name_ref: &ast::NameRef,
    cte_name: &Name,
    column_name: &Name,
) -> Option<SyntaxNodePtr> {
    let with_clause = find_parent_with_clause(name_ref.syntax())?;

    for with_table in with_clause.with_tables() {
        if let Some(name) = with_table.name()
            && Name::from_node(&name) == *cte_name
        {
            // Skip if we're inside this CTE's definition (CTE doesn't shadow itself)
            if with_table
                .syntax()
                .text_range()
                .contains_range(name_ref.syntax().text_range())
            {
                continue;
            }

            let column_list_len = if let Some(column_list) = with_table.column_list() {
                for column in column_list.columns() {
                    if let Some(col_name) = column.name()
                        && Name::from_node(&col_name) == *column_name
                    {
                        return Some(SyntaxNodePtr::new(col_name.syntax()));
                    }
                }
                column_list.columns().count()
            } else {
                0
            };

            let query = with_table.query()?;

            if let ast::WithQuery::Values(values) = query {
                if column_list_len > 0 {
                    continue;
                }
                if let Some(num_str) = column_name.0.strip_prefix("column")
                    && let Ok(col_num) = num_str.parse::<usize>()
                    && col_num > 0
                    && let Some(row_list) = values.row_list()
                    && let Some(first_row) = row_list.rows().next()
                    && let Some(expr) = first_row.exprs().nth(col_num - 1)
                {
                    return Some(SyntaxNodePtr::new(expr.syntax()));
                }
                continue;
            }

            if let ast::WithQuery::Table(table) = query {
                let path = table.relation_name()?.path()?;
                let (table_name, schema) = extract_table_schema_from_path(&path)?;

                if schema.is_none()
                    && let Some(nested_cte_column) =
                        resolve_cte_column(binder, root, name_ref, &table_name, column_name)
                {
                    return Some(nested_cte_column);
                }

                return resolve_column_from_table_or_view(
                    binder,
                    root,
                    name_ref,
                    &table_name,
                    &schema,
                    column_name,
                );
            }

            if let Some(column) =
                column_in_with_query(&query, binder, root, column_name, column_list_len)
            {
                return Some(column);
            }

            let Some(cte_select) = select_from_with_query(query) else {
                continue;
            };

            let select_clause = cte_select.select_clause()?;
            let target_list = select_clause.target_list()?;
            let from_clause = cte_select.from_clause();
            let mut column_index: usize = 0;

            for target in target_list.targets() {
                // Skip targets that are covered by the column list
                let target_column_count = from_clause
                    .as_ref()
                    .and_then(|from_clause| {
                        count_columns_for_target(binder, root, name_ref, &target, from_clause)
                    })
                    .unwrap_or(1);
                let column_list_end = column_index.saturating_add(target_column_count);
                if column_list_end <= column_list_len {
                    column_index = column_list_end;
                    continue;
                }

                if let Some((col_name, node)) = ColumnName::from_target(target.clone()) {
                    if let Some(col_name_str) = col_name.to_string()
                        && Name::from_string(col_name_str) == *column_name
                    {
                        return Some(SyntaxNodePtr::new(&node));
                    }

                    if matches!(col_name, ColumnName::Star)
                        && let Some(from_clause) = &from_clause
                        && let Some(result) = resolve_from_clause_for_cte_star(
                            binder,
                            root,
                            name_ref,
                            cte_name,
                            column_name,
                            from_clause,
                        )
                    {
                        return Some(result);
                    }
                }
                if let Some(expr) = target.expr()
                    && let ast::Expr::FieldExpr(field_expr) = expr
                    && let Some(table_name) = qualified_star_table_name(&field_expr)
                    && let Some(from_clause) = &from_clause
                    && let Some(result) = resolve_qualified_star_in_from_clause(
                        binder,
                        root,
                        name_ref,
                        cte_name,
                        column_name,
                        from_clause,
                        &table_name,
                    )
                {
                    return Some(result);
                }

                column_index = column_list_end;
            }
        }
    }

    None
}

fn column_in_with_query(
    query: &ast::WithQuery,
    binder: &Binder,
    root: &SyntaxNode,
    column_name: &Name,
    column_list_len: usize,
) -> Option<SyntaxNodePtr> {
    let (returning_clause, path) = match query {
        ast::WithQuery::Delete(delete) => (
            delete.returning_clause(),
            delete.relation_name().and_then(|r| r.path()),
        ),
        ast::WithQuery::Insert(insert) => (insert.returning_clause(), insert.path()),
        ast::WithQuery::Merge(merge) => (
            merge.returning_clause(),
            merge.relation_name().and_then(|r| r.path()),
        ),
        ast::WithQuery::Update(update) => (
            update.returning_clause(),
            update.relation_name().and_then(|r| r.path()),
        ),
        ast::WithQuery::Select(_)
        | ast::WithQuery::CompoundSelect(_)
        | ast::WithQuery::Table(_)
        | ast::WithQuery::Values(_)
        | ast::WithQuery::ParenSelect(_) => return None,
    };

    let target_list = returning_clause?.target_list()?;
    let path = path?;
    let mut column_index: usize = 0;
    for target in target_list.targets() {
        let target_column_count = if target.star_token().is_some() {
            count_columns_for_path(binder, root, &path).unwrap_or(1)
        } else {
            1
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
                return Some(SyntaxNodePtr::new(&node));
            }
            if matches!(col_name, ColumnName::Star)
                && let Some(ptr) = resolve_column_for_path(binder, root, &path, column_name.clone())
            {
                return Some(ptr);
            }
        }
        column_index = column_list_end;
    }

    None
}

fn resolve_subquery_column_ptr(
    binder: &Binder,
    root: &SyntaxNode,
    paren_select: &ast::ParenSelect,
    name_ref: &ast::NameRef,
    column_name: &Name,
    alias: Option<&ast::Alias>,
) -> Option<SyntaxNodePtr> {
    let select_variant = paren_select.select()?;

    if let Some(alias) = alias
        && let Some(column_list) = alias.column_list()
    {
        for col in column_list.columns() {
            if let Some(col_name) = col.name()
                && Name::from_node(&col_name) == *column_name
            {
                return Some(SyntaxNodePtr::new(col_name.syntax()));
            }
        }
        if matches!(select_variant, ast::SelectVariant::Values(_)) {
            return None;
        }
    }

    // TODO: this should just be a match stmt
    if let ast::SelectVariant::Table(table) = select_variant {
        let path = table.relation_name()?.path()?;
        let (table_name, schema) = extract_table_schema_from_path(&path)?;

        if schema.is_none()
            && let Some(cte_column_ptr) =
                resolve_cte_column(binder, root, name_ref, &table_name, column_name)
        {
            return Some(cte_column_ptr);
        }

        return resolve_column_from_table_or_view(
            binder,
            root,
            name_ref,
            &table_name,
            &schema,
            column_name,
        );
    }

    if let ast::SelectVariant::Values(values) = select_variant {
        if let Some(num_str) = column_name.0.strip_prefix("column")
            && let Ok(col_num) = num_str.parse::<usize>()
            && col_num > 0
            && let Some(row_list) = values.row_list()
            && let Some(first_row) = row_list.rows().next()
            && let Some(expr) = first_row.exprs().nth(col_num - 1)
        {
            return Some(SyntaxNodePtr::new(expr.syntax()));
        }
        return None;
    }

    let ast::SelectVariant::Select(subquery_select) = select_variant else {
        return None;
    };

    let select_clause = subquery_select.select_clause()?;
    let target_list = select_clause.target_list()?;

    for target in target_list.targets() {
        if let Some((col_name, node)) = ColumnName::from_target(target.clone()) {
            if let Some(col_name_str) = col_name.to_string()
                && Name::from_string(col_name_str) == *column_name
            {
                return Some(SyntaxNodePtr::new(&node));
            }
            if matches!(col_name, ColumnName::Star) {
                if let Some(from_clause) = subquery_select.from_clause() {
                    for from_item in from_clause.from_items() {
                        if let Some(result) =
                            resolve_from_item_column_ptr(binder, root, &from_item, name_ref)
                        {
                            return Some(result);
                        }
                    }

                    for join_expr in from_clause.join_exprs() {
                        if let Some(result) =
                            resolve_from_join_expr(&join_expr, &|from_item: &ast::FromItem| {
                                resolve_from_item_column_ptr(binder, root, from_item, name_ref)
                            })
                        {
                            return Some(result);
                        }
                    }
                }
            }
        }

        if let Some(expr) = target.expr()
            && let ast::Expr::FieldExpr(field_expr) = expr
            && let Some(table_name) = qualified_star_table_name(&field_expr)
            && let Some(from_clause) = subquery_select.from_clause()
            && let Some(from_item) = find_from_item_in_from_clause(&from_clause, &table_name)
            && let Some(result) = resolve_from_item_column_ptr(binder, root, &from_item, name_ref)
        {
            return Some(result);
        }
    }

    None
}

fn qualified_star_table_name(field_expr: &ast::FieldExpr) -> Option<Name> {
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

pub(crate) fn resolve_qualified_star_table_ptr(
    binder: &Binder,
    field_expr: &ast::FieldExpr,
) -> Option<SyntaxNodePtr> {
    let table_name = qualified_star_table_name(field_expr)?;
    let position = field_expr.syntax().text_range().start();

    for ancestor in field_expr.syntax().ancestors() {
        if let Some(select) = ast::Select::cast(ancestor.clone()) {
            let from_clause = select.from_clause()?;
            let from_item = find_from_item_in_from_clause(&from_clause, &table_name)?;
            let (table_name, schema) = table_and_schema_from_from_item(&from_item)?;

            if let Some(table_name_ptr) =
                resolve_table_name_ptr(binder, &table_name, &schema, position)
            {
                return Some(table_name_ptr);
            }

            if let Some(view_name_ptr) =
                resolve_view_name_ptr(binder, &table_name, &schema, position)
            {
                return Some(view_name_ptr);
            }

            if schema.is_none()
                && let Some(name_ref) = from_item.name_ref()
            {
                return resolve_cte_table(&name_ref, &table_name);
            }

            return None;
        }

        if let Some(update) = ast::Update::cast(ancestor.clone()) {
            let path = update.relation_name()?.path()?;
            return resolve_table_or_view_or_cte_ptrs(binder, position, &path)?
                .into_iter()
                .next();
        }

        if let Some(insert) = ast::Insert::cast(ancestor.clone()) {
            let path = insert.path()?;
            return resolve_table_or_view_or_cte_ptrs(binder, position, &path)?
                .into_iter()
                .next();
        }

        if let Some(delete) = ast::Delete::cast(ancestor.clone()) {
            let path = delete.relation_name()?.path()?;
            return resolve_table_or_view_or_cte_ptrs(binder, position, &path)?
                .into_iter()
                .next();
        }

        if let Some(merge) = ast::Merge::cast(ancestor) {
            let path = merge.relation_name()?.path()?;
            return resolve_table_or_view_or_cte_ptrs(binder, position, &path)?
                .into_iter()
                .next();
        }
    }

    None
}

fn resolve_table_or_view_or_cte_ptrs(
    binder: &Binder,
    position: TextSize,
    path: &ast::Path,
) -> Option<Vec<SyntaxNodePtr>> {
    let (table_name, schema) = extract_table_schema_from_path(path)?;

    let mut results = vec![];

    if let Some(table_name_ptr) = resolve_table_name_ptr(binder, &table_name, &schema, position) {
        results.push(table_name_ptr);
    }

    if let Some(view_name_ptr) = resolve_view_name_ptr(binder, &table_name, &schema, position) {
        results.push(view_name_ptr);
    }

    if schema.is_none()
        && let Some(segment) = path.segment()
        && let Some(name_ref) = segment.name_ref()
        && let Some(cte_ptr) = resolve_cte_table(&name_ref, &table_name)
    {
        results.push(cte_ptr);
    }

    if results.is_empty() {
        return None;
    }

    Some(results)
}

fn resolve_table_from_update_ptrs(
    binder: &Binder,
    position: TextSize,
    update: &ast::Update,
) -> Option<Vec<SyntaxNodePtr>> {
    let path = update.relation_name()?.path()?;
    resolve_table_or_view_or_cte_ptrs(binder, position, &path)
}

fn resolve_table_from_insert_ptrs(
    binder: &Binder,
    position: TextSize,
    insert: &ast::Insert,
) -> Option<Vec<SyntaxNodePtr>> {
    let path = insert.path()?;
    resolve_table_or_view_or_cte_ptrs(binder, position, &path)
}

fn resolve_table_from_delete_ptrs(
    binder: &Binder,
    position: TextSize,
    delete: &ast::Delete,
) -> Option<Vec<SyntaxNodePtr>> {
    let path = delete.relation_name()?.path()?;
    resolve_table_or_view_or_cte_ptrs(binder, position, &path)
}

fn resolve_table_from_merge_ptrs(
    binder: &Binder,
    position: TextSize,
    merge: &ast::Merge,
) -> Option<Vec<SyntaxNodePtr>> {
    let path = merge.relation_name()?.path()?;
    resolve_table_or_view_or_cte_ptrs(binder, position, &path)
}

pub(crate) fn resolve_unqualified_star_table_ptrs(
    binder: &Binder,
    target: &ast::Target,
) -> Option<Vec<SyntaxNodePtr>> {
    target.star_token()?;

    let position = target.syntax().text_range().start();

    for ancestor in target.syntax().ancestors() {
        if let Some(select) = ast::Select::cast(ancestor.clone()) {
            let from_clause = select.from_clause()?;
            let results = table_ptrs_from_clause(binder, &from_clause);
            if results.is_empty() {
                return None;
            }

            return Some(results);
        }

        if let Some(update) = ast::Update::cast(ancestor.clone()) {
            return resolve_table_from_update_ptrs(binder, position, &update);
        }

        if let Some(insert) = ast::Insert::cast(ancestor.clone()) {
            return resolve_table_from_insert_ptrs(binder, position, &insert);
        }

        if let Some(delete) = ast::Delete::cast(ancestor.clone()) {
            return resolve_table_from_delete_ptrs(binder, position, &delete);
        }

        if let Some(merge) = ast::Merge::cast(ancestor) {
            return resolve_table_from_merge_ptrs(binder, position, &merge);
        }
    }

    None
}

pub(crate) fn resolve_unqualified_star_in_arg_list_ptrs(
    binder: &Binder,
    arg_list: &ast::ArgList,
) -> Option<Vec<SyntaxNodePtr>> {
    let select = arg_list.syntax().ancestors().find_map(ast::Select::cast)?;
    let from_clause = select.from_clause()?;
    let results = table_ptrs_from_clause(binder, &from_clause);

    if results.is_empty() {
        return None;
    }

    Some(results)
}

pub(crate) fn table_ptrs_from_clause(
    binder: &Binder,
    from_clause: &ast::FromClause,
) -> Vec<SyntaxNodePtr> {
    let mut results = vec![];

    for from_item in from_clause.from_items() {
        collect_tables_from_item(binder, &from_item, &mut results);
    }

    for join_expr in from_clause.join_exprs() {
        collect_table_ptrs_from_join_expr(binder, &join_expr, &mut results);
    }

    results
}

fn collect_table_ptrs_from_join_expr(
    binder: &Binder,
    join_expr: &ast::JoinExpr,
    results: &mut Vec<SyntaxNodePtr>,
) {
    if let Some(nested) = join_expr.join_expr() {
        collect_table_ptrs_from_join_expr(binder, &nested, results);
    }

    if let Some(from_item) = join_expr.from_item() {
        collect_tables_from_item(binder, &from_item, results);
    }

    if let Some(join) = join_expr.join()
        && let Some(from_item) = join.from_item()
    {
        collect_tables_from_item(binder, &from_item, results);
    }
}

fn collect_tables_from_item(
    binder: &Binder,
    from_item: &ast::FromItem,
    results: &mut Vec<SyntaxNodePtr>,
) {
    if let Some(paren_select) = from_item.paren_select() {
        results.push(SyntaxNodePtr::new(paren_select.syntax()));
        return;
    }

    let Some((table_name, schema)) = table_and_schema_from_from_item(from_item) else {
        return;
    };

    let position = from_item.syntax().text_range().start();
    if let Some(table_name_ptr) = resolve_table_name_ptr(binder, &table_name, &schema, position) {
        results.push(table_name_ptr);
        return;
    }

    if let Some(view_name_ptr) = resolve_view_name_ptr(binder, &table_name, &schema, position) {
        results.push(view_name_ptr);
        return;
    }

    if schema.is_none()
        && let Some(name_ref) = from_item.name_ref()
        && let Some(cte_ptr) = resolve_cte_table(&name_ref, &table_name)
    {
        results.push(cte_ptr);
        return;
    }
}

pub(crate) enum TableSource {
    WithTable(ast::WithTable),
    CreateView(ast::CreateView),
    CreateMaterializedView(ast::CreateMaterializedView),
    CreateTable(ast::CreateTableLike),
    ParenSelect(ast::ParenSelect),
}

pub(crate) fn find_table_source(node: &SyntaxNode) -> Option<TableSource> {
    if let Some(paren_select) = ast::ParenSelect::cast(node.clone()) {
        return Some(TableSource::ParenSelect(paren_select));
    }

    for ancestor in node.ancestors() {
        if let Some(with_table) = ast::WithTable::cast(ancestor.clone()) {
            return Some(TableSource::WithTable(with_table));
        }

        if let Some(create_view) = ast::CreateView::cast(ancestor.clone()) {
            return Some(TableSource::CreateView(create_view));
        }

        if let Some(create_materialized_view) = ast::CreateMaterializedView::cast(ancestor.clone())
        {
            return Some(TableSource::CreateMaterializedView(
                create_materialized_view,
            ));
        }

        if let Some(create_table) = ast::CreateTableLike::cast(ancestor.clone()) {
            return Some(TableSource::CreateTable(create_table));
        }
    }

    None
}

pub(crate) fn select_from_with_query(query: ast::WithQuery) -> Option<ast::Select> {
    let select_variant = match query {
        ast::WithQuery::Select(s) => ast::SelectVariant::Select(s),
        ast::WithQuery::ParenSelect(ps) => ps.select()?,
        ast::WithQuery::CompoundSelect(compound) => compound.lhs()?,
        _ => return None,
    };

    match select_variant {
        ast::SelectVariant::Select(s) => Some(s),
        ast::SelectVariant::CompoundSelect(compound) => match compound.lhs()? {
            ast::SelectVariant::Select(s) => Some(s),
            _ => None,
        },
        _ => None,
    }
}

fn count_columns_for_target(
    binder: &Binder,
    root: &SyntaxNode,
    name_ref: &ast::NameRef,
    target: &ast::Target,
    from_clause: &ast::FromClause,
) -> Option<usize> {
    if target.star_token().is_some() {
        return count_columns_for_from_clause(binder, root, name_ref, from_clause);
    }

    if let Some(expr) = target.expr()
        && let ast::Expr::FieldExpr(field_expr) = expr
        && let Some(table_name) = qualified_star_table_name(&field_expr)
        && let Some(from_item) = find_from_item_in_from_clause(from_clause, &table_name)
    {
        return count_columns_for_from_item(binder, root, name_ref, &from_item);
    }

    Some(1)
}

fn count_columns_for_from_clause(
    binder: &Binder,
    root: &SyntaxNode,
    name_ref: &ast::NameRef,
    from_clause: &ast::FromClause,
) -> Option<usize> {
    let mut total: usize = 0;
    let mut found = false;

    for from_item in from_clause.from_items() {
        if let Some(count) = count_columns_for_from_item(binder, root, name_ref, &from_item) {
            total = total.saturating_add(count);
            found = true;
        }
    }

    for join_expr in from_clause.join_exprs() {
        if let Some(count) = count_columns_for_join_expr(binder, root, name_ref, &join_expr) {
            total = total.saturating_add(count);
            found = true;
        }
    }

    found.then_some(total)
}

fn count_columns_for_join_expr(
    binder: &Binder,
    root: &SyntaxNode,
    name_ref: &ast::NameRef,
    join_expr: &ast::JoinExpr,
) -> Option<usize> {
    let mut total: usize = 0;
    let mut found = false;

    if let Some(nested) = join_expr.join_expr()
        && let Some(count) = count_columns_for_join_expr(binder, root, name_ref, &nested)
    {
        total = total.saturating_add(count);
        found = true;
    }

    if let Some(from_item) = join_expr.from_item()
        && let Some(count) = count_columns_for_from_item(binder, root, name_ref, &from_item)
    {
        total = total.saturating_add(count);
        found = true;
    }

    if let Some(join) = join_expr.join()
        && let Some(from_item) = join.from_item()
        && let Some(count) = count_columns_for_from_item(binder, root, name_ref, &from_item)
    {
        total = total.saturating_add(count);
        found = true;
    }

    found.then_some(total)
}

fn count_columns_for_from_item(
    binder: &Binder,
    root: &SyntaxNode,
    name_ref: &ast::NameRef,
    from_item: &ast::FromItem,
) -> Option<usize> {
    let (table_name, schema) = table_and_schema_from_from_item(from_item)?;
    let position = name_ref.syntax().text_range().start();

    if let Some(count) = count_columns_for_table_name(binder, root, &table_name, &schema, position)
    {
        return Some(count);
    }

    if schema.is_none()
        && let Some(cte_column_count) = count_columns_for_cte(name_ref, &table_name)
    {
        return Some(cte_column_count);
    }

    None
}

fn count_columns_for_cte(name_ref: &ast::NameRef, cte_name: &Name) -> Option<usize> {
    let with_clause = find_parent_with_clause(name_ref.syntax())?;

    for with_table in with_clause.with_tables() {
        if let Some(name) = with_table.name()
            && Name::from_node(&name) == *cte_name
        {
            if with_table
                .syntax()
                .text_range()
                .contains_range(name_ref.syntax().text_range())
            {
                return None;
            }

            if let Some(column_list) = with_table.column_list() {
                return Some(column_list.columns().count());
            }

            let query = with_table.query()?;

            if let ast::WithQuery::Values(values) = query {
                if let Some(row_list) = values.row_list()
                    && let Some(first_row) = row_list.rows().next()
                {
                    return Some(first_row.exprs().count());
                }
                return None;
            }

            let select = select_from_with_query(query)?;

            if let Some(target_list) = select.select_clause().and_then(|c| c.target_list()) {
                return Some(target_list.targets().count());
            }
        }
    }

    None
}

fn resolve_from_clause_for_cte_star(
    binder: &Binder,
    root: &SyntaxNode,
    name_ref: &ast::NameRef,
    cte_name: &Name,
    column_name: &Name,
    from_clause: &ast::FromClause,
) -> Option<SyntaxNodePtr> {
    for from_item in from_clause.from_items() {
        if let Some(result) = resolve_from_item_for_cte_star(
            binder,
            root,
            &from_item,
            name_ref,
            cte_name,
            column_name,
        ) {
            return Some(result);
        }
    }

    for join_expr in from_clause.join_exprs() {
        if let Some(result) = resolve_from_join_expr(&join_expr, &|from_item: &ast::FromItem| {
            resolve_from_item_for_cte_star(binder, root, from_item, name_ref, cte_name, column_name)
        }) {
            return Some(result);
        }
    }

    None
}

fn resolve_qualified_star_in_from_clause(
    binder: &Binder,
    root: &SyntaxNode,
    name_ref: &ast::NameRef,
    cte_name: &Name,
    column_name: &Name,
    from_clause: &ast::FromClause,
    table_name: &Name,
) -> Option<SyntaxNodePtr> {
    let from_item = find_from_item_in_from_clause(from_clause, table_name)?;
    resolve_from_item_for_cte_star(binder, root, &from_item, name_ref, cte_name, column_name)
}

fn resolve_column_from_paren_expr(
    binder: &Binder,
    root: &SyntaxNode,
    paren_expr: &ast::ParenExpr,
    name_ref: &ast::NameRef,
    column_name: &Name,
) -> Option<SyntaxNodePtr> {
    if let Some(select) = paren_expr.select() {
        if let Some(select_clause) = select.select_clause()
            && let Some(target_list) = select_clause.target_list()
        {
            for target in target_list.targets() {
                if let Some((col_name, node)) = ColumnName::from_target(target.clone())
                    && let Some(col_name_str) = col_name.to_string()
                    && Name::from_string(col_name_str) == *column_name
                {
                    return Some(SyntaxNodePtr::new(&node));
                }
            }
        }
        return None;
    }

    if let Some(ast::Expr::ParenExpr(paren_expr)) = paren_expr.expr() {
        return resolve_column_from_paren_expr(binder, root, &paren_expr, name_ref, column_name);
    }

    if let Some(from_item) = paren_expr.from_item()
        && let Some(paren_select) = from_item.paren_select()
    {
        let alias = from_item.alias();
        return resolve_subquery_column_ptr(
            binder,
            root,
            &paren_select,
            name_ref,
            column_name,
            alias.as_ref(),
        );
    }

    None
}

pub(crate) fn resolve_insert_create_table(
    root: &SyntaxNode,
    binder: &Binder,
    insert: &ast::Insert,
) -> Option<ast::CreateTableLike> {
    let path = insert.path()?;
    let (table_name, schema) = extract_table_schema_from_path(&path)?;
    let position = insert.syntax().text_range().start();

    let table_name_ptr = resolve_table_name_ptr(binder, &table_name, &schema, position)?;
    let table_name_node = table_name_ptr.to_node(root);

    table_name_node
        .ancestors()
        .find_map(ast::CreateTableLike::cast)
}

pub(crate) fn resolve_table_info(binder: &Binder, path: &ast::Path) -> Option<(Schema, String)> {
    resolve_symbol_info(binder, path, SymbolKind::Table)
}

pub(crate) fn resolve_function_info(binder: &Binder, path: &ast::Path) -> Option<(Schema, String)> {
    resolve_symbol_info(binder, path, SymbolKind::Function)
}

pub(crate) fn resolve_aggregate_info(
    binder: &Binder,
    path: &ast::Path,
) -> Option<(Schema, String)> {
    resolve_symbol_info(binder, path, SymbolKind::Aggregate)
}

pub(crate) fn resolve_procedure_info(
    binder: &Binder,
    path: &ast::Path,
) -> Option<(Schema, String)> {
    resolve_symbol_info(binder, path, SymbolKind::Procedure)
}

pub(crate) fn resolve_type_info(binder: &Binder, path: &ast::Path) -> Option<(Schema, String)> {
    resolve_symbol_info(binder, path, SymbolKind::Type)
}

pub(crate) fn resolve_view_info(binder: &Binder, path: &ast::Path) -> Option<(Schema, String)> {
    resolve_symbol_info(binder, path, SymbolKind::View)
}

pub(crate) fn resolve_sequence_info(binder: &Binder, path: &ast::Path) -> Option<(Schema, String)> {
    resolve_symbol_info(binder, path, SymbolKind::Sequence)
}

pub(crate) fn collect_table_columns(
    binder: &Binder,
    root: &SyntaxNode,
    create_table: &impl ast::HasCreateTable,
) -> Vec<ast::Column> {
    collect_table_columns_impl(binder, root, create_table, 0)
}

// TODO: combine with find_column_in_create_table_impl
fn collect_table_columns_impl(
    binder: &Binder,
    root: &SyntaxNode,
    create_table: &impl ast::HasCreateTable,
    depth: usize,
) -> Vec<ast::Column> {
    if depth > 40 {
        log::info!("max depth reached, probably in a cycle");
        return vec![];
    }

    let mut columns = vec![];

    if let Some(inherits) = create_table.inherits() {
        for path in inherits.paths() {
            if let Some((table_name, schema)) = extract_table_schema_from_path(&path) {
                let position = path.syntax().text_range().start();
                if let Some(ResolvedTableName::Table(parent_table)) =
                    resolve_table_name(binder, root, &table_name, &schema, position)
                {
                    let inherited_columns =
                        collect_table_columns_impl(binder, root, &parent_table, depth + 1);
                    columns.extend(inherited_columns);
                }
            }
        }
    }

    if let Some(arg_list) = create_table.table_arg_list() {
        for arg in arg_list.args() {
            match &arg {
                ast::TableArg::Column(column) => {
                    columns.push(column.clone());
                }
                ast::TableArg::LikeClause(like_clause) => {
                    if let Some(path) = like_clause.path()
                        && let Some((table_name, schema)) = extract_table_schema_from_path(&path)
                    {
                        let position = path.syntax().text_range().start();
                        if let Some(ResolvedTableName::Table(source_table)) =
                            resolve_table_name(binder, root, &table_name, &schema, position)
                        {
                            let like_columns =
                                collect_table_columns_impl(binder, root, &source_table, depth + 1);
                            columns.extend(like_columns);
                        }
                    }
                }
                ast::TableArg::TableConstraint(_) => (),
            }
        }
    }

    columns
}

pub(crate) fn collect_view_column_names(create_view: &ast::CreateView) -> Vec<Name> {
    if let Some(column_list) = create_view.column_list() {
        let columns = collect_column_names_from_column_list(&column_list);
        if !columns.is_empty() {
            return columns;
        }
    }

    let Some(select) = select_from_view_query(create_view) else {
        return vec![];
    };
    let Some(select_clause) = select.select_clause() else {
        return vec![];
    };
    let Some(target_list) = select_clause.target_list() else {
        return vec![];
    };

    collect_target_list_column_names(&target_list)
}

pub(crate) fn collect_view_columns_with_types(
    create_view: &ast::CreateView,
) -> Vec<(Name, Option<Type>)> {
    let Some(select) = select_from_view_query(create_view) else {
        return vec![];
    };
    let Some(select_clause) = select.select_clause() else {
        return vec![];
    };
    let Some(target_list) = select_clause.target_list() else {
        return vec![];
    };

    collect_target_list_columns_with_types(&target_list)
}

pub(crate) fn collect_materialized_view_column_names(
    create_materialized_view: &ast::CreateMaterializedView,
) -> Vec<Name> {
    if let Some(column_list) = create_materialized_view.column_list() {
        let columns = collect_column_names_from_column_list(&column_list);
        if !columns.is_empty() {
            return columns;
        }
    }

    let Some(select) = select_from_materialized_view_query(create_materialized_view) else {
        return vec![];
    };
    let Some(select_clause) = select.select_clause() else {
        return vec![];
    };
    let Some(target_list) = select_clause.target_list() else {
        return vec![];
    };

    collect_target_list_column_names(&target_list)
}

pub(crate) fn collect_materialized_view_columns_with_types(
    create_materialized_view: &ast::CreateMaterializedView,
) -> Vec<(Name, Option<Type>)> {
    let Some(select) = select_from_materialized_view_query(create_materialized_view) else {
        return vec![];
    };
    let Some(select_clause) = select.select_clause() else {
        return vec![];
    };
    let Some(target_list) = select_clause.target_list() else {
        return vec![];
    };

    collect_target_list_columns_with_types(&target_list)
}

fn select_from_materialized_view_query(
    create_materialized_view: &ast::CreateMaterializedView,
) -> Option<ast::Select> {
    let query = create_materialized_view.query()?;
    match query {
        ast::SelectVariant::Select(select) => Some(select),
        _ => None,
    }
}

pub(crate) fn collect_with_table_column_names(with_table: &ast::WithTable) -> Vec<Name> {
    if let Some(column_list) = with_table.column_list() {
        let columns = collect_column_names_from_column_list(&column_list);
        if !columns.is_empty() {
            return columns;
        }
    }

    let Some(query) = with_table.query() else {
        return vec![];
    };

    if let ast::WithQuery::Values(values) = query {
        let mut results = vec![];
        if let Some(row_list) = values.row_list()
            && let Some(first_row) = row_list.rows().next()
        {
            for (idx, _expr) in first_row.exprs().enumerate() {
                results.push(Name::from_string(format!("column{}", idx + 1)));
            }
        }
        return results;
    }

    if let Some(columns) = columns_from_returning_clause(&query) {
        return columns;
    }

    let Some(cte_select) = select_from_with_query(query) else {
        return vec![];
    };
    let Some(select_clause) = cte_select.select_clause() else {
        return vec![];
    };
    let Some(target_list) = select_clause.target_list() else {
        return vec![];
    };

    collect_target_list_column_names(&target_list)
}

fn columns_from_returning_clause(query: &ast::WithQuery) -> Option<Vec<Name>> {
    let returning_clause = match query {
        ast::WithQuery::Delete(delete) => delete.returning_clause(),
        ast::WithQuery::Insert(insert) => insert.returning_clause(),
        ast::WithQuery::Merge(merge) => merge.returning_clause(),
        ast::WithQuery::Update(update) => update.returning_clause(),
        ast::WithQuery::Select(_)
        | ast::WithQuery::CompoundSelect(_)
        | ast::WithQuery::Table(_)
        | ast::WithQuery::Values(_)
        | ast::WithQuery::ParenSelect(_) => None,
    };
    if let Some(returning_clause) = returning_clause {
        if let Some(target_list) = returning_clause.target_list() {
            return Some(collect_target_list_column_names(&target_list));
        }
        return Some(vec![]);
    }
    None
}

fn resolve_symbol_info(
    binder: &Binder,
    path: &ast::Path,
    kind: SymbolKind,
) -> Option<(Schema, String)> {
    let name_str = extract_table_name_from_path(path)?;
    let schema = extract_schema_from_path(path);
    let position = path.syntax().text_range().start();
    binder.lookup_info(name_str, &schema, kind, position)
}

fn collect_column_names_from_column_list(column_list: &ast::ColumnList) -> Vec<Name> {
    let mut columns = vec![];
    for column in column_list.columns() {
        if let Some(name) = column.name() {
            columns.push(Name::from_node(&name));
        }
    }
    columns
}

fn collect_target_list_column_names(target_list: &ast::TargetList) -> Vec<Name> {
    let mut columns = vec![];
    for target in target_list.targets() {
        if let Some((col_name, _node)) = ColumnName::from_target(target)
            && let Some(col_name_str) = col_name.to_string()
        {
            columns.push(Name::from_string(col_name_str));
        }
    }
    columns
}

pub(crate) fn collect_with_table_columns_with_types(
    with_table: &ast::WithTable,
) -> Vec<(Name, Option<Type>)> {
    let Some(query) = with_table.query() else {
        return vec![];
    };

    if let ast::WithQuery::Values(values) = query {
        let mut results = vec![];
        if let Some(row_list) = values.row_list()
            && let Some(first_row) = row_list.rows().next()
        {
            for (idx, expr) in first_row.exprs().enumerate() {
                let name = Name::from_string(format!("column{}", idx + 1));
                let ty = infer_type_from_expr(&expr);
                results.push((name, ty));
            }
        }
        return results;
    }

    let Some(cte_select) = select_from_with_query(query) else {
        return vec![];
    };
    let Some(select_clause) = cte_select.select_clause() else {
        return vec![];
    };
    let Some(target_list) = select_clause.target_list() else {
        return vec![];
    };

    collect_target_list_columns_with_types(&target_list)
}

fn collect_target_list_columns_with_types(
    target_list: &ast::TargetList,
) -> Vec<(Name, Option<Type>)> {
    let mut columns = vec![];
    for target in target_list.targets() {
        if let Some((col_name, _node)) = ColumnName::from_target(target.clone())
            && let Some(col_name_str) = col_name.to_string()
        {
            let ty = target.expr().and_then(|e| infer_type_from_expr(&e));
            columns.push((Name::from_string(col_name_str), ty));
        }
    }
    columns
}

pub(crate) fn collect_paren_select_columns_with_types(
    binder: &Binder,
    root: &SyntaxNode,
    paren_select: &ast::ParenSelect,
) -> Vec<(Name, Option<Type>)> {
    let Some(select_variant) = paren_select.select() else {
        return vec![];
    };
    collect_select_variant_columns_with_types(binder, root, &select_variant)
}

fn collect_select_variant_columns_with_types(
    binder: &Binder,
    root: &SyntaxNode,
    select_variant: &ast::SelectVariant,
) -> Vec<(Name, Option<Type>)> {
    match select_variant {
        ast::SelectVariant::Values(values) => {
            let mut results = vec![];
            if let Some(row_list) = values.row_list()
                && let Some(first_row) = row_list.rows().next()
            {
                for (idx, expr) in first_row.exprs().enumerate() {
                    let name = Name::from_string(format!("column{}", idx + 1));
                    let ty = infer_type_from_expr(&expr);
                    results.push((name, ty));
                }
            }
            results
        }
        ast::SelectVariant::Select(select) => {
            let Some(select_clause) = select.select_clause() else {
                return vec![];
            };
            let Some(target_list) = select_clause.target_list() else {
                return vec![];
            };
            collect_target_list_columns_with_types(&target_list)
        }
        ast::SelectVariant::SelectInto(select_into) => {
            let Some(select_clause) = select_into.select_clause() else {
                return vec![];
            };
            let Some(target_list) = select_clause.target_list() else {
                return vec![];
            };
            collect_target_list_columns_with_types(&target_list)
        }
        ast::SelectVariant::ParenSelect(nested) => {
            collect_paren_select_columns_with_types(binder, root, nested)
        }
        ast::SelectVariant::CompoundSelect(compound) => {
            let Some(lhs) = compound.lhs() else {
                return vec![];
            };
            collect_select_variant_columns_with_types(binder, root, &lhs)
        }
        ast::SelectVariant::Table(table) => {
            let Some(path) = table.relation_name().and_then(|r| r.path()) else {
                return vec![];
            };
            let Some((table_name, schema)) = extract_table_schema_from_path(&path) else {
                return vec![];
            };
            let position = table.syntax().text_range().start();
            let Some(table_ptr) =
                binder.lookup_with(&table_name, SymbolKind::Table, position, &schema)
            else {
                return vec![];
            };
            let Some(create_table) = table_ptr
                .to_node(root)
                .ancestors()
                .find_map(ast::CreateTableLike::cast)
            else {
                return vec![];
            };
            let columns = collect_table_columns(binder, root, &create_table);
            columns
                .into_iter()
                .filter_map(|col| {
                    let name = Name::from_node(&col.name()?);
                    let ty = col.ty().and_then(|t| infer_type_from_ty(&t));
                    Some((name, ty))
                })
                .collect()
        }
    }
}

fn select_from_view_query(create_view: &ast::CreateView) -> Option<ast::Select> {
    let query = create_view.query()?;
    match query {
        ast::SelectVariant::Select(select) => Some(select),
        ast::SelectVariant::ParenSelect(paren_select) => match paren_select.select()? {
            ast::SelectVariant::Select(select) => Some(select),
            _ => None,
        },
        _ => None,
    }
}

fn extract_table_name_from_path(path: &ast::Path) -> Option<String> {
    let segment = path.segment()?;
    if let Some(name_ref) = segment.name_ref() {
        return Some(name_ref.syntax().text().to_string());
    }
    if let Some(name) = segment.name() {
        return Some(name.syntax().text().to_string());
    }
    None
}

fn extract_schema_from_path(path: &ast::Path) -> Option<String> {
    let segment = path.qualifier().and_then(|q| q.segment())?;
    if let Some(name_ref) = segment.name_ref() {
        return Some(name_ref.syntax().text().to_string());
    }
    if let Some(name) = segment.name() {
        return Some(name.syntax().text().to_string());
    }
    None
}

fn extract_param_signature(node: &impl ast::HasParamList) -> Option<Vec<Name>> {
    let param_list = node.param_list()?;
    let mut params = vec![];
    for param in param_list.params() {
        if let Some(ty) = param.ty()
            && let ast::Type::PathType(path_type) = ty
            && let Some(path) = path_type.path()
            && let Some(segment) = path.segment()
            && let Some(name_ref) = segment.name_ref()
        {
            params.push(Name::from_node(&name_ref));
        }
    }
    (!params.is_empty()).then_some(params)
}

fn unwrap_paren_expr(expr: ast::Expr) -> Option<ast::NameRef> {
    let mut current = expr;
    for _ in 0..10_000 {
        match current {
            ast::Expr::ParenExpr(paren_expr) => {
                current = paren_expr.expr()?;
            }
            ast::Expr::NameRef(nr) => return Some(nr),
            _ => return None,
        }
    }
    None
}

fn resolve_composite_type_field_ptr(
    binder: &Binder,
    root: &SyntaxNode,
    field_name_ref: &ast::NameRef,
) -> Option<SyntaxNodePtr> {
    let field_name = Name::from_node(field_name_ref);
    let field_expr = field_name_ref
        .syntax()
        .parent()
        .and_then(ast::FieldExpr::cast)?;
    let base = field_expr.base()?;

    let base_name_ref = unwrap_paren_expr(base)?;

    let column_ptr = resolve_select_column_ptr(binder, root, &base_name_ref)?;
    let column_node = column_ptr.to_node(root);

    let (type_name, schema) =
        if let Some(type_info) = resolve_composite_type_from_column_node(&column_node) {
            type_info
        } else {
            resolve_composite_type_from_cast_node(&column_node)?
        };

    let position = field_name_ref.syntax().text_range().start();
    let type_name_ptr = resolve_type_name_ptr(binder, &type_name, &schema, position)?;
    let type_node = type_name_ptr.to_node(root);

    let create_type = type_node.ancestors().find_map(ast::CreateType::cast)?;
    let column_list = create_type.column_list()?;

    for column in column_list.columns() {
        if let Some(col_name) = column.name()
            && Name::from_node(&col_name) == field_name
        {
            return Some(SyntaxNodePtr::new(col_name.syntax()));
        }
    }

    None
}

fn resolve_composite_type_from_column_node(
    column_node: &SyntaxNode,
) -> Option<(Name, Option<Schema>)> {
    let column = column_node.ancestors().find_map(ast::Column::cast)?;
    let ty = column.ty()?;
    extract_type_name_and_schema(&ty)
}

fn resolve_composite_type_from_cast_node(
    column_node: &SyntaxNode,
) -> Option<(Name, Option<Schema>)> {
    let target = column_node.ancestors().find_map(ast::Target::cast)?;
    let ast::Expr::CastExpr(cast_expr) = target.expr()? else {
        return None;
    };
    let ty = cast_expr.ty()?;
    extract_type_name_and_schema(&ty)
}

fn extract_type_name_and_schema(ty: &ast::Type) -> Option<(Name, Option<Schema>)> {
    match ty {
        ast::Type::PathType(path_type) => {
            let path = path_type.path()?;
            let (type_name, schema) = extract_table_schema_from_path(&path)?;
            Some((type_name, schema))
        }
        ast::Type::ExprType(expr_type) => {
            let expr = expr_type.expr()?;
            if let ast::Expr::FieldExpr(field_expr) = expr
                && let Some(field) = field_expr.field()
                && let Some(ast::Expr::NameRef(schema_name_ref)) = field_expr.base()
            {
                let type_name = Name::from_node(&field);
                let schema = Some(Schema(Name::from_node(&schema_name_ref)));
                Some((type_name, schema))
            } else {
                None
            }
        }
        _ => None,
    }
}

fn resolve_merge_column_ptr(
    binder: &Binder,
    root: &SyntaxNode,
    column_name_ref: &ast::NameRef,
) -> Option<SyntaxNodePtr> {
    let column_name = Name::from_node(column_name_ref);
    let merge = column_name_ref
        .syntax()
        .ancestors()
        .find_map(ast::Merge::cast)?;

    // Try resolving in source table first
    if let Some(using_on) = merge.using_on_clause()
        && let Some(from_item) = using_on.from_item()
        && let Some(ptr) = resolve_from_item_column_ptr(binder, root, &from_item, column_name_ref)
    {
        return Some(ptr);
    }

    let path = merge.relation_name()?.path()?;
    resolve_column_for_path(binder, root, &path, column_name)
}

// TODO: I think we could use trait(s) here to simplify this and have the
// callers pass in the stmt instead of the fields.
fn resolve_table_in_returning_clause(
    binder: &Binder,
    table_name_ref: &ast::NameRef,
    alias: Option<ast::Alias>,
    path: &ast::Path,
    returning_clause: Option<ast::ReturningClause>,
) -> Option<SyntaxNodePtr> {
    let table_name = Name::from_node(table_name_ref);
    let (stmt_table_name, schema) = extract_table_schema_from_path(path)?;

    let matched =
        match_table_in_returning_clause(&table_name, &stmt_table_name, alias, returning_clause)?;

    let position = table_name_ref.syntax().text_range().start();

    match matched {
        ReturningClauseMatch::ReturningAlias(name) => Some(SyntaxNodePtr::new(name.syntax())),
        ReturningClauseMatch::TableAlias(alias_name) => {
            Some(SyntaxNodePtr::new(alias_name.syntax()))
        }
        ReturningClauseMatch::PseudoTable => {
            resolve_table_name_ptr(binder, &stmt_table_name, &schema, position)
        }
        ReturningClauseMatch::Table => {
            resolve_table_name_ptr(binder, &table_name, &schema, position)
        }
    }
}

fn resolve_insert_table_name_ptr(
    binder: &Binder,
    table_name_ref: &ast::NameRef,
) -> Option<SyntaxNodePtr> {
    let insert = table_name_ref
        .syntax()
        .ancestors()
        .find_map(ast::Insert::cast)?;
    let path = insert.path()?;
    resolve_table_in_returning_clause(
        binder,
        table_name_ref,
        insert.alias(),
        &path,
        insert.returning_clause(),
    )
}

fn resolve_delete_table_name_ptr(
    binder: &Binder,
    table_name_ref: &ast::NameRef,
) -> Option<SyntaxNodePtr> {
    let delete = table_name_ref
        .syntax()
        .ancestors()
        .find_map(ast::Delete::cast)?;
    let path = delete.relation_name()?.path()?;
    resolve_table_in_returning_clause(
        binder,
        table_name_ref,
        delete.alias(),
        &path,
        delete.returning_clause(),
    )
}

fn resolve_update_table_name_ptr(
    binder: &Binder,
    table_name_ref: &ast::NameRef,
) -> Option<SyntaxNodePtr> {
    let update = table_name_ref
        .syntax()
        .ancestors()
        .find_map(ast::Update::cast)?;
    let path = update.relation_name()?.path()?;
    resolve_table_in_returning_clause(
        binder,
        table_name_ref,
        update.alias(),
        &path,
        update.returning_clause(),
    )
}

fn resolve_merge_table_name_ptr(
    binder: &Binder,
    table_name_ref: &ast::NameRef,
) -> Option<SyntaxNodePtr> {
    let table_name = Name::from_node(table_name_ref);
    let merge = table_name_ref
        .syntax()
        .ancestors()
        .find_map(ast::Merge::cast)?;

    let path = merge.relation_name()?.path()?;

    // Check USING clause for the source table - MERGE-specific
    if let Some(using_on) = merge.using_on_clause()
        && let Some(from_item) = using_on.from_item()
    {
        if let Some(item_name_ref) = from_item.name_ref() {
            let item_name = Name::from_node(&item_name_ref);
            if item_name == table_name {
                if let Some(cte_ptr) = resolve_cte_table(table_name_ref, &table_name) {
                    return Some(cte_ptr);
                }
                let position = table_name_ref.syntax().text_range().start();
                return resolve_table_name_ptr(binder, &item_name, &None, position);
            }
        }
        // Check for aliased source tables
        if let Some(alias) = from_item.alias()
            && let Some(alias_name) = alias.name()
            && Name::from_node(&alias_name) == table_name
        {
            return Some(SyntaxNodePtr::new(alias_name.syntax()));
        }
    }

    resolve_table_in_returning_clause(
        binder,
        table_name_ref,
        merge.alias(),
        &path,
        merge.returning_clause(),
    )
}

fn find_func_call_from_named_arg(name_ref: &ast::NameRef) -> Option<(Name, Option<Schema>)> {
    for a in name_ref.syntax().ancestors() {
        if let Some(call_expr) = ast::CallExpr::cast(a.clone()) {
            return match call_expr.expr()? {
                ast::Expr::NameRef(func_name_ref) => {
                    let func_name = Name::from_node(&func_name_ref);
                    Some((func_name, None))
                }
                ast::Expr::FieldExpr(field_expr) => {
                    let func_name_ref = field_expr.field()?;
                    let func_name = Name::from_node(&func_name_ref);

                    let schema = if let Some(base) = field_expr.base()
                        && let ast::Expr::NameRef(schema_name_ref) = base
                    {
                        Some(Schema(Name::from_node(&schema_name_ref)))
                    } else {
                        None
                    };

                    Some((func_name, schema))
                }
                _ => None,
            };
        } else if let Some(call) = ast::Call::cast(a) {
            let path = call.path()?;
            let (function_name, schema) = extract_table_schema_from_path(&path)?;
            return Some((function_name, schema));
        }
    }
    None
}

fn find_param_in_func_def(
    root: &SyntaxNode,
    function_ptr: SyntaxNodePtr,
    param_name: &Name,
) -> Option<SyntaxNodePtr> {
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
