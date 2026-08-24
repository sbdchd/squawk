-- keywords in types are lowercased
create table t (a INT, b NUMERIC(10, 2), c PG_CATALOG.VARCHAR(10), d "MyType");

-- character types
create table t (
  a VARCHAR(10),
  b CHARACTER VARYING,
  c NATIONAL CHAR VARYING (2),
  d NCHAR(3),
  e CHARACTER(4),
  f NATIONAL CHARACTER
);

-- bit & double types
create table t (a BIT, b BIT(4), c BIT VARYING, d BIT VARYING(3), e DOUBLE PRECISION);

-- date & time types
create table t (
  a TIME,
  b TIME(3) WITH TIME ZONE,
  c TIMESTAMP WITHOUT TIME ZONE,
  d TIMESTAMP(6) WITH TIME ZONE
);

-- interval types
create table t (
  a INTERVAL,
  b INTERVAL(6),
  c INTERVAL YEAR,
  d INTERVAL YEAR TO MONTH,
  e INTERVAL DAY TO HOUR,
  f INTERVAL HOUR TO MINUTE,
  g INTERVAL SECOND(2),
  h INTERVAL DAY TO SECOND(3)
);

-- array types
create table t (a INT[], b TEXT ARRAY, c TEXT ARRAY[4], d INT[][3], e INT[10][10]);

-- types in casts
select 1::int8, cast(1 as INT8), treat(2 as BIGINT), pg_catalog.varchar(10) 'foo';
select '1'::INTERVAL DAY TO SECOND(3), 'a'::CHARACTER VARYING(2), now()::TIMESTAMP(3) WITH TIME ZONE;
select 1::SETOF INT;

-- comments inside types
create table t (
  a NATIONAL /*a*/ CHAR /*b*/ VARYING /*c*/ (2),
  b INT /*d*/ [],
  c INTERVAL /*e*/ DAY TO /*f*/ SECOND /*g*/ (3),
  d NUMERIC /*h*/ ( /*i*/ 10 /*j*/ , /*k*/ 2 /*l*/ ),
  e TIME /*m*/ ( /*n*/ 3 /*o*/ ) /*p*/ WITH /*q*/ TIME /*r*/ ZONE,
  f INT [ /*s*/ ] [ /*t*/ 3 /*u*/ ],
  g TIMESTAMP /*v*/ WITHOUT /*w*/ TIME /*x*/ ZONE,
  h TIMESTAMP WITH /*y*/ TIME /*z*/ ZONE,
  i DOUBLE /*aa*/ PRECISION,
  j CHARACTER /*bb*/ VARYING,
  k NATIONAL /*cc*/ CHARACTER,
  l BIT /*dd*/ VARYING /*ee*/ ( /*ff*/ 3 /*gg*/ ),
  m INTERVAL YEAR /*hh*/ TO /*ii*/ MONTH
);
select 1::SETOF /*a*/ INT, 2::pg_catalog /*b*/ . /*c*/ int4;

-- line comments inside types
create table t (a INT -- one
[], b NUMERIC -- two
(10), c TEXT -- three
ARRAY[2], d TIME -- four
(3) WITH TIME ZONE, e INTERVAL DAY TO -- five
SECOND(3));

-- interval literals keep their qualifier
select interval '1' day to second(3), interval '2' year to month, interval(4) '3';

-- comments around casts
select 1 /*a*/ :: /*b*/ INT8;
select cast /*c*/ ( /*d*/ 1 /*e*/ as /*f*/ INT8 /*g*/ );
select a_very_long_expression_name_that_forces_the_select_to_wrap, cast /*c*/ ( /*d*/ 1 /*e*/ as /*f*/ INT8 /*g*/ );
select treat /*h*/ ( 2 as /*i*/ BIGINT );
select pg_catalog.varchar(10) /*j*/ 'foo';
select interval '4' /*k*/ year to month;

-- line comments before a type's trailing keywords
create table t (
  a DOUBLE -- one
  PRECISION,
  b BIT -- two
  VARYING(3),
  c NATIONAL -- three
  CHARACTER VARYING(2),
  d VARCHAR -- four
  (10),
  e TIMESTAMP WITHOUT -- five
  TIME ZONE,
  f TIME(3) WITH TIME -- six
  ZONE,
  g INTERVAL YEAR -- seven
  TO MONTH
);
select 1::DOUBLE -- eight
PRECISION;

create table a_very_long_table_name_for_type_wrapping (a_very_long_numeric_column_name numeric(12345, 12345), a_very_long_varchar_column_name varchar(12345), a_very_long_character_varying_column_name character varying(12345), a_very_long_national_character_varying_column_name national character varying(12345), a_very_long_nchar_column_name nchar(12345), a_very_long_bit_varying_column_name bit varying(12345), a_very_long_double_precision_column_name double precision, a_very_long_timestamp_column_name timestamp(12345) without time zone, a_very_long_time_column_name time(12345) with time zone, a_very_long_interval_column_name interval day to second(12345), a_very_long_array_column_name text[12345][12345]);
select a_very_long_expression_name::a_very_long_type_schema_name.a_very_long_type_name, cast(a_very_long_expression_name as character varying(12345)), cast(a_very_long_expression_name_long_long_long_long_long_long as character varying(12345)), treat(a_very_long_expression_name as a_very_long_type_schema_name.a_very_long_type_name), a_very_long_type_schema_name.a_very_long_type_name(12345) 'a very long typed string literal value', interval 'a very long interval literal value' day to second(12345);
