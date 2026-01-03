use squawk_syntax::ast::{self, AstNode};

#[derive(Debug)]
pub(crate) enum NameRefClass {
    DropTable,
    Table,
    DropIndex,
    DropType,
    DropView,
    DropMaterializedView,
    DropSequence,
    ForeignKeyTable,
    ForeignKeyColumn,
    ForeignKeyLocalColumn,
    CheckConstraintColumn,
    GeneratedColumn,
    UniqueConstraintColumn,
    PrimaryKeyConstraintColumn,
    NotNullConstraintColumn,
    ExcludeConstraintColumn,
    PartitionByColumn,
    PartitionOfTable,
    LikeTable,
    InheritsTable,
    DropFunction,
    DropAggregate,
    DropProcedure,
    DropRoutine,
    CallProcedure,
    DropSchema,
    CreateSchema,
    CreateIndex,
    CreateIndexColumn,
    SelectFunctionCall,
    SelectFromTable,
    SelectColumn,
    SelectQualifiedColumnTable,
    SelectQualifiedColumn,
    CompositeTypeField,
    InsertTable,
    InsertColumn,
    DeleteTable,
    DeleteWhereColumn,
    UpdateTable,
    UpdateWhereColumn,
    UpdateSetColumn,
    UpdateFromTable,
    SchemaQualifier,
    TypeReference,
}

pub(crate) fn classify_name_ref(name_ref: &ast::NameRef) -> Option<NameRefClass> {
    let mut in_call_expr = false;
    let mut in_arg_list = false;
    let mut in_column_list = false;
    let mut in_where_clause = false;
    let mut in_from_clause = false;
    let mut in_set_clause = false;
    let mut in_constraint_exclusion_list = false;
    let mut in_constraint_include_clause = false;
    let mut in_constraint_where_clause = false;
    let mut in_partition_item = false;

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
        for ancestor in parent.ancestors() {
            if ast::FromClause::can_cast(ancestor.kind()) {
                in_from_clause = true;
            }
            if ast::Select::can_cast(ancestor.kind()) && !in_from_clause {
                if is_function_call || is_schema_table_col {
                    return Some(NameRefClass::SchemaQualifier);
                } else {
                    return Some(NameRefClass::SelectQualifiedColumnTable);
                }
            }
        }
        return Some(NameRefClass::SchemaQualifier);
    }

    if let Some(parent) = name_ref.syntax().parent()
        && let Some(field_expr) = ast::FieldExpr::cast(parent.clone())
        && field_expr
            .field()
            // we're at the field in a FieldExpr, i.e., foo.bar
            //                                              ^^^
            .is_some_and(|field_name_ref| field_name_ref.syntax() == name_ref.syntax())
            // we're not inside a call expr
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
        let mut in_cast_expr = false;
        for ancestor in parent.ancestors() {
            if ast::CastExpr::can_cast(ancestor.kind()) {
                in_cast_expr = true;
            }
            if ast::FromClause::can_cast(ancestor.kind()) {
                in_from_clause = true;
            }
            if ast::Select::can_cast(ancestor.kind()) && !in_from_clause {
                if in_cast_expr {
                    return Some(NameRefClass::TypeReference);
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
        return Some(NameRefClass::SchemaQualifier);
    }

    let mut in_type = false;
    let mut in_schema_authorization = false;
    for ancestor in name_ref.syntax().ancestors() {
        if ast::PathType::can_cast(ancestor.kind()) || ast::ExprType::can_cast(ancestor.kind()) {
            in_type = true;
        }
        if in_type {
            return Some(NameRefClass::TypeReference);
        }
        if ast::SchemaAuthorization::can_cast(ancestor.kind()) {
            in_schema_authorization = true;
        }
        if ast::DropTable::can_cast(ancestor.kind()) {
            return Some(NameRefClass::DropTable);
        }
        if ast::Table::can_cast(ancestor.kind()) {
            return Some(NameRefClass::Table);
        }
        if ast::DropIndex::can_cast(ancestor.kind()) {
            return Some(NameRefClass::DropIndex);
        }
        if ast::DropType::can_cast(ancestor.kind()) {
            return Some(NameRefClass::DropType);
        }
        if ast::DropView::can_cast(ancestor.kind()) {
            return Some(NameRefClass::DropView);
        }
        if ast::DropMaterializedView::can_cast(ancestor.kind()) {
            return Some(NameRefClass::DropMaterializedView);
        }
        if ast::DropSequence::can_cast(ancestor.kind()) {
            return Some(NameRefClass::DropSequence);
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
                    return Some(NameRefClass::ForeignKeyLocalColumn);
                }
            } else {
                return Some(NameRefClass::ForeignKeyTable);
            }
        }
        if ast::CheckConstraint::can_cast(ancestor.kind()) {
            return Some(NameRefClass::CheckConstraintColumn);
        }
        if ast::GeneratedConstraint::can_cast(ancestor.kind()) {
            return Some(NameRefClass::GeneratedColumn);
        }
        if in_column_list && ast::UniqueConstraint::can_cast(ancestor.kind()) {
            return Some(NameRefClass::UniqueConstraintColumn);
        }
        if in_column_list && ast::PrimaryKeyConstraint::can_cast(ancestor.kind()) {
            return Some(NameRefClass::PrimaryKeyConstraintColumn);
        }
        if ast::NotNullConstraint::can_cast(ancestor.kind()) {
            return Some(NameRefClass::NotNullConstraintColumn);
        }
        if (in_constraint_exclusion_list
            || in_constraint_include_clause
            || in_constraint_where_clause)
            && ast::ExcludeConstraint::can_cast(ancestor.kind())
        {
            return Some(NameRefClass::ExcludeConstraintColumn);
        }
        if ast::LikeClause::can_cast(ancestor.kind()) {
            return Some(NameRefClass::LikeTable);
        }
        if ast::Inherits::can_cast(ancestor.kind()) {
            return Some(NameRefClass::InheritsTable);
        }
        if ast::CastExpr::can_cast(ancestor.kind()) && in_type {
            return Some(NameRefClass::TypeReference);
        }
        if ast::DropFunction::can_cast(ancestor.kind()) {
            return Some(NameRefClass::DropFunction);
        }
        if ast::DropAggregate::can_cast(ancestor.kind()) {
            return Some(NameRefClass::DropAggregate);
        }
        if ast::DropProcedure::can_cast(ancestor.kind()) {
            return Some(NameRefClass::DropProcedure);
        }
        if ast::DropRoutine::can_cast(ancestor.kind()) {
            return Some(NameRefClass::DropRoutine);
        }
        if ast::Call::can_cast(ancestor.kind()) {
            return Some(NameRefClass::CallProcedure);
        }
        if ast::DropSchema::can_cast(ancestor.kind()) {
            return Some(NameRefClass::DropSchema);
        }
        if in_schema_authorization
            && let Some(create_schema) = ast::CreateSchema::cast(ancestor.clone())
            && create_schema.name().is_none()
        {
            return Some(NameRefClass::CreateSchema);
        }
        if ast::CreateIndex::can_cast(ancestor.kind()) {
            if in_partition_item {
                return Some(NameRefClass::CreateIndexColumn);
            }
            return Some(NameRefClass::CreateIndex);
        }
        if in_partition_item && ast::CreateTable::can_cast(ancestor.kind()) {
            return Some(NameRefClass::PartitionByColumn);
        }
        if ast::PartitionOf::can_cast(ancestor.kind()) {
            return Some(NameRefClass::PartitionOfTable);
        }
        if ast::ArgList::can_cast(ancestor.kind()) {
            in_arg_list = true;
        }
        if ast::CallExpr::can_cast(ancestor.kind()) {
            in_call_expr = true;
        }
        if ast::FromClause::can_cast(ancestor.kind()) {
            in_from_clause = true;
        }
        if ast::Select::can_cast(ancestor.kind()) {
            if in_call_expr && !in_arg_list {
                return Some(NameRefClass::SelectFunctionCall);
            }
            if in_from_clause {
                return Some(NameRefClass::SelectFromTable);
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
            if in_column_list {
                return Some(NameRefClass::InsertColumn);
            }
            return Some(NameRefClass::InsertTable);
        }
        if ast::WhereClause::can_cast(ancestor.kind()) {
            in_where_clause = true;
        }
        if ast::SetClause::can_cast(ancestor.kind()) {
            in_set_clause = true;
        }
        if ast::Delete::can_cast(ancestor.kind()) {
            if in_where_clause {
                return Some(NameRefClass::DeleteWhereColumn);
            }
            return Some(NameRefClass::DeleteTable);
        }
        if ast::Update::can_cast(ancestor.kind()) {
            if in_where_clause {
                return Some(NameRefClass::UpdateWhereColumn);
            }
            if in_set_clause {
                return Some(NameRefClass::UpdateSetColumn);
            }
            if in_from_clause {
                return Some(NameRefClass::UpdateFromTable);
            }
            return Some(NameRefClass::UpdateTable);
        }
    }

    None
}

#[derive(Debug)]
pub(crate) enum NameClass {
    ColumnDefinition {
        create_table: ast::CreateTable,
        column: ast::Column,
    },
    CreateTable(ast::CreateTable),
    WithTable(ast::WithTable),
    CreateIndex(ast::CreateIndex),
    CreateSequence(ast::CreateSequence),
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
        if let Some(create_table) = ast::CreateTable::cast(ancestor.clone()) {
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
    }

    if let Some(with_table) = with_table_parent {
        return Some(NameClass::WithTable(with_table));
    }

    None
}
