use anyhow::Result;
use itertools::Itertools;
use rowan::Direction;
use squawk_line_index::{LineEnding, UniversalNewlines, find_newline};
use squawk_syntax::ast::{self, AstNode, LitKind, normalize_name_node};
use squawk_syntax::quote::{quote_bare_column_alias, quote_column_alias, quote_ident};
use squawk_syntax::{SyntaxElement, SyntaxKind, SyntaxNode, SyntaxToken};
use tiny_pretty::Doc;
use tiny_pretty::{LineBreak, PrintOptions, print};

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
                    let lines = token.text().universal_newlines().count();
                    if lines >= 2 {
                        doc = doc.append(Doc::empty_line()).append(Doc::empty_line());
                    } else {
                        doc = doc.append(Doc::empty_line());
                    }
                }
            }
        }
    }
    doc
}

fn build_create_table<'a>(create_table: &ast::CreateTable) -> Doc<'a> {
    let table_name = create_table.table_name().unwrap();
    let mut doc = Doc::text("create")
        .append(Doc::space())
        .append(Doc::text("table"))
        .append(Doc::space())
        .append(leading_comments(table_name.syntax()))
        .append(build_path(&table_name.path().unwrap()))
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
        .append(Doc::text(")"));

    doc = doc.append(build_semicolon(create_table.semicolon_token()));

    doc
}

fn build_path<'a>(path: &ast::Path) -> Doc<'a> {
    build_path_parts(path.qualifier(), path.dot_token(), path.segment())
}

fn build_path_ref<'a>(path: &ast::PathRef) -> Doc<'a> {
    build_path_parts(path.qualifier(), path.dot_token(), path.segment())
}

fn build_path_parts<'a>(
    qualifier: Option<ast::PathRef>,
    dot: Option<SyntaxToken>,
    segment: Option<impl AstNode>,
) -> Doc<'a> {
    let mut doc = Doc::nil();
    if let Some(qualifier) = qualifier {
        doc = doc
            .append(build_path_ref(&qualifier))
            .append(trailing_comments(qualifier.syntax()));
    }
    if dot.is_some() {
        doc = doc.append(Doc::text("."));
    }
    if let Some(segment) = segment {
        doc = doc
            .append(leading_comments(segment.syntax()))
            .append(build_name(segment.syntax()));
    }
    doc
}

fn build_name<'a>(node: &SyntaxNode) -> Doc<'a> {
    let mut tokens = node
        .children_with_tokens()
        .filter_map(|el| el.into_token())
        .filter(|token| token.kind() != SyntaxKind::WHITESPACE);

    let Some(ident) = tokens.next() else {
        return Doc::nil();
    };

    if is_unicode_escape(ident.text()) {
        let mut doc = Doc::text(ident.text().to_string());
        for token in tokens {
            let text = match token.kind() {
                SyntaxKind::STRING | SyntaxKind::COMMENT => token.text().to_string(),
                _ => token.text().to_ascii_lowercase(),
            };
            doc = doc.append(Doc::space()).append(Doc::text(text));
            if is_line_comment(&token) {
                doc = doc.append(Doc::hard_line());
            }
        }
        return doc;
    }

    Doc::text(quote_ident(&normalize_name_node(node)))
}

fn is_unicode_escape(text: &str) -> bool {
    text.strip_prefix(['u', 'U'])
        .is_some_and(|text| text.starts_with("&\""))
}

fn build_table_arg<'a>(arg: ast::TableArg) -> Doc<'a> {
    let doc = leading_comments(arg.syntax());
    let doc = doc.append(match &arg {
        ast::TableArg::Column(column) => {
            let mut doc = build_name(column.name().unwrap().syntax());
            if let Some(ty) = column.ty() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments(ty.syntax()))
                    .append(build_type(ty));
            }
            doc
        }
        ast::TableArg::LikeClause(like_clause) => build_like_clause(like_clause),
        ast::TableArg::TableConstraint(_table_constraint) => todo!(),
    });
    doc.append(trailing_comments(arg.syntax()))
}

fn build_like_clause<'a>(like_clause: &ast::LikeClause) -> Doc<'a> {
    let mut doc = Doc::text("like");

    if let Some(relation_name) = like_clause.relation_name_ref() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(relation_name.syntax()));
        if let Some(path) = relation_name.path_ref() {
            doc = doc.append(build_path_ref(&path));
        }
    }

    let options: Vec<Doc<'a>> = like_clause
        .like_options()
        .map(|option| {
            Doc::line_or_space()
                .append(leading_comments(option.syntax()))
                .append(build_like_option(&option))
        })
        .collect();
    if !options.is_empty() {
        doc = doc.append(Doc::list(options).nest(2).group());
    }

    doc
}

fn build_like_option<'a>(option: &ast::LikeOption) -> Doc<'a> {
    let (keyword, property) = match option {
        ast::LikeOption::ExcludingProperty(n) => ("excluding", n.table_property()),
        ast::LikeOption::IncludingProperty(n) => ("including", n.table_property()),
    };

    let mut doc = Doc::text(keyword);
    if let Some(property) = property {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(property.syntax()))
            .append(build_keyword_node(property.syntax()));
    }
    doc
}

fn build_select_doc<'a>(select: &ast::Select) -> Doc<'a> {
    let mut doc = Doc::text("select").append(Doc::line_or_space());

    if let Some(select_clause) = select.select_clause() {
        match select_clause.select_quantifier() {
            Some(ast::SelectQuantifier::DistinctClause(distinct_clause)) => {
                doc = doc.append(leading_comments(distinct_clause.syntax()));
                doc = doc.append(Doc::text("distinct")).append(Doc::space());
            }
            Some(ast::SelectQuantifier::All(all)) => {
                doc = doc.append(leading_comments(all.syntax()));
                doc = doc.append(Doc::text("all")).append(Doc::space());
            }
            None => (),
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

    if let Some(from) = select.from_clause() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(from.syntax()))
                .append(build_from_clause(from)),
        );
    }

    if let Some(group) = &select.group_by_clause() {
        let mut group_doc = Doc::line_or_space().append(leading_comments(group.syntax()));
        group_doc = group_doc.append(Doc::text("group")).append(Doc::space());
        if let Some(by_token) = group.by_token() {
            group_doc = group_doc.append(leading_comments_token(&by_token));
        }
        group_doc = group_doc.append(Doc::text("by")).append(Doc::space());
        if let Some(quantifier) = group.all_or_distinct() {
            group_doc = group_doc
                .append(leading_comments(quantifier.syntax()))
                .append(match quantifier {
                    ast::AllOrDistinct::All(_) => Doc::text("all"),
                    ast::AllOrDistinct::Distinct(_) => Doc::text("distinct"),
                })
                .append(Doc::space());
        }
        if let Some(list) = group.group_by_list() {
            group_doc = group_doc.append(build_group_by_list(list));
        }
        doc = doc.append(group_doc);
    }

    doc = doc.append(build_semicolon(select.semicolon_token()));

    doc.group()
}

fn build_from_clause<'a>(from: ast::FromClause) -> Doc<'a> {
    if from.join_exprs().next().is_some() {
        todo!("joins are not supported yet")
    }

    let from_items: Vec<_> = from
        .from_items()
        .map(|item| {
            let leading = leading_comments(item.syntax());
            let trailing = trailing_comments(item.syntax());
            leading.append(build_from_item(item)).append(trailing)
        })
        .collect();

    Doc::text("from").append(Doc::space()).append(
        Doc::list(
            Itertools::intersperse(
                from_items.into_iter(),
                Doc::text(",").append(Doc::line_or_space()),
            )
            .collect(),
        )
        .nest(2),
    )
}

fn build_from_item<'a>(item: ast::FromItem) -> Doc<'a> {
    match item {
        ast::FromItem::RelationFromItem(relation) => build_relation_from_item(relation),
        ast::FromItem::FunctionFromItem(_) => {
            todo!("function from items are not supported yet")
        }
        ast::FromItem::ExprFromItem(_) => todo!("expression from items are not supported yet"),
        ast::FromItem::ParenFromItem(_) => {
            todo!("parenthesized from items are not supported yet")
        }
        ast::FromItem::RowsFromItem(_) => todo!("rows from items are not supported yet"),
        ast::FromItem::GraphTableFromItem(_) => {
            todo!("graph_table from items are not supported yet")
        }
        ast::FromItem::JsonTableFromItem(_) => {
            todo!("json_table from items are not supported yet")
        }
        ast::FromItem::XmlTableFromItem(_) => {
            todo!("xmltable from items are not supported yet")
        }
    }
}

fn build_relation_from_item<'a>(relation: ast::RelationFromItem) -> Doc<'a> {
    let mut doc = if relation.only_token().is_some() {
        Doc::text("only").append(Doc::space())
    } else {
        Doc::nil()
    };

    if let Some(name) = relation.relation_name_ref() {
        doc = doc.append(leading_comments(name.syntax()));
        if let Some(path) = name.path_ref() {
            doc = doc.append(build_path_ref(&path));
        }
    }
    if let Some(star) = relation.star_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&star))
            .append(Doc::text("*"));
    }
    if let Some(tablesample) = relation.tablesample_clause() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(tablesample.syntax()))
            .append(build_tablesample_clause(tablesample));
    }
    doc.append(build_from_alias(relation.alias()))
}

fn build_tablesample_clause<'a>(tablesample: ast::TablesampleClause) -> Doc<'a> {
    let mut doc = Doc::text("tablesample").append(Doc::space());
    if let Some(call) = tablesample.call_expr() {
        doc = doc
            .append(leading_comments(call.syntax()))
            .append(build_call_expr(call));
    }
    if let Some(repeatable) = tablesample.repeatable_clause() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(repeatable.syntax()))
            .append(Doc::text("repeatable"));
        if let Some(l_paren) = repeatable.l_paren_token() {
            doc = doc.append(comments_before(l_paren));
        }
        doc = doc.append(Doc::text("("));
        if let Some(expr) = repeatable.expr() {
            doc = doc
                .append(leading_comments(expr.syntax()))
                .append(build_expr(expr));
        }
        if let Some(r_paren) = repeatable.r_paren_token() {
            doc = doc.append(comments_before(r_paren));
        }
        doc = doc.append(Doc::text(")"));
    }
    doc
}

fn build_from_alias<'a>(alias: Option<ast::FromAlias>) -> Doc<'a> {
    let Some(alias) = alias else {
        return Doc::nil();
    };
    let mut doc = Doc::space().append(leading_comments(alias.syntax()));
    if alias.as_token().is_some() {
        doc = doc.append(Doc::text("as")).append(Doc::space());
    }
    if let Some(name) = alias.name() {
        doc = doc
            .append(leading_comments(name.syntax()))
            .append(build_name(name.syntax()));
    }
    if let Some(columns) = alias.columns() {
        doc = doc.append(build_from_alias_columns(columns));
    }
    doc
}

fn build_from_alias_columns<'a>(columns: ast::FromAliasColumns) -> Doc<'a> {
    match columns {
        ast::FromAliasColumns::ColumnList(list) => {
            let items = list
                .column_names()
                .map(|name| {
                    leading_comments(name.syntax())
                        .append(build_name(name.syntax()))
                        .append(trailing_comments(name.syntax()))
                })
                .collect();
            comments_before(list.syntax().clone()).append(build_from_alias_column_list(
                list.l_paren_token(),
                items,
                list.r_paren_token(),
            ))
        }
        ast::FromAliasColumns::ColumnDefList(list) => {
            let items = list
                .column_defs()
                .map(|column| {
                    let mut doc = leading_comments(column.syntax());
                    if let Some(name) = column.name() {
                        doc = doc.append(build_name(name.syntax()));
                    }
                    if let Some(ty) = column.ty() {
                        doc = doc
                            .append(Doc::space())
                            .append(leading_comments(ty.syntax()))
                            .append(build_type(ty));
                    }
                    if let Some(collate) = column.collate() {
                        doc = doc
                            .append(Doc::space())
                            .append(leading_comments(collate.syntax()))
                            .append(build_collate_expr(collate));
                    }
                    doc.append(trailing_comments(column.syntax()))
                })
                .collect();
            comments_before(list.syntax().clone()).append(build_from_alias_column_list(
                list.l_paren_token(),
                items,
                list.r_paren_token(),
            ))
        }
    }
}

fn build_from_alias_column_list<'a>(
    l_paren: Option<SyntaxToken>,
    items: Vec<Doc<'a>>,
    r_paren: Option<SyntaxToken>,
) -> Doc<'a> {
    let mut doc = Doc::nil();
    if let Some(l_paren) = l_paren {
        doc = doc.append(comments_before(l_paren));
    }
    doc = doc.append(Doc::text("("));
    if items.is_empty() {
        if let Some(r_paren) = r_paren {
            doc = doc.append(comments_before(r_paren));
        }
    } else {
        doc = doc.append(
            Doc::list(
                Itertools::intersperse(
                    items.into_iter(),
                    Doc::text(",").append(Doc::line_or_space()),
                )
                .collect(),
            )
            .nest(2),
        );
    }
    doc.append(Doc::text(")")).group()
}

fn build_group_by_list<'a>(list: ast::GroupByList) -> Doc<'a> {
    leading_comments(list.syntax()).append(build_group_bys(list.group_bys()))
}

fn build_group_bys<'a>(group_bys: impl Iterator<Item = ast::GroupBy>) -> Doc<'a> {
    Doc::list(
        Itertools::intersperse(
            group_bys.map(|group_by| {
                let leading = leading_comments(group_by.syntax());
                let trailing = trailing_comments(group_by.syntax());
                leading.append(build_group_by(group_by)).append(trailing)
            }),
            Doc::text(",").append(Doc::line_or_space()),
        )
        .collect(),
    )
    .nest(2)
}

fn build_group_by<'a>(group_by: ast::GroupBy) -> Doc<'a> {
    match group_by {
        ast::GroupBy::GroupingExpr(grouping_expr) => grouping_expr
            .expr()
            .map(build_expr)
            .unwrap_or_else(Doc::nil),
        ast::GroupBy::GroupingRollup(rollup) => Doc::text("rollup").append(build_grouping_exprs(
            rollup.l_paren_token(),
            rollup.exprs(),
            rollup.r_paren_token(),
        )),
        ast::GroupBy::GroupingCube(cube) => Doc::text("cube").append(build_grouping_exprs(
            cube.l_paren_token(),
            cube.exprs(),
            cube.r_paren_token(),
        )),
        ast::GroupBy::GroupingSets(sets) => {
            let mut doc = Doc::text("grouping").append(Doc::space());
            if let Some(sets_token) = sets.sets_token() {
                doc = doc.append(leading_comments_token(&sets_token));
            }
            doc.append(Doc::text("sets"))
                .append(build_grouping_group_bys(
                    sets.l_paren_token(),
                    sets.group_bys(),
                    sets.r_paren_token(),
                ))
        }
    }
}

fn build_grouping_exprs<'a>(
    l_paren: Option<SyntaxToken>,
    exprs: impl Iterator<Item = ast::Expr>,
    r_paren: Option<SyntaxToken>,
) -> Doc<'a> {
    let exprs: Vec<_> = exprs
        .map(|expr| {
            let leading = leading_comments(expr.syntax());
            let trailing = trailing_comments(expr.syntax());
            leading.append(build_expr(expr)).append(trailing)
        })
        .collect();
    build_grouping_list(l_paren, exprs, r_paren)
}

fn build_grouping_group_bys<'a>(
    l_paren: Option<SyntaxToken>,
    group_bys: impl Iterator<Item = ast::GroupBy>,
    r_paren: Option<SyntaxToken>,
) -> Doc<'a> {
    let group_bys = group_bys
        .map(|group_by| {
            let leading = leading_comments(group_by.syntax());
            let trailing = trailing_comments(group_by.syntax());
            leading.append(build_group_by(group_by)).append(trailing)
        })
        .collect();
    build_grouping_list(l_paren, group_bys, r_paren)
}

fn build_grouping_list<'a>(
    l_paren: Option<SyntaxToken>,
    items: Vec<Doc<'a>>,
    r_paren: Option<SyntaxToken>,
) -> Doc<'a> {
    let mut doc = Doc::nil();
    if let Some(l_paren) = l_paren {
        doc = doc.append(comments_before(l_paren));
    }
    doc = doc.append(Doc::text("("));

    if items.is_empty() {
        if let Some(r_paren) = r_paren {
            doc = doc.append(comments_before(r_paren));
        }
    } else {
        doc = doc.append(
            Doc::list(
                Itertools::intersperse(
                    items.into_iter(),
                    Doc::text(",").append(Doc::line_or_space()),
                )
                .collect(),
            )
            .nest(2),
        );
    }

    doc.append(Doc::text(")")).group()
}

fn build_semicolon<'a>(semi: Option<SyntaxToken>) -> Doc<'a> {
    let Some(semi) = semi else {
        return Doc::nil();
    };
    let mut doc = Doc::nil();
    for comment in comment_tokens_before(semi) {
        doc = doc.append(Doc::text(comment.text().to_string()));
        if is_line_comment(&comment) {
            doc = doc.append(Doc::hard_line());
        }
    }
    doc.append(Doc::text(";"))
}

fn build_expr<'a>(expr: ast::Expr) -> Doc<'a> {
    match expr {
        ast::Expr::ArrayExpr(array_expr) => build_array_expr(array_expr),
        ast::Expr::BetweenExpr(between_expr) => build_between_expr(between_expr),
        ast::Expr::BinExpr(bin_expr) => build_bin_expr(bin_expr),
        ast::Expr::CallExpr(call_expr) => build_call_expr(call_expr),
        ast::Expr::CaseExpr(case_expr) => build_case_expr(case_expr),
        ast::Expr::CastExpr(cast_expr) => build_cast_expr(cast_expr),
        ast::Expr::Collate(collate) => build_collate_expr(collate),
        ast::Expr::FieldExpr(field_expr) => build_field_expr(field_expr),
        ast::Expr::IndexExpr(index_expr) => build_index_expr(index_expr),
        ast::Expr::Literal(literal) => build_literal(literal),
        ast::Expr::NameRef(name_ref) => build_name(name_ref.syntax()),
        ast::Expr::ParenExpr(paren_expr) => build_paren_expr(paren_expr),
        ast::Expr::PostfixExpr(postfix_expr) => build_postfix_expr(postfix_expr),
        ast::Expr::PrefixExpr(prefix_expr) => build_prefix_expr(prefix_expr),
        ast::Expr::SliceExpr(slice_expr) => build_slice_expr(slice_expr),
        ast::Expr::TupleExpr(tuple_expr) => build_tuple_expr(tuple_expr),
    }
}

fn build_array_expr<'a>(array_expr: ast::ArrayExpr) -> Doc<'a> {
    let mut doc = Doc::nil();

    // nested parts of array expressions don't require the array token
    if array_expr.array_token().is_some() {
        doc = doc.append(Doc::text("array"));
    };

    if let Some(select) = array_expr.select() {
        doc.append(Doc::text("("))
            .append(build_select_doc(&select))
            .append(Doc::text(")"))
    } else {
        doc.append(Doc::text("["))
            .append(Doc::list(
                Itertools::intersperse(
                    array_expr.exprs().map(build_expr),
                    Doc::text(",").append(Doc::space()),
                )
                .collect(),
            ))
            .append(Doc::text("]"))
    }
}

fn build_field_expr<'a>(field_expr: ast::FieldExpr) -> Doc<'a> {
    let mut doc = match field_expr.base() {
        Some(base) => build_expr(base),
        None => Doc::nil(),
    };

    if let Some(dot) = field_expr.dot_token() {
        doc = doc.append(comments_before(dot));
    }
    doc = doc.append(Doc::text("."));

    if let Some(star) = field_expr.star_token() {
        doc = doc
            .append(leading_comments_token(&star))
            .append(Doc::text("*"));
    } else if let Some(field) = field_expr.field() {
        doc = doc
            .append(leading_comments(field.syntax()))
            .append(build_name(field.syntax()));
    }

    doc
}

fn build_index_expr<'a>(index_expr: ast::IndexExpr) -> Doc<'a> {
    let mut doc = match index_expr.base() {
        Some(base) => build_expr(base),
        None => Doc::nil(),
    };

    if let Some(l_brack) = index_expr.l_brack_token() {
        doc = doc.append(comments_before(l_brack));
    }
    doc = doc.append(Doc::text("["));

    if let Some(index) = index_expr.index() {
        doc = doc
            .append(leading_comments(index.syntax()))
            .append(build_expr(index));
    }
    if let Some(r_brack) = index_expr.r_brack_token() {
        doc = doc.append(comments_before(r_brack));
    }
    doc.append(Doc::text("]"))
}

fn build_slice_expr<'a>(slice_expr: ast::SliceExpr) -> Doc<'a> {
    let mut doc = match slice_expr.base() {
        Some(base) => build_expr(base),
        None => Doc::nil(),
    };

    if let Some(l_brack) = slice_expr.l_brack_token() {
        doc = doc.append(comments_before(l_brack));
    }
    doc = doc.append(Doc::text("["));

    if let Some(start) = slice_expr.start() {
        doc = doc
            .append(leading_comments(start.syntax()))
            .append(build_expr(start));
    }
    if let Some(colon) = slice_expr.colon_token() {
        doc = doc.append(comments_before(colon));
    }
    doc = doc.append(Doc::text(":"));

    if let Some(end) = slice_expr.end() {
        doc = doc
            .append(leading_comments(end.syntax()))
            .append(build_expr(end));
    }
    if let Some(r_brack) = slice_expr.r_brack_token() {
        doc = doc.append(comments_before(r_brack));
    }
    doc.append(Doc::text("]"))
}

fn build_tuple_expr<'a>(tuple_expr: ast::TupleExpr) -> Doc<'a> {
    let mut doc = if tuple_expr.row_token().is_some() {
        Doc::text("row")
    } else {
        Doc::nil()
    };

    if let Some(l_paren) = tuple_expr.l_paren_token() {
        doc = doc.append(comments_before(l_paren));
    }
    doc = doc.append(Doc::text("("));

    if let Some(exprs) = build_comma_separated_exprs(tuple_expr.exprs()) {
        doc = doc.append(exprs);
    } else if let Some(r_paren) = tuple_expr.r_paren_token() {
        doc = doc.append(comments_before(r_paren));
    }

    doc.append(Doc::text(")"))
}

fn build_between_expr<'a>(between_expr: ast::BetweenExpr) -> Doc<'a> {
    let mut doc = build_expr(between_expr.target().unwrap());
    if between_expr.not_token().is_some() {
        doc = doc.append(Doc::space()).append(Doc::text("not"));
    }
    doc = doc.append(Doc::space()).append(Doc::text("between"));
    match between_expr.between_symmetry() {
        Some(ast::BetweenSymmetry::Asymmetric(_)) => {
            doc = doc.append(Doc::space()).append(Doc::text("asymmetric"));
        }
        Some(ast::BetweenSymmetry::Symmetric(_)) => {
            doc = doc.append(Doc::space()).append(Doc::text("symmetric"));
        }
        None => (),
    }
    doc.append(Doc::space())
        .append(build_expr(between_expr.start().unwrap()))
        .append(Doc::space())
        .append(Doc::text("and"))
        .append(Doc::space())
        .append(build_expr(between_expr.end().unwrap()))
}

fn build_call_expr<'a>(call_expr: ast::CallExpr) -> Doc<'a> {
    if let (Some(expr), Some(arg_list)) = (call_expr.expr(), call_expr.arg_list()) {
        let mut doc = build_expr(expr)
            .append(comments_before(arg_list.syntax().clone()))
            .append(build_call_arg_list(arg_list));
        if let Some(within_clause) = call_expr.within_clause() {
            doc = doc
                .append(Doc::space())
                .append(leading_comments(within_clause.syntax()))
                .append(build_within_clause(within_clause));
        }
        if let Some(filter_clause) = call_expr.filter_clause() {
            doc = doc
                .append(Doc::space())
                .append(leading_comments(filter_clause.syntax()))
                .append(build_filter_clause(filter_clause));
        }
        if let Some(null_treatment) = call_expr.null_treatment() {
            doc = doc
                .append(Doc::space())
                .append(leading_comments(null_treatment.syntax()))
                .append(build_null_treatment(null_treatment));
        }
        if let Some(over_clause) = call_expr.over_clause() {
            doc = doc
                .append(Doc::space())
                .append(leading_comments(over_clause.syntax()))
                .append(build_over_clause(over_clause));
        }
        doc
    } else if let Some(all_fn) = call_expr.all_fn() {
        build_parenthesized_expr_or_select_fn(
            "all",
            all_fn.l_paren_token(),
            all_fn.expr(),
            all_fn.select_variant(),
            all_fn.r_paren_token(),
        )
    } else if let Some(any_fn) = call_expr.any_fn() {
        build_parenthesized_expr_or_select_fn(
            "any",
            any_fn.l_paren_token(),
            any_fn.expr(),
            any_fn.select_variant(),
            any_fn.r_paren_token(),
        )
    } else if let Some(_collation_for_fn) = call_expr.collation_for_fn() {
        todo!("collation_for function expressions are not supported yet")
    } else if let Some(exists_fn) = call_expr.exists_fn() {
        build_parenthesized_expr_or_select_fn(
            "exists",
            exists_fn.l_paren_token(),
            None,
            exists_fn.select_variant(),
            exists_fn.r_paren_token(),
        )
    } else if let Some(extract_fn) = call_expr.extract_fn() {
        build_extract_fn(extract_fn)
    } else if let Some(_graph_table_fn) = call_expr.graph_table_fn() {
        todo!("graph_table function expressions are not supported yet")
    } else if let Some(json_array_agg_fn) = call_expr.json_array_agg_fn() {
        build_json_array_agg_fn(json_array_agg_fn)
    } else if let Some(json_array_fn) = call_expr.json_array_fn() {
        build_json_array_fn(json_array_fn)
    } else if let Some(_json_exists_fn) = call_expr.json_exists_fn() {
        todo!("json_exists function expressions are not supported yet")
    } else if let Some(_json_fn) = call_expr.json_fn() {
        todo!("json function expressions are not supported yet")
    } else if let Some(_json_object_agg_fn) = call_expr.json_object_agg_fn() {
        todo!("json_objectagg function expressions are not supported yet")
    } else if let Some(_json_object_fn) = call_expr.json_object_fn() {
        todo!("json_object function expressions are not supported yet")
    } else if let Some(_json_query_fn) = call_expr.json_query_fn() {
        todo!("json_query function expressions are not supported yet")
    } else if let Some(_json_scalar_fn) = call_expr.json_scalar_fn() {
        todo!("json_scalar function expressions are not supported yet")
    } else if let Some(_json_serialize_fn) = call_expr.json_serialize_fn() {
        todo!("json_serialize function expressions are not supported yet")
    } else if let Some(_json_value_fn) = call_expr.json_value_fn() {
        todo!("json_value function expressions are not supported yet")
    } else if let Some(_overlay_fn) = call_expr.overlay_fn() {
        todo!("overlay function expressions are not supported yet")
    } else if let Some(position_fn) = call_expr.position_fn() {
        build_position_fn(position_fn)
    } else if let Some(some_fn) = call_expr.some_fn() {
        build_parenthesized_expr_or_select_fn(
            "some",
            some_fn.l_paren_token(),
            some_fn.expr(),
            some_fn.select_variant(),
            some_fn.r_paren_token(),
        )
    } else if let Some(substring_fn) = call_expr.substring_fn() {
        build_substring_fn(substring_fn)
    } else if let Some(trim_fn) = call_expr.trim_fn() {
        build_trim_fn(trim_fn)
    } else if let Some(_xml_element_fn) = call_expr.xml_element_fn() {
        todo!("xmlelement function expressions are not supported yet")
    } else if let Some(_xml_exists_fn) = call_expr.xml_exists_fn() {
        todo!("xmlexists function expressions are not supported yet")
    } else if let Some(_xml_forest_fn) = call_expr.xml_forest_fn() {
        todo!("xmlforest function expressions are not supported yet")
    } else if let Some(_xml_parse_fn) = call_expr.xml_parse_fn() {
        todo!("xmlparse function expressions are not supported yet")
    } else if let Some(_xml_pi_fn) = call_expr.xml_pi_fn() {
        todo!("xmlpi function expressions are not supported yet")
    } else if let Some(_xml_root_fn) = call_expr.xml_root_fn() {
        todo!("xmlroot function expressions are not supported yet")
    } else if let Some(_xml_serialize_fn) = call_expr.xml_serialize_fn() {
        todo!("xmlserialize function expressions are not supported yet")
    } else {
        unreachable!("a call expression should contain a supported function node")
    }
}

fn build_json_array_fn<'a>(json_array_fn: ast::JsonArrayFn) -> Doc<'a> {
    let mut doc = Doc::text("json_array");
    if let Some(l_paren) = json_array_fn.l_paren_token() {
        doc = doc.append(comments_before(l_paren));
    }
    doc = doc.append(Doc::text("("));

    let exprs = json_array_fn.json_expr_formats().map(|value| {
        (
            leading_comments(value.syntax()).append(build_json_expr_format(value.clone())),
            value.syntax().clone(),
        )
    });
    let selects = json_array_fn.json_select_formats().map(|select| {
        (
            leading_comments(select.syntax()).append(build_json_select_format(select.clone())),
            select.syntax().clone(),
        )
    });
    let mut items = exprs.chain(selects);
    if let Some((first, mut previous_syntax)) = items.next() {
        let mut item_docs = vec![first];
        for (item, syntax) in items {
            item_docs.push(
                trailing_comments(&previous_syntax)
                    .append(Doc::text(","))
                    .append(Doc::space())
                    .append(item),
            );
            previous_syntax = syntax;
        }
        doc = doc.append(Doc::list(item_docs));
    }

    if let Some(null_clause) = json_array_fn.json_null_clause() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(null_clause.syntax()))
            .append(build_json_null_clause(null_clause));
    }
    if let Some(returning) = json_array_fn.json_returning_clause() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(returning.syntax()))
            .append(build_json_returning_clause(returning));
    }
    if let Some(r_paren) = json_array_fn.r_paren_token() {
        doc = doc.append(comments_before(r_paren));
    }
    doc.append(Doc::text(")"))
}

fn build_json_expr_format<'a>(value: ast::JsonExprFormat) -> Doc<'a> {
    let mut doc = value.expr().map(build_expr).unwrap_or_else(Doc::nil);
    if let Some(format) = value.json_format_clause() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(format.syntax()))
            .append(build_json_format_clause(format));
    }
    doc
}

fn build_json_select_format<'a>(select: ast::JsonSelectFormat) -> Doc<'a> {
    let mut doc = select
        .select_variant()
        .map(|select| match select {
            ast::SelectVariant::Select(select) => build_select_doc(&select),
            _ => todo!("this select variant is not supported yet"),
        })
        .unwrap_or_else(Doc::nil);
    if let Some(format) = select.json_format_clause() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(format.syntax()))
            .append(build_json_format_clause(format));
    }
    doc
}

fn build_json_array_agg_fn<'a>(json_array_agg_fn: ast::JsonArrayAggFn) -> Doc<'a> {
    let mut doc = Doc::text("json_arrayagg");
    if let Some(l_paren) = json_array_agg_fn.l_paren_token() {
        doc = doc.append(comments_before(l_paren));
    }
    doc = doc.append(Doc::text("("));

    if let Some(value) = json_array_agg_fn.json_value_expr() {
        doc = doc
            .append(leading_comments(value.syntax()))
            .append(build_json_value_expr(value));
    }
    if let Some(order_by) = json_array_agg_fn.order_by_clause() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(order_by.syntax()))
            .append(build_order_by_clause(order_by));
    }
    if let Some(null_clause) = json_array_agg_fn.json_null_clause() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(null_clause.syntax()))
            .append(build_json_null_clause(null_clause));
    }
    if let Some(returning) = json_array_agg_fn.json_returning_clause() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(returning.syntax()))
            .append(build_json_returning_clause(returning));
    }
    if let Some(r_paren) = json_array_agg_fn.r_paren_token() {
        doc = doc.append(comments_before(r_paren));
    }
    doc.append(Doc::text(")"))
}

fn build_json_value_expr<'a>(value: ast::JsonValueExpr) -> Doc<'a> {
    let mut doc = value.expr().map(build_expr).unwrap_or_else(Doc::nil);
    if let Some(format) = value.json_format_clause() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(format.syntax()))
            .append(build_json_format_clause(format));
    }
    doc
}

fn build_json_format_clause<'a>(format: ast::JsonFormatClause) -> Doc<'a> {
    let mut doc = Doc::text("format");
    if let Some(json_token) = format.json_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&json_token))
            .append(Doc::text("json"));
    }
    if let Some(encoding) = format.json_encoding_clause() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(encoding.syntax()))
            .append(build_json_encoding_clause(encoding));
    }
    doc
}

fn build_json_encoding_clause<'a>(clause: ast::JsonEncodingClause) -> Doc<'a> {
    let mut doc = Doc::text("encoding");
    if let Some(encoding) = clause.json_encoding() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(encoding.syntax()))
            .append(build_name(encoding.syntax()));
    }
    doc
}

fn build_json_null_clause<'a>(clause: ast::JsonNullClause) -> Doc<'a> {
    let (prefix, on_token, null_token) = match clause {
        ast::JsonNullClause::JsonAbsentOnNull(clause) => {
            ("absent", clause.on_token(), clause.null_token())
        }
        ast::JsonNullClause::JsonNullOnNull(clause) => {
            ("null", clause.on_token(), clause.null_token())
        }
    };

    let mut doc = Doc::text(prefix);
    if let Some(on_token) = on_token {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&on_token))
            .append(Doc::text("on"));
    }
    if let Some(null_token) = null_token {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&null_token))
            .append(Doc::text("null"));
    }
    doc
}

fn build_json_returning_clause<'a>(returning: ast::JsonReturningClause) -> Doc<'a> {
    let mut doc = Doc::text("returning");
    if let Some(ty) = returning.ty() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(ty.syntax()))
            .append(build_type(ty));
    }
    if let Some(format) = returning.json_format_clause() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(format.syntax()))
            .append(build_json_format_clause(format));
    }
    doc
}

fn build_substring_fn<'a>(substring_fn: ast::SubstringFn) -> Doc<'a> {
    let mut doc = Doc::text("substring");
    if let Some(l_paren) = substring_fn.l_paren_token() {
        doc = doc.append(comments_before(l_paren));
    }
    doc = doc.append(Doc::text("("));

    if let Some(args) = substring_fn.substring_args() {
        doc = doc
            .append(leading_comments(args.syntax()))
            .append(match args {
                ast::SubstringArgs::SubstringForFrom(args) => {
                    let mut doc = args.string().map(build_expr).unwrap_or_else(Doc::nil);
                    doc = append_keyword_expr(doc, args.for_token(), "for", args.count());
                    append_keyword_expr(doc, args.from_token(), "from", args.start())
                }
                ast::SubstringArgs::SubstringFromFor(args) => {
                    let mut doc = args.string().map(build_expr).unwrap_or_else(Doc::nil);
                    doc = append_keyword_expr(doc, args.from_token(), "from", args.start());
                    append_keyword_expr(doc, args.for_token(), "for", args.count())
                }
                ast::SubstringArgs::SubstringSimilarEscape(args) => {
                    let mut doc = args.string().map(build_expr).unwrap_or_else(Doc::nil);
                    doc = append_keyword_expr(doc, args.similar_token(), "similar", args.pattern());
                    append_keyword_expr(doc, args.escape_token(), "escape", args.escape())
                }
                ast::SubstringArgs::SubstringExprs(args) => {
                    build_comma_separated_exprs(args.exprs()).unwrap_or_else(Doc::nil)
                }
            });
    }

    if let Some(r_paren) = substring_fn.r_paren_token() {
        doc = doc.append(comments_before(r_paren));
    }
    doc.append(Doc::text(")"))
}

fn append_keyword_expr<'a>(
    mut doc: Doc<'a>,
    token: Option<SyntaxToken>,
    keyword: &'static str,
    expr: Option<ast::Expr>,
) -> Doc<'a> {
    if let Some(token) = token {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&token))
            .append(Doc::text(keyword));
    }
    if let Some(expr) = expr {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(expr.syntax()))
            .append(build_expr(expr));
    }
    doc
}

fn build_trim_fn<'a>(trim_fn: ast::TrimFn) -> Doc<'a> {
    let mut doc = Doc::text("trim");
    if let Some(l_paren) = trim_fn.l_paren_token() {
        doc = doc.append(comments_before(l_paren));
    }
    doc = doc.append(Doc::text("("));

    let has_side = if let Some(side) = trim_fn.trim_side() {
        doc = doc
            .append(leading_comments(side.syntax()))
            .append(match side {
                ast::TrimSide::TrimBoth(_) => Doc::text("both"),
                ast::TrimSide::TrimLeading(_) => Doc::text("leading"),
                ast::TrimSide::TrimTrailing(_) => Doc::text("trailing"),
            });
        true
    } else {
        false
    };

    if let Some(args) = trim_fn.trim_args() {
        if has_side {
            doc = doc.append(Doc::space());
        }
        doc = doc
            .append(leading_comments(args.syntax()))
            .append(match args {
                ast::TrimArgs::TrimFrom(args) => {
                    let mut doc = Doc::text("from");
                    if let Some(exprs) = build_comma_separated_exprs(args.exprs()) {
                        doc = doc.append(Doc::space()).append(exprs);
                    }
                    doc
                }
                ast::TrimArgs::TrimExprFrom(args) => {
                    let mut exprs = args.exprs();
                    let mut doc = exprs.next().map(build_expr).unwrap_or_else(Doc::nil);
                    if let Some(from) = args.from_token() {
                        doc = doc
                            .append(Doc::space())
                            .append(leading_comments_token(&from))
                            .append(Doc::text("from"));
                    }
                    if let Some(exprs) = build_comma_separated_exprs(exprs) {
                        doc = doc.append(Doc::space()).append(exprs);
                    }
                    doc
                }
                ast::TrimArgs::TrimExprs(args) => {
                    build_comma_separated_exprs(args.exprs()).unwrap_or_else(Doc::nil)
                }
            });
    }

    if let Some(r_paren) = trim_fn.r_paren_token() {
        doc = doc.append(comments_before(r_paren));
    }
    doc.append(Doc::text(")"))
}

fn build_comma_separated_exprs<'a>(exprs: impl Iterator<Item = ast::Expr>) -> Option<Doc<'a>> {
    let exprs: Vec<Doc<'a>> = exprs
        .map(|expr| {
            let leading = leading_comments(expr.syntax());
            let trailing = trailing_comments(expr.syntax());
            leading.append(build_expr(expr)).append(trailing)
        })
        .collect();
    if exprs.is_empty() {
        None
    } else {
        Some(Doc::list(
            Itertools::intersperse(exprs.into_iter(), Doc::text(",").append(Doc::space()))
                .collect(),
        ))
    }
}

fn build_position_fn<'a>(position_fn: ast::PositionFn) -> Doc<'a> {
    let mut doc = Doc::text("position");
    if let Some(l_paren) = position_fn.l_paren_token() {
        doc = doc.append(comments_before(l_paren));
    }
    doc = doc.append(Doc::text("("));

    if let Some(pos) = position_fn.pos() {
        doc = doc
            .append(leading_comments(pos.syntax()))
            .append(build_expr(pos));
    }
    if let Some(in_token) = position_fn.in_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&in_token))
            .append(Doc::text("in"));
    }
    if let Some(string) = position_fn.string() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(string.syntax()))
            .append(build_expr(string));
    }
    if let Some(r_paren) = position_fn.r_paren_token() {
        doc = doc.append(comments_before(r_paren));
    }
    doc.append(Doc::text(")"))
}

fn build_extract_fn<'a>(extract_fn: ast::ExtractFn) -> Doc<'a> {
    let mut doc = Doc::text("extract");
    if let Some(l_paren) = extract_fn.l_paren_token() {
        doc = doc.append(comments_before(l_paren));
    }
    doc = doc.append(Doc::text("("));

    if let Some(field) = extract_fn.extract_field() {
        doc = doc
            .append(leading_comments(field.syntax()))
            .append(match field {
                ast::ExtractField::ExtractFieldLiteral(field) => {
                    field.literal().map(build_literal).unwrap_or_else(Doc::nil)
                }
                ast::ExtractField::ExtractFieldName(field) => {
                    if field.ident_token().is_some() {
                        build_name(field.syntax())
                    } else {
                        build_keyword_node(field.syntax())
                    }
                }
            });
    }

    if let Some(from) = extract_fn.from_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&from))
            .append(Doc::text("from"));
    }
    if let Some(expr) = extract_fn.expr() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(expr.syntax()))
            .append(build_expr(expr));
    }
    if let Some(r_paren) = extract_fn.r_paren_token() {
        doc = doc.append(comments_before(r_paren));
    }
    doc.append(Doc::text(")"))
}

fn build_parenthesized_expr_or_select_fn<'a>(
    keyword: &'static str,
    l_paren: Option<SyntaxToken>,
    expr: Option<ast::Expr>,
    select: Option<ast::SelectVariant>,
    r_paren: Option<SyntaxToken>,
) -> Doc<'a> {
    let mut doc = Doc::text(keyword);
    if let Some(l_paren) = l_paren {
        doc = doc.append(comments_before(l_paren));
    }
    doc = doc.append(Doc::text("("));

    if let Some(expr) = expr {
        doc = doc
            .append(leading_comments(expr.syntax()))
            .append(build_expr(expr));
    } else if let Some(select) = select {
        doc = doc
            .append(leading_comments(select.syntax()))
            .append(match select {
                ast::SelectVariant::Select(select) => build_select_doc(&select),
                _ => todo!("this select variant is not supported yet"),
            });
    }

    if let Some(r_paren) = r_paren {
        doc = doc.append(comments_before(r_paren));
    }
    doc.append(Doc::text(")"))
}

fn build_call_arg_list<'a>(arg_list: ast::ArgList) -> Doc<'a> {
    let mut doc = Doc::nil();
    if let Some(l_paren) = arg_list.l_paren_token() {
        doc = doc.append(comments_before(l_paren));
    }
    doc = doc.append(Doc::text("("));

    if let Some(star) = arg_list.star_token() {
        doc = doc
            .append(leading_comments_token(&star))
            .append(Doc::text("*"));
        if let Some(r_paren) = arg_list.r_paren_token() {
            doc = doc.append(comments_before(r_paren));
        }
        return doc.append(Doc::text(")"));
    }

    let mut has_quantifier = false;
    if let Some(quantifier) = arg_list.all_or_distinct() {
        has_quantifier = true;
        doc = doc
            .append(leading_comments(quantifier.syntax()))
            .append(match quantifier {
                ast::AllOrDistinct::All(_) => Doc::text("all"),
                ast::AllOrDistinct::Distinct(_) => Doc::text("distinct"),
            });
    }

    let args: Vec<Doc<'a>> = arg_list
        .args()
        .map(|arg| {
            let leading = leading_comments(arg.syntax());
            let trailing = trailing_comments(arg.syntax());
            leading.append(build_call_arg(arg)).append(trailing)
        })
        .collect();
    if args.is_empty() {
        if let Some(r_paren) = arg_list.r_paren_token() {
            doc = doc.append(comments_before(r_paren));
        }
    } else {
        if has_quantifier {
            doc = doc.append(Doc::space());
        }
        doc = doc.append(Doc::list(
            Itertools::intersperse(args.into_iter(), Doc::text(",").append(Doc::space())).collect(),
        ));
    }

    doc.append(Doc::text(")"))
}

fn build_call_arg<'a>(arg: ast::Arg) -> Doc<'a> {
    let mut doc = if let Some(named_arg) = arg.named_arg() {
        build_named_call_arg(named_arg)
    } else {
        let mut doc = Doc::nil();
        if arg.variadic_token().is_some() {
            doc = doc.append(Doc::text("variadic")).append(Doc::space());
        }
        if let Some(expr) = arg.expr() {
            doc = doc
                .append(leading_comments(expr.syntax()))
                .append(build_expr(expr));
        }
        doc
    };
    if let Some(order_by_clause) = arg.order_by_clause() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(order_by_clause.syntax()))
            .append(build_order_by_clause(order_by_clause));
    }
    doc
}

fn build_order_by_clause<'a>(clause: ast::OrderByClause) -> Doc<'a> {
    let mut doc = Doc::text("order").append(Doc::space());
    if let Some(by_token) = clause.by_token() {
        doc = doc.append(leading_comments_token(&by_token));
    }
    doc = doc.append(Doc::text("by"));

    if let Some(list) = clause.sort_by_list() {
        let items = list
            .sort_bys()
            .map(|sort_by| leading_comments(sort_by.syntax()).append(build_sort_by(sort_by)));
        doc = doc
            .append(Doc::space())
            .append(leading_comments(list.syntax()))
            .append(Doc::list(
                Itertools::intersperse(items, Doc::text(",").append(Doc::space())).collect(),
            ));
    }
    doc
}

fn build_sort_by<'a>(sort_by: ast::SortBy) -> Doc<'a> {
    let mut doc = Doc::nil();
    if let Some(expr) = sort_by.expr() {
        doc = doc
            .append(leading_comments(expr.syntax()))
            .append(build_expr(expr));
    }

    if let Some(order) = sort_by.sort_order() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(order.syntax()))
            .append(match order {
                ast::SortOrder::SortAsc(_) => Doc::text("asc"),
                ast::SortOrder::SortDesc(_) => Doc::text("desc"),
                ast::SortOrder::SortUsing(using) => {
                    let mut doc = Doc::text("using");
                    if let Some(operator_call) = using.operator_call() {
                        doc = doc
                            .append(Doc::space())
                            .append(leading_comments(operator_call.syntax()))
                            .append(build_operator_call(&operator_call));
                    } else if let Some(op) = using.op() {
                        doc = doc
                            .append(Doc::space())
                            .append(leading_comments(op.syntax()))
                            .append(build_operator(&op));
                    }
                    doc
                }
            });
    }

    if let Some(nulls_order) = sort_by.nulls_order() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(nulls_order.syntax()))
            .append(Doc::text("nulls"))
            .append(Doc::space());
        let suffix = match nulls_order {
            ast::NullsOrder::NullsFirst(first) => first
                .first_token()
                .map(|token| leading_comments_token(&token))
                .unwrap_or_else(Doc::nil)
                .append(Doc::text("first")),
            ast::NullsOrder::NullsLast(last) => last
                .last_token()
                .map(|token| leading_comments_token(&token))
                .unwrap_or_else(Doc::nil)
                .append(Doc::text("last")),
        };
        doc = doc.append(suffix);
    }

    doc.append(trailing_comments(sort_by.syntax()))
}

fn build_named_call_arg<'a>(arg: ast::NamedArg) -> Doc<'a> {
    let mut doc = Doc::nil();
    if let Some(name) = arg.name() {
        doc = doc
            .append(leading_comments(name.syntax()))
            .append(build_name(name.syntax()));
    }

    if let Some(fat_arrow) = arg.fat_arrow_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&fat_arrow))
            .append(Doc::text("=>"));
    } else if let Some(colon_eq) = arg.colon_eq_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&colon_eq))
            .append(Doc::text(":="));
    }

    if let Some(expr) = arg.expr() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(expr.syntax()))
            .append(build_expr(expr));
    }
    doc
}

fn build_case_expr<'a>(case_expr: ast::CaseExpr) -> Doc<'a> {
    let mut doc = Doc::text("case");

    if let Some(expr) = case_expr.expr() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(expr.syntax()))
            .append(build_expr(expr));
    }

    if let Some(when_clause_list) = case_expr.when_clause_list() {
        for (index, when_clause) in when_clause_list.when_clauses().enumerate() {
            let list_comments = if index == 0 {
                leading_comments(when_clause_list.syntax())
            } else {
                Doc::nil()
            };
            doc = doc.append(
                Doc::line_or_space()
                    .append(list_comments)
                    .append(leading_comments(when_clause.syntax()))
                    .append(build_when_clause(when_clause))
                    .nest(2),
            );
        }
    }

    if let Some(else_clause) = case_expr.else_clause() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(else_clause.syntax()))
                .append(build_else_clause(else_clause))
                .nest(2),
        );
    }

    if let Some(end) = case_expr.end_token() {
        doc = doc
            .append(comments_before(end))
            .append(Doc::line_or_space());
    }
    doc.append(Doc::text("end")).group()
}

fn build_when_clause<'a>(when_clause: ast::WhenClause) -> Doc<'a> {
    let mut doc = Doc::text("when");
    if let Some(condition) = when_clause.condition() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(condition.syntax()))
            .append(build_expr(condition));
    }
    if let Some(then) = when_clause.then_token() {
        doc = doc
            .append(comments_before(then))
            .append(Doc::space())
            .append(Doc::text("then"));
    }
    if let Some(result) = when_clause.then() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(result.syntax()))
                .append(build_expr(result))
                .nest(2),
        );
    }
    doc.group()
}

fn build_else_clause<'a>(else_clause: ast::ElseClause) -> Doc<'a> {
    let mut doc = Doc::text("else");
    if let Some(expr) = else_clause.expr() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(expr.syntax()))
                .append(build_expr(expr))
                .nest(2),
        );
    }
    doc
}

fn build_cast_expr<'a>(cast_expr: ast::CastExpr) -> Doc<'a> {
    let mut doc = Doc::nil();
    if let Some(colon_colon) = cast_expr.colon_colon() {
        let ty = cast_expr.ty().unwrap();
        doc = doc
            .append(build_expr(cast_expr.expr().unwrap()))
            .append(comments_before(colon_colon.syntax().clone()))
            .append(Doc::text("::"))
            .append(leading_comments(ty.syntax()))
            .append(build_type(ty))
    } else if let Some(as_token) = cast_expr.as_token() {
        if cast_expr.cast_token().is_some() {
            doc = doc.append(Doc::text("cast"))
        } else if cast_expr.treat_token().is_some() {
            doc = doc.append(Doc::text("treat"))
        }
        let expr = cast_expr.expr().unwrap();
        let ty = cast_expr.ty().unwrap();
        if let Some(l_paren) = cast_expr.l_paren_token() {
            doc = doc.append(comments_before(l_paren));
        }
        doc = doc
            .append(Doc::text("("))
            .append(leading_comments(expr.syntax()))
            .append(build_expr(expr))
            .append(Doc::space())
            .append(leading_comments_token(&as_token))
            .append(Doc::text("as"))
            .append(Doc::space())
            .append(leading_comments(ty.syntax()))
            .append(build_type(ty));
        if let Some(r_paren) = cast_expr.r_paren_token() {
            doc = doc.append(comments_before(r_paren));
        }
        doc = doc.append(Doc::text(")"))
    } else {
        let literal = cast_expr.literal().unwrap();
        doc = doc
            .append(build_type(cast_expr.ty().unwrap()))
            .append(Doc::space())
            .append(leading_comments(literal.syntax()))
            .append(build_literal(literal));
        if let Some(qualifier) = cast_expr.interval_qualifier() {
            doc = doc
                .append(Doc::space())
                .append(leading_comments(qualifier.syntax()))
                .append(build_interval_qualifier(&qualifier))
        }
    }
    doc
}

fn build_collate_expr<'a>(collate: ast::Collate) -> Doc<'a> {
    let expr = collate.expr();
    let has_expr = expr.is_some();
    let mut doc = expr.map(build_expr).unwrap_or_else(Doc::nil);

    if let Some(collate_token) = collate.collate_token() {
        doc = doc.append(comments_before(collate_token));
    }
    if has_expr {
        doc = doc.append(Doc::space());
    }
    doc = doc.append(Doc::text("collate"));

    if let Some(collation) = collate.collation_ref() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(collation.syntax()));
        if let Some(path) = collation.path_ref() {
            doc = doc.append(build_path_ref(&path));
        }
    }
    doc
}

fn build_paren_expr<'a>(paren_expr: ast::ParenExpr) -> Doc<'a> {
    let mut doc = Doc::nil();
    if let Some(l_paren) = paren_expr.l_paren_token() {
        doc = doc.append(comments_before(l_paren));
    }
    doc = doc.append(Doc::text("("));

    if let Some(expr) = paren_expr.expr() {
        doc = doc
            .append(leading_comments(expr.syntax()))
            .append(build_expr(expr));
    } else if let Some(_compound_select) = paren_expr.compound_select() {
        todo!("parenthesized compound select nodes are not supported yet")
    } else if let Some(_from_item) = paren_expr.from_item() {
        todo!("parenthesized from item nodes are not supported yet")
    } else if let Some(_join_expr) = paren_expr.join_expr() {
        todo!("parenthesized join expression nodes are not supported yet")
    } else if let Some(_select) = paren_expr.select() {
        todo!("parenthesized select nodes are not supported yet")
    } else if let Some(_table) = paren_expr.table() {
        todo!("parenthesized table nodes are not supported yet")
    } else if let Some(_values) = paren_expr.values() {
        todo!("parenthesized values nodes are not supported yet")
    } else {
        unreachable!("a parenthesized expression should contain a node")
    }

    if let Some(r_paren) = paren_expr.r_paren_token() {
        doc = doc.append(comments_before(r_paren));
    }
    doc.append(Doc::text(")"))
}

fn build_postfix_expr<'a>(postfix_expr: ast::PostfixExpr) -> Doc<'a> {
    let expr = build_expr(postfix_expr.expr().unwrap());
    let op = match postfix_expr.op().unwrap() {
        ast::PostfixOp::AtLocal(_) => Doc::text("at local"),
        ast::PostfixOp::IsNull(_) => Doc::text("isnull"),
        ast::PostfixOp::NotNull(_) => Doc::text("notnull"),
        ast::PostfixOp::IsJson(n) => build_json_postfix("is json", n.json_keys_unique_clause()),
        ast::PostfixOp::IsJsonArray(n) => {
            build_json_postfix("is json array", n.json_keys_unique_clause())
        }
        ast::PostfixOp::IsJsonObject(n) => {
            build_json_postfix("is json object", n.json_keys_unique_clause())
        }
        ast::PostfixOp::IsJsonScalar(n) => {
            build_json_postfix("is json scalar", n.json_keys_unique_clause())
        }
        ast::PostfixOp::IsJsonValue(n) => {
            build_json_postfix("is json value", n.json_keys_unique_clause())
        }
        ast::PostfixOp::IsNormalized(n) => build_normalized_postfix("is", n.unicode_normal_form()),
        ast::PostfixOp::IsNotJson(n) => {
            build_json_postfix("is not json", n.json_keys_unique_clause())
        }
        ast::PostfixOp::IsNotJsonArray(n) => {
            build_json_postfix("is not json array", n.json_keys_unique_clause())
        }
        ast::PostfixOp::IsNotJsonObject(n) => {
            build_json_postfix("is not json object", n.json_keys_unique_clause())
        }
        ast::PostfixOp::IsNotJsonScalar(n) => {
            build_json_postfix("is not json scalar", n.json_keys_unique_clause())
        }
        ast::PostfixOp::IsNotJsonValue(n) => {
            build_json_postfix("is not json value", n.json_keys_unique_clause())
        }
        ast::PostfixOp::IsNotNormalized(n) => {
            build_normalized_postfix("is not", n.unicode_normal_form())
        }
    };
    expr.append(Doc::space()).append(op)
}

fn build_json_postfix<'a>(
    prefix: &'static str,
    clause: Option<ast::JsonKeysUniqueClause>,
) -> Doc<'a> {
    let mut doc = Doc::text(prefix);
    if let Some(clause) = clause {
        doc = doc
            .append(Doc::space())
            .append(build_json_keys_unique_clause(clause));
    }
    doc
}

fn build_normalized_postfix<'a>(
    prefix: &'static str,
    form: Option<ast::UnicodeNormalForm>,
) -> Doc<'a> {
    let mut doc = Doc::text(prefix);
    if let Some(form) = form {
        doc = doc
            .append(Doc::space())
            .append(build_unicode_normal_form(form));
    }
    doc.append(Doc::space()).append(Doc::text("normalized"))
}

fn build_bin_expr<'a>(bin_expr: ast::BinExpr) -> Doc<'a> {
    let lhs = bin_expr.lhs().unwrap();
    let rhs = bin_expr.rhs().unwrap();
    let before_op = trailing_comments(lhs.syntax());
    let after_op = leading_comments(rhs.syntax());

    build_expr(lhs)
        .append(before_op)
        .append(Doc::space())
        .append(build_op(bin_expr.op().unwrap()))
        .append(Doc::space())
        .append(after_op)
        .append(build_expr(rhs))
}

fn build_within_clause<'a>(within_clause: ast::WithinClause) -> Doc<'a> {
    let mut doc = Doc::text("within").append(Doc::space());
    if let Some(group_token) = within_clause.group_token() {
        doc = doc.append(leading_comments_token(&group_token));
    }
    doc = doc.append(Doc::text("group"));
    if let Some(l_paren) = within_clause.l_paren_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&l_paren))
            .append(Doc::text("("));
    }
    if let Some(order_by) = within_clause.order_by_clause() {
        doc = doc
            .append(leading_comments(order_by.syntax()))
            .append(build_order_by_clause(order_by));
    }
    if let Some(r_paren) = within_clause.r_paren_token() {
        doc = doc.append(comments_before(r_paren));
    }
    doc.append(Doc::text(")"))
}

fn build_over_clause<'a>(over_clause: ast::OverClause) -> Doc<'a> {
    let mut doc = Doc::text("over");
    if let Some(target) = over_clause.over_target() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(target.syntax()))
            .append(match target {
                ast::OverTarget::WindowRef(window_ref) => build_name(window_ref.syntax()),
                ast::OverTarget::OverWindowSpec(window_spec) => build_over_window_spec(window_spec),
            });
    }
    doc
}

fn build_over_window_spec<'a>(over_window_spec: ast::OverWindowSpec) -> Doc<'a> {
    let mut doc = Doc::text("(");
    if let Some(window_spec) = over_window_spec.window_spec() {
        doc = doc
            .append(leading_comments(window_spec.syntax()))
            .append(build_window_spec(window_spec));
    }
    if let Some(r_paren) = over_window_spec.r_paren_token() {
        doc = doc.append(comments_before(r_paren));
    }
    doc.append(Doc::text(")"))
}

fn build_window_spec<'a>(window_spec: ast::WindowSpec) -> Doc<'a> {
    let mut parts = Vec::new();
    if let Some(window_ref) = window_spec.window_ref() {
        parts.push(leading_comments(window_ref.syntax()).append(build_name(window_ref.syntax())));
    }
    if let Some(partition_by) = window_spec.partition_by_clause() {
        parts.push(
            leading_comments(partition_by.syntax()).append(build_partition_by_clause(partition_by)),
        );
    }
    if let Some(order_by) = window_spec.order_by_clause() {
        parts.push(leading_comments(order_by.syntax()).append(build_order_by_clause(order_by)));
    }
    if let Some(frame) = window_spec.frame_clause() {
        parts.push(leading_comments(frame.syntax()).append(build_frame_clause(frame)));
    }

    Doc::list(Itertools::intersperse(parts.into_iter(), Doc::space()).collect())
}

fn build_partition_by_clause<'a>(partition_by: ast::PartitionByClause) -> Doc<'a> {
    let mut doc = Doc::text("partition").append(Doc::space());
    if let Some(by_token) = partition_by.by_token() {
        doc = doc.append(leading_comments_token(&by_token));
    }
    doc = doc.append(Doc::text("by"));
    if let Some(exprs) = build_comma_separated_exprs(partition_by.exprs()) {
        doc = doc.append(Doc::space()).append(exprs);
    }
    doc
}

fn build_frame_clause<'a>(frame: ast::FrameClause) -> Doc<'a> {
    let mut doc = match frame.frame_units() {
        Some(ast::FrameUnits::FrameGroups(_)) => Doc::text("groups"),
        Some(ast::FrameUnits::FrameRange(_)) => Doc::text("range"),
        Some(ast::FrameUnits::FrameRows(_)) => Doc::text("rows"),
        None => Doc::nil(),
    };
    if let Some(extent) = frame.frame_extent() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(extent.syntax()))
            .append(build_frame_extent(extent));
    }
    if let Some(exclude) = frame.frame_exclude() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(exclude.syntax()))
            .append(build_frame_exclude(exclude));
    }
    doc
}

fn build_frame_extent<'a>(extent: ast::FrameExtent) -> Doc<'a> {
    match extent {
        ast::FrameExtent::FrameBetween(between) => {
            let mut doc = Doc::text("between");
            if let Some(start) = between.start() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments(start.syntax()))
                    .append(build_frame_bound(start));
            }
            if let Some(and_token) = between.and_token() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments_token(&and_token))
                    .append(Doc::text("and"));
            }
            if let Some(end) = between.end() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments(end.syntax()))
                    .append(build_frame_bound(end));
            }
            doc
        }
        ast::FrameExtent::FrameBound(bound) => build_frame_bound(bound),
    }
}

fn build_frame_bound<'a>(bound: ast::FrameBound) -> Doc<'a> {
    match bound {
        ast::FrameBound::CurrentRow(current_row) => Doc::text("current")
            .append(Doc::space())
            .append(
                current_row
                    .row_token()
                    .map(|token| leading_comments_token(&token))
                    .unwrap_or_else(Doc::nil),
            )
            .append(Doc::text("row")),
        ast::FrameBound::ExprFollowing(following) => {
            build_expr_frame_bound(following.expr(), following.following_token(), "following")
        }
        ast::FrameBound::ExprPreceding(preceding) => {
            build_expr_frame_bound(preceding.expr(), preceding.preceding_token(), "preceding")
        }
        ast::FrameBound::UnboundedFollowing(following) => Doc::text("unbounded")
            .append(Doc::space())
            .append(
                following
                    .following_token()
                    .map(|token| leading_comments_token(&token))
                    .unwrap_or_else(Doc::nil),
            )
            .append(Doc::text("following")),
        ast::FrameBound::UnboundedPreceding(preceding) => Doc::text("unbounded")
            .append(Doc::space())
            .append(
                preceding
                    .preceding_token()
                    .map(|token| leading_comments_token(&token))
                    .unwrap_or_else(Doc::nil),
            )
            .append(Doc::text("preceding")),
    }
}

fn build_expr_frame_bound<'a>(
    expr: Option<ast::Expr>,
    suffix_token: Option<SyntaxToken>,
    suffix: &'static str,
) -> Doc<'a> {
    let mut doc = expr.map(build_expr).unwrap_or_else(Doc::nil);
    if let Some(suffix_token) = suffix_token {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&suffix_token))
            .append(Doc::text(suffix));
    }
    doc
}

fn build_frame_exclude<'a>(exclude: ast::FrameExclude) -> Doc<'a> {
    let mut doc = Doc::text("exclude");
    if let Some(target) = exclude.frame_exclude_target() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(target.syntax()))
            .append(match target {
                ast::FrameExcludeTarget::CurrentRow(current_row) => Doc::text("current")
                    .append(Doc::space())
                    .append(
                        current_row
                            .row_token()
                            .map(|token| leading_comments_token(&token))
                            .unwrap_or_else(Doc::nil),
                    )
                    .append(Doc::text("row")),
                ast::FrameExcludeTarget::Group(_) => Doc::text("group"),
                ast::FrameExcludeTarget::NoOthers(no_others) => Doc::text("no")
                    .append(Doc::space())
                    .append(
                        no_others
                            .others_token()
                            .map(|token| leading_comments_token(&token))
                            .unwrap_or_else(Doc::nil),
                    )
                    .append(Doc::text("others")),
                ast::FrameExcludeTarget::Ties(_) => Doc::text("ties"),
            });
    }
    doc
}

fn build_filter_clause<'a>(filter_clause: ast::FilterClause) -> Doc<'a> {
    let mut doc = Doc::text("filter");
    if let Some(l_paren) = filter_clause.l_paren_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&l_paren))
            .append(Doc::text("("));
    }
    if let Some(where_token) = filter_clause.where_token() {
        doc = doc
            .append(leading_comments_token(&where_token))
            .append(Doc::text("where"));
    }
    if let Some(expr) = filter_clause.expr() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(expr.syntax()))
            .append(build_expr(expr));
    }
    if let Some(r_paren) = filter_clause.r_paren_token() {
        doc = doc.append(comments_before(r_paren));
    }
    doc.append(Doc::text(")"))
}

fn build_null_treatment<'a>(null_treatment: ast::NullTreatment) -> Doc<'a> {
    let (keyword, nulls_token) = match null_treatment {
        ast::NullTreatment::IgnoreNulls(ignore_nulls) => ("ignore", ignore_nulls.nulls_token()),
        ast::NullTreatment::RespectNulls(respect_nulls) => ("respect", respect_nulls.nulls_token()),
    };

    let mut doc = Doc::text(keyword);
    if let Some(nulls_token) = nulls_token {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&nulls_token))
            .append(Doc::text("nulls"));
    }
    doc
}

fn build_json_keys_unique_clause<'a>(clause: ast::JsonKeysUniqueClause) -> Doc<'a> {
    let prefix = match clause {
        ast::JsonKeysUniqueClause::JsonWithoutUniqueKeys(_) => "without",
        ast::JsonKeysUniqueClause::JsonWithUniqueKeys(_) => "with",
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

fn build_keyword_node<'a>(node: &SyntaxNode) -> Doc<'a> {
    let mut docs: Vec<Doc<'a>> = vec![];
    let mut after_line_comment = false;
    for el in node.children_with_tokens() {
        let Some(token) = el.into_token() else {
            continue;
        };
        match token.kind() {
            SyntaxKind::WHITESPACE => continue,
            SyntaxKind::COMMENT => {
                if !docs.is_empty() && !after_line_comment {
                    docs.push(Doc::space());
                }
                docs.push(Doc::text(token.text().to_string()));
                after_line_comment = is_line_comment(&token);
                if after_line_comment {
                    docs.push(Doc::hard_line());
                }
            }
            _ => {
                if !docs.is_empty() && !after_line_comment {
                    docs.push(Doc::space());
                }
                after_line_comment = false;
                docs.push(Doc::text(token.text().to_ascii_lowercase()));
            }
        }
    }
    Doc::list(docs)
}

fn build_op<'a>(op: ast::BinOp) -> Doc<'a> {
    match op {
        ast::BinOp::And(_) => Doc::text("and"),
        ast::BinOp::AtTimeZone(n) => build_keyword_node(n.syntax()),
        ast::BinOp::Caret(_) => Doc::text("^"),
        ast::BinOp::ColonColon(_) => Doc::text("::"),
        ast::BinOp::ColonEq(_) => Doc::text(":="),
        ast::BinOp::CustomOp(custom_op) => build_operator_part(custom_op.syntax()),
        ast::BinOp::Eq(_) => Doc::text("="),
        ast::BinOp::Escape(_) => Doc::text("escape"),
        ast::BinOp::FatArrow(_) => Doc::text("=>"),
        ast::BinOp::Gteq(_) => Doc::text(">="),
        ast::BinOp::Ilike(_) => Doc::text("ilike"),
        ast::BinOp::In(_) => Doc::text("in"),
        ast::BinOp::Is(_) => Doc::text("is"),
        ast::BinOp::IsDistinctFrom(n) => build_keyword_node(n.syntax()),
        ast::BinOp::IsNot(n) => build_keyword_node(n.syntax()),
        ast::BinOp::IsNotDistinctFrom(n) => build_keyword_node(n.syntax()),
        ast::BinOp::LAngle(_) => Doc::text("<"),
        ast::BinOp::Like(_) => Doc::text("like"),
        ast::BinOp::Lteq(_) => Doc::text("<="),
        ast::BinOp::Minus(_) => Doc::text("-"),
        ast::BinOp::Neq(_) => Doc::text("!="),
        ast::BinOp::Neqb(_) => Doc::text("<>"),
        ast::BinOp::NotIlike(n) => build_keyword_node(n.syntax()),
        ast::BinOp::NotIn(n) => build_keyword_node(n.syntax()),
        ast::BinOp::NotLike(n) => build_keyword_node(n.syntax()),
        ast::BinOp::NotSimilarTo(n) => build_keyword_node(n.syntax()),
        ast::BinOp::OperatorCall(op) => build_operator_call(&op),
        ast::BinOp::Or(_) => Doc::text("or"),
        ast::BinOp::Overlaps(_) => Doc::text("overlaps"),
        ast::BinOp::Percent(_) => Doc::text("%"),
        ast::BinOp::Plus(_) => Doc::text("+"),
        ast::BinOp::RAngle(_) => Doc::text(">"),
        ast::BinOp::SimilarTo(n) => build_keyword_node(n.syntax()),
        ast::BinOp::Slash(_) => Doc::text("/"),
        ast::BinOp::Star(_) => Doc::text("*"),
    }
}

fn build_operator_call<'a>(operator_call: &ast::OperatorCall) -> Doc<'a> {
    let mut doc = Doc::text("operator");

    if let Some(l_paren) = operator_call.l_paren_token() {
        doc = doc.append(comments_before(l_paren));
    }
    doc = doc.append(Doc::text("("));

    if let Some(op) = operator_call.op() {
        doc = doc
            .append(leading_comments(op.syntax()))
            .append(build_operator(&op));
    }

    if let Some(r_paren) = operator_call.r_paren_token() {
        doc = doc.append(comments_before(r_paren));
    }
    doc.append(Doc::text(")"))
}

fn build_prefix_expr<'a>(prefix_expr: ast::PrefixExpr) -> Doc<'a> {
    let expr = prefix_expr.expr().unwrap();
    let comments = comment_tokens_before(expr.syntax().clone());
    let (op, space_before_expr) = match prefix_expr.op().unwrap() {
        ast::PrefixOp::CustomOp(custom_op) => (build_operator_part(custom_op.syntax()), true),
        ast::PrefixOp::Minus(_) => (Doc::text("-"), !comments.is_empty()),
        ast::PrefixOp::Not(_) => (Doc::text("not"), true),
        ast::PrefixOp::OperatorCall(operator_call) => (build_operator_call(&operator_call), true),
        ast::PrefixOp::Plus(_) => (Doc::text("+"), !comments.is_empty()),
    };

    op.append(if space_before_expr {
        Doc::space()
    } else {
        Doc::nil()
    })
    .append(build_leading_comments(&comments))
    .append(build_expr(expr))
}

fn build_operator<'a>(op: &ast::Op) -> Doc<'a> {
    let path_ref = op.path_ref();
    let mut doc = Doc::nil();

    for element in op.syntax().children_with_tokens() {
        match element {
            rowan::NodeOrToken::Node(node) => {
                doc = doc.append(match path_ref.as_ref() {
                    Some(path) if path.syntax() == &node => build_path_ref(path),
                    _ => build_operator_part(&node),
                });
            }
            rowan::NodeOrToken::Token(token) => {
                doc = doc.append(build_operator_token(&token));
            }
        }
    }

    doc
}

fn build_operator_part<'a>(node: &SyntaxNode) -> Doc<'a> {
    Doc::list(
        node.children_with_tokens()
            .map(|element| match element {
                rowan::NodeOrToken::Node(node) => build_operator_part(&node),
                rowan::NodeOrToken::Token(token) => build_operator_token(&token),
            })
            .collect(),
    )
}

fn build_operator_token<'a>(token: &SyntaxToken) -> Doc<'a> {
    match token.kind() {
        SyntaxKind::WHITESPACE => Doc::nil(),
        SyntaxKind::COMMENT => {
            let doc = Doc::text(token.text().to_string());
            if is_line_comment(token) {
                doc.append(Doc::hard_line())
            } else {
                doc
            }
        }
        _ => Doc::text(token.text().to_ascii_lowercase()),
    }
}

fn build_literal<'a>(lit: ast::Literal) -> Doc<'a> {
    let Some(kind) = lit.kind() else {
        return Doc::nil();
    };
    match kind {
        LitKind::Default(_) => Doc::text("default"),
        LitKind::False(_) => Doc::text("false"),
        LitKind::IntNumber(t) => Doc::text(t.text().to_string()),
        LitKind::Null(_) => Doc::text("null"),
        LitKind::NumericNumber(t) => Doc::text(t.text().to_string()),
        LitKind::PositionalParam(t) => Doc::text(t.text().to_string()),
        LitKind::True(_) => Doc::text("true"),
        LitKind::BitString(_)
        | LitKind::ByteString(_)
        | LitKind::DollarQuotedString(_)
        | LitKind::EscString(_)
        | LitKind::NationalString(_)
        | LitKind::String(_)
        | LitKind::UnicodeEscString(_) => build_string_literal(&lit),
    }
}

fn build_string_literal<'a>(lit: &ast::Literal) -> Doc<'a> {
    let parts: Vec<Doc<'a>> = lit
        .syntax()
        .children_with_tokens()
        .filter_map(|el| match el {
            rowan::NodeOrToken::Token(t) if t.kind() != SyntaxKind::WHITESPACE => {
                Some(Doc::text(format_string_token(&t)))
            }
            _ => None,
        })
        .collect();
    Doc::list(Itertools::intersperse(parts.into_iter(), Doc::hard_line()).collect())
}

fn format_string_token(t: &SyntaxToken) -> String {
    let text = t.text();
    if matches!(
        t.kind(),
        SyntaxKind::STRING | SyntaxKind::DOLLAR_QUOTED_STRING
    ) {
        return text.to_string();
    }
    match text.find('\'') {
        Some(idx) => {
            let (prefix, rest) = text.split_at(idx);
            let mut s = String::with_capacity(text.len());
            s.push_str(&prefix.to_ascii_lowercase());
            s.push_str(rest);
            s
        }
        None => text.to_string(),
    }
}

fn build_type<'a>(ty: ast::Type) -> Doc<'a> {
    match ty {
        ast::Type::ArrayType(array_type) => {
            let mut doc = match array_type.ty() {
                Some(inner) => build_type(inner),
                None => Doc::nil(),
            };
            if let Some(array_token) = array_type.array_token() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments_token(&array_token))
                    .append(Doc::text("array"));
            }
            for bound in array_type.array_bounds() {
                doc = doc
                    .append(comments_before(bound.syntax().clone()))
                    .append(build_array_bound(&bound));
            }
            doc
        }
        ast::Type::BitType(bit_type) => {
            build_keyword_node(bit_type.syntax()).append(build_type_args(bit_type.arg_list()))
        }
        ast::Type::BitVaryingType(bit_varying_type) => {
            build_keyword_node(bit_varying_type.syntax())
                .append(build_type_args(bit_varying_type.arg_list()))
        }
        ast::Type::CharacterType(character_type) => build_keyword_node(character_type.syntax())
            .append(build_type_args(character_type.arg_list())),
        ast::Type::VarcharType(varchar_type) => build_keyword_node(varchar_type.syntax())
            .append(build_type_args(varchar_type.arg_list())),
        ast::Type::DoubleType(double_type) => build_keyword_node(double_type.syntax()),
        ast::Type::ExprType(expr_type) => match expr_type.expr() {
            Some(expr) => build_expr(expr),
            None => Doc::nil(),
        },
        ast::Type::IntervalType(interval_type) => {
            let mut doc = build_setof(interval_type.setof_token());
            if let Some(interval_token) = interval_type.interval_token() {
                doc = doc
                    .append(leading_comments_token(&interval_token))
                    .append(Doc::text("interval"));
            }
            doc = doc.append(build_type_precision(
                interval_type.l_paren_token(),
                interval_type.literal(),
                interval_type.r_paren_token(),
            ));
            if let Some(qualifier) = interval_type.interval_qualifier() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments(qualifier.syntax()))
                    .append(build_interval_qualifier(&qualifier));
            }
            doc
        }
        ast::Type::PathType(path_type) => {
            let mut doc = build_setof(path_type.setof_token());
            if let Some(path) = path_type.path_ref() {
                doc = doc
                    .append(leading_comments(path.syntax()))
                    .append(build_path_ref(&path));
            }
            let arg_list = path_type.arg_list();
            if let Some(arg_list) = &arg_list {
                doc = doc.append(comments_before(arg_list.syntax().clone()));
            }
            doc.append(build_type_args(arg_list))
        }
        ast::Type::PercentType(percent_type) => {
            let mut doc = build_setof(percent_type.setof_token());
            if let Some(path) = percent_type.path_ref() {
                doc = doc
                    .append(leading_comments(path.syntax()))
                    .append(build_path_ref(&path));
            }
            if let Some(clause) = percent_type.percent_type_clause() {
                doc = doc.append(comments_before(clause.syntax().clone()));
                if clause.percent_token().is_some() {
                    doc = doc.append(Doc::text("%"));
                }
                if let Some(type_token) = clause.type_token() {
                    doc = doc
                        .append(comments_before(type_token))
                        .append(Doc::text("type"));
                }
            }
            doc
        }
        ast::Type::TimeType(time_type) => {
            let mut doc = build_setof(time_type.setof_token());
            if let Some(time_token) = time_type.time_token() {
                doc = doc
                    .append(leading_comments_token(&time_token))
                    .append(Doc::text("time"));
            }
            doc.append(build_type_precision(
                time_type.l_paren_token(),
                time_type.literal(),
                time_type.r_paren_token(),
            ))
            .append(build_timezone(time_type.timezone()))
        }
        ast::Type::TimestampType(timestamp_type) => {
            let mut doc = build_setof(timestamp_type.setof_token());
            if let Some(timestamp_token) = timestamp_type.timestamp_token() {
                doc = doc
                    .append(leading_comments_token(&timestamp_token))
                    .append(Doc::text("timestamp"));
            }
            doc.append(build_type_precision(
                timestamp_type.l_paren_token(),
                timestamp_type.literal(),
                timestamp_type.r_paren_token(),
            ))
            .append(build_timezone(timestamp_type.timezone()))
        }
    }
}

fn build_setof<'a>(setof: Option<SyntaxToken>) -> Doc<'a> {
    match setof {
        Some(_) => Doc::text("setof").append(Doc::space()),
        None => Doc::nil(),
    }
}

fn build_array_bound<'a>(bound: &ast::ArrayBound) -> Doc<'a> {
    let mut doc = Doc::text("[");
    if let Some(expr) = bound.expr() {
        doc = doc
            .append(leading_comments(expr.syntax()))
            .append(build_expr(expr));
    }
    if let Some(r_brack) = bound.r_brack_token() {
        doc = doc.append(comments_before(r_brack));
    }
    doc.append(Doc::text("]"))
}

fn build_type_args<'a>(arg_list: Option<ast::ArgList>) -> Doc<'a> {
    let Some(arg_list) = arg_list else {
        return Doc::nil();
    };
    let args: Vec<Doc<'a>> = arg_list
        .args()
        .map(|arg| {
            let mut doc = leading_comments(arg.syntax());
            if let Some(expr) = arg.expr() {
                doc = doc.append(build_expr(expr));
            }
            doc.append(trailing_comments(arg.syntax()))
        })
        .collect();
    let mut doc = Doc::text("(");
    if args.is_empty() {
        if let Some(r_paren) = arg_list.r_paren_token() {
            doc = doc.append(comments_before(r_paren));
        }
    } else {
        doc = doc.append(Doc::list(
            Itertools::intersperse(args.into_iter(), Doc::text(",").append(Doc::space())).collect(),
        ));
    }
    doc.append(Doc::text(")"))
}

fn build_type_precision<'a>(
    l_paren: Option<SyntaxToken>,
    literal: Option<ast::Literal>,
    r_paren: Option<SyntaxToken>,
) -> Doc<'a> {
    let Some(l_paren) = l_paren else {
        return Doc::nil();
    };
    let mut doc = comments_before(l_paren).append(Doc::text("("));
    if let Some(literal) = literal {
        doc = doc
            .append(leading_comments(literal.syntax()))
            .append(build_literal(literal));
    }
    if let Some(r_paren) = r_paren {
        doc = doc.append(comments_before(r_paren));
    }
    doc.append(Doc::text(")"))
}

fn build_timezone<'a>(timezone: Option<ast::Timezone>) -> Doc<'a> {
    let Some(timezone) = timezone else {
        return Doc::nil();
    };
    let doc = Doc::space().append(leading_comments(timezone.syntax()));
    match timezone {
        ast::Timezone::WithTimezone(with_timezone) => {
            doc.append(build_keyword_node(with_timezone.syntax()))
        }
        ast::Timezone::WithoutTimezone(without_timezone) => {
            doc.append(build_keyword_node(without_timezone.syntax()))
        }
    }
}

fn build_interval_qualifier<'a>(qualifier: &ast::IntervalQualifier) -> Doc<'a> {
    match qualifier {
        ast::IntervalQualifier::IntervalSecond(second) => {
            let mut doc = Doc::nil();
            if let Some(unit) = second
                .day_token()
                .or_else(|| second.hour_token())
                .or_else(|| second.minute_token())
            {
                doc = doc
                    .append(Doc::text(unit.text().to_ascii_lowercase()))
                    .append(Doc::space());
            }
            if let Some(to_token) = second.to_token() {
                doc = doc
                    .append(leading_comments_token(&to_token))
                    .append(Doc::text("to"))
                    .append(Doc::space());
            }
            if let Some(second_token) = second.second_token() {
                doc = doc
                    .append(leading_comments_token(&second_token))
                    .append(Doc::text("second"));
            }
            doc.append(build_type_precision(
                second.l_paren_token(),
                second.literal(),
                second.r_paren_token(),
            ))
        }
        ast::IntervalQualifier::IntervalDay(day) => build_keyword_node(day.syntax()),
        ast::IntervalQualifier::IntervalHour(hour) => build_keyword_node(hour.syntax()),
        ast::IntervalQualifier::IntervalMinute(minute) => build_keyword_node(minute.syntax()),
        ast::IntervalQualifier::IntervalMonth(month) => build_keyword_node(month.syntax()),
        ast::IntervalQualifier::IntervalYear(year) => build_keyword_node(year.syntax()),
    }
}

fn comments_before<'a>(el: impl Into<SyntaxElement>) -> Doc<'a> {
    let mut doc = Doc::nil();
    for token in comment_tokens_before(el) {
        doc = doc
            .append(Doc::space())
            .append(Doc::text(token.text().to_string()));
        if is_line_comment(&token) {
            doc = doc.append(Doc::hard_line());
        }
    }
    doc
}

fn comment_tokens_before(el: impl Into<SyntaxElement>) -> Vec<SyntaxToken> {
    let mut tokens: Vec<SyntaxToken> = vec![];
    let mut curr = el.into().prev_sibling_or_token();
    while let Some(rowan::NodeOrToken::Token(token)) = curr {
        match token.kind() {
            SyntaxKind::COMMENT => tokens.push(token.clone()),
            SyntaxKind::WHITESPACE => (),
            _ => break,
        }
        curr = token.prev_sibling_or_token();
    }
    tokens.reverse();
    tokens
}

fn leading_comments_token<'a>(token: &SyntaxToken) -> Doc<'a> {
    build_leading_comments(&comment_tokens_before(token.clone()))
}

fn is_line_comment(token: &SyntaxToken) -> bool {
    token.text().starts_with("--")
}

fn leading_comments<'a>(node: &SyntaxNode) -> Doc<'a> {
    build_leading_comments(&comment_tokens_before(node.clone()))
}

fn build_leading_comments<'a>(tokens: &[SyntaxToken]) -> Doc<'a> {
    let mut doc = Doc::nil();
    for token in tokens {
        doc = doc.append(Doc::text(token.text().to_string()));
        doc = doc.append(if is_line_comment(token) {
            Doc::hard_line()
        } else {
            Doc::space()
        });
    }
    doc
}

fn trailing_comments<'a>(node: &SyntaxNode) -> Doc<'a> {
    let mut doc = Doc::nil();
    let mut after_line_comment = false;
    for next in node.siblings_with_tokens(Direction::Next).skip(1) {
        match next {
            rowan::NodeOrToken::Node(_node) => {
                break;
            }
            rowan::NodeOrToken::Token(token) => {
                if token.kind() == SyntaxKind::COMMENT {
                    if !after_line_comment {
                        doc = doc.append(Doc::space());
                    }
                    doc = doc.append(Doc::text(token.text().to_string()));
                    after_line_comment = is_line_comment(&token);
                    if after_line_comment {
                        doc = doc.append(Doc::hard_line());
                    }
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

        if let Some(column_name) = as_name.name() {
            let alias = if as_name.as_token().is_some() {
                quote_column_alias(&column_name.text())
            } else {
                quote_bare_column_alias(&column_name.text())
            };
            doc = doc.append(Doc::space()).append(Doc::text(alias));
        }
    }

    doc = doc.append(trailing_comments(target.syntax()));

    Some(doc)
}

pub fn fmt(text: &str) -> Result<String> {
    let line_ending = find_newline(text)
        .map(|(_, ending)| ending)
        .unwrap_or_default();

    let line_break = match line_ending {
        LineEnding::Cr => LineBreak::Cr,
        LineEnding::CrLf => LineBreak::Crlf,
        LineEnding::Lf => LineBreak::Lf,
    };

    let parse = ast::SourceFile::parse(text);
    let file = parse.tree();
    debug_assert_eq!(
        parse.errors(),
        vec![],
        "should bail out when there's parse errors"
    );
    let doc = build_source_file(&file);

    Ok(print(
        &doc,
        &PrintOptions {
            line_break,
            ..Default::default()
        },
    ))
}
