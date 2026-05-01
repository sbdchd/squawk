use itertools::Itertools;
use rowan::Direction;
use squawk_syntax::ast::{self, AstNode};
use squawk_syntax::{SyntaxKind, SyntaxNode, SyntaxToken};
use tiny_pretty::Doc;
use tiny_pretty::{PrintOptions, print};

// TODO: anytime we have `syntax().to_string()`, it means we have to do more to
// actually convert the data into the IR. to_string() is a temp hack

fn build_source_file(source_file: &ast::SourceFile) -> Doc<'_> {
    let mut doc = Doc::nil();
    for el in source_file.syntax().children_with_tokens() {
        match el {
            rowan::NodeOrToken::Node(node) => {
                if let Some(stmt) = ast::Stmt::cast(node) {
                    match stmt {
                        ast::Stmt::Select(select) => {
                            doc = doc.append(build_select_doc(&select));
                        }
                        ast::Stmt::CreateTable(create_table) => {
                            doc = doc.append(build_create_table(&create_table));
                        }
                        _ => (),
                    }
                }
            }
            rowan::NodeOrToken::Token(token) => {
                if token.kind() == SyntaxKind::COMMENT {
                    doc = doc.append(Doc::text(token.text().to_string()));
                } else if token.kind() == SyntaxKind::WHITESPACE {
                    // TODO: I think we can improve this
                    let lines = token.text().lines().count();
                    if lines >= 2 {
                        doc = doc.append(Doc::empty_line()).append(Doc::empty_line());
                    } else {
                        doc = doc.append(Doc::empty_line());
                    }
                } else if token.kind() == SyntaxKind::SEMICOLON {
                    doc = doc.append(Doc::text(";"));
                }
            }
        }
    }
    doc
}

fn build_create_table<'a>(create_table: &ast::CreateTable) -> Doc<'a> {
    Doc::text("create")
        .append(Doc::space())
        .append(Doc::text("table"))
        .append(Doc::space())
        .append(Doc::text(
            create_table.path().map(|x| x.syntax().to_string()).unwrap(),
        ))
        .append(Doc::text("("))
        .append(
            Doc::line_or_nil()
                .append(Doc::list(
                    Itertools::intersperse(
                        create_table
                            .table_arg_list()
                            .unwrap()
                            .args()
                            .map(build_table_arg),
                        Doc::text(",").append(Doc::hard_line()),
                    )
                    .collect(),
                ))
                .nest(2)
                .append(Doc::line_or_nil())
                .group(),
        )
        .append(Doc::text(")"))
}

fn build_table_arg<'a>(create_table: ast::TableArg) -> Doc<'a> {
    match create_table {
        ast::TableArg::Column(column) => Doc::text(column.name().unwrap().syntax().to_string())
            .append(Doc::space())
            .append(Doc::text(column.ty().unwrap().syntax().to_string())),
        ast::TableArg::LikeClause(_like_clause) => todo!(),
        ast::TableArg::TableConstraint(_table_constraint) => todo!(),
    }
}

fn build_select_doc<'a>(select: &ast::Select) -> Doc<'a> {
    let mut doc = Doc::text("select").append(Doc::line_or_space());

    if let Some(select_clause) = select.select_clause() {
        if let Some(distinct_clause) = select_clause.distinct_clause() {
            doc = doc.append(leading_comments(distinct_clause.syntax()));
            doc = doc.append(Doc::text("distinct")).append(Doc::space());
        }
        if let Some(all_token) = select_clause.all_token() {
            doc = doc.append(leading_comments_token(&all_token));
            doc = doc.append(Doc::text("all")).append(Doc::space());
        }
        if let Some(target_list) = select_clause.target_list() {
            doc = doc.append(leading_comments(target_list.syntax()));
            doc = doc
                .append(Doc::list(
                    Itertools::intersperse(
                        target_list.targets().flat_map(build_target),
                        Doc::text(",").append(Doc::line_or_space()),
                    )
                    .collect(),
                ))
                .nest(2);
        }
    }

    if let Some(from) = &select.from_clause() {
        doc = doc.append(
            Doc::line_or_space()
                .append(Doc::text("from"))
                .append(Doc::space())
                .append(Doc::text(
                    from.from_items().next().unwrap().syntax().to_string(),
                )),
        );
    }

    if let Some(group) = &select.group_by_clause() {
        doc = doc.append(
            Doc::line_or_space()
                .append(Doc::text("group by"))
                .append(Doc::space())
                .append(Doc::text(
                    group.group_by_list().unwrap().syntax().to_string(),
                )),
        );
    }

    doc.group()
}

fn build_expr<'a>(expr: ast::Expr) -> Doc<'a> {
    match expr {
        ast::Expr::ArrayExpr(array_expr) => {
            let mut doc = Doc::nil();

            // nested parts of array expressions don't require the array token
            if array_expr.array_token().is_some() {
                doc = doc.append(Doc::text("array"));
            };

            if let Some(select) = array_expr.select() {
                doc = doc
                    .append(Doc::text("("))
                    .append(build_select_doc(&select))
                    .append(Doc::text(")"))
            } else {
                doc = doc
                    .append(Doc::text("["))
                    .append(Doc::list(
                        Itertools::intersperse(
                            array_expr.exprs().map(build_expr),
                            Doc::text(",").append(Doc::space()),
                        )
                        .collect(),
                    ))
                    .append(Doc::text("]"));
            }

            doc
        }
        ast::Expr::BetweenExpr(between_expr) => {
            let mut doc = build_expr(between_expr.target().unwrap());
            if between_expr.not_token().is_some() {
                doc = doc.append(Doc::space()).append(Doc::text("not"));
            }
            doc = doc.append(Doc::space()).append(Doc::text("between"));
            if between_expr.symmetric_token().is_some() {
                doc = doc.append(Doc::space()).append(Doc::text("symmetric"));
            }
            doc.append(Doc::space())
                .append(build_expr(between_expr.start().unwrap()))
                .append(Doc::space())
                .append(Doc::text("and"))
                .append(Doc::space())
                .append(build_expr(between_expr.end().unwrap()))
        }
        ast::Expr::BinExpr(bin_expr) => build_expr(bin_expr.lhs().unwrap())
            .append(Doc::space())
            .append(build_op(bin_expr.op().unwrap()))
            .append(Doc::space())
            .append(build_expr(bin_expr.rhs().unwrap())),
        // ast::Expr::CallExpr(call_expr) => todo!(),
        // ast::Expr::CaseExpr(case_expr) => todo!(),
        ast::Expr::CastExpr(cast_expr) => {
            let mut doc = Doc::nil();
            if cast_expr.colon_colon().is_some() {
                doc = doc
                    .append(build_expr(cast_expr.expr().unwrap()))
                    .append(Doc::text("::"))
                    .append(build_type(cast_expr.ty().unwrap()))
            } else if cast_expr.as_token().is_some() {
                if cast_expr.cast_token().is_some() {
                    doc = doc.append(Doc::text("cast"))
                } else if cast_expr.treat_token().is_some() {
                    doc = doc.append(Doc::text("treat"))
                }
                doc = doc
                    .append(Doc::text("("))
                    .append(build_expr(cast_expr.expr().unwrap()))
                    .append(Doc::space())
                    .append(Doc::text("as"))
                    .append(Doc::space())
                    .append(build_type(cast_expr.ty().unwrap()))
                    .append(Doc::text(")"))
            } else {
                doc = doc
                    .append(build_type(cast_expr.ty().unwrap()))
                    .append(Doc::space())
                    .append(build_literal(cast_expr.literal().unwrap()))
            }
            doc
        }
        // ast::Expr::FieldExpr(field_expr) => todo!(),
        // ast::Expr::IndexExpr(index_expr) => todo!(),
        // ast::Expr::Literal(literal) => todo!(),
        // ast::Expr::NameRef(name_ref) => todo!(),
        // ast::Expr::ParenExpr(paren_expr) => todo!(),
        ast::Expr::PostfixExpr(postfix_expr) => {
            let expr = build_expr(postfix_expr.expr().unwrap());
            let op = match postfix_expr.op().unwrap() {
                ast::PostfixOp::AtLocal(_) => Doc::text("at local"),
                ast::PostfixOp::IsNull(_) => Doc::text("isnull"),
                ast::PostfixOp::NotNull(_) => Doc::text("notnull"),
                ast::PostfixOp::IsJson(n) => {
                    let mut doc = Doc::text("is json");
                    if let Some(clause) = n.json_keys_unique_clause() {
                        doc = doc
                            .append(Doc::space())
                            .append(build_json_keys_unique_clause(clause));
                    }
                    doc
                }
                ast::PostfixOp::IsJsonArray(n) => {
                    let mut doc = Doc::text("is json array");
                    if let Some(clause) = n.json_keys_unique_clause() {
                        doc = doc
                            .append(Doc::space())
                            .append(build_json_keys_unique_clause(clause));
                    }
                    doc
                }
                ast::PostfixOp::IsJsonObject(n) => {
                    let mut doc = Doc::text("is json object");
                    if let Some(clause) = n.json_keys_unique_clause() {
                        doc = doc
                            .append(Doc::space())
                            .append(build_json_keys_unique_clause(clause));
                    }
                    doc
                }
                ast::PostfixOp::IsJsonScalar(n) => {
                    let mut doc = Doc::text("is json scalar");
                    if let Some(clause) = n.json_keys_unique_clause() {
                        doc = doc
                            .append(Doc::space())
                            .append(build_json_keys_unique_clause(clause));
                    }
                    doc
                }
                ast::PostfixOp::IsJsonValue(n) => {
                    let mut doc = Doc::text("is json value");
                    if let Some(clause) = n.json_keys_unique_clause() {
                        doc = doc
                            .append(Doc::space())
                            .append(build_json_keys_unique_clause(clause));
                    }
                    doc
                }
                ast::PostfixOp::IsNormalized(n) => {
                    let mut doc = Doc::text("is");
                    if let Some(form) = n.unicode_normal_form() {
                        doc = doc
                            .append(Doc::space())
                            .append(build_unicode_normal_form(form));
                    }
                    doc.append(Doc::space()).append(Doc::text("normalized"))
                }
                ast::PostfixOp::IsNotJson(n) => {
                    let mut doc = Doc::text("is not json");
                    if let Some(clause) = n.json_keys_unique_clause() {
                        doc = doc
                            .append(Doc::space())
                            .append(build_json_keys_unique_clause(clause));
                    }
                    doc
                }
                ast::PostfixOp::IsNotJsonArray(n) => {
                    let mut doc = Doc::text("is not json array");
                    if let Some(clause) = n.json_keys_unique_clause() {
                        doc = doc
                            .append(Doc::space())
                            .append(build_json_keys_unique_clause(clause));
                    }
                    doc
                }
                ast::PostfixOp::IsNotJsonObject(n) => {
                    let mut doc = Doc::text("is not json object");
                    if let Some(clause) = n.json_keys_unique_clause() {
                        doc = doc
                            .append(Doc::space())
                            .append(build_json_keys_unique_clause(clause));
                    }
                    doc
                }
                ast::PostfixOp::IsNotJsonScalar(n) => {
                    let mut doc = Doc::text("is not json scalar");
                    if let Some(clause) = n.json_keys_unique_clause() {
                        doc = doc
                            .append(Doc::space())
                            .append(build_json_keys_unique_clause(clause));
                    }
                    doc
                }
                ast::PostfixOp::IsNotJsonValue(n) => {
                    let mut doc = Doc::text("is not json value");
                    if let Some(clause) = n.json_keys_unique_clause() {
                        doc = doc
                            .append(Doc::space())
                            .append(build_json_keys_unique_clause(clause));
                    }
                    doc
                }
                ast::PostfixOp::IsNotNormalized(n) => {
                    let mut doc = Doc::text("is not");
                    if let Some(form) = n.unicode_normal_form() {
                        doc = doc
                            .append(Doc::space())
                            .append(build_unicode_normal_form(form));
                    }
                    doc.append(Doc::space()).append(Doc::text("normalized"))
                }
            };
            expr.append(Doc::space()).append(op)
        }
        // ast::Expr::PrefixExpr(prefix_expr) => todo!(),
        // ast::Expr::SliceExpr(slice_expr) => todo!(),
        // ast::Expr::TupleExpr(tuple_expr) => todo!(),
        _ => Doc::text(expr.syntax().to_string()),
    }
}

fn build_json_keys_unique_clause<'a>(clause: ast::JsonKeysUniqueClause) -> Doc<'a> {
    let prefix = if clause.with_token().is_some() {
        "with"
    } else {
        "without"
    };
    Doc::text(prefix)
        .append(Doc::space())
        .append(Doc::text("unique"))
        .append(Doc::space())
        .append(Doc::text("keys"))
}

fn build_unicode_normal_form<'a>(form: ast::UnicodeNormalForm) -> Doc<'a> {
    if form.nfc_token().is_some() {
        Doc::text("nfc")
    } else if form.nfd_token().is_some() {
        Doc::text("nfd")
    } else if form.nfkc_token().is_some() {
        Doc::text("nfkc")
    } else {
        Doc::text("nfkd")
    }
}

fn build_op<'a>(op: ast::BinOp) -> Doc<'a> {
    match op {
        ast::BinOp::And(_) => todo!(),
        ast::BinOp::AtTimeZone(_) => todo!(),
        ast::BinOp::Caret(_) => todo!(),
        ast::BinOp::Collate(_) => todo!(),
        ast::BinOp::ColonColon(_) => todo!(),
        ast::BinOp::ColonEq(_) => todo!(),
        ast::BinOp::CustomOp(custom_op) => Doc::text(custom_op.syntax().to_string()),
        ast::BinOp::Eq(_) => todo!(),
        ast::BinOp::FatArrow(_) => todo!(),
        ast::BinOp::Gteq(_) => todo!(),
        ast::BinOp::Ilike(_) => todo!(),
        ast::BinOp::In(_) => todo!(),
        ast::BinOp::Is(_) => todo!(),
        ast::BinOp::IsDistinctFrom(_) => todo!(),
        ast::BinOp::IsNot(_) => todo!(),
        ast::BinOp::IsNotDistinctFrom(_) => todo!(),
        ast::BinOp::LAngle(_) => todo!(),
        ast::BinOp::Like(_) => todo!(),
        ast::BinOp::Lteq(_) => todo!(),
        ast::BinOp::Minus(_) => todo!(),
        ast::BinOp::Neq(_) => todo!(),
        ast::BinOp::Neqb(_) => todo!(),
        ast::BinOp::NotIlike(_) => todo!(),
        ast::BinOp::NotIn(_) => todo!(),
        ast::BinOp::NotLike(_) => todo!(),
        ast::BinOp::NotSimilarTo(_) => todo!(),
        ast::BinOp::OperatorCall(_) => todo!(),
        ast::BinOp::Or(_) => todo!(),
        ast::BinOp::Overlaps(_) => todo!(),
        ast::BinOp::Percent(_) => todo!(),
        ast::BinOp::Plus(_) => Doc::text("+"),
        ast::BinOp::RAngle(_) => todo!(),
        ast::BinOp::SimilarTo(_) => todo!(),
        ast::BinOp::Slash(_) => todo!(),
        ast::BinOp::Star(_) => todo!(),
    }
}

fn build_literal<'a>(lit: ast::Literal) -> Doc<'a> {
    Doc::text(lit.syntax().to_string())
}

fn build_type<'a>(ty: ast::Type) -> Doc<'a> {
    Doc::text(ty.syntax().to_string())
}

fn leading_comments_token<'a>(node: &SyntaxToken) -> Doc<'a> {
    let mut doc = Doc::nil();
    for next in node.siblings_with_tokens(Direction::Prev).skip(1) {
        println!("prev");
        match next {
            rowan::NodeOrToken::Node(node) => {
                println!("before node {:?}", node);
                break;
            }
            rowan::NodeOrToken::Token(token) => {
                println!("before token {:?}", token);
                if token.kind() == SyntaxKind::COMMENT {
                    doc = doc
                        .append(Doc::text(token.text().to_string()))
                        .append(Doc::space());
                } else if token.kind() == SyntaxKind::WHITESPACE {
                    continue;
                } else {
                    break;
                }
            }
        }
    }
    doc
}

fn leading_comments<'a>(node: &SyntaxNode) -> Doc<'a> {
    let mut doc = Doc::nil();
    for next in node.siblings_with_tokens(Direction::Prev).skip(1) {
        println!("prev");
        match next {
            rowan::NodeOrToken::Node(node) => {
                println!("before node {:?}", node);
                break;
            }
            rowan::NodeOrToken::Token(token) => {
                println!("before token {:?}", token);
                if token.kind() == SyntaxKind::COMMENT {
                    let is_block = token.text().starts_with("--");
                    doc = doc
                        .append(Doc::text(token.text().to_string()))
                        .append(if is_block {
                            Doc::hard_line()
                        } else {
                            Doc::space()
                        });
                } else if token.kind() == SyntaxKind::WHITESPACE {
                    continue;
                } else {
                    break;
                }
            }
        }
    }
    doc
}

fn trailing_comments<'a>(node: &SyntaxNode) -> Doc<'a> {
    let mut doc = Doc::nil();
    for next in node.siblings_with_tokens(Direction::Next).skip(1) {
        println!("after");
        match next {
            rowan::NodeOrToken::Node(node) => {
                println!("after node {:?}", node);
                break;
            }
            rowan::NodeOrToken::Token(token) => {
                println!("after token {:?}", token);
                if token.kind() == SyntaxKind::COMMENT {
                    doc = doc
                        .append(Doc::space())
                        .append(Doc::text(token.text().to_string()));
                } else if token.kind() == SyntaxKind::WHITESPACE {
                    continue;
                } else {
                    break;
                }
            }
        }
    }
    doc
}

fn build_target<'a>(target: ast::Target) -> Option<Doc<'a>> {
    let mut doc = leading_comments(target.syntax());

    if target.star_token().is_some() {
        return Some(doc.append(Doc::text("*")));
    }
    let expr = target.expr()?;
    doc = doc.append(build_expr(expr));

    if let Some(as_name) = target.as_name() {
        if as_name.as_token().is_some() {
            doc = doc.append(Doc::space()).append(Doc::text("as"))
        }

        if let Some(name) = as_name.name() {
            // TODO: quoting or not?
            doc = doc
                .append(Doc::space())
                .append(Doc::text(name.syntax().to_string()));
        }
    }

    doc = doc.append(trailing_comments(target.syntax()));

    Some(doc)
}

pub fn fmt(text: &str) -> String {
    let parse = ast::SourceFile::parse(text);
    let file = parse.tree();
    println!("{}", text);
    println!("---");
    println!("{:#?}", file.syntax());
    println!("---");
    debug_assert_eq!(
        parse.errors(),
        vec![],
        "should bail out when there's parse errors"
    );
    let doc = build_source_file(&file);
    print(&doc, &PrintOptions::default())
}
