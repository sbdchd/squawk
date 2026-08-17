use rowan::TextSize;
use salsa::Database as Db;
use squawk_syntax::ast;

use crate::{
    db::{File, bind},
    name::Name,
    symbols::SymbolKind,
};

pub(super) struct UnnestCall {
    pub(super) name: ast::NameRef,
    pub(super) args: Vec<ast::Arg>,
}

pub(super) fn unnest_call(call_expr: &ast::CallExpr) -> Option<UnnestCall> {
    let ast::Expr::NameRef(name_ref) = call_expr.expr()? else {
        return None;
    };
    if Name::from_node(&name_ref) != "unnest" {
        return None;
    }
    if call_expr.over_clause().is_some()
        || call_expr.filter_clause().is_some()
        || call_expr.within_clause().is_some()
        || call_expr.null_treatment().is_some()
    {
        return None;
    }
    let arg_list = call_expr.arg_list()?;
    if arg_list.star_token().is_some()
        || matches!(
            arg_list.all_or_distinct(),
            Some(ast::AllOrDistinct::Distinct(_))
        )
    {
        return None;
    }
    let mut args = vec![];
    for arg in arg_list.args() {
        if arg.variadic_token().is_some()
            || arg.named_arg().is_some()
            || arg.order_by_clause().is_some()
        {
            return None;
        }
        args.push(arg);
    }
    if args.is_empty() {
        return None;
    }
    Some(UnnestCall {
        name: name_ref,
        args,
    })
}

pub(super) fn unnest_shadowed(db: &dyn Db, file: File, position: TextSize) -> bool {
    let binder = bind(db, file);
    let schemas = binder.resolved_schemas(position, None);
    binder
        .lookup_with("unnest", SymbolKind::Function, &schemas)
        .is_some()
}
