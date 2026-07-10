use crate::{location::LocationKind, symbols::Name};
use squawk_syntax::{
    SyntaxKind, SyntaxNode,
    ast::{self, AstNode},
};

#[derive(Debug, Clone, Copy)]
pub(crate) enum NameRefClass {
    Aggregate,
    AlterColumn,
    CallProcedure,
    Channel,
    Collation,
    CompositeTypeField,
    Constraint,
    ConstraintColumn,
    CopyColumn,
    CreateIndexColumn,
    Cursor,
    Database,
    DeleteColumn,
    DeleteQualifiedColumnTable,
    EventTrigger,
    Extension,
    ForeignDataWrapper,
    ForeignKeyColumn,
    ForeignKeyTable,
    FromTable,
    Function,
    FunctionCall,
    FunctionName,
    Index,
    InsertColumn,
    InsertQualifiedColumnTable,
    InsertTable,
    JoinUsingColumn,
    Language,
    LikeTable,
    MergeColumn,
    MergeQualifiedColumnTable,
    NamedArgParameter,
    ParamDefault,
    Policy,
    PolicyColumn,
    PolicyQualifiedColumnTable,
    PreparedStatement,
    PrivilegeColumn,
    PrivilegeObjectTable,
    Procedure,
    ProcedureCall,
    PropertyGraph,
    PropertyGraphColumn,
    Publication,
    PublicationColumn,
    QualifiedColumn,
    Role,
    Routine,
    Rule,
    RulePseudoColumn,
    RulePseudoColumnTable,
    Schema,
    SelectColumn,
    SelectFunctionCall,
    SelectGroupByAliasOrColumn,
    SelectOrderByAliasOrColumn,
    SelectQualifiedColumn,
    SelectQualifiedColumnTable,
    Sequence,
    Server,
    StatisticsColumn,
    Subscription,
    Table,
    TableAndColumnsColumn,
    Tablespace,
    Trigger,
    TriggerEventColumn,
    TriggerWhenColumn,
    TriggerWhenColumnTable,
    Type,
    UpdateColumn,
    UpdateQualifiedColumnTable,
    View,
    Window,
}

fn is_create_aggregate_function_option(option_name: &Name) -> bool {
    matches!(
        option_name.0.as_str(),
        "combinefunc"
            | "deserialfunc"
            | "finalfunc"
            | "mfinalfunc"
            | "minvfunc"
            | "msfunc"
            | "serialfunc"
            | "sfunc"
    )
}

fn is_create_operator_function_option(option_name: &Name) -> bool {
    matches!(option_name.0.as_str(), "function" | "procedure")
}

fn is_create_type_function_option(option_name: &Name) -> bool {
    matches!(
        option_name.0.as_str(),
        "analyze"
            | "canonical"
            | "input"
            | "output"
            | "receive"
            | "send"
            | "subscript"
            | "subtype_diff"
            | "typmod_in"
            | "typmod_out"
    )
}

fn classify_ddl_function_option_value(ty_node: &SyntaxNode) -> Option<NameRefClass> {
    let attribute_option = ty_node
        .parent()
        .and_then(ast::AttributeValue::cast)?
        .syntax()
        .parent()
        .and_then(ast::AttributeOption::cast)?;
    let attr_name = Name::from_node(&attribute_option.name()?);
    let ddl_node = attribute_option
        .syntax()
        .parent()
        .and_then(|attribute_list| attribute_list.parent())?;
    let ddl_node = if ast::CreateTypeKind::can_cast(ddl_node.kind()) {
        ddl_node.parent()?
    } else {
        ddl_node
    };

    if ast::CreateOperator::can_cast(ddl_node.kind())
        && is_create_operator_function_option(&attr_name)
    {
        return Some(NameRefClass::FunctionName);
    }
    if ast::CreateAggregate::can_cast(ddl_node.kind())
        && is_create_aggregate_function_option(&attr_name)
    {
        return Some(NameRefClass::FunctionName);
    }
    if ast::CreateType::can_cast(ddl_node.kind()) && is_create_type_function_option(&attr_name) {
        return Some(NameRefClass::FunctionName);
    }
    None
}

fn is_rule_old_new_ref(name_ref: &ast::NameRef) -> bool {
    name_ref
        .syntax()
        .first_token()
        .is_some_and(|t| matches!(t.kind(), SyntaxKind::OLD_KW | SyntaxKind::NEW_KW))
        && name_ref
            .syntax()
            .ancestors()
            .any(|a| ast::CreateRule::can_cast(a.kind()))
}

fn is_special_fn(kind: SyntaxKind) -> bool {
    matches!(
        kind,
        SyntaxKind::EXTRACT_FN
            | SyntaxKind::COLLATION_FOR_FN
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
            | SyntaxKind::GRAPH_TABLE_FN
    )
}

pub(crate) fn classify_name_ref(node: &SyntaxNode) -> Option<NameRefClass> {
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
    let mut in_using_clause = false;
    let mut in_returning_clause = false;
    let mut in_when_clause = false;
    let mut in_special_sql_fn = false;
    let mut in_filter_clause = false;
    let mut in_conflict_target = false;
    let mut in_group_by_clause = false;
    let mut in_order_by_clause = false;
    let mut in_distinct_clause = false;

    // TODO: can we combine this if and the one that follows?
    if let Some(parent) = node.parent()
        && let Some(field_expr) = ast::FieldExpr::cast(parent.clone())
        && let Some(base) = field_expr.base()
        && let ast::Expr::NameRef(base_name_ref) = base
        // check that the name_ref we're looking at in the field expr is the
        // base name_ref, i.e., the schema, rather than the item
        && base_name_ref.syntax() == node
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

        if is_rule_old_new_ref(&base_name_ref) {
            return Some(NameRefClass::RulePseudoColumnTable);
        }

        let mut in_arg_list = false;
        let mut in_from_clause = false;
        let mut in_on_clause = false;
        let mut in_returning_clause = false;
        let mut in_set_clause = false;
        let mut in_where_clause = false;
        let mut in_when_clause = false;
        let mut in_when_condition = false;
        for ancestor in parent.ancestors() {
            if ast::ArgList::can_cast(ancestor.kind()) {
                in_arg_list = true;
            }
            if ast::WhenCondition::can_cast(ancestor.kind()) {
                in_when_condition = true;
            }
            if ast::CreateTrigger::can_cast(ancestor.kind()) && in_when_condition {
                return Some(NameRefClass::TriggerWhenColumnTable);
            }
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
            if ast::Select::can_cast(ancestor.kind())
                && (!in_from_clause || in_on_clause || in_arg_list)
            {
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

    if let Some(parent) = node.parent()
        && let Some(field_expr) = ast::FieldExpr::cast(parent.clone())
        && field_expr
            .field()
            // we're at the field in a FieldExpr, i.e., foo.bar
            //                                              ^^^
            .is_some_and(|field_name_ref| field_name_ref.syntax() == node)
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

        if let Some(ast::Expr::NameRef(base_name_ref)) = field_expr.base()
            && is_rule_old_new_ref(&base_name_ref)
        {
            return Some(NameRefClass::RulePseudoColumn);
        }

        let mut in_from_clause = false;
        let mut in_on_clause = false;
        let mut in_cast_expr = false;
        let mut in_when_clause = false;
        let mut in_when_condition = false;
        let mut in_returning_clause = false;
        for ancestor in parent.ancestors() {
            if ast::OnClause::can_cast(ancestor.kind()) {
                in_on_clause = true;
            }
            if ast::WhenCondition::can_cast(ancestor.kind()) {
                in_when_condition = true;
            }
            if ast::CreateTrigger::can_cast(ancestor.kind()) && in_when_condition {
                return Some(NameRefClass::TriggerWhenColumn);
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

    // %type clause paths (max 3 segments):
    //   column%type, table.column%type, schema.table.column%type
    if let Some(parent) = node.parent()
        && let Some(mut path) = ast::PathSegment::cast(parent)
            .and_then(|p| p.syntax().parent().and_then(ast::Path::cast))
    {
        let mut hops_up = 0;
        while let Some(next) = path.syntax().parent().and_then(ast::Path::cast) {
            path = next;
            hops_up += 1;
        }
        if path
            .syntax()
            .parent()
            .is_some_and(|p| ast::PercentType::can_cast(p.kind()))
        {
            return match hops_up {
                0 => Some(NameRefClass::QualifiedColumn),
                1 => Some(NameRefClass::Table),
                2 => Some(NameRefClass::Schema),
                _ => None,
            };
        }
    }

    if let Some(parent) = node.parent()
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

    if let Some(parent) = node.parent()
        && let Some(path) = ast::PathSegment::cast(parent)
            .and_then(|p| p.syntax().parent().and_then(ast::Path::cast))
        && let Some(stmt_parent) = path.syntax().parent()
        && (ast::AlterPropertyGraph::can_cast(stmt_parent.kind())
            || ast::DropPropertyGraph::can_cast(stmt_parent.kind())
            || ast::GraphTableFn::can_cast(stmt_parent.kind()))
    {
        return Some(NameRefClass::PropertyGraph);
    }

    if let Some(parent) = node.parent()
        && let Some(path) = ast::PathSegment::cast(parent)
            .and_then(|p| p.syntax().parent().and_then(ast::Path::cast))
        && let Some(stmt_parent) = path.syntax().parent()
        && (ast::AlterType::can_cast(stmt_parent.kind())
            || ast::AlterDomain::can_cast(stmt_parent.kind()))
    {
        return Some(NameRefClass::Type);
    }

    if let Some(parent) = node.parent()
        && let Some(expr_as_name) = ast::ExprAsName::cast(parent)
        && let Some(expr_as_name_list) = ast::ExprAsNameList::cast(expr_as_name.syntax().parent()?)
        && ast::Properties::cast(expr_as_name_list.syntax().parent()?).is_some()
    {
        return Some(NameRefClass::PropertyGraphColumn);
    }

    let mut in_type = false;
    for ancestor in node.ancestors() {
        if let Some(privilege_objects) = ast::PrivilegeObjects::cast(ancestor.clone()) {
            return classify_privilege_object(&privilege_objects);
        }
        if in_column_list
            && (ast::Grant::can_cast(ancestor.kind()) || ast::Revoke::can_cast(ancestor.kind()))
        {
            return Some(NameRefClass::PrivilegeColumn);
        }
        if let Some(function_sig) = ast::FunctionSig::cast(ancestor.clone())
            && function_sig
                .syntax()
                .parent()
                .is_some_and(|parent| ast::WithFunction::can_cast(parent.kind()))
            && function_sig
                .path()
                .is_some_and(|path| path.syntax().text_range().contains_range(node.text_range()))
        {
            return Some(NameRefClass::Function);
        }
        if ast::PathType::can_cast(ancestor.kind()) || ast::ExprType::can_cast(ancestor.kind()) {
            if let Some(class) = classify_ddl_function_option_value(&ancestor) {
                return Some(class);
            }
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
        if ast::SetConstraints::can_cast(ancestor.kind()) {
            return Some(NameRefClass::Constraint);
        }
        if in_column_list
            && (ast::VertexTableDef::can_cast(ancestor.kind())
                || ast::EdgeTableDef::can_cast(ancestor.kind())
                || ast::SourceVertexTable::can_cast(ancestor.kind())
                || ast::DestVertexTable::can_cast(ancestor.kind()))
        {
            return Some(NameRefClass::PropertyGraphColumn);
        }
        if in_column_list
            && (ast::Vacuum::can_cast(ancestor.kind()) || ast::Analyze::can_cast(ancestor.kind()))
        {
            return Some(NameRefClass::TableAndColumnsColumn);
        }
        if in_column_list && ast::Copy::can_cast(ancestor.kind()) {
            return Some(NameRefClass::CopyColumn);
        }
        if ast::StatTypes::can_cast(ancestor.kind()) {
            return None;
        }
        if ast::FromTable::can_cast(ancestor.kind()) {
            return Some(NameRefClass::Table);
        }
        if ast::RuleOn::can_cast(ancestor.kind()) && !in_where_clause {
            return Some(NameRefClass::Table);
        }
        if ast::TriggerEventUpdate::can_cast(ancestor.kind()) {
            return Some(NameRefClass::TriggerEventColumn);
        }
        if ast::CreateStatistics::can_cast(ancestor.kind()) {
            return Some(NameRefClass::StatisticsColumn);
        }
        if let Some(publication_object) = ast::PublicationObject::cast(ancestor.clone()) {
            if publication_object.table_token().is_some() {
                if in_column_list || in_constraint_where_clause {
                    return Some(NameRefClass::PublicationColumn);
                }
                return Some(NameRefClass::Table);
            }
            if publication_object.schema_token().is_some() {
                return Some(NameRefClass::Schema);
            }
            return None;
        }
        if ast::AlterConstraint::can_cast(ancestor.kind())
            || ast::DropConstraint::can_cast(ancestor.kind())
            || ast::RenameConstraint::can_cast(ancestor.kind())
            || ast::ValidateConstraint::can_cast(ancestor.kind())
        {
            return Some(NameRefClass::Constraint);
        }
        if ast::DropTable::can_cast(ancestor.kind())
            || ast::DropForeignTable::can_cast(ancestor.kind())
            || ast::Truncate::can_cast(ancestor.kind())
            || ast::Lock::can_cast(ancestor.kind())
            || ast::Vacuum::can_cast(ancestor.kind())
            || ast::Analyze::can_cast(ancestor.kind())
            || ast::Copy::can_cast(ancestor.kind())
            || ast::AlterTable::can_cast(ancestor.kind())
            || ast::AlterForeignTable::can_cast(ancestor.kind())
            || ast::OnTable::can_cast(ancestor.kind())
            || ast::AttachPartition::can_cast(ancestor.kind())
            || ast::DetachPartition::can_cast(ancestor.kind())
            || ast::Table::can_cast(ancestor.kind())
            || ast::Inherits::can_cast(ancestor.kind())
            || ast::PartitionOf::can_cast(ancestor.kind())
            || ast::VertexTableDef::can_cast(ancestor.kind())
            || ast::EdgeTableDef::can_cast(ancestor.kind())
            || ast::SourceVertexTable::can_cast(ancestor.kind())
            || ast::DestVertexTable::can_cast(ancestor.kind())
            || ast::ObjectTable::can_cast(ancestor.kind())
            || ast::ObjectForeignTable::can_cast(ancestor.kind())
        {
            return Some(NameRefClass::Table);
        }
        if ast::AlterColumn::can_cast(ancestor.kind())
            || ast::DropColumn::can_cast(ancestor.kind())
            || ast::RenameColumn::can_cast(ancestor.kind())
        {
            return Some(NameRefClass::AlterColumn);
        }
        if ast::ObjectColumn::can_cast(ancestor.kind()) {
            return Some(NameRefClass::QualifiedColumn);
        }
        if let Some(comment_constraint) = ast::ObjectConstraint::cast(ancestor.clone()) {
            if comment_constraint
                .name_ref()
                .is_some_and(|constraint_name| constraint_name.syntax() == node)
            {
                return Some(NameRefClass::Constraint);
            }
            if comment_constraint.domain_token().is_some() {
                return Some(NameRefClass::Type);
            }
            return Some(NameRefClass::Table);
        }
        if let Some(routine) = ast::ObjectRoutine::cast(ancestor.clone()) {
            if routine.procedure_token().is_some() {
                return Some(NameRefClass::Procedure);
            }
            if routine.routine_token().is_some() {
                return Some(NameRefClass::Routine);
            }
            return Some(NameRefClass::Function);
        }
        if ast::ObjectAggregate::can_cast(ancestor.kind()) {
            return Some(NameRefClass::Aggregate);
        }
        if ast::ObjectSchema::can_cast(ancestor.kind())
            || ast::AlterDefaultPrivileges::can_cast(ancestor.kind())
        {
            return Some(NameRefClass::Schema);
        }
        if ast::ObjectTable::can_cast(ancestor.kind())
            || ast::ObjectForeignTable::can_cast(ancestor.kind())
        {
            return Some(NameRefClass::Table);
        }
        if ast::ObjectMaterializedView::can_cast(ancestor.kind()) {
            return Some(NameRefClass::View);
        }
        if ast::ObjectEventTrigger::can_cast(ancestor.kind()) {
            return Some(NameRefClass::EventTrigger);
        }
        if ast::ForProvider::can_cast(ancestor.kind()) {
            return None;
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
        if let Some(using_method) = ast::UsingMethod::cast(ancestor.clone())
            && ast::Cluster::can_cast(using_method.syntax().parent()?.kind())
        {
            return Some(NameRefClass::Index);
        }
        if ast::Cluster::can_cast(ancestor.kind()) {
            return Some(NameRefClass::Table);
        }
        if ast::DropIndex::can_cast(ancestor.kind())
            || ast::AlterIndex::can_cast(ancestor.kind())
            || ast::UsingIndex::can_cast(ancestor.kind())
            || ast::ObjectIndex::can_cast(ancestor.kind())
        {
            return Some(NameRefClass::Index);
        }
        if ast::DropType::can_cast(ancestor.kind())
            || ast::DropDomain::can_cast(ancestor.kind())
            || ast::ObjectType::can_cast(ancestor.kind())
            || ast::ObjectDomain::can_cast(ancestor.kind())
        {
            return Some(NameRefClass::Type);
        }
        if ast::DropView::can_cast(ancestor.kind())
            || ast::AlterView::can_cast(ancestor.kind())
            || ast::DropMaterializedView::can_cast(ancestor.kind())
            || ast::AlterMaterializedView::can_cast(ancestor.kind())
            || ast::Refresh::can_cast(ancestor.kind())
            || ast::ObjectView::can_cast(ancestor.kind())
        {
            return Some(NameRefClass::View);
        }
        if ast::DropSequence::can_cast(ancestor.kind())
            || ast::AlterSequence::can_cast(ancestor.kind())
            || ast::ObjectSequence::can_cast(ancestor.kind())
        {
            return Some(NameRefClass::Sequence);
        }
        if ast::DropTrigger::can_cast(ancestor.kind())
            || ast::AlterTrigger::can_cast(ancestor.kind())
        {
            return Some(NameRefClass::Trigger);
        }
        if ast::DropPolicy::can_cast(ancestor.kind()) || ast::AlterPolicy::can_cast(ancestor.kind())
        {
            return Some(NameRefClass::Policy);
        }
        if ast::DropRule::can_cast(ancestor.kind()) || ast::AlterRule::can_cast(ancestor.kind()) {
            return Some(NameRefClass::Rule);
        }
        if ast::DropEventTrigger::can_cast(ancestor.kind())
            || ast::AlterEventTrigger::can_cast(ancestor.kind())
        {
            return Some(NameRefClass::EventTrigger);
        }
        if ast::DropDatabase::can_cast(ancestor.kind())
            || ast::AlterDatabase::can_cast(ancestor.kind())
            || ast::ObjectDatabase::can_cast(ancestor.kind())
        {
            return Some(NameRefClass::Database);
        }
        if ast::DropServer::can_cast(ancestor.kind())
            || ast::AlterServer::can_cast(ancestor.kind())
            || ast::ServerName::can_cast(ancestor.kind())
            || ast::ObjectServer::can_cast(ancestor.kind())
        {
            return Some(NameRefClass::Server);
        }
        if ast::CreateServer::can_cast(ancestor.kind())
            || ast::DropForeignDataWrapper::can_cast(ancestor.kind())
            || ast::AlterForeignDataWrapper::can_cast(ancestor.kind())
        {
            return Some(NameRefClass::ForeignDataWrapper);
        }
        if ast::DropPublication::can_cast(ancestor.kind())
            || ast::AlterPublication::can_cast(ancestor.kind())
        {
            return Some(NameRefClass::Publication);
        }
        if ast::DropSubscription::can_cast(ancestor.kind())
            || ast::AlterSubscription::can_cast(ancestor.kind())
        {
            return Some(NameRefClass::Subscription);
        }
        if ast::DropLanguage::can_cast(ancestor.kind())
            || ast::AlterLanguage::can_cast(ancestor.kind())
        {
            return Some(NameRefClass::Language);
        }
        if ast::Collate::can_cast(ancestor.kind())
            || ast::DropCollation::can_cast(ancestor.kind())
            || ast::AlterCollation::can_cast(ancestor.kind())
        {
            return Some(NameRefClass::Collation);
        }
        if let Some(create_extension) = ast::CreateExtension::cast(ancestor.clone())
            && create_extension.schema_token().is_some()
        {
            return Some(NameRefClass::Schema);
        }
        if ast::DropExtension::can_cast(ancestor.kind())
            || ast::AlterExtension::can_cast(ancestor.kind())
            || ast::ObjectExtension::can_cast(ancestor.kind())
        {
            return Some(NameRefClass::Extension);
        }
        if ast::AlterRole::can_cast(ancestor.kind())
            || ast::DropRole::can_cast(ancestor.kind())
            || ast::SetRole::can_cast(ancestor.kind())
            || ast::RoleRef::can_cast(ancestor.kind())
            || ast::ObjectRole::can_cast(ancestor.kind())
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
            || ast::AlterTablespace::can_cast(ancestor.kind())
            || ast::Tablespace::can_cast(ancestor.kind())
            || ast::SetTablespace::can_cast(ancestor.kind())
            || ast::ConstraintIndexTablespace::can_cast(ancestor.kind())
            || ast::ObjectTablespace::can_cast(ancestor.kind())
        {
            return Some(NameRefClass::Tablespace);
        }
        if ast::SetNullColumns::can_cast(ancestor.kind()) {
            return Some(NameRefClass::ConstraintColumn);
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
                        .contains_range(node.text_range())
                {
                    return Some(NameRefClass::ForeignKeyColumn);
                }
                if let Some(from_columns) = foreign_key.from_columns()
                    && from_columns
                        .syntax()
                        .text_range()
                        .contains_range(node.text_range())
                {
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
                    .contains_range(node.text_range())
            {
                return Some(NameRefClass::ForeignKeyColumn);
            }
            if let Some(path) = references_constraint.table()
                && path.syntax().text_range().contains_range(node.text_range())
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
            if in_function_name {
                return Some(NameRefClass::FunctionCall);
            }
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
            if in_function_name {
                return Some(NameRefClass::FunctionCall);
            }
            return Some(NameRefClass::ConstraintColumn);
        }
        if ast::LikeClause::can_cast(ancestor.kind()) {
            return Some(NameRefClass::LikeTable);
        }
        if ast::CastExpr::can_cast(ancestor.kind()) && in_type {
            return Some(NameRefClass::Type);
        }
        if ast::DropFunction::can_cast(ancestor.kind())
            || ast::AlterFunction::can_cast(ancestor.kind())
        {
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
        if ast::DropSchema::can_cast(ancestor.kind())
            || ast::AlterSchema::can_cast(ancestor.kind())
            || ast::SetSchema::can_cast(ancestor.kind())
            || ast::ObjectSchema::can_cast(ancestor.kind())
        {
            return Some(NameRefClass::Schema);
        }
        if ast::CreateIndex::can_cast(ancestor.kind()) {
            if in_partition_item || in_constraint_include_clause || in_where_clause {
                if in_function_name {
                    return Some(NameRefClass::FunctionCall);
                }
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
            if in_function_name {
                return Some(NameRefClass::FunctionCall);
            }
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
        if ast::OverClause::can_cast(ancestor.kind()) && !in_function_name {
            if node
                .parent()
                .is_some_and(|parent| ast::OverClause::can_cast(parent.kind()))
            {
                return Some(NameRefClass::Window);
            }

            return Some(NameRefClass::SelectColumn);
        }
        if ast::FilterClause::can_cast(ancestor.kind()) {
            in_filter_clause = true;
        }
        if ast::CallExpr::can_cast(ancestor.kind()) {
            if !in_arg_list && !in_filter_clause {
                in_function_name = true;
            }
        }
        if ast::DefaultConstraint::can_cast(ancestor.kind()) && in_function_name {
            return Some(NameRefClass::FunctionCall);
        }
        if ast::ParamDefault::can_cast(ancestor.kind()) {
            if in_function_name {
                return Some(NameRefClass::FunctionCall);
            }
            return Some(NameRefClass::ParamDefault);
        }
        if ast::OnClause::can_cast(ancestor.kind()) {
            in_on_clause = true;
        }
        if ast::FromClause::can_cast(ancestor.kind()) {
            in_from_clause = true;
        }
        if ast::GroupByClause::can_cast(ancestor.kind()) {
            in_group_by_clause = true;
        }
        if ast::OrderByClause::can_cast(ancestor.kind()) {
            in_order_by_clause = true;
        }
        if ast::DistinctClause::can_cast(ancestor.kind()) {
            in_distinct_clause = true;
        }
        if ast::Select::can_cast(ancestor.kind()) {
            if in_function_name && !in_special_sql_fn {
                return Some(NameRefClass::SelectFunctionCall);
            }
            if in_from_clause && !in_on_clause {
                if in_arg_list {
                    return Some(NameRefClass::SelectColumn);
                }
                return Some(NameRefClass::FromTable);
            }
            if in_group_by_clause && is_grouping_or_distinct_el(node) {
                return Some(NameRefClass::SelectGroupByAliasOrColumn);
            }
            if in_distinct_clause && is_grouping_or_distinct_el(node) {
                return Some(NameRefClass::SelectOrderByAliasOrColumn);
            }
            if in_order_by_clause
                && let Some(parent) = node.parent()
                && ast::SortBy::can_cast(parent.kind())
            {
                return Some(NameRefClass::SelectOrderByAliasOrColumn);
            }
            // Classify as SelectColumn for target list, WHERE, etc.
            // (anything in SELECT except FROM clause)
            return Some(NameRefClass::SelectColumn);
        }
        if ast::CompoundSelect::can_cast(ancestor.kind())
            && in_order_by_clause
            && let Some(parent) = node.parent()
            && ast::SortBy::can_cast(parent.kind())
        {
            return Some(NameRefClass::SelectOrderByAliasOrColumn);
        }
        if ast::ColumnList::can_cast(ancestor.kind())
            || ast::ColumnRefList::can_cast(ancestor.kind())
        {
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
        if ast::ConflictIndexItem::can_cast(ancestor.kind()) {
            in_conflict_target = true;
        }
        if ast::ConflictOnConstraint::can_cast(ancestor.kind()) {
            return Some(NameRefClass::Constraint);
        }
        if ast::Insert::can_cast(ancestor.kind()) {
            if in_function_name && !in_special_sql_fn {
                return Some(NameRefClass::SelectFunctionCall);
            }
            if in_returning_clause
                || in_column_list
                || in_set_clause
                || in_where_clause
                || in_conflict_target
            {
                return Some(NameRefClass::InsertColumn);
            }
            return Some(NameRefClass::InsertTable);
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
        if ast::UsingClause::can_cast(ancestor.kind())
            || ast::UsingOnClause::can_cast(ancestor.kind())
        {
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

fn is_grouping_or_distinct_el(node: &SyntaxNode) -> bool {
    node.ancestors()
        .skip(1)
        .find(|n| !ast::ParenExpr::can_cast(n.kind()) && !ast::TupleExpr::can_cast(n.kind()))
        .is_some_and(|n| {
            ast::GroupBy::can_cast(n.kind()) || ast::DistinctClause::can_cast(n.kind())
        })
}

fn classify_privilege_object(privilege_objects: &ast::PrivilegeObjects) -> Option<NameRefClass> {
    match privilege_objects {
        ast::PrivilegeObjects::PrivilegeRoutine(routine) => {
            if routine.procedure_token().is_some() {
                return Some(NameRefClass::Procedure);
            }
            if routine.routine_token().is_some() {
                return Some(NameRefClass::Routine);
            }
            Some(NameRefClass::Function)
        }
        ast::PrivilegeObjects::PrivilegeType(_) => Some(NameRefClass::Type),
        ast::PrivilegeObjects::PrivilegeTable(table) => {
            if table.domain_token().is_some() {
                return Some(NameRefClass::Type);
            }
            if table.sequence_token().is_some() {
                return Some(NameRefClass::Sequence);
            }
            Some(NameRefClass::PrivilegeObjectTable)
        }
        ast::PrivilegeObjects::PrivilegeName(name) => {
            if name.schema_token().is_some() {
                return Some(NameRefClass::Schema);
            }
            if name.database_token().is_some() {
                return Some(NameRefClass::Database);
            }
            if name.tablespace_token().is_some() {
                return Some(NameRefClass::Tablespace);
            }
            // language
            None
        }
        ast::PrivilegeObjects::PrivilegeForeign(foreign) => {
            if foreign.server_token().is_some() {
                return Some(NameRefClass::Server);
            }
            // data wrapper
            None
        }
        ast::PrivilegeObjects::PrivilegePropertyGraph(_) => Some(NameRefClass::PropertyGraph),
        ast::PrivilegeObjects::PrivilegeAllInSchema(_) => Some(NameRefClass::Schema),
        ast::PrivilegeObjects::PrivilegeDefault(_) => Some(NameRefClass::PrivilegeObjectTable),
        ast::PrivilegeObjects::PrivilegeParameter(_)
        | ast::PrivilegeObjects::PrivilegeLargeObject(_) => None,
    }
}

pub(crate) fn classify_def_node(def_node: &SyntaxNode) -> Option<LocationKind> {
    let mut in_column = false;
    let mut in_column_list = false;
    for ancestor in def_node.ancestors() {
        if ast::Column::can_cast(ancestor.kind()) {
            in_column = true;
        }
        if ast::ColumnList::can_cast(ancestor.kind()) {
            in_column_list = true;
        }
        if ast::Param::can_cast(ancestor.kind()) {
            return Some(LocationKind::NamedArgParameter);
        }
        if ast::ConstraintName::can_cast(ancestor.kind())
            || ast::RenameConstraint::can_cast(ancestor.kind())
        {
            return Some(LocationKind::Constraint);
        }
        if ast::CreateTableLike::can_cast(ancestor.kind()) {
            if in_column {
                return Some(LocationKind::Column);
            }
            return Some(LocationKind::Table);
        }
        if ast::CreateType::can_cast(ancestor.kind()) {
            if in_column {
                return Some(LocationKind::Column);
            }
            return Some(LocationKind::Type);
        }
        if ast::CreateFunction::can_cast(ancestor.kind()) {
            return Some(LocationKind::Function);
        }
        if ast::CreateProcedure::can_cast(ancestor.kind()) {
            return Some(LocationKind::Procedure);
        }
        if ast::WithTable::can_cast(ancestor.kind()) {
            if in_column_list {
                return Some(LocationKind::Column);
            }
            return Some(LocationKind::Table);
        }
        if ast::CreateTableAs::can_cast(ancestor.kind()) {
            return Some(LocationKind::Table);
        }
        if ast::SelectInto::can_cast(ancestor.kind()) {
            return Some(LocationKind::Table);
        }
        if ast::CreateIndex::can_cast(ancestor.kind()) {
            return Some(LocationKind::Index);
        }
        if ast::CreateSequence::can_cast(ancestor.kind()) {
            return Some(LocationKind::Sequence);
        }
        if ast::CreateTrigger::can_cast(ancestor.kind()) {
            return Some(LocationKind::Trigger);
        }
        if ast::CreateEventTrigger::can_cast(ancestor.kind()) {
            return Some(LocationKind::EventTrigger);
        }
        if ast::CreateTablespace::can_cast(ancestor.kind()) {
            return Some(LocationKind::Tablespace);
        }
        if ast::CreateDatabase::can_cast(ancestor.kind()) {
            return Some(LocationKind::Database);
        }
        if ast::CreateServer::can_cast(ancestor.kind()) {
            return Some(LocationKind::Server);
        }
        if ast::CreateForeignDataWrapper::can_cast(ancestor.kind()) {
            return Some(LocationKind::ForeignDataWrapper);
        }
        if ast::CreatePublication::can_cast(ancestor.kind()) {
            return Some(LocationKind::Publication);
        }
        if ast::CreateSubscription::can_cast(ancestor.kind()) {
            return Some(LocationKind::Subscription);
        }
        if ast::CreateLanguage::can_cast(ancestor.kind()) {
            return Some(LocationKind::Language);
        }
        if ast::CreateCollation::can_cast(ancestor.kind()) {
            return Some(LocationKind::Collation);
        }
        if ast::CreateExtension::can_cast(ancestor.kind()) {
            return Some(LocationKind::Extension);
        }
        if ast::CreateRole::can_cast(ancestor.kind()) {
            return Some(LocationKind::Role);
        }
        if ast::CreateAggregate::can_cast(ancestor.kind()) {
            return Some(LocationKind::Aggregate);
        }
        if ast::CreateSchema::can_cast(ancestor.kind()) {
            return Some(LocationKind::Schema);
        }
        if ast::CreateView::can_cast(ancestor.kind())
            || ast::CreateMaterializedView::can_cast(ancestor.kind())
        {
            if in_column_list {
                return Some(LocationKind::Column);
            }
            return Some(LocationKind::View);
        }
        if ast::CreatePolicy::can_cast(ancestor.kind()) {
            return Some(LocationKind::Policy);
        }
        if ast::CreateRule::can_cast(ancestor.kind()) {
            return Some(LocationKind::Rule);
        }
        if ast::CreatePropertyGraph::can_cast(ancestor.kind()) {
            return Some(LocationKind::PropertyGraph);
        }
        if ast::Declare::can_cast(ancestor.kind()) {
            return Some(LocationKind::Cursor);
        }
        if ast::Prepare::can_cast(ancestor.kind()) {
            return Some(LocationKind::PreparedStatement);
        }
        if ast::Listen::can_cast(ancestor.kind()) {
            return Some(LocationKind::Channel);
        }
        if ast::Alias::can_cast(ancestor.kind()) {
            if in_column {
                return Some(LocationKind::Column);
            }
            return Some(LocationKind::Table);
        }
        if ast::WindowDef::can_cast(ancestor.kind()) {
            return Some(LocationKind::Window);
        }
        if ast::AsName::can_cast(ancestor.kind())
            || ast::ParenSelect::can_cast(ancestor.kind())
            || ast::Values::can_cast(ancestor.kind())
            || ast::Select::can_cast(ancestor.kind())
        {
            return Some(LocationKind::Column);
        }
    }
    None
}

#[test]
fn special_function() {
    for kind in (0..SyntaxKind::__LAST as u16)
        .map(SyntaxKind::from)
        .filter(|kind| format!("{kind:?}").ends_with("_FN"))
    {
        assert!(
            is_special_fn(kind),
            "unhandled special function kind: {kind:?}. Please update is_special_fn"
        )
    }
}
