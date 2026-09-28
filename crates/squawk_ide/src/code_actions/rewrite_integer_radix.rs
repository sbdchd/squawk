use num_bigint::BigUint;
use rowan::TextSize;
use salsa::Database as Db;
use squawk_linter::Edit;
use squawk_syntax::{
    SyntaxKind,
    ast::{self, AstNode},
};

use crate::{file::InFile, offsets::token_from_offset};

use super::{ActionKind, CodeAction};

const INTEGER_RADIXES: [ast::IntegerRadix; 4] = [
    ast::IntegerRadix::Binary,
    ast::IntegerRadix::Octal,
    ast::IntegerRadix::Decimal,
    ast::IntegerRadix::Hexadecimal,
];

fn format_integer(radix: ast::IntegerRadix, value: &BigUint) -> String {
    let mut digits = value.to_str_radix(radix.base());
    if radix == ast::IntegerRadix::Hexadecimal {
        digits.make_ascii_uppercase();
    }
    let prefix = match radix {
        ast::IntegerRadix::Binary => "0b",
        ast::IntegerRadix::Decimal => "",
        ast::IntegerRadix::Hexadecimal => "0x",
        ast::IntegerRadix::Octal => "0o",
    };
    format!("{prefix}{digits}")
}

fn integer_radix_name(radix: ast::IntegerRadix) -> &'static str {
    match radix {
        ast::IntegerRadix::Binary => "binary",
        ast::IntegerRadix::Decimal => "decimal",
        ast::IntegerRadix::Hexadecimal => "hexadecimal",
        ast::IntegerRadix::Octal => "octal",
    }
}

pub(super) fn rewrite_integer_radix(
    db: &dyn Db,
    position: InFile<TextSize>,
    actions: &mut Vec<CodeAction>,
) -> Option<()> {
    let token = token_from_offset(db, position)?;
    if token.kind() != SyntaxKind::INT_NUMBER {
        return None;
    }

    let literal = token.parent().and_then(ast::Literal::cast)?;
    let source_radix = literal.integer_radix()?;
    let value = literal.integer_value()?;
    for target_radix in INTEGER_RADIXES {
        if target_radix == source_radix {
            continue;
        }

        let replacement = format_integer(target_radix, &value);
        actions.push(CodeAction {
            title: format!("Rewrite integer as {}", integer_radix_name(target_radix)),
            edits: vec![Edit::replace(token.text_range(), replacement)],
            kind: ActionKind::RefactorRewrite,
        });
    }

    Some(())
}

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;

    use crate::{
        code_actions::test_utils::{
            apply_code_action, code_action_not_applicable, code_action_not_applicable_with_errors,
        },
        test_utils::Fixture,
    };

    use super::rewrite_integer_radix;

    fn available_actions(sql: &str) -> String {
        let fixture = Fixture::new(sql);
        let mut actions = vec![];
        rewrite_integer_radix(fixture.db(), fixture.marker().offset_before(), &mut actions);

        actions
            .iter()
            .map(|action| {
                let edit = action.edits.first().expect("expected edit");
                let replacement = edit.text.as_deref().expect("expected replacement");
                format!("{} -> {replacement}", action.title)
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[test]
    fn rewrite_decimal_integer() {
        assert_snapshot!(available_actions("select 100$00;"), @"
        Rewrite integer as binary -> 0b1111101000
        Rewrite integer as octal -> 0o1750
        Rewrite integer as hexadecimal -> 0x3E8
        ");
    }

    #[test]
    fn rewrite_binary_integer() {
        assert_snapshot!(available_actions("select 0b1111101000$0;"), @"
        Rewrite integer as octal -> 0o1750
        Rewrite integer as decimal -> 1000
        Rewrite integer as hexadecimal -> 0x3E8
        ");
    }

    #[test]
    fn rewrite_octal_integer() {
        assert_snapshot!(available_actions("select 0o1750$0;"), @"
        Rewrite integer as binary -> 0b1111101000
        Rewrite integer as decimal -> 1000
        Rewrite integer as hexadecimal -> 0x3E8
        ");
    }

    #[test]
    fn rewrite_hexadecimal_integer() {
        assert_snapshot!(available_actions("select 0x3E8$0;"), @"
        Rewrite integer as binary -> 0b1111101000
        Rewrite integer as octal -> 0o1750
        Rewrite integer as decimal -> 1000
        ");
    }

    #[test]
    fn rewrite_integer_with_separators_and_uppercase_prefix() {
        assert_snapshot!(available_actions("select 0X_FF_FF$0;"), @"
        Rewrite integer as binary -> 0b1111111111111111
        Rewrite integer as octal -> 0o177777
        Rewrite integer as decimal -> 65535
        ");
    }

    #[test]
    fn rewrite_arbitrarily_large_integer() {
        assert_snapshot!(available_actions(
            "select 340282366920938463463374607431768211456$0;"
        ), @"
        Rewrite integer as binary -> 0b100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
        Rewrite integer as octal -> 0o4000000000000000000000000000000000000000000
        Rewrite integer as hexadecimal -> 0x100000000000000000000000000000000
        ");
    }

    #[test]
    fn rewrite_negative_integer() {
        assert_snapshot!(
            apply_code_action(rewrite_integer_radix, "select -1_00$0;"),
            @"select -0b1100100;"
        );
    }

    #[test]
    fn rewrite_integer_not_applicable_to_non_integer() {
        assert!(code_action_not_applicable(
            rewrite_integer_radix,
            "select 1.$05;"
        ));
        assert!(code_action_not_applicable(
            rewrite_integer_radix,
            "select 1e$05;"
        ));
    }

    #[test]
    fn rewrite_integer_not_applicable_to_invalid_integer() {
        for sql in ["select 0b2$0;", "select 0x$0;"] {
            assert!(code_action_not_applicable_with_errors(
                rewrite_integer_radix,
                sql
            ));
        }
    }
}
