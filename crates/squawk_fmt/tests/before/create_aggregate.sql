create aggregate public.sum_of_squares(integer) (sfunc = public.sum_squares_state, stype = bigint, initcond = '0');

create or replace aggregate public.count_any(*) (sfunc = public.count_state, stype = bigint);

create aggregate public.old_style_sum (sfunc = public.sum_state, basetype = integer, stype = bigint);

create or replace aggregate extraordinarily_long_schema_name.extraordinarily_long_aggregate_name(double precision, numeric, bigint) (sfunc = extraordinarily_long_schema_name.extraordinarily_long_transition_function_name, stype = extraordinarily_long_schema_name.extraordinarily_long_state_type_name, parallel = safe);

create /* or replace */ or /* replace keyword */ replace /* aggregate keyword */ aggregate /* name */ public.commented_aggregate(/* parameter */ integer) /* attributes */ (/* first attribute */ sfunc /* equals */ = /* value */ public.commented_state, /* second attribute */ stype = /* type value */ bigint /* closing parenthesis */) /* semicolon */;

create aggregate public.ordered_set_aggregate(/* direct parameter */ double precision /* order keyword */ order /* by keyword */ by /* first ordered parameter */ anyelement, /* second ordered parameter */ text /* closing parenthesis */) (sfunc = public.ordered_set_state, stype = internal);

create aggregate extraordinarily_long_schema_name.extraordinarily_long_ordered_set_aggregate_name(extraordinarily_long_schema_name.extraordinarily_long_direct_argument_type order by extraordinarily_long_schema_name.extraordinarily_long_first_ordered_argument_type, extraordinarily_long_schema_name.extraordinarily_long_second_ordered_argument_type) (sfunc = extraordinarily_long_schema_name.extraordinarily_long_transition_function_name, stype = internal);
