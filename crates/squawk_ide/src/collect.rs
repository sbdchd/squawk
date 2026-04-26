use crate::ast_nav;
use crate::builtins::builtins_file;
use crate::column_name::ColumnName;
use crate::db::{File, parse};
use crate::infer::{Type, infer_type_from_expr, infer_type_from_ty};
use crate::location::LocationKind;
use crate::name::{self, Name};
use crate::resolve::{
    ResolvedTableName, find_from_item_in_from_clause, qualified_star_table_name,
    resolve_table_like, resolve_table_name, table_ptr_from_from_item,
};
use salsa::Database as Db;
use squawk_syntax::{
    SyntaxNodePtr,
    ast::{self, AstNode},
};

pub(crate) fn columns_from_create_table(
    db: &dyn Db,
    file: File,
    create_table: &ast::CreateTableLike,
) -> Vec<(Name, Option<SyntaxNodePtr>)> {
    let mut columns = vec![];
    columns_from_create_table_impl(db, file, create_table, &mut columns, 0);
    columns
}

fn columns_from_create_table_impl(
    db: &dyn Db,
    file: File,
    create_table: &ast::CreateTableLike,
    columns: &mut Vec<(Name, Option<SyntaxNodePtr>)>,
    depth: usize,
) {
    if depth > 40 {
        log::info!("max depth reached, probably in a cycle");
        return;
    }

    if let Some(inherits) = create_table.inherits() {
        for path in inherits.paths() {
            if let Some((schema, table_name)) = name::schema_and_name_path(&path) {
                let position = path.syntax().text_range().start();
                if let Some(ResolvedTableName::Table(parent_table)) =
                    resolve_table_name(db, file, &table_name, &schema, position)
                {
                    columns_from_create_table_impl(db, file, &parent_table, columns, depth + 1);
                }
            }
        }
    }

    if let Some(arg_list) = create_table.table_arg_list() {
        for arg in arg_list.args() {
            match &arg {
                ast::TableArg::Column(column) => {
                    if let Some(name) = column.name() {
                        let col_name = Name::from_node(&name);
                        columns.push((col_name, Some(SyntaxNodePtr::new(name.syntax()))));
                    }
                }
                ast::TableArg::LikeClause(like_clause) => {
                    if let Some(path) = like_clause.path()
                        && let Some((schema, table_name)) = name::schema_and_name_path(&path)
                    {
                        let position = path.syntax().text_range().start();
                        if let Some(ResolvedTableName::Table(source_table)) =
                            resolve_table_name(db, file, &table_name, &schema, position)
                        {
                            columns_from_create_table_impl(
                                db,
                                file,
                                &source_table,
                                columns,
                                depth + 1,
                            );
                        }
                    }
                }
                ast::TableArg::TableConstraint(_) => (),
            }
        }
    }
}

pub(crate) fn table_columns(
    db: &dyn Db,
    file: File,
    create_table: &impl ast::HasCreateTable,
) -> Vec<(Name, Option<Type>)> {
    table_columns_impl(db, file, create_table, 0)
}

// TODO: combine with find_column_in_create_table_impl
fn table_columns_impl(
    db: &dyn Db,
    file: File,
    create_table: &impl ast::HasCreateTable,
    depth: usize,
) -> Vec<(Name, Option<Type>)> {
    if depth > 40 {
        log::info!("max depth reached, probably in a cycle");
        return vec![];
    }

    let mut columns = vec![];

    if let Some(inherits) = create_table.inherits() {
        for path in inherits.paths() {
            if let Some((schema, table_name)) = name::schema_and_name_path(&path) {
                let position = path.syntax().text_range().start();
                if let Some(ResolvedTableName::Table(parent_table)) =
                    resolve_table_name(db, file, &table_name, &schema, position)
                {
                    let inherited_columns = table_columns_impl(db, file, &parent_table, depth + 1);
                    columns.extend(inherited_columns);
                }
            }
        }
    }

    if let Some(arg_list) = create_table.table_arg_list() {
        for arg in arg_list.args() {
            match &arg {
                ast::TableArg::Column(column) => {
                    if let Some(name) = column.name() {
                        let ty = column.ty().and_then(|ty| infer_type_from_ty(&ty));
                        columns.push((Name::from_node(&name), ty));
                    }
                }
                ast::TableArg::LikeClause(like_clause) => {
                    if let Some(path) = like_clause.path()
                        && let Some((schema, table_name)) = name::schema_and_name_path(&path)
                    {
                        let position = path.syntax().text_range().start();
                        if let Some(ResolvedTableName::Table(source_table)) =
                            resolve_table_name(db, file, &table_name, &schema, position)
                        {
                            let like_columns =
                                table_columns_impl(db, file, &source_table, depth + 1);
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

pub(crate) fn create_table_as_columns_with_types(
    create_table_as: &ast::CreateTableAs,
) -> Vec<(Name, Option<Type>)> {
    create_table_as
        .query()
        .and_then(ast_nav::select_from_variant)
        .and_then(|x| x.select_clause())
        .and_then(|x| x.target_list())
        .map(|x| target_list_columns_with_types(&x))
        .unwrap_or_default()
}

fn columns_from_returning_clause_with_types(
    query: &ast::WithQuery,
) -> Option<Vec<(Name, Option<Type>)>> {
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
            return Some(target_list_columns_with_types(&target_list));
        }
        return Some(vec![]);
    }

    None
}

pub(crate) fn view_like_columns_with_types(
    create_view: &ast::CreateViewLike,
) -> Vec<(Name, Option<Type>)> {
    let alias_columns: Vec<Name> = create_view
        .column_list()
        .into_iter()
        .flat_map(|column_list| column_list.columns())
        .filter_map(|column| column.name().map(|name| Name::from_node(&name)))
        .collect();

    let Some(select) = create_view.query().and_then(ast_nav::select_from_variant) else {
        return vec![];
    };
    let Some(select_clause) = select.select_clause() else {
        return vec![];
    };
    let Some(target_list) = select_clause.target_list() else {
        return vec![];
    };

    let base_columns = target_list_columns_with_types(&target_list);

    if alias_columns.is_empty() {
        return base_columns;
    }

    let mut results = vec![];

    for (idx, alias_name) in alias_columns.iter().enumerate() {
        results.push((
            alias_name.clone(),
            base_columns.get(idx).and_then(|(_, ty)| ty.clone()),
        ));
    }

    results.extend(base_columns.into_iter().skip(alias_columns.len()));

    results
}

fn with_table_column_names(db: &dyn Db, file: File, with_table: ast::WithTable) -> Vec<Name> {
    with_table_columns_with_types(db, file, with_table)
        .into_iter()
        .map(|(name, _)| name)
        .collect()
}

pub(crate) fn with_table_columns_with_types(
    db: &dyn Db,
    file: File,
    with_table: ast::WithTable,
) -> Vec<(Name, Option<Type>)> {
    let alias_columns: Vec<Name> = with_table
        .column_list()
        .into_iter()
        .flat_map(|column_list| column_list.columns())
        .filter_map(|column| column.name().map(|name| Name::from_node(&name)))
        .collect();

    let base_columns = with_table_query_columns_with_types(db, file, with_table);

    if alias_columns.is_empty() {
        return base_columns;
    }

    let mut results = vec![];

    for (idx, alias_name) in alias_columns.iter().enumerate() {
        results.push((
            alias_name.clone(),
            base_columns.get(idx).and_then(|(_, ty)| ty.clone()),
        ));
    }

    results.extend(base_columns.into_iter().skip(alias_columns.len()));

    results
}

fn with_table_query_columns_with_types(
    db: &dyn Db,
    file: File,
    with_table: ast::WithTable,
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

    if let Some(columns) = columns_from_returning_clause_with_types(&query) {
        return columns;
    }

    let Some(cte_select) = ast_nav::select_from_with_query(query) else {
        return vec![];
    };
    let Some(target_list) = cte_select.select_clause().and_then(|x| x.target_list()) else {
        return vec![];
    };

    let from_clause = cte_select.from_clause();
    let mut columns = vec![];

    for target in target_list.targets() {
        if let Some((col_name, _node)) = ColumnName::from_target(target.clone()) {
            if let Some(col_name_str) = col_name.to_string() {
                let ty = target.expr().and_then(|e| infer_type_from_expr(&e));
                columns.push((Name::from_string(col_name_str), ty));
                continue;
            }

            if target.star_token().is_some()
                && let Some(from_clause) = &from_clause
            {
                columns.extend(columns_for_star_from_clause(db, file, from_clause));
                continue;
            }
        }

        if let Some(expr) = target.expr()
            && let ast::Expr::FieldExpr(field_expr) = expr
            && let Some(table_name) = qualified_star_table_name(&field_expr)
            && let Some(from_clause) = &from_clause
            && let Some(from_item) = find_from_item_in_from_clause(from_clause, &table_name)
        {
            columns.extend(columns_for_star_from_from_item(db, file, &from_item));
        }
    }

    columns
}

fn columns_for_star_from_clause(
    db: &dyn Db,
    file: File,
    from_clause: &ast::FromClause,
) -> Vec<(Name, Option<Type>)> {
    let mut columns = vec![];

    for from_item in ast_nav::iter_from_clause(from_clause) {
        columns.extend(columns_for_star_from_from_item(db, file, &from_item));
    }

    columns
}

fn columns_for_star_from_from_item(
    db: &dyn Db,
    file: File,
    from_item: &ast::FromItem,
) -> Vec<(Name, Option<Type>)> {
    if let Some(alias) = from_item.alias()
        && alias.column_list().is_some()
    {
        return columns_for_star_from_alias(db, file, from_item, &alias);
    }

    let Some(table_ptr) = table_ptr_from_from_item(db, file, from_item) else {
        return vec![];
    };

    columns_for_star_from_table_ptr(db, file, &table_ptr)
}

fn columns_for_star_from_alias(
    db: &dyn Db,
    file: File,
    from_item: &ast::FromItem,
    alias: &ast::Alias,
) -> Vec<(Name, Option<Type>)> {
    let alias_columns: Vec<Name> = alias
        .column_list()
        .into_iter()
        .flat_map(|column_list| column_list.columns())
        .filter_map(|column| column.name().map(|name| Name::from_node(&name)))
        .collect();

    let Some(table_ptr) = table_ptr_from_from_item(db, file, from_item) else {
        return vec![];
    };

    let base_columns = columns_for_star_from_table_ptr(db, file, &table_ptr);
    let mut results = vec![];

    for (idx, alias_name) in alias_columns.iter().enumerate() {
        results.push((
            alias_name.clone(),
            base_columns.get(idx).and_then(|(_, ty)| ty.clone()),
        ));
    }

    results.extend(base_columns.into_iter().skip(alias_columns.len()));

    results
}

fn columns_for_star_from_table_ptr(
    db: &dyn Db,
    file: File,
    table_ptr: &SyntaxNodePtr,
) -> Vec<(Name, Option<Type>)> {
    let tree = parse(db, file).tree();
    let root = tree.syntax();
    let table_node = table_ptr.to_node(root);

    match ast_nav::parent_source(&table_node) {
        Some(ast_nav::ParentSouce::Alias(alias)) => {
            let Some(from_item) = alias.syntax().ancestors().find_map(ast::FromItem::cast) else {
                return vec![];
            };
            columns_for_star_from_alias(db, file, &from_item, &alias)
        }
        Some(ast_nav::ParentSouce::WithTable(with_table)) => {
            with_table_columns_with_types(db, file, with_table)
        }
        Some(ast_nav::ParentSouce::CreateTable(create_table)) => {
            table_columns(db, file, &create_table)
        }
        Some(ast_nav::ParentSouce::CreateTableAs(create_table_as)) => {
            create_table_as_columns_with_types(&create_table_as)
        }
        Some(ast_nav::ParentSouce::CreateView(create_view)) => {
            view_like_columns_with_types(&create_view)
        }
        Some(ast_nav::ParentSouce::ParenSelect(paren_select)) => {
            paren_select_columns_with_types(db, file, &paren_select)
        }
        None => vec![],
    }
}

fn target_list_columns_with_types(target_list: &ast::TargetList) -> Vec<(Name, Option<Type>)> {
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

pub(crate) fn paren_select_columns_with_types(
    db: &dyn Db,
    file: File,
    paren_select: &ast::ParenSelect,
) -> Vec<(Name, Option<Type>)> {
    let Some(select_variant) = paren_select.select() else {
        return vec![];
    };
    select_variant_columns_with_types(db, file, &select_variant)
}

fn select_variant_columns_with_types(
    db: &dyn Db,
    file: File,
    select_variant: &ast::SelectVariant,
) -> Vec<(Name, Option<Type>)> {
    let tree = parse(db, file).tree();
    let root = tree.syntax();
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
            target_list_columns_with_types(&target_list)
        }
        ast::SelectVariant::SelectInto(select_into) => {
            let Some(select_clause) = select_into.select_clause() else {
                return vec![];
            };
            let Some(target_list) = select_clause.target_list() else {
                return vec![];
            };
            target_list_columns_with_types(&target_list)
        }
        ast::SelectVariant::ParenSelect(nested) => {
            paren_select_columns_with_types(db, file, nested)
        }
        ast::SelectVariant::CompoundSelect(compound) => {
            let Some(lhs) = compound.lhs() else {
                return vec![];
            };
            select_variant_columns_with_types(db, file, &lhs)
        }
        ast::SelectVariant::Table(table) => {
            let Some(path) = table.relation_name().and_then(|r| r.path()) else {
                return vec![];
            };
            let Some((schema, table_name)) = name::schema_and_name_path(&path) else {
                return vec![];
            };
            let name_ref = path.segment().and_then(|segment| segment.name_ref());
            let position = table.syntax().text_range().start();
            let Some((ptr, kind)) =
                resolve_table_like(db, file, name_ref.as_ref(), &table_name, &schema, position)
            else {
                return vec![];
            };
            let node = ptr.to_node(root);
            match kind {
                LocationKind::View => node
                    .ancestors()
                    .find_map(ast::CreateViewLike::cast)
                    .map(|v| view_like_columns_with_types(&v))
                    .unwrap_or_default(),
                LocationKind::Table => {
                    if let Some(with_table) = node.ancestors().find_map(ast::WithTable::cast) {
                        return with_table_column_names(db, file, with_table)
                            .into_iter()
                            .map(|name| (name, None))
                            .collect();
                    }
                    node.ancestors()
                        .find_map(ast::CreateTableLike::cast)
                        .map(|t| table_columns(db, file, &t))
                        .unwrap_or_default()
                }
                _ => vec![],
            }
        }
    }
}

pub(crate) fn star_column_names(db: &dyn Db, file: File, table_ptr: &SyntaxNodePtr) -> Vec<Name> {
    let source_file = parse(db, file).tree();
    let root = source_file.syntax();
    let table_name_node = table_ptr.to_node(root);

    match ast_nav::parent_source(&table_name_node) {
        Some(ast_nav::ParentSouce::Alias(alias)) => alias
            .column_list()
            .into_iter()
            .flat_map(|column_list| column_list.columns())
            .filter_map(|column| column.name().map(|name| Name::from_node(&name)))
            .collect(),
        Some(ast_nav::ParentSouce::WithTable(with_table)) => {
            let columns = with_table_column_names(db, file, with_table.clone());
            if !columns.is_empty() {
                return columns;
            }

            with_table_column_names(db, builtins_file(db), with_table)
        }
        Some(ast_nav::ParentSouce::CreateTable(create_table)) => {
            table_columns(db, file, &create_table)
                .into_iter()
                .map(|(name, _)| name)
                .collect()
        }
        Some(ast_nav::ParentSouce::CreateTableAs(create_table_as)) => {
            create_table_as_columns_with_types(&create_table_as)
                .into_iter()
                .map(|(name, _)| name)
                .collect()
        }
        Some(ast_nav::ParentSouce::CreateView(create_view)) => {
            view_like_columns_with_types(&create_view)
                .into_iter()
                .map(|(name, _)| name)
                .collect()
        }
        Some(ast_nav::ParentSouce::ParenSelect(paren_select)) => {
            let columns: Vec<Name> = paren_select_columns_with_types(db, file, &paren_select)
                .into_iter()
                .map(|(name, _ty)| name)
                .collect();
            if !columns.is_empty() {
                return columns;
            }
            return star_column_names_from_paren_select(db, file, &paren_select);
        }
        None => vec![],
    }
}

fn star_column_names_from_paren_select(
    db: &dyn Db,
    file: File,
    paren_select: &ast::ParenSelect,
) -> Vec<Name> {
    let Some(ast::SelectVariant::Select(select)) = paren_select.select() else {
        return vec![];
    };
    let Some(from_clause) = select.from_clause() else {
        return vec![];
    };
    let mut columns = vec![];
    for from_item in ast_nav::iter_from_clause(&from_clause) {
        if let Some(table_ptr) = table_ptr_from_from_item(db, file, &from_item) {
            columns.extend(star_column_names(db, file, &table_ptr));
        }
    }
    columns
}
