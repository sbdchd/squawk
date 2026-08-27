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
                    doc = doc.append(build_stmt(stmt));
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

fn build_empty_stmt<'a>(empty_stmt: &ast::EmptyStmt) -> Doc<'a> {
    build_semicolon(empty_stmt.semicolon_token())
}

fn build_begin<'a>(begin: &ast::Begin) -> Doc<'a> {
    let mut doc = if begin.start_token().is_some() {
        Doc::text("start")
    } else {
        Doc::text("begin")
    };

    if let Some(token) = begin.work_token().or_else(|| begin.transaction_token()) {
        let keyword = if token.kind() == SyntaxKind::WORK_KW {
            "work"
        } else {
            "transaction"
        };
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&token))
            .append(Doc::text(keyword));
    }
    if let Some(modes) = begin.transaction_mode_list() {
        let mode_docs = modes.transaction_modes().map(|mode| {
            (
                leading_comments(mode.syntax()).append(build_keyword_node(mode.syntax())),
                mode.syntax().clone(),
            )
        });
        if let Some(modes_doc) = build_comma_separated_docs(mode_docs) {
            doc = doc.append(
                Doc::line_or_space()
                    .append(leading_comments(modes.syntax()))
                    .append(modes_doc)
                    .nest(2),
            );
        }
    }

    doc.group().append(build_semicolon(begin.semicolon_token()))
}

fn build_commit<'a>(commit: ast::Commit) -> Doc<'a> {
    match commit {
        ast::Commit::CommitPrepared(commit) => {
            let mut doc = Doc::text("commit");
            if let Some(prepared) = commit.prepared_token() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments_token(&prepared))
                    .append(Doc::text("prepared"));
            }
            if let Some(literal) = commit.literal() {
                doc = doc.append(
                    Doc::line_or_space()
                        .append(leading_comments(literal.syntax()))
                        .append(build_literal(literal))
                        .nest(2),
                );
            }
            doc.group()
                .append(build_semicolon(commit.semicolon_token()))
        }
        ast::Commit::CommitTransaction(commit) => {
            let mut doc = if commit.end_token().is_some() {
                Doc::text("end")
            } else {
                Doc::text("commit")
            };
            if let Some(token) = commit.work_token().or_else(|| commit.transaction_token()) {
                let keyword = if token.kind() == SyntaxKind::WORK_KW {
                    "work"
                } else {
                    "transaction"
                };
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments_token(&token))
                    .append(Doc::text(keyword));
            }
            if let Some(chain) = commit.chain_clause() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments(chain.syntax()))
                    .append(build_keyword_node(chain.syntax()));
            }
            doc.append(build_semicolon(commit.semicolon_token()))
        }
    }
}

fn build_rollback<'a>(rollback: ast::Rollback) -> Doc<'a> {
    match rollback {
        ast::Rollback::RollbackPrepared(rollback) => {
            let mut doc = Doc::text("rollback");
            if let Some(prepared) = rollback.prepared_token() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments_token(&prepared))
                    .append(Doc::text("prepared"));
            }
            if let Some(literal) = rollback.literal() {
                doc = doc.append(
                    Doc::line_or_space()
                        .append(leading_comments(literal.syntax()))
                        .append(build_literal(literal))
                        .nest(2),
                );
            }
            doc.group()
                .append(build_semicolon(rollback.semicolon_token()))
        }
        ast::Rollback::RollbackToSavepoint(rollback) => {
            let mut doc = Doc::text("rollback");
            if let Some(token) = rollback
                .work_token()
                .or_else(|| rollback.transaction_token())
            {
                let keyword = if token.kind() == SyntaxKind::WORK_KW {
                    "work"
                } else {
                    "transaction"
                };
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments_token(&token))
                    .append(Doc::text(keyword));
            }
            if let Some(to) = rollback.to_token() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments_token(&to))
                    .append(Doc::text("to"));
            }
            if let Some(savepoint) = rollback.savepoint_token() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments_token(&savepoint))
                    .append(Doc::text("savepoint"));
            }
            if let Some(savepoint) = rollback.savepoint_ref() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments(savepoint.syntax()))
                    .append(build_name(savepoint.syntax()));
            }
            doc.append(build_semicolon(rollback.semicolon_token()))
        }
        ast::Rollback::RollbackTransaction(rollback) => {
            let mut doc = if rollback.abort_token().is_some() {
                Doc::text("abort")
            } else {
                Doc::text("rollback")
            };
            if let Some(token) = rollback
                .work_token()
                .or_else(|| rollback.transaction_token())
            {
                let keyword = if token.kind() == SyntaxKind::WORK_KW {
                    "work"
                } else {
                    "transaction"
                };
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments_token(&token))
                    .append(Doc::text(keyword));
            }
            if let Some(chain) = rollback.chain_clause() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments(chain.syntax()))
                    .append(build_keyword_node(chain.syntax()));
            }
            doc.append(build_semicolon(rollback.semicolon_token()))
        }
    }
}

fn build_prepare<'a>(prepare: &ast::Prepare) -> Doc<'a> {
    let mut header_body = Doc::nil();
    let mut has_header_body = false;
    if let Some(name) = prepare.name() {
        has_header_body = true;
        header_body = header_body
            .append(leading_comments(name.syntax()))
            .append(build_name(name.syntax()));
    }
    if let Some(params) = prepare.param_list() {
        has_header_body = true;
        header_body = header_body
            .append(leading_comments(params.syntax()))
            .append(build_function_param_list(params));
    }
    if let Some(as_token) = prepare.as_token() {
        if has_header_body {
            header_body = header_body.append(Doc::line_or_space());
        }
        has_header_body = true;
        header_body = header_body
            .append(leading_comments_token(&as_token))
            .append(Doc::text("as"));
    }

    let mut header = Doc::text("prepare");
    if has_header_body {
        header = header.append(Doc::line_or_space().append(header_body).nest(2));
    }
    let mut doc = header.group();
    if let Some(stmt) = prepare.stmt() {
        doc = doc.append(
            Doc::hard_line()
                .append(leading_comments(stmt.syntax()))
                .append(build_preparable_stmt(stmt))
                .nest(2),
        );
    }
    doc.append(build_semicolon(prepare.semicolon_token()))
}

fn build_prepare_transaction<'a>(prepare: &ast::PrepareTransaction) -> Doc<'a> {
    let mut doc = Doc::text("prepare");
    if let Some(transaction) = prepare.transaction_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&transaction))
            .append(Doc::text("transaction"));
    }
    if let Some(literal) = prepare.literal() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(literal.syntax()))
                .append(build_literal(literal))
                .nest(2),
        );
    }
    doc.group()
        .append(build_semicolon(prepare.semicolon_token()))
}

fn build_savepoint_create<'a>(savepoint: &ast::SavepointCreate) -> Doc<'a> {
    let mut doc = Doc::text("savepoint");
    if let Some(name) = savepoint.savepoint() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(name.syntax()))
            .append(build_name(name.syntax()));
    }
    doc.append(build_semicolon(savepoint.semicolon_token()))
}

fn build_release_savepoint<'a>(release: &ast::ReleaseSavepoint) -> Doc<'a> {
    let mut doc = Doc::text("release");
    if let Some(savepoint) = release.savepoint_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&savepoint))
            .append(Doc::text("savepoint"));
    }
    if let Some(name) = release.savepoint_ref() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(name.syntax()))
            .append(build_name(name.syntax()));
    }
    doc.append(build_semicolon(release.semicolon_token()))
}

fn build_insert<'a>(insert: &ast::Insert) -> Doc<'a> {
    let mut doc = Doc::nil();
    if let Some(with_clause) = insert.with_clause() {
        doc = doc
            .append(leading_comments(with_clause.syntax()))
            .append(build_with_clause(with_clause))
            .append(Doc::hard_line());
        if let Some(insert_token) = insert.insert_token() {
            doc = doc.append(leading_comments_token(&insert_token));
        }
    }

    doc = doc.append(Doc::text("insert"));
    if let Some(into_token) = insert.into_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&into_token))
            .append(Doc::text("into"));
    }
    if let Some(relation) = insert.relation_name_ref() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(relation.syntax()));
        if let Some(path) = relation.path_ref() {
            doc = doc.append(build_path_ref(&path));
        }
    }
    if let Some(alias) = insert.alias() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(alias.syntax()))
            .append(build_required_as_alias(alias));
    }
    if let Some(columns) = insert.column_target_list() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(columns.syntax()))
            .append(build_column_target_list(columns));
    }
    if let Some(overriding) = insert.overriding_clause() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(overriding.syntax()))
            .append(build_overriding_clause(overriding));
    }
    if let Some(source) = insert.insert_source() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(source.syntax()))
            .append(build_insert_source(source));
    }
    if let Some(on_conflict) = insert.on_conflict_clause() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(on_conflict.syntax()))
            .append(build_on_conflict_clause(on_conflict));
    }
    if let Some(returning) = insert.returning_clause() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(returning.syntax()))
            .append(build_returning_clause(returning));
    }

    doc.append(build_semicolon(insert.semicolon_token()))
        .group()
}

fn build_required_as_alias<'a>(alias: ast::RequiredAsAlias) -> Doc<'a> {
    let mut doc = alias
        .as_token()
        .map(|token| leading_comments_token(&token).append(Doc::text("as")))
        .unwrap_or_else(Doc::nil);
    if let Some(name) = alias.name() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(name.syntax()))
            .append(build_name(name.syntax()));
    }
    doc
}

fn build_overriding_clause<'a>(overriding: ast::OverridingClause) -> Doc<'a> {
    let (middle, middle_token, value_token) = match overriding {
        ast::OverridingClause::OverridingSystemValue(value) => {
            ("system", value.system_token(), value.value_token())
        }
        ast::OverridingClause::OverridingUserValue(value) => {
            ("user", value.user_token(), value.value_token())
        }
    };
    let mut doc = Doc::text("overriding");
    if let Some(token) = middle_token {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&token))
            .append(Doc::text(middle));
    }
    if let Some(token) = value_token {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&token))
            .append(Doc::text("value"));
    }
    doc
}

fn build_insert_source<'a>(source: ast::InsertSource) -> Doc<'a> {
    match source {
        ast::InsertSource::DefaultValues(default_values) => {
            let mut doc = Doc::text("default");
            if let Some(values_token) = default_values.values_token() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments_token(&values_token))
                    .append(Doc::text("values"));
            }
            doc
        }
        ast::InsertSource::SelectVariant(select) => build_select_variant(select),
    }
}

fn build_on_conflict_clause<'a>(on_conflict: ast::OnConflictClause) -> Doc<'a> {
    let mut doc = Doc::text("on");
    if let Some(conflict_token) = on_conflict.conflict_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&conflict_token))
            .append(Doc::text("conflict"));
    }
    if let Some(target) = on_conflict.conflict_target() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(target.syntax()))
            .append(build_conflict_target(target));
    }
    if let Some(action) = on_conflict.conflict_action() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(action.syntax()))
                .append(build_conflict_action(action))
                .nest(2),
        );
    }
    doc.group()
}

fn build_conflict_target<'a>(target: ast::ConflictTarget) -> Doc<'a> {
    match target {
        ast::ConflictTarget::ConflictOnConstraint(constraint) => {
            let mut doc = Doc::text("on");
            if let Some(token) = constraint.constraint_token() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments_token(&token))
                    .append(Doc::text("constraint"));
            }
            if let Some(name) = constraint.constraint_name_ref() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments(name.syntax()));
                if let Some(path) = name.path_ref() {
                    doc = doc.append(build_path_ref(&path));
                }
            }
            doc
        }
        ast::ConflictTarget::ConflictOnIndex(index) => {
            let mut doc = index
                .conflict_index_item_list()
                .map(build_conflict_index_item_list)
                .unwrap_or_else(Doc::nil);
            if let Some(where_clause) = index.where_clause() {
                doc = doc
                    .append(Doc::line_or_space())
                    .append(leading_comments(where_clause.syntax()))
                    .append(build_where_clause(where_clause));
            }
            doc
        }
    }
}

fn build_conflict_index_item_list<'a>(items: ast::ConflictIndexItemList) -> Doc<'a> {
    let mut doc = items
        .l_paren_token()
        .map(comments_before)
        .unwrap_or_else(Doc::nil)
        .append(Doc::text("("));
    let item_docs = items.conflict_index_items().map(|item| {
        let syntax = item.syntax().clone();
        (
            leading_comments(item.syntax()).append(build_conflict_index_item(item)),
            syntax,
        )
    });
    let mut body = build_comma_separated_docs(item_docs).unwrap_or_else(Doc::nil);
    if let Some(r_paren) = items.r_paren_token() {
        body = body.append(comments_before(r_paren));
    }
    doc = doc.append(wrap_body(body)).append(Doc::text(")"));
    doc.group()
}

fn build_conflict_index_item<'a>(item: ast::ConflictIndexItem) -> Doc<'a> {
    let mut doc = if let Some(collate) = item.collate() {
        build_collate_expr(collate)
    } else {
        item.expr().map(build_expr).unwrap_or_else(Doc::nil)
    };
    if let Some(op_class) = item.op_class_ref() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(op_class.syntax()));
        if let Some(path) = op_class.path_ref() {
            doc = doc.append(build_path_ref(&path));
        }
    }
    doc
}

fn build_conflict_action<'a>(action: ast::ConflictAction) -> Doc<'a> {
    match action {
        ast::ConflictAction::ConflictDoNothing(action) => {
            let mut doc = Doc::text("do");
            if let Some(nothing_token) = action.nothing_token() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments_token(&nothing_token))
                    .append(Doc::text("nothing"));
            }
            doc
        }
        ast::ConflictAction::ConflictDoUpdateSet(action) => {
            let mut doc = Doc::text("do");
            if let Some(update_token) = action.update_token() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments_token(&update_token))
                    .append(Doc::text("update"));
            }
            if let Some(set_clause) = action.set_clause() {
                doc = doc
                    .append(Doc::line_or_space())
                    .append(leading_comments(set_clause.syntax()))
                    .append(build_set_clause(set_clause));
            }
            if let Some(where_clause) = action.where_clause() {
                doc = doc
                    .append(Doc::line_or_space())
                    .append(leading_comments(where_clause.syntax()))
                    .append(build_where_clause(where_clause));
            }
            doc
        }
        ast::ConflictAction::ConflictDoSelect(action) => {
            let mut doc = Doc::text("do");
            if let Some(select_token) = action.select_token() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments_token(&select_token))
                    .append(Doc::text("select"));
            }
            if let Some(locking) = action.locking_clause() {
                doc = doc
                    .append(Doc::line_or_space())
                    .append(leading_comments(locking.syntax()))
                    .append(build_locking_clause(locking));
            }
            if let Some(where_clause) = action.where_clause() {
                doc = doc
                    .append(Doc::line_or_space())
                    .append(leading_comments(where_clause.syntax()))
                    .append(build_where_clause(where_clause));
            }
            doc
        }
    }
}

fn build_delete<'a>(delete: &ast::Delete) -> Doc<'a> {
    let mut doc = Doc::nil();
    if let Some(with_clause) = delete.with_clause() {
        doc = doc
            .append(leading_comments(with_clause.syntax()))
            .append(build_with_clause(with_clause))
            .append(Doc::hard_line());
        if let Some(delete_token) = delete.delete_token() {
            doc = doc.append(leading_comments_token(&delete_token));
        }
    }

    doc = doc.append(Doc::text("delete"));
    if let Some(from_token) = delete.from_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&from_token))
            .append(Doc::text("from"));
    }
    if let Some(relation) = delete.relation_name() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(relation.syntax()))
            .append(build_relation_name(relation));
    }
    if let Some(for_portion_of) = delete.for_portion_of() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(for_portion_of.syntax()))
            .append(build_for_portion_of(for_portion_of));
    }
    if let Some(alias) = delete.alias() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(alias.syntax()))
            .append(build_optional_as_alias(alias));
    }
    if let Some(using_clause) = delete.using_clause() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(using_clause.syntax()))
            .append(build_using_clause(using_clause));
    }
    if let Some(where_clause) = delete.where_clause_or_current_of() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(where_clause.syntax()))
            .append(build_where_clause_or_current_of(where_clause));
    }
    if let Some(returning_clause) = delete.returning_clause() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(returning_clause.syntax()))
            .append(build_returning_clause(returning_clause));
    }

    doc.append(build_semicolon(delete.semicolon_token()))
        .group()
}

fn build_optional_as_alias<'a>(alias: ast::OptionalAsAlias) -> Doc<'a> {
    let mut doc = Doc::nil();
    if let Some(as_token) = alias.as_token() {
        doc = doc
            .append(leading_comments_token(&as_token))
            .append(Doc::text("as"))
            .append(Doc::space());
    }
    if let Some(name) = alias.name() {
        doc = doc
            .append(leading_comments(name.syntax()))
            .append(build_name(name.syntax()));
    }
    doc
}

fn build_using_clause<'a>(using_clause: ast::UsingClause) -> Doc<'a> {
    let mut doc = using_clause
        .using_token()
        .map(|token| leading_comments_token(&token).append(Doc::text("using")))
        .unwrap_or_else(Doc::nil);
    let items = using_clause.from_items().map(|item| {
        (
            leading_comments(item.syntax()).append(build_from_item(item.clone())),
            item.syntax().clone(),
        )
    });
    if let Some(items) = build_comma_separated_docs(items) {
        doc = doc.append(Doc::space()).append(items.nest(2));
    }
    doc
}

fn build_where_clause_or_current_of<'a>(clause: ast::WhereClauseOrCurrentOf) -> Doc<'a> {
    match clause {
        ast::WhereClauseOrCurrentOf::WhereClause(clause) => build_where_clause(clause),
        ast::WhereClauseOrCurrentOf::WhereCurrentOf(current_of) => {
            build_where_current_of(current_of)
        }
    }
}

fn build_where_current_of<'a>(current_of: ast::WhereCurrentOf) -> Doc<'a> {
    let mut doc = Doc::nil();
    if let Some(where_token) = current_of.where_token() {
        doc = doc
            .append(leading_comments_token(&where_token))
            .append(Doc::text("where"));
    }
    if let Some(current_token) = current_of.current_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&current_token))
            .append(Doc::text("current"));
    }
    if let Some(of_token) = current_of.of_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&of_token))
            .append(Doc::text("of"));
    }
    if let Some(cursor) = current_of.cursor_ref() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(cursor.syntax()))
            .append(build_name(cursor.syntax()));
    }
    doc
}

fn build_returning_clause<'a>(returning: ast::ReturningClause) -> Doc<'a> {
    let mut doc = returning
        .returning_token()
        .map(|token| leading_comments_token(&token).append(Doc::text("returning")))
        .unwrap_or_else(Doc::nil);
    if let Some(options) = returning.returning_option_list() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(options.syntax()))
            .append(build_returning_option_list(options));
    }
    if let Some(target_list) = returning.target_list() {
        let targets = Doc::list(
            Itertools::intersperse(
                target_list.targets().flat_map(build_target),
                Doc::text(",").append(Doc::line_or_space()),
            )
            .collect(),
        );
        doc = doc
            .append(Doc::space())
            .append(leading_comments(target_list.syntax()))
            .append(targets.nest(2).group());
    }
    doc
}

fn build_returning_option_list<'a>(options: ast::ReturningOptionList) -> Doc<'a> {
    let mut doc = options
        .with_token()
        .map(|token| leading_comments_token(&token).append(Doc::text("with")))
        .unwrap_or_else(Doc::nil);
    if let Some(l_paren) = options.l_paren_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&l_paren))
            .append(Doc::text("("));
    }
    let items = options.returning_options().map(|option| {
        let syntax = option.syntax().clone();
        (
            leading_comments(option.syntax()).append(build_returning_option(option)),
            syntax,
        )
    });
    let mut body = build_comma_separated_docs(items).unwrap_or_else(Doc::nil);
    if let Some(r_paren) = options.r_paren_token() {
        body = body.append(comments_before(r_paren));
    }
    doc.append(wrap_body(body)).append(Doc::text(")")).group()
}

fn build_returning_option<'a>(option: ast::ReturningOption) -> Doc<'a> {
    let (keyword, as_token, name) = match option {
        ast::ReturningOption::ReturningOld(old) => ("old", old.as_token(), old.name()),
        ast::ReturningOption::ReturningNew(new) => ("new", new.as_token(), new.name()),
    };
    let mut doc = Doc::text(keyword);
    if let Some(as_token) = as_token {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&as_token))
            .append(Doc::text("as"));
    }
    if let Some(name) = name {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(name.syntax()))
            .append(build_name(name.syntax()));
    }
    doc
}

fn build_for_portion_of<'a>(portion: ast::ForPortionOf) -> Doc<'a> {
    let mut doc = Doc::text("for");
    if let Some(portion_token) = portion.portion_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&portion_token))
            .append(Doc::text("portion"));
    }
    if let Some(of_token) = portion.of_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&of_token))
            .append(Doc::text("of"));
    }
    if let Some(column) = portion.column_name_ref() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(column.syntax()))
            .append(build_name(column.syntax()));
    }
    if let Some(range) = portion.range() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(range.syntax()))
            .append(build_portion_target(range));
    }
    doc
}

fn build_portion_target<'a>(target: ast::PortionTarget) -> Doc<'a> {
    match target {
        ast::PortionTarget::PortionFromTo(range) => {
            let mut doc = Doc::text("from");
            if let Some(from) = range.from() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments(from.syntax()))
                    .append(build_expr(from));
            }
            if let Some(to_token) = range.to_token() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments_token(&to_token))
                    .append(Doc::text("to"));
            }
            if let Some(to) = range.to() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments(to.syntax()))
                    .append(build_expr(to));
            }
            doc
        }
        ast::PortionTarget::PortionRange(range) => {
            let mut doc = range
                .l_paren_token()
                .map(comments_before)
                .unwrap_or_else(Doc::nil)
                .append(Doc::text("("));
            if let Some(expr) = range.expr() {
                doc = doc
                    .append(leading_comments(expr.syntax()))
                    .append(build_expr(expr));
            }
            if let Some(r_paren) = range.r_paren_token() {
                doc = doc.append(comments_before(r_paren));
            }
            doc.append(Doc::text(")"))
        }
    }
}

fn build_merge<'a>(merge: &ast::Merge) -> Doc<'a> {
    let mut doc = Doc::nil();
    if let Some(with_clause) = merge.with_clause() {
        doc = doc
            .append(leading_comments(with_clause.syntax()))
            .append(build_with_clause(with_clause))
            .append(Doc::hard_line());
        if let Some(merge_token) = merge.merge_token() {
            doc = doc.append(leading_comments_token(&merge_token));
        }
    }

    doc = doc.append(Doc::text("merge"));
    if let Some(into_token) = merge.into_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&into_token))
            .append(Doc::text("into"));
    }
    if let Some(relation) = merge.table_relation_name() {
        let trailing = merge
            .alias()
            .is_some()
            .then(|| trailing_comments(relation.syntax()));
        doc = doc
            .append(Doc::space())
            .append(build_table_relation_name(relation));
        if let Some(trailing) = trailing {
            doc = doc.append(trailing);
        }
    }
    if let Some(alias) = merge.alias() {
        doc = doc
            .append(Doc::space())
            .append(build_optional_as_alias(alias));
    }
    if let Some(using) = merge.using_on_clause() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(using.syntax()))
            .append(build_using_on_clause(using));
    }
    for when_clause in merge.merge_when_clauses() {
        doc = doc
            .append(Doc::hard_line())
            .append(leading_comments(when_clause.syntax()))
            .append(build_merge_when_clause(when_clause));
    }
    if let Some(returning) = merge.returning_clause() {
        doc = doc
            .append(Doc::hard_line())
            .append(leading_comments(returning.syntax()))
            .append(build_returning_clause(returning));
    }

    doc.append(build_semicolon(merge.semicolon_token())).group()
}

fn build_using_on_clause<'a>(using: ast::UsingOnClause) -> Doc<'a> {
    let mut doc = Doc::text("using");
    if let Some(item) = using.from_item() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(item.syntax()))
            .append(build_from_item(item));
    }
    if let Some(on_clause) = using.on_clause() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(on_clause.syntax()))
            .append(build_on_clause(on_clause));
    }
    doc.group()
}

fn build_on_clause<'a>(on_clause: ast::OnClause) -> Doc<'a> {
    let mut doc = Doc::text("on");
    if let Some(expr) = on_clause.expr() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(expr.syntax()))
            .append(build_expr(expr));
    }
    doc
}

fn build_merge_when_clause<'a>(clause: ast::MergeWhenClause) -> Doc<'a> {
    let (mut doc, condition, then_token, action) = match clause {
        ast::MergeWhenClause::MergeWhenMatched(clause) => {
            let mut doc = Doc::text("when");
            if let Some(token) = clause.matched_token() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments_token(&token))
                    .append(Doc::text("matched"));
            }
            (
                doc,
                clause.merge_condition(),
                clause.then_token(),
                clause.merge_action(),
            )
        }
        ast::MergeWhenClause::MergeWhenNotMatchedSource(clause) => {
            let mut doc =
                build_merge_when_not_matched_prefix(clause.not_token(), clause.matched_token());
            if let Some(token) = clause.by_token() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments_token(&token))
                    .append(Doc::text("by"));
            }
            if let Some(token) = clause.source_token() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments_token(&token))
                    .append(Doc::text("source"));
            }
            (
                doc,
                clause.merge_condition(),
                clause.then_token(),
                clause.merge_action(),
            )
        }
        ast::MergeWhenClause::MergeWhenNotMatchedTarget(clause) => {
            let mut doc =
                build_merge_when_not_matched_prefix(clause.not_token(), clause.matched_token());
            if let Some(by_target) = clause.by_target() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments(by_target.syntax()));
                if let Some(token) = by_target.by_token() {
                    doc = doc
                        .append(leading_comments_token(&token))
                        .append(Doc::text("by"));
                }
                if let Some(token) = by_target.target_token() {
                    doc = doc
                        .append(Doc::space())
                        .append(leading_comments_token(&token))
                        .append(Doc::text("target"));
                }
            }
            (
                doc,
                clause.merge_condition(),
                clause.then_token(),
                clause.merge_action(),
            )
        }
    };

    if let Some(condition) = condition {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(condition.syntax()))
            .append(build_merge_condition(condition));
    }
    if let Some(then_token) = then_token {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&then_token))
            .append(Doc::text("then"));
    }
    doc = doc.group();
    if let Some(action) = action {
        doc = doc.append(
            Doc::hard_line()
                .append(leading_comments(action.syntax()))
                .append(build_merge_action(action))
                .nest(2),
        );
    }
    doc
}

fn build_merge_when_not_matched_prefix<'a>(
    not_token: Option<SyntaxToken>,
    matched_token: Option<SyntaxToken>,
) -> Doc<'a> {
    let mut doc = Doc::text("when");
    if let Some(token) = not_token {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&token))
            .append(Doc::text("not"));
    }
    if let Some(token) = matched_token {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&token))
            .append(Doc::text("matched"));
    }
    doc
}

fn build_merge_condition<'a>(condition: ast::MergeCondition) -> Doc<'a> {
    let mut doc = Doc::text("and");
    if let Some(expr) = condition.expr() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(expr.syntax()))
                .append(build_expr(expr))
                .nest(2),
        );
    }
    doc
}

fn build_merge_action<'a>(action: ast::MergeAction) -> Doc<'a> {
    match action {
        ast::MergeAction::MergeDelete(_) => Doc::text("delete"),
        ast::MergeAction::MergeDoNothing(action) => {
            let mut doc = Doc::text("do");
            if let Some(token) = action.nothing_token() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments_token(&token))
                    .append(Doc::text("nothing"));
            }
            doc
        }
        ast::MergeAction::MergeUpdate(action) => {
            let mut doc = Doc::text("update");
            if let Some(set_clause) = action.set_clause() {
                doc = doc
                    .append(Doc::line_or_space())
                    .append(leading_comments(set_clause.syntax()))
                    .append(build_set_clause(set_clause));
            }
            doc.group()
        }
        ast::MergeAction::MergeInsert(action) => {
            let mut doc = Doc::text("insert");
            if let Some(columns) = action.column_target_list() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments(columns.syntax()))
                    .append(build_column_target_list(columns));
            }
            if let Some(overriding) = action.overriding_clause() {
                doc = doc
                    .append(Doc::line_or_space())
                    .append(leading_comments(overriding.syntax()))
                    .append(build_overriding_clause(overriding));
            }
            if let Some(values) = action.values() {
                doc = doc
                    .append(Doc::line_or_space())
                    .append(leading_comments(values.syntax()))
                    .append(build_values(&values));
            } else if let Some(default_values) = action.default_values() {
                doc = doc
                    .append(Doc::line_or_space())
                    .append(leading_comments(default_values.syntax()))
                    .append(build_default_values(default_values));
            }
            doc.group()
        }
    }
}

fn build_default_values<'a>(default_values: ast::DefaultValues) -> Doc<'a> {
    let mut doc = Doc::text("default");
    if let Some(token) = default_values.values_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&token))
            .append(Doc::text("values"));
    }
    doc
}

fn build_update<'a>(update: &ast::Update) -> Doc<'a> {
    let mut doc = Doc::nil();
    if let Some(with_clause) = update.with_clause() {
        doc = doc
            .append(leading_comments(with_clause.syntax()))
            .append(build_with_clause(with_clause))
            .append(Doc::hard_line());
        if let Some(update_token) = update.update_token() {
            doc = doc.append(leading_comments_token(&update_token));
        }
    }

    doc = doc.append(Doc::text("update"));
    if let Some(relation) = update.relation_name() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(relation.syntax()))
            .append(build_relation_name(relation));
    }
    if let Some(for_portion_of) = update.for_portion_of() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(for_portion_of.syntax()))
            .append(build_for_portion_of(for_portion_of));
    }
    if let Some(alias) = update.alias() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(alias.syntax()))
            .append(build_optional_as_alias(alias));
    }
    if let Some(set_clause) = update.set_clause() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(set_clause.syntax()))
            .append(build_set_clause(set_clause));
    }
    if let Some(from_clause) = update.from_clause() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(from_clause.syntax()))
            .append(build_from_clause(from_clause));
    }
    if let Some(where_clause) = update.where_clause_or_current_of() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(where_clause.syntax()))
            .append(build_where_clause_or_current_of(where_clause));
    }
    if let Some(returning_clause) = update.returning_clause() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(returning_clause.syntax()))
            .append(build_returning_clause(returning_clause));
    }

    doc.append(build_semicolon(update.semicolon_token()))
        .group()
}

fn build_set_clause<'a>(set_clause: ast::SetClause) -> Doc<'a> {
    let mut doc = set_clause
        .set_token()
        .map(|token| leading_comments_token(&token).append(Doc::text("set")))
        .unwrap_or_else(Doc::nil);
    if let Some(columns) = set_clause.set_column_list() {
        let items = columns.set_columns().map(|column| {
            let syntax = column.syntax().clone();
            (
                leading_comments(column.syntax()).append(build_set_column(column)),
                syntax,
            )
        });
        if let Some(items) = build_comma_separated_docs(items) {
            doc = doc.append(
                Doc::line_or_space()
                    .append(leading_comments(columns.syntax()))
                    .append(items)
                    .nest(2)
                    .group(),
            );
        }
    }
    doc
}

fn build_set_column<'a>(column: ast::SetColumn) -> Doc<'a> {
    match column {
        ast::SetColumn::SetSingleColumn(column) => {
            let mut doc = column
                .column_target()
                .map(build_column_target)
                .unwrap_or_else(Doc::nil);
            if let Some(eq_token) = column.eq_token() {
                doc = doc.append(
                    Doc::line_or_space()
                        .append(leading_comments_token(&eq_token))
                        .append(Doc::text("="))
                        .nest(2),
                );
            }
            if let Some(expr) = column.set_expr() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments(expr.syntax()))
                    .append(build_set_expr(expr));
            }
            doc.group()
        }
        ast::SetColumn::SetMultipleColumns(columns) => {
            let mut doc = columns
                .column_target_list()
                .map(build_column_target_list)
                .unwrap_or_else(Doc::nil);
            if let Some(eq_token) = columns.eq_token() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments_token(&eq_token))
                    .append(Doc::text("="));
            }
            if let Some(exprs) = columns.set_expr_list() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments(exprs.syntax()))
                    .append(build_set_expr_list(exprs));
            } else if let Some(select) = columns.paren_select() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments(select.syntax()))
                    .append(build_paren_select(select));
            }
            doc
        }
    }
}

fn build_column_target_list<'a>(targets: ast::ColumnTargetList) -> Doc<'a> {
    let mut doc = targets
        .l_paren_token()
        .map(comments_before)
        .unwrap_or_else(Doc::nil)
        .append(Doc::text("("));
    let items = targets.column_targets().map(|target| {
        let syntax = target.syntax().clone();
        (
            leading_comments(target.syntax()).append(build_column_target(target)),
            syntax,
        )
    });
    let mut body = build_comma_separated_docs(items).unwrap_or_else(Doc::nil);
    if let Some(r_paren) = targets.r_paren_token() {
        body = body.append(comments_before(r_paren));
    }
    doc = doc.append(wrap_body(body)).append(Doc::text(")"));
    doc.group()
}

fn build_column_target<'a>(target: ast::ColumnTarget) -> Doc<'a> {
    let mut doc = target
        .name()
        .map(|name| build_name(name.syntax()))
        .unwrap_or_else(Doc::nil);
    for accessor in target.accessors() {
        doc = doc
            .append(leading_comments(accessor.syntax()))
            .append(build_accessor(accessor));
    }
    doc
}

fn build_accessor<'a>(accessor: ast::Accessor) -> Doc<'a> {
    match accessor {
        ast::Accessor::FieldAccessor(field) => {
            let mut doc = field
                .dot_token()
                .map(comments_before)
                .unwrap_or_else(Doc::nil)
                .append(Doc::text("."));
            if let Some(star) = field.star_token() {
                doc = doc
                    .append(leading_comments_token(&star))
                    .append(Doc::text("*"));
            } else if let Some(name) = field.composite_field_ref() {
                doc = doc
                    .append(leading_comments(name.syntax()))
                    .append(build_name(name.syntax()));
            }
            doc
        }
        ast::Accessor::IndexAccessor(index) => {
            let mut body = index
                .index()
                .map(|expr| leading_comments(expr.syntax()).append(build_expr(expr)))
                .unwrap_or_else(Doc::nil);
            if let Some(r_brack) = index.r_brack_token() {
                body = body.append(comments_before(r_brack));
            }
            index
                .l_brack_token()
                .map(comments_before)
                .unwrap_or_else(Doc::nil)
                .append(Doc::text("["))
                .append(wrap_body(body))
                .append(Doc::text("]"))
                .group()
        }
        ast::Accessor::SliceAccessor(slice) => {
            let mut body = slice
                .start()
                .map(|expr| leading_comments(expr.syntax()).append(build_expr(expr)))
                .unwrap_or_else(Doc::nil);
            if let Some(colon) = slice.colon_token() {
                body = body.append(comments_before(colon));
            }
            body = body.append(Doc::text(":"));
            if let Some(end) = slice.end() {
                body = body
                    .append(leading_comments(end.syntax()))
                    .append(build_expr(end));
            }
            if let Some(r_brack) = slice.r_brack_token() {
                body = body.append(comments_before(r_brack));
            }
            slice
                .l_brack_token()
                .map(comments_before)
                .unwrap_or_else(Doc::nil)
                .append(Doc::text("["))
                .append(wrap_body(body))
                .append(Doc::text("]"))
                .group()
        }
    }
}

fn build_set_expr_list<'a>(exprs: ast::SetExprList) -> Doc<'a> {
    let mut doc = Doc::nil();
    if let Some(row_token) = exprs.row_token() {
        doc = doc
            .append(leading_comments_token(&row_token))
            .append(Doc::text("row"));
    }
    if let Some(l_paren) = exprs.l_paren_token() {
        doc = doc.append(comments_before(l_paren)).append(Doc::text("("));
    }
    let items = exprs.set_exprs().map(|expr| {
        let syntax = expr.syntax().clone();
        (
            leading_comments(expr.syntax()).append(build_set_expr(expr)),
            syntax,
        )
    });
    let mut body = build_comma_separated_docs(items).unwrap_or_else(Doc::nil);
    if let Some(r_paren) = exprs.r_paren_token() {
        body = body.append(comments_before(r_paren));
    }
    doc.append(wrap_body(body)).append(Doc::text(")")).group()
}

fn build_set_expr<'a>(expr: ast::SetExpr) -> Doc<'a> {
    if let Some(expr) = expr.expr() {
        build_expr(expr)
    } else if let Some(default) = expr.default_token() {
        leading_comments_token(&default).append(Doc::text("default"))
    } else {
        Doc::nil()
    }
}

fn build_truncate<'a>(truncate: &ast::Truncate) -> Doc<'a> {
    let mut doc = Doc::text("truncate");

    if let Some(table_token) = truncate.table_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&table_token))
            .append(Doc::text("table"));
    }

    if let Some(table_list) = truncate.table_list() {
        let tables = Doc::list(
            Itertools::intersperse(
                table_list.table_relation_names().map(|relation| {
                    let trailing = trailing_comments(relation.syntax());
                    build_table_relation_name(relation).append(trailing)
                }),
                Doc::text(",").append(Doc::line_or_space()),
            )
            .collect(),
        );
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(table_list.syntax()))
                .append(tables)
                .nest(2),
        );
    }

    if let Some(identity_action) = truncate.identity_action() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(identity_action.syntax()))
            .append(build_keyword_node(identity_action.syntax()));
    }

    if let Some(drop_behavior) = truncate.drop_behavior() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(drop_behavior.syntax()))
            .append(build_keyword_node(drop_behavior.syntax()));
    }

    doc.append(build_semicolon(truncate.semicolon_token()))
        .group()
}

fn build_table_relation_name<'a>(relation: ast::TableRelationName) -> Doc<'a> {
    let mut doc = leading_comments(relation.syntax());
    let has_only = relation.only_token().is_some();

    if let Some(only_token) = relation.only_token() {
        doc = doc
            .append(leading_comments_token(&only_token))
            .append(Doc::text("only"));
    }
    if let Some(l_paren) = relation.l_paren_token() {
        if has_only && comment_tokens_before(l_paren.clone()).is_empty() {
            doc = doc.append(Doc::space());
        }
        doc = doc.append(comments_before(l_paren)).append(Doc::text("("));
    }
    if let Some(table_name) = relation.table_name_ref() {
        if has_only && relation.l_paren_token().is_none() {
            doc = doc.append(Doc::space());
        }
        doc = doc.append(leading_comments(table_name.syntax()));
        if let Some(path) = table_name.path_ref() {
            doc = doc.append(build_path_ref(&path));
        }
    }
    if let Some(r_paren) = relation.r_paren_token() {
        doc = doc.append(comments_before(r_paren)).append(Doc::text(")"));
    }
    if let Some(star) = relation.star_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&star))
            .append(Doc::text("*"));
    }

    doc
}

fn build_create_function<'a>(create_function: &ast::CreateFunction) -> Doc<'a> {
    let mut doc = Doc::text("create");
    if let Some(or_replace) = create_function.or_replace() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(or_replace.syntax()))
            .append(build_keyword_node(or_replace.syntax()));
    }
    if let Some(function_token) = create_function.function_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&function_token));
    } else {
        doc = doc.append(Doc::space());
    }
    doc = doc.append(Doc::text("function"));
    if let Some(name) = create_function.name() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(name.syntax()));
        if let Some(path) = name.path() {
            doc = doc.append(build_path(&path));
        }
    }
    if let Some(params) = create_function.param_list() {
        doc = doc
            .append(leading_comments(params.syntax()))
            .append(build_function_param_list(params));
    }
    if let Some(ret_type) = create_function.ret_type() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(ret_type.syntax()))
            .append(build_function_ret_type(ret_type.clone()))
            .append(trailing_comments(ret_type.syntax()));
    }
    doc = doc.group();

    if let Some(options) = create_function.option_list() {
        for option in options.options() {
            doc = doc.append(
                Doc::hard_line()
                    .append(leading_comments(option.syntax()))
                    .append(build_function_option(option))
                    .nest(2),
            );
        }
    }
    doc.append(build_semicolon(create_function.semicolon_token()))
}

fn build_function_param_list<'a>(params: ast::ParamList) -> Doc<'a> {
    let doc = params
        .l_paren_token()
        .map(comments_before)
        .unwrap_or_else(Doc::nil)
        .append(Doc::text("("));
    let mut body = if let Some(star) = params.star_token() {
        leading_comments_token(&star).append(Doc::text("*"))
    } else {
        let param_docs = params.params().map(|param| {
            let syntax = param.syntax().clone();
            (
                leading_comments(param.syntax()).append(build_function_param(param)),
                syntax,
            )
        });
        build_comma_separated_docs(param_docs).unwrap_or_else(Doc::nil)
    };
    if let Some(r_paren) = params.r_paren_token() {
        body = body.append(comments_before(r_paren));
    }
    doc.append(wrap_body(body)).append(Doc::text(")"))
}

fn build_function_param<'a>(param: ast::Param) -> Doc<'a> {
    let mut doc = Doc::nil();
    if let Some(mode) = param.mode() {
        doc = doc
            .append(leading_comments(mode.syntax()))
            .append(build_keyword_node(mode.syntax()));
    }
    if let Some(name) = param.name() {
        if param.mode().is_some() {
            doc = doc.append(Doc::space());
        }
        doc = doc
            .append(leading_comments(name.syntax()))
            .append(build_name(name.syntax()));
    }
    if let Some(ty) = param.ty() {
        if param.mode().is_some() || param.name().is_some() {
            doc = doc.append(Doc::space());
        }
        doc = doc
            .append(leading_comments(ty.syntax()))
            .append(build_type(ty));
    }
    if let Some(default) = param.param_default() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(default.syntax()));
        if let Some(default_token) = default.default_token() {
            doc = doc
                .append(leading_comments_token(&default_token))
                .append(Doc::text("default"));
        } else if let Some(eq_token) = default.eq_token() {
            doc = doc
                .append(leading_comments_token(&eq_token))
                .append(Doc::text("="));
        }
        if let Some(expr) = default.expr() {
            doc = doc
                .append(Doc::space())
                .append(leading_comments(expr.syntax()))
                .append(build_expr(expr));
        }
    }
    doc
}

fn build_function_ret_type<'a>(ret_type: ast::RetType) -> Doc<'a> {
    let mut doc = Doc::text("returns");
    if let Some(table_token) = ret_type.table_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&table_token))
            .append(Doc::text("table"));
    }
    if let Some(args) = ret_type.table_arg_list() {
        let mut body = Doc::list(
            Itertools::intersperse(
                args.args().map(build_table_arg),
                Doc::text(",").append(Doc::line_or_space()),
            )
            .collect(),
        );
        if args.args().next().is_none() {
            if let Some(r_paren) = args.r_paren_token() {
                body = body.append(comments_before(r_paren));
            }
        }
        let args_doc = args
            .l_paren_token()
            .map(comments_before)
            .unwrap_or_else(Doc::nil)
            .append(Doc::text("("))
            .append(wrap_body(body))
            .append(Doc::text(")"))
            .group();
        doc = doc
            .append(Doc::space())
            .append(leading_comments(args.syntax()))
            .append(args_doc);
    } else if let Some(ty) = ret_type.ty() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(ty.syntax()))
            .append(build_type(ty));
    }
    doc
}

fn build_function_option<'a>(option: ast::FuncOption) -> Doc<'a> {
    match option {
        ast::FuncOption::AsFuncOption(option) => build_as_function_option(option),
        ast::FuncOption::BeginFuncOptionList(options) => build_begin_function_option_list(options),
        ast::FuncOption::CostFuncOption(option) => {
            build_literal_function_option("cost", option.literal())
        }
        ast::FuncOption::LanguageFuncOption(option) => {
            let mut doc = Doc::text("language");
            if let Some(language) = option.language_ref() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments(language.syntax()))
                    .append(build_name(language.syntax()));
            } else if let Some(literal) = option.literal() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments(literal.syntax()))
                    .append(build_literal(literal));
            }
            doc
        }
        ast::FuncOption::ResetFuncOption(option) => {
            let mut doc = Doc::text("reset");
            if let Some(all) = option.all_token() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments_token(&all))
                    .append(Doc::text("all"));
            } else if let Some(parameter) = option.config_parameter_ref() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments(parameter.syntax()));
                if let Some(path) = parameter.path_ref() {
                    doc = doc.append(build_path_ref(&path));
                }
            }
            doc
        }
        ast::FuncOption::ReturnFuncOption(option) => build_return_function_option(option),
        ast::FuncOption::RowsFuncOption(option) => {
            build_literal_function_option("rows", option.literal())
        }
        ast::FuncOption::SetFuncOption(option) => option
            .set_config_param()
            .map(build_set_config_param)
            .unwrap_or_else(Doc::nil),
        ast::FuncOption::SupportFuncOption(option) => {
            let mut doc = Doc::text("support");
            if let Some(function) = option.function_name_ref() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments(function.syntax()));
                if let Some(path) = function.path_ref() {
                    doc = doc.append(build_path_ref(&path));
                }
            }
            doc
        }
        ast::FuncOption::TransformFuncOption(option) => {
            let transforms = option.transform_for_types().map(|transform| {
                let syntax = transform.syntax().clone();
                let mut doc = leading_comments(transform.syntax()).append(Doc::text("for"));
                if let Some(type_token) = transform.type_token() {
                    doc = doc
                        .append(Doc::space())
                        .append(leading_comments_token(&type_token))
                        .append(Doc::text("type"));
                }
                if let Some(ty) = transform.ty() {
                    doc = doc
                        .append(Doc::space())
                        .append(leading_comments(ty.syntax()))
                        .append(build_type(ty));
                }
                (doc, syntax)
            });
            Doc::text("transform").append(
                Doc::space()
                    .append(build_comma_separated_docs(transforms).unwrap_or_else(Doc::nil)),
            )
        }
        ast::FuncOption::CalledOnNullInputFuncOption(option) => build_keyword_node(option.syntax()),
        ast::FuncOption::LeakproofFuncOption(option) => build_keyword_node(option.syntax()),
        ast::FuncOption::NotLeakproofFuncOption(option) => build_keyword_node(option.syntax()),
        ast::FuncOption::ParallelFuncOption(option) => build_keyword_node(option.syntax()),
        ast::FuncOption::ReturnsNullOnNullInputFuncOption(option) => {
            build_keyword_node(option.syntax())
        }
        ast::FuncOption::SecurityDefinerFuncOption(option) => build_keyword_node(option.syntax()),
        ast::FuncOption::SecurityInvokerFuncOption(option) => build_keyword_node(option.syntax()),
        ast::FuncOption::StrictFuncOption(option) => build_keyword_node(option.syntax()),
        ast::FuncOption::VolatilityFuncOption(option) => build_keyword_node(option.syntax()),
        ast::FuncOption::WindowFuncOption(option) => build_keyword_node(option.syntax()),
    }
}

fn build_begin_function_option_list<'a>(options: ast::BeginFuncOptionList) -> Doc<'a> {
    let mut doc = Doc::text("begin");
    if let Some(atomic) = options.atomic_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&atomic))
            .append(Doc::text("atomic"));
    }

    for option in options.begin_func_options() {
        let option_comments = leading_comments(option.syntax());
        let option_doc = match option {
            ast::BeginFuncOption::ReturnFuncOption(option) => build_return_function_option(option),
            ast::BeginFuncOption::Stmt(stmt) => build_stmt(stmt),
        };
        doc = doc.append(
            Doc::hard_line()
                .append(option_comments)
                .append(option_doc)
                .nest(2),
        );
    }

    let end_doc = options
        .end_token()
        .map(|end| leading_comments_token(&end))
        .unwrap_or_else(Doc::nil)
        .append(Doc::text("end"));
    doc.append(Doc::hard_line()).append(end_doc)
}

fn build_call<'a>(call: &ast::Call) -> Doc<'a> {
    let mut doc = Doc::text("call");
    if let Some(procedure) = call.procedure_name_ref() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(procedure.syntax()));
        if let Some(path) = procedure.path_ref() {
            doc = doc.append(build_path_ref(&path));
        }
    }
    if let Some(args) = call.arg_list() {
        doc = doc
            .append(comments_before(args.syntax().clone()))
            .append(build_call_arg_list(args));
    }
    doc.group().append(build_semicolon(call.semicolon_token()))
}

fn build_checkpoint<'a>(checkpoint: &ast::Checkpoint) -> Doc<'a> {
    let mut doc = Doc::text("checkpoint");
    if let Some(options) = checkpoint.checkpoint_option_list() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(options.syntax()))
            .append(build_checkpoint_option_list(options));
    }
    doc.group()
        .append(build_semicolon(checkpoint.semicolon_token()))
}

fn build_checkpoint_option_list<'a>(list: ast::CheckpointOptionList) -> Doc<'a> {
    let mut body = build_comma_separated_docs(list.checkpoint_options().map(|option| {
        (
            leading_comments(option.syntax()).append(build_checkpoint_option(option.clone())),
            option.syntax().clone(),
        )
    }))
    .unwrap_or_else(Doc::nil);
    if let Some(r_paren) = list.r_paren_token() {
        body = body.append(comments_before(r_paren));
    }

    list.l_paren_token()
        .map(comments_before)
        .unwrap_or_else(Doc::nil)
        .append(Doc::text("("))
        .append(wrap_body(body))
        .append(Doc::text(")"))
        .group()
}

fn build_checkpoint_option<'a>(option: ast::CheckpointOption) -> Doc<'a> {
    let mut doc = option
        .checkpoint_option_name()
        .map(|name| build_keyword_node(name.syntax()))
        .unwrap_or_else(Doc::nil);
    if let Some(expr) = option.expr() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(expr.syntax()))
            .append(build_expr(expr));
    }
    doc.group()
}

fn build_deallocate<'a>(deallocate: &ast::Deallocate) -> Doc<'a> {
    let mut doc = Doc::text("deallocate");

    if let Some(prepare) = deallocate.prepare_token() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments_token(&prepare))
                .append(Doc::text("prepare"))
                .nest(2),
        );
    }

    let target = if let Some(statement) = deallocate.prepared_statement_ref() {
        Some(leading_comments(statement.syntax()).append(build_name(statement.syntax())))
    } else {
        deallocate
            .all_token()
            .map(|all| leading_comments_token(&all).append(Doc::text("all")))
    };
    if let Some(target) = target {
        doc = doc.append(Doc::line_or_space().append(target).nest(2));
    }

    doc.group()
        .append(build_semicolon(deallocate.semicolon_token()))
}

fn build_declare<'a>(declare: &ast::Declare) -> Doc<'a> {
    let mut doc = Doc::text("declare");

    if let Some(cursor) = declare.cursor() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(cursor.syntax()))
                .append(build_name(cursor.syntax()))
                .nest(2),
        );
    }
    if let Some(binary) = declare.binary_token() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments_token(&binary))
                .append(Doc::text("binary"))
                .nest(2),
        );
    }
    if let Some(sensitivity) = declare.cursor_sensitivity() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(sensitivity.syntax()))
                .append(build_keyword_node(sensitivity.syntax()))
                .nest(2),
        );
    }
    if let Some(scroll) = declare.cursor_scroll() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(scroll.syntax()))
                .append(build_keyword_node(scroll.syntax()))
                .nest(2),
        );
    }
    if let Some(cursor) = declare.cursor_token() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments_token(&cursor))
                .append(Doc::text("cursor"))
                .nest(2),
        );
    }
    if let Some(hold) = declare.cursor_hold() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(hold.syntax()))
                .append(build_keyword_node(hold.syntax()))
                .nest(2),
        );
    }
    doc = doc.group();

    let has_for = if let Some(for_token) = declare.for_token() {
        doc = doc
            .append(Doc::hard_line())
            .append(leading_comments_token(&for_token))
            .append(Doc::text("for"));
        true
    } else {
        false
    };
    if let Some(query) = declare.query() {
        doc = doc
            .append(if has_for {
                Doc::space()
            } else {
                Doc::hard_line()
            })
            .append(leading_comments(query.syntax()))
            .append(build_select_variant(query));
    }

    doc.append(build_semicolon(declare.semicolon_token()))
}

fn build_lock<'a>(lock: &ast::Lock) -> Doc<'a> {
    let mut doc = Doc::text("lock");
    if let Some(table) = lock.table_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&table))
            .append(Doc::text("table"));
    }
    if let Some(relations) = lock.relation_list() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(relations.syntax()))
                .append(build_lock_relation_list(relations))
                .nest(2),
        );
    }
    if let Some(mode) = lock.lock_mode_clause() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(mode.syntax()))
                .append(build_lock_mode_clause(mode))
                .nest(2),
        );
    }
    if let Some(nowait) = lock.nowait() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(nowait.syntax()))
                .append(build_keyword_node(nowait.syntax()))
                .nest(2),
        );
    }
    doc.group().append(build_semicolon(lock.semicolon_token()))
}

fn build_lock_relation_list<'a>(list: ast::RelationList) -> Doc<'a> {
    build_comma_separated_docs(list.relation_names().map(|relation| {
        (
            leading_comments(relation.syntax()).append(build_relation_name(relation.clone())),
            relation.syntax().clone(),
        )
    }))
    .unwrap_or_else(Doc::nil)
}

fn build_lock_mode_clause<'a>(clause: ast::LockModeClause) -> Doc<'a> {
    let mut doc = Doc::text("in");
    if let Some(mode) = clause.lock_mode() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(mode.syntax()))
            .append(build_keyword_node(mode.syntax()));
    }
    if let Some(mode_token) = clause.mode_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&mode_token))
            .append(Doc::text("mode"));
    }
    doc.group()
}

fn build_reindex<'a>(reindex: &ast::Reindex) -> Doc<'a> {
    let mut doc = Doc::text("reindex");
    if let Some(options) = reindex.reindex_option_list() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(options.syntax()))
            .append(build_reindex_option_list(options));
    }
    if let Some(target) = reindex.reindex_target() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(target.syntax()))
                .append(build_reindex_target(target))
                .nest(2),
        );
    }
    doc.group()
        .append(build_semicolon(reindex.semicolon_token()))
}

fn build_reindex_option_list<'a>(list: ast::ReindexOptionList) -> Doc<'a> {
    let mut body = build_comma_separated_docs(list.reindex_options().map(|option| {
        (
            leading_comments(option.syntax()).append(build_reindex_option(option.clone())),
            option.syntax().clone(),
        )
    }))
    .unwrap_or_else(Doc::nil);
    if let Some(r_paren) = list.r_paren_token() {
        body = body.append(comments_before(r_paren));
    }

    list.l_paren_token()
        .map(comments_before)
        .unwrap_or_else(Doc::nil)
        .append(Doc::text("("))
        .append(wrap_body(body))
        .append(Doc::text(")"))
        .group()
}

fn build_reindex_option<'a>(option: ast::ReindexOption) -> Doc<'a> {
    match option {
        ast::ReindexOption::ReindexOptionConcurrently(option) => {
            let doc = Doc::text("concurrently");
            build_reindex_boolean_option_value(
                doc,
                option.literal(),
                option.ident_token(),
                option.no_token(),
                option.yes_token(),
            )
        }
        ast::ReindexOption::ReindexOptionVerbose(option) => {
            let doc = Doc::text("verbose");
            build_reindex_boolean_option_value(
                doc,
                option.literal(),
                option.ident_token(),
                option.no_token(),
                option.yes_token(),
            )
        }
        ast::ReindexOption::ReindexOptionTablespace(option) => {
            let mut doc = Doc::text("tablespace");
            if let Some(tablespace) = option.tablespace_ref() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments(tablespace.syntax()))
                    .append(build_name(tablespace.syntax()));
            }
            doc.group()
        }
    }
}

fn build_reindex_boolean_option_value<'a>(
    mut doc: Doc<'a>,
    literal: Option<ast::Literal>,
    ident: Option<SyntaxToken>,
    no: Option<SyntaxToken>,
    yes: Option<SyntaxToken>,
) -> Doc<'a> {
    let value = if let Some(literal) = literal {
        Some(leading_comments(literal.syntax()).append(build_literal(literal)))
    } else if let Some(ident) = ident {
        let text = if ident.text().starts_with('"') {
            ident.text().to_string()
        } else {
            ident.text().to_ascii_lowercase()
        };
        Some(leading_comments_token(&ident).append(Doc::text(text)))
    } else if let Some(no) = no {
        Some(leading_comments_token(&no).append(Doc::text("no")))
    } else {
        yes.map(|yes| leading_comments_token(&yes).append(Doc::text("yes")))
    };
    if let Some(value) = value {
        doc = doc.append(Doc::space()).append(value);
    }
    doc.group()
}

fn build_reindex_target<'a>(target: ast::ReindexTarget) -> Doc<'a> {
    match target {
        ast::ReindexTarget::ReindexTargetDatabase(target) => {
            let name = target
                .database_ref()
                .map(|name| leading_comments(name.syntax()).append(build_name(name.syntax())));
            build_reindex_target_parts("database", target.concurrently_token(), name)
        }
        ast::ReindexTarget::ReindexTargetIndex(target) => {
            let name = target.index_ref().map(|name| {
                let mut doc = leading_comments(name.syntax());
                if let Some(path) = name.path_ref() {
                    doc = doc.append(build_path_ref(&path));
                }
                doc
            });
            build_reindex_target_parts("index", target.concurrently_token(), name)
        }
        ast::ReindexTarget::ReindexTargetSchema(target) => {
            let name = target
                .schema_ref()
                .map(|name| leading_comments(name.syntax()).append(build_name(name.syntax())));
            build_reindex_target_parts("schema", target.concurrently_token(), name)
        }
        ast::ReindexTarget::ReindexTargetSystem(target) => {
            let name = target
                .database_ref()
                .map(|name| leading_comments(name.syntax()).append(build_name(name.syntax())));
            build_reindex_target_parts("system", target.concurrently_token(), name)
        }
        ast::ReindexTarget::ReindexTargetTable(target) => {
            let name = target.table_name_ref().map(|name| {
                let mut doc = leading_comments(name.syntax());
                if let Some(path) = name.path_ref() {
                    doc = doc.append(build_path_ref(&path));
                }
                doc
            });
            build_reindex_target_parts("table", target.concurrently_token(), name)
        }
    }
}

fn build_reindex_target_parts<'a>(
    keyword: &'static str,
    concurrently: Option<SyntaxToken>,
    name: Option<Doc<'a>>,
) -> Doc<'a> {
    let mut doc = Doc::text(keyword);
    if let Some(concurrently) = concurrently {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&concurrently))
            .append(Doc::text("concurrently"));
    }
    if let Some(name) = name {
        doc = doc.append(Doc::space()).append(name);
    }
    doc.group()
}

fn build_reset<'a>(reset: &ast::Reset) -> Doc<'a> {
    let mut doc = Doc::text("reset");
    if let Some(target) = reset.reset_target() {
        let target_doc = leading_comments(target.syntax()).append(match target {
            ast::ResetTarget::All(target) => build_keyword_node(target.syntax()),
            ast::ResetTarget::ConfigParameterRef(target) => {
                if let Some(path) = target.path_ref() {
                    build_path_ref(&path)
                } else {
                    Doc::nil()
                }
            }
            ast::ResetTarget::ResetTimeZone(target) => build_keyword_node(target.syntax()),
            ast::ResetTarget::ResetTransactionIsolation(target) => {
                build_keyword_node(target.syntax())
            }
        });
        doc = doc.append(Doc::line_or_space().append(target_doc).nest(2));
    }
    doc.group().append(build_semicolon(reset.semicolon_token()))
}

fn build_load<'a>(load: &ast::Load) -> Doc<'a> {
    let mut doc = Doc::text("load");
    if let Some(literal) = load.literal() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(literal.syntax()))
                .append(build_literal(literal))
                .nest(2),
        );
    }
    doc.group().append(build_semicolon(load.semicolon_token()))
}

fn build_discard<'a>(discard: &ast::Discard) -> Doc<'a> {
    let mut doc = Doc::text("discard");
    if let Some(target) = discard.discard_target() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(target.syntax()))
                .append(build_keyword_node(target.syntax()))
                .nest(2),
        );
    }
    doc.group()
        .append(build_semicolon(discard.semicolon_token()))
}

fn build_fetch<'a>(fetch: &ast::Fetch) -> Doc<'a> {
    let mut doc = Doc::text("fetch");
    if let Some(action) = fetch.cursor_action() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(action.syntax()))
                .append(build_cursor_action(action))
                .nest(2),
        );
    }
    if let Some(token) = fetch.from_token().or_else(|| fetch.in_token()) {
        let keyword = if token.kind() == SyntaxKind::FROM_KW {
            "from"
        } else {
            "in"
        };
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments_token(&token))
                .append(Doc::text(keyword))
                .nest(2),
        );
    }
    if let Some(cursor) = fetch.cursor_ref() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(cursor.syntax()))
                .append(build_name(cursor.syntax()))
                .nest(2),
        );
    }
    doc.group().append(build_semicolon(fetch.semicolon_token()))
}

fn build_cursor_action<'a>(action: ast::CursorAction) -> Doc<'a> {
    match action {
        ast::CursorAction::Absolute(action) => build_cursor_action_expr("absolute", action.expr()),
        ast::CursorAction::Relative(action) => build_cursor_action_expr("relative", action.expr()),
        ast::CursorAction::Backward(action) => {
            build_cursor_action_optional_value("backward", action.all_token(), action.expr())
        }
        ast::CursorAction::Forward(action) => {
            build_cursor_action_optional_value("forward", action.all_token(), action.expr())
        }
        ast::CursorAction::All(action) => build_keyword_node(action.syntax()),
        ast::CursorAction::First(action) => build_keyword_node(action.syntax()),
        ast::CursorAction::Last(action) => build_keyword_node(action.syntax()),
        ast::CursorAction::Next(action) => build_keyword_node(action.syntax()),
        ast::CursorAction::Prior(action) => build_keyword_node(action.syntax()),
        ast::CursorAction::Expr(expr) => build_expr(expr),
    }
}

fn build_cursor_action_expr<'a>(keyword: &'static str, expr: Option<ast::Expr>) -> Doc<'a> {
    let mut doc = Doc::text(keyword);
    if let Some(expr) = expr {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(expr.syntax()))
            .append(build_expr(expr));
    }
    doc.group()
}

fn build_cursor_action_optional_value<'a>(
    keyword: &'static str,
    all_token: Option<SyntaxToken>,
    expr: Option<ast::Expr>,
) -> Doc<'a> {
    let mut doc = Doc::text(keyword);
    if let Some(all) = all_token {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&all))
            .append(Doc::text("all"));
    } else if let Some(expr) = expr {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(expr.syntax()))
            .append(build_expr(expr));
    }
    doc.group()
}

fn build_close<'a>(close: &ast::Close) -> Doc<'a> {
    let target = if let Some(all) = close.all_token() {
        Some(leading_comments_token(&all).append(Doc::text("all")))
    } else {
        close
            .cursor_ref()
            .map(|cursor| leading_comments(cursor.syntax()).append(build_name(cursor.syntax())))
    };

    let mut doc = Doc::text("close");
    if let Some(target) = target {
        doc = doc.append(Doc::line_or_space().append(target).nest(2));
    }
    doc.group().append(build_semicolon(close.semicolon_token()))
}

fn build_cluster<'a>(cluster: &ast::Cluster) -> Doc<'a> {
    let mut doc = Doc::text("cluster");

    if let Some(verbose) = cluster.verbose_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&verbose))
            .append(Doc::text("verbose"));
    } else if let Some(options) = cluster.option_item_list() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(options.syntax()))
                .append(build_option_item_list(options))
                .nest(2),
        );
    }

    if let Some(table) = cluster.table_name_ref() {
        let mut table_doc = leading_comments(table.syntax());
        if let Some(path) = table.path_ref() {
            table_doc = table_doc.append(build_path_ref(&path));
        }
        doc = doc.append(Doc::line_or_space().append(table_doc).nest(2));
        if let Some(using_index) = cluster.cluster_using_index() {
            doc = doc
                .append(Doc::line_or_space())
                .append(leading_comments(using_index.syntax()))
                .append(build_cluster_using_index(using_index));
        }
    } else if let Some(legacy) = cluster.cluster_legacy() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(legacy.syntax()))
                .append(build_cluster_legacy(legacy))
                .nest(2),
        );
    }

    doc.group()
        .append(build_semicolon(cluster.semicolon_token()))
}

fn build_cluster_using_index<'a>(using_index: ast::ClusterUsingIndex) -> Doc<'a> {
    let mut doc = Doc::text("using");
    if let Some(index) = using_index.index_ref() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(index.syntax()));
        if let Some(path) = index.path_ref() {
            doc = doc.append(build_path_ref(&path));
        }
    }
    doc
}

fn build_cluster_legacy<'a>(legacy: ast::ClusterLegacy) -> Doc<'a> {
    let mut doc = Doc::nil();
    if let Some(index) = legacy.index_ref() {
        doc = doc.append(leading_comments(index.syntax()));
        if let Some(path) = index.path_ref() {
            doc = doc.append(build_path_ref(&path));
        }
    }
    if let Some(on_path) = legacy.on_path() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(on_path.syntax()))
            .append(Doc::text("on"));
        if let Some(table) = on_path.table_name_ref() {
            doc = doc
                .append(Doc::space())
                .append(leading_comments(table.syntax()));
            if let Some(path) = table.path_ref() {
                doc = doc.append(build_path_ref(&path));
            }
        }
    }
    doc.group()
}

fn build_option_item_list<'a>(list: ast::OptionItemList) -> Doc<'a> {
    let mut body = build_comma_separated_docs(list.option_items().map(|option| {
        (
            leading_comments(option.syntax()).append(build_option_item(option.clone())),
            option.syntax().clone(),
        )
    }))
    .unwrap_or_else(Doc::nil);
    if let Some(r_paren) = list.r_paren_token() {
        body = body.append(comments_before(r_paren));
    }

    list.l_paren_token()
        .map(comments_before)
        .unwrap_or_else(Doc::nil)
        .append(Doc::text("("))
        .append(wrap_body(body))
        .append(Doc::text(")"))
        .group()
}

fn build_option_item<'a>(option: ast::OptionItem) -> Doc<'a> {
    let mut doc = option
        .option_item_key()
        .map(|key| build_keyword_node(key.syntax()))
        .unwrap_or_else(Doc::nil);
    if let Some(value) = option.option_item_value() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(value.syntax()))
            .append(build_option_item_value(value));
    }
    doc.group()
}

fn build_option_item_value<'a>(value: ast::OptionItemValue) -> Doc<'a> {
    if let Some(expr) = value.expr() {
        build_expr(expr)
    } else if let Some(name) = value.option_item_value_name() {
        build_keyword_node(name.syntax())
    } else {
        Doc::text("default")
    }
}

fn build_analyze_option_list<'a>(list: ast::OptionItemList) -> Doc<'a> {
    let mut body = build_comma_separated_docs(list.option_items().map(|option| {
        let syntax = option.syntax().clone();
        (
            leading_comments(&syntax).append(build_option_item(option)),
            syntax,
        )
    }))
    .unwrap_or_else(Doc::nil);
    if let Some(r_paren) = list.r_paren_token() {
        body = body.append(comments_before(r_paren));
    }

    list.l_paren_token()
        .map(comments_before)
        .unwrap_or_else(Doc::nil)
        .append(Doc::text("("))
        .append(Doc::hard_line().append(body.group()).nest(2))
        .append(Doc::hard_line())
        .append(Doc::text(")"))
}

fn build_analyze<'a>(analyze: &ast::Analyze) -> Doc<'a> {
    let mut doc = if analyze.analyse_token().is_some() {
        Doc::text("analyse")
    } else {
        Doc::text("analyze")
    };

    if let Some(verbose) = analyze.verbose_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&verbose))
            .append(Doc::text("verbose"));
    }
    if let Some(options) = analyze.option_item_list() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(options.syntax()))
            .append(build_analyze_option_list(options));
    }
    if let Some(tables) = analyze.table_and_columns_list() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(tables.syntax()))
                .append(build_table_and_columns_list(tables))
                .nest(2),
        );
    }

    doc.group()
        .append(build_semicolon(analyze.semicolon_token()))
}

fn build_vacuum<'a>(vacuum: &ast::Vacuum) -> Doc<'a> {
    let mut doc = Doc::text("vacuum");

    for (token, keyword) in [
        (vacuum.full_token(), "full"),
        (vacuum.freeze_token(), "freeze"),
        (vacuum.verbose_token(), "verbose"),
        (vacuum.analyze_token(), "analyze"),
        (vacuum.analyse_token(), "analyse"),
    ] {
        if let Some(token) = token {
            doc = doc
                .append(Doc::space())
                .append(leading_comments_token(&token))
                .append(Doc::text(keyword));
        }
    }

    if let Some(options) = vacuum.vacuum_option_list() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(options.syntax()))
            .append(build_vacuum_option_list(options));
    }

    if let Some(tables) = vacuum.table_and_columns_list() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(tables.syntax()))
                .append(build_table_and_columns_list(tables))
                .nest(2),
        );
    }

    doc.group()
        .append(build_semicolon(vacuum.semicolon_token()))
}

fn build_vacuum_option_list<'a>(list: ast::VacuumOptionList) -> Doc<'a> {
    let mut body = build_comma_separated_docs(list.vacuum_options().map(|option| {
        (
            leading_comments(option.syntax()).append(build_vacuum_option(option.clone())),
            option.syntax().clone(),
        )
    }))
    .unwrap_or_else(Doc::nil);
    if let Some(r_paren) = list.r_paren_token() {
        body = body.append(comments_before(r_paren));
    }

    list.l_paren_token()
        .map(comments_before)
        .unwrap_or_else(Doc::nil)
        .append(Doc::text("("))
        .append(wrap_body(body))
        .append(Doc::text(")"))
        .group()
}

fn build_vacuum_option<'a>(option: ast::VacuumOption) -> Doc<'a> {
    let mut doc = option
        .vacuum_option_name()
        .map(|name| build_keyword_node(name.syntax()))
        .unwrap_or_else(Doc::nil);
    if let Some(value) = option.vacuum_option_value() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(value.syntax()))
            .append(build_vacuum_option_value(value));
    }
    doc.group()
}

fn build_vacuum_option_value<'a>(value: ast::VacuumOptionValue) -> Doc<'a> {
    if let Some(expr) = value.expr() {
        build_expr(expr)
    } else if let Some(name) = value.vacuum_option_value_name() {
        build_keyword_node(name.syntax())
    } else if value.no_token().is_some() {
        Doc::text("no")
    } else if value.yes_token().is_some() {
        Doc::text("yes")
    } else {
        unreachable!("vacuum option value must have a value")
    }
}

fn build_table_and_columns_list<'a>(list: ast::TableAndColumnsList) -> Doc<'a> {
    build_comma_separated_docs(list.table_and_columnss().map(|table| {
        (
            leading_comments(table.syntax()).append(build_table_and_columns(table.clone())),
            table.syntax().clone(),
        )
    }))
    .unwrap_or_else(Doc::nil)
}

fn build_table_and_columns<'a>(table: ast::TableAndColumns) -> Doc<'a> {
    let mut doc = table
        .table_relation_name()
        .map(build_table_relation_name)
        .unwrap_or_else(Doc::nil);
    if let Some(columns) = table.column_ref_list() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(columns.syntax()))
            .append(build_column_ref_list(columns));
    }
    doc
}

fn build_do<'a>(do_stmt: &ast::Do) -> Doc<'a> {
    let language = do_stmt.do_language();
    let body = do_stmt.body();
    let language_before_body = match (&language, &body) {
        (Some(language), Some(body)) => {
            language.syntax().text_range().start() < body.syntax().text_range().start()
        }
        _ => false,
    };

    let mut doc = Doc::text("do");
    if language_before_body {
        if let Some(language) = language.as_ref() {
            doc = doc.append(
                Doc::line_or_space()
                    .append(leading_comments(language.syntax()))
                    .append(build_do_language(language.clone()))
                    .nest(2),
            );
        }
    }
    if let Some(body) = body {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(body.syntax()))
                .append(build_literal(body))
                .nest(2),
        );
    }
    if !language_before_body {
        if let Some(language) = language {
            doc = doc.append(
                Doc::line_or_space()
                    .append(leading_comments(language.syntax()))
                    .append(build_do_language(language))
                    .nest(2),
            );
        }
    }

    doc.group()
        .append(build_semicolon(do_stmt.semicolon_token()))
}

fn build_do_language<'a>(language: ast::DoLanguage) -> Doc<'a> {
    let mut doc = Doc::text("language");
    if let Some(name) = language.language_ref() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(name.syntax()))
            .append(build_name(name.syntax()));
    } else if let Some(literal) = language.literal() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(literal.syntax()))
            .append(build_literal(literal));
    }
    doc
}

fn build_copy<'a>(copy: &ast::Copy) -> Doc<'a> {
    let mut doc = Doc::text("copy");

    if let Some(query) = copy.copy_query() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(query.syntax()))
            .append(build_copy_query(query));
    } else if let Some(table) = copy.copy_table() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(table.syntax()))
            .append(build_copy_table(table));
    }

    if let Some(direction) = copy.copy_direction() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(direction.syntax()))
                .append(build_copy_direction(direction)),
        );
    }

    let has_with = copy.with_token().is_some();
    if let Some(with) = copy.with_token() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments_token(&with))
            .append(Doc::text("with"));
    }

    if let Some(options) = copy.copy_option_list() {
        doc = doc
            .append(if has_with {
                Doc::space()
            } else {
                Doc::line_or_space()
            })
            .append(leading_comments(options.syntax()))
            .append(build_copy_option_list(options));
    } else {
        for option in copy.copy_legacy_options() {
            doc = doc
                .append(Doc::line_or_space())
                .append(leading_comments(option.syntax()))
                .append(build_copy_legacy_option(option));
        }
    }

    if let Some(where_clause) = copy.where_clause() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(where_clause.syntax()))
            .append(build_where_clause(where_clause));
    }

    doc.group().append(build_semicolon(copy.semicolon_token()))
}

fn build_copy_query<'a>(query: ast::CopyQuery) -> Doc<'a> {
    let mut body = Doc::nil();
    if let Some(stmt) = query.preparable_stmt() {
        body = body
            .append(leading_comments(stmt.syntax()))
            .append(build_preparable_stmt(stmt));
    }
    if let Some(r_paren) = query.r_paren_token() {
        body = body.append(comments_before(r_paren));
    }

    query
        .l_paren_token()
        .map(comments_before)
        .unwrap_or_else(Doc::nil)
        .append(Doc::text("("))
        .append(wrap_body(body))
        .append(Doc::text(")"))
        .group()
}

fn build_preparable_stmt<'a>(stmt: ast::PreparableStmt) -> Doc<'a> {
    match stmt {
        ast::PreparableStmt::CompoundSelect(stmt) => build_compound_select(&stmt),
        ast::PreparableStmt::Delete(stmt) => build_delete(&stmt),
        ast::PreparableStmt::Insert(stmt) => build_insert(&stmt),
        ast::PreparableStmt::Merge(stmt) => build_merge(&stmt),
        ast::PreparableStmt::Select(stmt) => build_select_doc(&stmt),
        ast::PreparableStmt::SelectInto(stmt) => build_select_into(&stmt),
        ast::PreparableStmt::Table(stmt) => build_table(&stmt),
        ast::PreparableStmt::Update(stmt) => build_update(&stmt),
        ast::PreparableStmt::Values(stmt) => build_values(&stmt),
    }
}

fn build_copy_table<'a>(table: ast::CopyTable) -> Doc<'a> {
    let mut doc = Doc::nil();
    if table.binary_token().is_some() {
        doc = doc.append(Doc::text("binary")).append(Doc::space());
    }
    if let Some(name) = table.table_name_ref() {
        doc = doc.append(leading_comments(name.syntax()));
        if let Some(path) = name.path_ref() {
            doc = doc.append(build_path_ref(&path));
        }
    }
    if let Some(columns) = table.column_ref_list() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(columns.syntax()))
            .append(build_column_ref_list(columns));
    }
    doc
}

fn build_copy_direction<'a>(direction: ast::CopyDirection) -> Doc<'a> {
    match direction {
        ast::CopyDirection::CopyFrom(from) => {
            let mut doc = Doc::text("from");
            if let Some(source) = from.copy_source() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments(source.syntax()))
                    .append(build_copy_source(source));
            }
            doc
        }
        ast::CopyDirection::CopyTo(to) => {
            let mut doc = Doc::text("to");
            if let Some(target) = to.copy_target() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments(target.syntax()))
                    .append(build_copy_target(target));
            }
            doc
        }
    }
}

fn build_copy_source<'a>(source: ast::CopySource) -> Doc<'a> {
    match source {
        ast::CopySource::CopyProgram(program) => build_copy_program(program),
        ast::CopySource::CopyStdin(_) => Doc::text("stdin"),
        ast::CopySource::CopyStdout(_) => Doc::text("stdout"),
    }
}

fn build_copy_target<'a>(target: ast::CopyTarget) -> Doc<'a> {
    match target {
        ast::CopyTarget::CopyProgram(program) => build_copy_program(program),
        ast::CopyTarget::CopyStdout(_) => Doc::text("stdout"),
    }
}

fn build_copy_program<'a>(program: ast::CopyProgram) -> Doc<'a> {
    let mut doc = Doc::nil();
    if program.program_token().is_some() {
        doc = doc.append(Doc::text("program"));
    }
    if let Some(literal) = program.literal() {
        if program.program_token().is_some() {
            doc = doc.append(Doc::space());
        }
        doc = doc
            .append(leading_comments(literal.syntax()))
            .append(build_literal(literal));
    }
    doc
}

fn build_copy_option_list<'a>(list: ast::CopyOptionList) -> Doc<'a> {
    let mut body = build_comma_separated_docs(list.copy_options().map(|option| {
        (
            leading_comments(option.syntax()).append(build_copy_option(option.clone())),
            option.syntax().clone(),
        )
    }))
    .unwrap_or_else(Doc::nil);
    if let Some(r_paren) = list.r_paren_token() {
        body = body.append(comments_before(r_paren));
    }

    list.l_paren_token()
        .map(comments_before)
        .unwrap_or_else(Doc::nil)
        .append(Doc::text("("))
        .append(wrap_body(body))
        .append(Doc::text(")"))
        .group()
}

fn build_copy_option<'a>(option: ast::CopyOption) -> Doc<'a> {
    let mut doc = option
        .copy_option_key()
        .map(|key| build_keyword_node(key.syntax()))
        .unwrap_or_else(Doc::nil);
    if let Some(value) = option.copy_option_value() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(value.syntax()))
            .append(build_copy_option_value(value));
    }
    doc.group()
}

fn build_copy_option_value<'a>(value: ast::CopyOptionValue) -> Doc<'a> {
    if let Some(list) = value.copy_option_list() {
        build_copy_option_list(list)
    } else if let Some(name) = value.copy_option_value_name() {
        build_keyword_node(name.syntax())
    } else if let Some(expr) = value.expr() {
        build_expr(expr)
    } else if value.star_token().is_some() {
        Doc::text("*")
    } else if value.default_token().is_some() {
        Doc::text("default")
    } else if value.on_token().is_some() {
        Doc::text("on")
    } else if value.off_token().is_some() {
        Doc::text("off")
    } else {
        unreachable!("copy option value must have a value")
    }
}

fn build_copy_legacy_option<'a>(option: ast::CopyLegacyOption) -> Doc<'a> {
    let keyword = if option.binary_token().is_some() {
        "binary"
    } else if option.freeze_token().is_some() {
        "freeze"
    } else if option.csv_token().is_some() {
        "csv"
    } else if option.header_token().is_some() {
        "header"
    } else if option.json_token().is_some() {
        "json"
    } else if option.delimiter_token().is_some() {
        "delimiter"
    } else if option.null_token().is_some() {
        "null"
    } else if option.quote_token().is_some() {
        "quote"
    } else if option.escape_token().is_some() {
        "escape"
    } else if option.encoding_token().is_some() {
        "encoding"
    } else if option.force_token().is_some() {
        "force"
    } else {
        unreachable!("copy legacy option must have a keyword")
    };
    let mut doc = Doc::text(keyword);

    if let Some(as_token) = option.as_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&as_token))
            .append(Doc::text("as"));
    }
    if let Some(literal) = option.literal() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(literal.syntax()))
            .append(build_literal(literal));
    }
    if let Some(kind) = option.copy_force_kind() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(kind.syntax()))
            .append(build_keyword_node(kind.syntax()));
    }
    if let Some(star) = option.star_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&star))
            .append(Doc::text("*"));
    } else {
        let names = option.column_name_refs().map(|name| {
            (
                leading_comments(name.syntax()).append(build_name(name.syntax())),
                name.syntax().clone(),
            )
        });
        if let Some(names) = build_comma_separated_docs(names) {
            doc = doc.append(Doc::space()).append(names);
        }
    }
    doc.group()
}

fn build_stmt<'a>(stmt: ast::Stmt) -> Doc<'a> {
    match stmt {
        ast::Stmt::AlterPublication(stmt) => build_alter_publication(&stmt),
        ast::Stmt::AlterSubscription(stmt) => build_alter_subscription(&stmt),
        ast::Stmt::Begin(stmt) => build_begin(&stmt),
        ast::Stmt::Commit(stmt) => build_commit(stmt),
        ast::Stmt::CompoundSelect(stmt) => build_compound_select(&stmt),
        ast::Stmt::CreateFunction(stmt) => build_create_function(&stmt),
        ast::Stmt::CreateIndex(stmt) => build_create_index(&stmt),
        ast::Stmt::CreatePublication(stmt) => build_create_publication(&stmt),
        ast::Stmt::CreateSubscription(stmt) => build_create_subscription(&stmt),
        ast::Stmt::CreateTable(stmt) => build_create_table(&stmt),
        ast::Stmt::CreateTableAs(stmt) => build_create_table_as(&stmt),
        ast::Stmt::CreateView(stmt) => build_create_view(&stmt),
        ast::Stmt::Delete(stmt) => build_delete(&stmt),
        ast::Stmt::DropPublication(stmt) => build_drop_publication(&stmt),
        ast::Stmt::DropSubscription(stmt) => build_drop_subscription(&stmt),
        ast::Stmt::EmptyStmt(stmt) => build_empty_stmt(&stmt),
        ast::Stmt::Insert(stmt) => build_insert(&stmt),
        ast::Stmt::Merge(stmt) => build_merge(&stmt),
        ast::Stmt::ParenSelect(stmt) => build_paren_select(stmt),
        ast::Stmt::PrepareTransaction(stmt) => build_prepare_transaction(&stmt),
        ast::Stmt::ReleaseSavepoint(stmt) => build_release_savepoint(&stmt),
        ast::Stmt::Rollback(stmt) => build_rollback(stmt),
        ast::Stmt::SavepointCreate(stmt) => build_savepoint_create(&stmt),
        ast::Stmt::Select(stmt) => build_select_doc(&stmt),
        ast::Stmt::SelectInto(stmt) => build_select_into(&stmt),
        ast::Stmt::Table(stmt) => build_table(&stmt),
        ast::Stmt::Truncate(stmt) => build_truncate(&stmt),
        ast::Stmt::Update(stmt) => build_update(&stmt),
        ast::Stmt::Values(stmt) => build_values(&stmt),
        ast::Stmt::AlterAggregate(_) => todo!(),
        ast::Stmt::AlterCollation(_) => todo!(),
        ast::Stmt::AlterConversion(_) => todo!(),
        ast::Stmt::AlterDatabase(_) => todo!(),
        ast::Stmt::AlterDefaultPrivileges(_) => todo!(),
        ast::Stmt::AlterDomain(_) => todo!(),
        ast::Stmt::AlterEventTrigger(_) => todo!(),
        ast::Stmt::AlterExtension(_) => todo!(),
        ast::Stmt::AlterForeignDataWrapper(_) => todo!(),
        ast::Stmt::AlterForeignTable(_) => todo!(),
        ast::Stmt::AlterFunction(_) => todo!(),
        ast::Stmt::AlterGroup(_) => todo!(),
        ast::Stmt::AlterIndex(_) => todo!(),
        ast::Stmt::AlterLanguage(_) => todo!(),
        ast::Stmt::AlterLargeObject(_) => todo!(),
        ast::Stmt::AlterMaterializedView(_) => todo!(),
        ast::Stmt::AlterOperator(_) => todo!(),
        ast::Stmt::AlterOperatorClass(_) => todo!(),
        ast::Stmt::AlterOperatorFamily(_) => todo!(),
        ast::Stmt::AlterPolicy(_) => todo!(),
        ast::Stmt::AlterProcedure(_) => todo!(),
        ast::Stmt::AlterPropertyGraph(_) => todo!(),
        ast::Stmt::AlterRole(_) => todo!(),
        ast::Stmt::AlterRoutine(_) => todo!(),
        ast::Stmt::AlterRule(_) => todo!(),
        ast::Stmt::AlterSchema(_) => todo!(),
        ast::Stmt::AlterSequence(_) => todo!(),
        ast::Stmt::AlterServer(_) => todo!(),
        ast::Stmt::AlterStatistics(_) => todo!(),
        ast::Stmt::AlterSystem(_) => todo!(),
        ast::Stmt::AlterTable(_) => todo!(),
        ast::Stmt::AlterTablespace(_) => todo!(),
        ast::Stmt::AlterTextSearchConfiguration(_) => todo!(),
        ast::Stmt::AlterTextSearchDictionary(_) => todo!(),
        ast::Stmt::AlterTextSearchParser(_) => todo!(),
        ast::Stmt::AlterTextSearchTemplate(_) => todo!(),
        ast::Stmt::AlterTrigger(_) => todo!(),
        ast::Stmt::AlterType(_) => todo!(),
        ast::Stmt::AlterUser(_) => todo!(),
        ast::Stmt::AlterUserMapping(_) => todo!(),
        ast::Stmt::AlterView(_) => todo!(),
        ast::Stmt::Analyze(stmt) => build_analyze(&stmt),
        ast::Stmt::Call(stmt) => build_call(&stmt),
        ast::Stmt::Checkpoint(stmt) => build_checkpoint(&stmt),
        ast::Stmt::Close(stmt) => build_close(&stmt),
        ast::Stmt::Cluster(stmt) => build_cluster(&stmt),
        ast::Stmt::CommentOn(_) => todo!(),
        ast::Stmt::Copy(stmt) => build_copy(&stmt),
        ast::Stmt::CreateAccessMethod(_) => todo!(),
        ast::Stmt::CreateAggregate(_) => todo!(),
        ast::Stmt::CreateCast(_) => todo!(),
        ast::Stmt::CreateCollation(_) => todo!(),
        ast::Stmt::CreateConversion(_) => todo!(),
        ast::Stmt::CreateDatabase(_) => todo!(),
        ast::Stmt::CreateDomain(_) => todo!(),
        ast::Stmt::CreateEventTrigger(_) => todo!(),
        ast::Stmt::CreateExtension(_) => todo!(),
        ast::Stmt::CreateForeignDataWrapper(_) => todo!(),
        ast::Stmt::CreateForeignTable(stmt) => build_create_foreign_table(&stmt),
        ast::Stmt::CreateGroup(_) => todo!(),
        ast::Stmt::CreateLanguage(_) => todo!(),
        ast::Stmt::CreateMaterializedView(_) => todo!(),
        ast::Stmt::CreateOperator(_) => todo!(),
        ast::Stmt::CreateOperatorClass(_) => todo!(),
        ast::Stmt::CreateOperatorFamily(_) => todo!(),
        ast::Stmt::CreatePolicy(_) => todo!(),
        ast::Stmt::CreateProcedure(_) => todo!(),
        ast::Stmt::CreatePropertyGraph(_) => todo!(),
        ast::Stmt::CreateRole(_) => todo!(),
        ast::Stmt::CreateRule(_) => todo!(),
        ast::Stmt::CreateSchema(_) => todo!(),
        ast::Stmt::CreateSequence(_) => todo!(),
        ast::Stmt::CreateServer(_) => todo!(),
        ast::Stmt::CreateStatistics(_) => todo!(),
        ast::Stmt::CreateTablespace(_) => todo!(),
        ast::Stmt::CreateTextSearchConfiguration(_) => todo!(),
        ast::Stmt::CreateTextSearchDictionary(_) => todo!(),
        ast::Stmt::CreateTextSearchParser(_) => todo!(),
        ast::Stmt::CreateTextSearchTemplate(_) => todo!(),
        ast::Stmt::CreateTransform(_) => todo!(),
        ast::Stmt::CreateTrigger(_) => todo!(),
        ast::Stmt::CreateType(_) => todo!(),
        ast::Stmt::CreateUser(_) => todo!(),
        ast::Stmt::CreateUserMapping(_) => todo!(),
        ast::Stmt::Deallocate(stmt) => build_deallocate(&stmt),
        ast::Stmt::Declare(stmt) => build_declare(&stmt),
        ast::Stmt::Discard(stmt) => build_discard(&stmt),
        ast::Stmt::Do(stmt) => build_do(&stmt),
        ast::Stmt::DropAccessMethod(_) => todo!(),
        ast::Stmt::DropAggregate(_) => todo!(),
        ast::Stmt::DropCast(_) => todo!(),
        ast::Stmt::DropCollation(_) => todo!(),
        ast::Stmt::DropConversion(_) => todo!(),
        ast::Stmt::DropDatabase(_) => todo!(),
        ast::Stmt::DropDomain(_) => todo!(),
        ast::Stmt::DropEventTrigger(_) => todo!(),
        ast::Stmt::DropExtension(_) => todo!(),
        ast::Stmt::DropForeignDataWrapper(_) => todo!(),
        ast::Stmt::DropForeignTable(_) => todo!(),
        ast::Stmt::DropFunction(_) => todo!(),
        ast::Stmt::DropGroup(_) => todo!(),
        ast::Stmt::DropIndex(_) => todo!(),
        ast::Stmt::DropLanguage(_) => todo!(),
        ast::Stmt::DropMaterializedView(_) => todo!(),
        ast::Stmt::DropOperator(_) => todo!(),
        ast::Stmt::DropOperatorClass(_) => todo!(),
        ast::Stmt::DropOperatorFamily(_) => todo!(),
        ast::Stmt::DropOwned(_) => todo!(),
        ast::Stmt::DropPolicy(_) => todo!(),
        ast::Stmt::DropProcedure(_) => todo!(),
        ast::Stmt::DropPropertyGraph(_) => todo!(),
        ast::Stmt::DropRole(_) => todo!(),
        ast::Stmt::DropRoutine(_) => todo!(),
        ast::Stmt::DropRule(_) => todo!(),
        ast::Stmt::DropSchema(_) => todo!(),
        ast::Stmt::DropSequence(_) => todo!(),
        ast::Stmt::DropServer(_) => todo!(),
        ast::Stmt::DropStatistics(_) => todo!(),
        ast::Stmt::DropTable(_) => todo!(),
        ast::Stmt::DropTablespace(_) => todo!(),
        ast::Stmt::DropTextSearchConfig(_) => todo!(),
        ast::Stmt::DropTextSearchDict(_) => todo!(),
        ast::Stmt::DropTextSearchParser(_) => todo!(),
        ast::Stmt::DropTextSearchTemplate(_) => todo!(),
        ast::Stmt::DropTransform(_) => todo!(),
        ast::Stmt::DropTrigger(_) => todo!(),
        ast::Stmt::DropType(_) => todo!(),
        ast::Stmt::DropUser(_) => todo!(),
        ast::Stmt::DropUserMapping(_) => todo!(),
        ast::Stmt::DropView(_) => todo!(),
        ast::Stmt::Execute(stmt) => build_execute(stmt),
        ast::Stmt::Explain(stmt) => build_explain(&stmt),
        ast::Stmt::Fetch(stmt) => build_fetch(&stmt),
        ast::Stmt::Grant(stmt) => build_grant(&stmt),
        ast::Stmt::ImportForeignSchema(stmt) => build_import_foreign_schema(&stmt),
        ast::Stmt::Listen(stmt) => build_listen(&stmt),
        ast::Stmt::Load(stmt) => build_load(&stmt),
        ast::Stmt::Lock(stmt) => build_lock(&stmt),
        ast::Stmt::Move(stmt) => build_move(&stmt),
        ast::Stmt::Notify(stmt) => build_notify(&stmt),
        ast::Stmt::Prepare(stmt) => build_prepare(&stmt),
        ast::Stmt::Reassign(stmt) => build_reassign(&stmt),
        ast::Stmt::Refresh(stmt) => build_refresh(&stmt),
        ast::Stmt::Reindex(stmt) => build_reindex(&stmt),
        ast::Stmt::Repack(stmt) => build_repack(&stmt),
        ast::Stmt::Reset(stmt) => build_reset(&stmt),
        ast::Stmt::ResetRole(stmt) => build_reset_role(&stmt),
        ast::Stmt::ResetSessionAuth(stmt) => build_reset_session_auth(&stmt),
        ast::Stmt::Revoke(stmt) => build_revoke(&stmt),
        ast::Stmt::SecurityLabel(stmt) => build_security_label(&stmt),
        ast::Stmt::Set(stmt) => build_set(&stmt),
        ast::Stmt::SetConstraints(stmt) => build_set_constraints(&stmt),
        ast::Stmt::SetRole(stmt) => build_set_role(&stmt),
        ast::Stmt::SetSessionAuth(stmt) => build_set_session_auth(&stmt),
        ast::Stmt::SetTransaction(stmt) => build_set_transaction(&stmt),
        ast::Stmt::Show(stmt) => build_show(&stmt),
        ast::Stmt::Unlisten(stmt) => build_unlisten(&stmt),
        ast::Stmt::Vacuum(stmt) => build_vacuum(&stmt),
    }
}

fn build_return_function_option<'a>(option: ast::ReturnFuncOption) -> Doc<'a> {
    let mut doc = Doc::text("return");
    if let Some(expr) = option.expr() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(expr.syntax()))
            .append(build_expr(expr));
    }
    doc.append(build_semicolon(option.semicolon_token()))
}

fn build_literal_function_option<'a>(
    keyword: &'static str,
    literal: Option<ast::Literal>,
) -> Doc<'a> {
    let mut doc = Doc::text(keyword);
    if let Some(literal) = literal {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(literal.syntax()))
            .append(build_literal(literal));
    }
    doc
}

fn build_as_function_option<'a>(option: ast::AsFuncOption) -> Doc<'a> {
    let mut doc = Doc::text("as");
    if let Some(target) = option.as_func_target() {
        match target {
            ast::AsFuncTarget::AsDefinition(definition) => {
                if let Some(literal) = definition.literal() {
                    doc = doc
                        .append(Doc::space())
                        .append(leading_comments(definition.syntax()))
                        .append(leading_comments(literal.syntax()))
                        .append(build_literal(literal));
                }
            }
            ast::AsFuncTarget::AsObjFile(obj_file) => {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments(obj_file.syntax()));
                if let Some(literal) = obj_file.obj_file() {
                    doc = doc
                        .append(leading_comments(literal.syntax()))
                        .append(build_literal(literal));
                }
                if let Some(comma) = obj_file.comma_token() {
                    doc = doc.append(comments_before(comma)).append(Doc::text(","));
                }
                if let Some(literal) = obj_file.link_symbol() {
                    doc = doc
                        .append(Doc::line_or_space())
                        .append(leading_comments(literal.syntax()))
                        .append(build_literal(literal));
                }
                doc = doc.group();
            }
        }
    }
    doc
}

fn build_explain<'a>(explain: &ast::Explain) -> Doc<'a> {
    let mut doc = Doc::text("explain");
    if let Some(mode) = explain.explain_mode() {
        let mode_syntax = mode.syntax().clone();
        let parenthesized = matches!(&mode, ast::ExplainMode::ExplainOptionList(_));
        let mode_doc = match mode {
            ast::ExplainMode::ExplainAnalyze(analyze) => {
                let mut doc = if analyze.analyse_token().is_some() {
                    Doc::text("analyse")
                } else {
                    Doc::text("analyze")
                };
                if let Some(verbose) = analyze.explain_verbose() {
                    doc = doc
                        .append(Doc::space())
                        .append(leading_comments(verbose.syntax()))
                        .append(build_keyword_node(verbose.syntax()));
                }
                doc
            }
            ast::ExplainMode::ExplainVerbose(verbose) => build_keyword_node(verbose.syntax()),
            ast::ExplainMode::ExplainOptionList(options) => build_explain_option_list(options),
        };
        let mode_doc = leading_comments(&mode_syntax).append(mode_doc);
        doc = if parenthesized {
            doc.append(Doc::space()).append(mode_doc)
        } else {
            doc.append(Doc::line_or_space().append(mode_doc).nest(2))
        };
    }
    if let Some(stmt) = explain.explain_stmt() {
        doc = doc.append(
            Doc::hard_line()
                .append(leading_comments(stmt.syntax()))
                .append(build_explain_stmt(stmt))
                .nest(2),
        );
    }
    doc.group()
        .append(build_semicolon(explain.semicolon_token()))
}

fn build_explain_option_list<'a>(list: ast::ExplainOptionList) -> Doc<'a> {
    let mut body = build_comma_separated_docs(list.explain_options().map(|option| {
        let syntax = option.syntax().clone();
        let mut doc = option
            .explain_option_name()
            .map(|name| build_keyword_node(name.syntax()))
            .unwrap_or_else(Doc::nil);
        if let Some(value) = option.explain_option_value() {
            let value_doc = value
                .expr()
                .map(build_expr)
                .unwrap_or_else(|| build_keyword_node(value.syntax()));
            doc = doc
                .append(Doc::space())
                .append(leading_comments(value.syntax()))
                .append(value_doc);
        }
        (leading_comments(&syntax).append(doc), syntax)
    }))
    .unwrap_or_else(Doc::nil);
    if let Some(r_paren) = list.r_paren_token() {
        body = body.append(comments_before(r_paren));
    }
    list.l_paren_token()
        .map(comments_before)
        .unwrap_or_else(Doc::nil)
        .append(Doc::text("("))
        .append(Doc::hard_line().append(body.group()).nest(2))
        .append(Doc::hard_line())
        .append(Doc::text(")"))
}

fn build_create_materialized_view<'a>(view: &ast::CreateMaterializedView) -> Doc<'a> {
    let mut doc = Doc::text("create");
    for (token, keyword) in [
        (view.materialized_token(), "materialized"),
        (view.view_token(), "view"),
    ] {
        if let Some(token) = token {
            doc = doc
                .append(Doc::space())
                .append(leading_comments_token(&token))
                .append(Doc::text(keyword));
        }
    }
    if let Some(if_not_exists) = view.if_not_exists() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(if_not_exists.syntax()))
            .append(build_keyword_node(if_not_exists.syntax()));
    }
    if let Some(name) = view.view() {
        if let Some(path) = name.path() {
            doc = doc
                .append(Doc::space())
                .append(leading_comments(name.syntax()))
                .append(build_path(&path));
        }
    }
    if let Some(columns) = view.column_list() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(columns.syntax()))
            .append(build_cte_column_list(columns));
    }
    if let Some(using) = view.using_method() {
        let mut option = Doc::text("using");
        if let Some(method) = using.access_method_ref() {
            option = option
                .append(Doc::space())
                .append(leading_comments(method.syntax()))
                .append(build_name(method.syntax()));
        }
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(using.syntax()))
                .append(option)
                .nest(2),
        );
    }
    if let Some(params) = view.with_params() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(params.syntax()))
                .append(build_with_params(params))
                .nest(2),
        );
    }
    if let Some(tablespace) = view.tablespace_clause() {
        let mut option = Doc::text("tablespace");
        if let Some(name) = tablespace.tablespace_ref() {
            option = option
                .append(Doc::space())
                .append(leading_comments(name.syntax()))
                .append(build_name(name.syntax()));
        }
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(tablespace.syntax()))
                .append(option)
                .nest(2),
        );
    }
    if let Some(as_token) = view.as_token() {
        doc = doc
            .append(Doc::hard_line())
            .append(leading_comments_token(&as_token))
            .append(Doc::text("as"));
    }
    if let Some(query) = view.query() {
        doc = doc.append(
            Doc::hard_line()
                .append(leading_comments(query.syntax()))
                .append(build_select_variant(query))
                .nest(2),
        );
    }
    if let Some(data) = view.data_option() {
        doc = doc.append(
            Doc::hard_line()
                .append(leading_comments(data.syntax()))
                .append(build_keyword_node(data.syntax()))
                .nest(2),
        );
    }
    doc.group().append(build_semicolon(view.semicolon_token()))
}

fn build_explain_stmt<'a>(stmt: ast::ExplainStmt) -> Doc<'a> {
    match stmt {
        ast::ExplainStmt::CompoundSelect(stmt) => build_compound_select(&stmt),
        ast::ExplainStmt::CreateTableAs(stmt) => build_create_table_as(&stmt),
        ast::ExplainStmt::Declare(stmt) => build_declare(&stmt),
        ast::ExplainStmt::Delete(stmt) => build_delete(&stmt),
        ast::ExplainStmt::Execute(stmt) => build_execute(stmt),
        ast::ExplainStmt::Insert(stmt) => build_insert(&stmt),
        ast::ExplainStmt::Merge(stmt) => build_merge(&stmt),
        ast::ExplainStmt::ParenSelect(stmt) => build_paren_select(stmt),
        ast::ExplainStmt::Select(stmt) => build_select_doc(&stmt),
        ast::ExplainStmt::SelectInto(stmt) => build_select_into(&stmt),
        ast::ExplainStmt::Table(stmt) => build_table(&stmt),
        ast::ExplainStmt::Update(stmt) => build_update(&stmt),
        ast::ExplainStmt::Values(stmt) => build_values(&stmt),
        ast::ExplainStmt::CreateMaterializedView(stmt) => build_create_materialized_view(&stmt),
    }
}

fn build_role_ref_list<'a>(list: ast::RoleRefList) -> Doc<'a> {
    build_comma_separated_docs(list.role_refs().map(|role| {
        let syntax = role.syntax().clone();
        (
            leading_comments(&syntax).append(build_role_ref(&role)),
            syntax,
        )
    }))
    .unwrap_or_else(Doc::nil)
}

fn build_keyword_tokens<'a, const N: usize>(
    tokens: [(Option<SyntaxToken>, &'static str); N],
) -> Doc<'a> {
    let mut doc = Doc::nil();
    let mut has_keyword = false;
    for (token, keyword) in tokens {
        if let Some(token) = token {
            if has_keyword {
                doc = doc.append(Doc::space());
            }
            has_keyword = true;
            doc = doc
                .append(leading_comments_token(&token))
                .append(Doc::text(keyword));
        }
    }
    doc
}

fn build_revoke_command<'a>(command: ast::RevokeCommand) -> Doc<'a> {
    let mut doc = if let Some(role) = command.role_ref() {
        build_role_ref(&role)
    } else if command.ident_token().is_some() {
        build_name(command.syntax())
    } else {
        let tokens = [
            (command.all_token(), "all"),
            (command.alter_token(), "alter"),
            (command.create_token(), "create"),
            (command.delete_token(), "delete"),
            (command.execute_token(), "execute"),
            (command.insert_token(), "insert"),
            (command.references_token(), "references"),
            (command.select_token(), "select"),
            (command.system_token(), "system"),
            (command.temp_token(), "temp"),
            (command.temporary_token(), "temporary"),
            (command.trigger_token(), "trigger"),
            (command.truncate_token(), "truncate"),
            (command.update_token(), "update"),
        ];
        build_keyword_tokens(tokens)
    };
    if let Some(columns) = command.column_ref_list() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(columns.syntax()))
            .append(build_column_ref_list(columns));
    }
    doc.group()
}

fn build_privileges<'a>(privileges: ast::Privileges) -> Doc<'a> {
    match privileges {
        ast::Privileges::AllPrivileges(all) => {
            let mut doc = build_keyword_tokens([
                (all.all_token(), "all"),
                (all.privileges_token(), "privileges"),
            ]);
            if let Some(columns) = all.column_ref_list() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments(columns.syntax()))
                    .append(build_column_ref_list(columns));
            }
            doc
        }
        ast::Privileges::RevokeCommandList(commands) => {
            build_comma_separated_docs(commands.revoke_commands().map(|command| {
                let syntax = command.syntax().clone();
                (
                    leading_comments(&syntax).append(build_revoke_command(command)),
                    syntax,
                )
            }))
            .unwrap_or_else(Doc::nil)
        }
    }
}

fn build_path_items<'a>(items: Vec<(SyntaxNode, ast::PathRef)>) -> Doc<'a> {
    build_comma_separated_docs(items.into_iter().map(|(syntax, path)| {
        (
            leading_comments(&syntax).append(build_path_ref(&path)),
            syntax,
        )
    }))
    .unwrap_or_else(Doc::nil)
}

fn build_name_items<'a>(items: Vec<SyntaxNode>) -> Doc<'a> {
    build_comma_separated_docs(items.into_iter().map(|syntax| {
        let doc = leading_comments(&syntax).append(build_name(&syntax));
        (doc, syntax)
    }))
    .unwrap_or_else(Doc::nil)
}

fn append_privilege_items<'a>(prefix: Doc<'a>, items: Doc<'a>) -> Doc<'a> {
    prefix
        .append(Doc::line_or_space().append(items).nest(2))
        .group()
}

fn build_function_sig<'a>(sig: ast::FunctionSig) -> Doc<'a> {
    let mut doc = sig
        .function_name_ref()
        .and_then(|name| name.path_ref())
        .map(|path| build_path_ref(&path))
        .unwrap_or_else(Doc::nil);
    if let Some(params) = sig.param_list() {
        doc = doc
            .append(leading_comments(params.syntax()))
            .append(build_function_param_list(params));
    }
    doc
}

fn build_procedure_sig<'a>(sig: ast::ProcedureSig) -> Doc<'a> {
    let mut doc = sig
        .procedure_name_ref()
        .and_then(|name| name.path_ref())
        .map(|path| build_path_ref(&path))
        .unwrap_or_else(Doc::nil);
    if let Some(params) = sig.param_list() {
        doc = doc
            .append(leading_comments(params.syntax()))
            .append(build_function_param_list(params));
    }
    doc
}

fn build_routine_sig<'a>(sig: ast::RoutineSig) -> Doc<'a> {
    let mut doc = sig
        .routine_name_ref()
        .and_then(|name| name.path_ref())
        .map(|path| build_path_ref(&path))
        .unwrap_or_else(Doc::nil);
    if let Some(params) = sig.param_list() {
        doc = doc
            .append(leading_comments(params.syntax()))
            .append(build_function_param_list(params));
    }
    doc
}

fn build_privilege_objects<'a>(objects: ast::PrivilegeObjects) -> Doc<'a> {
    macro_rules! direct_items {
        ($node:ident, $method:ident, $tokens:expr) => {{
            let items =
                build_name_items($node.$method().map(|item| item.syntax().clone()).collect());
            append_privilege_items(build_keyword_tokens($tokens), items)
        }};
    }
    match objects {
        ast::PrivilegeObjects::PrivilegeAllFunctionsInSchema(node) => direct_items!(
            node,
            schema_refs,
            [
                (node.all_token(), "all"),
                (node.functions_token(), "functions"),
                (node.in_token(), "in"),
                (node.schema_token(), "schema")
            ]
        ),
        ast::PrivilegeObjects::PrivilegeAllProceduresInSchema(node) => direct_items!(
            node,
            schema_refs,
            [
                (node.all_token(), "all"),
                (node.procedures_token(), "procedures"),
                (node.in_token(), "in"),
                (node.schema_token(), "schema")
            ]
        ),
        ast::PrivilegeObjects::PrivilegeAllRoutinesInSchema(node) => direct_items!(
            node,
            schema_refs,
            [
                (node.all_token(), "all"),
                (node.routines_token(), "routines"),
                (node.in_token(), "in"),
                (node.schema_token(), "schema")
            ]
        ),
        ast::PrivilegeObjects::PrivilegeAllSequencesInSchema(node) => direct_items!(
            node,
            schema_refs,
            [
                (node.all_token(), "all"),
                (node.sequences_token(), "sequences"),
                (node.in_token(), "in"),
                (node.schema_token(), "schema")
            ]
        ),
        ast::PrivilegeObjects::PrivilegeAllTablesInSchema(node) => direct_items!(
            node,
            schema_refs,
            [
                (node.all_token(), "all"),
                (node.tables_token(), "tables"),
                (node.in_token(), "in"),
                (node.schema_token(), "schema")
            ]
        ),
        ast::PrivilegeObjects::PrivilegeDatabase(node) => {
            direct_items!(node, database_refs, [(node.database_token(), "database")])
        }
        ast::PrivilegeObjects::PrivilegeDefault(node) => build_path_items(
            node.relation_name_refs()
                .filter_map(|item| {
                    let syntax = item.syntax().clone();
                    item.path_ref().map(|path| (syntax, path))
                })
                .collect(),
        ),
        ast::PrivilegeObjects::PrivilegeDomain(node) => {
            let items = build_path_items(
                node.domain_refs()
                    .filter_map(|item| {
                        let syntax = item.syntax().clone();
                        item.path_ref().map(|path| (syntax, path))
                    })
                    .collect(),
            );
            append_privilege_items(
                build_keyword_tokens([(node.domain_token(), "domain")]),
                items,
            )
        }
        ast::PrivilegeObjects::PrivilegeForeignDataWrapper(node) => direct_items!(
            node,
            foreign_data_wrapper_refs,
            [
                (node.foreign_token(), "foreign"),
                (node.data_token(), "data"),
                (node.wrapper_token(), "wrapper")
            ]
        ),
        ast::PrivilegeObjects::PrivilegeForeignServer(node) => direct_items!(
            node,
            server_refs,
            [
                (node.foreign_token(), "foreign"),
                (node.server_token(), "server")
            ]
        ),
        ast::PrivilegeObjects::PrivilegeLanguage(node) => {
            direct_items!(node, language_refs, [(node.language_token(), "language")])
        }
        ast::PrivilegeObjects::PrivilegeParameter(node) => {
            let items =
                build_comma_separated_docs(node.config_parameter_refs().filter_map(|item| {
                    let syntax = item.syntax().clone();
                    item.path_ref().map(|path| {
                        (
                            leading_comments(&syntax).append(build_path_ref(&path)),
                            syntax,
                        )
                    })
                }))
                .unwrap_or_else(Doc::nil);
            append_privilege_items(
                build_keyword_tokens([(node.parameter_token(), "parameter")]),
                items,
            )
        }
        ast::PrivilegeObjects::PrivilegePropertyGraph(node) => {
            let items = build_path_items(
                node.property_graph_refs()
                    .filter_map(|item| {
                        let syntax = item.syntax().clone();
                        item.path_ref().map(|path| (syntax, path))
                    })
                    .collect(),
            );
            append_privilege_items(
                build_keyword_tokens([
                    (node.property_token(), "property"),
                    (node.graph_token(), "graph"),
                ]),
                items,
            )
        }
        ast::PrivilegeObjects::PrivilegeSchema(node) => {
            direct_items!(node, schema_refs, [(node.schema_token(), "schema")])
        }
        ast::PrivilegeObjects::PrivilegeSequence(node) => {
            let items = build_path_items(
                node.sequence_refs()
                    .filter_map(|item| {
                        let syntax = item.syntax().clone();
                        item.path_ref().map(|path| (syntax, path))
                    })
                    .collect(),
            );
            append_privilege_items(
                build_keyword_tokens([(node.sequence_token(), "sequence")]),
                items,
            )
        }
        ast::PrivilegeObjects::PrivilegeTable(node) => {
            let items = build_path_items(
                node.relation_name_refs()
                    .filter_map(|item| {
                        let syntax = item.syntax().clone();
                        item.path_ref().map(|path| (syntax, path))
                    })
                    .collect(),
            );
            append_privilege_items(build_keyword_tokens([(node.table_token(), "table")]), items)
        }
        ast::PrivilegeObjects::PrivilegeTablespace(node) => direct_items!(
            node,
            tablespace_refs,
            [(node.tablespace_token(), "tablespace")]
        ),
        ast::PrivilegeObjects::PrivilegeType(node) => {
            let items = build_path_items(
                node.type_name_refs()
                    .filter_map(|item| {
                        let syntax = item.syntax().clone();
                        item.path_ref().map(|path| (syntax, path))
                    })
                    .collect(),
            );
            append_privilege_items(build_keyword_tokens([(node.type_token(), "type")]), items)
        }
        ast::PrivilegeObjects::PrivilegeLargeObject(node) => {
            let items = build_comma_separated_docs(node.literals().map(|item| {
                let syntax = item.syntax().clone();
                (
                    leading_comments(&syntax).append(build_literal(item)),
                    syntax,
                )
            }))
            .unwrap_or_else(Doc::nil);
            append_privilege_items(
                build_keyword_tokens([
                    (node.large_token(), "large"),
                    (node.object_token(), "object"),
                ]),
                items,
            )
        }
        ast::PrivilegeObjects::PrivilegeFunction(node) => {
            let items = node
                .function_sig_list()
                .and_then(|list| {
                    build_comma_separated_docs(list.function_sigs().map(|sig| {
                        let syntax = sig.syntax().clone();
                        (
                            leading_comments(&syntax).append(build_function_sig(sig)),
                            syntax,
                        )
                    }))
                })
                .unwrap_or_else(Doc::nil);
            append_privilege_items(
                build_keyword_tokens([(node.function_token(), "function")]),
                items,
            )
        }
        ast::PrivilegeObjects::PrivilegeProcedure(node) => {
            let items = node
                .procedure_sig_list()
                .and_then(|list| {
                    build_comma_separated_docs(list.procedure_sigs().map(|sig| {
                        let syntax = sig.syntax().clone();
                        (
                            leading_comments(&syntax).append(build_procedure_sig(sig)),
                            syntax,
                        )
                    }))
                })
                .unwrap_or_else(Doc::nil);
            append_privilege_items(
                build_keyword_tokens([(node.procedure_token(), "procedure")]),
                items,
            )
        }
        ast::PrivilegeObjects::PrivilegeRoutine(node) => {
            let items = node
                .routine_sig_list()
                .and_then(|list| {
                    build_comma_separated_docs(list.routine_sigs().map(|sig| {
                        let syntax = sig.syntax().clone();
                        (
                            leading_comments(&syntax).append(build_routine_sig(sig)),
                            syntax,
                        )
                    }))
                })
                .unwrap_or_else(Doc::nil);
            append_privilege_items(
                build_keyword_tokens([(node.routine_token(), "routine")]),
                items,
            )
        }
    }
}

fn build_grant_with_clause<'a>(with: ast::GrantWithClause) -> Doc<'a> {
    let mut doc = Doc::text("with");
    if let Some(option) = with.grant_option() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(option.syntax()))
            .append(build_keyword_node(option.syntax()));
    } else if let Some(options) = with.grant_role_option_list() {
        let options_doc = build_comma_separated_docs(options.grant_role_options().map(|option| {
            let syntax = option.syntax().clone();
            let mut option_doc = option
                .grant_role_option_name()
                .map(|name| build_keyword_node(name.syntax()))
                .unwrap_or_else(Doc::nil);
            let value = option
                .option_token()
                .map(|token| (token, "option"))
                .or_else(|| option.true_token().map(|token| (token, "true")))
                .or_else(|| option.false_token().map(|token| (token, "false")));
            if let Some((token, keyword)) = value {
                option_doc = option_doc
                    .append(Doc::space())
                    .append(leading_comments_token(&token))
                    .append(Doc::text(keyword));
            }
            (leading_comments(&syntax).append(option_doc), syntax)
        }))
        .unwrap_or_else(Doc::nil);
        doc = doc.append(Doc::line_or_space().append(options_doc).nest(2));
    }
    doc.group()
}

fn build_granted_by_clause<'a>(granted: ast::GrantedByClause) -> Doc<'a> {
    let mut doc = Doc::text("granted");
    if let Some(by) = granted.by_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&by))
            .append(Doc::text("by"));
    }
    if let Some(role) = granted.role_ref() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(role.syntax()))
            .append(build_role_ref(&role));
    }
    doc.group()
}

fn build_grant<'a>(grant: &ast::Grant) -> Doc<'a> {
    let mut doc = Doc::text("grant");
    if let Some(privileges) = grant.privileges() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(privileges.syntax()))
                .append(build_privileges(privileges))
                .nest(2),
        );
    }
    if let Some(on) = grant.on_privilege_objects_clause() {
        let mut on_doc = Doc::text("on");
        if let Some(objects) = on.privilege_objects() {
            on_doc = on_doc.append(
                Doc::line_or_space()
                    .append(leading_comments(objects.syntax()))
                    .append(build_privilege_objects(objects))
                    .nest(2),
            );
        }
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(on.syntax()))
                .append(on_doc)
                .nest(2),
        );
    }
    if let Some(to) = grant.to_token() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments_token(&to))
                .append(Doc::text("to"))
                .nest(2),
        );
    }
    if let Some(roles) = grant.role_ref_list() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(roles.syntax()))
                .append(build_role_ref_list(roles))
                .nest(2),
        );
    }
    if let Some(with) = grant.grant_with_clause() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(with.syntax()))
                .append(build_grant_with_clause(with))
                .nest(2),
        );
    }
    if let Some(granted) = grant.granted_by_clause() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(granted.syntax()))
                .append(build_granted_by_clause(granted))
                .nest(2),
        );
    }
    doc.group().append(build_semicolon(grant.semicolon_token()))
}

fn build_revoke_option_for<'a>(option: ast::RevokeOptionFor) -> Doc<'a> {
    match option {
        ast::RevokeOptionFor::AdminOptionFor(option) => build_keyword_node(option.syntax()),
        ast::RevokeOptionFor::GrantOptionFor(option) => build_keyword_node(option.syntax()),
        ast::RevokeOptionFor::InheritOptionFor(option) => build_keyword_node(option.syntax()),
        ast::RevokeOptionFor::SetOptionFor(option) => build_keyword_node(option.syntax()),
    }
}

fn build_revoke<'a>(revoke: &ast::Revoke) -> Doc<'a> {
    let mut doc = Doc::text("revoke");
    if let Some(option) = revoke.revoke_option_for() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(option.syntax()))
                .append(build_revoke_option_for(option))
                .nest(2),
        );
    }
    if let Some(privileges) = revoke.privileges() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(privileges.syntax()))
                .append(build_privileges(privileges))
                .nest(2),
        );
    }
    if let Some(on) = revoke.on_privilege_objects_clause() {
        let mut on_doc = Doc::text("on");
        if let Some(objects) = on.privilege_objects() {
            on_doc = on_doc.append(
                Doc::line_or_space()
                    .append(leading_comments(objects.syntax()))
                    .append(build_privilege_objects(objects))
                    .nest(2),
            );
        }
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(on.syntax()))
                .append(on_doc)
                .nest(2),
        );
    }
    if let Some(from) = revoke.from_token() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments_token(&from))
                .append(Doc::text("from"))
                .nest(2),
        );
    }
    if let Some(roles) = revoke.role_ref_list() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(roles.syntax()))
                .append(build_role_ref_list(roles))
                .nest(2),
        );
    }
    if let Some(granted) = revoke.granted_by_clause() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(granted.syntax()))
                .append(build_granted_by_clause(granted))
                .nest(2),
        );
    }
    if let Some(behavior) = revoke.drop_behavior() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(behavior.syntax()))
                .append(build_drop_behavior(behavior))
                .nest(2),
        );
    }
    doc.group()
        .append(build_semicolon(revoke.semicolon_token()))
}

fn build_import_table_filter<'a>(filter: ast::ImportTableFilter) -> Doc<'a> {
    let (mut doc, names, l_paren, r_paren) = match filter {
        ast::ImportTableFilter::ExceptTables(filter) => (
            Doc::text("except"),
            filter
                .remote_table_name_refs()
                .map(|name| (name.syntax().clone(), build_name(name.syntax())))
                .collect::<Vec<_>>(),
            filter.l_paren_token(),
            filter.r_paren_token(),
        ),
        ast::ImportTableFilter::LimitToTables(filter) => {
            let mut doc = Doc::text("limit");
            if let Some(to) = filter.to_token() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments_token(&to))
                    .append(Doc::text("to"));
            }
            (
                doc,
                filter
                    .remote_table_name_refs()
                    .map(|name| (name.syntax().clone(), build_name(name.syntax())))
                    .collect::<Vec<_>>(),
                filter.l_paren_token(),
                filter.r_paren_token(),
            )
        }
    };
    let mut body = build_comma_separated_docs(
        names
            .into_iter()
            .map(|(syntax, name)| (leading_comments(&syntax).append(name), syntax)),
    )
    .unwrap_or_else(Doc::nil);
    if let Some(r_paren) = r_paren {
        body = body.append(comments_before(r_paren));
    }
    doc = doc
        .append(Doc::space())
        .append(l_paren.map(comments_before).unwrap_or_else(Doc::nil))
        .append(Doc::text("("))
        .append(wrap_body(body))
        .append(Doc::text(")"));
    doc.group()
}

fn build_import_foreign_schema<'a>(import: &ast::ImportForeignSchema) -> Doc<'a> {
    let mut doc = Doc::text("import");
    for (token, keyword) in [
        (import.foreign_token(), "foreign"),
        (import.schema_token(), "schema"),
    ] {
        if let Some(token) = token {
            doc = doc.append(
                Doc::line_or_space()
                    .append(leading_comments_token(&token))
                    .append(Doc::text(keyword))
                    .nest(2),
            );
        }
    }
    if let Some(schema) = import.schema_ref() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(schema.syntax()))
                .append(build_name(schema.syntax()))
                .nest(2),
        );
    }
    if let Some(filter) = import.import_table_filter() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(filter.syntax()))
                .append(build_import_table_filter(filter))
                .nest(2),
        );
    }
    if let Some(from) = import.from_token() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments_token(&from))
                .append(Doc::text("from"))
                .nest(2),
        );
    }
    if let Some(server) = import.server_clause() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(server.syntax()))
                .append(build_server_clause(server))
                .nest(2),
        );
    }
    if let Some(into) = import.into_schema() {
        let mut into_doc = Doc::text("into");
        if let Some(schema) = into.schema_ref() {
            into_doc = into_doc
                .append(Doc::space())
                .append(leading_comments(schema.syntax()))
                .append(build_name(schema.syntax()));
        }
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(into.syntax()))
                .append(into_doc)
                .nest(2),
        );
    }
    if let Some(options) = import.alter_option_list() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(options.syntax()))
                .append(build_alter_option_list(options))
                .nest(2),
        );
    }
    doc.group()
        .append(build_semicolon(import.semicolon_token()))
}

fn build_listen<'a>(listen: &ast::Listen) -> Doc<'a> {
    let mut doc = Doc::text("listen");
    if let Some(channel) = listen.channel() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(channel.syntax()))
                .append(build_name(channel.syntax()))
                .nest(2),
        );
    }
    doc.group()
        .append(build_semicolon(listen.semicolon_token()))
}

fn build_move<'a>(move_stmt: &ast::Move) -> Doc<'a> {
    let mut doc = Doc::text("move");
    if let Some(action) = move_stmt.cursor_action() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(action.syntax()))
                .append(build_cursor_action(action))
                .nest(2),
        );
    }
    if let Some(token) = move_stmt.from_token().or_else(|| move_stmt.in_token()) {
        let keyword = if token.kind() == SyntaxKind::FROM_KW {
            "from"
        } else {
            "in"
        };
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments_token(&token))
                .append(Doc::text(keyword))
                .nest(2),
        );
    }
    if let Some(cursor) = move_stmt.cursor_ref() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(cursor.syntax()))
                .append(build_name(cursor.syntax()))
                .nest(2),
        );
    }
    doc.group()
        .append(build_semicolon(move_stmt.semicolon_token()))
}

fn build_notify<'a>(notify: &ast::Notify) -> Doc<'a> {
    let mut doc = Doc::text("notify");
    if let Some(channel) = notify.channel_ref() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(channel.syntax()))
                .append(build_name(channel.syntax()))
                .nest(2),
        );
    }
    if let Some(comma) = notify.comma_token() {
        doc = doc.append(comments_before(comma)).append(Doc::text(","));
    }
    if let Some(payload) = notify.literal() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(payload.syntax()))
                .append(build_literal(payload))
                .nest(2),
        );
    }
    doc.group()
        .append(build_semicolon(notify.semicolon_token()))
}

fn build_reassign<'a>(reassign: &ast::Reassign) -> Doc<'a> {
    let mut doc = Doc::text("reassign");
    for (token, keyword) in [
        (reassign.owned_token(), "owned"),
        (reassign.by_token(), "by"),
    ] {
        if let Some(token) = token {
            doc = doc.append(
                Doc::line_or_space()
                    .append(leading_comments_token(&token))
                    .append(Doc::text(keyword))
                    .nest(2),
            );
        }
    }
    if let Some(roles) = reassign.before() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(roles.syntax()))
                .append(build_role_ref_list(roles))
                .nest(2),
        );
    }
    if let Some(to) = reassign.to_token() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments_token(&to))
                .append(Doc::text("to"))
                .nest(2),
        );
    }
    if let Some(roles) = reassign.after() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(roles.syntax()))
                .append(build_role_ref_list(roles))
                .nest(2),
        );
    }
    doc.group()
        .append(build_semicolon(reassign.semicolon_token()))
}

fn build_refresh<'a>(refresh: &ast::Refresh) -> Doc<'a> {
    let mut doc = Doc::text("refresh");
    for (token, keyword) in [
        (refresh.materialized_token(), "materialized"),
        (refresh.view_token(), "view"),
        (refresh.concurrently_token(), "concurrently"),
    ] {
        if let Some(token) = token {
            doc = doc.append(
                Doc::line_or_space()
                    .append(leading_comments_token(&token))
                    .append(Doc::text(keyword))
                    .nest(2),
            );
        }
    }
    if let Some(view) = refresh.view_ref() {
        let view_doc = view
            .path_ref()
            .map(|path| build_path_ref(&path))
            .unwrap_or_else(Doc::nil);
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(view.syntax()))
                .append(view_doc)
                .nest(2),
        );
    }
    if let Some(data) = refresh.data_option() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(data.syntax()))
                .append(build_keyword_node(data.syntax()))
                .nest(2),
        );
    }
    doc.group()
        .append(build_semicolon(refresh.semicolon_token()))
}

fn build_repack<'a>(repack: &ast::Repack) -> Doc<'a> {
    let mut doc = Doc::text("repack");
    if let Some(options) = repack.option_item_list() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(options.syntax()))
                .append(build_option_item_list(options))
                .nest(2),
        );
    }
    if let Some(tables) = repack.table_and_columns_list() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(tables.syntax()))
                .append(build_table_and_columns_list(tables))
                .nest(2),
        );
    }
    if let Some(using_index) = repack.using_index() {
        let mut using_doc = Doc::text("using");
        if let Some(index) = using_index.index_token() {
            using_doc = using_doc
                .append(Doc::space())
                .append(leading_comments_token(&index))
                .append(Doc::text("index"));
        }
        if let Some(index) = using_index.index_ref() {
            let name = index
                .path_ref()
                .map(|path| build_path_ref(&path))
                .unwrap_or_else(Doc::nil);
            using_doc = using_doc
                .append(Doc::space())
                .append(leading_comments(index.syntax()))
                .append(name);
        }
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(using_index.syntax()))
                .append(using_doc)
                .nest(2),
        );
    }
    doc.group()
        .append(build_semicolon(repack.semicolon_token()))
}

fn build_reset_role<'a>(reset: &ast::ResetRole) -> Doc<'a> {
    let mut doc = Doc::text("reset");
    if let Some(role) = reset.role_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&role))
            .append(Doc::text("role"));
    }
    doc.append(build_semicolon(reset.semicolon_token()))
}

fn build_reset_session_auth<'a>(reset: &ast::ResetSessionAuth) -> Doc<'a> {
    let mut doc = Doc::text("reset");
    for (token, keyword) in [
        (reset.session_token(), "session"),
        (reset.authorization_token(), "authorization"),
    ] {
        if let Some(token) = token {
            doc = doc
                .append(Doc::space())
                .append(leading_comments_token(&token))
                .append(Doc::text(keyword));
        }
    }
    doc.append(build_semicolon(reset.semicolon_token()))
}

fn append_security_object_value<'a>(prefix: Doc<'a>, value: Doc<'a>) -> Doc<'a> {
    prefix
        .append(Doc::line_or_space().append(value).nest(2))
        .group()
}

fn build_security_object_value<'a>(node: &impl AstNode, value: Doc<'a>) -> Doc<'a> {
    leading_comments(node.syntax()).append(value)
}

fn build_aggregate_sig<'a>(aggregate: ast::Aggregate) -> Doc<'a> {
    let mut doc = aggregate
        .path_ref()
        .map(|path| build_path_ref(&path))
        .unwrap_or_else(Doc::nil);
    if let Some(params) = aggregate.param_list() {
        doc = doc
            .append(leading_comments(params.syntax()))
            .append(build_function_param_list(params));
    }
    doc
}

fn build_security_label_object<'a>(object: ast::SecurityLabelObject) -> Doc<'a> {
    match object {
        ast::SecurityLabelObject::ObjectAggregate(node) => {
            let value = node
                .aggregate()
                .map(|value| {
                    let doc = build_aggregate_sig(value.clone());
                    build_security_object_value(&value, doc)
                })
                .unwrap_or_else(Doc::nil);
            append_security_object_value(
                build_keyword_tokens([(node.aggregate_token(), "aggregate")]),
                value,
            )
        }
        ast::SecurityLabelObject::ObjectColumn(node) => {
            let value = node
                .name()
                .map(|name| {
                    let doc = name
                        .path_ref()
                        .map(|path| build_path_ref(&path))
                        .unwrap_or_else(Doc::nil);
                    build_security_object_value(&name, doc)
                })
                .unwrap_or_else(Doc::nil);
            append_security_object_value(
                build_keyword_tokens([(node.column_token(), "column")]),
                value,
            )
        }
        ast::SecurityLabelObject::ObjectDatabase(node) => append_security_object_value(
            build_keyword_tokens([(node.database_token(), "database")]),
            node.database_ref()
                .map(|name| build_security_object_value(&name, build_name(name.syntax())))
                .unwrap_or_else(Doc::nil),
        ),
        ast::SecurityLabelObject::ObjectDomain(node) => append_security_object_value(
            build_keyword_tokens([(node.domain_token(), "domain")]),
            node.domain_ref()
                .map(|name| {
                    let doc = name
                        .path_ref()
                        .map(|path| build_path_ref(&path))
                        .unwrap_or_else(Doc::nil);
                    build_security_object_value(&name, doc)
                })
                .unwrap_or_else(Doc::nil),
        ),
        ast::SecurityLabelObject::ObjectEventTrigger(node) => append_security_object_value(
            build_keyword_tokens([
                (node.event_token(), "event"),
                (node.trigger_token(), "trigger"),
            ]),
            node.event_trigger_ref()
                .map(|name| build_security_object_value(&name, build_name(name.syntax())))
                .unwrap_or_else(Doc::nil),
        ),
        ast::SecurityLabelObject::ObjectForeignTable(node) => append_security_object_value(
            build_keyword_tokens([
                (node.foreign_token(), "foreign"),
                (node.table_token(), "table"),
            ]),
            node.table_name_ref()
                .map(|name| {
                    let doc = name
                        .path_ref()
                        .map(|path| build_path_ref(&path))
                        .unwrap_or_else(Doc::nil);
                    build_security_object_value(&name, doc)
                })
                .unwrap_or_else(Doc::nil),
        ),
        ast::SecurityLabelObject::ObjectFunction(node) => append_security_object_value(
            build_keyword_tokens([(node.function_token(), "function")]),
            node.function_sig()
                .map(|value| {
                    let doc = build_function_sig(value.clone());
                    build_security_object_value(&value, doc)
                })
                .unwrap_or_else(Doc::nil),
        ),
        ast::SecurityLabelObject::ObjectLanguage(node) => append_security_object_value(
            build_keyword_tokens([
                (node.procedural_token(), "procedural"),
                (node.language_token(), "language"),
            ]),
            node.language_ref()
                .map(|name| build_security_object_value(&name, build_name(name.syntax())))
                .unwrap_or_else(Doc::nil),
        ),
        ast::SecurityLabelObject::ObjectLargeObject(node) => append_security_object_value(
            build_keyword_tokens([
                (node.large_token(), "large"),
                (node.object_token(), "object"),
            ]),
            node.literal()
                .map(|value| {
                    let doc = build_literal(value.clone());
                    build_security_object_value(&value, doc)
                })
                .unwrap_or_else(Doc::nil),
        ),
        ast::SecurityLabelObject::ObjectMaterializedView(node) => append_security_object_value(
            build_keyword_tokens([
                (node.materialized_token(), "materialized"),
                (node.view_token(), "view"),
            ]),
            node.view_ref()
                .map(|name| {
                    let doc = name
                        .path_ref()
                        .map(|path| build_path_ref(&path))
                        .unwrap_or_else(Doc::nil);
                    build_security_object_value(&name, doc)
                })
                .unwrap_or_else(Doc::nil),
        ),
        ast::SecurityLabelObject::ObjectProcedure(node) => append_security_object_value(
            build_keyword_tokens([(node.procedure_token(), "procedure")]),
            node.procedure_sig()
                .map(|value| {
                    let doc = build_procedure_sig(value.clone());
                    build_security_object_value(&value, doc)
                })
                .unwrap_or_else(Doc::nil),
        ),
        ast::SecurityLabelObject::ObjectPublication(node) => append_security_object_value(
            build_keyword_tokens([(node.publication_token(), "publication")]),
            node.publication_ref()
                .map(|name| build_security_object_value(&name, build_name(name.syntax())))
                .unwrap_or_else(Doc::nil),
        ),
        ast::SecurityLabelObject::ObjectRole(node) => append_security_object_value(
            build_keyword_tokens([(node.role_token(), "role")]),
            node.role_ref()
                .map(|role| {
                    let doc = build_role_ref(&role);
                    build_security_object_value(&role, doc)
                })
                .unwrap_or_else(Doc::nil),
        ),
        ast::SecurityLabelObject::ObjectRoutine(node) => append_security_object_value(
            build_keyword_tokens([(node.routine_token(), "routine")]),
            node.routine_sig()
                .map(|value| {
                    let doc = build_routine_sig(value.clone());
                    build_security_object_value(&value, doc)
                })
                .unwrap_or_else(Doc::nil),
        ),
        ast::SecurityLabelObject::ObjectSchema(node) => append_security_object_value(
            build_keyword_tokens([(node.schema_token(), "schema")]),
            node.schema_ref()
                .map(|name| build_security_object_value(&name, build_name(name.syntax())))
                .unwrap_or_else(Doc::nil),
        ),
        ast::SecurityLabelObject::ObjectSequence(node) => append_security_object_value(
            build_keyword_tokens([(node.sequence_token(), "sequence")]),
            node.sequence_ref()
                .map(|name| {
                    let doc = name
                        .path_ref()
                        .map(|path| build_path_ref(&path))
                        .unwrap_or_else(Doc::nil);
                    build_security_object_value(&name, doc)
                })
                .unwrap_or_else(Doc::nil),
        ),
        ast::SecurityLabelObject::ObjectSubscription(node) => append_security_object_value(
            build_keyword_tokens([(node.subscription_token(), "subscription")]),
            node.subscription_ref()
                .map(|name| build_security_object_value(&name, build_name(name.syntax())))
                .unwrap_or_else(Doc::nil),
        ),
        ast::SecurityLabelObject::ObjectTable(node) => append_security_object_value(
            build_keyword_tokens([(node.table_token(), "table")]),
            node.table_name_ref()
                .map(|name| {
                    let doc = name
                        .path_ref()
                        .map(|path| build_path_ref(&path))
                        .unwrap_or_else(Doc::nil);
                    build_security_object_value(&name, doc)
                })
                .unwrap_or_else(Doc::nil),
        ),
        ast::SecurityLabelObject::ObjectTablespace(node) => append_security_object_value(
            build_keyword_tokens([(node.tablespace_token(), "tablespace")]),
            node.tablespace_ref()
                .map(|name| build_security_object_value(&name, build_name(name.syntax())))
                .unwrap_or_else(Doc::nil),
        ),
        ast::SecurityLabelObject::ObjectType(node) => append_security_object_value(
            build_keyword_tokens([(node.type_token(), "type")]),
            node.type_name_ref()
                .map(|name| {
                    let doc = name
                        .path_ref()
                        .map(|path| build_path_ref(&path))
                        .unwrap_or_else(Doc::nil);
                    build_security_object_value(&name, doc)
                })
                .unwrap_or_else(Doc::nil),
        ),
        ast::SecurityLabelObject::ObjectView(node) => append_security_object_value(
            build_keyword_tokens([(node.view_token(), "view")]),
            node.view_ref()
                .map(|name| {
                    let doc = name
                        .path_ref()
                        .map(|path| build_path_ref(&path))
                        .unwrap_or_else(Doc::nil);
                    build_security_object_value(&name, doc)
                })
                .unwrap_or_else(Doc::nil),
        ),
    }
}

fn build_security_label<'a>(label: &ast::SecurityLabel) -> Doc<'a> {
    let mut doc = Doc::text("security");
    if let Some(token) = label.label_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&token))
            .append(Doc::text("label"));
    }
    if let Some(provider) = label.for_provider() {
        let mut provider_doc = Doc::text("for");
        if let Some(name) = provider.security_label_provider() {
            provider_doc = provider_doc
                .append(Doc::space())
                .append(leading_comments(name.syntax()))
                .append(build_name(name.syntax()));
        } else if let Some(literal) = provider.literal() {
            provider_doc = provider_doc
                .append(Doc::space())
                .append(leading_comments(literal.syntax()))
                .append(build_literal(literal));
        }
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(provider.syntax()))
                .append(provider_doc)
                .nest(2),
        );
    }
    let object = label.security_label_object();
    if let Some(on) = label.on_token() {
        let mut on_doc = leading_comments_token(&on).append(Doc::text("on"));
        if let Some(object) = object {
            on_doc = on_doc
                .append(Doc::space())
                .append(leading_comments(object.syntax()))
                .append(build_security_label_object(object));
        }
        doc = doc.append(Doc::line_or_space().append(on_doc).nest(2));
    } else if let Some(object) = object {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(object.syntax()))
                .append(build_security_label_object(object))
                .nest(2),
        );
    }

    let value = if let Some(literal) = label.literal() {
        Some(leading_comments(literal.syntax()).append(build_literal(literal)))
    } else {
        label
            .null_token()
            .map(|null| leading_comments_token(&null).append(Doc::text("null")))
    };
    if let Some(is) = label.is_token() {
        let mut is_doc = leading_comments_token(&is).append(Doc::text("is"));
        if let Some(value) = value {
            is_doc = is_doc.append(Doc::space()).append(value);
        }
        doc = doc.append(Doc::line_or_space().append(is_doc).nest(2));
    } else if let Some(value) = value {
        doc = doc.append(Doc::line_or_space().append(value).nest(2));
    }
    doc.group().append(build_semicolon(label.semicolon_token()))
}

fn build_set_constraints<'a>(set: &ast::SetConstraints) -> Doc<'a> {
    let mut doc = Doc::text("set");
    if let Some(constraints) = set.constraints_token() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments_token(&constraints))
                .append(Doc::text("constraints"))
                .nest(2),
        );
    }

    let names = set.constraint_name_refs().map(|name| {
        let syntax = name.syntax().clone();
        let name_doc = name
            .path_ref()
            .map(|path| build_path_ref(&path))
            .unwrap_or_else(Doc::nil);
        (leading_comments(&syntax).append(name_doc), syntax)
    });
    let target = if let Some(names) = build_comma_separated_docs(names) {
        Some(names)
    } else {
        set.all_token()
            .map(|all| leading_comments_token(&all).append(Doc::text("all")))
    };
    if let Some(target) = target {
        doc = doc.append(Doc::line_or_space().append(target).nest(2));
    }
    if let Some(timing) = set.constraint_timing() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(timing.syntax()))
                .append(build_keyword_node(timing.syntax()))
                .nest(2),
        );
    }
    doc.group().append(build_semicolon(set.semicolon_token()))
}

fn build_role_ref<'a>(role: &ast::RoleRef) -> Doc<'a> {
    if let Some(group) = role.group_token() {
        let mut doc = leading_comments_token(&group).append(Doc::text("group"));
        if let Some(ident) = role.ident_token() {
            doc = doc
                .append(Doc::space())
                .append(leading_comments_token(&ident))
                .append(build_name(role.syntax()));
        }
        doc
    } else if role.ident_token().is_some() {
        build_name(role.syntax())
    } else {
        build_keyword_node(role.syntax())
    }
}

fn build_set_role_target<'a>(target: ast::SetRoleTarget) -> Doc<'a> {
    match target {
        ast::SetRoleTarget::Literal(literal) => build_literal(literal),
        ast::SetRoleTarget::RoleRef(role) => build_role_ref(&role),
        ast::SetRoleTarget::SetRoleNone(none) => build_keyword_node(none.syntax()),
    }
}

fn build_set_role<'a>(set: &ast::SetRole) -> Doc<'a> {
    let mut doc = Doc::text("set");
    if let Some(scope) = set.set_scope() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(scope.syntax()))
                .append(build_keyword_node(scope.syntax()))
                .nest(2),
        );
    }
    if let Some(role) = set.role_token() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments_token(&role))
                .append(Doc::text("role"))
                .nest(2),
        );
    }
    if let Some(target) = set.set_role_target() {
        let comments = leading_comments(target.syntax());
        doc = doc.append(
            Doc::line_or_space()
                .append(comments)
                .append(build_set_role_target(target))
                .nest(2),
        );
    }
    doc.group().append(build_semicolon(set.semicolon_token()))
}

fn build_set_session_auth_target<'a>(target: ast::SetSessionAuthTarget) -> Doc<'a> {
    match target {
        ast::SetSessionAuthTarget::Literal(literal) => build_literal(literal),
        ast::SetSessionAuthTarget::RoleRef(role) => build_role_ref(&role),
        ast::SetSessionAuthTarget::SetSessionAuthDefault(default) => {
            build_keyword_node(default.syntax())
        }
    }
}

fn build_set_session_auth<'a>(set: &ast::SetSessionAuth) -> Doc<'a> {
    let mut doc = Doc::text("set");
    if let Some(scope) = set.set_scope() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(scope.syntax()))
                .append(build_keyword_node(scope.syntax()))
                .nest(2),
        );
    }
    if let Some(session) = set.session_token() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments_token(&session))
                .append(Doc::text("session"))
                .nest(2),
        );
    }
    if let Some(authorization) = set.authorization_token() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments_token(&authorization))
                .append(Doc::text("authorization"))
                .nest(2),
        );
    }
    if let Some(target) = set.set_session_auth_target() {
        let comments = leading_comments(target.syntax());
        doc = doc.append(
            Doc::line_or_space()
                .append(comments)
                .append(build_set_session_auth_target(target))
                .nest(2),
        );
    }
    doc.group().append(build_semicolon(set.semicolon_token()))
}

fn build_transaction_mode_list<'a>(list: ast::TransactionModeList) -> Doc<'a> {
    let modes = list.transaction_modes().map(|mode| {
        let syntax = mode.syntax().clone();
        (
            leading_comments(&syntax).append(build_keyword_node(&syntax)),
            syntax,
        )
    });
    build_comma_separated_docs(modes).unwrap_or_else(Doc::nil)
}

fn build_set_transaction<'a>(set: &ast::SetTransaction) -> Doc<'a> {
    let mut doc = Doc::text("set");
    let body = if let Some(characteristics) = set.session_characteristics() {
        let mut body = Doc::text("session");
        for (token, keyword) in [
            (characteristics.characteristics_token(), "characteristics"),
            (characteristics.as_token(), "as"),
            (characteristics.transaction_token(), "transaction"),
        ] {
            if let Some(token) = token {
                body = body
                    .append(Doc::line_or_space())
                    .append(leading_comments_token(&token))
                    .append(Doc::text(keyword));
            }
        }
        if let Some(modes) = characteristics.transaction_mode_list() {
            body = body.append(
                Doc::line_or_space()
                    .append(leading_comments(modes.syntax()))
                    .append(build_transaction_mode_list(modes))
                    .nest(2),
            );
        }
        Some((leading_comments(characteristics.syntax()), body))
    } else if let Some(modes) = set.transaction_modes() {
        let mut body = Doc::text("transaction");
        if let Some(list) = modes.transaction_mode_list() {
            body = body.append(
                Doc::line_or_space()
                    .append(leading_comments(list.syntax()))
                    .append(build_transaction_mode_list(list))
                    .nest(2),
            );
        }
        Some((leading_comments(modes.syntax()), body))
    } else {
        set.transaction_snapshot().map(|snapshot| {
            let mut body = Doc::text("transaction");
            if let Some(token) = snapshot.snapshot_token() {
                body = body
                    .append(Doc::line_or_space())
                    .append(leading_comments_token(&token))
                    .append(Doc::text("snapshot"));
            }
            if let Some(literal) = snapshot.literal() {
                body = body.append(
                    Doc::line_or_space()
                        .append(leading_comments(literal.syntax()))
                        .append(build_literal(literal))
                        .nest(2),
                );
            }
            (leading_comments(snapshot.syntax()), body)
        })
    };
    if let Some((comments, body)) = body {
        doc = doc.append(Doc::line_or_space().append(comments).append(body).nest(2));
    }
    doc.group().append(build_semicolon(set.semicolon_token()))
}

fn build_show<'a>(show: &ast::Show) -> Doc<'a> {
    let mut doc = Doc::text("show");
    if let Some(action) = show.show_action() {
        let comments = leading_comments(action.syntax());
        let action_doc = match action {
            ast::ShowAction::ConfigParameterRef(parameter) => parameter
                .path_ref()
                .map(|path| build_path_ref(&path))
                .unwrap_or_else(Doc::nil),
            ast::ShowAction::All(action) => build_keyword_node(action.syntax()),
            ast::ShowAction::SessionAuthorization(action) => build_keyword_node(action.syntax()),
            ast::ShowAction::TimeZone(action) => build_keyword_node(action.syntax()),
            ast::ShowAction::TransactionIsolationLevel(action) => {
                build_keyword_node(action.syntax())
            }
        };
        doc = doc.append(
            Doc::line_or_space()
                .append(comments)
                .append(action_doc)
                .nest(2),
        );
    }
    doc.group().append(build_semicolon(show.semicolon_token()))
}

fn build_unlisten<'a>(unlisten: &ast::Unlisten) -> Doc<'a> {
    let mut doc = Doc::text("unlisten");
    let target = if let Some(channel) = unlisten.channel_ref() {
        Some(leading_comments(channel.syntax()).append(build_name(channel.syntax())))
    } else {
        unlisten
            .star_token()
            .map(|star| leading_comments_token(&star).append(Doc::text("*")))
    };
    if let Some(target) = target {
        doc = doc.append(Doc::line_or_space().append(target).nest(2));
    }
    doc.group()
        .append(build_semicolon(unlisten.semicolon_token()))
}

fn build_set<'a>(set: &ast::Set) -> Doc<'a> {
    let mut doc = Doc::text("set");
    if let Some(scope) = set.set_scope() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(scope.syntax()))
                .append(build_keyword_node(scope.syntax()))
                .nest(2),
        );
    }
    if let Some(target) = set.set_target() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(target.syntax()))
                .append(build_set_target(target))
                .nest(2),
        );
    }
    doc.group().append(build_semicolon(set.semicolon_token()))
}

fn build_set_target<'a>(target: ast::SetTarget) -> Doc<'a> {
    match target {
        ast::SetTarget::SetCatalog(target) => build_set_literal_target("catalog", target.literal()),
        ast::SetTarget::SetSchemaValue(target) => {
            build_set_literal_target("schema", target.literal())
        }
        ast::SetTarget::SetConfig(target) => build_set_config(target),
        ast::SetTarget::SetTimeZone(target) => build_set_time_zone(target),
        ast::SetTarget::SetXmlOption(target) => build_set_xml_option(target),
    }
}

fn build_set_literal_target<'a>(keyword: &'static str, literal: Option<ast::Literal>) -> Doc<'a> {
    let mut doc = Doc::text(keyword);
    if let Some(literal) = literal {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(literal.syntax()))
            .append(build_literal(literal));
    }
    doc.group()
}

fn build_set_xml_option<'a>(target: ast::SetXmlOption) -> Doc<'a> {
    let mut doc = Doc::text("xml");
    if let Some(option) = target.option_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&option))
            .append(Doc::text("option"));
    }
    if let Some(value) = target.xml_document_or_content() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(value.syntax()))
            .append(build_keyword_node(value.syntax()));
    }
    doc.group()
}

fn build_set_time_zone<'a>(target: ast::SetTimeZone) -> Doc<'a> {
    let mut doc = Doc::text("time");
    if let Some(zone) = target.zone_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&zone))
            .append(Doc::text("zone"));
    }
    let value = if let Some(value) = target.config_value() {
        let value_doc = match value.clone() {
            ast::ConfigValue::ConfigValueName(name) => build_name(name.syntax()),
            ast::ConfigValue::Literal(literal) => build_literal(literal),
        };
        Some(leading_comments(value.syntax()).append(value_doc))
    } else if let Some(default) = target.default_token() {
        Some(leading_comments_token(&default).append(Doc::text("default")))
    } else {
        target
            .local_token()
            .map(|local| leading_comments_token(&local).append(Doc::text("local")))
    };
    if let Some(value) = value {
        doc = doc.append(Doc::space()).append(value);
    }
    doc.group()
}

fn build_set_config<'a>(set: ast::SetConfig) -> Doc<'a> {
    let mut doc = Doc::nil();
    if let Some(parameter) = set.config_parameter_ref() {
        doc = doc.append(leading_comments(parameter.syntax()));
        if let Some(path) = parameter.path_ref() {
            doc = doc.append(build_path_ref(&path));
        }
    }
    if let Some(assignment) = set.config_assignment() {
        let comments = leading_comments(assignment.syntax());
        doc = doc
            .append(Doc::line_or_space())
            .append(comments)
            .append(build_config_assignment(assignment));
    }
    doc.group()
}

fn build_set_config_param<'a>(set: ast::SetConfigParam) -> Doc<'a> {
    let mut doc = Doc::text("set");
    if let Some(parameter) = set.config_parameter_ref() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(parameter.syntax()));
        if let Some(path) = parameter.path_ref() {
            doc = doc.append(build_path_ref(&path));
        }
    }
    if let Some(assignment) = set.config_assignment() {
        let comments = leading_comments(assignment.syntax());
        doc = doc
            .append(Doc::space())
            .append(comments)
            .append(build_config_assignment(assignment));
    }
    doc.group()
}

fn build_config_assignment<'a>(assignment: ast::ConfigAssignment) -> Doc<'a> {
    match assignment {
        ast::ConfigAssignment::FromCurrent(current) => build_keyword_node(current.syntax()),
        ast::ConfigAssignment::ToConfigValue(values) => {
            let mut doc = if let Some(eq) = values.eq_token() {
                leading_comments_token(&eq).append(Doc::text("="))
            } else if let Some(to) = values.to_token() {
                leading_comments_token(&to).append(Doc::text("to"))
            } else {
                Doc::nil()
            };
            let value_docs = values.config_values().map(|value| {
                let syntax = value.syntax().clone();
                let value_doc = match value {
                    ast::ConfigValue::ConfigValueName(name) => build_name(name.syntax()),
                    ast::ConfigValue::Literal(literal) => build_literal(literal),
                };
                (leading_comments(&syntax).append(value_doc), syntax)
            });
            if let Some(values_doc) = build_comma_separated_docs(value_docs) {
                doc = doc.append(Doc::space()).append(values_doc);
            } else if let Some(default) = values.default_token() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments_token(&default))
                    .append(Doc::text("default"));
            } else if let Some(null) = values.null_token() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments_token(&null))
                    .append(Doc::text("null"));
            }
            doc.group()
        }
    }
}

fn build_create_index<'a>(create_index: &ast::CreateIndex) -> Doc<'a> {
    let mut doc = Doc::text("create");

    if let Some(unique_token) = create_index.unique_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&unique_token))
            .append(Doc::text("unique"));
    }
    if let Some(index_token) = create_index.index_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&index_token));
    } else {
        doc = doc.append(Doc::space());
    }
    doc = doc.append(Doc::text("index"));
    if let Some(concurrently_token) = create_index.concurrently_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&concurrently_token))
            .append(Doc::text("concurrently"));
    }
    if let Some(if_not_exists) = create_index.if_not_exists() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(if_not_exists.syntax()))
            .append(build_keyword_node(if_not_exists.syntax()));
    }
    if let Some(index) = create_index.index() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(index.syntax()));
        if let Some(path) = index.path() {
            doc = doc.append(build_path(&path));
        }
    }
    let mut on_doc = Doc::nil();
    if let Some(on_token) = create_index.on_token() {
        on_doc = on_doc
            .append(leading_comments_token(&on_token))
            .append(Doc::text("on"));
    }
    if let Some(table) = create_index.table_relation_name() {
        on_doc = on_doc
            .append(Doc::space())
            .append(build_table_relation_name(table));
    }
    if let Some(using_method) = create_index.using_method() {
        let mut using_doc = leading_comments(using_method.syntax()).append(Doc::text("using"));
        if let Some(method) = using_method.access_method_ref() {
            using_doc = using_doc
                .append(Doc::space())
                .append(leading_comments(method.syntax()))
                .append(build_name(method.syntax()));
        }
        if let Some(items) = create_index.partition_item_list() {
            using_doc = using_doc
                .append(Doc::space())
                .append(leading_comments(items.syntax()))
                .append(build_create_table_partition_items(items));
        }
        on_doc = on_doc.append(Doc::line_or_space()).append(using_doc);
        doc = doc.append(Doc::hard_line().append(on_doc.group()).nest(2));
    } else {
        if let Some(items) = create_index.partition_item_list() {
            on_doc = on_doc
                .append(Doc::space())
                .append(leading_comments(items.syntax()))
                .append(build_create_table_partition_items(items));
        }
        doc = doc.append(Doc::line_or_space().append(on_doc).nest(2));
    }
    if let Some(include) = create_index.index_include_clause() {
        let mut include_doc = leading_comments(include.syntax()).append(Doc::text("include"));
        if let Some(items) = include.partition_item_list() {
            include_doc = include_doc
                .append(Doc::space())
                .append(leading_comments(items.syntax()))
                .append(build_create_table_partition_items(items));
        }
        doc = doc.append(Doc::line_or_space().append(include_doc).nest(2));
    }
    if let Some(nulls) = create_index.nulls_distinct_option() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(nulls.syntax()))
                .append(build_keyword_node(nulls.syntax()))
                .nest(2),
        );
    }
    if let Some(params) = create_index.with_params() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(params.syntax()))
                .append(build_with_params(params))
                .nest(2),
        );
    }
    if let Some(tablespace) = create_index.tablespace_clause() {
        let mut tablespace_doc =
            leading_comments(tablespace.syntax()).append(Doc::text("tablespace"));
        if let Some(tablespace_ref) = tablespace.tablespace_ref() {
            tablespace_doc = tablespace_doc
                .append(Doc::space())
                .append(leading_comments(tablespace_ref.syntax()))
                .append(build_name(tablespace_ref.syntax()));
        }
        doc = doc.append(Doc::line_or_space().append(tablespace_doc).nest(2));
    }
    if let Some(where_clause) = create_index.where_clause() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(where_clause.syntax()))
                .append(build_where_clause(where_clause))
                .nest(2),
        );
    }

    doc.group()
        .append(build_semicolon(create_index.semicolon_token()))
}

fn build_create_view<'a>(create_view: &ast::CreateView) -> Doc<'a> {
    let mut doc = Doc::text("create");

    if let Some(or_replace) = create_view.or_replace() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(or_replace.syntax()))
            .append(build_keyword_node(or_replace.syntax()));
    }
    if let Some(persistence) = create_view.persistence() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(persistence.syntax()))
            .append(build_keyword_node(persistence.syntax()));
    }
    if let Some(recursive) = create_view.recursive_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&recursive))
            .append(Doc::text("recursive"));
    }
    if let Some(view_token) = create_view.view_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&view_token));
    } else {
        doc = doc.append(Doc::space());
    }
    doc = doc.append(Doc::text("view"));

    if let Some(view) = create_view.view() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(view.syntax()));
        if let Some(path) = view.path() {
            doc = doc.append(build_path(&path));
        }
    }
    if let Some(columns) = create_view.column_list() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(columns.syntax()))
            .append(build_cte_column_list(columns));
    }
    let has_with_params = if let Some(with_params) = create_view.with_params() {
        doc = doc.append(
            Doc::hard_line()
                .append(leading_comments(with_params.syntax()))
                .append(build_with_params(with_params))
                .nest(2),
        );
        true
    } else {
        false
    };
    if let Some(as_token) = create_view.as_token() {
        let as_doc = leading_comments_token(&as_token).append(Doc::text("as"));
        doc = if has_with_params {
            doc.append(Doc::hard_line().append(as_doc).nest(2))
        } else {
            doc.append(Doc::space()).append(as_doc)
        };
    }
    if let Some(query) = create_view.query() {
        doc = doc.append(
            Doc::hard_line()
                .append(leading_comments(query.syntax()))
                .append(build_select_variant(query))
                .nest(2),
        );
    }
    if let Some(check_option) = create_view.with_check_option() {
        doc = doc.append(
            Doc::hard_line()
                .append(leading_comments(check_option.syntax()))
                .append(build_with_check_option(check_option))
                .nest(2),
        );
    }

    doc.append(build_semicolon(create_view.semicolon_token()))
        .group()
}

fn build_create_table_as<'a>(create_table_as: &ast::CreateTableAs) -> Doc<'a> {
    let mut doc = Doc::text("create");

    if let Some(persistence) = create_table_as.persistence() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(persistence.syntax()))
            .append(build_keyword_node(persistence.syntax()));
    }
    if let Some(table_token) = create_table_as.table_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&table_token));
    } else {
        doc = doc.append(Doc::space());
    }
    doc = doc.append(Doc::text("table"));

    if let Some(if_not_exists) = create_table_as.if_not_exists() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(if_not_exists.syntax()))
            .append(build_keyword_node(if_not_exists.syntax()));
    }
    if let Some(table_name) = create_table_as.table_name() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(table_name.syntax()));
        if let Some(path) = table_name.path() {
            doc = doc.append(build_path(&path));
        }
    }
    if let Some(arg_list) = create_table_as.table_arg_list() {
        let comments = comments_before(arg_list.syntax().clone());
        if comment_tokens_before(arg_list.syntax().clone()).is_empty() {
            doc = doc.append(Doc::space());
        } else {
            doc = doc.append(comments).append(Doc::space());
        }
        let body = Doc::list(
            Itertools::intersperse(
                arg_list.args().map(build_table_arg),
                Doc::text(",").append(Doc::hard_line()),
            )
            .collect(),
        );
        doc = doc
            .append(Doc::text("("))
            .append(wrap_body(body).group())
            .append(Doc::text(")"));
    }

    let mut has_table_option = false;
    if let Some(using_method) = create_table_as.using_method() {
        has_table_option = true;
        let mut option = leading_comments(using_method.syntax()).append(Doc::text("using"));
        if let Some(method) = using_method.access_method_ref() {
            option = option
                .append(Doc::space())
                .append(leading_comments(method.syntax()))
                .append(build_name(method.syntax()));
        }
        doc = doc.append(Doc::line_or_space().append(option).nest(2));
    }
    if let Some(params) = create_table_as.table_params() {
        has_table_option = true;
        let option = match params {
            ast::TableParams::WithParams(params) => {
                leading_comments(params.syntax()).append(build_with_params(params))
            }
            ast::TableParams::WithoutOids(without_oids) => leading_comments(without_oids.syntax())
                .append(build_keyword_node(without_oids.syntax())),
        };
        doc = doc.append(Doc::line_or_space().append(option).nest(2));
    }
    if let Some(on_commit) = create_table_as.on_commit() {
        has_table_option = true;
        let mut option = leading_comments(on_commit.syntax()).append(Doc::text("on"));
        if let Some(commit_token) = on_commit.commit_token() {
            option = option
                .append(Doc::space())
                .append(leading_comments_token(&commit_token));
        } else {
            option = option.append(Doc::space());
        }
        option = option.append(Doc::text("commit"));
        if let Some(action) = on_commit.on_commit_action() {
            option = option
                .append(Doc::space())
                .append(leading_comments(action.syntax()))
                .append(build_keyword_node(action.syntax()));
        }
        doc = doc.append(Doc::line_or_space().append(option).nest(2));
    }
    if let Some(tablespace) = create_table_as.tablespace_clause() {
        has_table_option = true;
        let mut option = leading_comments(tablespace.syntax()).append(Doc::text("tablespace"));
        if let Some(name) = tablespace.tablespace_ref() {
            option = option
                .append(Doc::space())
                .append(leading_comments(name.syntax()))
                .append(build_name(name.syntax()));
        }
        doc = doc.append(Doc::line_or_space().append(option).nest(2));
    }

    if let Some(as_token) = create_table_as.as_token() {
        let as_doc = leading_comments_token(&as_token).append(Doc::text("as"));
        doc = if has_table_option {
            doc.append(Doc::hard_line().append(as_doc).nest(2))
        } else {
            doc.append(Doc::space()).append(as_doc)
        };
    }
    if let Some(query) = create_table_as.query() {
        let query_comments = leading_comments(query.syntax());
        let query_doc = match query {
            ast::CreateTableAsQuery::SelectVariant(select) => build_select_variant(select),
            ast::CreateTableAsQuery::Execute(execute) => build_execute(execute),
        };
        doc = doc.append(
            Doc::hard_line()
                .append(query_comments)
                .append(query_doc)
                .nest(2),
        );
    }
    if let Some(data_option) = create_table_as.data_option() {
        doc = doc.append(
            Doc::hard_line()
                .append(leading_comments(data_option.syntax()))
                .append(build_keyword_node(data_option.syntax()))
                .nest(2),
        );
    }

    doc.append(build_semicolon(create_table_as.semicolon_token()))
        .group()
}

fn build_execute<'a>(execute: ast::Execute) -> Doc<'a> {
    let mut doc = Doc::text("execute");
    if let Some(statement) = execute.prepared_statement_ref() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(statement.syntax()))
            .append(build_name(statement.syntax()));
    }
    if let Some(args) = execute.arg_list() {
        doc = doc
            .append(comments_before(args.syntax().clone()))
            .append(build_call_arg_list(args));
    }
    doc.group()
        .append(build_semicolon(execute.semicolon_token()))
}

fn build_with_check_option<'a>(check_option: ast::WithCheckOption) -> Doc<'a> {
    let mut doc = Doc::text("with");
    if let Some(level) = check_option.check_option_level() {
        let syntax = match &level {
            ast::CheckOptionLevel::CascadedCheckOption(level) => level.syntax(),
            ast::CheckOptionLevel::LocalCheckOption(level) => level.syntax(),
        };
        doc = doc
            .append(Doc::space())
            .append(leading_comments(syntax))
            .append(build_keyword_node(syntax));
    }
    if let Some(check_token) = check_option.check_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&check_token))
            .append(Doc::text("check"));
    }
    if let Some(option_token) = check_option.option_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&option_token))
            .append(Doc::text("option"));
    }
    doc
}

fn build_create_foreign_table<'a>(create_table: &ast::CreateForeignTable) -> Doc<'a> {
    let mut doc = Doc::text("create");

    if let Some(foreign) = create_table.foreign_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&foreign))
            .append(Doc::text("foreign"));
    }
    if let Some(table) = create_table.table_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&table))
            .append(Doc::text("table"));
    }
    if let Some(if_not_exists) = create_table.if_not_exists() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(if_not_exists.syntax()))
            .append(build_keyword_node(if_not_exists.syntax()));
    }
    if let Some(table_name) = create_table.table_name() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(table_name.syntax()));
        if let Some(path) = table_name.path() {
            doc = doc.append(build_path(&path));
        }
    }

    if let Some(partition_of) = create_table.partition_of() {
        let mut partition_doc = leading_comments(partition_of.syntax())
            .append(build_create_table_partition_of(partition_of));
        if let Some(arg_list) = create_table.table_arg_list() {
            partition_doc = append_table_arg_list(partition_doc, arg_list);
        }
        if let Some(partition_type) = create_table.partition_type() {
            let separator = if matches!(&partition_type, ast::PartitionType::PartitionDefault(_)) {
                Doc::space()
            } else {
                Doc::line_or_space()
            };
            partition_doc = partition_doc
                .append(separator)
                .append(leading_comments(partition_type.syntax()))
                .append(build_create_table_partition_type(partition_type));
        }
        doc = doc.append(Doc::hard_line().append(partition_doc).nest(2));
    } else {
        if let Some(arg_list) = create_table.table_arg_list() {
            doc = append_table_arg_list(doc, arg_list);
        }
        if let Some(inherits) = create_table.inherits() {
            doc = doc.append(
                Doc::line_or_space()
                    .append(leading_comments(inherits.syntax()))
                    .append(build_create_table_inherits(inherits))
                    .nest(2),
            );
        }
        if let Some(partition_type) = create_table.partition_type() {
            doc = doc.append(
                Doc::line_or_space()
                    .append(leading_comments(partition_type.syntax()))
                    .append(build_create_table_partition_type(partition_type))
                    .nest(2),
            );
        }
    }
    if let Some(server) = create_table.server_clause() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(server.syntax()))
                .append(build_server_clause(server))
                .nest(2),
        );
    }
    if let Some(options) = create_table.alter_option_list() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(options.syntax()))
                .append(build_alter_option_list(options))
                .nest(2),
        );
    }

    doc.group()
        .append(build_semicolon(create_table.semicolon_token()))
}

fn append_table_arg_list<'a>(mut doc: Doc<'a>, arg_list: ast::TableArgList) -> Doc<'a> {
    doc = doc.append(leading_comments(arg_list.syntax()));
    if let Some(l_paren) = arg_list.l_paren_token() {
        if comment_tokens_before(l_paren.clone()).is_empty() {
            doc = doc.append(Doc::space());
        } else {
            doc = doc.append(comments_before(l_paren));
        }
    }
    let body = Doc::list(
        Itertools::intersperse(
            arg_list.args().map(build_table_arg),
            Doc::text(",").append(Doc::hard_line()),
        )
        .collect(),
    );
    doc.append(Doc::text("("))
        .append(wrap_body(body).group())
        .append(Doc::text(")"))
}

fn build_server_clause<'a>(server: ast::ServerClause) -> Doc<'a> {
    let mut doc = Doc::text("server");
    if let Some(server_ref) = server.server_ref() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(server_ref.syntax()))
            .append(build_name(server_ref.syntax()));
    }
    doc
}

fn build_create_table<'a>(create_table: &ast::CreateTable) -> Doc<'a> {
    let mut doc = Doc::text("create");

    if let Some(persistence) = create_table.persistence() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(persistence.syntax()))
            .append(build_keyword_node(persistence.syntax()));
    }

    if let Some(table_token) = create_table.table_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&table_token));
    } else {
        doc = doc.append(Doc::space());
    }
    doc = doc.append(Doc::text("table"));

    if let Some(if_not_exists) = create_table.if_not_exists() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(if_not_exists.syntax()))
            .append(build_keyword_node(if_not_exists.syntax()));
    }

    if let Some(table_name) = create_table.table_name() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(table_name.syntax()));
        if let Some(path) = table_name.path() {
            doc = doc.append(build_path(&path));
        }
    }

    if let Some(partition_of) = create_table.partition_of() {
        doc = doc.append(
            Doc::hard_line()
                .append(leading_comments(partition_of.syntax()))
                .append(build_create_table_partition_of(partition_of))
                .nest(2),
        );
    }

    if let Some(of_type) = create_table.of_type() {
        let mut of_type_doc = leading_comments(of_type.syntax()).append(Doc::text("of"));
        if let Some(ty) = of_type.ty() {
            of_type_doc = of_type_doc
                .append(Doc::space())
                .append(leading_comments(ty.syntax()))
                .append(build_type(ty));
        }
        doc = doc.append(Doc::hard_line().append(of_type_doc).nest(2));
    }

    if let Some(arg_list) = create_table.table_arg_list() {
        if let Some(l_paren) = arg_list.l_paren_token() {
            if comment_tokens_before(l_paren.clone()).is_empty() {
                doc = doc.append(Doc::space());
            } else {
                doc = doc.append(comments_before(l_paren));
            }
        }
        let body = Doc::list(
            Itertools::intersperse(
                arg_list.args().map(build_table_arg),
                Doc::text(",").append(Doc::hard_line()),
            )
            .collect(),
        );
        doc = doc
            .append(Doc::text("("))
            .append(wrap_body(body).group())
            .append(Doc::text(")"));
    }

    if let Some(partition_type) = create_table.partition_type() {
        let separator = if matches!(&partition_type, ast::PartitionType::PartitionDefault(_)) {
            Doc::space()
        } else {
            Doc::line_or_space()
        };
        doc = doc.append(
            separator
                .append(leading_comments(partition_type.syntax()))
                .append(build_create_table_partition_type(partition_type))
                .nest(2),
        );
    }

    if let Some(inherits) = create_table.inherits() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(inherits.syntax()))
                .append(build_create_table_inherits(inherits))
                .nest(2),
        );
    }

    if let Some(partition_by) = create_table.partition_by() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(partition_by.syntax()))
                .append(build_create_table_partition_by(partition_by))
                .nest(2),
        );
    }

    if let Some(using_method) = create_table.using_method() {
        let mut using_doc = leading_comments(using_method.syntax()).append(Doc::text("using"));
        if let Some(method) = using_method.access_method_ref() {
            using_doc = using_doc
                .append(Doc::space())
                .append(leading_comments(method.syntax()))
                .append(build_name(method.syntax()));
        }
        doc = doc.append(Doc::line_or_space().append(using_doc).nest(2));
    }

    if let Some(params) = create_table.table_params() {
        let (separator, params_doc) = match params {
            ast::TableParams::WithParams(params) => (
                Doc::line_or_space(),
                leading_comments(params.syntax()).append(build_with_params(params)),
            ),
            ast::TableParams::WithoutOids(without_oids) => (
                Doc::hard_line(),
                leading_comments(without_oids.syntax())
                    .append(build_keyword_node(without_oids.syntax())),
            ),
        };
        doc = doc.append(separator.append(params_doc).nest(2));
    }

    if let Some(on_commit) = create_table.on_commit() {
        let mut on_commit_doc = leading_comments(on_commit.syntax()).append(Doc::text("on"));
        if let Some(commit_token) = on_commit.commit_token() {
            on_commit_doc = on_commit_doc
                .append(Doc::space())
                .append(leading_comments_token(&commit_token));
        } else {
            on_commit_doc = on_commit_doc.append(Doc::space());
        }
        on_commit_doc = on_commit_doc.append(Doc::text("commit"));
        if let Some(action) = on_commit.on_commit_action() {
            on_commit_doc = on_commit_doc
                .append(Doc::space())
                .append(leading_comments(action.syntax()))
                .append(build_keyword_node(action.syntax()));
        }
        doc = doc.append(Doc::hard_line().append(on_commit_doc).nest(2));
    }

    if let Some(tablespace) = create_table.tablespace_clause() {
        let mut tablespace_doc =
            leading_comments(tablespace.syntax()).append(Doc::text("tablespace"));
        if let Some(tablespace_ref) = tablespace.tablespace_ref() {
            tablespace_doc = tablespace_doc
                .append(Doc::space())
                .append(leading_comments(tablespace_ref.syntax()))
                .append(build_name(tablespace_ref.syntax()));
        }
        doc = doc.append(Doc::line_or_space().append(tablespace_doc).nest(2));
    }

    doc.group()
        .append(build_semicolon(create_table.semicolon_token()))
}

fn build_create_table_partition_of<'a>(partition_of: ast::PartitionOf) -> Doc<'a> {
    let mut doc = Doc::text("partition");
    if let Some(of_token) = partition_of.of_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&of_token));
    } else {
        doc = doc.append(Doc::space());
    }
    doc = doc.append(Doc::text("of"));
    if let Some(table) = partition_of.table_name_ref() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(table.syntax()));
        if let Some(path) = table.path_ref() {
            doc = doc.append(build_path_ref(&path));
        }
    }
    doc
}

fn build_create_table_inherits<'a>(inherits: ast::Inherits) -> Doc<'a> {
    let mut doc = Doc::text("inherits");
    if let Some(l_paren) = inherits.l_paren_token() {
        if comment_tokens_before(l_paren.clone()).is_empty() {
            doc = doc.append(Doc::space());
        } else {
            doc = doc.append(comments_before(l_paren));
        }
    }
    let tables = inherits.table_name_refs().map(|table| {
        let mut item = leading_comments(table.syntax());
        if let Some(path) = table.path_ref() {
            item = item.append(build_path_ref(&path));
        }
        (item, table.syntax().clone())
    });
    let mut body = build_comma_separated_docs(tables).unwrap_or_else(Doc::nil);
    if let Some(r_paren) = inherits.r_paren_token() {
        body = body.append(comments_before(r_paren));
    }
    doc.append(Doc::text("("))
        .append(wrap_body(body))
        .append(Doc::text(")"))
        .group()
}

fn build_create_table_partition_by<'a>(partition_by: ast::PartitionBy) -> Doc<'a> {
    let mut doc = Doc::text("partition");
    if let Some(by_token) = partition_by.by_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&by_token));
    } else {
        doc = doc.append(Doc::space());
    }
    doc = doc.append(Doc::text("by"));
    if let Some(strategy) = partition_by.partition_strategy() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(strategy.syntax()))
            .append(build_keyword_node(strategy.syntax()));
    }
    if let Some(items) = partition_by.partition_item_list() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(items.syntax()))
            .append(build_create_table_partition_items(items));
    }
    doc
}

fn build_create_table_partition_items<'a>(items: ast::PartitionItemList) -> Doc<'a> {
    let doc = items
        .l_paren_token()
        .map(comments_before)
        .unwrap_or_else(Doc::nil)
        .append(Doc::text("("));
    let item_docs = items.partition_items().map(|item| {
        let mut item_doc = leading_comments(item.syntax());
        if let Some(expr) = item.expr() {
            item_doc = item_doc.append(build_expr(expr));
        }
        if item.expr().is_none() {
            if let Some(collate) = item.collate() {
                item_doc = item_doc.append(build_collate_expr(collate));
            }
        }
        if let Some(op_class) = item.op_class_ref() {
            item_doc = item_doc
                .append(Doc::space())
                .append(leading_comments(op_class.syntax()));
            if let Some(path) = op_class.path_ref() {
                item_doc = item_doc.append(build_path_ref(&path));
            }
        }
        if let Some(attributes) = item.attribute_list() {
            item_doc = item_doc
                .append(Doc::space())
                .append(leading_comments(attributes.syntax()))
                .append(build_attribute_list(attributes));
        }
        if let Some(order) = item.sort_order() {
            item_doc = item_doc
                .append(Doc::space())
                .append(leading_comments(order.syntax()))
                .append(build_keyword_node(order.syntax()));
        }
        if let Some(nulls) = item.nulls_order() {
            item_doc = item_doc
                .append(Doc::space())
                .append(leading_comments(nulls.syntax()))
                .append(build_keyword_node(nulls.syntax()));
        }
        (item_doc, item.syntax().clone())
    });
    let mut body = build_comma_separated_docs(item_docs).unwrap_or_else(Doc::nil);
    if let Some(r_paren) = items.r_paren_token() {
        body = body.append(comments_before(r_paren));
    }
    doc.append(wrap_body(body)).append(Doc::text(")")).group()
}

fn build_create_table_partition_type<'a>(partition_type: ast::PartitionType) -> Doc<'a> {
    match partition_type {
        ast::PartitionType::PartitionDefault(_) => Doc::text("default"),
        ast::PartitionType::PartitionForValuesIn(values) => {
            let mut doc = build_partition_for_values_prefix(values.values_token());
            if let Some(in_token) = values.in_token() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments_token(&in_token));
            } else {
                doc = doc.append(Doc::space());
            }
            doc = doc.append(Doc::text("in"));
            if let Some(l_paren) = values.l_paren_token() {
                if comment_tokens_before(l_paren.clone()).is_empty() {
                    doc = doc.append(Doc::space());
                } else {
                    doc = doc.append(comments_before(l_paren));
                }
            }
            let body = build_comma_separated_exprs(values.exprs()).unwrap_or_else(Doc::nil);
            doc.append(Doc::text("("))
                .append(wrap_body(body))
                .append(Doc::text(")"))
                .group()
        }
        ast::PartitionType::PartitionForValuesFrom(values) => {
            let mut doc = build_partition_for_values_prefix(values.values_token());
            if let Some(from_token) = values.from_token() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments_token(&from_token));
            } else {
                doc = doc.append(Doc::space());
            }
            doc = doc.append(Doc::text("from"));
            if let Some(from) = values.from() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments(from.syntax()))
                    .append(build_create_table_partition_values(
                        from.l_paren_token(),
                        from.exprs(),
                        from.r_paren_token(),
                    ));
            }
            doc = doc.append(Doc::line_or_space());
            if let Some(to_token) = values.to_token() {
                doc = doc.append(leading_comments_token(&to_token));
            }
            doc = doc.append(Doc::text("to"));
            if let Some(to) = values.to() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments(to.syntax()))
                    .append(build_create_table_partition_values(
                        to.l_paren_token(),
                        to.exprs(),
                        to.r_paren_token(),
                    ));
            }
            doc.group()
        }
        ast::PartitionType::PartitionForValuesWith(values) => {
            let mut doc = build_partition_for_values_prefix(values.values_token());
            if let Some(with_token) = values.with_token() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments_token(&with_token));
            } else {
                doc = doc.append(Doc::space());
            }
            doc = doc.append(Doc::text("with"));
            if let Some(l_paren) = values.l_paren_token() {
                if comment_tokens_before(l_paren.clone()).is_empty() {
                    doc = doc.append(Doc::space());
                } else {
                    doc = doc.append(comments_before(l_paren));
                }
            }
            let mut parts = Vec::new();
            if let Some(modulus) = values.modulus() {
                parts.push(
                    leading_comments(modulus.syntax())
                        .append(build_keyword_node(modulus.syntax()))
                        .append(trailing_comments(modulus.syntax())),
                );
            }
            if let Some(remainder) = values.remainder() {
                parts.push(
                    leading_comments(remainder.syntax())
                        .append(build_keyword_node(remainder.syntax()))
                        .append(trailing_comments(remainder.syntax())),
                );
            }
            let body = Doc::list(
                Itertools::intersperse(
                    parts.into_iter(),
                    Doc::text(",").append(Doc::line_or_space()),
                )
                .collect(),
            );
            doc.append(Doc::text("("))
                .append(wrap_body(body))
                .append(Doc::text(")"))
                .group()
        }
    }
}

fn build_partition_for_values_prefix<'a>(values_token: Option<SyntaxToken>) -> Doc<'a> {
    let mut doc = Doc::text("for");
    if let Some(values_token) = values_token {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&values_token));
    } else {
        doc = doc.append(Doc::space());
    }
    doc.append(Doc::text("values"))
}

fn build_create_table_partition_values<'a>(
    l_paren: Option<SyntaxToken>,
    exprs: impl Iterator<Item = ast::Expr>,
    _r_paren: Option<SyntaxToken>,
) -> Doc<'a> {
    let doc = l_paren
        .map(comments_before)
        .unwrap_or_else(Doc::nil)
        .append(Doc::text("("));
    let body = build_comma_separated_exprs(exprs).unwrap_or_else(Doc::nil);
    doc.append(wrap_body(body)).append(Doc::text(")")).group()
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
        ast::TableArg::Column(column) => build_column(column),
        ast::TableArg::LikeClause(like_clause) => build_like_clause(like_clause),
        ast::TableArg::TableConstraint(table_constraint) => {
            build_table_constraint(table_constraint.clone())
        }
    });
    doc.append(trailing_comments(arg.syntax()))
}

fn build_column<'a>(column: &ast::Column) -> Doc<'a> {
    let mut doc = column
        .name()
        .map(|name| build_name(name.syntax()))
        .unwrap_or_else(Doc::nil);
    if let Some(ty) = column.ty() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(ty.syntax()))
            .append(build_type(ty));
    }
    if let Some(storage) = column.storage() {
        let mut clause = Doc::text("storage");
        if let Some(mode) = storage.storage_mode() {
            clause = clause
                .append(Doc::space())
                .append(leading_comments(mode.syntax()))
                .append(build_keyword_node(mode.syntax()));
        }
        doc = append_column_clause(doc, storage.syntax(), clause);
    }
    if let Some(compression) = column.compression_method() {
        let mut clause = Doc::text("compression");
        if let Some(method) = compression.compression_method_name() {
            clause = clause
                .append(Doc::space())
                .append(leading_comments(method.syntax()))
                .append(build_keyword_node(method.syntax()));
        }
        doc = append_column_clause(doc, compression.syntax(), clause);
    }
    if let Some(options) = column.with_options() {
        doc = append_column_clause(doc, options.syntax(), build_keyword_node(options.syntax()));
    }
    if let Some(options) = column.alter_option_list() {
        let syntax = options.syntax().clone();
        doc = append_column_clause(doc, &syntax, build_alter_option_list(options));
    }
    if let Some(collate) = column.collate() {
        let syntax = collate.syntax().clone();
        doc = append_column_clause(doc, &syntax, build_collate_expr(collate));
    }
    for constraint in column.constraints() {
        let syntax = constraint.syntax().clone();
        doc = append_column_clause(doc, &syntax, build_column_constraint(constraint));
    }
    doc.group()
}

fn append_column_clause<'a>(doc: Doc<'a>, syntax: &SyntaxNode, clause: Doc<'a>) -> Doc<'a> {
    doc.append(
        Doc::line_or_space()
            .append(leading_comments(syntax))
            .append(clause)
            .nest(2),
    )
}

fn build_alter_option_list<'a>(list: ast::AlterOptionList) -> Doc<'a> {
    let mut doc = Doc::text("options");
    if let Some(l_paren) = list.l_paren_token() {
        if comment_tokens_before(l_paren.clone()).is_empty() {
            doc = doc.append(Doc::space());
        } else {
            doc = doc.append(comments_before(l_paren));
        }
    } else {
        doc = doc.append(Doc::space());
    }
    let items = list.alter_options().map(|option| {
        let item = leading_comments(option.syntax()).append(build_alter_option(&option));
        (item, option.syntax().clone())
    });
    let mut body = build_comma_separated_docs(items).unwrap_or_else(Doc::nil);
    if let Some(r_paren) = list.r_paren_token() {
        body = body.append(comments_before(r_paren));
    }
    doc.append(Doc::text("("))
        .append(Doc::hard_line().append(body).nest(2))
        .append(Doc::hard_line())
        .append(Doc::text(")"))
}

fn build_alter_option<'a>(option: &ast::AlterOption) -> Doc<'a> {
    match option {
        ast::AlterOption::AddForeignOption(option) => {
            let mut doc = option
                .add_token()
                .map(|token| leading_comments_token(&token).append(Doc::text("add")))
                .unwrap_or_else(Doc::nil);
            if let Some(name) = option.foreign_option_name() {
                if option.add_token().is_some() {
                    doc = doc.append(Doc::space());
                }
                doc = doc
                    .append(leading_comments(name.syntax()))
                    .append(build_name(name.syntax()));
            }
            if let Some(value) = option.literal() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments(value.syntax()))
                    .append(build_literal(value));
            }
            doc
        }
        ast::AlterOption::SetForeignOption(option) => {
            let mut doc = Doc::text("set");
            if let Some(name) = option.foreign_option_name() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments(name.syntax()))
                    .append(build_name(name.syntax()));
            }
            if let Some(value) = option.literal() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments(value.syntax()))
                    .append(build_literal(value));
            }
            doc
        }
        ast::AlterOption::DropForeignOption(option) => {
            let mut doc = Doc::text("drop");
            if let Some(name) = option.foreign_option_name() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments(name.syntax()))
                    .append(build_name(name.syntax()));
            }
            doc
        }
    }
}

fn build_column_constraint<'a>(constraint: ast::ColumnConstraint) -> Doc<'a> {
    match constraint {
        ast::ColumnConstraint::CheckConstraint(constraint) => build_check_constraint(constraint),
        ast::ColumnConstraint::DefaultConstraint(constraint) => {
            let mut doc = build_constraint_name_clause(constraint.constraint_name_clause());
            if let Some(default) = constraint.default_token() {
                doc = doc
                    .append(leading_comments_token(&default))
                    .append(Doc::text("default"));
            }
            if let Some(expr) = constraint.expr() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments(expr.syntax()))
                    .append(build_expr(expr));
            }
            append_constraint_options(doc, constraint.constraint_options())
                .nest(2)
                .group()
        }
        ast::ColumnConstraint::ExcludeConstraint(constraint) => {
            build_exclude_constraint(constraint)
        }
        ast::ColumnConstraint::GeneratedConstraint(constraint) => {
            build_generated_constraint(constraint)
        }
        ast::ColumnConstraint::NotNullConstraint(constraint) => {
            let mut doc = build_constraint_name_clause(constraint.constraint_name_clause());
            if let Some(not) = constraint.not_token() {
                doc = doc
                    .append(leading_comments_token(&not))
                    .append(Doc::text("not"));
            }
            if let Some(null) = constraint.null_token() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments_token(&null))
                    .append(Doc::text("null"));
            }
            if let Some(column) = constraint.column_name_ref() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments(column.syntax()))
                    .append(build_name(column.syntax()));
            }
            append_constraint_options(doc, constraint.constraint_options())
                .nest(2)
                .group()
        }
        ast::ColumnConstraint::NullConstraint(constraint) => {
            let mut doc = build_constraint_name_clause(constraint.constraint_name_clause());
            if let Some(null) = constraint.null_token() {
                doc = doc
                    .append(leading_comments_token(&null))
                    .append(Doc::text("null"));
            }
            append_constraint_options(doc, constraint.constraint_options())
                .nest(2)
                .group()
        }
        ast::ColumnConstraint::PrimaryKeyConstraint(constraint) => {
            build_primary_key_constraint(constraint)
        }
        ast::ColumnConstraint::ReferencesConstraint(constraint) => {
            build_references_constraint(constraint)
        }
        ast::ColumnConstraint::UniqueConstraint(constraint) => build_unique_constraint(constraint),
    }
}

fn build_references_constraint<'a>(constraint: ast::ReferencesConstraint) -> Doc<'a> {
    let mut doc = build_constraint_name_clause(constraint.constraint_name_clause());
    if let Some(references) = constraint.references_token() {
        doc = doc
            .append(leading_comments_token(&references))
            .append(Doc::text("references"));
    }
    if let Some(table) = constraint.table() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(table.syntax()));
        if let Some(path) = table.path_ref() {
            doc = doc.append(build_path_ref(&path));
        }
    }
    if let Some(column) = constraint.column() {
        if let Some(l_paren) = constraint.l_paren_token() {
            doc = doc.append(comments_before(l_paren));
        }
        doc = doc
            .append(Doc::text("("))
            .append(leading_comments(column.syntax()))
            .append(build_name(column.syntax()));
        if let Some(r_paren) = constraint.r_paren_token() {
            doc = doc.append(comments_before(r_paren));
        }
        doc = doc.append(Doc::text(")"));
    }
    if let Some(match_type) = constraint.match_type() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(match_type.syntax()))
            .append(build_keyword_node(match_type.syntax()));
    }
    if let Some(action) = constraint.on_delete_action() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(action.syntax()))
            .append(build_reference_action(
                action.on_token(),
                action.delete_token(),
                "delete",
                action.ref_action(),
            ));
    }
    if let Some(action) = constraint.on_update_action() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(action.syntax()))
            .append(build_reference_action(
                action.on_token(),
                action.update_token(),
                "update",
                action.ref_action(),
            ));
    }
    append_constraint_options(doc, constraint.constraint_options())
        .nest(2)
        .group()
}

fn build_generated_constraint<'a>(constraint: ast::GeneratedConstraint) -> Doc<'a> {
    let mut doc = build_constraint_name_clause(constraint.constraint_name_clause());
    if let Some(generated) = constraint.generated_token() {
        doc = doc
            .append(leading_comments_token(&generated))
            .append(Doc::text("generated"));
    }
    if let Some(generated_as) = constraint.generated_as() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(generated_as.syntax()))
            .append(match generated_as {
                ast::GeneratedAs::GeneratedIdentity(identity) => {
                    let mut body = identity
                        .generated_when()
                        .map(build_generated_when)
                        .unwrap_or_else(Doc::nil);
                    if let Some(as_token) = identity.as_token() {
                        body = body
                            .append(Doc::space())
                            .append(leading_comments_token(&as_token))
                            .append(Doc::text("as"));
                    }
                    if let Some(identity_token) = identity.identity_token() {
                        body = body
                            .append(Doc::space())
                            .append(leading_comments_token(&identity_token))
                            .append(Doc::text("identity"));
                    }
                    if let Some(options) = identity.sequence_option_list() {
                        body = body
                            .append(Doc::space())
                            .append(leading_comments(options.syntax()))
                            .append(build_sequence_option_list(options));
                    }
                    body
                }
                ast::GeneratedAs::GeneratedStored(stored) => {
                    let mut body = stored
                        .generated_when()
                        .map(build_generated_when)
                        .unwrap_or_else(Doc::nil);
                    if let Some(as_token) = stored.as_token() {
                        body = body
                            .append(Doc::space())
                            .append(leading_comments_token(&as_token))
                            .append(Doc::text("as"));
                    }
                    if let Some(l_paren) = stored.l_paren_token() {
                        if comment_tokens_before(l_paren.clone()).is_empty() {
                            body = body.append(Doc::space());
                        } else {
                            body = body.append(comments_before(l_paren));
                        }
                    }
                    let mut expr = stored
                        .expr()
                        .map(|expr| leading_comments(expr.syntax()).append(build_expr(expr)))
                        .unwrap_or_else(Doc::nil);
                    if let Some(r_paren) = stored.r_paren_token() {
                        expr = expr.append(comments_before(r_paren));
                    }
                    body = body
                        .append(Doc::text("("))
                        .append(wrap_body(expr))
                        .append(Doc::text(")"));
                    if let Some(kind) = stored.generated_kind() {
                        body = body
                            .append(Doc::space())
                            .append(leading_comments(kind.syntax()))
                            .append(build_keyword_node(kind.syntax()));
                    }
                    body.group()
                }
            });
    }
    doc = doc.group();
    for option in constraint.constraint_options() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(option.syntax()))
                .append(build_keyword_node(option.syntax()))
                .nest(2),
        );
    }
    doc.group()
}

fn build_generated_when<'a>(when: ast::GeneratedWhen) -> Doc<'a> {
    match when {
        ast::GeneratedWhen::GeneratedAlways(always) => build_keyword_node(always.syntax()),
        ast::GeneratedWhen::GeneratedByDefault(by_default) => {
            build_keyword_node(by_default.syntax())
        }
    }
}

fn build_sequence_option_list<'a>(list: ast::SequenceOptionList) -> Doc<'a> {
    let doc = list
        .l_paren_token()
        .map(comments_before)
        .unwrap_or_else(Doc::nil)
        .append(Doc::text("("));
    let options = list
        .sequence_options()
        .map(|option| leading_comments(option.syntax()).append(build_sequence_option(option)));
    let mut body = Doc::list(Itertools::intersperse(options, Doc::line_or_space()).collect());
    if let Some(r_paren) = list.r_paren_token() {
        body = body.append(comments_before(r_paren));
    }
    doc.append(Doc::hard_line().append(body).nest(2))
        .append(Doc::hard_line())
        .append(Doc::text(")"))
}

fn build_sequence_option<'a>(option: ast::SequenceOption) -> Doc<'a> {
    match option {
        ast::SequenceOption::OptionAsType(option) => {
            let mut doc = Doc::text("as");
            if let Some(ty) = option.ty() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments(ty.syntax()))
                    .append(build_type(ty));
            }
            doc
        }
        ast::SequenceOption::OptionCache(option) => {
            append_optional_literal(Doc::text("cache"), option.literal())
        }
        ast::SequenceOption::OptionIncrement(option) => {
            let mut doc = option
                .increment_token()
                .map(|token| leading_comments_token(&token).append(Doc::text("increment")))
                .unwrap_or_else(Doc::nil);
            if let Some(by) = option.by_token() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments_token(&by))
                    .append(Doc::text("by"));
            }
            append_optional_literal(doc, option.literal())
        }
        ast::SequenceOption::OptionMaxValue(option) => {
            append_optional_literal(Doc::text("maxvalue"), option.literal())
        }
        ast::SequenceOption::OptionMinValue(option) => {
            append_optional_literal(Doc::text("minvalue"), option.literal())
        }
        ast::SequenceOption::OptionRestart(option) => {
            let mut doc = option
                .restart_token()
                .map(|token| leading_comments_token(&token).append(Doc::text("restart")))
                .unwrap_or_else(Doc::nil);
            if let Some(with) = option.with_token() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments_token(&with))
                    .append(Doc::text("with"));
            }
            append_optional_literal(doc, option.literal())
        }
        ast::SequenceOption::OptionStart(option) => {
            let mut doc = option
                .start_token()
                .map(|token| leading_comments_token(&token).append(Doc::text("start")))
                .unwrap_or_else(Doc::nil);
            if let Some(with) = option.with_token() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments_token(&with))
                    .append(Doc::text("with"));
            }
            append_optional_literal(doc, option.literal())
        }
        ast::SequenceOption::OptionOwnedBy(option) => {
            let mut doc = option
                .owned_token()
                .map(|token| leading_comments_token(&token).append(Doc::text("owned")))
                .unwrap_or_else(Doc::nil);
            if let Some(by) = option.by_token() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments_token(&by))
                    .append(Doc::text("by"));
            }
            if let Some(target) = option.owned_by_target() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments(target.syntax()))
                    .append(match target {
                        ast::OwnedByTarget::OwnedByNone(_) => Doc::text("none"),
                        ast::OwnedByTarget::QualifiedColumnNameRef(name) => name
                            .path_ref()
                            .map(|path| build_path_ref(&path))
                            .unwrap_or_else(Doc::nil),
                    });
            }
            doc
        }
        ast::SequenceOption::OptionSequenceName(option) => {
            let mut doc = option
                .sequence_token()
                .map(|token| leading_comments_token(&token).append(Doc::text("sequence")))
                .unwrap_or_else(Doc::nil);
            if let Some(name) = option.name_token() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments_token(&name))
                    .append(Doc::text("name"));
            }
            if let Some(sequence) = option.sequence() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments(sequence.syntax()));
                if let Some(path) = sequence.path() {
                    doc = doc.append(build_path(&path));
                }
            }
            doc
        }
        ast::SequenceOption::OptionCycle(option) => build_keyword_node(option.syntax()),
        ast::SequenceOption::OptionLogged(option) => build_keyword_node(option.syntax()),
        ast::SequenceOption::OptionNoCycle(option) => build_keyword_node(option.syntax()),
        ast::SequenceOption::OptionNoMaxValue(option) => build_keyword_node(option.syntax()),
        ast::SequenceOption::OptionNoMinValue(option) => build_keyword_node(option.syntax()),
        ast::SequenceOption::OptionUnlogged(option) => build_keyword_node(option.syntax()),
    }
}

fn append_optional_literal<'a>(doc: Doc<'a>, literal: Option<ast::Literal>) -> Doc<'a> {
    literal.map_or(doc.clone(), |literal| {
        doc.append(Doc::space())
            .append(leading_comments(literal.syntax()))
            .append(build_literal(literal))
    })
}

fn build_table_constraint<'a>(constraint: ast::TableConstraint) -> Doc<'a> {
    match constraint {
        ast::TableConstraint::CheckConstraint(constraint) => build_check_constraint(constraint),
        ast::TableConstraint::ExcludeConstraint(constraint) => build_exclude_constraint(constraint),
        ast::TableConstraint::ForeignKeyConstraint(constraint) => {
            build_foreign_key_constraint(constraint)
        }
        ast::TableConstraint::PrimaryKeyConstraint(constraint) => {
            build_primary_key_constraint(constraint)
        }
        ast::TableConstraint::UniqueConstraint(constraint) => build_unique_constraint(constraint),
    }
}

fn build_constraint_name_clause<'a>(clause: Option<ast::ConstraintNameClause>) -> Doc<'a> {
    let Some(clause) = clause else {
        return Doc::nil();
    };
    let mut doc = Doc::text("constraint");
    if let Some(name) = clause.constraint_name() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(name.syntax()))
            .append(build_name(name.syntax()));
    }
    doc.append(Doc::space())
}

fn build_check_constraint<'a>(constraint: ast::CheckConstraint) -> Doc<'a> {
    let mut doc = build_constraint_name_clause(constraint.constraint_name_clause());
    if let Some(check) = constraint.check_token() {
        doc = doc
            .append(leading_comments_token(&check))
            .append(Doc::text("check"));
    }
    if let Some(l_paren) = constraint.l_paren_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&l_paren));
    }
    doc = doc.append(Doc::text("("));

    let mut body = Doc::nil();
    if let Some(expr) = constraint.expr() {
        body = body
            .append(leading_comments(expr.syntax()))
            .append(build_expr(expr));
    }
    if let Some(r_paren) = constraint.r_paren_token() {
        body = body.append(comments_before(r_paren));
    }
    doc = doc.append(wrap_body(body)).append(Doc::text(")")).group();

    let mut options = Doc::nil();
    for option in constraint.constraint_options() {
        options = options
            .append(Doc::line_or_space())
            .append(leading_comments(option.syntax()))
            .append(build_keyword_node(option.syntax()));
    }
    doc.append(options.nest(2)).group()
}

fn build_primary_key_constraint<'a>(constraint: ast::PrimaryKeyConstraint) -> Doc<'a> {
    let mut doc = build_constraint_name_clause(constraint.constraint_name_clause());
    if let Some(primary) = constraint.primary_token() {
        doc = doc
            .append(leading_comments_token(&primary))
            .append(Doc::text("primary"));
    }
    if let Some(key) = constraint.key_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&key))
            .append(Doc::text("key"));
    }
    if let Some(using_index) = constraint.using_index() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(using_index.syntax()))
            .append(build_using_index_name(using_index));
    } else if let Some(parameters) = constraint.index_parameters() {
        doc = doc
            .append(leading_comments(parameters.syntax()))
            .append(build_index_parameters(parameters));
    }
    append_constraint_options(doc, constraint.constraint_options())
        .nest(2)
        .group()
}

fn build_unique_constraint<'a>(constraint: ast::UniqueConstraint) -> Doc<'a> {
    let mut doc = build_constraint_name_clause(constraint.constraint_name_clause());
    if let Some(unique) = constraint.unique_token() {
        doc = doc
            .append(leading_comments_token(&unique))
            .append(Doc::text("unique"));
    }
    if let Some(using_index) = constraint.using_index() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(using_index.syntax()))
            .append(build_using_index_name(using_index));
    } else if let Some(parameters) = constraint.index_parameters() {
        doc = doc
            .append(leading_comments(parameters.syntax()))
            .append(build_index_parameters(parameters));
    }
    append_constraint_options(doc, constraint.constraint_options())
        .nest(2)
        .group()
}

fn build_using_index_name<'a>(using_index: ast::UsingIndexName) -> Doc<'a> {
    let mut doc = Doc::text("using");
    if let Some(index) = using_index.index_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&index))
            .append(Doc::text("index"));
    }
    if let Some(index) = using_index.index_ref() {
        if let Some(path) = index.path_ref() {
            doc = doc
                .append(Doc::space())
                .append(leading_comments(index.syntax()))
                .append(build_path_ref(&path));
        }
    }
    doc
}

fn build_index_parameters<'a>(parameters: ast::IndexParameters) -> Doc<'a> {
    let mut doc = Doc::nil();
    if let Some(nulls) = parameters.nulls_distinct_option() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(nulls.syntax()))
            .append(build_keyword_node(nulls.syntax()));
    }
    if let Some(columns) = parameters.column_list() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(columns.syntax()))
            .append(build_constraint_column_ref_list(columns));
    }
    if let Some(include) = parameters.constraint_include_clause() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(include.syntax()))
            .append(build_constraint_include_clause(include));
    }
    if let Some(with_params) = parameters.with_params() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(with_params.syntax()))
            .append(build_with_params(with_params));
    }
    if let Some(tablespace) = parameters.constraint_index_tablespace() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(tablespace.syntax()))
            .append(build_constraint_index_tablespace(tablespace));
    }
    doc
}

fn build_constraint_column_ref_list<'a>(list: ast::ConstraintColumnRefList) -> Doc<'a> {
    let suffix = list.without_overlaps().map(|overlaps| {
        Doc::space()
            .append(leading_comments(overlaps.syntax()))
            .append(build_keyword_node(overlaps.syntax()))
    });
    build_column_names(
        list.l_paren_token(),
        list.column_name_refs(),
        suffix,
        list.r_paren_token(),
    )
}

fn build_column_ref_list<'a>(list: ast::ColumnRefList) -> Doc<'a> {
    build_column_names(
        list.l_paren_token(),
        list.column_name_refs(),
        None,
        list.r_paren_token(),
    )
}

fn build_column_names<'a>(
    l_paren: Option<SyntaxToken>,
    names: impl Iterator<Item = ast::ColumnNameRef>,
    suffix: Option<Doc<'a>>,
    r_paren: Option<SyntaxToken>,
) -> Doc<'a> {
    let doc = l_paren
        .map(comments_before)
        .unwrap_or_else(Doc::nil)
        .append(Doc::text("("));
    let items = names.map(|name| {
        (
            leading_comments(name.syntax()).append(build_name(name.syntax())),
            name.syntax().clone(),
        )
    });
    let mut body = build_comma_separated_docs(items).unwrap_or_else(Doc::nil);
    if let Some(suffix) = suffix {
        body = body.append(suffix);
    }
    if let Some(r_paren) = r_paren {
        body = body.append(comments_before(r_paren));
    }
    doc.append(wrap_body(body)).append(Doc::text(")")).group()
}

fn build_constraint_include_clause<'a>(include: ast::ConstraintIncludeClause) -> Doc<'a> {
    let mut doc = Doc::text("include");
    if let Some(columns) = include.column_ref_list() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(columns.syntax()))
            .append(build_column_ref_list(columns));
    }
    doc
}

fn build_with_params<'a>(with_params: ast::WithParams) -> Doc<'a> {
    let mut doc = Doc::text("with");
    if let Some(attributes) = with_params.attribute_list() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(attributes.syntax()))
            .append(build_attribute_list(attributes));
    }
    doc
}

fn build_attribute_list<'a>(list: ast::AttributeList) -> Doc<'a> {
    let doc = list
        .l_paren_token()
        .map(comments_before)
        .unwrap_or_else(Doc::nil)
        .append(Doc::text("("));
    let items = list.attribute_options().map(|option| {
        let mut item = option
            .namespace()
            .map(|namespace| build_name(namespace.syntax()))
            .unwrap_or_else(Doc::nil);
        if let Some(dot) = option.dot_token() {
            item = item.append(comments_before(dot)).append(Doc::text("."));
        }
        if let Some(name) = option.name() {
            item = item
                .append(leading_comments(name.syntax()))
                .append(build_name(name.syntax()));
        }
        if let Some(eq) = option.eq_token() {
            item = item
                .append(Doc::space())
                .append(leading_comments_token(&eq))
                .append(Doc::text("="));
        }
        if let Some(value) = option.attribute_value() {
            item = item
                .append(Doc::space())
                .append(leading_comments(value.syntax()))
                .append(build_attribute_value(value));
        }
        (
            leading_comments(option.syntax()).append(item),
            option.syntax().clone(),
        )
    });
    let mut body = build_comma_separated_docs(items).unwrap_or_else(Doc::nil);
    if let Some(r_paren) = list.r_paren_token() {
        body = body.append(comments_before(r_paren));
    }
    doc.append(wrap_body(body)).append(Doc::text(")")).group()
}

fn build_attribute_value<'a>(value: ast::AttributeValue) -> Doc<'a> {
    if let Some(literal) = value.literal() {
        build_literal(literal)
    } else if let Some(ty) = value.ty() {
        build_type(ty)
    } else if value.none_token().is_some() {
        Doc::text("none")
    } else if let Some(op) = value.op() {
        if value.operator_token().is_some() {
            let mut doc = Doc::text("operator");
            if let Some(l_paren) = value.l_paren_token() {
                doc = doc.append(comments_before(l_paren));
            }
            doc = doc.append(Doc::text("(")).append(build_operator(&op));
            if let Some(r_paren) = value.r_paren_token() {
                doc = doc.append(comments_before(r_paren));
            }
            doc.append(Doc::text(")"))
        } else {
            build_operator(&op)
        }
    } else {
        Doc::nil()
    }
}

fn build_constraint_index_tablespace<'a>(tablespace: ast::ConstraintIndexTablespace) -> Doc<'a> {
    let mut doc = Doc::text("using");
    if let Some(index) = tablespace.index_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&index))
            .append(Doc::text("index"));
    }
    if let Some(token) = tablespace.tablespace_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&token))
            .append(Doc::text("tablespace"));
    }
    if let Some(name) = tablespace.tablespace_ref() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(name.syntax()))
            .append(build_name(name.syntax()));
    }
    doc
}

fn append_constraint_options<'a>(
    mut doc: Doc<'a>,
    options: impl Iterator<Item = ast::ConstraintOption>,
) -> Doc<'a> {
    for option in options {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(option.syntax()))
            .append(build_keyword_node(option.syntax()));
    }
    doc
}

fn build_foreign_key_constraint<'a>(constraint: ast::ForeignKeyConstraint) -> Doc<'a> {
    let mut doc = build_constraint_name_clause(constraint.constraint_name_clause());
    if let Some(foreign) = constraint.foreign_token() {
        doc = doc
            .append(leading_comments_token(&foreign))
            .append(Doc::text("foreign"));
    }
    if let Some(key) = constraint.key_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&key))
            .append(Doc::text("key"));
    }
    if let Some(columns) = constraint.from_columns() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(columns.syntax()))
            .append(build_foreign_key_column_list(columns));
    }
    if let Some(references) = constraint.references_token() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments_token(&references))
            .append(Doc::text("references"));
    }
    if let Some(table) = constraint.table_name_ref() {
        if let Some(path) = table.path_ref() {
            doc = doc
                .append(Doc::space())
                .append(leading_comments(table.syntax()))
                .append(build_path_ref(&path));
        }
    }
    if let Some(columns) = constraint.to_columns() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(columns.syntax()))
            .append(build_foreign_key_column_list(columns));
    }
    if let Some(match_type) = constraint.match_type() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(match_type.syntax()))
            .append(build_keyword_node(match_type.syntax()));
    }
    if let Some(action) = constraint.on_delete_action() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(action.syntax()))
            .append(build_reference_action(
                action.on_token(),
                action.delete_token(),
                "delete",
                action.ref_action(),
            ));
    }
    if let Some(action) = constraint.on_update_action() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(action.syntax()))
            .append(build_reference_action(
                action.on_token(),
                action.update_token(),
                "update",
                action.ref_action(),
            ));
    }
    append_constraint_options(doc, constraint.constraint_options())
        .nest(2)
        .group()
}

fn build_foreign_key_column_list<'a>(list: ast::ForeignKeyColumnList) -> Doc<'a> {
    let suffix = list.period_column().map(|period| {
        let mut doc = Doc::space()
            .append(leading_comments(period.syntax()))
            .append(Doc::text("period"));
        if let Some(name) = period.name() {
            doc = doc
                .append(Doc::space())
                .append(leading_comments(name.syntax()))
                .append(build_name(name.syntax()));
        }
        doc
    });
    build_column_names(
        list.l_paren_token(),
        list.column_name_refs(),
        suffix,
        list.r_paren_token(),
    )
}

fn build_reference_action<'a>(
    on: Option<SyntaxToken>,
    kind_token: Option<SyntaxToken>,
    kind: &'static str,
    action: Option<ast::RefAction>,
) -> Doc<'a> {
    let mut doc = on
        .map(|token| leading_comments_token(&token).append(Doc::text("on")))
        .unwrap_or_else(Doc::nil);
    if let Some(token) = kind_token {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&token))
            .append(Doc::text(kind));
    }
    let Some(action) = action else {
        return doc;
    };
    doc = doc
        .append(Doc::space())
        .append(leading_comments(action.syntax()));
    match action {
        ast::RefAction::SetNullColumns(action) => {
            if let Some(set) = action.set_token() {
                doc = doc
                    .append(leading_comments_token(&set))
                    .append(Doc::text("set"));
            }
            if let Some(null) = action.null_token() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments_token(&null))
                    .append(Doc::text("null"));
            }
            if let Some(columns) = action.column_ref_list() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments(columns.syntax()))
                    .append(build_column_ref_list(columns));
            }
            doc
        }
        ast::RefAction::SetDefaultColumns(action) => {
            if let Some(set) = action.set_token() {
                doc = doc
                    .append(leading_comments_token(&set))
                    .append(Doc::text("set"));
            }
            if let Some(default) = action.default_token() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments_token(&default))
                    .append(Doc::text("default"));
            }
            if let Some(columns) = action.column_ref_list() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments(columns.syntax()))
                    .append(build_column_ref_list(columns));
            }
            doc
        }
        action => doc.append(build_keyword_node(action.syntax())),
    }
}

fn build_exclude_constraint<'a>(constraint: ast::ExcludeConstraint) -> Doc<'a> {
    let mut doc = build_constraint_name_clause(constraint.constraint_name_clause());
    if let Some(exclude) = constraint.exclude_token() {
        doc = doc
            .append(leading_comments_token(&exclude))
            .append(Doc::text("exclude"));
    }
    if let Some(method) = constraint.constraint_index_method() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(method.syntax()))
            .append(Doc::text("using"));
        if let Some(name) = method.access_method_ref() {
            doc = doc
                .append(Doc::space())
                .append(leading_comments(name.syntax()))
                .append(build_name(name.syntax()));
        }
    }
    if let Some(list) = constraint.constraint_exclusion_list() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(list.syntax()))
            .append(build_constraint_exclusion_list(list));
    }
    if let Some(include) = constraint.constraint_include_clause() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(include.syntax()))
            .append(build_constraint_include_clause(include));
    }
    if let Some(with_params) = constraint.with_params() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(with_params.syntax()))
            .append(build_with_params(with_params));
    }
    if let Some(tablespace) = constraint.constraint_index_tablespace() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(tablespace.syntax()))
            .append(build_constraint_index_tablespace(tablespace));
    }
    if let Some(where_clause) = constraint.where_condition_clause() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(where_clause.syntax()))
            .append(build_where_condition_clause(where_clause));
    }
    append_constraint_options(doc, constraint.constraint_options())
        .nest(2)
        .group()
}

fn build_constraint_exclusion_list<'a>(list: ast::ConstraintExclusionList) -> Doc<'a> {
    let doc = list
        .l_paren_token()
        .map(comments_before)
        .unwrap_or_else(Doc::nil)
        .append(Doc::text("("));
    let items = list.constraint_exclusions().map(|exclusion| {
        let mut item = exclusion.expr().map(build_expr).unwrap_or_else(Doc::nil);
        if let Some(with) = exclusion.with_token() {
            item = item
                .append(Doc::line_or_space())
                .append(leading_comments_token(&with))
                .append(Doc::text("with"));
        }
        if let Some(op) = exclusion.op() {
            item = item
                .append(Doc::space())
                .append(leading_comments(op.syntax()))
                .append(build_operator(&op));
        } else if let Some(op) = exclusion.operator_call() {
            item = item
                .append(Doc::space())
                .append(leading_comments(op.syntax()))
                .append(build_operator_call(&op));
        }
        (
            leading_comments(exclusion.syntax()).append(item.nest(2).group()),
            exclusion.syntax().clone(),
        )
    });
    let mut body = build_comma_separated_docs(items).unwrap_or_else(Doc::nil);
    if let Some(r_paren) = list.r_paren_token() {
        body = body.append(comments_before(r_paren));
    }
    doc.append(wrap_body(body)).append(Doc::text(")")).group()
}

fn build_where_condition_clause<'a>(where_clause: ast::WhereConditionClause) -> Doc<'a> {
    let mut doc = Doc::text("where");
    if let Some(l_paren) = where_clause.l_paren_token() {
        if comment_tokens_before(l_paren.clone()).is_empty() {
            doc = doc.append(Doc::space());
        } else {
            doc = doc.append(comments_before(l_paren));
        }
    }
    doc = doc.append(Doc::text("("));

    let mut body = Doc::nil();
    if let Some(expr) = where_clause.expr() {
        body = body
            .append(leading_comments(expr.syntax()))
            .append(build_expr(expr));
    }
    if let Some(r_paren) = where_clause.r_paren_token() {
        body = body.append(comments_before(r_paren));
    }
    doc.append(wrap_body(body)).append(Doc::text(")")).group()
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

fn build_values<'a>(values: &ast::Values) -> Doc<'a> {
    let mut doc = Doc::nil();
    if let Some(with_clause) = values.with_clause() {
        doc = doc
            .append(leading_comments(with_clause.syntax()))
            .append(build_with_clause(with_clause))
            .append(Doc::hard_line());
        if let Some(values_token) = values.values_token() {
            doc = doc.append(leading_comments_token(&values_token));
        }
    }

    let mut values_doc = Doc::text("values");
    if let Some(row_list) = values.row_list() {
        let rows = row_list.rows().map(|row| {
            (
                leading_comments(row.syntax()).append(build_row(row.clone())),
                row.syntax().clone(),
            )
        });
        if let Some(rows) = build_comma_separated_docs(rows) {
            values_doc = values_doc.append(
                Doc::space()
                    .append(leading_comments(row_list.syntax()))
                    .append(rows),
            );
        }
    }
    doc = doc.append(values_doc.group());

    if let Some(order_by) = values.order_by_clause() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(order_by.syntax()))
            .append(build_order_by_clause(order_by));
    }
    for locking in values.locking_clauses() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(locking.syntax()))
            .append(build_locking_clause(locking));
    }
    if let Some(limit) = values.limit_clause() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(limit.syntax()))
            .append(build_limit_clause(limit));
    }
    if let Some(fetch) = values.fetch_clause() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(fetch.syntax()))
            .append(build_fetch_clause(fetch));
    }
    if let Some(offset) = values.offset_clause() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(offset.syntax()))
            .append(build_offset_clause(offset));
    }

    doc.append(build_semicolon(values.semicolon_token()))
        .group()
}

fn build_row<'a>(row: ast::Row) -> Doc<'a> {
    let mut doc = row
        .l_paren_token()
        .map(comments_before)
        .unwrap_or_else(Doc::nil)
        .append(Doc::text("("));
    let exprs = build_comma_separated_exprs(row.exprs());
    let has_exprs = exprs.is_some();
    let mut body = exprs.unwrap_or_else(Doc::nil);
    if !has_exprs {
        if let Some(r_paren) = row.r_paren_token() {
            body = body.append(comments_before(r_paren));
        }
    }
    doc = doc.append(wrap_body(body)).append(Doc::text(")")).group();
    doc
}

fn build_table<'a>(table: &ast::Table) -> Doc<'a> {
    let mut doc = Doc::nil();
    if let Some(with_clause) = table.with_clause() {
        doc = doc
            .append(leading_comments(with_clause.syntax()))
            .append(build_with_clause(with_clause))
            .append(Doc::hard_line());
        if let Some(table_token) = table.table_token() {
            doc = doc.append(leading_comments_token(&table_token));
        }
    }

    let mut table_doc = Doc::text("table");
    if let Some(relation) = table.relation_name() {
        table_doc = table_doc.append(
            Doc::line_or_space()
                .append(leading_comments(relation.syntax()))
                .append(build_relation_name(relation))
                .nest(2),
        );
    }
    doc = doc.append(table_doc.group());

    if let Some(order_by) = table.order_by_clause() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(order_by.syntax()))
            .append(build_order_by_clause(order_by));
    }
    for locking in table.locking_clauses() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(locking.syntax()))
            .append(build_locking_clause(locking));
    }
    if let Some(limit) = table.limit_clause() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(limit.syntax()))
            .append(build_limit_clause(limit));
    }
    if let Some(fetch) = table.fetch_clause() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(fetch.syntax()))
            .append(build_fetch_clause(fetch));
    }
    if let Some(offset) = table.offset_clause() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(offset.syntax()))
            .append(build_offset_clause(offset));
    }

    doc.append(build_semicolon(table.semicolon_token())).group()
}

fn build_relation_name<'a>(relation: ast::RelationName) -> Doc<'a> {
    let mut doc = Doc::nil();
    let has_only = relation.only_token().is_some();
    if let Some(only) = relation.only_token() {
        doc = doc
            .append(leading_comments_token(&only))
            .append(Doc::text("only"));
    }
    if let Some(l_paren) = relation.l_paren_token() {
        if has_only && comment_tokens_before(l_paren.clone()).is_empty() {
            doc = doc.append(Doc::space());
        }
        doc = doc.append(comments_before(l_paren));
        doc = doc.append(Doc::text("("));
    }
    if let Some(name) = relation.relation_name_ref() {
        if has_only && relation.l_paren_token().is_none() {
            doc = doc.append(Doc::space());
        }
        doc = doc.append(leading_comments(name.syntax()));
        if let Some(path) = name.path_ref() {
            doc = doc.append(build_path_ref(&path));
        }
    }
    if let Some(r_paren) = relation.r_paren_token() {
        doc = doc.append(comments_before(r_paren));
        doc = doc.append(Doc::text(")"));
    }
    if let Some(star) = relation.star_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&star))
            .append(Doc::text("*"));
    }
    doc
}

fn build_select_into<'a>(select_into: &ast::SelectInto) -> Doc<'a> {
    let mut select_body = Doc::nil();
    if let Some(select_clause) = select_into.select_clause() {
        match select_clause.select_quantifier() {
            Some(ast::SelectQuantifier::DistinctClause(distinct_clause)) => {
                select_body = select_body
                    .append(leading_comments(distinct_clause.syntax()))
                    .append(Doc::text("distinct"));
                if let Some(distinct_on) = distinct_clause.distinct_on() {
                    select_body = select_body
                        .append(Doc::space())
                        .append(leading_comments(distinct_on.syntax()))
                        .append(build_distinct_on(distinct_on));
                }
                select_body = select_body.append(Doc::space());
            }
            Some(ast::SelectQuantifier::All(all)) => {
                select_body = select_body
                    .append(leading_comments(all.syntax()))
                    .append(Doc::text("all"))
                    .append(Doc::space());
            }
            None => (),
        }
        if let Some(target_list) = select_clause.target_list() {
            select_body = select_body
                .append(leading_comments(target_list.syntax()))
                .append(Doc::list(
                    Itertools::intersperse(
                        target_list.targets().flat_map(build_target),
                        Doc::text(",").append(Doc::line_or_space()),
                    )
                    .collect(),
                ));
        }
    }
    let mut doc = Doc::nil();
    if let Some(with_clause) = select_into.with_clause() {
        doc = doc
            .append(leading_comments(with_clause.syntax()))
            .append(build_with_clause(with_clause))
            .append(Doc::hard_line());
    }
    if select_into.with_clause().is_some() {
        if let Some(select_clause) = select_into.select_clause() {
            doc = doc.append(leading_comments(select_clause.syntax()));
        }
    }
    doc = doc.append(
        Doc::text("select")
            .append(Doc::line_or_space().append(select_body).nest(2))
            .group(),
    );

    if let Some(into) = select_into.into_clause() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(into.syntax()))
            .append(build_into_clause(into));
    }
    if let Some(from) = select_into.from_clause() {
        doc = doc
            .group()
            .append(Doc::line_or_space())
            .append(leading_comments(from.syntax()))
            .append(build_from_clause(from));
    }
    if let Some(where_clause) = select_into.where_clause() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(where_clause.syntax()))
            .append(build_where_clause(where_clause));
    }
    if let Some(group) = select_into.group_by_clause() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(group.syntax()))
            .append(build_select_group_by_clause(group));
    }
    if let Some(having) = select_into.having_clause() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(having.syntax()))
            .append(build_having_clause(having));
    }
    if let Some(window) = select_into.window_clause() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(window.syntax()))
            .append(build_window_clause(window));
    }
    if let Some(order_by) = select_into.order_by_clause() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(order_by.syntax()))
            .append(build_order_by_clause(order_by));
    }
    for locking in select_into.locking_clauses() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(locking.syntax()))
            .append(build_locking_clause(locking));
    }
    if let Some(limit) = select_into.limit_clause() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(limit.syntax()))
            .append(build_limit_clause(limit));
    }
    if let Some(offset) = select_into.offset_clause() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(offset.syntax()))
            .append(build_offset_clause(offset));
    }
    if let Some(filter) = select_into.filter_clause() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(filter.syntax()))
            .append(build_filter_clause(filter));
    }
    doc.append(build_semicolon(select_into.semicolon_token()))
        .group()
}

fn build_with_clause<'a>(with_clause: ast::WithClause) -> Doc<'a> {
    let mut doc = Doc::text("with");
    if let Some(recursive) = with_clause.recursive_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&recursive))
            .append(Doc::text("recursive"));
    }
    let tables = with_clause.with_tables().map(|table| {
        (
            leading_comments(table.syntax()).append(build_with_table(table.clone())),
            table.syntax().clone(),
        )
    });
    if let Some(tables) = build_comma_separated_docs(tables) {
        doc = doc.append(Doc::space()).append(tables);
    }
    doc
}

fn build_with_table<'a>(table: ast::WithTable) -> Doc<'a> {
    let mut doc = table
        .name()
        .map(|name| build_name(name.syntax()))
        .unwrap_or_else(Doc::nil);
    if let Some(columns) = table.column_list() {
        doc = doc
            .append(leading_comments(columns.syntax()))
            .append(build_cte_column_list(columns));
    }
    if let Some(as_token) = table.as_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&as_token))
            .append(Doc::text("as"));
    }
    if let Some(materialized) = table.materialized_option() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(materialized.syntax()))
            .append(match materialized {
                ast::MaterializedOption::Materialized(_) => Doc::text("materialized"),
                ast::MaterializedOption::NotMaterialized(not_materialized) => Doc::text("not")
                    .append(Doc::space())
                    .append(
                        not_materialized
                            .materialized_token()
                            .map(|token| leading_comments_token(&token))
                            .unwrap_or_else(Doc::nil),
                    )
                    .append(Doc::text("materialized")),
            });
    }
    if let Some(l_paren) = table.l_paren_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&l_paren))
            .append(Doc::text("("));
    }
    let mut body = table
        .query()
        .map(|query| leading_comments(query.syntax()).append(build_with_query(query)))
        .unwrap_or_else(Doc::nil);
    if let Some(r_paren) = table.r_paren_token() {
        body = body.append(comments_before(r_paren));
    }
    doc = doc
        .append(Doc::hard_line().append(body).nest(2))
        .append(Doc::hard_line())
        .append(Doc::text(")"));
    if let Some(search) = table.search_clause() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(search.syntax()))
            .append(build_search_clause(search));
    }
    if let Some(cycle) = table.cycle_clause() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(cycle.syntax()))
            .append(build_cycle_clause(cycle));
    }
    doc.group()
}

fn build_search_clause<'a>(search: ast::SearchClause) -> Doc<'a> {
    let mut doc = Doc::text("search");
    if let Some(order) = search.search_order() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(order.syntax()))
            .append(match order {
                ast::SearchOrder::BreadthFirst(first) => Doc::text("breadth")
                    .append(Doc::space())
                    .append(
                        first
                            .first_token()
                            .map(|token| leading_comments_token(&token))
                            .unwrap_or_else(Doc::nil),
                    )
                    .append(Doc::text("first")),
                ast::SearchOrder::DepthFirst(first) => Doc::text("depth")
                    .append(Doc::space())
                    .append(
                        first
                            .first_token()
                            .map(|token| leading_comments_token(&token))
                            .unwrap_or_else(Doc::nil),
                    )
                    .append(Doc::text("first")),
            });
    }
    if let Some(by_token) = search.by_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&by_token))
            .append(Doc::text("by"));
    }
    if let Some(columns) = search.columns() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(columns.syntax()))
            .append(build_column_name_refs(columns.column_name_refs()));
    }
    if let Some(set_column) = search.set_column() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(set_column.syntax()))
            .append(build_search_set_column(set_column));
    }
    doc.group()
}

fn build_search_set_column<'a>(set_column: ast::SearchSetColumn) -> Doc<'a> {
    let mut doc = Doc::text("set");
    if let Some(column) = set_column.column_name_ref() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(column.syntax()))
            .append(build_name(column.syntax()));
    }
    doc
}

fn build_cycle_clause<'a>(cycle: ast::CycleClause) -> Doc<'a> {
    let mut doc = Doc::text("cycle");
    if let Some(columns) = cycle.columns() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(columns.syntax()))
            .append(build_column_name_refs(columns.column_name_refs()));
    }
    if let Some(set_column) = cycle.set_column() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(set_column.syntax()))
            .append(build_cycle_set_column(set_column));
    }
    if let Some(path) = cycle.path() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(path.syntax()))
            .append(build_cycle_path(path));
    }
    doc.group()
}

fn build_cycle_set_column<'a>(set_column: ast::CycleSetColumn) -> Doc<'a> {
    let mut doc = Doc::text("set");
    if let Some(column) = set_column.column_name_ref() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(column.syntax()))
            .append(build_name(column.syntax()));
    }
    if let Some(column_to) = set_column.column_to() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(column_to.syntax()))
            .append(build_cycle_column_to(column_to));
    }
    doc
}

fn build_cycle_column_to<'a>(column_to: ast::CycleColumnTo) -> Doc<'a> {
    let mut doc = Doc::text("to");
    if let Some(expr) = column_to.expr() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(expr.syntax()))
            .append(build_expr(expr));
    }
    if let Some(default) = column_to.default() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(default.syntax()))
            .append(Doc::text("default"));
        if let Some(expr) = default.expr() {
            doc = doc
                .append(Doc::space())
                .append(leading_comments(expr.syntax()))
                .append(build_expr(expr));
        }
    }
    doc
}

fn build_cycle_path<'a>(path: ast::CyclePath) -> Doc<'a> {
    let mut doc = Doc::text("using");
    if let Some(column) = path.column_name_ref() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(column.syntax()))
            .append(build_name(column.syntax()));
    }
    doc
}

fn build_column_name_refs<'a>(columns: impl Iterator<Item = ast::ColumnNameRef>) -> Doc<'a> {
    let columns = columns.map(|column| {
        (
            leading_comments(column.syntax()).append(build_name(column.syntax())),
            column.syntax().clone(),
        )
    });
    build_comma_separated_docs(columns).unwrap_or_else(Doc::nil)
}

fn build_cte_column_list<'a>(columns: ast::ColumnList) -> Doc<'a> {
    let mut doc = columns
        .l_paren_token()
        .map(comments_before)
        .unwrap_or_else(Doc::nil)
        .append(Doc::text("("));
    let names = columns.column_names().map(|name| {
        (
            leading_comments(name.syntax()).append(build_name(name.syntax())),
            name.syntax().clone(),
        )
    });
    let mut body = build_comma_separated_docs(names).unwrap_or_else(Doc::nil);
    if let Some(r_paren) = columns.r_paren_token() {
        body = body.append(comments_before(r_paren));
    }
    doc = doc.append(wrap_body(body)).append(Doc::text(")")).group();
    doc
}

fn build_with_query<'a>(query: ast::WithQuery) -> Doc<'a> {
    match query {
        ast::WithQuery::CompoundSelect(select) => build_compound_select(&select),
        ast::WithQuery::ParenSelect(select) => build_paren_select(select),
        ast::WithQuery::Select(select) => build_select_doc(&select),
        ast::WithQuery::Table(table) => build_table(&table),
        ast::WithQuery::Values(values) => build_values(&values),
        ast::WithQuery::Delete(delete) => build_delete(&delete),
        ast::WithQuery::Insert(insert) => build_insert(&insert),
        ast::WithQuery::Merge(merge) => build_merge(&merge),
        ast::WithQuery::Update(update) => build_update(&update),
    }
}

fn build_distinct_on<'a>(distinct_on: ast::DistinctOn) -> Doc<'a> {
    let mut doc = Doc::text("on");
    if let Some(l_paren) = distinct_on.l_paren_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&l_paren))
            .append(Doc::text("("));
    }
    let exprs: Vec<_> = distinct_on.exprs().collect();
    let has_exprs = !exprs.is_empty();
    let mut body = build_comma_separated_exprs(exprs.into_iter()).unwrap_or_else(Doc::nil);
    if !has_exprs {
        if let Some(r_paren) = distinct_on.r_paren_token() {
            body = body.append(comments_before(r_paren));
        }
    }
    doc.append(wrap_body(body)).append(Doc::text(")")).group()
}

fn build_having_clause<'a>(having: ast::HavingClause) -> Doc<'a> {
    let mut doc = Doc::text("having");
    if let Some(expr) = having.expr() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(expr.syntax()))
            .append(build_expr(expr));
    }
    doc
}

fn build_window_clause<'a>(window: ast::WindowClause) -> Doc<'a> {
    let defs = window.window_defs().map(|def| {
        (
            leading_comments(def.syntax()).append(build_window_def(def.clone())),
            def.syntax().clone(),
        )
    });
    let mut doc = Doc::text("window");
    if let Some(defs) = build_comma_separated_docs(defs) {
        doc = doc.append(Doc::space()).append(defs.nest(2));
    }
    doc.group()
}

fn build_window_def<'a>(def: ast::WindowDef) -> Doc<'a> {
    let mut doc = def
        .window()
        .map(|window| build_name(window.syntax()))
        .unwrap_or_else(Doc::nil);
    if let Some(as_token) = def.as_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&as_token))
            .append(Doc::text("as"));
    }
    if let Some(l_paren) = def.l_paren_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&l_paren))
            .append(Doc::text("("));
    }
    let mut body = def
        .window_spec()
        .map(|spec| leading_comments(spec.syntax()).append(build_window_spec(spec)))
        .unwrap_or_else(Doc::nil);
    if let Some(r_paren) = def.r_paren_token() {
        body = body.append(comments_before(r_paren));
    }
    doc.append(wrap_body(body)).append(Doc::text(")")).group()
}

fn build_into_clause<'a>(into: ast::IntoClause) -> Doc<'a> {
    let mut doc = Doc::text("into");
    if let Some(persistence) = into.persistence() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(persistence.syntax()))
            .append(build_keyword_node(persistence.syntax()));
    }
    if let Some(table_token) = into.table_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&table_token))
            .append(Doc::text("table"));
    }
    if let Some(table_name) = into.table_name() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(table_name.syntax()));
        if let Some(path) = table_name.path() {
            doc = doc.append(build_path(&path));
        }
    }
    doc
}

fn build_select_group_by_clause<'a>(group: ast::GroupByClause) -> Doc<'a> {
    let mut doc = Doc::text("group").append(Doc::space());
    if let Some(by_token) = group.by_token() {
        doc = doc.append(leading_comments_token(&by_token));
    }
    doc = doc.append(Doc::text("by")).append(Doc::space());
    if let Some(quantifier) = group.all_or_distinct() {
        doc = doc
            .append(leading_comments(quantifier.syntax()))
            .append(match quantifier {
                ast::AllOrDistinct::All(_) => Doc::text("all"),
                ast::AllOrDistinct::Distinct(_) => Doc::text("distinct"),
            })
            .append(Doc::space());
    }
    if let Some(list) = group.group_by_list() {
        doc = doc.append(build_group_by_list(list));
    }
    doc
}

fn build_create_publication<'a>(stmt: &ast::CreatePublication) -> Doc<'a> {
    let mut doc = Doc::text("create");
    if let Some(token) = stmt.publication_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&token))
            .append(Doc::text("publication"));
    }
    if let Some(publication) = stmt.publication() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(publication.syntax()))
            .append(build_name(publication.syntax()));
    }
    if let Some(clause) = stmt.publication_for_clause() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(clause.syntax()))
            .append(build_publication_for_clause(clause));
    }
    if let Some(params) = stmt.with_params() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(params.syntax()))
            .append(build_with_params(params));
    }
    doc.append(build_semicolon(stmt.semicolon_token())).group()
}

fn build_publication_for_clause<'a>(clause: ast::PublicationForClause) -> Doc<'a> {
    match clause {
        ast::PublicationForClause::ForAllPublicationObjects(clause) => {
            let mut doc = Doc::text("for");
            if let Some(objects) = build_all_publication_objects(clause.all_publication_objects()) {
                doc = doc.append(Doc::line_or_space().append(objects).nest(2));
            }
            if let Some(except) = clause.except_table_clause() {
                doc = doc
                    .append(Doc::line_or_space())
                    .append(leading_comments(except.syntax()))
                    .append(build_except_table_clause(except));
            }
            doc.group()
        }
        ast::PublicationForClause::ForPublicationObjects(clause) => {
            let mut doc = Doc::text("for");
            if let Some(objects) = build_publication_objects(clause.publication_objects()) {
                doc = doc.append(Doc::line_or_space().append(objects).nest(2));
            }
            doc.group()
        }
    }
}

fn build_all_publication_objects<'a>(
    objects: impl Iterator<Item = ast::AllPublicationObject>,
) -> Option<Doc<'a>> {
    build_comma_separated_docs(objects.map(|object| {
        let syntax = object.syntax().clone();
        let doc = leading_comments(object.syntax()).append(match object {
            ast::AllPublicationObject::AllPublicationTables(object) => {
                let mut doc = Doc::text("all");
                if let Some(token) = object.tables_token() {
                    doc = doc
                        .append(Doc::space())
                        .append(leading_comments_token(&token))
                        .append(Doc::text("tables"));
                }
                doc
            }
            ast::AllPublicationObject::AllPublicationSequences(object) => {
                let mut doc = Doc::text("all");
                if let Some(token) = object.sequences_token() {
                    doc = doc
                        .append(Doc::space())
                        .append(leading_comments_token(&token))
                        .append(Doc::text("sequences"));
                }
                doc
            }
        });
        (doc, syntax)
    }))
}

fn build_publication_objects<'a>(
    objects: impl Iterator<Item = ast::PublicationObject>,
) -> Option<Doc<'a>> {
    build_comma_separated_docs(objects.map(|object| {
        let syntax = object.syntax().clone();
        (
            leading_comments(object.syntax()).append(build_publication_object(object)),
            syntax,
        )
    }))
}

fn build_publication_object<'a>(object: ast::PublicationObject) -> Doc<'a> {
    match object {
        ast::PublicationObject::PublicationObjectCurrentSchema(_) => Doc::text("current_schema"),
        ast::PublicationObject::PublicationObjectTable(object) => {
            let mut doc = Doc::text("table");
            if let Some(token) = object.only_token() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments_token(&token))
                    .append(Doc::text("only"));
            }
            let parenthesized = object.l_paren_token().is_some();
            if let Some(l_paren) = object.l_paren_token() {
                doc = doc
                    .append(Doc::space())
                    .append(comments_before(l_paren))
                    .append(Doc::text("("));
            }
            if let Some(table) = object.table_name_ref() {
                if !parenthesized {
                    doc = doc.append(Doc::space());
                }
                doc = doc.append(leading_comments(table.syntax()));
                if let Some(path) = table.path_ref() {
                    doc = doc.append(build_path_ref(&path));
                }
            }
            if let Some(r_paren) = object.r_paren_token() {
                doc = doc.append(comments_before(r_paren)).append(Doc::text(")"));
            }
            if let Some(star) = object.star_token() {
                doc = doc
                    .append(leading_comments_token(&star))
                    .append(Doc::text("*"));
            }
            if let Some(columns) = object.column_ref_list() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments(columns.syntax()))
                    .append(build_column_ref_list(columns));
            }
            if let Some(where_clause) = object.where_condition_clause() {
                doc = doc
                    .append(Doc::line_or_space())
                    .append(leading_comments(where_clause.syntax()))
                    .append(build_where_condition_clause(where_clause));
            }
            doc.group()
        }
        ast::PublicationObject::PublicationObjectTablesInSchema(object) => {
            let mut doc = Doc::text("tables");
            if let Some(token) = object.in_token() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments_token(&token))
                    .append(Doc::text("in"));
            }
            if let Some(token) = object.schema_token() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments_token(&token))
                    .append(Doc::text("schema"));
            }
            if let Some(token) = object.current_schema_token() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments_token(&token))
                    .append(Doc::text("current_schema"));
            } else if let Some(schema) = object.schema_ref() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments(schema.syntax()))
                    .append(build_name(schema.syntax()));
            }
            if let Some(where_clause) = object.where_condition_clause() {
                doc = doc
                    .append(Doc::line_or_space())
                    .append(leading_comments(where_clause.syntax()))
                    .append(build_where_condition_clause(where_clause));
            }
            doc.group()
        }
    }
}

fn build_except_table_clause<'a>(clause: ast::ExceptTableClause) -> Doc<'a> {
    let mut doc = Doc::text("except");
    if let Some(l_paren) = clause.l_paren_token() {
        doc = doc
            .append(Doc::space())
            .append(comments_before(l_paren))
            .append(Doc::text("("));
    }
    let items = clause.except_table_names().map(|name| {
        let mut item = Doc::nil();
        if let Some(table_token) = name.table_token() {
            item = item
                .append(leading_comments_token(&table_token))
                .append(Doc::text("table"))
                .append(Doc::space());
        }
        if let Some(table) = name.table_relation_name() {
            item = item
                .append(leading_comments(table.syntax()))
                .append(build_table_relation_name(table));
        }
        (
            leading_comments(name.syntax()).append(item),
            name.syntax().clone(),
        )
    });
    let mut body = build_comma_separated_docs(items).unwrap_or_else(Doc::nil);
    if let Some(r_paren) = clause.r_paren_token() {
        body = body.append(comments_before(r_paren));
    }
    doc.append(wrap_body(body)).append(Doc::text(")")).group()
}

fn build_alter_publication<'a>(stmt: &ast::AlterPublication) -> Doc<'a> {
    let mut doc = Doc::text("alter");
    if let Some(token) = stmt.publication_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&token))
            .append(Doc::text("publication"));
    }
    if let Some(publication) = stmt.publication_ref() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(publication.syntax()))
            .append(build_name(publication.syntax()));
    }
    if let Some(action) = stmt.action() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(action.syntax()))
            .append(build_alter_publication_action(action));
    }
    doc.append(build_semicolon(stmt.semicolon_token())).group()
}

fn build_alter_publication_action<'a>(action: ast::AlterPublicationAction) -> Doc<'a> {
    match action {
        ast::AlterPublicationAction::AddPublicationObjects(action) => {
            build_publication_object_action("add", action.publication_objects())
        }
        ast::AlterPublicationAction::DropPublicationObjects(action) => {
            build_publication_object_action("drop", action.publication_objects())
        }
        ast::AlterPublicationAction::SetPublicationObjects(action) => {
            build_publication_object_action("set", action.publication_objects())
        }
        ast::AlterPublicationAction::SetAllPublicationObjectList(action) => {
            let mut doc = Doc::text("set");
            if let Some(objects) = build_all_publication_objects(action.all_publication_objects()) {
                doc = doc.append(Doc::line_or_space().append(objects).nest(2));
            }
            if let Some(except) = action.except_table_clause() {
                doc = doc
                    .append(Doc::line_or_space())
                    .append(leading_comments(except.syntax()))
                    .append(build_except_table_clause(except));
            }
            doc.group()
        }
        ast::AlterPublicationAction::SetOptions(action) => build_set_options(action),
        ast::AlterPublicationAction::OwnerTo(action) => build_owner_to(action),
        ast::AlterPublicationAction::PublicationRenameTo(action) => {
            let mut doc = Doc::text("rename");
            if let Some(token) = action.to_token() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments_token(&token))
                    .append(Doc::text("to"));
            }
            if let Some(publication) = action.publication() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments(publication.syntax()))
                    .append(build_name(publication.syntax()));
            }
            doc
        }
    }
}

fn build_publication_object_action<'a>(
    keyword: &'static str,
    objects: impl Iterator<Item = ast::PublicationObject>,
) -> Doc<'a> {
    let mut doc = Doc::text(keyword);
    if let Some(objects) = build_publication_objects(objects) {
        doc = doc.append(Doc::line_or_space().append(objects).nest(2));
    }
    doc.group()
}

fn build_set_options<'a>(options: ast::SetOptions) -> Doc<'a> {
    let mut doc = Doc::text("set");
    if let Some(attributes) = options.attribute_list() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(attributes.syntax()))
            .append(build_attribute_list(attributes));
    }
    doc
}

fn build_owner_to<'a>(owner: ast::OwnerTo) -> Doc<'a> {
    let mut doc = Doc::text("owner");
    if let Some(token) = owner.to_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&token))
            .append(Doc::text("to"));
    }
    if let Some(role) = owner.role_ref() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(role.syntax()))
            .append(build_name(role.syntax()));
    }
    doc
}

fn build_create_subscription<'a>(stmt: &ast::CreateSubscription) -> Doc<'a> {
    let mut doc = Doc::text("create");
    if let Some(token) = stmt.subscription_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&token))
            .append(Doc::text("subscription"));
    }
    if let Some(subscription) = stmt.subscription() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(subscription.syntax()))
            .append(build_name(subscription.syntax()));
    }
    if let Some(source) = stmt.source() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(source.syntax()))
            .append(build_subscription_source(source));
    }
    if let Some(token) = stmt.publication_token() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments_token(&token))
            .append(Doc::text("publication"));
    }
    if let Some(publications) = build_publication_refs(stmt.publication_refs()) {
        doc = doc.append(Doc::space()).append(publications.nest(2));
    }
    if let Some(params) = stmt.with_params() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(params.syntax()))
            .append(build_with_params(params));
    }
    doc.append(build_semicolon(stmt.semicolon_token())).group()
}

fn build_subscription_source<'a>(source: ast::SubscriptionSource) -> Doc<'a> {
    match source {
        ast::SubscriptionSource::ConnectionClause(source) => {
            let mut doc = Doc::text("connection");
            if let Some(literal) = source.literal() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments(literal.syntax()))
                    .append(build_literal(literal));
            }
            doc
        }
        ast::SubscriptionSource::ServerClause(source) => {
            let mut doc = Doc::text("server");
            if let Some(server) = source.server_ref() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments(server.syntax()))
                    .append(build_name(server.syntax()));
            }
            doc
        }
    }
}

fn build_publication_refs<'a>(
    publications: impl Iterator<Item = ast::PublicationRef>,
) -> Option<Doc<'a>> {
    build_comma_separated_docs(publications.map(|publication| {
        (
            leading_comments(publication.syntax()).append(build_name(publication.syntax())),
            publication.syntax().clone(),
        )
    }))
}

fn build_alter_subscription<'a>(stmt: &ast::AlterSubscription) -> Doc<'a> {
    let mut doc = Doc::text("alter");
    if let Some(token) = stmt.subscription_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&token))
            .append(Doc::text("subscription"));
    }
    if let Some(subscription) = stmt.subscription_ref() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(subscription.syntax()))
            .append(build_name(subscription.syntax()));
    }
    if let Some(action) = stmt.action() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(action.syntax()))
            .append(build_alter_subscription_action(action));
    }
    doc.append(build_semicolon(stmt.semicolon_token())).group()
}

fn build_alter_subscription_action<'a>(action: ast::AlterSubscriptionAction) -> Doc<'a> {
    match action {
        ast::AlterSubscriptionAction::ConnectionClause(action) => {
            build_subscription_source(ast::SubscriptionSource::ConnectionClause(action))
        }
        ast::AlterSubscriptionAction::ServerClause(action) => {
            build_subscription_source(ast::SubscriptionSource::ServerClause(action))
        }
        ast::AlterSubscriptionAction::SetOptions(action) => build_set_options(action),
        ast::AlterSubscriptionAction::AddPublication(action) => {
            build_subscription_publication_action(
                "add",
                action.publication_token(),
                action.publication_refs(),
                action.with_params(),
            )
        }
        ast::AlterSubscriptionAction::SetPublication(action) => {
            build_subscription_publication_action(
                "set",
                action.publication_token(),
                action.publication_refs(),
                action.with_params(),
            )
        }
        ast::AlterSubscriptionAction::DropSubscriptionPublication(action) => {
            build_subscription_publication_action(
                "drop",
                action.publication_token(),
                action.publication_refs(),
                action.with_params(),
            )
        }
        ast::AlterSubscriptionAction::RefreshPublication(action) => {
            let mut doc = Doc::text("refresh").append(Doc::space());
            if let Some(token) = action.publication_token() {
                doc = doc
                    .append(leading_comments_token(&token))
                    .append(Doc::text("publication"));
            }
            if let Some(params) = action.with_params() {
                doc = doc
                    .append(Doc::line_or_space())
                    .append(leading_comments(params.syntax()))
                    .append(build_with_params(params));
            }
            doc.group()
        }
        ast::AlterSubscriptionAction::EnableSubscription(_) => Doc::text("enable"),
        ast::AlterSubscriptionAction::DisableSubscription(_) => Doc::text("disable"),
        ast::AlterSubscriptionAction::SkipSubscription(action) => {
            let mut doc = Doc::text("skip");
            if let Some(attributes) = action.attribute_list() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments(attributes.syntax()))
                    .append(build_attribute_list(attributes));
            }
            doc
        }
        ast::AlterSubscriptionAction::OwnerTo(action) => build_owner_to(action),
        ast::AlterSubscriptionAction::SubscriptionRenameTo(action) => {
            let mut doc = Doc::text("rename");
            if let Some(token) = action.to_token() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments_token(&token))
                    .append(Doc::text("to"));
            }
            if let Some(subscription) = action.subscription() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments(subscription.syntax()))
                    .append(build_name(subscription.syntax()));
            }
            doc
        }
    }
}

fn build_subscription_publication_action<'a>(
    keyword: &'static str,
    publication_token: Option<SyntaxToken>,
    publications: impl Iterator<Item = ast::PublicationRef>,
    params: Option<ast::WithParams>,
) -> Doc<'a> {
    let mut doc = Doc::text(keyword).append(Doc::space());
    if let Some(token) = publication_token {
        doc = doc
            .append(leading_comments_token(&token))
            .append(Doc::text("publication"));
    }
    if let Some(publications) = build_publication_refs(publications) {
        doc = doc.append(Doc::space()).append(publications.nest(2));
    }
    if let Some(params) = params {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(params.syntax()))
            .append(build_with_params(params));
    }
    doc.group()
}

fn build_drop_publication<'a>(stmt: &ast::DropPublication) -> Doc<'a> {
    let mut doc = Doc::text("drop");
    if let Some(token) = stmt.publication_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&token))
            .append(Doc::text("publication"));
    }
    if let Some(if_exists) = stmt.if_exists() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(if_exists.syntax()))
            .append(build_if_exists(if_exists));
    }
    if let Some(publications) = build_publication_refs(stmt.publication_refs()) {
        doc = doc
            .append(Doc::line_or_space())
            .append(publications.nest(2));
    }
    if let Some(behavior) = stmt.drop_behavior() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(behavior.syntax()))
            .append(build_drop_behavior(behavior));
    }
    doc.append(build_semicolon(stmt.semicolon_token())).group()
}

fn build_drop_subscription<'a>(stmt: &ast::DropSubscription) -> Doc<'a> {
    let mut doc = Doc::text("drop");
    if let Some(token) = stmt.subscription_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&token))
            .append(Doc::text("subscription"));
    }
    if let Some(if_exists) = stmt.if_exists() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(if_exists.syntax()))
            .append(build_if_exists(if_exists));
    }
    if let Some(subscription) = stmt.subscription_ref() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(subscription.syntax()))
            .append(build_name(subscription.syntax()));
    }
    if let Some(behavior) = stmt.drop_behavior() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(behavior.syntax()))
            .append(build_drop_behavior(behavior));
    }
    doc.append(build_semicolon(stmt.semicolon_token())).group()
}

fn build_if_exists<'a>(if_exists: ast::IfExists) -> Doc<'a> {
    let mut doc = Doc::text("if");
    if let Some(token) = if_exists.exists_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&token))
            .append(Doc::text("exists"));
    }
    doc
}

fn build_drop_behavior<'a>(behavior: ast::DropBehavior) -> Doc<'a> {
    match behavior {
        ast::DropBehavior::Cascade(_) => Doc::text("cascade"),
        ast::DropBehavior::Restrict(_) => Doc::text("restrict"),
    }
}

fn build_select_doc<'a>(select: &ast::Select) -> Doc<'a> {
    build_select_doc_ungrouped(select).group()
}

fn has_single_call_target(select: &ast::Select) -> bool {
    let Some(select_clause) = select.select_clause() else {
        return false;
    };
    if select_clause.select_quantifier().is_some() {
        return false;
    }
    let Some(target_list) = select_clause.target_list() else {
        return false;
    };
    let mut targets = target_list.targets();
    let Some(target) = targets.next() else {
        return false;
    };
    if targets.next().is_some() {
        return false;
    }
    matches!(target.expr(), Some(ast::Expr::CallExpr(_)))
}

fn build_select_doc_ungrouped<'a>(select: &ast::Select) -> Doc<'a> {
    let mut doc = Doc::nil();
    if let Some(with_clause) = select.with_clause() {
        doc = doc
            .append(leading_comments(with_clause.syntax()))
            .append(build_with_clause(with_clause))
            .append(Doc::hard_line());
        if let Some(select_clause) = select.select_clause() {
            doc = doc.append(leading_comments(select_clause.syntax()));
        }
    }
    let mut select_doc = Doc::text("select");
    let mut select_body = Doc::nil();
    if let Some(select_clause) = select.select_clause() {
        match select_clause.select_quantifier() {
            Some(ast::SelectQuantifier::DistinctClause(distinct_clause)) => {
                select_body = select_body
                    .append(leading_comments(distinct_clause.syntax()))
                    .append(Doc::text("distinct"));
                if let Some(distinct_on) = distinct_clause.distinct_on() {
                    select_body = select_body
                        .append(Doc::space())
                        .append(leading_comments(distinct_on.syntax()))
                        .append(build_distinct_on(distinct_on));
                }
                select_body = select_body.append(Doc::space());
            }
            Some(ast::SelectQuantifier::All(all)) => {
                select_body = select_body
                    .append(leading_comments(all.syntax()))
                    .append(Doc::text("all"))
                    .append(Doc::space());
            }
            None => (),
        }
        if let Some(target_list) = select_clause.target_list() {
            select_body = select_body
                .append(leading_comments(target_list.syntax()))
                .append(Doc::list(
                    Itertools::intersperse(
                        target_list.targets().flat_map(build_target),
                        Doc::text(",").append(Doc::line_or_space()),
                    )
                    .collect(),
                ));
        }
    }
    let has_distinct_on = matches!(
        select
            .select_clause()
            .and_then(|clause| clause.select_quantifier()),
        Some(ast::SelectQuantifier::DistinctClause(clause))
            if clause.distinct_on().is_some()
    );
    select_doc = if has_single_call_target(select) {
        select_doc.append(Doc::space()).append(select_body)
    } else if has_distinct_on {
        select_doc.append(Doc::space()).append(select_body.nest(2))
    } else {
        select_doc.append(Doc::line_or_space().append(select_body).nest(2))
    };
    doc = if select.with_clause().is_some() {
        doc.append(select_doc.group())
    } else {
        doc.append(select_doc)
    };
    if select.from_clause().is_some() {
        doc = doc.group();
    }

    if let Some(from) = select.from_clause() {
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(from.syntax()))
                .append(build_from_clause(from)),
        );
    }

    if let Some(where_clause) = select.where_clause() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(where_clause.syntax()))
            .append(build_where_clause(where_clause));
    }
    if let Some(group) = select.group_by_clause() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(group.syntax()))
            .append(build_select_group_by_clause(group));
    }
    if let Some(having) = select.having_clause() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(having.syntax()))
            .append(build_having_clause(having));
    }
    if let Some(window) = select.window_clause() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(window.syntax()))
            .append(build_window_clause(window));
    }
    if let Some(order_by) = select.order_by_clause() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(order_by.syntax()))
            .append(build_order_by_clause(order_by));
    }
    for locking in select.locking_clauses() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(locking.syntax()))
            .append(build_locking_clause(locking));
    }
    if let Some(limit) = select.limit_clause() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(limit.syntax()))
            .append(build_limit_clause(limit));
    }
    if let Some(fetch) = select.fetch_clause() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(fetch.syntax()))
            .append(build_fetch_clause(fetch));
    }
    if let Some(offset) = select.offset_clause() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(offset.syntax()))
            .append(build_offset_clause(offset));
    }
    if let Some(filter) = select.filter_clause() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(filter.syntax()))
            .append(build_filter_clause(filter));
    }

    doc = doc.append(build_semicolon(select.semicolon_token()));

    doc
}

fn build_from_clause<'a>(from: ast::FromClause) -> Doc<'a> {
    let from_items = from.from_items().map(|item| {
        let syntax = item.syntax().clone();
        (
            leading_comments(item.syntax()).append(build_from_item(item)),
            syntax,
        )
    });
    let join_exprs = from.join_exprs().map(|join_expr| {
        let syntax = join_expr.syntax().clone();
        (
            leading_comments(join_expr.syntax()).append(build_join_expr(join_expr)),
            syntax,
        )
    });
    let body = build_comma_separated_docs(from_items.chain(join_exprs)).unwrap_or_else(Doc::nil);

    Doc::text("from").append(Doc::space()).append(body.nest(2))
}

fn build_join_expr<'a>(join_expr: ast::JoinExpr) -> Doc<'a> {
    let mut doc = if let Some(left) = join_expr.join_expr() {
        leading_comments(left.syntax()).append(build_join_expr(left))
    } else if let Some(left) = join_expr.from_item() {
        leading_comments(left.syntax()).append(build_from_item(left))
    } else {
        Doc::nil()
    };

    if let Some(join) = join_expr.join() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(join.syntax()))
            .append(build_join(join));
    }
    doc.group()
}

fn build_join<'a>(join: ast::Join) -> Doc<'a> {
    let mut doc = Doc::nil();
    if let Some(natural) = join.natural_token() {
        doc = doc
            .append(leading_comments_token(&natural))
            .append(Doc::text("natural"))
            .append(Doc::space());
    }
    if let Some(join_type) = join.join_type() {
        doc = doc
            .append(leading_comments(join_type.syntax()))
            .append(build_keyword_node(join_type.syntax()));
    }
    if let Some(item) = join.from_item() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(item.syntax()))
            .append(build_from_item(item));
    }
    if let Some(condition) = join.join_condition() {
        match condition {
            ast::JoinCondition::OnClause(on_clause) => {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments(on_clause.syntax()))
                    .append(build_join_on_clause(on_clause));
            }
            ast::JoinCondition::JoinUsingClause(using) => {
                let condition_doc =
                    leading_comments(using.syntax()).append(build_join_using_clause(using));
                doc = doc.append(Doc::line_or_space().append(condition_doc).nest(2));
            }
        }
    }
    doc.group()
}

fn build_join_on_clause<'a>(on_clause: ast::OnClause) -> Doc<'a> {
    let mut doc = Doc::text("on");
    if let Some(expr) = on_clause.expr() {
        let expr_doc = leading_comments(expr.syntax()).append(build_expr(expr));
        doc = doc.append(Doc::line_or_space().append(expr_doc).nest(2));
    }
    doc
}

fn build_join_using_clause<'a>(using: ast::JoinUsingClause) -> Doc<'a> {
    let mut doc = Doc::text("using");
    if let Some(columns) = using.column_ref_list() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(columns.syntax()))
            .append(build_column_ref_list(columns));
    }
    if let Some(alias) = using.alias() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(alias.syntax()))
            .append(build_required_as_alias(alias));
    }
    doc
}

fn build_from_item<'a>(item: ast::FromItem) -> Doc<'a> {
    match item {
        ast::FromItem::RelationFromItem(relation) => build_relation_from_item(relation),
        ast::FromItem::FunctionFromItem(function) => build_function_from_item(function),
        ast::FromItem::ExprFromItem(expr) => build_expr_from_item(expr),
        ast::FromItem::ParenFromItem(paren) => build_paren_from_item(paren),
        ast::FromItem::RowsFromItem(rows) => build_rows_from_item(rows),
        ast::FromItem::GraphTableFromItem(graph_table) => build_graph_table_from_item(graph_table),
        ast::FromItem::JsonTableFromItem(json_table) => build_json_table_from_item(json_table),
        ast::FromItem::XmlTableFromItem(xml_table) => build_xml_table_from_item(xml_table),
    }
}

fn build_function_from_item<'a>(item: ast::FunctionFromItem) -> Doc<'a> {
    let mut doc = Doc::nil();
    if let Some(lateral) = item.lateral_token() {
        doc = doc
            .append(leading_comments_token(&lateral))
            .append(Doc::text("lateral"))
            .append(Doc::space());
    }
    if let Some(call) = item.call_expr() {
        doc = doc
            .append(leading_comments(call.syntax()))
            .append(build_call_expr(call));
    }
    if let Some(ordinality) = item.with_ordinality() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(ordinality.syntax()))
            .append(build_with_ordinality(ordinality));
    }
    doc.append(build_from_alias(item.alias()))
}

fn build_with_ordinality<'a>(ordinality: ast::WithOrdinality) -> Doc<'a> {
    let mut doc = Doc::text("with");
    if let Some(ordinality_token) = ordinality.ordinality_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&ordinality_token))
            .append(Doc::text("ordinality"));
    }
    doc
}

fn build_expr_from_item<'a>(item: ast::ExprFromItem) -> Doc<'a> {
    let mut doc = Doc::nil();
    if let Some(lateral) = item.lateral_token() {
        doc = doc
            .append(leading_comments_token(&lateral))
            .append(Doc::text("lateral"))
            .append(Doc::space());
    }
    if let Some(cast) = item.cast_expr() {
        doc = doc
            .append(leading_comments(cast.syntax()))
            .append(build_cast_expr(cast));
    } else if let Some(call) = item.call_expr() {
        doc = doc
            .append(leading_comments(call.syntax()))
            .append(build_call_expr(call));
    }
    doc.append(build_from_alias(item.alias()))
}

fn build_paren_from_item<'a>(item: ast::ParenFromItem) -> Doc<'a> {
    let mut doc = Doc::nil();
    if let Some(only) = item.only_token() {
        doc = doc
            .append(leading_comments_token(&only))
            .append(Doc::text("only"))
            .append(Doc::space());
    }
    if let Some(lateral) = item.lateral_token() {
        doc = doc
            .append(leading_comments_token(&lateral))
            .append(Doc::text("lateral"))
            .append(Doc::space());
    }
    if let Some(select) = item.paren_select() {
        doc = doc
            .append(leading_comments(select.syntax()))
            .append(build_paren_select(select));
    } else if let Some(expr) = item.paren_expr() {
        doc = doc
            .append(leading_comments(expr.syntax()))
            .append(build_paren_expr(expr));
    }
    doc.append(build_from_alias(item.alias()))
}

fn build_select_variant<'a>(select: ast::SelectVariant) -> Doc<'a> {
    match select {
        ast::SelectVariant::CompoundSelect(compound_select) => {
            build_compound_select(&compound_select)
        }
        ast::SelectVariant::ParenSelect(select) => build_paren_select(select),
        ast::SelectVariant::Select(select) => build_select_doc(&select),
        ast::SelectVariant::SelectInto(select_into) => build_select_into(&select_into),
        ast::SelectVariant::Table(table) => build_table(&table),
        ast::SelectVariant::Values(values) => build_values(&values),
    }
}

fn build_compound_select<'a>(select: &ast::CompoundSelect) -> Doc<'a> {
    let mut doc = select
        .lhs()
        .map(build_select_variant)
        .unwrap_or_else(Doc::nil);

    if let Some(op) = select.op() {
        let (syntax, keyword, quantifier) = match op {
            ast::CompoundOp::Union(op) => (op.syntax().clone(), "union", op.all_or_distinct()),
            ast::CompoundOp::Intersect(op) => {
                (op.syntax().clone(), "intersect", op.all_or_distinct())
            }
            ast::CompoundOp::Except(op) => (op.syntax().clone(), "except", op.all_or_distinct()),
        };
        let mut op_doc = leading_comments(&syntax).append(Doc::text(keyword));
        if let Some(quantifier) = quantifier {
            op_doc = op_doc
                .append(Doc::space())
                .append(leading_comments(quantifier.syntax()))
                .append(match quantifier {
                    ast::AllOrDistinct::All(_) => Doc::text("all"),
                    ast::AllOrDistinct::Distinct(_) => Doc::text("distinct"),
                });
        }
        doc = doc.append(Doc::line_or_space()).append(op_doc);
    }

    if let Some(rhs) = select.rhs() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(rhs.syntax()))
            .append(build_select_variant(rhs));
    }
    if let Some(order_by) = select.order_by_clause() {
        doc = doc
            .append(Doc::hard_line())
            .append(leading_comments(order_by.syntax()))
            .append(build_order_by_clause(order_by));
    }
    for locking in select.locking_clauses() {
        doc = doc
            .append(Doc::hard_line())
            .append(leading_comments(locking.syntax()))
            .append(build_locking_clause(locking));
    }
    if let Some(limit) = select.limit_clause() {
        doc = doc
            .append(Doc::hard_line())
            .append(leading_comments(limit.syntax()))
            .append(build_limit_clause(limit));
    }
    if let Some(fetch) = select.fetch_clause() {
        doc = doc
            .append(Doc::hard_line())
            .append(leading_comments(fetch.syntax()))
            .append(build_fetch_clause(fetch));
    }
    if let Some(offset) = select.offset_clause() {
        doc = doc
            .append(Doc::hard_line())
            .append(leading_comments(offset.syntax()))
            .append(build_offset_clause(offset));
    }

    doc.append(build_semicolon(select.semicolon_token()))
        .group()
}

fn build_locking_clause<'a>(locking: ast::LockingClause) -> Doc<'a> {
    let mut doc = Doc::text("for");
    if let Some(strength) = locking.lock_strength() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(strength.syntax()))
            .append(build_keyword_node(strength.syntax()));
    }
    if let Some(of) = locking.locking_of() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(of.syntax()))
            .append(Doc::text("of"));
        if let Some(exprs) = build_comma_separated_exprs(of.exprs()) {
            doc = doc.append(Doc::space()).append(exprs);
        }
    }
    if let Some(wait) = locking.lock_wait() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(wait.syntax()))
            .append(build_keyword_node(wait.syntax()));
    }
    doc.group()
}

fn build_limit_clause<'a>(limit: ast::LimitClause) -> Doc<'a> {
    let mut doc = Doc::text("limit");
    if let Some(value) = limit.limit_value() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(value.syntax()))
            .append(match value {
                ast::LimitValue::All(_) => Doc::text("all"),
                ast::LimitValue::Expr(expr) => build_expr(expr),
            });
    }
    doc
}

fn build_fetch_clause<'a>(fetch: ast::FetchClause) -> Doc<'a> {
    let mut doc = Doc::text("fetch");
    if let Some(token) = fetch.first_token().or_else(|| fetch.next_token()) {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&token))
            .append(Doc::text(token.text().to_ascii_lowercase()));
    }
    if let Some(expr) = fetch.expr() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(expr.syntax()))
            .append(build_expr(expr));
    }
    if let Some(token) = fetch.row_token().or_else(|| fetch.rows_token()) {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&token))
            .append(Doc::text(token.text().to_ascii_lowercase()));
    }
    if let Some(quantity) = fetch.fetch_quantity() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(quantity.syntax()))
            .append(build_keyword_node(quantity.syntax()));
    }
    doc
}

fn build_offset_clause<'a>(offset: ast::OffsetClause) -> Doc<'a> {
    let mut doc = Doc::text("offset");
    if let Some(expr) = offset.expr() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(expr.syntax()))
            .append(build_expr(expr));
    }
    if let Some(token) = offset.row_token().or_else(|| offset.rows_token()) {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&token))
            .append(Doc::text(token.text().to_ascii_lowercase()));
    }
    doc
}

fn build_paren_select<'a>(select: ast::ParenSelect) -> Doc<'a> {
    let mut doc = Doc::nil();
    if let Some(with_clause) = select.with_clause() {
        doc = doc
            .append(leading_comments(with_clause.syntax()))
            .append(build_with_clause(with_clause))
            .append(Doc::hard_line());
    }

    let has_with_clause = select.with_clause().is_some();
    let mut paren_doc = select
        .l_paren_token()
        .map(|token| {
            if has_with_clause {
                leading_comments_token(&token)
            } else {
                comments_before(token)
            }
        })
        .unwrap_or_else(Doc::nil)
        .append(Doc::text("("));
    let mut body = select
        .select()
        .map(|select| leading_comments(select.syntax()).append(build_select_variant(select)))
        .unwrap_or_else(Doc::nil);
    if let Some(r_paren) = select.r_paren_token() {
        body = body.append(comments_before(r_paren));
    }
    paren_doc = paren_doc
        .append(wrap_body(body))
        .append(Doc::text(")"))
        .group();
    doc = doc.append(paren_doc);

    if let Some(order_by) = select.order_by_clause() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(order_by.syntax()))
            .append(build_order_by_clause(order_by));
    }
    for locking in select.locking_clauses() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(locking.syntax()))
            .append(build_locking_clause(locking));
    }
    if let Some(limit) = select.limit_clause() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(limit.syntax()))
            .append(build_limit_clause(limit));
    }
    if let Some(offset) = select.offset_clause() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(offset.syntax()))
            .append(build_offset_clause(offset));
    }
    if let Some(fetch) = select.fetch_clause() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(fetch.syntax()))
            .append(build_fetch_clause(fetch));
    }

    doc.append(build_semicolon(select.semicolon_token()))
        .group()
}

fn build_rows_from_item<'a>(item: ast::RowsFromItem) -> Doc<'a> {
    let mut doc = Doc::nil();
    if let Some(lateral) = item.lateral_token() {
        doc = doc
            .append(leading_comments_token(&lateral))
            .append(Doc::text("lateral"))
            .append(Doc::space());
    }
    if let Some(rows) = item.rows_token() {
        doc = doc
            .append(leading_comments_token(&rows))
            .append(Doc::text("rows"));
    }
    if let Some(from) = item.from_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&from))
            .append(Doc::text("from"));
    }
    if let Some(l_paren) = item.l_paren_token() {
        if comment_tokens_before(l_paren.clone()).is_empty() {
            doc = doc.append(Doc::space());
        } else {
            doc = doc.append(comments_before(l_paren));
        }
    }
    doc = doc.append(Doc::text("("));

    let args = item.rows_from_args().map(|arg| {
        (
            leading_comments(arg.syntax()).append(build_rows_from_arg(arg.clone())),
            arg.syntax().clone(),
        )
    });
    let mut body = build_comma_separated_docs(args).unwrap_or_else(Doc::nil);
    if let Some(r_paren) = item.r_paren_token() {
        body = body.append(comments_before(r_paren));
    }
    doc = doc.append(wrap_body(body)).append(Doc::text(")")).group();

    if let Some(ordinality) = item.with_ordinality() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(ordinality.syntax()))
            .append(build_with_ordinality(ordinality));
    }
    doc.append(build_from_alias(item.alias()))
}

fn build_rows_from_arg<'a>(arg: ast::RowsFromArg) -> Doc<'a> {
    let mut doc = arg
        .call_expr()
        .map(build_call_expr)
        .unwrap_or_else(Doc::nil);
    if let Some(as_token) = arg.as_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&as_token))
            .append(Doc::text("as"));
    }
    if let Some(columns) = arg.column_def_list() {
        doc = doc.append(build_from_alias_columns(columns.into()));
    }
    doc
}

fn build_json_table_from_item<'a>(item: ast::JsonTableFromItem) -> Doc<'a> {
    let mut doc = Doc::nil();
    if let Some(lateral) = item.lateral_token() {
        doc = doc
            .append(leading_comments_token(&lateral))
            .append(Doc::text("lateral"))
            .append(Doc::space());
    }
    if let Some(json_table) = item.json_table() {
        doc = doc
            .append(leading_comments(json_table.syntax()))
            .append(build_json_table(json_table));
    }
    doc.append(build_from_alias(item.alias()))
}

fn build_json_table<'a>(json_table: ast::JsonTable) -> Doc<'a> {
    let mut doc = Doc::text("json_table");
    if let Some(l_paren) = json_table.l_paren_token() {
        doc = doc.append(comments_before(l_paren));
    }
    doc = doc.append(Doc::text("("));

    let mut body = Doc::nil();
    if let Some(document) = json_table.document_expr() {
        body = body
            .append(leading_comments(document.syntax()))
            .append(build_expr(document));
    }
    if let Some(format) = json_table.json_format_clause() {
        body = body
            .append(Doc::line_or_space())
            .append(leading_comments(format.syntax()))
            .append(build_json_format_clause(format));
    }
    if let Some(comma) = json_table.comma_token() {
        body = body
            .append(comments_before(comma))
            .append(Doc::text(","))
            .append(Doc::line_or_space());
    }
    if let Some(path) = json_table.path_expr() {
        body = body
            .append(leading_comments(path.syntax()))
            .append(build_expr(path));
    }
    if let Some(name) = json_table.json_path_name_clause() {
        body = body
            .append(Doc::space())
            .append(leading_comments(name.syntax()))
            .append(build_json_path_name_clause(name));
    }
    if let Some(passing) = json_table.json_passing_clause() {
        body = body
            .append(Doc::line_or_space())
            .append(leading_comments(passing.syntax()))
            .append(build_json_passing_clause(passing));
    }
    if let Some(columns) = json_table.json_table_column_list() {
        body = body
            .append(Doc::line_or_space())
            .append(leading_comments(columns.syntax()))
            .append(build_json_table_column_list(columns));
    }
    if let Some(plan) = json_table.json_table_plan_clause() {
        body = body
            .append(Doc::line_or_space())
            .append(leading_comments(plan.syntax()))
            .append(build_json_table_plan_clause(plan));
    }
    if let Some(on_error) = json_table.json_on_error_clause() {
        body = body
            .append(Doc::line_or_space())
            .append(leading_comments(on_error.syntax()))
            .append(build_json_on_error_clause(on_error));
    }
    if let Some(r_paren) = json_table.r_paren_token() {
        body = body.append(comments_before(r_paren));
    }
    doc.append(wrap_body(body)).append(Doc::text(")")).group()
}

fn build_json_path_name_clause<'a>(clause: ast::JsonPathNameClause) -> Doc<'a> {
    let mut doc = Doc::text("as");
    if let Some(name) = clause.json_path_name() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(name.syntax()))
            .append(build_name(name.syntax()));
    }
    doc
}

fn build_json_path_clause<'a>(clause: ast::JsonPathClause) -> Doc<'a> {
    let mut doc = Doc::text("path");
    if let Some(expr) = clause.expr() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(expr.syntax()))
            .append(build_expr(expr));
    }
    doc
}

fn build_json_table_column_list<'a>(list: ast::JsonTableColumnList) -> Doc<'a> {
    let mut doc = Doc::text("columns");
    if let Some(l_paren) = list.l_paren_token() {
        if comment_tokens_before(l_paren.clone()).is_empty() {
            doc = doc.append(Doc::space());
        } else {
            doc = doc.append(comments_before(l_paren));
        }
    }
    doc = doc.append(Doc::text("("));
    let columns = list.json_table_columns().map(|column| {
        (
            leading_comments(column.syntax()).append(build_json_table_column(column.clone())),
            column.syntax().clone(),
        )
    });
    let mut body = build_comma_separated_docs(columns).unwrap_or_else(Doc::nil);
    if let Some(r_paren) = list.r_paren_token() {
        body = body.append(comments_before(r_paren));
    }
    doc.append(wrap_body(body)).append(Doc::text(")")).group()
}

fn build_json_table_column<'a>(column: ast::JsonTableColumn) -> Doc<'a> {
    match column {
        ast::JsonTableColumn::JsonTableOrdinalityColumn(column) => {
            let mut doc = column
                .column_name()
                .map(|name| build_name(name.syntax()))
                .unwrap_or_else(Doc::nil);
            if let Some(for_token) = column.for_token() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments_token(&for_token))
                    .append(Doc::text("for"));
            }
            if let Some(ordinality) = column.ordinality_token() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments_token(&ordinality))
                    .append(Doc::text("ordinality"));
            }
            doc
        }
        ast::JsonTableColumn::JsonTableValueColumn(column) => {
            let mut doc = build_json_table_typed_column(column.column_name(), column.ty());
            if let Some(format) = column.json_format_clause() {
                doc = append_json_table_column_clause(doc, format, build_json_format_clause);
            }
            if let Some(path) = column.json_path_clause() {
                doc = append_json_table_column_clause(doc, path, build_json_path_clause);
            }
            if let Some(wrapper) = column.json_wrapper_behavior_clause() {
                doc = append_json_table_column_clause(
                    doc,
                    wrapper,
                    build_json_wrapper_behavior_clause,
                );
            }
            if let Some(quotes) = column.json_quotes_clause() {
                doc = append_json_table_column_clause(doc, quotes, build_json_quotes_clause);
            }
            if let Some(on_empty) = column.json_on_empty_clause() {
                doc = append_json_table_column_clause(doc, on_empty, build_json_on_empty_clause);
            }
            if let Some(on_error) = column.json_on_error_clause() {
                doc = append_json_table_column_clause(doc, on_error, build_json_on_error_clause);
            }
            doc.group()
        }
        ast::JsonTableColumn::JsonTableExistsColumn(column) => {
            let mut doc = build_json_table_typed_column(column.column_name(), column.ty());
            if let Some(exists) = column.exists_token() {
                doc = doc
                    .append(Doc::line_or_space())
                    .append(leading_comments_token(&exists))
                    .append(Doc::text("exists"));
            }
            if let Some(path) = column.json_path_clause() {
                doc = append_json_table_column_clause(doc, path, build_json_path_clause);
            }
            if let Some(on_error) = column.json_on_error_clause() {
                doc = append_json_table_column_clause(doc, on_error, build_json_on_error_clause);
            }
            doc.group()
        }
        ast::JsonTableColumn::JsonTableNestedColumn(column) => {
            let mut doc = Doc::text("nested");
            if let Some(path_token) = column.path_token() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments_token(&path_token))
                    .append(Doc::text("path"));
            }
            if let Some(expr) = column.expr() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments(expr.syntax()))
                    .append(build_expr(expr));
            }
            if let Some(name) = column.json_path_name_clause() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments(name.syntax()))
                    .append(build_json_path_name_clause(name));
            }
            if let Some(columns) = column.json_table_column_list() {
                doc = doc
                    .append(Doc::line_or_space())
                    .append(leading_comments(columns.syntax()))
                    .append(build_json_table_column_list(columns));
            }
            doc.group()
        }
    }
}

fn build_json_table_typed_column<'a>(
    name: Option<ast::ColumnName>,
    ty: Option<ast::Type>,
) -> Doc<'a> {
    let mut doc = name
        .map(|name| build_name(name.syntax()))
        .unwrap_or_else(Doc::nil);
    if let Some(ty) = ty {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(ty.syntax()))
            .append(build_type(ty));
    }
    doc
}

fn append_json_table_column_clause<'a, T: AstNode>(
    doc: Doc<'a>,
    clause: T,
    build: impl FnOnce(T) -> Doc<'a>,
) -> Doc<'a> {
    doc.append(Doc::line_or_space())
        .append(leading_comments(clause.syntax()))
        .append(build(clause))
}

fn build_json_table_plan_clause<'a>(clause: ast::JsonTablePlanClause) -> Doc<'a> {
    let mut doc = Doc::text("plan");
    if let Some(default) = clause.default_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&default))
            .append(Doc::text("default"));
    }
    if let Some(l_paren) = clause.l_paren_token() {
        if comment_tokens_before(l_paren.clone()).is_empty() {
            doc = doc.append(Doc::space());
        } else {
            doc = doc.append(comments_before(l_paren));
        }
    }
    doc = doc.append(Doc::text("("));
    let plans = clause.json_table_plans().map(|plan| {
        (
            leading_comments(plan.syntax()).append(build_json_table_plan(plan.clone())),
            plan.syntax().clone(),
        )
    });
    let mut body = build_comma_separated_docs(plans).unwrap_or_else(Doc::nil);
    if let Some(r_paren) = clause.r_paren_token() {
        body = body.append(comments_before(r_paren));
    }
    doc.append(wrap_body(body)).append(Doc::text(")")).group()
}

fn build_json_table_plan<'a>(plan: ast::JsonTablePlan) -> Doc<'a> {
    match plan {
        ast::JsonTablePlan::JsonPathNameRef(name) => build_name(name.syntax()),
        ast::JsonTablePlan::JsonTablePlanChoice(choice) => choice
            .json_table_plan_operator()
            .map(build_json_table_plan_operator)
            .unwrap_or_else(Doc::nil),
        ast::JsonTablePlan::JsonTablePlanJoin(join) => {
            let mut doc = join
                .lhs()
                .map(build_json_table_plan)
                .unwrap_or_else(Doc::nil);
            if let Some(operator) = join.json_table_plan_operator() {
                doc = doc
                    .append(Doc::line_or_space())
                    .append(leading_comments(operator.syntax()))
                    .append(build_json_table_plan_operator(operator));
            }
            if let Some(rhs) = join.rhs() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments(rhs.syntax()))
                    .append(build_json_table_plan(rhs));
            }
            doc.group()
        }
        ast::JsonTablePlan::ParenJsonTablePlan(plan) => {
            let mut doc = plan
                .l_paren_token()
                .map(comments_before)
                .unwrap_or_else(Doc::nil)
                .append(Doc::text("("));
            let mut body = plan
                .json_table_plan()
                .map(|plan| leading_comments(plan.syntax()).append(build_json_table_plan(plan)))
                .unwrap_or_else(Doc::nil);
            if let Some(r_paren) = plan.r_paren_token() {
                body = body.append(comments_before(r_paren));
            }
            doc = doc.append(wrap_body(body)).append(Doc::text(")")).group();
            doc
        }
    }
}

fn build_json_table_plan_operator<'a>(operator: ast::JsonTablePlanOperator) -> Doc<'a> {
    match operator {
        ast::JsonTablePlanOperator::JsonTablePlanCross(_) => Doc::text("cross"),
        ast::JsonTablePlanOperator::JsonTablePlanInner(_) => Doc::text("inner"),
        ast::JsonTablePlanOperator::JsonTablePlanOuter(_) => Doc::text("outer"),
        ast::JsonTablePlanOperator::JsonTablePlanUnion(_) => Doc::text("union"),
    }
}

fn build_xml_table_from_item<'a>(item: ast::XmlTableFromItem) -> Doc<'a> {
    let mut doc = Doc::nil();
    if let Some(lateral) = item.lateral_token() {
        doc = doc
            .append(leading_comments_token(&lateral))
            .append(Doc::text("lateral"))
            .append(Doc::space());
    }
    if let Some(xml_table) = item.xml_table() {
        doc = doc
            .append(leading_comments(xml_table.syntax()))
            .append(build_xml_table(xml_table));
    }
    doc.append(build_from_alias(item.alias()))
}

fn build_xml_table<'a>(xml_table: ast::XmlTable) -> Doc<'a> {
    let mut doc = Doc::text("xmltable");
    if let Some(l_paren) = xml_table.l_paren_token() {
        doc = doc.append(comments_before(l_paren));
    }
    doc = doc.append(Doc::text("("));

    let mut body = Doc::nil();
    if let Some(namespaces) = xml_table.xml_namespace_list() {
        if let Some(xmlnamespaces) = xml_table.xmlnamespaces_token() {
            body = body
                .append(leading_comments_token(&xmlnamespaces))
                .append(Doc::text("xmlnamespaces"));
        }
        body = body
            .append(comments_before(namespaces.syntax().clone()))
            .append(build_xml_namespace_list(namespaces));
        if let Some(comma) = xml_table.comma_token() {
            body = body
                .append(comments_before(comma))
                .append(Doc::text(","))
                .append(Doc::line_or_space());
        }
    }
    if let Some(passing) = xml_table.xml_row_passing_clause() {
        body = body
            .append(leading_comments(passing.syntax()))
            .append(build_xml_row_passing_clause(passing));
    }
    if let Some(columns) = xml_table.xml_table_column_list() {
        body = body
            .append(Doc::line_or_space())
            .append(leading_comments(columns.syntax()))
            .append(build_xml_table_column_list(columns));
    }
    if let Some(r_paren) = xml_table.r_paren_token() {
        body = body.append(comments_before(r_paren));
    }
    doc.append(wrap_body(body)).append(Doc::text(")")).group()
}

fn build_xml_namespace_list<'a>(list: ast::XmlNamespaceList) -> Doc<'a> {
    let mut doc = list
        .l_paren_token()
        .map(comments_before)
        .unwrap_or_else(Doc::nil)
        .append(Doc::text("("));
    let namespaces = list.xml_namespaces().map(|namespace| {
        (
            leading_comments(namespace.syntax()).append(build_xml_namespace(namespace.clone())),
            namespace.syntax().clone(),
        )
    });
    let mut body = build_comma_separated_docs(namespaces).unwrap_or_else(Doc::nil);
    if let Some(r_paren) = list.r_paren_token() {
        body = body.append(comments_before(r_paren));
    }
    doc = doc.append(wrap_body(body)).append(Doc::text(")")).group();
    doc
}

fn build_xml_namespace<'a>(namespace: ast::XmlNamespace) -> Doc<'a> {
    let mut doc = Doc::nil();
    if let Some(default) = namespace.default_token() {
        doc = doc
            .append(leading_comments_token(&default))
            .append(Doc::text("default"))
            .append(Doc::space());
    }
    if let Some(expr) = namespace.expr() {
        doc = doc
            .append(leading_comments(expr.syntax()))
            .append(build_expr(expr));
    }
    if let Some(as_token) = namespace.as_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&as_token))
            .append(Doc::text("as"));
    }
    if let Some(prefix) = namespace.prefix() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(prefix.syntax()))
            .append(build_name(prefix.syntax()));
    }
    doc
}

fn build_xml_row_passing_clause<'a>(clause: ast::XmlRowPassingClause) -> Doc<'a> {
    let mut doc = clause.row().map(build_expr).unwrap_or_else(Doc::nil);
    if let Some(passing) = clause.passing_token() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments_token(&passing))
            .append(Doc::text("passing"));
    }
    if let Some(mech) = clause.xml_passing_mech() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(mech.syntax()))
            .append(build_xml_passing_mech(mech));
    }
    doc = doc.group();
    if let Some(passing_doc) = clause.xml_passing_doc() {
        let mut passing_doc_doc = leading_comments(passing_doc.syntax());
        if let Some(expr) = passing_doc.expr() {
            passing_doc_doc = passing_doc_doc
                .append(leading_comments(expr.syntax()))
                .append(build_expr(expr));
        }
        if let Some(mech) = passing_doc.xml_passing_mech() {
            passing_doc_doc = passing_doc_doc
                .append(Doc::space())
                .append(leading_comments(mech.syntax()))
                .append(build_xml_passing_mech(mech));
        }
        doc = doc.append(Doc::line_or_space().append(passing_doc_doc).nest(2));
    }
    doc.group()
}

fn build_xml_table_column_list<'a>(list: ast::XmlTableColumnList) -> Doc<'a> {
    let mut doc = Doc::text("columns");
    let columns = list.xml_table_columns().map(|column| {
        (
            leading_comments(column.syntax()).append(build_xml_table_column(column.clone())),
            column.syntax().clone(),
        )
    });
    if let Some(columns) = build_comma_separated_docs(columns) {
        doc = doc.append(Doc::hard_line().append(columns).nest(2));
    }
    doc
}

fn build_xml_table_column<'a>(column: ast::XmlTableColumn) -> Doc<'a> {
    let mut doc = column
        .column_name()
        .map(|name| build_name(name.syntax()))
        .unwrap_or_else(Doc::nil);
    if let Some(ty) = column.ty() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(ty.syntax()))
            .append(build_type(ty));
    } else {
        if let Some(for_token) = column.for_token() {
            doc = doc
                .append(Doc::space())
                .append(leading_comments_token(&for_token))
                .append(Doc::text("for"));
        }
        if let Some(ordinality) = column.ordinality_token() {
            doc = doc
                .append(Doc::space())
                .append(leading_comments_token(&ordinality))
                .append(Doc::text("ordinality"));
        }
    }
    if let Some(options) = column.xml_column_option_list() {
        let mut first = true;
        for option in options.xml_column_options() {
            doc = doc.append(Doc::line_or_space());
            if first {
                doc = doc.append(leading_comments(options.syntax()));
                first = false;
            }
            doc = doc
                .append(leading_comments(option.syntax()))
                .append(build_xml_column_option(option));
        }
    }
    doc.group()
}

fn build_xml_column_option<'a>(option: ast::XmlColumnOption) -> Doc<'a> {
    match option {
        ast::XmlColumnOption::OptionDefault(option) => {
            Doc::text("default").append(Doc::space()).append(
                option
                    .expr()
                    .map(|expr| leading_comments(expr.syntax()).append(build_expr(expr)))
                    .unwrap_or_else(Doc::nil),
            )
        }
        ast::XmlColumnOption::OptionIdent(option) => {
            let mut doc = build_name(option.syntax());
            if let Some(expr) = option.expr() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments(expr.syntax()))
                    .append(build_expr(expr));
            }
            doc
        }
        ast::XmlColumnOption::OptionNotNull(option) => {
            build_two_keywords(option.not_token(), "not", option.null_token(), "null")
        }
        ast::XmlColumnOption::OptionNull(_) => Doc::text("null"),
        ast::XmlColumnOption::OptionPath(option) => Doc::text("path").append(Doc::space()).append(
            option
                .expr()
                .map(|expr| leading_comments(expr.syntax()).append(build_expr(expr)))
                .unwrap_or_else(Doc::nil),
        ),
    }
}

fn build_graph_table_from_item<'a>(item: ast::GraphTableFromItem) -> Doc<'a> {
    let mut doc = Doc::nil();
    if let Some(lateral) = item.lateral_token() {
        doc = doc
            .append(leading_comments_token(&lateral))
            .append(Doc::text("lateral"))
            .append(Doc::space());
    }
    if let Some(graph_table) = item.graph_table_fn() {
        doc = doc
            .append(leading_comments(graph_table.syntax()))
            .append(build_graph_table_fn(graph_table));
    }
    doc.append(build_from_alias(item.alias()))
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
            .append(build_call_expr_with_spacing(call, true));
    }
    if let Some(repeatable) = tablesample.repeatable_clause() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(repeatable.syntax()))
            .append(Doc::text("repeatable"));
        if let Some(l_paren) = repeatable.l_paren_token() {
            if comment_tokens_before(l_paren.clone()).is_empty() {
                doc = doc.append(Doc::space());
            } else {
                doc = doc.append(comments_before(l_paren));
            }
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
        if comment_tokens_before(l_paren.clone()).is_empty() {
            doc = doc.append(Doc::space());
        } else {
            doc = doc.append(comments_before(l_paren));
        }
    }
    doc = doc.append(Doc::text("("));

    let has_items = !items.is_empty();
    let mut body = if has_items {
        Doc::list(
            Itertools::intersperse(
                items.into_iter(),
                Doc::text(",").append(Doc::line_or_space()),
            )
            .collect(),
        )
    } else {
        Doc::nil()
    };
    if !has_items {
        if let Some(r_paren) = r_paren {
            body = body.append(comments_before(r_paren));
        }
    }
    doc.append(wrap_body(body)).append(Doc::text(")")).group()
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
        if comment_tokens_before(l_paren.clone()).is_empty() {
            doc = doc.append(Doc::space());
        } else {
            doc = doc.append(comments_before(l_paren));
        }
    }
    doc = doc.append(Doc::text("("));

    if items.is_empty() {
        if let Some(r_paren) = r_paren {
            doc = doc.append(comments_before(r_paren));
        }
    } else {
        doc = doc.append(wrap_body(Doc::list(
            Itertools::intersperse(
                items.into_iter(),
                Doc::text(",").append(Doc::line_or_space()),
            )
            .collect(),
        )));
    }

    doc.append(Doc::text(")")).group()
}

fn wrap_body<'a>(body: Doc<'a>) -> Doc<'a> {
    Doc::line_or_nil()
        .append(body)
        .nest(2)
        .append(Doc::line_or_nil())
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
        if let Some(l_paren) = array_expr.l_paren_token() {
            doc = doc.append(comments_before(l_paren));
        }
        let mut body = leading_comments(select.syntax()).append(build_select_doc(&select));
        if let Some(r_paren) = array_expr.r_paren_token() {
            body = body.append(comments_before(r_paren));
        }
        doc.append(Doc::text("("))
            .append(wrap_body(body))
            .append(Doc::text(")"))
            .group()
    } else {
        if let Some(l_brack) = array_expr.l_brack_token() {
            doc = doc.append(comments_before(l_brack));
        }
        doc = doc.append(Doc::text("["));

        let exprs = array_expr.exprs().map(|expr| {
            let syntax = expr.syntax().clone();
            let doc = leading_comments(expr.syntax()).append(build_expr(expr));
            (doc, syntax)
        });
        let mut body = build_comma_separated_docs(exprs).unwrap_or_else(Doc::nil);
        if let Some(r_brack) = array_expr.r_brack_token() {
            body = body.append(comments_before(r_brack));
        }
        doc.append(wrap_body(body)).append(Doc::text("]")).group()
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

    let mut body = Doc::nil();
    if let Some(index) = index_expr.index() {
        body = body
            .append(leading_comments(index.syntax()))
            .append(match index {
                ast::Expr::BinExpr(binary) => build_bin_expr(binary),
                expression => build_expr(expression),
            });
    }
    if let Some(r_brack) = index_expr.r_brack_token() {
        body = body.append(comments_before(r_brack));
    }
    doc.append(wrap_body(body)).append(Doc::text("]")).group()
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

    let exprs = build_comma_separated_exprs(tuple_expr.exprs());
    let has_exprs = exprs.is_some();
    let mut body = exprs.unwrap_or_else(Doc::nil);
    if !has_exprs {
        if let Some(r_paren) = tuple_expr.r_paren_token() {
            body = body.append(comments_before(r_paren));
        }
    }

    doc.append(wrap_body(body)).append(Doc::text(")")).group()
}

fn build_between_expr<'a>(between_expr: ast::BetweenExpr) -> Doc<'a> {
    let mut doc = build_expr(between_expr.target().unwrap()).append(Doc::line_or_space());
    if between_expr.not_token().is_some() {
        doc = doc.append(Doc::text("not")).append(Doc::space());
    }
    doc = doc.append(Doc::text("between"));
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
        .append(Doc::line_or_space())
        .append(Doc::text("and"))
        .append(Doc::space())
        .append(build_expr(between_expr.end().unwrap()))
        .nest(2)
        .group()
}

fn build_call_expr<'a>(call_expr: ast::CallExpr) -> Doc<'a> {
    build_call_expr_with_spacing(call_expr, false)
}

fn build_call_expr_with_spacing<'a>(call_expr: ast::CallExpr, space_before_paren: bool) -> Doc<'a> {
    if let (Some(expr), Some(arg_list)) = (call_expr.expr(), call_expr.arg_list()) {
        let mut doc = build_expr(expr);
        if space_before_paren && comment_tokens_before(arg_list.syntax().clone()).is_empty() {
            doc = doc.append(Doc::space());
        }
        doc = doc
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
    } else if let Some(collation_for_fn) = call_expr.collation_for_fn() {
        build_collation_for_fn(collation_for_fn)
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
    } else if let Some(graph_table_fn) = call_expr.graph_table_fn() {
        build_graph_table_fn(graph_table_fn)
    } else if let Some(json_array_agg_fn) = call_expr.json_array_agg_fn() {
        build_json_array_agg_fn(json_array_agg_fn)
    } else if let Some(json_array_fn) = call_expr.json_array_fn() {
        build_json_array_fn(json_array_fn)
    } else if let Some(json_exists_fn) = call_expr.json_exists_fn() {
        build_json_exists_fn(json_exists_fn)
    } else if let Some(json_fn) = call_expr.json_fn() {
        build_json_fn(json_fn)
    } else if let Some(json_object_agg_fn) = call_expr.json_object_agg_fn() {
        build_json_object_agg_fn(json_object_agg_fn)
    } else if let Some(json_object_fn) = call_expr.json_object_fn() {
        build_json_object_fn(json_object_fn)
    } else if let Some(json_query_fn) = call_expr.json_query_fn() {
        build_json_query_fn(json_query_fn)
    } else if let Some(json_scalar_fn) = call_expr.json_scalar_fn() {
        build_json_scalar_fn(json_scalar_fn)
    } else if let Some(json_serialize_fn) = call_expr.json_serialize_fn() {
        build_json_serialize_fn(json_serialize_fn)
    } else if let Some(json_value_fn) = call_expr.json_value_fn() {
        build_json_value_fn(json_value_fn)
    } else if let Some(overlay_fn) = call_expr.overlay_fn() {
        build_overlay_fn(overlay_fn)
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
    } else if let Some(xml_element_fn) = call_expr.xml_element_fn() {
        build_xml_element_fn(xml_element_fn)
    } else if let Some(xml_exists_fn) = call_expr.xml_exists_fn() {
        build_xml_exists_fn(xml_exists_fn)
    } else if let Some(xml_forest_fn) = call_expr.xml_forest_fn() {
        build_xml_forest_fn(xml_forest_fn)
    } else if let Some(xml_parse_fn) = call_expr.xml_parse_fn() {
        build_xml_parse_fn(xml_parse_fn)
    } else if let Some(xml_pi_fn) = call_expr.xml_pi_fn() {
        build_xml_pi_fn(xml_pi_fn)
    } else if let Some(xml_root_fn) = call_expr.xml_root_fn() {
        build_xml_root_fn(xml_root_fn)
    } else if let Some(xml_serialize_fn) = call_expr.xml_serialize_fn() {
        build_xml_serialize_fn(xml_serialize_fn)
    } else {
        unreachable!("a call expression should contain a supported function node")
    }
}

fn build_graph_table_fn<'a>(graph_table_fn: ast::GraphTableFn) -> Doc<'a> {
    let mut doc = Doc::text("graph_table");
    if let Some(l_paren) = graph_table_fn.l_paren_token() {
        doc = doc.append(comments_before(l_paren));
    }
    doc = doc.append(Doc::text("("));

    let mut body = Doc::nil();
    if let Some(graph) = graph_table_fn.property_graph_ref() {
        if let Some(path) = graph.path_ref() {
            body = body
                .append(leading_comments(graph.syntax()))
                .append(build_path_ref(&path));
        }
    }
    if let Some(match_token) = graph_table_fn.match_token() {
        body = body
            .append(Doc::line_or_space())
            .append(leading_comments_token(&match_token))
            .append(Doc::text("match"));
    }
    if let Some(patterns) = graph_table_fn.path_pattern_list() {
        body = body.append(
            Doc::line_or_space()
                .append(leading_comments(patterns.syntax()))
                .append(build_path_pattern_list(patterns))
                .nest(2),
        );
    }
    if let Some(where_clause) = graph_table_fn.where_clause() {
        body = body
            .append(Doc::line_or_space())
            .append(leading_comments(where_clause.syntax()))
            .append(build_where_clause(where_clause));
    }
    if let Some(columns) = graph_table_fn.columns_token() {
        body = body
            .append(Doc::line_or_space())
            .append(leading_comments_token(&columns))
            .append(Doc::text("columns"));
    }
    if let Some(columns) = graph_table_fn.expr_as_column_name_list() {
        if comment_tokens_before(columns.syntax().clone()).is_empty() {
            body = body.append(Doc::space());
        } else {
            body = body.append(comments_before(columns.syntax().clone()));
        }
        body = body.append(build_expr_as_column_name_list(columns));
    }

    if let Some(r_paren) = graph_table_fn.r_paren_token() {
        body = body.append(comments_before(r_paren));
    }
    doc.append(wrap_body(body).group()).append(Doc::text(")"))
}

fn build_path_pattern_list<'a>(patterns: ast::PathPatternList) -> Doc<'a> {
    let items = patterns.path_patterns().map(|pattern| {
        (
            leading_comments(pattern.syntax()).append(build_path_pattern(pattern.clone())),
            pattern.syntax().clone(),
        )
    });
    build_comma_separated_docs(items).unwrap_or_else(Doc::nil)
}

fn build_path_pattern<'a>(pattern: ast::PathPattern) -> Doc<'a> {
    Doc::list(
        Itertools::intersperse(
            pattern
                .path_factors()
                .map(|factor| leading_comments(factor.syntax()).append(build_path_factor(factor))),
            Doc::line_or_nil(),
        )
        .collect(),
    )
    .nest(2)
    .group()
}

fn build_path_factor<'a>(factor: ast::PathFactor) -> Doc<'a> {
    let mut doc = factor
        .path_primary()
        .map(build_path_primary)
        .unwrap_or_else(Doc::nil);
    if let Some(qualifier) = factor.graph_pattern_qualifier() {
        doc = doc
            .append(leading_comments(qualifier.syntax()))
            .append(build_graph_pattern_qualifier(qualifier));
    }
    doc
}

fn build_path_primary<'a>(primary: ast::PathPrimary) -> Doc<'a> {
    match primary {
        ast::PathPrimary::VertexPattern(pattern) => build_vertex_pattern(pattern),
        ast::PathPrimary::EdgeLeft(edge) => build_edge_left(edge),
        ast::PathPrimary::EdgeRight(edge) => build_edge_right(edge),
        ast::PathPrimary::EdgeAny(edge) => build_edge_any(edge),
        ast::PathPrimary::ParenGraphPattern(pattern) => build_paren_graph_pattern(pattern),
    }
}

fn build_vertex_pattern<'a>(pattern: ast::VertexPattern) -> Doc<'a> {
    let mut doc = pattern
        .l_paren_token()
        .map(comments_before)
        .unwrap_or_else(Doc::nil)
        .append(Doc::text("("))
        .append(build_graph_pattern_inner(
            pattern.element_variable(),
            pattern.is_label(),
            pattern.where_clause(),
        ));
    if let Some(r_paren) = pattern.r_paren_token() {
        doc = doc.append(comments_before(r_paren));
    }
    doc.append(Doc::text(")"))
}

fn build_edge_left<'a>(edge: ast::EdgeLeft) -> Doc<'a> {
    let mut doc = Doc::text("<");
    if let Some(minus) = edge.minus_token() {
        doc = doc.append(comments_before(minus));
    }
    doc = doc.append(Doc::text("-"));
    if let Some(l_brack) = edge.l_brack_token() {
        doc = doc
            .append(comments_before(l_brack))
            .append(Doc::text("["))
            .append(build_graph_pattern_inner(
                edge.element_variable(),
                edge.is_label(),
                edge.where_clause(),
            ));
        if let Some(r_brack) = edge.r_brack_token() {
            doc = doc.append(comments_before(r_brack));
        }
        doc = doc.append(Doc::text("]"));
        if let Some(minus) = edge.end_minus_token() {
            doc = doc.append(comments_before(minus));
        }
        doc = doc.append(Doc::text("-"));
    }
    doc
}

fn build_edge_right<'a>(edge: ast::EdgeRight) -> Doc<'a> {
    let mut doc = Doc::text("-");
    if let Some(l_brack) = edge.l_brack_token() {
        doc = doc
            .append(comments_before(l_brack))
            .append(Doc::text("["))
            .append(build_graph_pattern_inner(
                edge.element_variable(),
                edge.is_label(),
                edge.where_clause(),
            ));
        if let Some(r_brack) = edge.r_brack_token() {
            doc = doc.append(comments_before(r_brack));
        }
        doc = doc.append(Doc::text("]"));
        if let Some(minus) = edge.end_minus_token() {
            doc = doc.append(comments_before(minus));
        }
        doc = doc.append(Doc::text("-"));
    }
    if let Some(r_angle) = edge.r_angle_token() {
        doc = doc.append(comments_before(r_angle));
    }
    doc.append(Doc::text(">"))
}

fn build_edge_any<'a>(edge: ast::EdgeAny) -> Doc<'a> {
    let mut doc = Doc::text("-");
    if let Some(l_brack) = edge.l_brack_token() {
        doc = doc
            .append(comments_before(l_brack))
            .append(Doc::text("["))
            .append(build_graph_pattern_inner(
                edge.element_variable(),
                edge.is_label(),
                edge.where_clause(),
            ));
        if let Some(r_brack) = edge.r_brack_token() {
            doc = doc.append(comments_before(r_brack));
        }
        doc = doc.append(Doc::text("]"));
        if let Some(minus) = edge.end_minus_token() {
            doc = doc.append(comments_before(minus));
        }
        doc = doc.append(Doc::text("-"));
    }
    doc
}

fn build_graph_pattern_inner<'a>(
    variable: Option<ast::ElementVariable>,
    label: Option<ast::IsLabel>,
    where_clause: Option<ast::WhereClause>,
) -> Doc<'a> {
    let mut doc = Doc::nil();
    if let Some(variable) = variable {
        doc = doc
            .append(leading_comments(variable.syntax()))
            .append(build_name(variable.syntax()));
    }
    if let Some(label) = label {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(label.syntax()))
            .append(build_is_label(label));
    }
    if let Some(where_clause) = where_clause {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(where_clause.syntax()))
            .append(build_where_clause(where_clause));
    }
    doc.nest(2).group()
}

fn build_is_label<'a>(label: ast::IsLabel) -> Doc<'a> {
    let mut doc = label
        .is_token()
        .map(|token| leading_comments_token(&token).append(Doc::text("is")))
        .unwrap_or_else(Doc::nil);
    if let Some(expr) = label.expr() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(expr.syntax()))
            .append(build_expr(expr));
    }
    doc
}

fn build_where_clause<'a>(where_clause: ast::WhereClause) -> Doc<'a> {
    let mut doc = where_clause
        .where_token()
        .map(|token| leading_comments_token(&token).append(Doc::text("where")))
        .unwrap_or_else(Doc::nil);
    if let Some(expr) = where_clause.expr() {
        let expr_doc = match expr.clone() {
            ast::Expr::BinExpr(bin_expr) => {
                if let Some(logical) = bin_expr.op().as_ref().and_then(logical_op) {
                    build_logical_expr(bin_expr, logical)
                } else {
                    build_expr(expr.clone())
                }
            }
            _ => build_expr(expr.clone()),
        };
        doc = doc.append(
            Doc::line_or_space()
                .append(leading_comments(expr.syntax()))
                .append(expr_doc)
                .nest(2),
        );
    }
    doc.group()
}

fn build_paren_graph_pattern<'a>(pattern: ast::ParenGraphPattern) -> Doc<'a> {
    let mut doc = pattern
        .l_paren_token()
        .map(comments_before)
        .unwrap_or_else(Doc::nil)
        .append(Doc::text("("));
    if let Some(inner) = pattern.path_pattern() {
        doc = doc
            .append(leading_comments(inner.syntax()))
            .append(build_path_pattern(inner));
    }
    if let Some(where_clause) = pattern.where_clause() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(where_clause.syntax()))
            .append(build_where_clause(where_clause));
    }
    if let Some(r_paren) = pattern.r_paren_token() {
        doc = doc.append(comments_before(r_paren));
    }
    doc.append(Doc::text(")"))
}

fn build_graph_pattern_qualifier<'a>(qualifier: ast::GraphPatternQualifier) -> Doc<'a> {
    let mut doc = qualifier
        .l_curly_token()
        .map(comments_before)
        .unwrap_or_else(Doc::nil)
        .append(Doc::text("{"));
    if let Some(min) = qualifier.min() {
        if let Some(literal) = min.literal() {
            doc = doc
                .append(leading_comments(min.syntax()))
                .append(build_literal(literal));
        }
    }
    if let Some(comma) = qualifier.comma_token() {
        doc = doc.append(comments_before(comma)).append(Doc::text(","));
    }
    if let Some(max) = qualifier.max() {
        if qualifier.comma_token().is_some() {
            doc = doc.append(Doc::space());
        }
        if let Some(literal) = max.literal() {
            doc = doc
                .append(leading_comments(max.syntax()))
                .append(build_literal(literal));
        }
    }
    if let Some(r_curly) = qualifier.r_curly_token() {
        doc = doc.append(comments_before(r_curly));
    }
    doc.append(Doc::text("}"))
}

fn build_expr_as_column_name_list<'a>(list: ast::ExprAsColumnNameList) -> Doc<'a> {
    let doc = list
        .l_paren_token()
        .map(comments_before)
        .unwrap_or_else(Doc::nil)
        .append(Doc::text("("));
    let items = list.expr_as_column_names().map(|item| {
        let mut item_doc = item.expr().map(build_expr).unwrap_or_else(Doc::nil);
        if let Some(as_token) = item.as_token() {
            item_doc = item_doc
                .append(Doc::space())
                .append(leading_comments_token(&as_token))
                .append(Doc::text("as"));
        }
        if let Some(name) = item.column_name() {
            item_doc = item_doc
                .append(Doc::space())
                .append(leading_comments(name.syntax()))
                .append(build_name(name.syntax()));
        }
        (
            leading_comments(item.syntax()).append(item_doc),
            item.syntax().clone(),
        )
    });
    let mut body = build_comma_separated_docs(items).unwrap_or_else(Doc::nil);
    if let Some(r_paren) = list.r_paren_token() {
        body = body.append(comments_before(r_paren));
    }
    doc.append(wrap_body(body)).append(Doc::text(")")).group()
}

fn build_xml_element_fn<'a>(xml_element_fn: ast::XmlElementFn) -> Doc<'a> {
    let mut doc = Doc::text("xmlelement");
    if let Some(l_paren) = xml_element_fn.l_paren_token() {
        doc = doc.append(comments_before(l_paren));
    }
    doc = doc.append(Doc::text("("));

    let mut body = Doc::nil();
    if let Some(name) = xml_element_fn.name_token() {
        body = body
            .append(leading_comments_token(&name))
            .append(Doc::text("name"));
    }

    let Some(tag) = xml_element_fn.tag() else {
        return doc.append(Doc::text(")"));
    };
    body = body
        .append(Doc::space())
        .append(leading_comments(tag.syntax()))
        .append(build_name(tag.syntax()));

    let mut items = Vec::new();
    if let Some(attrs) = xml_element_fn.expr_as_xml_attr_list() {
        let attrs_doc = xml_element_fn
            .xmlattributes_token()
            .map(|token| {
                leading_comments_token(&token)
                    .append(Doc::text("xmlattributes"))
                    .append(comments_before(attrs.syntax().clone()))
            })
            .unwrap_or_else(Doc::nil)
            .append(build_expr_as_xml_attr_list(attrs.clone()));
        items.push((attrs_doc, attrs.syntax().clone()));
    }
    items.extend(xml_element_fn.exprs().map(|expr| {
        (
            leading_comments(expr.syntax()).append(build_expr(expr.clone())),
            expr.syntax().clone(),
        )
    }));

    let mut previous = tag.syntax().clone();
    for (item, syntax) in items {
        body = body
            .append(trailing_comments(&previous))
            .append(Doc::text(","))
            .append(Doc::line_or_space())
            .append(item);
        previous = syntax;
    }

    if let Some(r_paren) = xml_element_fn.r_paren_token() {
        body = body.append(comments_before(r_paren));
    }
    doc.append(wrap_body(body)).append(Doc::text(")")).group()
}

fn build_expr_as_xml_attr_list<'a>(attrs: ast::ExprAsXmlAttrList) -> Doc<'a> {
    let mut doc = Doc::nil();
    if let Some(l_paren) = attrs.l_paren_token() {
        doc = doc.append(comments_before(l_paren));
    }
    doc = doc.append(Doc::text("("));

    let items = attrs.expr_as_xml_attrs().map(|attr| {
        let mut item = attr.expr().map(build_expr).unwrap_or_else(Doc::nil);
        if let Some(as_token) = attr.as_token() {
            item = item
                .append(Doc::space())
                .append(leading_comments_token(&as_token))
                .append(Doc::text("as"));
        }
        if let Some(name) = attr.attr() {
            item = item
                .append(Doc::space())
                .append(leading_comments(name.syntax()))
                .append(build_name(name.syntax()));
        }
        (
            leading_comments(attr.syntax()).append(item),
            attr.syntax().clone(),
        )
    });
    let mut body = build_comma_separated_docs(items).unwrap_or_else(Doc::nil);

    if let Some(r_paren) = attrs.r_paren_token() {
        body = body.append(comments_before(r_paren));
    }
    doc.append(wrap_body(body)).append(Doc::text(")")).group()
}

fn build_xml_exists_fn<'a>(xml_exists_fn: ast::XmlExistsFn) -> Doc<'a> {
    let mut doc = Doc::text("xmlexists");
    if let Some(l_paren) = xml_exists_fn.l_paren_token() {
        doc = doc.append(comments_before(l_paren));
    }
    doc = doc.append(Doc::text("("));

    let mut body = Doc::nil();

    if let Some(passing) = xml_exists_fn.xml_row_passing_clause() {
        if let Some(row) = passing.row() {
            body = body
                .append(leading_comments(passing.syntax()))
                .append(build_expr(row));
        }
        if let Some(passing_token) = passing.passing_token() {
            body = body
                .append(Doc::line_or_space())
                .append(leading_comments_token(&passing_token))
                .append(Doc::text("passing"));
        }
        if let Some(mech) = passing.xml_passing_mech() {
            body = body
                .append(Doc::line_or_space())
                .append(leading_comments(mech.syntax()))
                .append(build_xml_passing_mech(mech));
        }
        if let Some(passing_doc) = passing.xml_passing_doc() {
            if let Some(expr) = passing_doc.expr() {
                body = body
                    .append(Doc::line_or_space())
                    .append(leading_comments(passing_doc.syntax()))
                    .append(build_expr(expr));
            }
            if let Some(mech) = passing_doc.xml_passing_mech() {
                body = body
                    .append(Doc::line_or_space())
                    .append(leading_comments(mech.syntax()))
                    .append(build_xml_passing_mech(mech));
            }
        }
    }

    if let Some(r_paren) = xml_exists_fn.r_paren_token() {
        body = body.append(comments_before(r_paren));
    }
    doc.append(wrap_body(body)).append(Doc::text(")")).group()
}

fn build_xml_forest_fn<'a>(xml_forest_fn: ast::XmlForestFn) -> Doc<'a> {
    Doc::text("xmlforest")
        .append(
            xml_forest_fn
                .expr_as_element_tag_list()
                .map(|list| {
                    comments_before(list.syntax().clone())
                        .append(build_expr_as_element_tag_list(list))
                })
                .unwrap_or_else(Doc::nil),
        )
        .group()
}

fn build_expr_as_element_tag_list<'a>(list: ast::ExprAsElementTagList) -> Doc<'a> {
    let mut doc = Doc::nil();
    if let Some(l_paren) = list.l_paren_token() {
        doc = doc.append(comments_before(l_paren));
    }
    doc = doc.append(Doc::text("("));

    let items = list.expr_as_element_tags().map(|item| {
        let mut item_doc = item.expr().map(build_expr).unwrap_or_else(Doc::nil);
        if let Some(as_token) = item.as_token() {
            item_doc = item_doc
                .append(Doc::space())
                .append(leading_comments_token(&as_token))
                .append(Doc::text("as"));
        }
        if let Some(tag) = item.tag() {
            item_doc = item_doc
                .append(Doc::space())
                .append(leading_comments(tag.syntax()))
                .append(build_name(tag.syntax()));
        }
        (
            leading_comments(item.syntax()).append(item_doc),
            item.syntax().clone(),
        )
    });
    let mut body = build_comma_separated_docs(items).unwrap_or_else(Doc::nil);

    if let Some(r_paren) = list.r_paren_token() {
        body = body.append(comments_before(r_paren));
    }
    doc.append(wrap_body(body)).append(Doc::text(")")).group()
}

fn build_xml_parse_fn<'a>(xml_parse_fn: ast::XmlParseFn) -> Doc<'a> {
    let mut doc = Doc::text("xmlparse");
    if let Some(l_paren) = xml_parse_fn.l_paren_token() {
        doc = doc.append(comments_before(l_paren));
    }
    doc = doc.append(Doc::text("("));

    let mut body = Doc::nil();

    if let Some(kind) = xml_parse_fn.xml_document_or_content() {
        body = body
            .append(leading_comments(kind.syntax()))
            .append(build_xml_document_or_content(kind));
    }
    if let Some(expr) = xml_parse_fn.expr() {
        body = body
            .append(Doc::line_or_space())
            .append(leading_comments(expr.syntax()))
            .append(build_expr(expr));
    }
    if let Some(whitespace) = xml_parse_fn.xml_whitespace() {
        body = body
            .append(Doc::line_or_space())
            .append(leading_comments(whitespace.syntax()))
            .append(build_xml_whitespace(whitespace));
    }

    if let Some(r_paren) = xml_parse_fn.r_paren_token() {
        body = body.append(comments_before(r_paren));
    }
    doc.append(wrap_body(body)).append(Doc::text(")")).group()
}

fn build_xml_pi_fn<'a>(xml_pi_fn: ast::XmlPiFn) -> Doc<'a> {
    let mut doc = Doc::text("xmlpi");
    if let Some(l_paren) = xml_pi_fn.l_paren_token() {
        doc = doc.append(comments_before(l_paren));
    }
    doc = doc.append(Doc::text("("));

    let mut body = Doc::nil();

    if let Some(name) = xml_pi_fn.name_token() {
        body = body
            .append(leading_comments_token(&name))
            .append(Doc::text("name"));
    }
    if let Some(target) = xml_pi_fn.target() {
        body = body
            .append(Doc::space())
            .append(leading_comments(target.syntax()))
            .append(build_name(target.syntax()));
    }
    if let Some(expr) = xml_pi_fn.expr() {
        if let Some(comma) = xml_pi_fn.comma_token() {
            body = body.append(comments_before(comma));
        }
        body = body
            .append(Doc::text(","))
            .append(Doc::line_or_space())
            .append(leading_comments(expr.syntax()))
            .append(build_expr(expr));
    }

    if let Some(r_paren) = xml_pi_fn.r_paren_token() {
        body = body.append(comments_before(r_paren));
    }
    doc.append(wrap_body(body)).append(Doc::text(")")).group()
}

fn build_xml_root_fn<'a>(xml_root_fn: ast::XmlRootFn) -> Doc<'a> {
    let mut doc = Doc::text("xmlroot");
    if let Some(l_paren) = xml_root_fn.l_paren_token() {
        doc = doc.append(comments_before(l_paren));
    }
    doc = doc.append(Doc::text("("));

    let mut body = Doc::nil();

    if let Some(expr) = xml_root_fn.expr() {
        body = body
            .append(leading_comments(expr.syntax()))
            .append(build_expr(expr));
    }
    if let Some(comma) = xml_root_fn.comma_token() {
        body = body.append(comments_before(comma));
    }
    body = body.append(Doc::text(",")).append(Doc::line_or_space());
    if let Some(version) = xml_root_fn.xml_root_version() {
        body = body
            .append(leading_comments(version.syntax()))
            .append(build_xml_root_version(version));
    }
    if let Some(standalone) = xml_root_fn.xml_standalone() {
        body = body
            .append(leading_comments(standalone.syntax()))
            .append(build_xml_standalone(standalone));
    }

    if let Some(r_paren) = xml_root_fn.r_paren_token() {
        body = body.append(comments_before(r_paren));
    }
    doc.append(wrap_body(body)).append(Doc::text(")")).group()
}

fn build_xml_root_version<'a>(version: ast::XmlRootVersion) -> Doc<'a> {
    match version {
        ast::XmlRootVersion::XmlVersionExpr(version) => {
            let mut doc = version
                .version_token()
                .map(|token| leading_comments_token(&token).append(Doc::text("version")))
                .unwrap_or_else(Doc::nil);
            if let Some(expr) = version.expr() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments(expr.syntax()))
                    .append(build_expr(expr));
            }
            doc
        }
        ast::XmlRootVersion::XmlVersionNoValue(version) => {
            let mut doc = version
                .version_token()
                .map(|token| leading_comments_token(&token).append(Doc::text("version")))
                .unwrap_or_else(Doc::nil);
            if let Some(no) = version.no_token() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments_token(&no))
                    .append(Doc::text("no"));
            }
            if let Some(value) = version.value_token() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments_token(&value))
                    .append(Doc::text("value"));
            }
            doc
        }
    }
}

fn build_xml_standalone<'a>(standalone: ast::XmlStandalone) -> Doc<'a> {
    let (comma, standalone_token, no_or_yes, value, text) = match standalone {
        ast::XmlStandalone::StandaloneYes(node) => (
            node.comma_token(),
            node.standalone_token(),
            node.yes_token(),
            None,
            "yes",
        ),
        ast::XmlStandalone::StandaloneNo(node) => (
            node.comma_token(),
            node.standalone_token(),
            node.no_token(),
            None,
            "no",
        ),
        ast::XmlStandalone::StandaloneNoValue(node) => (
            node.comma_token(),
            node.standalone_token(),
            node.no_token(),
            node.value_token(),
            "no",
        ),
    };

    let mut doc = comma.map(comments_before).unwrap_or_else(Doc::nil);
    doc = doc.append(Doc::text(",")).append(Doc::line_or_space());
    if let Some(token) = standalone_token {
        doc = doc
            .append(leading_comments_token(&token))
            .append(Doc::text("standalone"));
    }
    if let Some(token) = no_or_yes {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&token))
            .append(Doc::text(text));
    }
    if let Some(token) = value {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&token))
            .append(Doc::text("value"));
    }
    doc
}

fn build_xml_serialize_fn<'a>(xml_serialize_fn: ast::XmlSerializeFn) -> Doc<'a> {
    let mut doc = Doc::text("xmlserialize");
    if let Some(l_paren) = xml_serialize_fn.l_paren_token() {
        doc = doc.append(comments_before(l_paren));
    }
    doc = doc.append(Doc::text("("));

    let mut body = Doc::nil();

    if let Some(kind) = xml_serialize_fn.xml_document_or_content() {
        body = body
            .append(leading_comments(kind.syntax()))
            .append(build_xml_document_or_content(kind));
    }
    if let Some(expr) = xml_serialize_fn.expr() {
        body = body
            .append(Doc::space())
            .append(leading_comments(expr.syntax()))
            .append(build_expr(expr));
    }
    if let Some(as_token) = xml_serialize_fn.as_token() {
        body = body
            .append(Doc::line_or_space())
            .append(leading_comments_token(&as_token))
            .append(Doc::text("as"));
    }
    if let Some(ty) = xml_serialize_fn.ty() {
        body = body
            .append(Doc::space())
            .append(leading_comments(ty.syntax()))
            .append(build_type(ty));
    }
    if let Some(indent) = xml_serialize_fn.xml_indent() {
        body = body
            .append(Doc::line_or_space())
            .append(leading_comments(indent.syntax()))
            .append(build_xml_indent(indent));
    }

    if let Some(r_paren) = xml_serialize_fn.r_paren_token() {
        body = body.append(comments_before(r_paren));
    }
    doc.append(wrap_body(body)).append(Doc::text(")")).group()
}

fn build_xml_document_or_content<'a>(kind: ast::XmlDocumentOrContent) -> Doc<'a> {
    match kind {
        ast::XmlDocumentOrContent::XmlDocument(_) => Doc::text("document"),
        ast::XmlDocumentOrContent::XmlContent(_) => Doc::text("content"),
    }
}

fn build_xml_whitespace<'a>(whitespace: ast::XmlWhitespace) -> Doc<'a> {
    let (first, second, text) = match whitespace {
        ast::XmlWhitespace::PreserveWhitespace(node) => {
            (node.preserve_token(), node.whitespace_token(), "preserve")
        }
        ast::XmlWhitespace::StripWhitespace(node) => {
            (node.strip_token(), node.whitespace_token(), "strip")
        }
    };
    build_two_keywords(first, text, second, "whitespace")
}

fn build_xml_indent<'a>(indent: ast::XmlIndent) -> Doc<'a> {
    match indent {
        ast::XmlIndent::Indent(_) => Doc::text("indent"),
        ast::XmlIndent::NoIndent(node) => {
            build_two_keywords(node.no_token(), "no", node.indent_token(), "indent")
        }
    }
}

fn build_two_keywords<'a>(
    first: Option<SyntaxToken>,
    first_text: &'static str,
    second: Option<SyntaxToken>,
    second_text: &'static str,
) -> Doc<'a> {
    let mut doc = first
        .map(|token| leading_comments_token(&token).append(Doc::text(first_text)))
        .unwrap_or_else(Doc::nil);
    if let Some(token) = second {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&token))
            .append(Doc::text(second_text));
    }
    doc
}

fn build_xml_passing_mech<'a>(mech: ast::XmlPassingMech) -> Doc<'a> {
    let (by, end, text) = match mech {
        ast::XmlPassingMech::XmlPassingMechByRef(mech) => {
            (mech.by_token(), mech.ref_token(), "ref")
        }
        ast::XmlPassingMech::XmlPassingMechByValue(mech) => {
            (mech.by_token(), mech.value_token(), "value")
        }
    };
    let mut doc = by
        .map(|token| leading_comments_token(&token).append(Doc::text("by")))
        .unwrap_or_else(Doc::nil);
    if let Some(end) = end {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&end))
            .append(Doc::text(text));
    }
    doc
}

fn build_json_object_fn<'a>(json_object_fn: ast::JsonObjectFn) -> Doc<'a> {
    let mut doc = Doc::text("json_object");
    if let Some(l_paren) = json_object_fn.l_paren_token() {
        doc = doc.append(comments_before(l_paren));
    }
    doc = doc.append(Doc::text("("));

    let mut body = Doc::nil();

    let exprs = json_object_fn.exprs().map(|expr| {
        (
            leading_comments(expr.syntax()).append(build_expr(expr.clone())),
            expr.syntax().clone(),
        )
    });
    let key_values = json_object_fn.json_key_values().map(|key_value| {
        (
            leading_comments(key_value.syntax()).append(build_json_key_value(key_value.clone())),
            key_value.syntax().clone(),
        )
    });
    let items = build_comma_separated_docs(exprs.chain(key_values));
    let mut has_content = items.is_some();
    if let Some(items) = items {
        body = body.append(items);
    }

    if let Some(null_clause) = json_object_fn.json_null_clause() {
        if has_content {
            body = body.append(Doc::line_or_space());
        }
        body = body
            .append(leading_comments(null_clause.syntax()))
            .append(build_json_null_clause(null_clause));
        has_content = true;
    }
    if let Some(unique) = json_object_fn.json_keys_unique_clause() {
        if has_content {
            body = body.append(Doc::line_or_space());
        }
        body = body
            .append(leading_comments(unique.syntax()))
            .append(build_json_keys_unique_clause(unique));
        has_content = true;
    }
    if let Some(returning) = json_object_fn.json_returning_clause() {
        if has_content {
            body = body.append(Doc::line_or_space());
        }
        body = body
            .append(leading_comments(returning.syntax()))
            .append(build_json_returning_clause(returning));
    }
    if let Some(r_paren) = json_object_fn.r_paren_token() {
        body = body.append(comments_before(r_paren));
    }
    doc.append(wrap_body(body)).append(Doc::text(")")).group()
}

fn build_json_object_agg_fn<'a>(json_object_agg_fn: ast::JsonObjectAggFn) -> Doc<'a> {
    let mut doc = Doc::text("json_objectagg");
    if let Some(l_paren) = json_object_agg_fn.l_paren_token() {
        doc = doc.append(comments_before(l_paren));
    }
    doc = doc.append(Doc::text("("));

    let mut body = Doc::nil();

    if let Some(key_value) = json_object_agg_fn.json_key_value() {
        body = body
            .append(leading_comments(key_value.syntax()))
            .append(build_json_key_value(key_value));
    }
    if let Some(null_clause) = json_object_agg_fn.json_null_clause() {
        body = body
            .append(Doc::line_or_space())
            .append(leading_comments(null_clause.syntax()))
            .append(build_json_null_clause(null_clause));
    }
    if let Some(unique) = json_object_agg_fn.json_keys_unique_clause() {
        body = body
            .append(Doc::line_or_space())
            .append(leading_comments(unique.syntax()))
            .append(build_json_keys_unique_clause(unique));
    }
    if let Some(returning) = json_object_agg_fn.json_returning_clause() {
        body = body
            .append(Doc::line_or_space())
            .append(leading_comments(returning.syntax()))
            .append(build_json_returning_clause(returning));
    }
    if let Some(r_paren) = json_object_agg_fn.r_paren_token() {
        body = body.append(comments_before(r_paren));
    }
    doc.append(wrap_body(body)).append(Doc::text(")")).group()
}

fn build_json_key_value<'a>(key_value: ast::JsonKeyValue) -> Doc<'a> {
    let mut doc = key_value.expr().map(build_expr).unwrap_or_else(Doc::nil);
    if let Some(colon) = key_value.colon_token() {
        doc = doc.append(comments_before(colon)).append(Doc::text(":"));
    } else if let Some(value_token) = key_value.value_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&value_token))
            .append(Doc::text("value"));
    }
    if let Some(value) = key_value.json_value_expr() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(value.syntax()))
            .append(build_json_value_expr(value));
    }
    doc
}

fn build_json_fn<'a>(json_fn: ast::JsonFn) -> Doc<'a> {
    let mut doc = Doc::text("json");
    if let Some(l_paren) = json_fn.l_paren_token() {
        doc = doc.append(comments_before(l_paren));
    }
    doc = doc.append(Doc::text("("));

    let mut body = Doc::nil();

    if let Some(expr) = json_fn.expr() {
        body = body
            .append(leading_comments(expr.syntax()))
            .append(build_expr(expr));
    }
    if let Some(format) = json_fn.json_format_clause() {
        body = body
            .append(Doc::line_or_space())
            .append(leading_comments(format.syntax()))
            .append(build_json_format_clause(format));
    }
    if let Some(unique) = json_fn.json_keys_unique_clause() {
        body = body
            .append(Doc::line_or_space())
            .append(leading_comments(unique.syntax()))
            .append(build_json_keys_unique_clause(unique));
    }
    if let Some(r_paren) = json_fn.r_paren_token() {
        body = body.append(comments_before(r_paren));
    }
    doc.append(wrap_body(body)).append(Doc::text(")")).group()
}

fn build_json_scalar_fn<'a>(json_scalar_fn: ast::JsonScalarFn) -> Doc<'a> {
    build_parenthesized_expr_or_select_fn(
        "json_scalar",
        json_scalar_fn.l_paren_token(),
        json_scalar_fn.expr(),
        None,
        json_scalar_fn.r_paren_token(),
    )
}

fn build_json_serialize_fn<'a>(json_serialize_fn: ast::JsonSerializeFn) -> Doc<'a> {
    let mut doc = Doc::text("json_serialize");
    if let Some(l_paren) = json_serialize_fn.l_paren_token() {
        doc = doc.append(comments_before(l_paren));
    }
    doc = doc.append(Doc::text("("));

    let mut body = Doc::nil();

    if let Some(expr) = json_serialize_fn.expr() {
        body = body
            .append(leading_comments(expr.syntax()))
            .append(build_expr(expr));
    }
    if let Some(format) = json_serialize_fn.json_format_clause() {
        body = body
            .append(Doc::line_or_space())
            .append(leading_comments(format.syntax()))
            .append(build_json_format_clause(format));
    }
    if let Some(returning) = json_serialize_fn.json_returning_clause() {
        body = body
            .append(Doc::line_or_space())
            .append(leading_comments(returning.syntax()))
            .append(build_json_returning_clause(returning));
    }
    if let Some(r_paren) = json_serialize_fn.r_paren_token() {
        body = body.append(comments_before(r_paren));
    }
    doc.append(wrap_body(body)).append(Doc::text(")")).group()
}

fn build_json_query_fn<'a>(json_query_fn: ast::JsonQueryFn) -> Doc<'a> {
    let (doc, mut body) = build_json_document_path_fn(
        "json_query",
        json_query_fn.l_paren_token(),
        json_query_fn.document(),
        json_query_fn.json_format_clause(),
        json_query_fn.comma_token(),
        json_query_fn.path(),
    );
    if let Some(passing) = json_query_fn.json_passing_clause() {
        body = body
            .append(Doc::line_or_space())
            .append(leading_comments(passing.syntax()))
            .append(build_json_passing_clause(passing));
    }
    if let Some(returning) = json_query_fn.json_returning_clause() {
        body = body
            .append(Doc::line_or_space())
            .append(leading_comments(returning.syntax()))
            .append(build_json_returning_clause(returning));
    }
    if let Some(wrapper) = json_query_fn.json_wrapper_behavior_clause() {
        body = body
            .append(Doc::line_or_space())
            .append(leading_comments(wrapper.syntax()))
            .append(build_json_wrapper_behavior_clause(wrapper));
    }
    if let Some(quotes) = json_query_fn.json_quotes_clause() {
        body = body
            .append(Doc::line_or_space())
            .append(leading_comments(quotes.syntax()))
            .append(build_json_quotes_clause(quotes));
    }
    if let Some(on_empty) = json_query_fn.json_on_empty_clause() {
        body = body
            .append(Doc::line_or_space())
            .append(leading_comments(on_empty.syntax()))
            .append(build_json_on_empty_clause(on_empty));
    }
    if let Some(on_error) = json_query_fn.json_on_error_clause() {
        body = body
            .append(Doc::line_or_space())
            .append(leading_comments(on_error.syntax()))
            .append(build_json_on_error_clause(on_error));
    }
    if let Some(r_paren) = json_query_fn.r_paren_token() {
        body = body.append(comments_before(r_paren));
    }
    doc.append(wrap_body(body)).append(Doc::text(")")).group()
}

fn build_json_value_fn<'a>(json_value_fn: ast::JsonValueFn) -> Doc<'a> {
    let (doc, mut body) = build_json_document_path_fn(
        "json_value",
        json_value_fn.l_paren_token(),
        json_value_fn.document(),
        json_value_fn.json_format_clause(),
        json_value_fn.comma_token(),
        json_value_fn.path(),
    );
    if let Some(passing) = json_value_fn.json_passing_clause() {
        body = body
            .append(Doc::line_or_space())
            .append(leading_comments(passing.syntax()))
            .append(build_json_passing_clause(passing));
    }
    if let Some(returning) = json_value_fn.json_returning_clause() {
        body = body
            .append(Doc::line_or_space())
            .append(leading_comments(returning.syntax()))
            .append(build_json_returning_clause(returning));
    }
    if let Some(on_empty) = json_value_fn.json_on_empty_clause() {
        body = body
            .append(Doc::line_or_space())
            .append(leading_comments(on_empty.syntax()))
            .append(build_json_on_empty_clause(on_empty));
    }
    if let Some(on_error) = json_value_fn.json_on_error_clause() {
        body = body
            .append(Doc::line_or_space())
            .append(leading_comments(on_error.syntax()))
            .append(build_json_on_error_clause(on_error));
    }
    if let Some(r_paren) = json_value_fn.r_paren_token() {
        body = body.append(comments_before(r_paren));
    }
    doc.append(wrap_body(body)).append(Doc::text(")")).group()
}

fn build_json_document_path_fn<'a>(
    keyword: &'static str,
    l_paren: Option<SyntaxToken>,
    document: Option<ast::Expr>,
    format: Option<ast::JsonFormatClause>,
    comma: Option<SyntaxToken>,
    path: Option<ast::Expr>,
) -> (Doc<'a>, Doc<'a>) {
    let mut doc = Doc::text(keyword);
    if let Some(l_paren) = l_paren {
        doc = doc.append(comments_before(l_paren));
    }
    doc = doc.append(Doc::text("("));

    let mut body = Doc::nil();
    if let Some(document) = document {
        body = body
            .append(leading_comments(document.syntax()))
            .append(build_expr(document));
    }
    if let Some(format) = format {
        body = body
            .append(Doc::line_or_space())
            .append(leading_comments(format.syntax()))
            .append(build_json_format_clause(format));
    }
    if let Some(comma) = comma {
        body = body
            .append(comments_before(comma))
            .append(Doc::text(","))
            .append(Doc::line_or_space());
    }
    if let Some(path) = path {
        body = body
            .append(leading_comments(path.syntax()))
            .append(build_expr(path));
    }
    (doc, body)
}

fn build_json_exists_fn<'a>(json_exists_fn: ast::JsonExistsFn) -> Doc<'a> {
    let mut doc = Doc::text("json_exists");
    if let Some(l_paren) = json_exists_fn.l_paren_token() {
        doc = doc.append(comments_before(l_paren));
    }
    doc = doc.append(Doc::text("("));

    let mut body = Doc::nil();

    if let Some(document) = json_exists_fn.document() {
        body = body
            .append(leading_comments(document.syntax()))
            .append(build_expr(document));
    }
    if let Some(format) = json_exists_fn.json_format_clause() {
        body = body
            .append(Doc::line_or_space())
            .append(leading_comments(format.syntax()))
            .append(build_json_format_clause(format));
    }
    if let Some(comma) = json_exists_fn.comma_token() {
        body = body
            .append(comments_before(comma))
            .append(Doc::text(","))
            .append(Doc::line_or_space());
    }
    if let Some(path) = json_exists_fn.path() {
        body = body
            .append(leading_comments(path.syntax()))
            .append(build_expr(path));
    }
    if let Some(passing) = json_exists_fn.json_passing_clause() {
        body = body
            .append(Doc::line_or_space())
            .append(leading_comments(passing.syntax()))
            .append(build_json_passing_clause(passing));
    }
    if let Some(on_error) = json_exists_fn.json_on_error_clause() {
        body = body
            .append(Doc::line_or_space())
            .append(leading_comments(on_error.syntax()))
            .append(build_json_on_error_clause(on_error));
    }
    if let Some(r_paren) = json_exists_fn.r_paren_token() {
        body = body.append(comments_before(r_paren));
    }
    doc.append(wrap_body(body)).append(Doc::text(")")).group()
}

fn build_json_passing_clause<'a>(passing: ast::JsonPassingClause) -> Doc<'a> {
    let mut doc = Doc::text("passing");
    let mut args = passing.json_passing_args();
    if let Some(first) = args.next() {
        let mut previous_syntax = first.syntax().clone();
        doc = doc
            .append(Doc::space())
            .append(leading_comments(first.syntax()))
            .append(build_json_passing_arg(first));
        for arg in args {
            doc = doc
                .append(trailing_comments(&previous_syntax))
                .append(Doc::text(","))
                .append(Doc::line_or_space())
                .append(leading_comments(arg.syntax()))
                .append(build_json_passing_arg(arg.clone()));
            previous_syntax = arg.syntax().clone();
        }
    }
    doc.nest(2).group()
}

fn build_json_passing_arg<'a>(arg: ast::JsonPassingArg) -> Doc<'a> {
    let mut doc = arg.expr().map(build_expr).unwrap_or_else(Doc::nil);
    if let Some(as_token) = arg.as_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&as_token))
            .append(Doc::text("as"));
    }
    if let Some(name) = arg.name() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(name.syntax()))
            .append(build_name(name.syntax()));
    }
    doc
}

fn build_json_wrapper_behavior_clause<'a>(clause: ast::JsonWrapperBehaviorClause) -> Doc<'a> {
    match clause {
        ast::JsonWrapperBehaviorClause::JsonWithConditionalWrapper(clause) => {
            let mut doc = Doc::text("with");
            doc = append_keyword_token(doc, clause.conditional_token(), "conditional");
            doc = append_keyword_token(doc, clause.array_token(), "array");
            append_keyword_token(doc, clause.wrapper_token(), "wrapper")
        }
        ast::JsonWrapperBehaviorClause::JsonWithUnconditionalWrapper(clause) => {
            let mut doc = Doc::text("with");
            doc = append_keyword_token(doc, clause.unconditional_token(), "unconditional");
            doc = append_keyword_token(doc, clause.array_token(), "array");
            append_keyword_token(doc, clause.wrapper_token(), "wrapper")
        }
        ast::JsonWrapperBehaviorClause::JsonWithoutWrapper(clause) => {
            let mut doc = Doc::text("without");
            doc = append_keyword_token(doc, clause.array_token(), "array");
            append_keyword_token(doc, clause.wrapper_token(), "wrapper")
        }
    }
}

fn build_json_quotes_clause<'a>(clause: ast::JsonQuotesClause) -> Doc<'a> {
    let mut doc = clause
        .quotes_behavior()
        .map(|behavior| match behavior {
            ast::QuotesBehavior::KeepQuotes(behavior) => {
                append_keyword_token(Doc::text("keep"), behavior.quotes_token(), "quotes")
            }
            ast::QuotesBehavior::OmitQuotes(behavior) => {
                append_keyword_token(Doc::text("omit"), behavior.quotes_token(), "quotes")
            }
        })
        .unwrap_or_else(Doc::nil);
    if let Some(on_scalar) = clause.on_scalar_string() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(on_scalar.syntax()))
            .append(Doc::text("on"));
        doc = append_keyword_token(doc, on_scalar.scalar_token(), "scalar");
        doc = append_keyword_token(doc, on_scalar.string_token(), "string");
    }
    doc
}

fn build_json_on_empty_clause<'a>(clause: ast::JsonOnEmptyClause) -> Doc<'a> {
    let mut doc = clause
        .json_behavior()
        .map(build_json_behavior)
        .unwrap_or_else(Doc::nil);
    doc = append_keyword_token(doc, clause.on_token(), "on");
    append_keyword_token(doc, clause.empty_token(), "empty")
}

fn build_json_on_error_clause<'a>(clause: ast::JsonOnErrorClause) -> Doc<'a> {
    let mut doc = clause
        .json_behavior()
        .map(build_json_behavior)
        .unwrap_or_else(Doc::nil);
    if let Some(on_token) = clause.on_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&on_token))
            .append(Doc::text("on"));
    }
    if let Some(error_token) = clause.error_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&error_token))
            .append(Doc::text("error"));
    }
    doc
}

fn build_json_behavior<'a>(behavior: ast::JsonBehavior) -> Doc<'a> {
    match behavior {
        ast::JsonBehavior::JsonBehaviorDefault(behavior) => {
            let mut doc = Doc::text("default");
            if let Some(expr) = behavior.expr() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments(expr.syntax()))
                    .append(build_expr(expr));
            }
            doc
        }
        ast::JsonBehavior::JsonBehaviorEmptyArray(behavior) => {
            let mut doc = Doc::text("empty");
            if let Some(array_token) = behavior.array_token() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments_token(&array_token))
                    .append(Doc::text("array"));
            }
            doc
        }
        ast::JsonBehavior::JsonBehaviorEmptyObject(behavior) => {
            let mut doc = Doc::text("empty");
            if let Some(object_token) = behavior.object_token() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments_token(&object_token))
                    .append(Doc::text("object"));
            }
            doc
        }
        ast::JsonBehavior::JsonBehaviorError(_) => Doc::text("error"),
        ast::JsonBehavior::JsonBehaviorFalse(_) => Doc::text("false"),
        ast::JsonBehavior::JsonBehaviorNull(_) => Doc::text("null"),
        ast::JsonBehavior::JsonBehaviorTrue(_) => Doc::text("true"),
        ast::JsonBehavior::JsonBehaviorUnknown(_) => Doc::text("unknown"),
    }
}

fn build_json_array_fn<'a>(json_array_fn: ast::JsonArrayFn) -> Doc<'a> {
    let mut doc = Doc::text("json_array");
    if let Some(l_paren) = json_array_fn.l_paren_token() {
        doc = doc.append(comments_before(l_paren));
    }
    doc = doc.append(Doc::text("("));

    let mut body = Doc::nil();

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
    if let Some(items) = build_comma_separated_docs(exprs.chain(selects)) {
        body = body.append(items);
    }

    if let Some(null_clause) = json_array_fn.json_null_clause() {
        body = body
            .append(Doc::line_or_space())
            .append(leading_comments(null_clause.syntax()))
            .append(build_json_null_clause(null_clause));
    }
    if let Some(returning) = json_array_fn.json_returning_clause() {
        body = body
            .append(Doc::line_or_space())
            .append(leading_comments(returning.syntax()))
            .append(build_json_returning_clause(returning));
    }
    if let Some(r_paren) = json_array_fn.r_paren_token() {
        body = body.append(comments_before(r_paren));
    }
    doc.append(wrap_body(body)).append(Doc::text(")")).group()
}

fn build_comma_separated_docs<'a>(
    mut items: impl Iterator<Item = (Doc<'a>, SyntaxNode)>,
) -> Option<Doc<'a>> {
    let (first, mut previous_syntax) = items.next()?;
    let mut docs = vec![first];
    for (item, syntax) in items {
        docs.push(
            trailing_comments(&previous_syntax)
                .append(Doc::text(","))
                .append(Doc::line_or_space())
                .append(item),
        );
        previous_syntax = syntax;
    }
    Some(Doc::list(docs))
}

fn build_json_expr_format<'a>(value: ast::JsonExprFormat) -> Doc<'a> {
    let mut doc = value.expr().map(build_expr).unwrap_or_else(Doc::nil);
    if let Some(format) = value.json_format_clause() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(format.syntax()))
            .append(build_json_format_clause(format));
    }
    doc.group()
}

fn build_json_select_format<'a>(select: ast::JsonSelectFormat) -> Doc<'a> {
    let mut doc = select
        .select_variant()
        .map(build_select_variant)
        .unwrap_or_else(Doc::nil);
    if let Some(format) = select.json_format_clause() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(format.syntax()))
            .append(build_json_format_clause(format));
    }
    doc.group()
}

fn build_json_array_agg_fn<'a>(json_array_agg_fn: ast::JsonArrayAggFn) -> Doc<'a> {
    let mut doc = Doc::text("json_arrayagg");
    if let Some(l_paren) = json_array_agg_fn.l_paren_token() {
        doc = doc.append(comments_before(l_paren));
    }
    doc = doc.append(Doc::text("("));

    let mut body = Doc::nil();

    if let Some(value) = json_array_agg_fn.json_value_expr() {
        body = body
            .append(leading_comments(value.syntax()))
            .append(build_json_value_expr(value));
    }
    if let Some(order_by) = json_array_agg_fn.order_by_clause() {
        body = body
            .append(Doc::line_or_space())
            .append(leading_comments(order_by.syntax()))
            .append(build_order_by_clause(order_by));
    }
    if let Some(null_clause) = json_array_agg_fn.json_null_clause() {
        body = body
            .append(Doc::line_or_space())
            .append(leading_comments(null_clause.syntax()))
            .append(build_json_null_clause(null_clause));
    }
    if let Some(returning) = json_array_agg_fn.json_returning_clause() {
        body = body
            .append(Doc::line_or_space())
            .append(leading_comments(returning.syntax()))
            .append(build_json_returning_clause(returning));
    }
    if let Some(r_paren) = json_array_agg_fn.r_paren_token() {
        body = body.append(comments_before(r_paren));
    }
    doc.append(wrap_body(body)).append(Doc::text(")")).group()
}

fn build_json_value_expr<'a>(value: ast::JsonValueExpr) -> Doc<'a> {
    let mut doc = value.expr().map(build_expr).unwrap_or_else(Doc::nil);
    if let Some(format) = value.json_format_clause() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(format.syntax()))
            .append(build_json_format_clause(format));
    }
    doc.group()
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
            .append(Doc::line_or_space())
            .append(leading_comments(encoding.syntax()))
            .append(build_json_encoding_clause(encoding));
    }
    doc.group()
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
            ("null", clause.on_token(), clause.on_null_token())
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
            .append(Doc::line_or_space())
            .append(leading_comments(format.syntax()))
            .append(build_json_format_clause(format));
    }
    doc.nest(2).group()
}

fn build_overlay_fn<'a>(overlay_fn: ast::OverlayFn) -> Doc<'a> {
    let mut doc = Doc::text("overlay");
    if let Some(l_paren) = overlay_fn.l_paren_token() {
        doc = doc.append(comments_before(l_paren));
    }
    doc = doc.append(Doc::text("("));

    let mut body = Doc::nil();
    if let Some(args) = overlay_fn.overlay_args() {
        body = body
            .append(leading_comments(args.syntax()))
            .append(match args {
                ast::OverlayArgs::OverlayPlacing(args) => {
                    let mut doc = args
                        .string()
                        .map(|expr| leading_comments(expr.syntax()).append(build_expr(expr)))
                        .unwrap_or_else(Doc::nil);
                    doc = append_line_keyword_expr(
                        doc,
                        args.placing_token(),
                        "placing",
                        args.placing(),
                    );
                    doc = append_line_keyword_expr(doc, args.from_token(), "from", args.from());
                    append_line_keyword_expr(doc, args.for_token(), "for", args.for_()).group()
                }
                ast::OverlayArgs::OverlayExprs(args) => {
                    let items = args.overlay_exprs().map(|arg| {
                        let syntax = arg.syntax().clone();
                        let doc = leading_comments(arg.syntax()).append(match arg {
                            ast::OverlayExpr::Expr(expr) => build_expr(expr),
                            ast::OverlayExpr::NamedArg(arg) => build_named_call_arg(arg),
                        });
                        (doc, syntax)
                    });
                    build_comma_separated_docs(items).unwrap_or_else(Doc::nil)
                }
            });
    }

    if let Some(r_paren) = overlay_fn.r_paren_token() {
        body = body.append(comments_before(r_paren));
    }
    doc = doc.append(wrap_body(body));
    doc.append(Doc::text(")")).group()
}

fn build_substring_fn<'a>(substring_fn: ast::SubstringFn) -> Doc<'a> {
    let mut doc = Doc::text("substring");
    if let Some(l_paren) = substring_fn.l_paren_token() {
        doc = doc.append(comments_before(l_paren));
    }
    doc = doc.append(Doc::text("("));

    let mut body = Doc::nil();

    if let Some(args) = substring_fn.substring_args() {
        body = body
            .append(leading_comments(args.syntax()))
            .append(match args {
                ast::SubstringArgs::SubstringForFrom(args) => {
                    let mut body = args.string().map(build_expr).unwrap_or_else(Doc::nil);
                    body = append_line_keyword_expr(body, args.for_token(), "for", args.count());
                    append_line_keyword_expr(body, args.from_token(), "from", args.start())
                }
                ast::SubstringArgs::SubstringFromFor(args) => {
                    let mut body = args.string().map(build_expr).unwrap_or_else(Doc::nil);
                    body = append_line_keyword_expr(body, args.from_token(), "from", args.start());
                    append_line_keyword_expr(body, args.for_token(), "for", args.count())
                }
                ast::SubstringArgs::SubstringSimilarEscape(args) => {
                    let mut body = args.string().map(build_expr).unwrap_or_else(Doc::nil);
                    body = append_line_keyword_expr(
                        body,
                        args.similar_token(),
                        "similar",
                        args.pattern(),
                    );
                    append_line_keyword_expr(body, args.escape_token(), "escape", args.escape())
                }
                ast::SubstringArgs::SubstringExprs(args) => {
                    build_comma_separated_exprs(args.exprs()).unwrap_or_else(Doc::nil)
                }
            });
    }

    if let Some(r_paren) = substring_fn.r_paren_token() {
        body = body.append(comments_before(r_paren));
    }
    doc.append(wrap_body(body)).append(Doc::text(")")).group()
}

fn append_keyword_token<'a>(
    mut doc: Doc<'a>,
    token: Option<SyntaxToken>,
    keyword: &'static str,
) -> Doc<'a> {
    if let Some(token) = token {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&token))
            .append(Doc::text(keyword));
    }
    doc
}

fn append_line_keyword_expr<'a>(
    mut doc: Doc<'a>,
    token: Option<SyntaxToken>,
    keyword: &'static str,
    expr: Option<ast::Expr>,
) -> Doc<'a> {
    if let Some(token) = token {
        doc = doc
            .append(Doc::line_or_space())
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

    let mut body = Doc::nil();

    let has_side = if let Some(side) = trim_fn.trim_side() {
        body = body
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
            body = body.append(Doc::space());
        }
        body = body
            .append(leading_comments(args.syntax()))
            .append(match args {
                ast::TrimArgs::TrimFrom(args) => {
                    let mut body = Doc::text("from");
                    if let Some(exprs) = build_comma_separated_exprs(args.exprs()) {
                        body = body.append(Doc::space()).append(exprs);
                    }
                    body
                }
                ast::TrimArgs::TrimExprFrom(args) => {
                    let mut exprs = args.exprs();
                    let mut body = exprs.next().map(build_expr).unwrap_or_else(Doc::nil);
                    if let Some(from) = args.from_token() {
                        body = body
                            .append(Doc::line_or_space())
                            .append(leading_comments_token(&from))
                            .append(Doc::text("from"));
                    }
                    if let Some(exprs) = build_comma_separated_exprs(exprs) {
                        body = body.append(Doc::space()).append(exprs);
                    }
                    body
                }
                ast::TrimArgs::TrimExprs(args) => {
                    build_comma_separated_exprs(args.exprs()).unwrap_or_else(Doc::nil)
                }
            });
    }

    if let Some(r_paren) = trim_fn.r_paren_token() {
        body = body.append(comments_before(r_paren));
    }
    doc.append(wrap_body(body)).append(Doc::text(")")).group()
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
        Some(
            Doc::list(
                Itertools::intersperse(
                    exprs.into_iter(),
                    Doc::text(",").append(Doc::line_or_space()),
                )
                .collect(),
            )
            .group(),
        )
    }
}

fn build_position_fn<'a>(position_fn: ast::PositionFn) -> Doc<'a> {
    let mut doc = Doc::text("position");
    if let Some(l_paren) = position_fn.l_paren_token() {
        doc = doc.append(comments_before(l_paren));
    }
    doc = doc.append(Doc::text("("));

    let mut body = Doc::nil();

    if let Some(pos) = position_fn.pos() {
        body = body
            .append(leading_comments(pos.syntax()))
            .append(build_expr(pos));
    }
    if let Some(in_token) = position_fn.in_token() {
        body = body
            .append(Doc::line_or_space())
            .append(leading_comments_token(&in_token))
            .append(Doc::text("in"));
    }
    if let Some(string) = position_fn.string() {
        body = body
            .append(Doc::line_or_space())
            .append(leading_comments(string.syntax()))
            .append(build_expr(string));
    }
    if let Some(r_paren) = position_fn.r_paren_token() {
        body = body.append(comments_before(r_paren));
    }
    doc.append(wrap_body(body)).append(Doc::text(")")).group()
}

fn build_collation_for_fn<'a>(collation_for_fn: ast::CollationForFn) -> Doc<'a> {
    let mut doc = Doc::text("collation");
    if let Some(for_token) = collation_for_fn.for_token() {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&for_token))
            .append(Doc::text("for"));
    }
    if let Some(l_paren) = collation_for_fn.l_paren_token() {
        if comment_tokens_before(l_paren.clone()).is_empty() {
            doc = doc.append(Doc::space());
        } else {
            doc = doc.append(comments_before(l_paren));
        }
    } else {
        doc = doc.append(Doc::space());
    }
    doc = doc.append(Doc::text("("));

    let mut body = Doc::nil();
    if let Some(expr) = collation_for_fn.expr() {
        body = body
            .append(leading_comments(expr.syntax()))
            .append(build_expr(expr));
    }
    if let Some(r_paren) = collation_for_fn.r_paren_token() {
        body = body.append(comments_before(r_paren));
    }
    doc = doc.append(wrap_body(body));
    doc.append(Doc::text(")")).group()
}

fn build_extract_fn<'a>(extract_fn: ast::ExtractFn) -> Doc<'a> {
    let mut doc = Doc::text("extract");
    if let Some(l_paren) = extract_fn.l_paren_token() {
        doc = doc.append(comments_before(l_paren));
    }
    doc = doc.append(Doc::text("("));

    let mut body = Doc::nil();

    if let Some(field) = extract_fn.extract_field() {
        body = body
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
        body = body
            .append(Doc::line_or_space())
            .append(leading_comments_token(&from))
            .append(Doc::text("from"));
    }
    if let Some(expr) = extract_fn.expr() {
        body = body
            .append(Doc::space())
            .append(leading_comments(expr.syntax()))
            .append(build_expr(expr));
    }
    if let Some(r_paren) = extract_fn.r_paren_token() {
        body = body.append(comments_before(r_paren));
    }
    doc.append(wrap_body(body)).append(Doc::text(")")).group()
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

    let mut body = Doc::nil();
    if let Some(expr) = expr {
        body = body
            .append(leading_comments(expr.syntax()))
            .append(build_expr(expr));
    } else if let Some(select) = select {
        body = body
            .append(leading_comments(select.syntax()))
            .append(match select {
                ast::SelectVariant::Select(select) => build_select_doc_ungrouped(&select),
                select => build_select_variant(select),
            });
    }

    if let Some(r_paren) = r_paren {
        body = body.append(comments_before(r_paren));
    }
    doc = doc.append(wrap_body(body));
    doc.append(Doc::text(")")).group()
}

fn build_call_arg_list<'a>(arg_list: ast::ArgList) -> Doc<'a> {
    let mut doc = Doc::nil();
    if let Some(l_paren) = arg_list.l_paren_token() {
        doc = doc.append(comments_before(l_paren));
    }
    doc = doc.append(Doc::text("("));

    let mut body = Doc::nil();
    if let Some(star) = arg_list.star_token() {
        body = body
            .append(leading_comments_token(&star))
            .append(Doc::text("*"));
    } else {
        let mut has_quantifier = false;
        if let Some(quantifier) = arg_list.all_or_distinct() {
            has_quantifier = true;
            body = body
                .append(leading_comments(quantifier.syntax()))
                .append(match quantifier {
                    ast::AllOrDistinct::All(_) => Doc::text("all"),
                    ast::AllOrDistinct::Distinct(_) => Doc::text("distinct"),
                });
        }

        let args = arg_list.args().map(|arg| {
            let syntax = arg.syntax().clone();
            let doc = leading_comments(arg.syntax()).append(build_call_arg(arg));
            (doc, syntax)
        });
        if let Some(args) = build_comma_separated_docs(args) {
            if has_quantifier {
                body = body.append(Doc::space());
            }
            body = body.append(args);
        }
    }

    if let Some(r_paren) = arg_list.r_paren_token() {
        body = body.append(comments_before(r_paren));
    }
    doc = doc.append(wrap_body(body));

    doc.append(Doc::text(")")).group()
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
        let body = leading_comments(list.syntax()).append(Doc::list(
            Itertools::intersperse(items, Doc::text(",").append(Doc::line_or_space())).collect(),
        ));
        doc = doc.append(Doc::line_or_space().append(body).nest(2));
    }
    doc.group()
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
        let mut body = leading_comments(expr.syntax())
            .append(build_expr(expr))
            .append(Doc::line_or_space())
            .append(leading_comments_token(&as_token))
            .append(Doc::text("as"))
            .append(Doc::line_or_space())
            .append(leading_comments(ty.syntax()))
            .append(build_type(ty))
            .group();
        if let Some(r_paren) = cast_expr.r_paren_token() {
            body = body.append(comments_before(r_paren));
        }
        doc = doc
            .append(Doc::text("("))
            .append(wrap_body(body))
            .append(Doc::text(")"))
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

    let mut body = Doc::nil();
    if let Some(expr) = paren_expr.expr() {
        body = body
            .append(leading_comments(expr.syntax()))
            .append(match expr {
                ast::Expr::BinExpr(binary) => build_bin_expr(binary),
                expression => build_expr(expression),
            });
    } else if let Some(compound_select) = paren_expr.compound_select() {
        body = body
            .append(leading_comments(compound_select.syntax()))
            .append(build_compound_select(&compound_select));
    } else if let Some(from_item) = paren_expr.from_item() {
        body = body
            .append(leading_comments(from_item.syntax()))
            .append(build_from_item(from_item));
    } else if let Some(join_expr) = paren_expr.join_expr() {
        body = body
            .append(leading_comments(join_expr.syntax()))
            .append(build_join_expr(join_expr));
    } else if let Some(select) = paren_expr.select() {
        body = body
            .append(leading_comments(select.syntax()))
            .append(build_select_doc(&select));
    } else if let Some(table) = paren_expr.table() {
        body = body
            .append(leading_comments(table.syntax()))
            .append(build_table(&table));
    } else if let Some(values) = paren_expr.values() {
        body = body
            .append(leading_comments(values.syntax()))
            .append(build_values(&values));
    } else {
        unreachable!("a parenthesized expression should contain a node")
    }

    if let Some(r_paren) = paren_expr.r_paren_token() {
        body = body.append(comments_before(r_paren));
    }
    doc.append(wrap_body(body)).append(Doc::text(")")).group()
}

fn build_postfix_expr<'a>(postfix_expr: ast::PostfixExpr) -> Doc<'a> {
    let expr = build_expr(postfix_expr.expr().unwrap());
    let op = match postfix_expr.op().unwrap() {
        ast::PostfixOp::AtLocal(n) => {
            build_two_keywords(n.at_token(), "at", n.local_token(), "local")
        }
        ast::PostfixOp::IsNull(_) => Doc::text("isnull"),
        ast::PostfixOp::NotNull(_) => Doc::text("notnull"),
        ast::PostfixOp::IsJson(n) => build_json_postfix(
            [(n.is_token(), "is"), (n.json_token(), "json")],
            n.json_keys_unique_clause(),
        ),
        ast::PostfixOp::IsJsonArray(n) => build_json_postfix(
            [
                (n.is_token(), "is"),
                (n.json_token(), "json"),
                (n.array_token(), "array"),
            ],
            n.json_keys_unique_clause(),
        ),
        ast::PostfixOp::IsJsonObject(n) => build_json_postfix(
            [
                (n.is_token(), "is"),
                (n.json_token(), "json"),
                (n.object_token(), "object"),
            ],
            n.json_keys_unique_clause(),
        ),
        ast::PostfixOp::IsJsonScalar(n) => build_json_postfix(
            [
                (n.is_token(), "is"),
                (n.json_token(), "json"),
                (n.scalar_token(), "scalar"),
            ],
            n.json_keys_unique_clause(),
        ),
        ast::PostfixOp::IsJsonValue(n) => build_json_postfix(
            [
                (n.is_token(), "is"),
                (n.json_token(), "json"),
                (n.value_token(), "value"),
            ],
            n.json_keys_unique_clause(),
        ),
        ast::PostfixOp::IsNormalized(n) => build_normalized_postfix(
            [(n.is_token(), "is")],
            n.unicode_normal_form(),
            n.normalized_token(),
        ),
        ast::PostfixOp::IsNotJson(n) => build_json_postfix(
            [
                (n.is_token(), "is"),
                (n.not_token(), "not"),
                (n.json_token(), "json"),
            ],
            n.json_keys_unique_clause(),
        ),
        ast::PostfixOp::IsNotJsonArray(n) => build_json_postfix(
            [
                (n.is_token(), "is"),
                (n.not_token(), "not"),
                (n.json_token(), "json"),
                (n.array_token(), "array"),
            ],
            n.json_keys_unique_clause(),
        ),
        ast::PostfixOp::IsNotJsonObject(n) => build_json_postfix(
            [
                (n.is_token(), "is"),
                (n.not_token(), "not"),
                (n.json_token(), "json"),
                (n.object_token(), "object"),
            ],
            n.json_keys_unique_clause(),
        ),
        ast::PostfixOp::IsNotJsonScalar(n) => build_json_postfix(
            [
                (n.is_token(), "is"),
                (n.not_token(), "not"),
                (n.json_token(), "json"),
                (n.scalar_token(), "scalar"),
            ],
            n.json_keys_unique_clause(),
        ),
        ast::PostfixOp::IsNotJsonValue(n) => build_json_postfix(
            [
                (n.is_token(), "is"),
                (n.not_token(), "not"),
                (n.json_token(), "json"),
                (n.value_token(), "value"),
            ],
            n.json_keys_unique_clause(),
        ),
        ast::PostfixOp::IsNotNormalized(n) => build_normalized_postfix(
            [(n.is_token(), "is"), (n.not_token(), "not")],
            n.unicode_normal_form(),
            n.normalized_token(),
        ),
    };
    expr.append(Doc::space()).append(op)
}

fn build_postfix_keywords<'a>(
    keywords: impl IntoIterator<Item = (Option<SyntaxToken>, &'static str)>,
) -> Doc<'a> {
    let mut doc = Doc::nil();
    let mut has_keyword = false;
    for (token, text) in keywords {
        let Some(token) = token else {
            continue;
        };
        if has_keyword {
            doc = doc.append(Doc::space());
        }
        doc = doc
            .append(leading_comments_token(&token))
            .append(Doc::text(text));
        has_keyword = true;
    }
    doc
}

fn build_json_postfix<'a>(
    keywords: impl IntoIterator<Item = (Option<SyntaxToken>, &'static str)>,
    clause: Option<ast::JsonKeysUniqueClause>,
) -> Doc<'a> {
    let mut doc = build_postfix_keywords(keywords);
    if let Some(clause) = clause {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(clause.syntax()))
            .append(build_json_keys_unique_clause(clause));
    }
    doc
}

fn build_normalized_postfix<'a>(
    keywords: impl IntoIterator<Item = (Option<SyntaxToken>, &'static str)>,
    form: Option<ast::UnicodeNormalForm>,
    normalized_token: Option<SyntaxToken>,
) -> Doc<'a> {
    let mut doc = build_postfix_keywords(keywords);
    if let Some(form) = form {
        doc = doc
            .append(Doc::space())
            .append(leading_comments(form.syntax()))
            .append(build_unicode_normal_form(form));
    }
    append_keyword_token(doc, normalized_token, "normalized")
}

#[derive(Clone, Copy, PartialEq)]
enum LogicalOp {
    And,
    Or,
}

fn logical_op(op: &ast::BinOp) -> Option<LogicalOp> {
    match op {
        ast::BinOp::And(_) => Some(LogicalOp::And),
        ast::BinOp::Or(_) => Some(LogicalOp::Or),
        _ => None,
    }
}

fn build_logical_expr<'a>(bin_expr: ast::BinExpr, logical: LogicalOp) -> Doc<'a> {
    let lhs = bin_expr.lhs().unwrap();
    let rhs = bin_expr.rhs().unwrap();
    let lhs_doc = match lhs.clone() {
        ast::Expr::BinExpr(inner) if inner.op().as_ref().and_then(logical_op) == Some(logical) => {
            build_logical_expr(inner, logical)
        }
        lhs => build_expr(lhs),
    };
    let rhs_doc = match rhs.clone() {
        ast::Expr::BinExpr(inner) if inner.op().as_ref().and_then(logical_op) == Some(logical) => {
            build_logical_expr(inner, logical)
        }
        rhs => build_expr(rhs),
    };

    lhs_doc
        .append(trailing_comments(lhs.syntax()))
        .append(Doc::line_or_space())
        .append(build_op(bin_expr.op().unwrap()))
        .append(Doc::space())
        .append(leading_comments(rhs.syntax()))
        .append(rhs_doc)
}

fn build_bin_expr<'a>(bin_expr: ast::BinExpr) -> Doc<'a> {
    if let Some(logical) = bin_expr.op().as_ref().and_then(logical_op) {
        return build_logical_expr(bin_expr, logical).nest(2).group();
    }

    let lhs = bin_expr.lhs().unwrap();
    let rhs = bin_expr.rhs().unwrap();
    let before_op = trailing_comments(lhs.syntax());
    let after_op = leading_comments(rhs.syntax());
    let rhs_is_uncommented_quantifier = comment_tokens_before(rhs.syntax().clone()).is_empty()
        && match &rhs {
            ast::Expr::CallExpr(call) => {
                call.all_fn().is_some() || call.any_fn().is_some() || call.some_fn().is_some()
            }
            _ => false,
        };

    let doc = build_expr(lhs)
        .append(before_op)
        .append(if rhs_is_uncommented_quantifier {
            Doc::space()
        } else {
            Doc::line_or_space()
        })
        .append(build_op(bin_expr.op().unwrap()))
        .append(Doc::space())
        .append(after_op)
        .append(build_expr(rhs));
    if rhs_is_uncommented_quantifier {
        doc
    } else {
        doc.nest(2).group()
    }
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

    let mut body = Doc::nil();
    if let Some(order_by) = within_clause.order_by_clause() {
        body = body
            .append(leading_comments(order_by.syntax()))
            .append(build_order_by_clause(order_by));
    }
    if let Some(r_paren) = within_clause.r_paren_token() {
        body = body.append(comments_before(r_paren));
    }
    doc.append(wrap_body(body)).append(Doc::text(")")).group()
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
    let doc = Doc::text("(");
    let mut body = Doc::nil();
    if let Some(window_spec) = over_window_spec.window_spec() {
        body = body
            .append(leading_comments(window_spec.syntax()))
            .append(build_window_spec(window_spec));
    }
    if let Some(r_paren) = over_window_spec.r_paren_token() {
        body = body.append(comments_before(r_paren));
    }
    doc.append(wrap_body(body)).append(Doc::text(")")).group()
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

    Doc::list(Itertools::intersperse(parts.into_iter(), Doc::line_or_space()).collect()).group()
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
            .append(Doc::line_or_space())
            .append(leading_comments(exclude.syntax()))
            .append(build_frame_exclude(exclude));
    }
    doc.nest(2).group()
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
                    .append(Doc::line_or_space())
                    .append(leading_comments_token(&and_token))
                    .append(Doc::text("and"));
            }
            if let Some(end) = between.end() {
                doc = doc
                    .append(Doc::space())
                    .append(leading_comments(end.syntax()))
                    .append(build_frame_bound(end));
            }
            doc.nest(2).group()
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

    let mut body = Doc::nil();
    if let Some(where_token) = filter_clause.where_token() {
        body = body
            .append(leading_comments_token(&where_token))
            .append(Doc::text("where"));
    }
    if let Some(expr) = filter_clause.expr() {
        body = body
            .append(Doc::space())
            .append(leading_comments(expr.syntax()))
            .append(build_expr(expr));
    }
    if let Some(r_paren) = filter_clause.r_paren_token() {
        body = body.append(comments_before(r_paren));
    }
    doc.append(wrap_body(body)).append(Doc::text(")")).group()
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
    let (prefix, unique_token, keys_token) = match clause {
        ast::JsonKeysUniqueClause::JsonWithoutUniqueKeys(clause) => {
            ("without", clause.unique_token(), clause.keys_token())
        }
        ast::JsonKeysUniqueClause::JsonWithUniqueKeys(clause) => {
            ("with", clause.unique_token(), clause.keys_token())
        }
    };

    let mut doc = Doc::text(prefix);
    if let Some(unique_token) = unique_token {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&unique_token))
            .append(Doc::text("unique"));
    }
    if let Some(keys_token) = keys_token {
        doc = doc
            .append(Doc::space())
            .append(leading_comments_token(&keys_token))
            .append(Doc::text("keys"));
    }
    doc
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
        LitKind::Off(_) => Doc::text("off"),
        LitKind::On(_) => Doc::text("on"),
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
