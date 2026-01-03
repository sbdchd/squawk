use rowan::TextSize;
use smallvec::{SmallVec, smallvec};
use squawk_syntax::{
    SyntaxNode, SyntaxNodePtr,
    ast::{self, AstNode},
};

use crate::binder::Binder;
use crate::classify::{NameRefClass, classify_name_ref};
use crate::column_name::ColumnName;
pub(crate) use crate::symbols::Schema;
use crate::symbols::{Name, SymbolKind};

pub(crate) fn resolve_name_ref(
    binder: &Binder,
    name_ref: &ast::NameRef,
) -> Option<SmallVec<[SyntaxNodePtr; 1]>> {
    let context = classify_name_ref(name_ref)?;

    match context {
        NameRefClass::DropTable
        | NameRefClass::Table
        | NameRefClass::CreateIndex
        | NameRefClass::InsertTable
        | NameRefClass::DeleteTable
        | NameRefClass::UpdateTable => {
            let path = find_containing_path(name_ref)?;
            let table_name = extract_table_name(&path)?;
            let schema = extract_schema_name(&path);
            let position = name_ref.syntax().text_range().start();
            resolve_table(binder, &table_name, &schema, position).map(|ptr| smallvec![ptr])
        }
        NameRefClass::SelectFromTable => {
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

            if let Some(ptr) = resolve_table(binder, &table_name, &schema, position) {
                return Some(smallvec![ptr]);
            }

            resolve_view(binder, &table_name, &schema, position).map(|ptr| smallvec![ptr])
        }
        NameRefClass::DropIndex => {
            let path = find_containing_path(name_ref)?;
            let index_name = extract_table_name(&path)?;
            let schema = extract_schema_name(&path);
            let position = name_ref.syntax().text_range().start();
            resolve_index(binder, &index_name, &schema, position).map(|ptr| smallvec![ptr])
        }
        NameRefClass::DropType | NameRefClass::TypeReference => {
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
                let type_name = extract_table_name(&path)?;
                let schema = extract_schema_name(&path);
                (type_name, schema)
            };
            let position = name_ref.syntax().text_range().start();
            resolve_type(binder, &type_name, &schema, position).map(|ptr| smallvec![ptr])
        }
        NameRefClass::DropView | NameRefClass::DropMaterializedView => {
            let path = find_containing_path(name_ref)?;
            let view_name = extract_table_name(&path)?;
            let schema = extract_schema_name(&path);
            let position = name_ref.syntax().text_range().start();
            resolve_view(binder, &view_name, &schema, position).map(|ptr| smallvec![ptr])
        }
        NameRefClass::DropSequence => {
            let path = find_containing_path(name_ref)?;
            let sequence_name = extract_table_name(&path)?;
            let schema = extract_schema_name(&path);
            let position = name_ref.syntax().text_range().start();
            resolve_sequence(binder, &sequence_name, &schema, position).map(|ptr| smallvec![ptr])
        }
        NameRefClass::DropDatabase => {
            let database_name = Name::from_node(name_ref);
            resolve_database(binder, &database_name).map(|ptr| smallvec![ptr])
        }
        NameRefClass::SequenceOwnedByColumn => {
            let sequence_option = name_ref
                .syntax()
                .ancestors()
                .find_map(ast::SequenceOption::cast)?;
            let path = sequence_option.path()?;
            let column_name = Name::from_node(name_ref);
            let table_path = path.qualifier()?;
            resolve_column_for_path(binder, &table_path, column_name).map(|ptr| smallvec![ptr])
        }
        NameRefClass::Tablespace => {
            let tablespace_name = Name::from_node(name_ref);
            resolve_tablespace(binder, &tablespace_name).map(|ptr| smallvec![ptr])
        }
        NameRefClass::ForeignKeyTable => {
            let path = name_ref.syntax().ancestors().find_map(ast::Path::cast)?;
            let table_name = extract_table_name(&path)?;
            let schema = extract_schema_name(&path);
            let position = name_ref.syntax().text_range().start();
            resolve_table(binder, &table_name, &schema, position).map(|ptr| smallvec![ptr])
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
            resolve_column_for_path(binder, &path, column_name).map(|ptr| smallvec![ptr])
        }
        NameRefClass::ForeignKeyLocalColumn => {
            let create_table = name_ref
                .syntax()
                .ancestors()
                .find_map(ast::CreateTable::cast)?;
            let column_name = Name::from_node(name_ref);
            find_column_in_create_table(&create_table, &column_name).map(|ptr| smallvec![ptr])
        }
        NameRefClass::CheckConstraintColumn => {
            let create_table = name_ref
                .syntax()
                .ancestors()
                .find_map(ast::CreateTable::cast)?;
            let column_name = Name::from_node(name_ref);
            find_column_in_create_table(&create_table, &column_name).map(|ptr| smallvec![ptr])
        }
        NameRefClass::GeneratedColumn => {
            let create_table = name_ref
                .syntax()
                .ancestors()
                .find_map(ast::CreateTable::cast)?;
            let column_name = Name::from_node(name_ref);
            find_column_in_create_table(&create_table, &column_name).map(|ptr| smallvec![ptr])
        }
        NameRefClass::UniqueConstraintColumn => {
            let create_table = name_ref
                .syntax()
                .ancestors()
                .find_map(ast::CreateTable::cast)?;
            let column_name = Name::from_node(name_ref);
            find_column_in_create_table(&create_table, &column_name).map(|ptr| smallvec![ptr])
        }
        NameRefClass::PrimaryKeyConstraintColumn => {
            let create_table = name_ref
                .syntax()
                .ancestors()
                .find_map(ast::CreateTable::cast)?;
            let column_name = Name::from_node(name_ref);
            find_column_in_create_table(&create_table, &column_name).map(|ptr| smallvec![ptr])
        }
        NameRefClass::NotNullConstraintColumn => {
            let create_table = name_ref
                .syntax()
                .ancestors()
                .find_map(ast::CreateTable::cast)?;
            let column_name = Name::from_node(name_ref);
            find_column_in_create_table(&create_table, &column_name).map(|ptr| smallvec![ptr])
        }
        NameRefClass::ExcludeConstraintColumn => {
            let create_table = name_ref
                .syntax()
                .ancestors()
                .find_map(ast::CreateTable::cast)?;
            let column_name = Name::from_node(name_ref);
            find_column_in_create_table(&create_table, &column_name).map(|ptr| smallvec![ptr])
        }
        NameRefClass::PartitionByColumn => {
            let create_table = name_ref
                .syntax()
                .ancestors()
                .find_map(ast::CreateTable::cast)?;
            let column_name = Name::from_node(name_ref);
            find_column_in_create_table(&create_table, &column_name).map(|ptr| smallvec![ptr])
        }
        NameRefClass::PartitionOfTable => {
            let path = find_containing_path(name_ref)?;
            let table_name = extract_table_name(&path)?;
            let schema = extract_schema_name(&path);
            let position = name_ref.syntax().text_range().start();
            resolve_table(binder, &table_name, &schema, position).map(|ptr| smallvec![ptr])
        }
        NameRefClass::LikeTable => {
            let like_clause = name_ref
                .syntax()
                .ancestors()
                .find_map(ast::LikeClause::cast)?;
            let path = like_clause.path()?;
            let table_name = extract_table_name(&path)?;
            let schema = extract_schema_name(&path);
            let position = name_ref.syntax().text_range().start();
            resolve_table(binder, &table_name, &schema, position).map(|ptr| smallvec![ptr])
        }
        NameRefClass::InheritsTable => {
            let path = find_containing_path(name_ref)?;
            let table_name = extract_table_name(&path)?;
            let schema = extract_schema_name(&path);
            let position = name_ref.syntax().text_range().start();
            resolve_table(binder, &table_name, &schema, position).map(|ptr| smallvec![ptr])
        }
        NameRefClass::DropFunction => {
            let function_sig = name_ref
                .syntax()
                .ancestors()
                .find_map(ast::FunctionSig::cast)?;
            let path = function_sig.path()?;
            let function_name = extract_table_name(&path)?;
            let schema = extract_schema_name(&path);
            let params = extract_param_signature(&function_sig);
            let position = name_ref.syntax().text_range().start();
            resolve_function(binder, &function_name, &schema, params.as_deref(), position)
                .map(|ptr| smallvec![ptr])
        }
        NameRefClass::DropAggregate => {
            let aggregate = name_ref
                .syntax()
                .ancestors()
                .find_map(ast::Aggregate::cast)?;
            let path = aggregate.path()?;
            let aggregate_name = extract_table_name(&path)?;
            let schema = extract_schema_name(&path);
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
        NameRefClass::DropProcedure => {
            let function_sig = name_ref
                .syntax()
                .ancestors()
                .find_map(ast::FunctionSig::cast)?;
            let path = function_sig.path()?;
            let procedure_name = extract_table_name(&path)?;
            let schema = extract_schema_name(&path);
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
        NameRefClass::DropRoutine => {
            let function_sig = name_ref
                .syntax()
                .ancestors()
                .find_map(ast::FunctionSig::cast)?;
            let path = function_sig.path()?;
            let routine_name = extract_table_name(&path)?;
            let schema = extract_schema_name(&path);
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
            let procedure_name = extract_table_name(&path)?;
            let schema = extract_schema_name(&path);
            let position = name_ref.syntax().text_range().start();
            resolve_procedure(binder, &procedure_name, &schema, None, position)
                .map(|ptr| smallvec![ptr])
        }
        NameRefClass::DropSchema | NameRefClass::SchemaQualifier | NameRefClass::CreateSchema => {
            let schema_name = Name::from_node(name_ref);
            resolve_schema(binder, &schema_name).map(|ptr| smallvec![ptr])
        }
        NameRefClass::DefaultConstraintFunctionCall => {
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
                && let Some(ptr) = resolve_fn_call_column(binder, name_ref)
            {
                return Some(smallvec![ptr]);
            }

            None
        }
        NameRefClass::CreateIndexColumn => {
            resolve_create_index_column(binder, name_ref).map(|ptr| smallvec![ptr])
        }
        NameRefClass::SelectColumn => {
            resolve_select_column(binder, name_ref).map(|ptr| smallvec![ptr])
        }
        NameRefClass::SelectQualifiedColumnTable => {
            resolve_select_qualified_column_table(binder, name_ref).map(|ptr| smallvec![ptr])
        }
        NameRefClass::SelectQualifiedColumn => {
            resolve_select_qualified_column(binder, name_ref).map(|ptr| smallvec![ptr])
        }
        NameRefClass::CompositeTypeField => {
            resolve_composite_type_field(binder, name_ref).map(|ptr| smallvec![ptr])
        }
        NameRefClass::InsertColumn => {
            resolve_insert_column(binder, name_ref).map(|ptr| smallvec![ptr])
        }
        NameRefClass::DeleteWhereColumn => {
            resolve_delete_where_column(binder, name_ref).map(|ptr| smallvec![ptr])
        }
        NameRefClass::UpdateWhereColumn | NameRefClass::UpdateSetColumn => {
            resolve_update_where_column(binder, name_ref).map(|ptr| smallvec![ptr])
        }
        NameRefClass::JoinUsingColumn => resolve_join_using_columns(binder, name_ref),
        NameRefClass::UpdateFromTable => {
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

            if let Some(ptr) = resolve_table(binder, &table_name, &schema, position) {
                return Some(smallvec![ptr]);
            }

            resolve_view(binder, &table_name, &schema, position).map(|ptr| smallvec![ptr])
        }
    }
}

fn resolve_table(
    binder: &Binder,
    table_name: &Name,
    schema: &Option<Schema>,
    position: TextSize,
) -> Option<SyntaxNodePtr> {
    resolve_for_kind(binder, table_name, schema, position, SymbolKind::Table)
}

fn resolve_index(
    binder: &Binder,
    index_name: &Name,
    schema: &Option<Schema>,
    position: TextSize,
) -> Option<SyntaxNodePtr> {
    resolve_for_kind(binder, index_name, schema, position, SymbolKind::Index)
}

fn resolve_type(
    binder: &Binder,
    type_name: &Name,
    schema: &Option<Schema>,
    position: TextSize,
) -> Option<SyntaxNodePtr> {
    resolve_for_kind(binder, type_name, schema, position, SymbolKind::Type)
}

fn resolve_view(
    binder: &Binder,
    view_name: &Name,
    schema: &Option<Schema>,
    position: TextSize,
) -> Option<SyntaxNodePtr> {
    resolve_for_kind(binder, view_name, schema, position, SymbolKind::View)
}

fn resolve_sequence(
    binder: &Binder,
    sequence_name: &Name,
    schema: &Option<Schema>,
    position: TextSize,
) -> Option<SyntaxNodePtr> {
    resolve_for_kind(
        binder,
        sequence_name,
        schema,
        position,
        SymbolKind::Sequence,
    )
}

fn resolve_tablespace(binder: &Binder, tablespace_name: &Name) -> Option<SyntaxNodePtr> {
    let symbols = binder.scopes[binder.root_scope()].get(tablespace_name)?;
    let symbol_id = symbols.iter().copied().find(|id| {
        let symbol = &binder.symbols[*id];
        symbol.kind == SymbolKind::Tablespace
    })?;
    Some(binder.symbols[symbol_id].ptr)
}

fn resolve_database(binder: &Binder, database_name: &Name) -> Option<SyntaxNodePtr> {
    let symbols = binder.scopes[binder.root_scope()].get(database_name)?;
    let symbol_id = symbols.iter().copied().find(|id| {
        let symbol = &binder.symbols[*id];
        symbol.kind == SymbolKind::Database
    })?;
    Some(binder.symbols[symbol_id].ptr)
}

fn resolve_for_kind(
    binder: &Binder,
    name: &Name,
    schema: &Option<Schema>,
    position: TextSize,
    kind: SymbolKind,
) -> Option<SyntaxNodePtr> {
    let symbols = binder.scopes[binder.root_scope()].get(name)?;

    if let Some(schema) = schema {
        let symbol_id = symbols.iter().copied().find(|id| {
            let symbol = &binder.symbols[*id];
            symbol.kind == kind && &symbol.schema == schema
        })?;
        return Some(binder.symbols[symbol_id].ptr);
    } else {
        let search_path = binder.search_path_at(position);
        for search_schema in search_path {
            if let Some(symbol_id) = symbols.iter().copied().find(|id| {
                let symbol = &binder.symbols[*id];
                symbol.kind == kind && &symbol.schema == search_schema
            }) {
                return Some(binder.symbols[symbol_id].ptr);
            }
        }
    }
    None
}

fn resolve_for_kind_with_params(
    binder: &Binder,
    name: &Name,
    schema: &Option<Schema>,
    params: Option<&[Name]>,
    position: TextSize,
    kind: SymbolKind,
) -> Option<SyntaxNodePtr> {
    let symbols = binder.scopes[binder.root_scope()].get(name)?;

    if let Some(schema) = schema {
        let symbol_id = symbols.iter().copied().find(|id| {
            let symbol = &binder.symbols[*id];
            let params_match = match (&symbol.params, params) {
                (Some(sym_params), Some(req_params)) => sym_params.as_slice() == req_params,
                (None, None) => true,
                (_, None) => true,
                _ => false,
            };
            symbol.kind == kind && &symbol.schema == schema && params_match
        })?;
        return Some(binder.symbols[symbol_id].ptr);
    } else {
        let search_path = binder.search_path_at(position);
        for search_schema in search_path {
            if let Some(symbol_id) = symbols.iter().copied().find(|id| {
                let symbol = &binder.symbols[*id];
                let params_match = match (&symbol.params, params) {
                    (Some(sym_params), Some(req_params)) => sym_params.as_slice() == req_params,
                    (None, None) => true,
                    (_, None) => true,
                    _ => false,
                };
                symbol.kind == kind && &symbol.schema == search_schema && params_match
            }) {
                return Some(binder.symbols[symbol_id].ptr);
            }
        }
    }
    None
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
    let symbols = binder.scopes[binder.root_scope()].get(schema_name)?;
    let symbol_id = symbols.iter().copied().find(|id| {
        let symbol = &binder.symbols[*id];
        symbol.kind == SymbolKind::Schema
    })?;
    Some(binder.symbols[symbol_id].ptr)
}

fn resolve_create_index_column(binder: &Binder, name_ref: &ast::NameRef) -> Option<SyntaxNodePtr> {
    let column_name = Name::from_node(name_ref);

    let create_index = name_ref
        .syntax()
        .ancestors()
        .find_map(ast::CreateIndex::cast)?;
    let relation_name = create_index.relation_name()?;
    let path = relation_name.path()?;

    resolve_column_for_path(binder, &path, column_name)
}

fn resolve_column_for_path(
    binder: &Binder,
    path: &ast::Path,
    column_name: Name,
) -> Option<SyntaxNodePtr> {
    let table_name = extract_table_name(path)?;
    let schema = extract_schema_name(path);
    let position = path.syntax().text_range().start();

    let table_ptr = resolve_table(binder, &table_name, &schema, position)?;

    let root = &path.syntax().ancestors().last()?;
    let table_name_node = table_ptr.to_node(root);

    let create_table = table_name_node
        .ancestors()
        .find_map(ast::CreateTable::cast)?;

    find_column_in_create_table(&create_table, &column_name)
}

fn resolve_insert_column(binder: &Binder, name_ref: &ast::NameRef) -> Option<SyntaxNodePtr> {
    let column_name = Name::from_node(name_ref);

    let insert = name_ref.syntax().ancestors().find_map(ast::Insert::cast)?;
    let path = insert.path()?;

    resolve_column_for_path(binder, &path, column_name)
}

fn resolve_select_qualified_column_table(
    binder: &Binder,
    name_ref: &ast::NameRef,
) -> Option<SyntaxNodePtr> {
    let table_name = Name::from_node(name_ref);

    let field_expr = name_ref.syntax().parent().and_then(ast::FieldExpr::cast)?;

    let explicit_schema = if field_expr
        .field()
        .is_some_and(|f| f.syntax() == name_ref.syntax())
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
        let position = name_ref.syntax().text_range().start();
        return resolve_table(binder, &table_name, &Some(schema), position);
    }

    let select = name_ref.syntax().ancestors().find_map(ast::Select::cast)?;
    let from_clause = select.from_clause()?;
    let from_item = find_from_item_in_from_clause(&from_clause, &table_name)?;

    if let Some(alias_name) = from_item.alias().and_then(|a| a.name())
        && Name::from_node(&alias_name) == table_name
    {
        return Some(SyntaxNodePtr::new(alias_name.syntax()));
    }

    let (table_name, schema) = if let Some(name_ref_node) = from_item.name_ref() {
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

    let position = name_ref.syntax().text_range().start();
    resolve_table(binder, &table_name, &schema, position)
}

// TODO: this is similar to resolve_from_item_for_column, maybe we can simplify
fn resolve_select_qualified_column(
    binder: &Binder,
    name_ref: &ast::NameRef,
) -> Option<SyntaxNodePtr> {
    let column_name = Name::from_node(name_ref);

    let field_expr = name_ref.syntax().parent().and_then(ast::FieldExpr::cast)?;

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

    let position = name_ref.syntax().text_range().start();

    let (table_name, schema) = if let Some(schema) = explicit_schema {
        (column_table_name, Some(schema))
    } else {
        let select = name_ref.syntax().ancestors().find_map(ast::Select::cast)?;
        let from_clause = select.from_clause()?;
        let from_item = find_from_item_in_from_clause(&from_clause, &column_table_name)?;

        // `from t as u`
        // `from t as u(a, b, c)`
        if let Some(alias) = from_item.alias()
            && let Some(alias_name) = alias.name()
            && Name::from_node(&alias_name) == column_table_name
        {
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
                    return resolve_cte_column(name_ref, &cte_name, &column_name);
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

    let root = &name_ref.syntax().ancestors().last()?;

    if let Some(table_ptr) = resolve_table(binder, &table_name, &schema, position) {
        let table_name_node = table_ptr.to_node(root);

        if let Some(create_table) = table_name_node.ancestors().find_map(ast::CreateTable::cast) {
            // 1. Try to find a matching column (columns take precedence)
            if let Some(ptr) = find_column_in_create_table(&create_table, &column_name) {
                return Some(ptr);
            }
            // 2. No column found, check for field-style function call
            // e.g., select t.b from t where b is a function that takes t as an argument
            return resolve_function(binder, &column_name, &schema, None, position);
        }
    }

    // ditto as above but with views
    if let Some(view_ptr) = resolve_view(binder, &table_name, &schema, position) {
        let view_name_node = view_ptr.to_node(root);

        if let Some(create_view) = view_name_node.ancestors().find_map(ast::CreateView::cast) {
            if let Some(ptr) = find_column_in_create_view(&create_view, &column_name) {
                return Some(ptr);
            }

            return resolve_function(binder, &column_name, &schema, None, position);
        }
    }

    None
}

fn resolve_from_item_for_column(
    binder: &Binder,
    from_item: &ast::FromItem,
    name_ref: &ast::NameRef,
) -> Option<SyntaxNodePtr> {
    let column_name = Name::from_node(name_ref);
    if let Some(paren_select) = from_item.paren_select() {
        return resolve_subquery_column(binder, &paren_select, name_ref, &column_name);
    }

    if let Some(paren_expr) = from_item.paren_expr() {
        return resolve_column_from_paren_expr(binder, &paren_expr, name_ref, &column_name);
    }

    let (table_name, schema) = if let Some(name_ref_node) = from_item.name_ref() {
        (Name::from_node(&name_ref_node), None)
    } else {
        let field_expr = from_item.field_expr()?;
        let table_name = Name::from_node(&field_expr.field()?);
        let ast::Expr::NameRef(schema_name_ref) = field_expr.base()? else {
            return None;
        };
        let schema = Schema(Name::from_node(&schema_name_ref));
        (table_name, Some(schema))
    };

    if schema.is_none()
        && let Some(cte_column_ptr) = resolve_cte_column(name_ref, &table_name, &column_name)
    {
        return Some(cte_column_ptr);
    }

    let position = name_ref.syntax().text_range().start();
    let root = &name_ref.syntax().ancestors().last()?;

    if let Some(table_ptr) = resolve_table(binder, &table_name, &schema, position) {
        let table_name_node = table_ptr.to_node(root);

        if let Some(create_table) = table_name_node.ancestors().find_map(ast::CreateTable::cast) {
            // 1. try to find a matching column
            if let Some(ptr) = find_column_in_create_table(&create_table, &column_name) {
                return Some(ptr);
            }

            // 2. No column found, check if the name matches the table name.
            // For example, in:
            // ```sql
            // create table t(a int);
            // select t from t;
            // ```
            if column_name == table_name {
                return Some(table_ptr);
            }
        }
    }

    // ditto as above but with view
    if let Some(view_ptr) = resolve_view(binder, &table_name, &schema, position) {
        let view_name_node = view_ptr.to_node(root);

        if let Some(create_view) = view_name_node.ancestors().find_map(ast::CreateView::cast) {
            if let Some(ptr) = find_column_in_create_view(&create_view, &column_name) {
                return Some(ptr);
            }

            if column_name == table_name {
                return Some(view_ptr);
            }
        }
    }

    None
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

fn resolve_select_column(binder: &Binder, name_ref: &ast::NameRef) -> Option<SyntaxNodePtr> {
    let select = name_ref.syntax().ancestors().find_map(ast::Select::cast)?;
    let from_clause = select.from_clause()?;

    for from_item in from_clause.from_items() {
        if let Some(result) = resolve_from_item_for_column(binder, &from_item, name_ref) {
            return Some(result);
        }
    }

    for join_expr in from_clause.join_exprs() {
        if let Some(result) = resolve_from_join_expr(&join_expr, &|from_item: &ast::FromItem| {
            resolve_from_item_for_column(binder, from_item, name_ref)
        }) {
            return Some(result);
        }
    }

    None
}

fn resolve_delete_where_column(binder: &Binder, name_ref: &ast::NameRef) -> Option<SyntaxNodePtr> {
    let column_name = Name::from_node(name_ref);

    let delete = name_ref.syntax().ancestors().find_map(ast::Delete::cast)?;
    let relation_name = delete.relation_name()?;
    let path = relation_name.path()?;

    resolve_column_for_path(binder, &path, column_name)
}

fn resolve_join_using_columns(
    binder: &Binder,
    name_ref: &ast::NameRef,
) -> Option<SmallVec<[SyntaxNodePtr; 1]>> {
    let join_expr = name_ref
        .syntax()
        .ancestors()
        .find_map(ast::JoinExpr::cast)?;

    let mut results: SmallVec<[SyntaxNodePtr; 1]> = SmallVec::new();

    collect_from_join_expr(&join_expr, &mut results, &|from_item: &ast::FromItem| {
        resolve_from_item_for_column(binder, from_item, name_ref)
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

fn resolve_update_where_column(binder: &Binder, name_ref: &ast::NameRef) -> Option<SyntaxNodePtr> {
    let column_name = Name::from_node(name_ref);

    let update = name_ref.syntax().ancestors().find_map(ast::Update::cast)?;

    // `update t set a = b from u`
    if let Some(from_clause) = update.from_clause() {
        for from_item in from_clause.from_items() {
            if let Some(result) = resolve_from_item_for_column(binder, &from_item, name_ref) {
                return Some(result);
            }
        }

        for join_expr in from_clause.join_exprs() {
            if let Some(result) =
                resolve_from_join_expr(&join_expr, &|from_item: &ast::FromItem| {
                    resolve_from_item_for_column(binder, from_item, name_ref)
                })
            {
                return Some(result);
            }
        }
    }

    // `update t set a = b`
    let relation_name = update.relation_name()?;
    let path = relation_name.path()?;

    resolve_column_for_path(binder, &path, column_name)
}

fn resolve_fn_call_column(binder: &Binder, name_ref: &ast::NameRef) -> Option<SyntaxNodePtr> {
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
            resolve_from_item_for_fn_call_column(binder, &from_item, &column_name, name_ref)
        {
            return Some(result);
        }
    }

    for join_expr in from_clause.join_exprs() {
        if let Some(result) = resolve_from_join_expr(&join_expr, &|from_item: &ast::FromItem| {
            resolve_from_item_for_fn_call_column(binder, from_item, &column_name, name_ref)
        }) {
            return Some(result);
        }
    }

    None
}

fn resolve_from_item_for_fn_call_column(
    binder: &Binder,
    from_item: &ast::FromItem,
    column_name: &Name,
    name_ref: &ast::NameRef,
) -> Option<SyntaxNodePtr> {
    let (table_name, schema) = if let Some(name_ref_node) = from_item.name_ref() {
        (Name::from_node(&name_ref_node), None)
    } else {
        let field_expr = from_item.field_expr()?;
        let table_name = Name::from_node(&field_expr.field()?);
        let ast::Expr::NameRef(schema_name_ref) = field_expr.base()? else {
            return None;
        };
        let schema = Schema(Name::from_node(&schema_name_ref));
        (table_name, Some(schema))
    };

    let position = name_ref.syntax().text_range().start();
    let table_ptr = resolve_table(binder, &table_name, &schema, position)?;

    let root = &name_ref.syntax().ancestors().last()?;
    let table_name_node = table_ptr.to_node(root);
    let create_table = table_name_node
        .ancestors()
        .find_map(ast::CreateTable::cast)?;

    find_column_in_create_table(&create_table, column_name)
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

fn find_from_item_in_from_clause(
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

fn extract_table_name(path: &ast::Path) -> Option<Name> {
    let segment = path.segment()?;
    let name_ref = segment.name_ref()?;
    Some(Name::from_node(&name_ref))
}

fn extract_schema_name(path: &ast::Path) -> Option<Schema> {
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
    create_table: &ast::CreateTable,
    column_name: &Name,
) -> Option<SyntaxNodePtr> {
    create_table.table_arg_list()?.args().find_map(|arg| {
        if let ast::TableArg::Column(column) = arg
            && let Some(name) = column.name()
            && Name::from_node(&name) == *column_name
        {
            return Some(SyntaxNodePtr::new(name.syntax()));
        } else {
            None
        }
    })
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

    let query = create_view.query()?;
    let select = match query {
        ast::SelectVariant::Select(s) => s,
        ast::SelectVariant::ParenSelect(ps) => match ps.select()? {
            ast::SelectVariant::Select(s) => s,
            _ => return None,
        },
        _ => return None,
    };

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

fn resolve_cte_table(name_ref: &ast::NameRef, cte_name: &Name) -> Option<SyntaxNodePtr> {
    let with_clause = find_parent_with_clause(name_ref.syntax())?;
    for with_table in with_clause.with_tables() {
        if let Some(name) = with_table.name()
            && Name::from_node(&name) == *cte_name
        {
            return Some(SyntaxNodePtr::new(name.syntax()));
        }
    }

    None
}

fn find_parent_with_clause(node: &SyntaxNode) -> Option<ast::WithClause> {
    node.ancestors().find_map(|x| {
        if let Some(select) = ast::Select::cast(x.clone()) {
            select.with_clause()
        } else if let Some(delete) = ast::Delete::cast(x.clone()) {
            delete.with_clause()
        } else if let Some(insert) = ast::Insert::cast(x.clone()) {
            insert.with_clause()
        } else if let Some(update) = ast::Update::cast(x) {
            update.with_clause()
        } else {
            None
        }
    })
}

fn resolve_cte_column(
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

            let select_variant = match query {
                ast::WithQuery::Select(s) => ast::SelectVariant::Select(s),
                ast::WithQuery::ParenSelect(ps) => ps.select()?,
                ast::WithQuery::CompoundSelect(compound) => compound.lhs()?,
                _ => continue,
            };

            let cte_select = match select_variant {
                ast::SelectVariant::Select(s) => s,
                ast::SelectVariant::CompoundSelect(compound) => match compound.lhs()? {
                    ast::SelectVariant::Select(s) => s,
                    _ => continue,
                },
                _ => continue,
            };

            let select_clause = cte_select.select_clause()?;
            let target_list = select_clause.target_list()?;

            for (idx, target) in target_list.targets().enumerate() {
                // Skip targets that are covered by the column list
                if idx < column_list_len {
                    continue;
                }

                if let Some((col_name, node)) = ColumnName::from_target(target.clone()) {
                    if let Some(col_name_str) = col_name.to_string()
                        && Name::from_string(col_name_str) == *column_name
                    {
                        return Some(SyntaxNodePtr::new(&node));
                    }

                    if matches!(col_name, ColumnName::Star) {
                        if let Some(from_clause) = cte_select.from_clause()
                            && let Some(from_item) = from_clause.from_items().next()
                            && let Some(from_name_ref) = from_item.name_ref()
                        {
                            let from_table_name = Name::from_node(&from_name_ref);
                            // Skip recursive CTE lookup if the FROM table has the same name as the current CTE
                            // (CTEs don't shadow themselves in their own definition)
                            if from_table_name != *cte_name {
                                return resolve_cte_column(name_ref, &from_table_name, column_name);
                            }
                        }
                    }
                }
            }
        }
    }

    None
}

fn resolve_subquery_column(
    binder: &Binder,
    paren_select: &ast::ParenSelect,
    name_ref: &ast::NameRef,
    column_name: &Name,
) -> Option<SyntaxNodePtr> {
    let select_variant = paren_select.select()?;
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
                            resolve_from_item_for_column(binder, &from_item, name_ref)
                        {
                            return Some(result);
                        }
                    }

                    for join_expr in from_clause.join_exprs() {
                        if let Some(result) =
                            resolve_from_join_expr(&join_expr, &|from_item: &ast::FromItem| {
                                resolve_from_item_for_column(binder, from_item, name_ref)
                            })
                        {
                            return Some(result);
                        }
                    }
                }
            }
        }
    }

    None
}

fn resolve_column_from_paren_expr(
    binder: &Binder,
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
        return resolve_column_from_paren_expr(binder, &paren_expr, name_ref, column_name);
    }

    if let Some(from_item) = paren_expr.from_item()
        && let Some(paren_select) = from_item.paren_select()
    {
        return resolve_subquery_column(binder, &paren_select, name_ref, column_name);
    }

    None
}

pub(crate) fn resolve_insert_create_table(
    file: &ast::SourceFile,
    binder: &Binder,
    insert: &ast::Insert,
) -> Option<ast::CreateTable> {
    let path = insert.path()?;
    let table_name = extract_table_name(&path)?;
    let schema = extract_schema_name(&path);
    let position = insert.syntax().text_range().start();

    let table_ptr = resolve_table(binder, &table_name, &schema, position)?;
    let root = file.syntax();
    let table_name_node = table_ptr.to_node(root);

    table_name_node.ancestors().find_map(ast::CreateTable::cast)
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

pub(crate) fn resolve_materialized_view_info(
    binder: &Binder,
    path: &ast::Path,
) -> Option<(Schema, String)> {
    resolve_symbol_info(binder, path, SymbolKind::View)
}

fn resolve_symbol_info(
    binder: &Binder,
    path: &ast::Path,
    kind: SymbolKind,
) -> Option<(Schema, String)> {
    let name_str = extract_table_name_from_path(path)?;
    let schema = extract_schema_from_path(path);

    let name_normalized = Name::from_string(name_str.clone());
    let symbols = binder.scopes[binder.root_scope()].get(&name_normalized)?;

    if let Some(schema_name) = schema {
        let schema_normalized = Schema::new(schema_name);
        let symbol_id = symbols.iter().copied().find(|id| {
            let symbol = &binder.symbols[*id];
            symbol.kind == kind && symbol.schema == schema_normalized
        })?;
        let symbol = &binder.symbols[symbol_id];
        return Some((symbol.schema.clone(), name_str));
    } else {
        let position = path.syntax().text_range().start();
        let search_path = binder.search_path_at(position);
        for search_schema in search_path {
            if let Some(symbol_id) = symbols.iter().copied().find(|id| {
                let symbol = &binder.symbols[*id];
                symbol.kind == kind && &symbol.schema == search_schema
            }) {
                let symbol = &binder.symbols[symbol_id];
                return Some((symbol.schema.clone(), name_str));
            }
        }
    }
    None
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
    path.qualifier()
        .and_then(|q| q.segment())
        .and_then(|s| s.name_ref())
        .map(|name_ref| name_ref.syntax().text().to_string())
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

fn resolve_composite_type_field(binder: &Binder, name_ref: &ast::NameRef) -> Option<SyntaxNodePtr> {
    let field_name = Name::from_node(name_ref);
    let field_expr = name_ref.syntax().parent().and_then(ast::FieldExpr::cast)?;
    let base = field_expr.base()?;

    let base_name_ref = unwrap_paren_expr(base)?;
    let root = &name_ref.syntax().ancestors().last()?;

    let (type_name, schema) =
        if let Some(type_info) = resolve_composite_type_from_column(binder, &base_name_ref, root) {
            type_info
        } else {
            resolve_composite_type_from_cast(binder, &base_name_ref, root)?
        };

    let position = name_ref.syntax().text_range().start();
    let type_ptr = resolve_type(binder, &type_name, &schema, position)?;
    let type_node = type_ptr.to_node(root);

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

fn resolve_composite_type_from_column(
    binder: &Binder,
    base_name_ref: &ast::NameRef,
    root: &SyntaxNode,
) -> Option<(Name, Option<Schema>)> {
    let column_ptr = resolve_select_column(binder, base_name_ref)?;
    let column_node = column_ptr.to_node(root);
    let column = column_node.ancestors().find_map(ast::Column::cast)?;
    let ty = column.ty()?;
    extract_type_name_and_schema(&ty)
}

fn resolve_composite_type_from_cast(
    binder: &Binder,
    base_name_ref: &ast::NameRef,
    root: &SyntaxNode,
) -> Option<(Name, Option<Schema>)> {
    let column_ptr = resolve_select_column(binder, base_name_ref)?;
    let column_node = column_ptr.to_node(root);
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
            let type_name = extract_table_name(&path)?;
            let schema = extract_schema_name(&path);
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
