use rowan::TextSize;
use salsa::Database as Db;
use squawk_linter::Edit;

use crate::file::InFile;

mod add_explicit_alias;
mod add_schema;
mod quote_identifier;
mod remove_else_clause;
mod remove_redundant_alias;
mod rewrite_as_dollar_quoted_string;
mod rewrite_as_regular_string;
mod rewrite_between_as_binary_expression;
mod rewrite_cast_to_double_colon;
mod rewrite_create_table_as_as_select_into;
mod rewrite_double_colon_to_cast;
mod rewrite_from;
mod rewrite_leading_from;
mod rewrite_not_equals_operator;
mod rewrite_select_as_table;
mod rewrite_select_as_values;
mod rewrite_select_into_as_create_table_as;
mod rewrite_table_as_select;
mod rewrite_timestamp_type;
mod rewrite_values_as_select;
mod unquote_identifier;

#[cfg(test)]
mod test_utils;

use add_explicit_alias::add_explicit_alias;
use add_schema::add_schema;
use quote_identifier::quote_identifier;
use remove_else_clause::remove_else_clause;
use remove_redundant_alias::remove_redundant_alias;
use rewrite_as_dollar_quoted_string::rewrite_as_dollar_quoted_string;
use rewrite_as_regular_string::rewrite_as_regular_string;
use rewrite_between_as_binary_expression::rewrite_between_as_binary_expression;
use rewrite_cast_to_double_colon::rewrite_cast_to_double_colon;
use rewrite_create_table_as_as_select_into::rewrite_create_table_as_as_select_into;
use rewrite_double_colon_to_cast::rewrite_double_colon_to_cast;
use rewrite_from::rewrite_from;
use rewrite_leading_from::rewrite_leading_from;
use rewrite_not_equals_operator::rewrite_not_equals_operator;
use rewrite_select_as_table::rewrite_select_as_table;
use rewrite_select_as_values::rewrite_select_as_values;
use rewrite_select_into_as_create_table_as::rewrite_select_into_as_create_table_as;
use rewrite_table_as_select::rewrite_table_as_select;
use rewrite_timestamp_type::rewrite_timestamp_type;
use rewrite_values_as_select::rewrite_values_as_select;
use unquote_identifier::unquote_identifier;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionKind {
    QuickFix,
    RefactorRewrite,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeAction {
    pub title: String,
    pub edits: Vec<Edit>,
    pub kind: ActionKind,
}

pub fn code_actions(db: &dyn Db, position: InFile<TextSize>) -> Option<Vec<CodeAction>> {
    let mut actions = vec![];
    rewrite_as_regular_string(db, position, &mut actions);
    rewrite_as_dollar_quoted_string(db, position, &mut actions);
    remove_else_clause(db, position, &mut actions);
    rewrite_table_as_select(db, position, &mut actions);
    rewrite_select_as_table(db, position, &mut actions);
    rewrite_from(db, position, &mut actions);
    rewrite_leading_from(db, position, &mut actions);
    rewrite_values_as_select(db, position, &mut actions);
    rewrite_select_as_values(db, position, &mut actions);
    rewrite_select_into_as_create_table_as(db, position, &mut actions);
    rewrite_create_table_as_as_select_into(db, position, &mut actions);
    add_schema(db, position, &mut actions);
    quote_identifier(db, position, &mut actions);
    unquote_identifier(db, position, &mut actions);
    add_explicit_alias(db, position, &mut actions);
    remove_redundant_alias(db, position, &mut actions);
    rewrite_cast_to_double_colon(db, position, &mut actions);
    rewrite_double_colon_to_cast(db, position, &mut actions);
    rewrite_between_as_binary_expression(db, position, &mut actions);
    rewrite_not_equals_operator(db, position, &mut actions);
    rewrite_timestamp_type(db, position, &mut actions);
    Some(actions)
}
