/// [`ast_nav`] operates on ast nodes. Functions should take in and return ast nodes.
///
/// There shouldn't be any dependency on Salsa.
use squawk_syntax::{
    SyntaxNode, SyntaxToken,
    ast::{self, AstNode},
};
use std::iter;

use crate::symbols::Name;

pub(crate) fn find_cte_with_table(
    name_ref: &ast::NameRef,
    cte_name: &Name,
) -> Option<ast::WithTable> {
    let with_clause = name_ref
        .syntax()
        .ancestors()
        .find_map(|query| ast::WithQuery::cast(query)?.with_clause())?;

    let is_recursive = with_clause.recursive_token().is_some();
    for with_table in with_clause.with_tables() {
        if let Some(name) = with_table.name()
            && Name::from_node(&name) == *cte_name
        {
            // Skip if we're inside this CTE's definition (CTE doesn't shadow itself)
            if !is_recursive
                && with_table
                    .syntax()
                    .text_range()
                    .contains_range(name_ref.syntax().text_range())
            {
                continue;
            }
            return Some(with_table);
        }
    }
    None
}

#[derive(Debug)]
pub(crate) enum ParentQuery {
    Select(ast::Select),
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
            if compound_select.union_token().is_some() && compound_select.all_token().is_some() {
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
    Alias(ast::Alias),
    CreateTable(ast::CreateTableLike),
    CreateTableAs(ast::CreateTableAs),
    CreateView(ast::CreateViewLike),
    ParenSelect(ast::ParenSelect),
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

        if let Some(alias) = ast::Alias::cast(ancestor.clone()) {
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
    }

    None
}

pub(crate) enum CreateTableArg {
    Column(ast::Column),
    Inherits(ast::Path),
    LikeClause(ast::LikeClause),
    TableConstraint(#[expect(unused)] ast::TableConstraint),
}

pub(crate) fn create_table_args(
    create_table: &impl ast::HasCreateTable,
) -> impl Iterator<Item = CreateTableArg> {
    let inherits_iter = create_table
        .inherits()
        .into_iter()
        .flat_map(|inherits| inherits.paths())
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
    from_clause.from_items().chain(
        from_clause
            .join_exprs()
            .flat_map(|join_expr| JoinExprIter::new(&join_expr)),
    )
}

pub(crate) fn iter_join_expr(join_expr: &ast::JoinExpr) -> impl Iterator<Item = ast::FromItem> {
    JoinExprIter::new(join_expr)
}

struct JoinExprIter {
    stack: Vec<JoinExprIterFrame>,
}

impl JoinExprIter {
    fn new(join_expr: &ast::JoinExpr) -> Self {
        Self {
            stack: vec![JoinExprIterFrame {
                join_expr: join_expr.clone(),
                state: JoinExprIterState::JoinExpr,
            }],
        }
    }
}

struct JoinExprIterFrame {
    join_expr: ast::JoinExpr,
    state: JoinExprIterState,
}

#[derive(Clone, Copy)]
enum JoinExprIterState {
    FromItem,
    Join,
    JoinExpr,
}

impl Iterator for JoinExprIter {
    type Item = ast::FromItem;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(frame) = self.stack.last_mut() {
            match frame.state {
                JoinExprIterState::JoinExpr => {
                    frame.state = JoinExprIterState::FromItem;

                    if let Some(nested_join) = frame.join_expr.join_expr() {
                        self.stack.push(JoinExprIterFrame {
                            join_expr: nested_join,
                            state: JoinExprIterState::JoinExpr,
                        });
                    }
                }
                JoinExprIterState::FromItem => {
                    frame.state = JoinExprIterState::Join;

                    if let Some(from_item) = frame.join_expr.from_item() {
                        return Some(from_item);
                    }
                }
                JoinExprIterState::Join => {
                    let from_item = frame.join_expr.join().and_then(|join| join.from_item());
                    self.stack.pop();

                    if from_item.is_some() {
                        return from_item;
                    }
                }
            }
        }

        None
    }
}
