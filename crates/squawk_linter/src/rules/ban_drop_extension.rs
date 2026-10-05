use crate::{Linter, Rule, Violation};
use squawk_syntax::{
    Parse, SourceFile,
    ast::{self, AstNode},
};

pub(crate) fn ban_drop_extension(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    for stmt in parse.tree().stmts() {
        if let ast::Stmt::DropExtension(node) = stmt {
            ctx.report(Violation::for_node(
                Rule::BanDropExtension,
                "Dropping an extension also removes its functions, types, and other objects, which may break existing clients.".into(),
                node.syntax(),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{lint_errors, lint_ok};

    #[test]
    fn drop_extension() {
        let errors = lint_errors("DROP EXTENSION IF EXISTS hstore;", Rule::BanDropExtension);
        assert!(errors.contains(
            "warning[ban-drop-extension]: Dropping an extension also removes its functions, types, and other objects, which may break existing clients."
        ));
        lint_ok("CREATE EXTENSION hstore;", Rule::BanDropExtension);
    }
}
