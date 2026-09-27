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

-- empty target lists
select;
select from onek;
select where 1 = 1;
select except select;
select all from onek;

with t(ch,val) AS (
  VALUES
    ('0',0),('1',1),('2',2),('3',3),('4',4),('5',5),('6',6),('7',7),('8',8),('9',9),
    ('A',10),('B',11),('C',12),('D',13),('E',14),('F',15),('G',16),('H',17),('I',18),('J',19)
)
select * from t;

with t as (
  select
    case
      when (select flag from use_numeric) then (select t from numeric_bitstream)
      when (select flag from use_alnum) then (select t from alnum_bitstream)
      else (select t from data_bits_byte)
    end as t
)
select *
from t;

-- above with
with 
-- above first
t as ( -- after open paren
  -- above select
  select 1
), -- after comma
-- above second
x as ( -- after open paren
  -- above select
  select 2
) -- after close paren
-- above final
select * from t; -- after statement
