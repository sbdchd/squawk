-- null
select null;
-- true
select true;
-- false
select false;
-- int number
select 42;
-- numeric number
select 3.14;
-- string
select 'hello';
-- esc string
select E'esc\n';
-- unicode esc string
select U&'unicode';
-- dollar quoted string
select $$dollar$$;
-- dollar quoted string with tag
select $tag$body$tag$;
-- bit string
select B'1010';
-- byte string
select X'AF';
-- positional param
select $1;
-- string continuation
select 'foo'
  'bar';
-- string continuation, multiple
select 'one'
  'two'
  'three';
-- esc string continuation
select E'esc\n'
  'tail';
-- unicode esc string continuation
select U&'uni'
  'tail';
-- bit string continuation
select B'1010'
  '0101';
-- byte string continuation
select X'AF'
  'BE';

select null as a_very_long_null_literal_column_alias, true as a_very_long_true_literal_column_alias, false as a_very_long_false_literal_column_alias, $1234567890 as a_very_long_positional_parameter_column_alias;
select 1234567890123456789012345678901234567890, 12345678901234567890.12345678901234567890, 'a very long ordinary string literal value that forces the literal target list to wrap', E'a very long escaped string literal value that forces the literal target list to wrap\n', U&'a very long unicode string literal value that forces the literal target list to wrap', $$a very long dollar quoted string literal value that forces the literal target list to wrap$$, $a_very_long_dollar_quote_tag$a very long tagged dollar quoted string literal value$a_very_long_dollar_quote_tag$, B'10101010101010101010101010101010101010101010101010101010101010101010101010101010', X'ABCDEFABCDEFABCDEFABCDEFABCDEFABCDEFABCDEFABCDEFABCDEFABCDEFABCDEFABCDEFABCDEF';
select 'a very long continued ordinary string literal value'
  'with a very long continuation segment', E'a very long continued escaped string literal value\n'
  'with a very long continuation segment', U&'a very long continued unicode string literal value'
  'with a very long continuation segment', B'1010101010101010101010101010101010101010'
  '0101010101010101010101010101010101010101', X'ABCDEFABCDEFABCDEFABCDEFABCDEFABCDEF'
  '123456123456123456123456123456123456';
