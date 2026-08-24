select 1;select 2;select 3;
select  'hello';
select now();

select  'really long string                                                    ',  'another really long string';

select foo as "Quoted Alias" from "Quoted Table";

select 1 as "foo";

-- aliases without an `as` are bare col labels, so keywords stay quoted
select 1 "foo", 2 "filter", 3 "day", 4 "array", 5 "Mixed";

select 1 as "foo", 2 as "filter", 3 as "day", 4 as "array";

select 1 /*a*/group /* b */by/*c */ 1;

select a_very_long_first_target_expression as a_very_long_first_column_alias, a_very_long_second_target_expression a_very_long_second_column_alias, a_very_long_third_target_expression as "A Very Long Quoted Third Column Alias" from a_very_long_schema_name.a_very_long_table_name as a_very_long_table_alias group by a_very_long_first_target_expression, a_very_long_second_target_expression, a_very_long_third_target_expression;
