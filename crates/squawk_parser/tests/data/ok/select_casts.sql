-- type_casts
select numeric '1234';
select '1234'::numeric;
select cast('1234' as numeric);
select int8('1234');

select 44::bit(3);
select cast(-44 as bit(12));
select '1110'::bit(4)::integer;

select '{1,2,3}'::int[];
select foo::int;
select foo::numeric[];

select '{}' :: int[] :: int8[] :: numeric[1];

select '{}'::int[]::int8[]::numeric[1];

-- based on postgres' gram.y

-- Bit
select '1001'::bit varying;

select '1001'::bit varying(4);

-- Character

select 'abc'::character;
select 'abc'::character varying;

select 'abc'::char;
select 'abc'::char varying;

select 'abc'::varchar;

select 'abc'::national character;
select 'abc'::national character varying;

select 'abc'::national char;
select 'abc'::national char varying;

select 'abc'::nchar;
select 'abc'::nchar varying;
select 'abc'::nchar varying[];

-- ConstDatetime

select '2024-01-01 12:34:56.123456'::timestamp(2);
select '2024-01-01 12:34:56.123456'::timestamp(2) with time zone;
select '2024-01-01 12:34:56.123456'::timestamp(2) without time zone;

select '2024-01-01 12:34:56.123456'::timestamp;
select '2024-01-01 12:34:56.123456'::timestamp with time zone;
select '2024-01-01 12:34:56.123456'::timestamp without time zone;

select '2024-01-01 12:34:56.123456'::time(2);
select '2024-01-01 12:34:56.123456'::time(2) with time zone;
select '2024-01-01 12:34:56.123456'::time(2) without time zone;

select '2024-01-01 12:34:56.123456'::time;
select '2024-01-01 12:34:56.123456'::time with time zone;
select '2024-01-01 12:34:56.123456'::time without time zone;

-- timestamp with time zone cast
select timestamp with time zone '2005-04-02 12:00:00-07';

-- cast w/ at time zone operator
select timestamp with time zone '2001-02-16 20:38:40-05' at time zone 'america/denver';

-- timestamp cast w/ at time zone operator
select timestamp '2001-02-16 20:38:40' at time zone 'america/denver';

-- multiple at time zone
select timestamp '2001-02-16 20:38:40' at time zone 'asia/tokyo' at time zone 'america/chicago';

-- cast and at local
select time with time zone '20:38:40-05' at local;

select c at local;
select timestamp with time zone '2001-02-16 20:38:40-05' at local;

-- ConstInterval

select '10 days'::interval;

select '10 days'::interval year;

select '10 days'::interval month;

select '10 days'::interval day;

select '10 days'::interval hour;

select '10 days'::interval minute;

select '10 days'::interval second;
select '10 days'::interval second(100);

select '10 days'::interval year to month;

select '10 days'::interval day to hour;

select '10 days'::interval day to minute;

select '10 days'::interval day to second;
select '10 days'::interval day to second(10);

select '10 days'::interval hour to minute;

select '10 days'::interval hour to second;
select '10 days'::interval hour to second(10);

select '10 days'::interval minute to second;
select '10 days'::interval minute to second(10);

select '10 days'::interval(10);

-- JsonType
select '{}'::json;

-- jsonb type cast
select '"foo"'::jsonb @> '"foo"'::jsonb;

-- GenericType
select ''::foo.bar;

select ''::foo.bar(buzz, bizz);

select 'abc'::varchar;
select 'abc'::varchar(5);
select ''::varchar(255)[];
select ''::varchar[5];

select ''::t(255)[];
select ''::foo.buzz(5);
select ''::bar.foo.buzz[5];
select ''::bar.foo.buzz(255)[];

-- Numeric
select ''::int;
select ''::integer;
select ''::smallint;
select ''::bigint;
select ''::float;
select ''::float(1);
select ''::double precision;
select ''::decimal;
select ''::decimal(1, 2, 3);
select ''::dec;
select ''::dec(1, 2, 3);
select ''::numeric;
select ''::numeric(1, 2);
select ''::boolean;
select ''::numeric(10,2)[10];


-- interval_cast_trailing
select interval '1' year;
select interval '1' month;
select interval '1' day;
select interval '1' hour;
select interval '1' minute;
select interval '1' second;
select interval '1' year to month;
select interval '1' day to hour;
select interval '1' day to minute;
select interval '1' day to second;
select interval '1' day to second(10);
select interval '1' hour to minute;
select interval '1' hour to second;
select interval '1' hour to second(10);
select interval '1' minute to second;
select interval '1' minute to second(10);

-- pgdoc_char
select ''::char(1) collate "C";

SELECT 'a '::CHAR(2) collate "C" < E'a\n'::CHAR(2);

select cast(x as b) collate "C" > b;

-- cast_array
select cast('{1}' as integer ARRAY[4]);

select cast('{1}' as int8 ARRAY);

select cast('{1}' as int[4]);

select cast('{1}' as integer[3][3]);

select '{1}'::integer[1][2][3][][][1000];

select array[]::integer[];

-- casts
select 44::bit(10); -- 0000101100
select 44::bit(3); -- 100
select cast(-44 as bit(12)); -- 111111010100
select '1110'::bit(4)::integer; -- 14

select '1'::pg_catalog.int8;
select '{1}'::pg_catalog.int8[];


-- cast
select cast(a as foo.bar);

-- treat
select treat(a as foo.b);
select treat('1231' as numeric);

