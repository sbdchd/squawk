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
                        ast::Stmt::AlterPublication(_) => todo!(),
                        ast::Stmt::AlterRole(_) => todo!(),
                        ast::Stmt::AlterRoutine(_) => todo!(),
                        ast::Stmt::AlterRule(_) => todo!(),
                        ast::Stmt::AlterSchema(_) => todo!(),
                        ast::Stmt::AlterSequence(_) => todo!(),
                        ast::Stmt::AlterServer(_) => todo!(),
                        ast::Stmt::AlterStatistics(_) => todo!(),
                        ast::Stmt::AlterSubscription(_) => todo!(),
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
                        ast::Stmt::Analyze(_) => todo!(),
                        ast::Stmt::Begin(_) => todo!(),
                        ast::Stmt::Call(_) => todo!(),
                        ast::Stmt::Checkpoint(_) => todo!(),
                        ast::Stmt::Close(_) => todo!(),
                        ast::Stmt::Cluster(_) => todo!(),
                        ast::Stmt::CommentOn(_) => todo!(),
                        ast::Stmt::CompoundSelect(compound_select) => {
                            doc = doc.append(build_compound_select(&compound_select));
                        }
                        ast::Stmt::Copy(_) => todo!(),
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
                        ast::Stmt::CreateForeignTable(_) => todo!(),
                        ast::Stmt::CreateFunction(_) => todo!(),
                        ast::Stmt::CreateGroup(_) => todo!(),
                        ast::Stmt::CreateIndex(_) => todo!(),
                        ast::Stmt::CreateLanguage(_) => todo!(),
                        ast::Stmt::CreateMaterializedView(_) => todo!(),
                        ast::Stmt::CreateOperator(_) => todo!(),
                        ast::Stmt::CreateOperatorClass(_) => todo!(),
                        ast::Stmt::CreateOperatorFamily(_) => todo!(),
                        ast::Stmt::CreatePolicy(_) => todo!(),
                        ast::Stmt::CreateProcedure(_) => todo!(),
                        ast::Stmt::CreatePropertyGraph(_) => todo!(),
                        ast::Stmt::CreatePublication(_) => todo!(),
                        ast::Stmt::CreateRole(_) => todo!(),
                        ast::Stmt::CreateRule(_) => todo!(),
                        ast::Stmt::CreateSchema(_) => todo!(),
                        ast::Stmt::CreateSequence(_) => todo!(),
                        ast::Stmt::CreateServer(_) => todo!(),
                        ast::Stmt::CreateStatistics(_) => todo!(),
                        ast::Stmt::CreateSubscription(_) => todo!(),
                        ast::Stmt::CreateTableAs(_) => todo!(),
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
                        ast::Stmt::CreateView(_) => todo!(),
                        ast::Stmt::Deallocate(_) => todo!(),
                        ast::Stmt::Declare(_) => todo!(),
                        ast::Stmt::Delete(_) => todo!(),
                        ast::Stmt::Discard(_) => todo!(),
                        ast::Stmt::Do(_) => todo!(),
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
                        ast::Stmt::DropPublication(_) => todo!(),
                        ast::Stmt::DropRole(_) => todo!(),
                        ast::Stmt::DropRoutine(_) => todo!(),
                        ast::Stmt::DropRule(_) => todo!(),
                        ast::Stmt::DropSchema(_) => todo!(),
                        ast::Stmt::DropSequence(_) => todo!(),
                        ast::Stmt::DropServer(_) => todo!(),
                        ast::Stmt::DropStatistics(_) => todo!(),
                        ast::Stmt::DropSubscription(_) => todo!(),
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
                        ast::Stmt::EmptyStmt(empty_stmt) => {
                            doc = doc.append(build_empty_stmt(&empty_stmt));
                        }
                        ast::Stmt::Execute(_) => todo!(),
                        ast::Stmt::Explain(_) => todo!(),
                        ast::Stmt::Fetch(_) => todo!(),
                        ast::Stmt::Grant(_) => todo!(),
                        ast::Stmt::ImportForeignSchema(_) => todo!(),
                        ast::Stmt::Insert(_) => todo!(),
                        ast::Stmt::Listen(_) => todo!(),
                        ast::Stmt::Load(_) => todo!(),
                        ast::Stmt::Lock(_) => todo!(),
                        ast::Stmt::Merge(_) => todo!(),
                        ast::Stmt::Move(_) => todo!(),
                        ast::Stmt::Notify(_) => todo!(),
                        ast::Stmt::ParenSelect(paren_select) => {
                            doc = doc.append(build_paren_select(paren_select));
                        }
                        ast::Stmt::Prepare(_) => todo!(),
                        ast::Stmt::PrepareTransaction(_) => todo!(),
                        ast::Stmt::Reassign(_) => todo!(),
                        ast::Stmt::Refresh(_) => todo!(),
                        ast::Stmt::Reindex(_) => todo!(),
                        ast::Stmt::ReleaseSavepoint(_) => todo!(),
                        ast::Stmt::Repack(_) => todo!(),
                        ast::Stmt::Reset(_) => todo!(),
                        ast::Stmt::ResetRole(_) => todo!(),
                        ast::Stmt::ResetSessionAuth(_) => todo!(),
                        ast::Stmt::Revoke(_) => todo!(),
                        ast::Stmt::SavepointCreate(_) => todo!(),
                        ast::Stmt::SecurityLabel(_) => todo!(),
                        ast::Stmt::SelectInto(select_into) => {
                            doc = doc.append(build_select_into(&select_into));
                        }
                        ast::Stmt::Set(_) => todo!(),
                        ast::Stmt::SetConstraints(_) => todo!(),
                        ast::Stmt::SetRole(_) => todo!(),
                        ast::Stmt::SetSessionAuth(_) => todo!(),
                        ast::Stmt::SetTransaction(_) => todo!(),
                        ast::Stmt::Show(_) => todo!(),
                        ast::Stmt::Table(table) => {
                            doc = doc.append(build_table(&table));
                        }
                        ast::Stmt::Truncate(_) => todo!(),
                        ast::Stmt::Unlisten(_) => todo!(),
                        ast::Stmt::Update(_) => todo!(),
                        ast::Stmt::Vacuum(_) => todo!(),
                        ast::Stmt::Values(values) => {
                            doc = doc.append(build_values(&values));
                        }
                        ast::Stmt::Commit(_) => todo!(),
                        ast::Stmt::Rollback(_) => todo!(),
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

fn build_empty_stmt<'a>(empty_stmt: &ast::EmptyStmt) -> Doc<'a> {
    build_semicolon(empty_stmt.semicolon_token())
}

fn build_create_table<'a>(create_table: &ast::CreateTable) -> Doc<'a> {
    if create_table.if_not_exists().is_some() {
        todo!("create table if not exists clauses are not supported yet")
    }
    if create_table.inherits().is_some() {
        todo!("create table inherits clauses are not supported yet")
    }
    if create_table.of_type().is_some() {
        todo!("create table of type clauses are not supported yet")
    }
    if create_table.on_commit().is_some() {
        todo!("create table on commit clauses are not supported yet")
    }
    if create_table.partition_by().is_some() {
        todo!("create table partition by clauses are not supported yet")
    }
    if create_table.partition_of().is_some() {
        todo!("create table partition of clauses are not supported yet")
    }
    if create_table.partition_type().is_some() {
        todo!("create table partition types are not supported yet")
    }
    if create_table.persistence().is_some() {
        todo!("create table persistence options are not supported yet")
    }
    if create_table.table_params().is_some() {
        todo!("create table parameters are not supported yet")
    }
    if create_table.tablespace_clause().is_some() {
        todo!("create table tablespace clauses are not supported yet")
    }
    if create_table.using_method().is_some() {
        todo!("create table access methods are not supported yet")
    }

    let table_name = create_table.table_name().unwrap();
    let arg_list = create_table.table_arg_list().unwrap();
    let mut doc = Doc::text("create")
        .append(Doc::space())
        .append(Doc::text("table"))
        .append(Doc::space())
        .append(leading_comments(table_name.syntax()))
        .append(build_path(&table_name.path().unwrap()));
    if let Some(l_paren) = arg_list.l_paren_token() {
        if comment_tokens_before(l_paren.clone()).is_empty() {
            doc = doc.append(Doc::space());
        } else {
            doc = doc.append(comments_before(l_paren));
        }
    }
    doc = doc
        .append(Doc::text("("))
        .append(
            wrap_body(Doc::list(
                Itertools::intersperse(
                    arg_list.args().map(build_table_arg),
                    Doc::text(",").append(Doc::hard_line()),
                )
                .collect(),
            ))
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
            if column.alter_option_list().is_some() {
                todo!("column alter options are not supported yet")
            }
            if column.collate().is_some() {
                todo!("column collations are not supported yet")
            }
            if column.compression_method().is_some() {
                todo!("column compression methods are not supported yet")
            }
            if column.constraints().next().is_some() {
                todo!("column constraints are not supported yet")
            }
            if column.storage().is_some() {
                todo!("column storage options are not supported yet")
            }
            if column.with_options().is_some() {
                todo!("column with options are not supported yet")
            }

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
        ast::TableArg::TableConstraint(table_constraint) => {
            build_table_constraint(table_constraint.clone())
        }
    });
    doc.append(trailing_comments(arg.syntax()))
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
    if values.with_clause().is_some() {
        todo!("values with clauses are not supported yet")
    }
    if values.locking_clauses().next().is_some() {
        todo!("values locking clauses are not supported yet")
    }
    if values.limit_clause().is_some() {
        todo!("values limit clauses are not supported yet")
    }
    if values.fetch_clause().is_some() {
        todo!("values fetch clauses are not supported yet")
    }
    if values.offset_clause().is_some() {
        todo!("values offset clauses are not supported yet")
    }

    let mut doc = Doc::text("values");
    if let Some(row_list) = values.row_list() {
        let rows = row_list.rows().map(|row| {
            (
                leading_comments(row.syntax()).append(build_row(row.clone())),
                row.syntax().clone(),
            )
        });
        if let Some(rows) = build_comma_separated_docs(rows) {
            doc = doc
                .append(
                    Doc::line_or_space()
                        .append(leading_comments(row_list.syntax()))
                        .append(rows)
                        .nest(2),
                )
                .group();
        }
    }
    if let Some(order_by) = values.order_by_clause() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(order_by.syntax()))
            .append(build_order_by_clause(order_by));
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
    if table.with_clause().is_some() {
        todo!("table with clauses are not supported yet")
    }
    if table.locking_clauses().next().is_some() {
        todo!("table locking clauses are not supported yet")
    }
    if table.limit_clause().is_some() {
        todo!("table limit clauses are not supported yet")
    }
    if table.fetch_clause().is_some() {
        todo!("table fetch clauses are not supported yet")
    }
    if table.offset_clause().is_some() {
        todo!("table offset clauses are not supported yet")
    }

    let mut doc = Doc::text("table");
    if let Some(relation) = table.relation_name() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(relation.syntax()))
            .append(build_relation_name(relation))
            .nest(2)
            .group();
    }
    if let Some(order_by) = table.order_by_clause() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(order_by.syntax()))
            .append(build_order_by_clause(order_by));
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
    if select_into.with_clause().is_some() {
        todo!("select into with clauses are not supported yet")
    }
    if select_into.where_clause().is_some() {
        todo!("select into where clauses are not supported yet")
    }
    if select_into.having_clause().is_some() {
        todo!("select into having clauses are not supported yet")
    }
    if select_into.window_clause().is_some() {
        todo!("select into window clauses are not supported yet")
    }
    if select_into.locking_clauses().next().is_some() {
        todo!("select into locking clauses are not supported yet")
    }
    if select_into.limit_clause().is_some() {
        todo!("select into limit clauses are not supported yet")
    }
    if select_into.offset_clause().is_some() {
        todo!("select into offset clauses are not supported yet")
    }
    if select_into.filter_clause().is_some() {
        todo!("select into filter clauses are not supported yet")
    }

    let mut select_body = Doc::nil();
    if let Some(select_clause) = select_into.select_clause() {
        match select_clause.select_quantifier() {
            Some(ast::SelectQuantifier::DistinctClause(distinct_clause)) => {
                if distinct_clause.distinct_on().is_some() {
                    todo!("select into distinct on clauses are not supported yet")
                }
                select_body = select_body
                    .append(leading_comments(distinct_clause.syntax()))
                    .append(Doc::text("distinct"))
                    .append(Doc::space());
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
    let mut doc = Doc::text("select")
        .append(Doc::line_or_space().append(select_body).nest(2))
        .group();

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
    if let Some(group) = select_into.group_by_clause() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(group.syntax()))
            .append(build_select_group_by_clause(group));
    }
    if let Some(order_by) = select_into.order_by_clause() {
        doc = doc
            .append(Doc::line_or_space())
            .append(leading_comments(order_by.syntax()))
            .append(build_order_by_clause(order_by));
    }
    doc.append(build_semicolon(select_into.semicolon_token()))
        .group()
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

fn build_select_doc<'a>(select: &ast::Select) -> Doc<'a> {
    build_select_doc_ungrouped(select).group()
}

fn build_select_doc_ungrouped<'a>(select: &ast::Select) -> Doc<'a> {
    if select.with_clause().is_some() {
        todo!("select with clauses are not supported yet")
    }
    if select.where_clause().is_some() {
        todo!("select where clauses are not supported yet")
    }
    if select.having_clause().is_some() {
        todo!("select having clauses are not supported yet")
    }
    if select.window_clause().is_some() {
        todo!("select window clauses are not supported yet")
    }
    if select.order_by_clause().is_some() {
        todo!("select order by clauses are not supported yet")
    }
    if select.locking_clauses().next().is_some() {
        todo!("select locking clauses are not supported yet")
    }
    if select.limit_clause().is_some() {
        todo!("select limit clauses are not supported yet")
    }
    if select.fetch_clause().is_some() {
        todo!("select fetch clauses are not supported yet")
    }
    if select.offset_clause().is_some() {
        todo!("select offset clauses are not supported yet")
    }
    if select.filter_clause().is_some() {
        todo!("select filter clauses are not supported yet")
    }

    let mut doc = Doc::text("select").append(Doc::line_or_space());

    if let Some(select_clause) = select.select_clause() {
        match select_clause.select_quantifier() {
            Some(ast::SelectQuantifier::DistinctClause(distinct_clause)) => {
                if distinct_clause.distinct_on().is_some() {
                    todo!("select distinct on clauses are not supported yet")
                }
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

    doc
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
    if select.with_clause().is_some() {
        todo!("parenthesized select with clauses are not supported yet")
    }
    if select.order_by_clause().is_some() {
        todo!("parenthesized select order by clauses are not supported yet")
    }
    if select.locking_clauses().next().is_some() {
        todo!("parenthesized select locking clauses are not supported yet")
    }
    if select.limit_clause().is_some() {
        todo!("parenthesized select limit clauses are not supported yet")
    }
    if select.offset_clause().is_some() {
        todo!("parenthesized select offset clauses are not supported yet")
    }
    if select.fetch_clause().is_some() {
        todo!("parenthesized select fetch clauses are not supported yet")
    }

    let mut doc = select
        .l_paren_token()
        .map(comments_before)
        .unwrap_or_else(Doc::nil)
        .append(Doc::text("("));
    let mut body = select
        .select()
        .map(|select| leading_comments(select.syntax()).append(build_select_variant(select)))
        .unwrap_or_else(Doc::nil);
    if let Some(r_paren) = select.r_paren_token() {
        body = body.append(comments_before(r_paren));
    }
    doc = doc.append(wrap_body(body)).append(Doc::text(")")).group();
    doc.append(build_semicolon(select.semicolon_token()))
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
                ast::Expr::BinExpr(binary) => build_bin_expr_doc(binary, false),
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
        doc = doc
            .append(Doc::space())
            .append(leading_comments(expr.syntax()))
            .append(build_expr(expr));
    }
    doc
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
                ast::Expr::BinExpr(binary) => build_bin_expr_doc(binary, false),
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
    } else if let Some(_join_expr) = paren_expr.join_expr() {
        todo!("parenthesized join expression nodes are not supported yet")
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
    build_bin_expr_doc(bin_expr, true)
}

fn build_bin_expr_doc<'a>(bin_expr: ast::BinExpr, wrap: bool) -> Doc<'a> {
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
    if rhs_is_uncommented_quantifier || !wrap {
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
