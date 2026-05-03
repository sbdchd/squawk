use rowan::TextSize;
use salsa::Database as Db;
use squawk_linter::Edit;

use crate::db::File;

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

#[salsa::tracked]
pub fn code_actions(db: &dyn Db, file: File, offset: TextSize) -> Option<Vec<CodeAction>> {
    let mut actions = vec![];
    rewrite_as_regular_string(db, file, &mut actions, offset);
    rewrite_as_dollar_quoted_string(db, file, &mut actions, offset);
    remove_else_clause(db, file, &mut actions, offset);
    rewrite_table_as_select(db, file, &mut actions, offset);
    rewrite_select_as_table(db, file, &mut actions, offset);
    rewrite_from(db, file, &mut actions, offset);
    rewrite_leading_from(db, file, &mut actions, offset);
    rewrite_values_as_select(db, file, &mut actions, offset);
    rewrite_select_as_values(db, file, &mut actions, offset);
    rewrite_select_into_as_create_table_as(db, file, &mut actions, offset);
    rewrite_create_table_as_as_select_into(db, file, &mut actions, offset);
    add_schema(db, file, &mut actions, offset);
    quote_identifier(db, file, &mut actions, offset);
    unquote_identifier(db, file, &mut actions, offset);
    add_explicit_alias(db, file, &mut actions, offset);
    remove_redundant_alias(db, file, &mut actions, offset);
    rewrite_cast_to_double_colon(db, file, &mut actions, offset);
    rewrite_double_colon_to_cast(db, file, &mut actions, offset);
    rewrite_between_as_binary_expression(db, file, &mut actions, offset);
    rewrite_not_equals_operator(db, file, &mut actions, offset);
    rewrite_timestamp_type(db, file, &mut actions, offset);
    Some(actions)
}
