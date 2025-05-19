-- logical_operators
-- and
select true and false;

-- or
select true or false;

-- not
select not true;

-- not is null
select not x is null;
select not x isnull;

-- geo_operators

-- add
select box '(1,1),(0,0)' + point '(2,0)';

-- concat
select path '[(0,0),(1,1)]' + path '[(2,2),(3,3),(4,4)]';

-- subtract
select box '(1,1),(0,0)' - point '(2,0)';

-- multiply
select path '((0,0),(1,0),(1,1))' * point '(3.0,0)';
select path '((0,0),(1,0),(1,1))' * point(cosd(45), sind(45));

-- divide
select path '((0,0),(1,0),(1,1))' / point '(2.0,0)';
select path '((0,0),(1,0),(1,1))' / point(cosd(45), sind(45));

-- length
select @-@ path '[(0,0),(1,0),(1,1)]';

-- center point
select @@ box '(2,2),(0,0)';

-- number of points
select # path '((1,0),(0,1),(-1,0))';

-- point intersection
select lseg '[(0,0),(1,1)]' # lseg '[(1,0),(0,1)]';

-- box intersection
select box '(2,2),(-1,-1)' # box '(1,1),(-2,-2)';

-- closest point
select point '(0,0)' ## lseg '[(2,0),(0,2)]';

-- distance between
select circle '<(0,0),1>' <-> circle '<(5,0),1>';

-- first contains
select circle '<(0,0),2>' @> point '(1,1)';

-- first contained in second
select point '(1,1)' <@ circle '<(0,0),2>';

-- objects overlap
select box '(1,1),(0,0)' && box '(2,2),(0,0)';

-- object strictly left
select circle '<(0,0),1>' << circle '<(5,0),1>';

-- object strictly right
select circle '<(5,0),1>' >> circle '<(0,0),1>';

-- first extended to right of second
select box '(1,1),(0,0)' &< box '(2,2),(0,0)';

-- first extends to left of second
select box '(3,3),(0,0)' &> box '(2,2),(0,0)';

-- first strictly below second
select box '(3,3),(0,0)' <<| box '(5,5),(3,4)';

-- first strictly above second
select box '(3,3),(0,0)' |>> box '(5,5),(3,4)';

-- first doesn't extend above second
select box '(1,1),(0,0)' &<| box '(2,2),(0,0)';

-- first doesn't extend below second
select box '(3,3),(0,0)' |&> box '(2,2),(0,0)';

-- first below second
select box '((1,1),(0,0))' <^ box '((2,2),(1,1))';

-- first above second
select box '((2,2),(1,1))' >^ box '((1,1),(0,0))';

-- objects intersect
select lseg '[(-1,0),(1,0)]' ?# box '(2,2),(-2,-2)';

-- line horizontal
select ?- lseg '[(-1,0),(1,0)]';

-- points horizontally aligned
select point '(1,0)' ?- point '(0,0)';

-- line vertical
select ?| lseg '[(-1,0),(1,0)]';

-- points vertically aligned
select point '(0,1)' ?| point '(0,0)';

-- lines perpendicular
select lseg '[(0,0),(0,1)]' ?-| lseg '[(0,0),(1,0)]';

-- lines parallel
select lseg '[(-1,0),(1,0)]' ?|| lseg '[(-1,2),(1,2)]';

-- objects the same
select polygon '((0,0),(1,1))' ~= polygon '((1,1),(0,0))';


-- network_operators

-- subnet contained by or equal
select inet '192.168.1/24' <<= inet '192.168.1/24';

-- subnet contain or equal subnet
select inet '192.168.1/24' >>= inet '192.168.1/24';


-- fts_operators

-- negative tsquery
select !! 'cat'::tsquery;

-- subnet contain or equal subnet
select inet '192.168.1/24' >>= inet '192.168.1/24';


-- array_operators
-- first array (as a set) contain the second (as a set)?
select array[1,4,3] @> array[3,1,3];

-- first array (as a set) contained by the second (as a set)?
select array[2,2,7] <@ array[1,7,4,2,6];

-- is there any overlap
select array[1,4,3] && array[2,1];

-- concat
select ARRAY[[1,2],[3,4]] || ARRAY[[5,6],[7,8],[9,0]];

-- starts with
select 'alphabet' ^@ 'alph';

-- array_access
-- first index
select a[0];

-- slicing
select b[1:2][1:1];

-- omitted bounds
select c[:2][2:];

-- omitted part 2
select schedule[:][1:1];


-- string_operators
-- concatenate
select 'Post' || 'greSQL';
select 'Value: ' || 42;

-- normalization check
select U&'\0061\0308bc' is normalized;
select U&'\0061\0308bc' is nfc normalized;
select U&'\0061\0308bc' is nfd normalized;
select U&'\0061\0308bc' is nfkc normalized;
select U&'\0061\0308bc' is nfkd normalized;
select U&'\0061\0308bc' is not nfd normalized;

-- pattern_matching
-- like
select 'foo' like 'bar';

-- not like
select 'foo' not like 'bar';

-- ~~
select 'a' ~~ 'b';

-- !~~
select 'a' !~~ 'b';

-- similar to
select 'abc' similar to 'abc';

-- posix regex
-- string matches regex case sensitive
select 'foo' ~ 'f.*';

-- string matches regex case insensitive
select 'a' ~* 'b';

-- string does not match regex case sensitive
select 'a' !~ 'b';

-- string does not match regex case insensitive
select 'a' !~* 'b';


-- compare_operators
-- less than
select 1 < 2;

-- greater than
select 5 > 3;

-- less than or equal to
select 8 <= 4;

-- greater than or equal to
select 9 >= 7;

-- equal
select 5 = 0;

-- not equal
select 1 <> 3;
select 1 != 3;

-- between (inclusive of the range endpoints)
select 2 between 1 and 3;
select 2 between foo() and bar();

-- not between (the negation of between)
select 2 not between 1 and 3;
select 2 not between foo() and bar();

-- between symmetric (between, after sorting the two endpoint values)
select 2 between symmetric 3 and 1;

-- not between symmetric (not between, after sorting the two endpoint values)
select 2 not between symmetric 3 and 1;

-- is distinct from (not equal, treating null as a comparable value)
select 1 is distinct from null;

-- is not distinct from (equal, treating null as a comparable value)
select 1 is not distinct from null;

-- at time zone
select '2024-01-01 12:00:00' at time zone 'UTC';

-- is null
select 1.5 is null;

-- is not null
select 'null' is not null;

-- isnull (non-standard syntax)
select 1 isnull;

-- notnull (non-standard syntax)
select 'foo' isnull;

-- is true
select true is true;

-- is not true
select true is not true;

-- is false
select true is false;

-- is not false
select true is not false;

-- is unknown
select true is unknown;

-- is not unknown
select true is not unknown;

-- math_operators
-- addition
select 2 + 3;

-- unary plus
select + 3.5;

-- subtraction
select 2 - 3;

-- negation
select - (-4);

-- multiplication
select 2 * 3;

-- division
select 5.0 / 2;
select 5 / 2;
select (-5) / 2;

-- modulo
select 5 % 4;

-- exponentiation
select 2 ^ 3;
select 2 ^ (3 ^ 3);

-- square root
select |/ 25.0;

-- cube root
select ||/ 64.0;

-- absolute value
select @ -5.0;

-- bitwise and
select 91 & 15;

-- bitwise or
select 32 | 3;

-- bitwise exclusive OR
select 17 # 5;

-- bitwise NOT
select ~1;

-- bitwise shift left
select 1 << 4;

-- bitwise shift right
select 8 >> 2;

-- bitstring_operators
-- bitwise and
select B'10001' & B'01101';

-- bitwise or
select B'10001' | B'01101';

-- bitwise xor
select B'10001' # B'01101';

-- bitwise not
select ~ B'10001';

-- bitshift left
select B'10001' << 3;

-- bitshift right
select B'10001' >> 2;

-- range
select int4range(2, 4) <@ int4range(1, 7);

-- json_ops
-- extract
select '[{"a":"foo"},{"b":"bar"},{"c":"baz"}]'::json -> 2;
select '{"a": {"b":"foo"}}'::json -> 'a';

-- extract as text
select '[1,2,3]'::json ->> 2;
select '{"a":1,"b":2}'::json ->> 'b';

-- extract path as json
select '{"a": {"b": ["foo","bar"]}}'::json #> '{a,b,1}';

-- extract path as text
select '{"a": {"b": ["foo","bar"]}}'::json #>> '{a,b,1}';

-- contains
select '"foo"'::jsonb @> '"foo"'::jsonb;

select '{"b":2}'::jsonb <@ '{"a":1, "b":2}'::jsonb;

-- existence
select '{"a":1, "b":2}'::jsonb ? 'b';
select '["a", "b", "c"]'::jsonb ? 'b';

-- any keys exist
select '{"a":1, "b":2, "c":3}'::jsonb ?| array['b', 'd'];

-- all keys exist
select '["a", "b", "c"]'::jsonb ?& array['a', 'b'];

-- concat
select '["a", "b"]'::jsonb || '["a", "d"]'::jsonb;
select '{"a": "b"}'::jsonb || '{"c": "d"}'::jsonb;
select '[1, 2]'::jsonb || '3'::jsonb;
select '{"a": "b"}'::jsonb || '42'::jsonb;
select '[1, 2]'::jsonb || jsonb_build_array('[3, 4]'::jsonb);

-- delete
select '{"a": "b", "c": "d"}'::jsonb - 'a';
select '["a", "b", "c", "b"]'::jsonb - 'b';
select '{"a": "b", "c": "d"}'::jsonb - '{a,c}'::text[];
select '["a", "b"]'::jsonb - 1;

-- delete at path
select '["a", {"b":1}]'::jsonb #- '{1,b}';

-- json path find
select '{"items": [1, 2, 3]}'::jsonb @? '$.items[*] ? (@ > 2)';

-- json path predicate check
select '{"a":[1,2,3,4,5]}'::jsonb @@ '$.a[*] > 2';

-- subscripting
select foo[1];
select bar['a'];
select ('[1, "2", null]'::jsonb)[1];
select ('{"a": 1}'::jsonb)['a'];
select ('{"a": {"b": {"c": 1}}}'::jsonb)['a']['b']['c'];

select mytable.arraycolumn[4];
select mytable.two_d_column[17][34];
select $1[10:42];
select (arrayfunction(a,b))[42];

-- select_with_collate
select a < ('foo' collate "fr_FR") from t;

select a < b collate "de_DE" from t;

select 'Å' = 'A' collate ignore_accent_case;

select a collate "de_DE" < b from t;

select a collate "C" < b collate "POSIX" from t;

select * from t order by a || b collate "fr_FR";

select !!a collate "C";

select -a collate "C";

select +a collate "C";

select -t.a[0] collate "C";
select -t.b::c collate "C";

-- cube_operators
select a && b;

select a @> b;

select a <@ b;

select a -> b;

select a ~> b;

select a <-> b;

select a <#> b;

select a <=> b;

-- more_ops
select a <+> b;

-- prefix operators
select ** a;

-- range_operators
-- contains
select int4range(2, 4) @> int4range(2, 3);

-- contains element
select '[2011-01-01,2011-03-01)'::tsrange @> '2011-01-10'::timestamp;

-- first contained by second
select int4range(2, 4) <@ int4range(1, 7);

-- element contained in range
select 42 <@ int4range(1, 7);

-- range overlap
select int8range(3, 7) && int8range(4, 12);

-- first range strictly left
select int8range(1, 10) << int8range(100, 110);

-- first range strictly right
select int8range(50, 60) >> int8range(20, 30);

-- first range not extend right of second
select int8range(1, 20) &< int8range(18, 20);

-- first range not extend left of second
select int8range(7, 20) &> int8range(5, 10);

-- ranges adjacent
select numrange(1.1, 2.2) -|- numrange(2.2, 3.3);

-- union
select numrange(5, 15) + numrange(10, 20);

-- intersection
select int8range(5, 15) * int8range(10, 20);

-- difference
select int8range(5, 15) - int8range(10, 20);

-- OVERLAPS
select (date '2001-02-16', date '2001-12-21') overlaps (date '2001-10-30', date '2002-10-30');

-- postfix operators
select a is not null;

select a is not unknown;

-- single tuple
select 1 in (1);

-- multi tuple
select 1 in (2, 3, 4);

-- not_in
-- select
select 1 not in (1);

-- single char operators that are okay in the prefix position
select +a;
select -b;
select ~h;
select !i;
select @j;
select #k;
select &n;
select |o;
select ?q;
select `p;
