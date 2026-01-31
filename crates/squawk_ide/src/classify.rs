use crate::symbols::Name;
use squawk_syntax::{
    SyntaxKind,
    ast::{self, AstNode},
};

#[derive(Debug)]
pub(crate) enum NameRefClass {
    Aggregate,
    AlterColumn,
    CallProcedure,
    Channel,
    CompositeTypeField,
    ConstraintColumn,
    CreateIndexColumn,
    Cursor,
    Database,
    DeleteColumn,
    DeleteQualifiedColumnTable,
    EventTrigger,
    Extension,
    ForeignKeyColumn,
    ForeignKeyTable,
    FromTable,
    Function,
    FunctionCall,
    FunctionName,
    Index,
    InsertColumn,
    InsertQualifiedColumnTable,
    JoinUsingColumn,
    LikeTable,
    MergeColumn,
    MergeQualifiedColumnTable,
    NamedArgParameter,
    Policy,
    PolicyColumn,
    PolicyQualifiedColumnTable,
    PreparedStatement,
    Procedure,
    ProcedureCall,
    QualifiedColumn,
    Role,
    Routine,
    Schema,
    SelectColumn,
    SelectFunctionCall,
    SelectQualifiedColumn,
    SelectQualifiedColumnTable,
    Sequence,
    Server,
    Table,
    Tablespace,
    Trigger,
    Type,
    UpdateColumn,
    UpdateQualifiedColumnTable,
    View,
}

fn is_special_fn(kind: SyntaxKind) -> bool {
    matches!(
        kind,
        SyntaxKind::EXTRACT_FN
            | SyntaxKind::JSON_EXISTS_FN
            | SyntaxKind::JSON_ARRAY_FN
            | SyntaxKind::JSON_OBJECT_FN
            | SyntaxKind::JSON_OBJECT_AGG_FN
            | SyntaxKind::JSON_ARRAY_AGG_FN
            | SyntaxKind::JSON_QUERY_FN
            | SyntaxKind::JSON_SCALAR_FN
            | SyntaxKind::JSON_SERIALIZE_FN
            | SyntaxKind::JSON_VALUE_FN
            | SyntaxKind::JSON_FN
            | SyntaxKind::SUBSTRING_FN
            | SyntaxKind::POSITION_FN
            | SyntaxKind::OVERLAY_FN
            | SyntaxKind::TRIM_FN
            | SyntaxKind::XML_ROOT_FN
            | SyntaxKind::XML_SERIALIZE_FN
            | SyntaxKind::XML_ELEMENT_FN
            | SyntaxKind::XML_FOREST_FN
            | SyntaxKind::XML_EXISTS_FN
            | SyntaxKind::XML_PARSE_FN
            | SyntaxKind::XML_PI_FN
            | SyntaxKind::SOME_FN
            | SyntaxKind::ANY_FN
            | SyntaxKind::ALL_FN
            | SyntaxKind::EXISTS_FN
    )
}

pub(crate) fn classify_name_ref(name_ref: &ast::NameRef) -> Option<NameRefClass> {
    let mut in_function_name = false;
    let mut in_arg_list = false;
    let mut in_column_list = false;
    let mut in_where_clause = false;
    let mut in_from_clause = false;
    let mut in_on_clause = false;
    let mut in_set_clause = false;
    let mut in_constraint_exclusion_list = false;
    let mut in_constraint_include_clause = false;
    let mut in_constraint_where_clause = false;
    let mut in_partition_item = false;
    let mut in_set_null_columns = false;
    let mut in_using_clause = false;
    let mut in_returning_clause = false;
    let mut in_when_clause = false;
    let mut in_special_sql_fn = false;

    // TODO: can we combine this if and the one that follows?
    if let Some(parent) = name_ref.syntax().parent()
        && let Some(field_expr) = ast::FieldExpr::cast(parent.clone())
        && let Some(base) = field_expr.base()
        && let ast::Expr::NameRef(base_name_ref) = base
        // check that the name_ref we're looking at in the field expr is the
        // base name_ref, i.e., the schema, rather than the item
        && base_name_ref.syntax() == name_ref.syntax()
    {
        let is_function_call = field_expr
            .syntax()
            .parent()
            .and_then(ast::CallExpr::cast)
            .is_some();
        let is_schema_table_col = field_expr
            .syntax()
            .parent()
            .and_then(ast::FieldExpr::cast)
            .is_some();

        let mut in_from_clause = false;
        let mut in_on_clause = false;
        let mut in_returning_clause = false;
        let mut in_set_clause = false;
        let mut in_where_clause = false;
        let mut in_when_clause = false;
        for ancestor in parent.ancestors() {
            if ast::OnClause::can_cast(ancestor.kind()) {
                in_on_clause = true;
            }
            if ast::FromClause::can_cast(ancestor.kind()) {
                in_from_clause = true;
            }
            if ast::ReturningClause::can_cast(ancestor.kind()) {
                in_returning_clause = true;
            }
            if ast::SetClause::can_cast(ancestor.kind()) {
                in_set_clause = true;
            }
            if ast::WhereClause::can_cast(ancestor.kind()) {
                in_where_clause = true;
            }
            if ast::MergeWhenClause::can_cast(ancestor.kind()) {
                in_when_clause = true;
            }
            if ast::Update::can_cast(ancestor.kind()) {
                if in_returning_clause || in_set_clause || in_where_clause || in_from_clause {
                    if is_function_call || is_schema_table_col {
                        return Some(NameRefClass::Schema);
                    } else {
                        return Some(NameRefClass::UpdateQualifiedColumnTable);
                    }
                }
            }
            if ast::Insert::can_cast(ancestor.kind()) {
                if in_returning_clause || (!in_from_clause && !in_on_clause) {
                    if is_function_call || is_schema_table_col {
                        return Some(NameRefClass::Schema);
                    } else {
                        return Some(NameRefClass::InsertQualifiedColumnTable);
                    }
                }
            }
            if ast::Delete::can_cast(ancestor.kind()) {
                if in_returning_clause || in_where_clause || in_using_clause {
                    if is_function_call || is_schema_table_col {
                        return Some(NameRefClass::Schema);
                    } else {
                        return Some(NameRefClass::DeleteQualifiedColumnTable);
                    }
                }
            }
            if ast::Merge::can_cast(ancestor.kind()) {
                if in_returning_clause || in_on_clause || in_when_clause {
                    if is_function_call || is_schema_table_col {
                        return Some(NameRefClass::Schema);
                    } else {
                        return Some(NameRefClass::MergeQualifiedColumnTable);
                    }
                }
            }
            if ast::Select::can_cast(ancestor.kind()) && (!in_from_clause || in_on_clause) {
                if is_function_call || is_schema_table_col {
                    return Some(NameRefClass::Schema);
                } else {
                    return Some(NameRefClass::SelectQualifiedColumnTable);
                }
            }
            if ast::CreatePolicy::can_cast(ancestor.kind())
                || ast::AlterPolicy::can_cast(ancestor.kind())
            {
                if is_function_call || is_schema_table_col {
                    return Some(NameRefClass::Schema);
                } else {
                    return Some(NameRefClass::PolicyQualifiedColumnTable);
                }
            }
        }
        return Some(NameRefClass::Schema);
    }

    if let Some(parent) = name_ref.syntax().parent()
        && let Some(field_expr) = ast::FieldExpr::cast(parent.clone())
        && field_expr
            .field()
            // we're at the field in a FieldExpr, i.e., foo.bar
            //                                              ^^^
            .is_some_and(|field_name_ref| field_name_ref.syntax() == name_ref.syntax())
            // we're not inside a call expr
        && field_expr.star_token().is_none()
        && field_expr
            .syntax()
            .parent()
            .and_then(ast::CallExpr::cast)
            .is_none()
    {
        let is_base_of_outer_field_expr = field_expr
            .syntax()
            .parent()
            .and_then(ast::FieldExpr::cast)
            .is_some();

        let mut in_from_clause = false;
        let mut in_on_clause = false;
        let mut in_cast_expr = false;
        let mut in_when_clause = false;
        let mut in_returning_clause = false;
        for ancestor in parent.ancestors() {
            if ast::OnClause::can_cast(ancestor.kind()) {
                in_on_clause = true;
            }
            if ast::CastExpr::can_cast(ancestor.kind()) {
                in_cast_expr = true;
            }
            if ast::FromClause::can_cast(ancestor.kind()) {
                in_from_clause = true;
            }
            if ast::MergeWhenClause::can_cast(ancestor.kind()) {
                in_when_clause = true;
            }
            if ast::ReturningClause::can_cast(ancestor.kind()) {
                in_returning_clause = true;
            }
            if ast::Merge::can_cast(ancestor.kind())
                && (in_on_clause || in_when_clause || in_returning_clause)
            {
                if let Some(base) = field_expr.base()
                    && matches!(base, ast::Expr::NameRef(_) | ast::Expr::FieldExpr(_))
                {
                    return Some(NameRefClass::SelectQualifiedColumn);
                }
            }
            if ast::Select::can_cast(ancestor.kind()) && (!in_from_clause || in_on_clause) {
                if in_cast_expr {
                    return Some(NameRefClass::Type);
                }
                if is_base_of_outer_field_expr {
                    return Some(NameRefClass::SelectQualifiedColumnTable);
                } else if let Some(base) = field_expr.base()
                    && matches!(base, ast::Expr::NameRef(_) | ast::Expr::FieldExpr(_))
                {
                    return Some(NameRefClass::SelectQualifiedColumn);
                } else if let Some(ast::Expr::ParenExpr(_)) = field_expr.base() {
                    return Some(NameRefClass::CompositeTypeField);
                } else {
                    return Some(NameRefClass::SelectQualifiedColumnTable);
                }
            }
            if ast::CreatePolicy::can_cast(ancestor.kind())
                || ast::AlterPolicy::can_cast(ancestor.kind())
            {
                if is_base_of_outer_field_expr {
                    return Some(NameRefClass::PolicyQualifiedColumnTable);
                } else {
                    return Some(NameRefClass::PolicyColumn);
                }
            }
        }
    }

    if let Some(parent) = name_ref.syntax().parent()
        && let Some(inner_path) = ast::PathSegment::cast(parent)
            .and_then(|p| p.syntax().parent().and_then(ast::Path::cast))
        && let Some(outer_path) = inner_path
            .syntax()
            .parent()
            .and_then(|p| ast::Path::cast(p).and_then(|p| p.qualifier()))
        && outer_path.syntax() == inner_path.syntax()
    {
        return Some(NameRefClass::Schema);
    }

    // Check for function/procedure reference in CREATE OPERATOR before the type check
    for ancestor in name_ref.syntax().ancestors() {
        if let Some(attr_option) = ast::AttributeOption::cast(ancestor.clone())
            && let Some(name) = attr_option.name()
        {
            let attr_name = Name::from_node(&name);
            if attr_name == Name::from_string("function")
                || attr_name == Name::from_string("procedure")
            {
                for outer in attr_option.syntax().ancestors() {
                    if ast::CreateOperator::can_cast(outer.kind()) {
                        return Some(NameRefClass::FunctionName);
                    }
                }
            }
        }
    }

    let mut in_type = false;
    for ancestor in name_ref.syntax().ancestors() {
        if ast::PathType::can_cast(ancestor.kind()) || ast::ExprType::can_cast(ancestor.kind()) {
            in_type = true;
        }
        if in_type {
            return Some(NameRefClass::Type);
        }
        if ast::Fetch::can_cast(ancestor.kind())
            || ast::Move::can_cast(ancestor.kind())
            || ast::Close::can_cast(ancestor.kind())
            || ast::WhereCurrentOf::can_cast(ancestor.kind())
        {
            return Some(NameRefClass::Cursor);
        }
        if ast::Execute::can_cast(ancestor.kind()) || ast::Deallocate::can_cast(ancestor.kind()) {
            return Some(NameRefClass::PreparedStatement);
        }
        if ast::Notify::can_cast(ancestor.kind()) || ast::Unlisten::can_cast(ancestor.kind()) {
            return Some(NameRefClass::Channel);
        }
        if ast::DropTable::can_cast(ancestor.kind())
            || ast::DropForeignTable::can_cast(ancestor.kind())
            || ast::Truncate::can_cast(ancestor.kind())
            || ast::Lock::can_cast(ancestor.kind())
            || ast::Vacuum::can_cast(ancestor.kind())
            || ast::AlterTable::can_cast(ancestor.kind())
            || ast::OnTable::can_cast(ancestor.kind())
            || ast::AttachPartition::can_cast(ancestor.kind())
            || ast::Table::can_cast(ancestor.kind())
            || ast::Inherits::can_cast(ancestor.kind())
            || ast::PartitionOf::can_cast(ancestor.kind())
        {
            return Some(NameRefClass::Table);
        }
        if ast::AlterColumn::can_cast(ancestor.kind()) || ast::DropColumn::can_cast(ancestor.kind())
        {
            return Some(NameRefClass::AlterColumn);
        }
        if let Some(comment_on) = ast::CommentOn::cast(ancestor.clone()) {
            if comment_on.table_token().is_some() {
                return Some(NameRefClass::Table);
            }
            if comment_on.column_token().is_some() {
                return Some(NameRefClass::QualifiedColumn);
            }
        }
        if let Some(reindex) = ast::Reindex::cast(ancestor.clone()) {
            if reindex.table_token().is_some() {
                return Some(NameRefClass::Table);
            }
            if reindex.index_token().is_some() {
                return Some(NameRefClass::Index);
            }
            if reindex.schema_token().is_some() {
                return Some(NameRefClass::Schema);
            }
            if reindex.database_token().is_some() || reindex.system_token().is_some() {
                return Some(NameRefClass::Database);
            }
        }
        if ast::DropIndex::can_cast(ancestor.kind()) {
            return Some(NameRefClass::Index);
        }
        if ast::DropType::can_cast(ancestor.kind()) || ast::DropDomain::can_cast(ancestor.kind()) {
            return Some(NameRefClass::Type);
        }
        if ast::DropView::can_cast(ancestor.kind())
            || ast::DropMaterializedView::can_cast(ancestor.kind())
            || ast::Refresh::can_cast(ancestor.kind())
        {
            return Some(NameRefClass::View);
        }
        if ast::DropSequence::can_cast(ancestor.kind()) {
            return Some(NameRefClass::Sequence);
        }
        if ast::DropTrigger::can_cast(ancestor.kind()) {
            return Some(NameRefClass::Trigger);
        }
        if ast::DropPolicy::can_cast(ancestor.kind()) || ast::AlterPolicy::can_cast(ancestor.kind())
        {
            return Some(NameRefClass::Policy);
        }
        if ast::DropEventTrigger::can_cast(ancestor.kind())
            || ast::AlterEventTrigger::can_cast(ancestor.kind())
        {
            return Some(NameRefClass::EventTrigger);
        }
        if ast::DropDatabase::can_cast(ancestor.kind()) {
            return Some(NameRefClass::Database);
        }
        if ast::DropServer::can_cast(ancestor.kind())
            || ast::AlterServer::can_cast(ancestor.kind())
            || ast::CreateServer::can_cast(ancestor.kind())
            || ast::ServerName::can_cast(ancestor.kind())
        {
            return Some(NameRefClass::Server);
        }
        if ast::DropExtension::can_cast(ancestor.kind())
            || ast::AlterExtension::can_cast(ancestor.kind())
        {
            return Some(NameRefClass::Extension);
        }
        if ast::AlterRole::can_cast(ancestor.kind())
            || ast::DropRole::can_cast(ancestor.kind())
            || ast::SetRole::can_cast(ancestor.kind())
            || ast::RoleRef::can_cast(ancestor.kind())
        {
            return Some(NameRefClass::Role);
        }
        if let Some(sequence_option) = ast::SequenceOption::cast(ancestor.clone())
            && sequence_option.owned_token().is_some()
            && sequence_option.by_token().is_some()
        {
            return Some(NameRefClass::QualifiedColumn);
        }
        if ast::DropTablespace::can_cast(ancestor.kind())
            || ast::Tablespace::can_cast(ancestor.kind())
            || ast::SetTablespace::can_cast(ancestor.kind())
            || ast::ConstraintIndexTablespace::can_cast(ancestor.kind())
        {
            return Some(NameRefClass::Tablespace);
        }
        if ast::SetNullColumns::can_cast(ancestor.kind()) {
            in_set_null_columns = true;
        }
        if let Some(foreign_key) = ast::ForeignKeyConstraint::cast(ancestor.clone()) {
            if in_column_list {
                // TODO: ast is too "flat" here, we need a unique node for to
                // and from columns to differentiate which would help us avoid
                // this
                if let Some(to_columns) = foreign_key.to_columns()
                    && to_columns
                        .syntax()
                        .text_range()
                        .contains_range(name_ref.syntax().text_range())
                {
                    return Some(NameRefClass::ForeignKeyColumn);
                }
                if let Some(from_columns) = foreign_key.from_columns()
                    && from_columns
                        .syntax()
                        .text_range()
                        .contains_range(name_ref.syntax().text_range())
                {
                    return Some(NameRefClass::ConstraintColumn);
                }
                if in_set_null_columns {
                    return Some(NameRefClass::ConstraintColumn);
                }
            } else {
                return Some(NameRefClass::ForeignKeyTable);
            }
        }
        if let Some(references_constraint) = ast::ReferencesConstraint::cast(ancestor.clone()) {
            // TODO: the ast is too flat here
            if let Some(column_ref) = references_constraint.column()
                && column_ref
                    .syntax()
                    .text_range()
                    .contains_range(name_ref.syntax().text_range())
            {
                return Some(NameRefClass::ForeignKeyColumn);
            }
            if let Some(path) = references_constraint.table()
                && path
                    .syntax()
                    .text_range()
                    .contains_range(name_ref.syntax().text_range())
            {
                return Some(NameRefClass::ForeignKeyTable);
            }
        }
        if ast::CreatePolicy::can_cast(ancestor.kind())
            || ast::AlterPolicy::can_cast(ancestor.kind())
        {
            return Some(NameRefClass::PolicyColumn);
        }
        if ast::CheckConstraint::can_cast(ancestor.kind())
            || ast::GeneratedConstraint::can_cast(ancestor.kind())
            || ast::NotNullConstraint::can_cast(ancestor.kind())
        {
            return Some(NameRefClass::ConstraintColumn);
        }
        if in_column_list
            && (ast::UniqueConstraint::can_cast(ancestor.kind())
                || ast::PrimaryKeyConstraint::can_cast(ancestor.kind()))
        {
            return Some(NameRefClass::ConstraintColumn);
        }
        if (in_constraint_exclusion_list
            || in_constraint_include_clause
            || in_constraint_where_clause)
            && ast::ExcludeConstraint::can_cast(ancestor.kind())
        {
            return Some(NameRefClass::ConstraintColumn);
        }
        if ast::LikeClause::can_cast(ancestor.kind()) {
            return Some(NameRefClass::LikeTable);
        }
        if ast::CastExpr::can_cast(ancestor.kind()) && in_type {
            return Some(NameRefClass::Type);
        }
        if ast::DropFunction::can_cast(ancestor.kind()) {
            return Some(NameRefClass::Function);
        }
        if ast::DropAggregate::can_cast(ancestor.kind()) {
            return Some(NameRefClass::Aggregate);
        }
        if ast::DropProcedure::can_cast(ancestor.kind()) {
            return Some(NameRefClass::Procedure);
        }
        if ast::DropRoutine::can_cast(ancestor.kind()) {
            return Some(NameRefClass::Routine);
        }
        if ast::Call::can_cast(ancestor.kind()) {
            return Some(NameRefClass::CallProcedure);
        }
        if ast::DropSchema::can_cast(ancestor.kind()) {
            return Some(NameRefClass::Schema);
        }
        if ast::CreateIndex::can_cast(ancestor.kind()) {
            if in_partition_item {
                return Some(NameRefClass::CreateIndexColumn);
            }
            return Some(NameRefClass::Table);
        }
        if let Some(create_trigger) = ast::CreateTrigger::cast(ancestor.clone())
            && in_function_name
        {
            if create_trigger.procedure_token().is_some() {
                return Some(NameRefClass::ProcedureCall);
            }
            return Some(NameRefClass::FunctionCall);
        }
        if let Some(create_event_trigger) = ast::CreateEventTrigger::cast(ancestor.clone())
            && in_function_name
        {
            if create_event_trigger.procedure_token().is_some() {
                return Some(NameRefClass::ProcedureCall);
            }
            return Some(NameRefClass::FunctionCall);
        }
        if in_partition_item && ast::CreateTableLike::can_cast(ancestor.kind()) {
            return Some(NameRefClass::ConstraintColumn);
        }
        if is_special_fn(ancestor.kind()) {
            in_special_sql_fn = true;
        }
        if ast::NamedArg::can_cast(ancestor.kind()) {
            return Some(NameRefClass::NamedArgParameter);
        }
        if ast::ArgList::can_cast(ancestor.kind()) {
            in_arg_list = true;
        }
        if ast::CallExpr::can_cast(ancestor.kind()) {
            if !in_arg_list {
                in_function_name = true;
            }
        }
        if ast::DefaultConstraint::can_cast(ancestor.kind()) && in_function_name {
            return Some(NameRefClass::FunctionCall);
        }
        if ast::OnClause::can_cast(ancestor.kind()) {
            in_on_clause = true;
        }
        if ast::FromClause::can_cast(ancestor.kind()) {
            in_from_clause = true;
        }
        if ast::Select::can_cast(ancestor.kind()) {
            if in_function_name && !in_special_sql_fn {
                return Some(NameRefClass::SelectFunctionCall);
            }
            if in_from_clause && !in_on_clause {
                return Some(NameRefClass::FromTable);
            }
            // Classify as SelectColumn for target list, WHERE, ORDER BY, GROUP BY, etc.
            // (anything in SELECT except FROM clause)
            return Some(NameRefClass::SelectColumn);
        }
        if ast::ColumnList::can_cast(ancestor.kind()) {
            in_column_list = true;
        }
        if ast::ConstraintExclusionList::can_cast(ancestor.kind()) {
            in_constraint_exclusion_list = true;
        }
        if ast::ConstraintIncludeClause::can_cast(ancestor.kind()) {
            in_constraint_include_clause = true;
        }
        if ast::WhereConditionClause::can_cast(ancestor.kind()) {
            in_constraint_where_clause = true;
        }
        if ast::PartitionItem::can_cast(ancestor.kind()) {
            in_partition_item = true;
        }
        if ast::Insert::can_cast(ancestor.kind()) {
            if in_returning_clause || in_column_list {
                return Some(NameRefClass::InsertColumn);
            }
            return Some(NameRefClass::Table);
        }
        if ast::JoinUsingClause::can_cast(ancestor.kind()) && in_column_list {
            return Some(NameRefClass::JoinUsingColumn);
        }
        if ast::WhereClause::can_cast(ancestor.kind()) {
            in_where_clause = true;
        }
        if ast::SetClause::can_cast(ancestor.kind()) {
            in_set_clause = true;
        }
        if ast::UsingClause::can_cast(ancestor.kind()) {
            in_using_clause = true;
        }
        if ast::UsingOnClause::can_cast(ancestor.kind()) {
            in_using_clause = true;
        }
        if ast::ReturningClause::can_cast(ancestor.kind()) {
            in_returning_clause = true;
        }
        if ast::Delete::can_cast(ancestor.kind()) {
            if in_returning_clause || in_where_clause {
                return Some(NameRefClass::DeleteColumn);
            }
            if in_using_clause {
                return Some(NameRefClass::FromTable);
            }
            return Some(NameRefClass::Table);
        }
        if ast::Update::can_cast(ancestor.kind()) {
            if in_returning_clause || in_where_clause || in_set_clause {
                return Some(NameRefClass::UpdateColumn);
            }
            if in_from_clause {
                return Some(NameRefClass::FromTable);
            }
            return Some(NameRefClass::Table);
        }
        if ast::MergeWhenClause::can_cast(ancestor.kind()) {
            in_when_clause = true;
        }
        if ast::Merge::can_cast(ancestor.kind()) {
            if in_when_clause || in_returning_clause || in_on_clause {
                return Some(NameRefClass::MergeColumn);
            }
            if in_using_clause {
                return Some(NameRefClass::FromTable);
            }
            return Some(NameRefClass::Table);
        }
    }

    None
}

#[derive(Debug)]
pub(crate) enum NameClass {
    ColumnDefinition {
        create_table: ast::CreateTableLike,
        column: ast::Column,
    },
    CreateTable(ast::CreateTableLike),
    WithTable(ast::WithTable),
    CreateIndex(ast::CreateIndex),
    CreateSequence(ast::CreateSequence),
    CreateTrigger(ast::CreateTrigger),
    CreateEventTrigger(ast::CreateEventTrigger),
    CreateTablespace(ast::CreateTablespace),
    CreateDatabase(ast::CreateDatabase),
    CreateServer(ast::CreateServer),
    CreateExtension(ast::CreateExtension),
    CreateRole(ast::CreateRole),
    CreateType(ast::CreateType),
    CreateFunction(ast::CreateFunction),
    CreateAggregate(ast::CreateAggregate),
    CreateProcedure(ast::CreateProcedure),
    CreateSchema(ast::CreateSchema),
    ViewColumnList {
        create_view: ast::CreateView,
        name: ast::Name,
    },
    CreateView(ast::CreateView),
    DeclareCursor(ast::Declare),
    PrepareStatement(ast::Prepare),
    Listen(ast::Listen),
}

pub(crate) fn classify_name(name: &ast::Name) -> Option<NameClass> {
    let parent = name.syntax().parent();
    let column_parent = parent.clone().and_then(ast::Column::cast);
    let with_table_parent = parent.and_then(ast::WithTable::cast);
    let mut has_column_list = false;

    for ancestor in name.syntax().ancestors() {
        if !has_column_list && ast::ColumnList::can_cast(ancestor.kind()) {
            has_column_list = true;
        }
        if let Some(create_table) = ast::CreateTableLike::cast(ancestor.clone()) {
            if let Some(column) = column_parent {
                return Some(NameClass::ColumnDefinition {
                    create_table,
                    column,
                });
            }
            return Some(NameClass::CreateTable(create_table));
        }
        if let Some(create_index) = ast::CreateIndex::cast(ancestor.clone()) {
            return Some(NameClass::CreateIndex(create_index));
        }
        if let Some(create_sequence) = ast::CreateSequence::cast(ancestor.clone()) {
            return Some(NameClass::CreateSequence(create_sequence));
        }
        if let Some(create_trigger) = ast::CreateTrigger::cast(ancestor.clone()) {
            return Some(NameClass::CreateTrigger(create_trigger));
        }
        if let Some(create_event_trigger) = ast::CreateEventTrigger::cast(ancestor.clone()) {
            return Some(NameClass::CreateEventTrigger(create_event_trigger));
        }
        if let Some(create_tablespace) = ast::CreateTablespace::cast(ancestor.clone()) {
            return Some(NameClass::CreateTablespace(create_tablespace));
        }
        if let Some(create_database) = ast::CreateDatabase::cast(ancestor.clone()) {
            return Some(NameClass::CreateDatabase(create_database));
        }
        if let Some(create_server) = ast::CreateServer::cast(ancestor.clone()) {
            return Some(NameClass::CreateServer(create_server));
        }
        if let Some(create_extension) = ast::CreateExtension::cast(ancestor.clone()) {
            return Some(NameClass::CreateExtension(create_extension));
        }
        if let Some(create_role) = ast::CreateRole::cast(ancestor.clone()) {
            return Some(NameClass::CreateRole(create_role));
        }
        if let Some(create_type) = ast::CreateType::cast(ancestor.clone()) {
            return Some(NameClass::CreateType(create_type));
        }
        if let Some(create_function) = ast::CreateFunction::cast(ancestor.clone()) {
            return Some(NameClass::CreateFunction(create_function));
        }
        if let Some(create_aggregate) = ast::CreateAggregate::cast(ancestor.clone()) {
            return Some(NameClass::CreateAggregate(create_aggregate));
        }
        if let Some(create_procedure) = ast::CreateProcedure::cast(ancestor.clone()) {
            return Some(NameClass::CreateProcedure(create_procedure));
        }
        if let Some(create_schema) = ast::CreateSchema::cast(ancestor.clone()) {
            return Some(NameClass::CreateSchema(create_schema));
        }
        if let Some(create_view) = ast::CreateView::cast(ancestor.clone()) {
            if has_column_list {
                return Some(NameClass::ViewColumnList {
                    create_view,
                    name: name.clone(),
                });
            }
            return Some(NameClass::CreateView(create_view));
        }
        if let Some(declare) = ast::Declare::cast(ancestor.clone()) {
            return Some(NameClass::DeclareCursor(declare));
        }
        if let Some(prepare) = ast::Prepare::cast(ancestor.clone()) {
            return Some(NameClass::PrepareStatement(prepare));
        }
        if let Some(listen) = ast::Listen::cast(ancestor.clone()) {
            return Some(NameClass::Listen(listen));
        }
    }

    if let Some(with_table) = with_table_parent {
        return Some(NameClass::WithTable(with_table));
    }

    None
}

#[test]
fn special_function() {
    for kind in (0..SyntaxKind::__LAST as u16)
        .map(SyntaxKind::from)
        .filter(|kind| format!("{:?}", kind).ends_with("_FN"))
    {
        assert!(
            is_special_fn(kind),
            "unhandled special function kind: {:?}. Please update is_special_fn",
            kind
        )
    }
}
