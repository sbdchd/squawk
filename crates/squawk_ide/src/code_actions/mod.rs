use rowan::TextSize;
use salsa::Database as Db;
use squawk_linter::Edit;

use crate::file::InFile;

mod add_explicit_alias;
mod add_schema;
mod convert_comment;
mod quote_identifier;
mod remove_else_clause;
mod remove_redundant_alias;
mod remove_routine_param_in;
mod rewrite_as_dollar_quoted_string;
mod rewrite_as_regular_string;
mod rewrite_at_local_as_timezone;
mod rewrite_at_time_zone_as_timezone;
mod rewrite_between_as_binary_expression;
mod rewrite_cast_to_double_colon;
mod rewrite_collation_for_as_function_call;
mod rewrite_colon_eq_as_fat_arrow;
mod rewrite_create_table_as_as_select_into;
mod rewrite_double_colon_to_cast;
mod rewrite_extract_as_function_call;
mod rewrite_from;
mod rewrite_function_param_default_as_equals;
mod rewrite_in_as_expression;
mod rewrite_integer_radix;
mod rewrite_is_normalized_as_function_call;
mod rewrite_json_value_as_colon;
mod rewrite_leading_from;
mod rewrite_normalize_as_function_call;
mod rewrite_not_equals_operator;
mod rewrite_null_predicate;
mod rewrite_overlaps_as_function_call;
mod rewrite_overlay_as_function_call;
mod rewrite_pattern_matching_as_operators;
mod rewrite_position_as_function_call;
mod rewrite_routine_param_in_out_as_inout;
mod rewrite_rows_from_as_unnest;
mod rewrite_select_as_table;
mod rewrite_select_as_values;
mod rewrite_select_into_as_create_table_as;
mod rewrite_substring_as_function_call;
mod rewrite_system_user_as_function_call;
mod rewrite_table_as_select;
mod rewrite_timestamp_type;
mod rewrite_trim_as_function_call;
mod rewrite_unnest_as_rows_from;
mod rewrite_values_as_select;
mod rewrite_xmlexists_as_function_call;
mod unnest;
mod unquote_identifier;

#[cfg(test)]
mod test_utils;

use add_explicit_alias::add_explicit_alias;
use add_schema::add_schema;
use convert_comment::convert_comment;
use quote_identifier::quote_identifier;
use remove_else_clause::remove_else_clause;
use remove_redundant_alias::remove_redundant_alias;
use remove_routine_param_in::remove_routine_param_in;
use rewrite_as_dollar_quoted_string::rewrite_as_dollar_quoted_string;
use rewrite_as_regular_string::rewrite_as_regular_string;
use rewrite_at_local_as_timezone::rewrite_at_local_as_timezone;
use rewrite_at_time_zone_as_timezone::rewrite_at_time_zone_as_timezone;
use rewrite_between_as_binary_expression::rewrite_between_as_binary_expression;
use rewrite_cast_to_double_colon::rewrite_cast_to_double_colon;
use rewrite_collation_for_as_function_call::rewrite_collation_for_as_function_call;
use rewrite_colon_eq_as_fat_arrow::rewrite_colon_eq_as_fat_arrow;
use rewrite_create_table_as_as_select_into::rewrite_create_table_as_as_select_into;
use rewrite_double_colon_to_cast::rewrite_double_colon_to_cast;
use rewrite_extract_as_function_call::rewrite_extract_as_function_call;
use rewrite_from::rewrite_from;
use rewrite_function_param_default_as_equals::rewrite_function_param_default_as_equals;
use rewrite_in_as_expression::rewrite_in_as_expression;
use rewrite_integer_radix::rewrite_integer_radix;
use rewrite_is_normalized_as_function_call::rewrite_is_normalized_as_function_call;
use rewrite_json_value_as_colon::rewrite_json_value_as_colon;
use rewrite_leading_from::rewrite_leading_from;
use rewrite_normalize_as_function_call::rewrite_normalize_as_function_call;
use rewrite_not_equals_operator::rewrite_not_equals_operator;
use rewrite_null_predicate::rewrite_null_predicate;
use rewrite_overlaps_as_function_call::rewrite_overlaps_as_function_call;
use rewrite_overlay_as_function_call::rewrite_overlay_as_function_call;
use rewrite_pattern_matching_as_operators::rewrite_pattern_matching_as_operators;
use rewrite_position_as_function_call::rewrite_position_as_function_call;
use rewrite_routine_param_in_out_as_inout::rewrite_routine_param_in_out_as_inout;
use rewrite_rows_from_as_unnest::rewrite_rows_from_as_unnest;
use rewrite_select_as_table::rewrite_select_as_table;
use rewrite_select_as_values::rewrite_select_as_values;
use rewrite_select_into_as_create_table_as::rewrite_select_into_as_create_table_as;
use rewrite_substring_as_function_call::rewrite_substring_as_function_call;
use rewrite_system_user_as_function_call::rewrite_system_user_as_function_call;
use rewrite_table_as_select::rewrite_table_as_select;
use rewrite_timestamp_type::rewrite_timestamp_type;
use rewrite_trim_as_function_call::rewrite_trim_as_function_call;
use rewrite_unnest_as_rows_from::rewrite_unnest_as_rows_from;
use rewrite_values_as_select::rewrite_values_as_select;
use rewrite_xmlexists_as_function_call::rewrite_xmlexists_as_function_call;
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
    convert_comment(db, position, &mut actions);
    rewrite_as_regular_string(db, position, &mut actions);
    rewrite_as_dollar_quoted_string(db, position, &mut actions);
    remove_else_clause(db, position, &mut actions);
    rewrite_table_as_select(db, position, &mut actions);
    rewrite_select_as_table(db, position, &mut actions);
    rewrite_from(db, position, &mut actions);
    rewrite_function_param_default_as_equals(db, position, &mut actions);
    rewrite_routine_param_in_out_as_inout(db, position, &mut actions);
    remove_routine_param_in(db, position, &mut actions);
    rewrite_integer_radix(db, position, &mut actions);
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
    rewrite_null_predicate(db, position, &mut actions);
    rewrite_colon_eq_as_fat_arrow(db, position, &mut actions);
    rewrite_json_value_as_colon(db, position, &mut actions);
    rewrite_timestamp_type(db, position, &mut actions);
    rewrite_unnest_as_rows_from(db, position, &mut actions);
    rewrite_rows_from_as_unnest(db, position, &mut actions);
    rewrite_at_time_zone_as_timezone(db, position, &mut actions);
    rewrite_at_local_as_timezone(db, position, &mut actions);
    rewrite_overlaps_as_function_call(db, position, &mut actions);
    rewrite_extract_as_function_call(db, position, &mut actions);
    rewrite_is_normalized_as_function_call(db, position, &mut actions);
    rewrite_collation_for_as_function_call(db, position, &mut actions);
    rewrite_normalize_as_function_call(db, position, &mut actions);
    rewrite_overlay_as_function_call(db, position, &mut actions);
    rewrite_position_as_function_call(db, position, &mut actions);
    rewrite_substring_as_function_call(db, position, &mut actions);
    rewrite_trim_as_function_call(db, position, &mut actions);
    rewrite_system_user_as_function_call(db, position, &mut actions);
    rewrite_xmlexists_as_function_call(db, position, &mut actions);
    rewrite_in_as_expression(db, position, &mut actions);
    rewrite_pattern_matching_as_operators(db, position, &mut actions);
    Some(actions)
}
