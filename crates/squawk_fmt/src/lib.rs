use itertools::Itertools;
use squawk_syntax::ast::{self, AstNode};
use tiny_pretty::Doc;
use tiny_pretty::{PrintOptions, print};

fn build_source_file(source_file: &ast::SourceFile) -> Doc<'_> {
    let mut doc = Doc::nil();
    for stmt in source_file.stmts() {
        match stmt {
            ast::Stmt::Select(select) => {
                doc = doc.append(build_select_doc(select));
            }
            ast::Stmt::CreateTable(create_table) => {
                doc = doc.append(build_create_table(create_table))
            }
            _ => (),
        }
        doc = doc
            .append(Doc::text(";"))
            .append(Doc::empty_line())
            .append(Doc::empty_line());
    }
    doc
}

fn build_create_table<'a>(create_table: ast::CreateTable) -> Doc<'a> {
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
                        Doc::text(",").append(Doc::line_or_space()),
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

fn build_select_doc<'a>(select: ast::Select) -> Doc<'a> {
    let mut doc = Doc::text("select").append(Doc::space());

    if let Some(targets) = select
        .select_clause()
        .and_then(|x| x.target_list())
        .map(|x| x.targets())
    {
        doc = doc
            .append(
                Doc::line_or_nil().append(Doc::list(
                    Itertools::intersperse(
                        targets.flat_map(|x| Some(Doc::text(x.expr()?.syntax().to_string()))),
                        Doc::text(",").append(Doc::line_or_space()),
                    )
                    .collect(),
                )),
            )
            .nest(2);
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

pub fn fmt(text: &str) -> String {
    let parse = ast::SourceFile::parse(text);
    let file = parse.tree();
    let doc = build_source_file(&file);
    print(&doc, &PrintOptions::default())
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn select() {
        assert_snapshot!(fmt("
select a(), date_trunc(1, 2), foo(), avg(a - b), bar(carrot), buzz(potato), foo.b from t group by c;
create table t(a int, b text);
"), @r"
        select 
          a(),
          date_trunc(1, 2),
          foo(),
          avg(a - b),
          bar(carrot),
          buzz(potato),
          foo.b
        from t
        group by c;

        create table t(a int, b text);
        ");
    }
}
