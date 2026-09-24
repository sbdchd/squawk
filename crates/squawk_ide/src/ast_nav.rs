/// [`ast_nav`] operates on ast nodes. Functions should take in and return ast nodes.
///
/// There shouldn't be any dependency on Salsa.
use squawk_syntax::{
    SyntaxKind, SyntaxNode, SyntaxToken,
    ast::{self, AstNode},
};
use std::iter;

use crate::name;
use crate::symbols::Name;

pub(crate) fn find_cte_with_table(
    name_ref: &impl ast::NameLike,
    cte_name: &Name,
) -> Option<ast::WithTable> {
    let ref_start = name_ref.syntax().text_range().start();

    for with_clause in name_ref
        .syntax()
        .ancestors()
        .filter_map(|query| ast::WithQuery::cast(query)?.with_clause())
    {
        let is_recursive = with_clause.recursive_token().is_some();
        if let Some(with_table) = with_clause
            .with_tables()
            // Without RECURSIVE, only CTEs before the reference are visible.
            .filter(|with_table| {
                is_recursive || with_table.syntax().text_range().end() <= ref_start
            })
            .find(|with_table| {
                with_table
                    .name()
                    .is_some_and(|name| Name::from_node(&name) == *cte_name)
            })
        {
            return Some(with_table);
        }
    }

    None
}

pub(crate) fn ancestors_outside_own_with_clause(
    node: &SyntaxNode,
) -> impl Iterator<Item = SyntaxNode> {
    let mut prev_was_with_clause = false;
    node.ancestors().filter(move |ancestor| {
        let skip = prev_was_with_clause;
        prev_was_with_clause = ast::WithClause::can_cast(ancestor.kind());
        !skip
    })
}

pub(crate) fn iter_values_columns(values: &ast::Values) -> impl Iterator<Item = (Name, ast::Expr)> {
    values
        .row_list()
        .into_iter()
        .flat_map(|rl| rl.rows().take(1))
        .flat_map(|r| r.exprs().enumerate())
        .map(|(idx, expr)| {
            let name = Name::from_string(format!("column{}", idx + 1));
            (name, expr)
        })
}

#[derive(Debug)]
pub(crate) enum ParentQuery {
    Select(ast::Select),
    SelectInto(ast::SelectInto),
    Update(ast::Update),
    Delete(ast::Delete),
    Insert(ast::Insert),
    Merge(ast::Merge),
}

pub(crate) fn target_parent_query(target: ast::Target) -> Option<ParentQuery> {
    node_parent_query(target.syntax())
}

pub(crate) fn node_parent_query(node: &SyntaxNode) -> Option<ParentQuery> {
    use ParentQuery::*;

    for ancestor in node.ancestors() {
        let result = if let Some(select) = ast::Select::cast(ancestor.clone()) {
            Select(select)
        } else if let Some(select_into) = ast::SelectInto::cast(ancestor.clone()) {
            SelectInto(select_into)
        } else if let Some(update) = ast::Update::cast(ancestor.clone()) {
            Update(update)
        } else if let Some(insert) = ast::Insert::cast(ancestor.clone()) {
            Insert(insert)
        } else if let Some(delete) = ast::Delete::cast(ancestor.clone()) {
            Delete(delete)
        } else if let Some(merge) = ast::Merge::cast(ancestor) {
            Merge(merge)
        } else {
            continue;
        };

        return Some(result);
    }

    None
}

#[derive(Debug)]
pub(crate) enum SelectContext {
    Compound(ast::CompoundSelect),
    Single(ast::Select),
}

impl SelectContext {
    pub(crate) fn iter(&self) -> Option<Box<dyn Iterator<Item = ast::Select>>> {
        fn variant_iter(
            variant: ast::SelectVariant,
        ) -> Option<Box<dyn Iterator<Item = ast::Select>>> {
            match variant {
                ast::SelectVariant::Select(select) => Some(Box::new(iter::once(select))),
                ast::SelectVariant::CompoundSelect(compound) => compound_iter(&compound),
                ast::SelectVariant::ParenSelect(_)
                | ast::SelectVariant::SelectInto(_)
                | ast::SelectVariant::Table(_)
                | ast::SelectVariant::Values(_) => None,
            }
        }

        fn compound_iter(
            node: &ast::CompoundSelect,
        ) -> Option<Box<dyn Iterator<Item = ast::Select>>> {
            let lhs_iter = node
                .lhs()
                .map(variant_iter)
                .unwrap_or_else(|| Some(Box::new(iter::empty())))?;
            let rhs_iter = node
                .rhs()
                .map(variant_iter)
                .unwrap_or_else(|| Some(Box::new(iter::empty())))?;
            Some(Box::new(lhs_iter.chain(rhs_iter)))
        }

        match self {
            SelectContext::Compound(compound) => compound_iter(compound),
            SelectContext::Single(select) => Some(Box::new(iter::once(select.clone()))),
        }
    }
}

pub(crate) fn find_select_parent(token: SyntaxToken) -> Option<SelectContext> {
    let mut found_select = None;
    let mut found_compound = None;

    for ancestor in token.parent_ancestors() {
        if let Some(compound_select) = ast::CompoundSelect::cast(ancestor.clone()) {
            if let Some(ast::CompoundOp::Union(union)) = compound_select.op()
                && matches!(union.all_or_distinct(), Some(ast::AllOrDistinct::All(_)))
            {
                found_compound = Some(SelectContext::Compound(compound_select));
            } else {
                break;
            }
        }

        if found_select.is_none()
            && let Some(select) = ast::Select::cast(ancestor)
        {
            found_select = Some(SelectContext::Single(select));
        }
    }

    found_compound.or(found_select)
}

///
/// ```sql
/// with t as (select 1)
/// select * from t;
/// -- becomes
/// select 1
/// ```
pub(crate) fn select_from_with_query(query: ast::WithQuery) -> Option<ast::Select> {
    let select_variant = match query {
        ast::WithQuery::Select(select) => ast::SelectVariant::Select(select),
        ast::WithQuery::ParenSelect(paren_select) => paren_select.select()?,
        ast::WithQuery::CompoundSelect(compound_select) => {
            ast::SelectVariant::CompoundSelect(compound_select)
        }
        _ => return None,
    };

    select_from_variant(select_variant)
}

/// Extract nested select ignoring, select into, table, values
///
/// ```sql
/// ((select 1))
/// -- or
/// select 1 union select 2
/// -- become
/// select 1
/// ```
pub(crate) fn select_from_variant(select_variant: ast::SelectVariant) -> Option<ast::Select> {
    match select_variant {
        ast::SelectVariant::Select(select) => return Some(select),
        ast::SelectVariant::CompoundSelect(compound) => {
            return select_from_variant(compound.lhs()?);
        }
        ast::SelectVariant::ParenSelect(paren_select) => {
            return select_from_variant(paren_select.select()?);
        }
        ast::SelectVariant::SelectInto(_)
        | ast::SelectVariant::Table(_)
        | ast::SelectVariant::Values(_) => {
            return None;
        }
    }
}

#[derive(Debug)]
pub(crate) enum ParentSouce {
    Alias(ast::FromAlias),
    CreateTable(ast::CreateTableLike),
    CreateTableAs(ast::CreateTableAs),
    CreateView(ast::CreateViewLike),
    ParenSelect(ast::ParenSelect),
    SelectInto(ast::SelectInto),
    WithTable(ast::WithTable),
}

pub(crate) fn parent_source(node: &SyntaxNode) -> Option<ParentSouce> {
    if let Some(paren_select) = ast::ParenSelect::cast(node.clone()) {
        return Some(ParentSouce::ParenSelect(paren_select));
    }

    for ancestor in node.ancestors() {
        if let Some(paren_select) = ast::ParenSelect::cast(ancestor.clone()) {
            return Some(ParentSouce::ParenSelect(paren_select));
        }

        if let Some(alias) = ast::FromAlias::cast(ancestor.clone()) {
            return Some(ParentSouce::Alias(alias));
        }

        if let Some(with_table) = ast::WithTable::cast(ancestor.clone()) {
            return Some(ParentSouce::WithTable(with_table));
        }

        if let Some(create_view) = ast::CreateViewLike::cast(ancestor.clone()) {
            return Some(ParentSouce::CreateView(create_view));
        }

        if let Some(create_table_as) = ast::CreateTableAs::cast(ancestor.clone()) {
            return Some(ParentSouce::CreateTableAs(create_table_as));
        }

        if let Some(create_table) = ast::CreateTableLike::cast(ancestor.clone()) {
            return Some(ParentSouce::CreateTable(create_table));
        }

        if let Some(select_into) = ast::SelectInto::cast(ancestor.clone()) {
            return Some(ParentSouce::SelectInto(select_into));
        }
    }

    None
}

pub(crate) enum CreateTableArg {
    Column(ast::Column),
    Inherits(ast::PathRef),
    LikeClause(ast::LikeClause),
    TableConstraint(#[expect(unused)] ast::TableConstraint),
}

pub(crate) fn create_table_args(
    create_table: &impl ast::HasCreateTable,
) -> impl Iterator<Item = CreateTableArg> {
    let inherits_iter = create_table
        .inherits()
        .into_iter()
        .flat_map(|inherits| inherits.table_name_refs())
        .filter_map(|table| table.path_ref())
        .map(CreateTableArg::Inherits);

    let args_iter = create_table
        .table_arg_list()
        .into_iter()
        .flat_map(|arg_list| arg_list.args())
        .map(|arg| match arg {
            ast::TableArg::Column(column) => CreateTableArg::Column(column),
            ast::TableArg::LikeClause(like_clause) => CreateTableArg::LikeClause(like_clause),
            ast::TableArg::TableConstraint(constraint) => {
                CreateTableArg::TableConstraint(constraint)
            }
        });

    inherits_iter.chain(args_iter)
}

struct UnwrapParenExpr {
    current: Option<ast::Expr>,
}

impl Iterator for UnwrapParenExpr {
    type Item = ast::Expr;

    fn next(&mut self) -> Option<Self::Item> {
        let expr = self.current.take()?;
        if let ast::Expr::ParenExpr(paren_expr) = &expr {
            self.current = paren_expr.expr();
        }
        Some(expr)
    }
}

pub(crate) fn unwrap_paren_expr(expr: ast::Expr) -> impl Iterator<Item = ast::Expr> {
    UnwrapParenExpr {
        current: Some(expr),
    }
}

pub(crate) fn iter_from_clause(
    from_clause: &ast::FromClause,
) -> impl Iterator<Item = ast::FromItem> {
    iter_from_items(from_clause.items())
}

// A FROM item can't see its siblings, unless it's lateral, then it can see
// the items before it. An `on` clause can only see the items in its own join.
pub(crate) fn visible_from_items(
    from_items: impl Iterator<Item = ast::FromItem>,
    node: &SyntaxNode,
) -> Vec<ast::FromItem> {
    let mut from_items = from_items.peekable();
    if let Some(from_item) = from_items.peek()
        && let Some(join_expr) = enclosing_on_clause_join_expr(node, from_item)
    {
        return iter_join_expr(&join_expr).collect();
    }
    let mut visible = vec![];
    collect_visible_from_items(from_items, node, &mut visible);
    visible
}

fn enclosing_on_clause_join_expr(
    node: &SyntaxNode,
    from_item: &ast::FromItem,
) -> Option<ast::JoinExpr> {
    let from_list = enclosing_from_list(from_item.syntax())?;
    node.ancestors()
        .filter_map(ast::OnClause::cast)
        .filter_map(|on_clause| {
            let join = on_clause.syntax().parent().and_then(ast::Join::cast)?;
            join.syntax().parent().and_then(ast::JoinExpr::cast)
        })
        .find(|join_expr| enclosing_from_list(join_expr.syntax()).as_ref() == Some(&from_list))
}

fn enclosing_from_list(node: &SyntaxNode) -> Option<SyntaxNode> {
    node.ancestors().find(|ancestor| {
        ast::FromClause::can_cast(ancestor.kind())
            || ast::UsingClause::can_cast(ancestor.kind())
            || ast::UsingOnClause::can_cast(ancestor.kind())
    })
}

fn collect_visible_from_items(
    from_items: impl Iterator<Item = ast::FromItem>,
    node: &SyntaxNode,
    visible: &mut Vec<ast::FromItem>,
) {
    for from_item in from_items {
        if !from_item
            .syntax()
            .text_range()
            .contains_range(node.text_range())
        {
            visible.push(from_item);
            continue;
        }
        hide_disallowed_lateral_items(&from_item, visible);
        if let ast::FromItem::ParenFromItem(paren) = &from_item
            && let Some(paren_expr) = paren.paren_expr()
        {
            collect_visible_from_items(
                iter_from_items(paren_expr.from_list_item().into_iter()),
                node,
                visible,
            );
        } else if !is_lateral_from_item(&from_item) {
            visible.clear();
        }
        return;
    }
}

fn hide_disallowed_lateral_items(from_item: &ast::FromItem, visible: &mut Vec<ast::FromItem>) {
    let Some(join) = from_item.syntax().parent().and_then(ast::Join::cast) else {
        return;
    };
    if !matches!(
        join.join_type(),
        Some(ast::JoinType::JoinRight(_) | ast::JoinType::JoinFull(_))
    ) {
        return;
    }
    let Some(lhs) = join
        .syntax()
        .parent()
        .and_then(ast::JoinExpr::cast)
        .and_then(|join_expr| join_expr.from_list_item())
    else {
        return;
    };
    let hidden: Vec<_> = iter_from_items(std::iter::once(lhs)).collect();
    visible.retain(|from_item| !hidden.contains(from_item));
}

fn is_lateral_from_item(from_item: &ast::FromItem) -> bool {
    match from_item {
        ast::FromItem::ExprFromItem(_)
        | ast::FromItem::FunctionFromItem(_)
        | ast::FromItem::JsonTableFromItem(_)
        | ast::FromItem::RowsFromItem(_)
        | ast::FromItem::XmlTableFromItem(_) => true,
        ast::FromItem::GraphTableFromItem(_)
        | ast::FromItem::ParenFromItem(_)
        | ast::FromItem::RelationFromItem(_) => from_item
            .syntax()
            .children_with_tokens()
            .any(|it| it.kind() == SyntaxKind::LATERAL_KW),
    }
}

pub(crate) fn iter_join_expr(join_expr: &ast::JoinExpr) -> impl Iterator<Item = ast::FromItem> {
    iter_from_items(std::iter::once(ast::FromListItem::JoinExpr(
        join_expr.clone(),
    )))
}

pub(crate) fn iter_from_items(
    items: impl Iterator<Item = ast::FromListItem>,
) -> impl Iterator<Item = ast::FromItem> {
    let mut stack = items.collect::<Vec<_>>();
    stack.reverse();
    FromItemIter { stack }
}

struct FromItemIter {
    stack: Vec<ast::FromListItem>,
}

impl Iterator for FromItemIter {
    type Item = ast::FromItem;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.stack.pop() {
            match item {
                ast::FromListItem::FromItem(from_item) => return Some(from_item),
                ast::FromListItem::JoinExpr(join_expr) => {
                    if let Some(rhs) = join_expr.join().and_then(|join| join.from_list_item()) {
                        self.stack.push(rhs);
                    }
                    if let Some(lhs) = join_expr.from_list_item() {
                        self.stack.push(lhs);
                    }
                }
            }
        }

        None
    }
}

pub(crate) enum RoutineKind {
    Function,
    Procedure,
}

pub(crate) fn enclosing_routine_name(
    node: &SyntaxNode,
) -> Option<(Name, ast::PathSegment, RoutineKind)> {
    node.ancestors().find_map(|ancestor| {
        let (path, kind) =
            if let Some(create_function) = ast::CreateFunction::cast(ancestor.clone()) {
                (create_function.name()?.path()?, RoutineKind::Function)
            } else {
                (
                    ast::CreateProcedure::cast(ancestor)?.name()?.path()?,
                    RoutineKind::Procedure,
                )
            };
        let (_, routine_name) = name::schema_and_name_definition(&path)?;
        Some((routine_name, path.segment()?, kind))
    })
}
