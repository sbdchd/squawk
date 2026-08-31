-- ok
select numeric(10, 2) '1';
select numeric(foo) '1';
select numeric(-1) '1';
select numeric('1') '1';
select numeric(e'1') '1';
select numeric(u&'1') '1';
select numeric($$1$$) '1';
select '1'::numeric(10, 2);
select cast('1' as numeric(10, 2));

-- errors
select foo(a => 1) 'x';
select foo(1 order by 2) 'x';
select numeric(1 + 2) '1';
select foo.bar(nested(1)) '100';
select '1'::numeric(1 + 2);
select cast('1' as numeric(1 + 2));
select numeric(+1) '1';
select numeric(foo.bar) '1';
select numeric(true) '1';
select numeric(null) '1';
select numeric(b'1') '1';
select numeric(x'1') '1';
select numeric(n'1') '1';
select numeric(variadic 1) '1';
