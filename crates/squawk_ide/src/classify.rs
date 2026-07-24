use crate::{location::LocationKind, name, symbols::Name};
use squawk_syntax::{
    SyntaxKind, SyntaxNode,
    ast::{self, AstNode},
};

#[derive(Debug, Clone, Copy)]
pub(crate) enum NameRefClass {
    AccessMethod,
    Aggregate,
    AlterColumn,
    Channel,
    Collation,
    CompositeTypeAttribute,
    CompositeTypeField,
    Constraint,
    ConstraintColumn,
    Conversion,
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
    Index,
    InsertColumn,
    InsertQualifiedColumnTable,
    InsertTable,
    JoinUsingColumn,
    JsonPath,
    Language,
    MergeColumn,
    MergeQualifiedColumnTable,
    NamedArgParameter,
    OperatorClass,
    OperatorFamily,
    ParamDefault,
    Policy,
    PolicyColumn,
    PolicyQualifiedColumnTable,
    PreparedStatement,
    PrivilegeColumn,
    PrivilegeObjectTable,
    Procedure,
    PropertyGraph,
    PropertyGraphColumn,
    Publication,
    PublicationColumn,
    QualifiedColumn,
    Relation,
    Role,
    Routine,
    Rule,
    RulePseudoColumn,
    RulePseudoColumnTable,
    Savepoint,
    Schema,
    SelectColumn,
    SelectFunctionCall,
    SelectGroupByAliasOrColumn,
    SelectOrderByAliasOrColumn,
    SelectQualifiedColumn,
    SelectQualifiedColumnTable,
    Sequence,
    Server,
    Statistics,
    StatisticsColumn,
    Subscription,
    Table,
    TableAndColumnsColumn,
    Tablespace,
    TextSearchConfiguration,
    TextSearchDictionary,
    TextSearchParser,
    TextSearchTemplate,
    Trigger,
    TriggerEventColumn,
    TriggerWhenColumn,
    TriggerWhenColumnTable,
    Type,
    UpdateColumn,
    UpdateQualifiedColumnTable,
    VertexTable,
    View,
    Window,
}

fn classify_object_definition(kind: SyntaxKind) -> Option<LocationKind> {
    Some(match kind {
        SyntaxKind::ACCESS_METHOD => LocationKind::AccessMethod,
        SyntaxKind::CHANNEL => LocationKind::Channel,
        SyntaxKind::COLLATION => LocationKind::Collation,
        SyntaxKind::CONSTRAINT_NAME => LocationKind::Constraint,
        SyntaxKind::CONVERSION => LocationKind::Conversion,
        SyntaxKind::CTE_NAME => LocationKind::Table,
        SyntaxKind::CURSOR => LocationKind::Cursor,
        SyntaxKind::DATABASE => LocationKind::Database,
        SyntaxKind::DOMAIN => LocationKind::Type,
        SyntaxKind::EVENT_TRIGGER => LocationKind::EventTrigger,
        SyntaxKind::EXTENSION => LocationKind::Extension,
        SyntaxKind::FOREIGN_DATA_WRAPPER => LocationKind::ForeignDataWrapper,
        SyntaxKind::FUNCTION_NAME => LocationKind::Function,
        SyntaxKind::INDEX => LocationKind::Index,
        SyntaxKind::JSON_PATH_NAME => LocationKind::JsonPath,
        SyntaxKind::LANGUAGE => LocationKind::Language,
        SyntaxKind::OP_CLASS_NAME => LocationKind::OperatorClass,
        SyntaxKind::OP_FAMILY_NAME => LocationKind::OperatorFamily,
        SyntaxKind::POLICY => LocationKind::Policy,
        SyntaxKind::PREPARED_STATEMENT => LocationKind::PreparedStatement,
        SyntaxKind::PROCEDURE_NAME => LocationKind::Procedure,
        SyntaxKind::PROPERTY_GRAPH => LocationKind::PropertyGraph,
        SyntaxKind::PUBLICATION => LocationKind::Publication,
        SyntaxKind::RULE => LocationKind::Rule,
        SyntaxKind::SAVEPOINT => LocationKind::Savepoint,
        SyntaxKind::SCHEMA => LocationKind::Schema,
        SyntaxKind::SEQUENCE => LocationKind::Sequence,
        SyntaxKind::SERVER => LocationKind::Server,
        SyntaxKind::STATISTICS => LocationKind::Statistics,
        SyntaxKind::SUBSCRIPTION => LocationKind::Subscription,
        SyntaxKind::TABLE_ALIAS | SyntaxKind::TABLE_NAME => LocationKind::Table,
        SyntaxKind::TABLESPACE => LocationKind::Tablespace,
        SyntaxKind::TEXT_SEARCH_CONFIGURATION => LocationKind::TextSearchConfiguration,
        SyntaxKind::TEXT_SEARCH_DICTIONARY => LocationKind::TextSearchDictionary,
        SyntaxKind::TEXT_SEARCH_PARSER => LocationKind::TextSearchParser,
        SyntaxKind::TEXT_SEARCH_TEMPLATE => LocationKind::TextSearchTemplate,
        SyntaxKind::TRIGGER => LocationKind::Trigger,
        SyntaxKind::TYPE_NAME => LocationKind::Type,
        SyntaxKind::VIEW => LocationKind::View,
        SyntaxKind::WINDOW => LocationKind::Window,
        _ => return None,
    })
}

fn classify_object_ref(kind: SyntaxKind) -> Option<NameRefClass> {
    Some(match kind {
        SyntaxKind::ACCESS_METHOD_REF => NameRefClass::AccessMethod,
        SyntaxKind::CHANNEL_REF => NameRefClass::Channel,
        SyntaxKind::COLLATION_REF => NameRefClass::Collation,
        SyntaxKind::CONSTRAINT_NAME_REF => NameRefClass::Constraint,
        SyntaxKind::CONVERSION_REF => NameRefClass::Conversion,
        SyntaxKind::CURSOR_REF => NameRefClass::Cursor,
        SyntaxKind::DATABASE_REF => NameRefClass::Database,
        SyntaxKind::DOMAIN_REF => NameRefClass::Type,
        SyntaxKind::EVENT_TRIGGER_REF => NameRefClass::EventTrigger,
        SyntaxKind::EXTENSION_REF => NameRefClass::Extension,
        SyntaxKind::FOREIGN_DATA_WRAPPER_REF => NameRefClass::ForeignDataWrapper,
        SyntaxKind::FUNCTION_NAME_REF => NameRefClass::Function,
        SyntaxKind::INDEX_REF => NameRefClass::Index,
        SyntaxKind::JSON_PATH_NAME_REF => NameRefClass::JsonPath,
        SyntaxKind::LANGUAGE_REF => NameRefClass::Language,
        SyntaxKind::OP_CLASS_REF => NameRefClass::OperatorClass,
        SyntaxKind::OP_FAMILY_REF => NameRefClass::OperatorFamily,
        SyntaxKind::PARAM_NAME_REF => NameRefClass::NamedArgParameter,
        SyntaxKind::POLICY_REF => NameRefClass::Policy,
        SyntaxKind::PREPARED_STATEMENT_REF => NameRefClass::PreparedStatement,
        SyntaxKind::PROCEDURE_NAME_REF => NameRefClass::Procedure,
        SyntaxKind::PROPERTY_GRAPH_REF => NameRefClass::PropertyGraph,
        SyntaxKind::PUBLICATION_REF => NameRefClass::Publication,
        SyntaxKind::RELATION_NAME_REF => NameRefClass::Relation,
        SyntaxKind::ROLE_REF => NameRefClass::Role,
        SyntaxKind::ROUTINE_NAME_REF => NameRefClass::Routine,
        SyntaxKind::RULE_REF => NameRefClass::Rule,
        SyntaxKind::SAVEPOINT_REF => NameRefClass::Savepoint,
        SyntaxKind::SCHEMA_REF => NameRefClass::Schema,
        SyntaxKind::SEQUENCE_REF => NameRefClass::Sequence,
        SyntaxKind::SERVER_REF => NameRefClass::Server,
        SyntaxKind::STATISTICS_REF => NameRefClass::Statistics,
        SyntaxKind::SUBSCRIPTION_REF => NameRefClass::Subscription,
        SyntaxKind::TABLESPACE_REF => NameRefClass::Tablespace,
        SyntaxKind::TEXT_SEARCH_CONFIGURATION_REF => NameRefClass::TextSearchConfiguration,
        SyntaxKind::TEXT_SEARCH_DICTIONARY_REF => NameRefClass::TextSearchDictionary,
        SyntaxKind::TEXT_SEARCH_PARSER_REF => NameRefClass::TextSearchParser,
        SyntaxKind::TEXT_SEARCH_TEMPLATE_REF => NameRefClass::TextSearchTemplate,
        SyntaxKind::TRIGGER_REF => NameRefClass::Trigger,
        SyntaxKind::TYPE_NAME_REF => NameRefClass::Type,
        SyntaxKind::VERTEX_TABLE_REF => NameRefClass::VertexTable,
        SyntaxKind::VIEW_REF => NameRefClass::View,
        SyntaxKind::WINDOW_REF => NameRefClass::Window,
        _ => return None,
    })
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

fn is_text_search_parser_function_option(option_name: &Name) -> bool {
    matches!(
        option_name.0.as_str(),
        "end" | "gettoken" | "headline" | "lextypes" | "start"
    )
}

fn is_text_search_template_function_option(option_name: &Name) -> bool {
    matches!(option_name.0.as_str(), "init" | "lexize")
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

    if (ast::CreateOperator::can_cast(ddl_node.kind())
        && is_create_operator_function_option(&attr_name))
        || (ast::CreateAggregate::can_cast(ddl_node.kind())
            && is_create_aggregate_function_option(&attr_name))
        || (ast::CreateType::can_cast(ddl_node.kind())
            && is_create_type_function_option(&attr_name))
        || (ast::CreateTextSearchParser::can_cast(ddl_node.kind())
            && is_text_search_parser_function_option(&attr_name))
        || (ast::CreateTextSearchTemplate::can_cast(ddl_node.kind())
            && is_text_search_template_function_option(&attr_name))
    {
        return Some(NameRefClass::Function);
    }
    if ast::CreateTextSearchConfiguration::can_cast(ddl_node.kind()) {
        if attr_name.0.as_str() == "parser" {
            return Some(NameRefClass::TextSearchParser);
        }
        if attr_name.0.as_str() == "copy" {
            return Some(NameRefClass::TextSearchConfiguration);
        }
    }
    if ast::CreateTextSearchDictionary::can_cast(ddl_node.kind())
        && attr_name.0.as_str() == "template"
    {
        return Some(NameRefClass::TextSearchTemplate);
    }
    None
}

fn is_search_path(config_parameter: Option<ast::ConfigParameterRef>) -> bool {
    let Some(path) = config_parameter.and_then(|config_parameter| config_parameter.path_ref())
    else {
        return false;
    };
    if path.qualifier().is_some() {
        return false;
    }
    path.segment()
        .and_then(|segment| segment.name_ref())
        .is_some_and(|name_ref| Name::from_node(&name_ref).0.as_str() == "search_path")
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

fn classify_call_expr_name_ref(
    node: &SyntaxNode,
    call_expr: &ast::CallExpr,
) -> Option<NameRefClass> {
    if !call_expr
        .expr()?
        .syntax()
        .text_range()
        .contains_range(node.text_range())
    {
        return None;
    }

    for ancestor in call_expr.syntax().ancestors() {
        if let Some(create_trigger) = ast::CreateTrigger::cast(ancestor.clone())
            && create_trigger
                .call_expr()
                .is_some_and(|execute_call| execute_call.syntax() == call_expr.syntax())
        {
            return Some(if create_trigger.procedure_token().is_some() {
                NameRefClass::Procedure
            } else {
                NameRefClass::Function
            });
        }
        if let Some(create_event_trigger) = ast::CreateEventTrigger::cast(ancestor.clone())
            && create_event_trigger
                .call_expr()
                .is_some_and(|execute_call| execute_call.syntax() == call_expr.syntax())
        {
            return Some(if create_event_trigger.procedure_token().is_some() {
                NameRefClass::Procedure
            } else {
                NameRefClass::Function
            });
        }
        if ast::Select::can_cast(ancestor.kind())
            || ast::SelectInto::can_cast(ancestor.kind())
            || ast::Insert::can_cast(ancestor.kind())
            || ast::ReturnFuncOption::can_cast(ancestor.kind())
        {
            return Some(NameRefClass::SelectFunctionCall);
        }
        if ast::Delete::can_cast(ancestor.kind())
            || ast::Merge::can_cast(ancestor.kind())
            || ast::Update::can_cast(ancestor.kind())
        {
            return Some(NameRefClass::Function);
        }
    }

    Some(NameRefClass::Function)
}

fn classify_object_column_path(node: &SyntaxNode) -> Option<NameRefClass> {
    let object_column = node.ancestors().find_map(ast::ObjectColumn::cast)?;
    let mut path = object_column.name()?.path_ref()?;
    let mut name_refs = Vec::new();

    loop {
        if let Some(name_ref) = path.segment().and_then(|segment| segment.name_ref()) {
            name_refs.push(name_ref);
        }
        let Some(qualifier) = path.qualifier() else {
            break;
        };
        path = qualifier;
    }

    name_refs.reverse();
    let idx = name_refs
        .iter()
        .position(|name_ref| name_ref.syntax() == node)?;
    let last_idx = name_refs.len().checked_sub(1)?;

    if idx == last_idx {
        Some(NameRefClass::QualifiedColumn)
    } else if idx + 1 == last_idx {
        Some(NameRefClass::Table)
    } else {
        Some(NameRefClass::Schema)
    }
}

pub(crate) fn classify_config_value_name(node: &SyntaxNode) -> Option<NameRefClass> {
    let parent = node.parent()?;
    let config_parameter = if let Some(set_config) = ast::SetConfig::cast(parent.clone()) {
        set_config.config_parameter_ref()
    } else if let Some(set_config_param) = ast::SetConfigParam::cast(parent) {
        set_config_param.config_parameter_ref()
    } else {
        return None;
    };
    if is_search_path(config_parameter) {
        Some(NameRefClass::Schema)
    } else {
        None
    }
}

pub(crate) fn classify_literal(node: &SyntaxNode) -> Option<NameRefClass> {
    let parent = node.parent()?;
    if ast::SetSchemaValue::can_cast(parent.kind()) {
        return Some(NameRefClass::Schema);
    }
    if let Some(set_config) = ast::SetConfig::cast(parent.clone())
        && is_search_path(set_config.config_parameter_ref())
        && set_config
            .config_values()
            .any(|config_value| config_value.syntax() == node)
    {
        return Some(NameRefClass::Schema);
    }
    if let Some(set_config_param) = ast::SetConfigParam::cast(parent)
        && is_search_path(set_config_param.config_parameter_ref())
        && set_config_param
            .literals()
            .any(|literal| literal.syntax() == node)
    {
        return Some(NameRefClass::Schema);
    }
    None
}

pub(crate) fn classify_name_ref(node: &SyntaxNode) -> Option<NameRefClass> {
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
    let mut in_conflict_target = false;
    let mut in_group_by_clause = false;
    let mut in_order_by_clause = false;
    let mut in_distinct_clause = false;
    let mut in_table_valued_column_list = false;
    let mut has_table_name_ref = false;

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
            if (ast::Select::can_cast(ancestor.kind())
                || ast::SelectInto::can_cast(ancestor.kind()))
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

        // i.e., `(expr).field`
        if !is_base_of_outer_field_expr && let Some(ast::Expr::ParenExpr(_)) = field_expr.base() {
            return Some(NameRefClass::CompositeTypeField);
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
            if (ast::Select::can_cast(ancestor.kind())
                || ast::SelectInto::can_cast(ancestor.kind()))
                && (!in_from_clause || in_on_clause)
            {
                if in_cast_expr {
                    return Some(NameRefClass::Type);
                }
                if is_base_of_outer_field_expr {
                    return Some(NameRefClass::SelectQualifiedColumnTable);
                } else if let Some(base) = field_expr.base()
                    && matches!(base, ast::Expr::NameRef(_) | ast::Expr::FieldExpr(_))
                {
                    return Some(NameRefClass::SelectQualifiedColumn);
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
            if ast::ReturnFuncOption::can_cast(ancestor.kind()) {
                if let Some(ast::Expr::NameRef(base)) = field_expr.base()
                    && enclosing_routine_name(&ancestor)
                        .is_some_and(|routine_name| Name::from_node(&base) == routine_name)
                {
                    return Some(NameRefClass::SelectColumn);
                }
                return None;
            }
        }
    }

    if let Some(class) = classify_object_column_path(node) {
        return Some(class);
    }

    // %type clause paths (max 3 segments):
    //   column%type, table.column%type, schema.table.column%type
    if let Some(parent) = node.parent()
        && let Some(mut path) = ast::PathSegmentRef::cast(parent)
            .and_then(|p| p.syntax().parent().and_then(ast::PathRef::cast))
    {
        let mut hops_up = 0;
        while let Some(next) = path.syntax().parent().and_then(ast::PathRef::cast) {
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
        if path
            .syntax()
            .parent()
            .is_some_and(|parent| ast::QualifiedColumnNameRef::can_cast(parent.kind()))
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
        && let Some(inner_path) = ast::PathSegmentRef::cast(parent)
            .and_then(|p| p.syntax().parent().and_then(ast::PathRef::cast))
        && let Some(outer_path) = inner_path.syntax().parent().and_then(|p| {
            ast::PathRef::cast(p.clone())
                .and_then(|p| p.qualifier())
                .or_else(|| ast::Path::cast(p).and_then(|p| p.qualifier()))
        })
        && outer_path.syntax() == inner_path.syntax()
    {
        return Some(NameRefClass::Schema);
    }

    if let Some(parent) = node.parent()
        && let Some(expr_as_property_name) = ast::ExprAsPropertyName::cast(parent)
        && let Some(expr_as_property_name_list) =
            ast::ExprAsPropertyNameList::cast(expr_as_property_name.syntax().parent()?)
        && ast::Properties::cast(expr_as_property_name_list.syntax().parent()?).is_some()
    {
        return Some(NameRefClass::PropertyGraphColumn);
    }

    let mut in_type = false;
    for ancestor in node.ancestors() {
        if ast::TableNameRef::can_cast(ancestor.kind()) {
            has_table_name_ref = true;
        } else if let Some(class) = classify_object_ref(ancestor.kind()) {
            return Some(class);
        }
        if ast::CopyTable::can_cast(ancestor.kind()) {
            return Some(if has_table_name_ref {
                NameRefClass::Table
            } else {
                NameRefClass::CopyColumn
            });
        }
        if ast::OnTable::can_cast(ancestor.kind()) {
            return Some(NameRefClass::Table);
        }
        if ast::TableAndColumns::can_cast(ancestor.kind()) {
            return Some(if has_table_name_ref {
                NameRefClass::Table
            } else {
                NameRefClass::TableAndColumnsColumn
            });
        }
        if let Some(call_expr) = ast::CallExpr::cast(ancestor.clone())
            && let Some(class) = classify_call_expr_name_ref(node, &call_expr)
        {
            return Some(class);
        }
        if let Some(privilege_objects) = ast::PrivilegeObjects::cast(ancestor.clone()) {
            return classify_privilege_object(&privilege_objects);
        }
        if in_column_list
            && (ast::Grant::can_cast(ancestor.kind()) || ast::Revoke::can_cast(ancestor.kind()))
        {
            return Some(NameRefClass::PrivilegeColumn);
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
        if in_column_list
            && (ast::VertexTableDef::can_cast(ancestor.kind())
                || ast::EdgeTableDef::can_cast(ancestor.kind())
                || ast::SourceVertexTable::can_cast(ancestor.kind())
                || ast::DestVertexTable::can_cast(ancestor.kind()))
        {
            return Some(NameRefClass::PropertyGraphColumn);
        }
        if ast::StatTypes::can_cast(ancestor.kind()) {
            return None;
        }
        if ast::TriggerEventUpdate::can_cast(ancestor.kind()) {
            return Some(NameRefClass::TriggerEventColumn);
        }
        if ast::CreateStatistics::can_cast(ancestor.kind()) {
            return Some(NameRefClass::StatisticsColumn);
        }
        if let Some(publication_object) = ast::PublicationObject::cast(ancestor.clone())
            && publication_object.table_token().is_some()
        {
            return Some(if has_table_name_ref {
                NameRefClass::Table
            } else {
                NameRefClass::PublicationColumn
            });
        }
        if ast::FromTable::can_cast(ancestor.kind()) {
            return Some(NameRefClass::Table);
        }
        if ast::AlterColumn::can_cast(ancestor.kind())
            || ast::AlterViewColumn::can_cast(ancestor.kind())
            || ast::DropColumn::can_cast(ancestor.kind())
            || ast::RenameColumn::can_cast(ancestor.kind())
        {
            return Some(NameRefClass::AlterColumn);
        }
        if ast::RenameAttribute::can_cast(ancestor.kind())
            || ast::DropAttribute::can_cast(ancestor.kind())
            || ast::AlterAttribute::can_cast(ancestor.kind())
        {
            return Some(NameRefClass::CompositeTypeAttribute);
        }
        if ast::ObjectAggregate::can_cast(ancestor.kind()) {
            return Some(NameRefClass::Aggregate);
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
        if ast::ReferencesConstraint::can_cast(ancestor.kind()) {
            return Some(if has_table_name_ref {
                NameRefClass::ForeignKeyTable
            } else {
                NameRefClass::ForeignKeyColumn
            });
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
        if ast::DropAggregate::can_cast(ancestor.kind())
            || ast::AlterAggregate::can_cast(ancestor.kind())
        {
            return Some(NameRefClass::Aggregate);
        }
        if ast::CreateIndex::can_cast(ancestor.kind()) {
            return Some(if has_table_name_ref {
                NameRefClass::Table
            } else {
                NameRefClass::CreateIndexColumn
            });
        }
        if in_partition_item && ast::CreateTableLike::can_cast(ancestor.kind()) {
            return Some(NameRefClass::ConstraintColumn);
        }
        if ast::NamedArg::can_cast(ancestor.kind()) {
            return Some(NameRefClass::NamedArgParameter);
        }
        if ast::ArgList::can_cast(ancestor.kind()) {
            in_arg_list = true;
        }
        if ast::XmlTableColumnList::can_cast(ancestor.kind())
            || ast::JsonTableColumnList::can_cast(ancestor.kind())
        {
            in_table_valued_column_list = true;
        }
        if (ast::XmlTable::can_cast(ancestor.kind()) || ast::JsonTable::can_cast(ancestor.kind()))
            && !in_table_valued_column_list
        {
            in_arg_list = true;
        }
        if ast::OverClause::can_cast(ancestor.kind()) {
            return Some(NameRefClass::SelectColumn);
        }
        if ast::ParamDefault::can_cast(ancestor.kind()) {
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
        if ast::Select::can_cast(ancestor.kind()) || ast::SelectInto::can_cast(ancestor.kind()) {
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
        if ast::Insert::can_cast(ancestor.kind()) {
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
            break;
        }
        if ast::Update::can_cast(ancestor.kind()) {
            if in_returning_clause || in_where_clause || in_set_clause {
                return Some(NameRefClass::UpdateColumn);
            }
            if in_from_clause {
                return Some(NameRefClass::FromTable);
            }
            break;
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
            break;
        }
        // SQL-body function: `create function f(x int) returns int language sql return x + 1;`
        if ast::ReturnFuncOption::can_cast(ancestor.kind()) {
            return Some(NameRefClass::SelectColumn);
        }
    }

    has_table_name_ref.then_some(NameRefClass::Table)
}

fn enclosing_routine_name(node: &SyntaxNode) -> Option<Name> {
    for ancestor in node.ancestors() {
        if let Some(create_function) = ast::CreateFunction::cast(ancestor.clone()) {
            let (_, routine_name) =
                name::schema_and_name_definition(&create_function.name()?.path()?)?;
            return Some(routine_name);
        }
        if let Some(create_procedure) = ast::CreateProcedure::cast(ancestor) {
            let (_, routine_name) =
                name::schema_and_name_definition(&create_procedure.name()?.path()?)?;
            return Some(routine_name);
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
        ast::PrivilegeObjects::PrivilegeDefault(_) | ast::PrivilegeObjects::PrivilegeTable(_) => {
            Some(NameRefClass::PrivilegeObjectTable)
        }
        _ => None,
    }
}

pub(crate) fn classify_def_node(def_node: &SyntaxNode) -> Option<LocationKind> {
    let mut in_column = false;
    let mut in_column_list = false;
    for ancestor in def_node.ancestors() {
        if let Some(class) = classify_object_definition(ancestor.kind()) {
            return Some(class);
        }
        if ast::Column::can_cast(ancestor.kind()) {
            in_column = true;
        }
        if ast::ColumnList::can_cast(ancestor.kind()) {
            in_column_list = true;
        }
        if ast::Param::can_cast(ancestor.kind()) {
            return Some(LocationKind::NamedArgParameter);
        }
        if in_column
            && (ast::CreateTableLike::can_cast(ancestor.kind())
                || ast::CreateType::can_cast(ancestor.kind()))
        {
            return Some(LocationKind::Column);
        }
        if ast::WithTable::can_cast(ancestor.kind()) {
            if in_column_list {
                return Some(LocationKind::Column);
            }
            return Some(LocationKind::Table);
        }
        if ast::CreateRole::can_cast(ancestor.kind())
            || ast::CreateUser::can_cast(ancestor.kind())
            || ast::CreateGroup::can_cast(ancestor.kind())
        {
            return Some(LocationKind::Role);
        }
        if ast::CreateAggregate::can_cast(ancestor.kind()) {
            return Some(LocationKind::Aggregate);
        }
        if ast::CreateSchema::can_cast(ancestor.kind()) {
            return Some(LocationKind::Schema);
        }
        if in_column_list
            && (ast::CreateView::can_cast(ancestor.kind())
                || ast::CreateMaterializedView::can_cast(ancestor.kind()))
        {
            return Some(LocationKind::Column);
        }
        if ast::FromAlias::can_cast(ancestor.kind())
            || ast::OptionalAsAlias::can_cast(ancestor.kind())
            || ast::RequiredAsAlias::can_cast(ancestor.kind())
        {
            if in_column {
                return Some(LocationKind::Column);
            }
            return Some(LocationKind::Table);
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
