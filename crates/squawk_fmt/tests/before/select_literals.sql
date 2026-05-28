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
