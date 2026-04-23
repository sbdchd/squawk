use squawk_syntax::ast;

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
