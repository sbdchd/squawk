-- squawk-ignore-file
-- pg version: 18.3
-- update via:
--   cargo xtask sync-builtins

create function gin_btree_consistent(
  internal,
  smallint,
  anyelement,
  integer,
  internal,
  internal
) returns boolean
  language c
  immutable
  strict;

create function gin_compare_prefix_anyenum(
  anyenum,
  anyenum,
  smallint,
  internal
) returns integer
  language c
  immutable
  strict;

create function gin_compare_prefix_bit(
  bit,
  bit,
  smallint,
  internal
) returns integer
  language c
  immutable
  strict;

create function gin_compare_prefix_bool(
  boolean,
  boolean,
  smallint,
  internal
) returns integer
  language c
  immutable
  strict;

create function gin_compare_prefix_bpchar(
  character,
  character,
  smallint,
  internal
) returns integer
  language c
  immutable
  strict;

create function gin_compare_prefix_bytea(
  bytea,
  bytea,
  smallint,
  internal
) returns integer
  language c
  immutable
  strict;

create function gin_compare_prefix_char(
  "char",
  "char",
  smallint,
  internal
) returns integer
  language c
  immutable
  strict;

create function gin_compare_prefix_cidr(
  cidr,
  cidr,
  smallint,
  internal
) returns integer
  language c
  immutable
  strict;

create function gin_compare_prefix_date(
  date,
  date,
  smallint,
  internal
) returns integer
  language c
  immutable
  strict;

create function gin_compare_prefix_float4(
  real,
  real,
  smallint,
  internal
) returns integer
  language c
  immutable
  strict;

create function gin_compare_prefix_float8(
  double precision,
  double precision,
  smallint,
  internal
) returns integer
  language c
  immutable
  strict;

create function gin_compare_prefix_inet(
  inet,
  inet,
  smallint,
  internal
) returns integer
  language c
  immutable
  strict;

create function gin_compare_prefix_int2(
  smallint,
  smallint,
  smallint,
  internal
) returns integer
  language c
  immutable
  strict;

create function gin_compare_prefix_int4(
  integer,
  integer,
  smallint,
  internal
) returns integer
  language c
  immutable
  strict;

create function gin_compare_prefix_int8(
  bigint,
  bigint,
  smallint,
  internal
) returns integer
  language c
  immutable
  strict;

create function gin_compare_prefix_interval(
  interval,
  interval,
  smallint,
  internal
) returns integer
  language c
  immutable
  strict;

create function gin_compare_prefix_macaddr(
  macaddr,
  macaddr,
  smallint,
  internal
) returns integer
  language c
  immutable
  strict;

create function gin_compare_prefix_macaddr8(
  macaddr8,
  macaddr8,
  smallint,
  internal
) returns integer
  language c
  immutable
  strict;

create function gin_compare_prefix_money(
  money,
  money,
  smallint,
  internal
) returns integer
  language c
  immutable
  strict;

create function gin_compare_prefix_name(
  name,
  name,
  smallint,
  internal
) returns integer
  language c
  immutable
  strict;

create function gin_compare_prefix_numeric(
  numeric,
  numeric,
  smallint,
  internal
) returns integer
  language c
  immutable
  strict;

create function gin_compare_prefix_oid(
  oid,
  oid,
  smallint,
  internal
) returns integer
  language c
  immutable
  strict;

create function gin_compare_prefix_text(
  text,
  text,
  smallint,
  internal
) returns integer
  language c
  immutable
  strict;

create function gin_compare_prefix_time(
  time without time zone,
  time without time zone,
  smallint,
  internal
) returns integer
  language c
  immutable
  strict;

create function gin_compare_prefix_timestamp(
  timestamp without time zone,
  timestamp without time zone,
  smallint,
  internal
) returns integer
  language c
  immutable
  strict;

create function gin_compare_prefix_timestamptz(
  timestamp with time zone,
  timestamp with time zone,
  smallint,
  internal
) returns integer
  language c
  immutable
  strict;

create function gin_compare_prefix_timetz(
  time with time zone,
  time with time zone,
  smallint,
  internal
) returns integer
  language c
  immutable
  strict;

create function gin_compare_prefix_uuid(
  uuid,
  uuid,
  smallint,
  internal
) returns integer
  language c
  immutable
  strict;

create function gin_compare_prefix_varbit(
  bit varying,
  bit varying,
  smallint,
  internal
) returns integer
  language c
  immutable
  strict;

create function gin_enum_cmp(anyenum, anyenum) returns integer
  language c
  immutable
  strict;

create function gin_extract_query_anyenum(
  anyenum,
  internal,
  smallint,
  internal,
  internal
) returns internal
  language c
  immutable
  strict;

create function gin_extract_query_bit(
  bit,
  internal,
  smallint,
  internal,
  internal
) returns internal
  language c
  immutable
  strict;

create function gin_extract_query_bool(
  boolean,
  internal,
  smallint,
  internal,
  internal
) returns internal
  language c
  immutable
  strict;

create function gin_extract_query_bpchar(
  character,
  internal,
  smallint,
  internal,
  internal
) returns internal
  language c
  immutable
  strict;

create function gin_extract_query_bytea(
  bytea,
  internal,
  smallint,
  internal,
  internal
) returns internal
  language c
  immutable
  strict;

create function gin_extract_query_char(
  "char",
  internal,
  smallint,
  internal,
  internal
) returns internal
  language c
  immutable
  strict;

create function gin_extract_query_cidr(
  cidr,
  internal,
  smallint,
  internal,
  internal
) returns internal
  language c
  immutable
  strict;

create function gin_extract_query_date(
  date,
  internal,
  smallint,
  internal,
  internal
) returns internal
  language c
  immutable
  strict;

create function gin_extract_query_float4(
  real,
  internal,
  smallint,
  internal,
  internal
) returns internal
  language c
  immutable
  strict;

create function gin_extract_query_float8(
  double precision,
  internal,
  smallint,
  internal,
  internal
) returns internal
  language c
  immutable
  strict;

create function gin_extract_query_inet(
  inet,
  internal,
  smallint,
  internal,
  internal
) returns internal
  language c
  immutable
  strict;

create function gin_extract_query_int2(
  smallint,
  internal,
  smallint,
  internal,
  internal
) returns internal
  language c
  immutable
  strict;

create function gin_extract_query_int4(
  integer,
  internal,
  smallint,
  internal,
  internal
) returns internal
  language c
  immutable
  strict;

create function gin_extract_query_int8(
  bigint,
  internal,
  smallint,
  internal,
  internal
) returns internal
  language c
  immutable
  strict;

create function gin_extract_query_interval(
  interval,
  internal,
  smallint,
  internal,
  internal
) returns internal
  language c
  immutable
  strict;

create function gin_extract_query_macaddr(
  macaddr,
  internal,
  smallint,
  internal,
  internal
) returns internal
  language c
  immutable
  strict;

create function gin_extract_query_macaddr8(
  macaddr8,
  internal,
  smallint,
  internal,
  internal
) returns internal
  language c
  immutable
  strict;

create function gin_extract_query_money(
  money,
  internal,
  smallint,
  internal,
  internal
) returns internal
  language c
  immutable
  strict;

create function gin_extract_query_name(
  name,
  internal,
  smallint,
  internal,
  internal
) returns internal
  language c
  immutable
  strict;

create function gin_extract_query_numeric(
  numeric,
  internal,
  smallint,
  internal,
  internal
) returns internal
  language c
  immutable
  strict;

create function gin_extract_query_oid(
  oid,
  internal,
  smallint,
  internal,
  internal
) returns internal
  language c
  immutable
  strict;

create function gin_extract_query_text(
  text,
  internal,
  smallint,
  internal,
  internal
) returns internal
  language c
  immutable
  strict;

create function gin_extract_query_time(
  time without time zone,
  internal,
  smallint,
  internal,
  internal
) returns internal
  language c
  immutable
  strict;

create function gin_extract_query_timestamp(
  timestamp without time zone,
  internal,
  smallint,
  internal,
  internal
) returns internal
  language c
  immutable
  strict;

create function gin_extract_query_timestamptz(
  timestamp with time zone,
  internal,
  smallint,
  internal,
  internal
) returns internal
  language c
  immutable
  strict;

create function gin_extract_query_timetz(
  time with time zone,
  internal,
  smallint,
  internal,
  internal
) returns internal
  language c
  immutable
  strict;

create function gin_extract_query_uuid(
  uuid,
  internal,
  smallint,
  internal,
  internal
) returns internal
  language c
  immutable
  strict;

create function gin_extract_query_varbit(
  bit varying,
  internal,
  smallint,
  internal,
  internal
) returns internal
  language c
  immutable
  strict;

create function gin_extract_value_anyenum(anyenum, internal) returns internal
  language c
  immutable
  strict;

create function gin_extract_value_bit(bit, internal) returns internal
  language c
  immutable
  strict;

create function gin_extract_value_bool(boolean, internal) returns internal
  language c
  immutable
  strict;

create function gin_extract_value_bpchar(character, internal) returns internal
  language c
  immutable
  strict;

create function gin_extract_value_bytea(bytea, internal) returns internal
  language c
  immutable
  strict;

create function gin_extract_value_char("char", internal) returns internal
  language c
  immutable
  strict;

create function gin_extract_value_cidr(cidr, internal) returns internal
  language c
  immutable
  strict;

create function gin_extract_value_date(date, internal) returns internal
  language c
  immutable
  strict;

create function gin_extract_value_float4(real, internal) returns internal
  language c
  immutable
  strict;

create function gin_extract_value_float8(
  double precision,
  internal
) returns internal
  language c
  immutable
  strict;

create function gin_extract_value_inet(inet, internal) returns internal
  language c
  immutable
  strict;

create function gin_extract_value_int2(smallint, internal) returns internal
  language c
  immutable
  strict;

create function gin_extract_value_int4(integer, internal) returns internal
  language c
  immutable
  strict;

create function gin_extract_value_int8(bigint, internal) returns internal
  language c
  immutable
  strict;

create function gin_extract_value_interval(interval, internal) returns internal
  language c
  immutable
  strict;

create function gin_extract_value_macaddr(macaddr, internal) returns internal
  language c
  immutable
  strict;

create function gin_extract_value_macaddr8(macaddr8, internal) returns internal
  language c
  immutable
  strict;

create function gin_extract_value_money(money, internal) returns internal
  language c
  immutable
  strict;

create function gin_extract_value_name(name, internal) returns internal
  language c
  immutable
  strict;

create function gin_extract_value_numeric(numeric, internal) returns internal
  language c
  immutable
  strict;

create function gin_extract_value_oid(oid, internal) returns internal
  language c
  immutable
  strict;

create function gin_extract_value_text(text, internal) returns internal
  language c
  immutable
  strict;

create function gin_extract_value_time(
  time without time zone,
  internal
) returns internal
  language c
  immutable
  strict;

create function gin_extract_value_timestamp(
  timestamp without time zone,
  internal
) returns internal
  language c
  immutable
  strict;

create function gin_extract_value_timestamptz(
  timestamp with time zone,
  internal
) returns internal
  language c
  immutable
  strict;

create function gin_extract_value_timetz(
  time with time zone,
  internal
) returns internal
  language c
  immutable
  strict;

create function gin_extract_value_uuid(uuid, internal) returns internal
  language c
  immutable
  strict;

create function gin_extract_value_varbit(bit varying, internal) returns internal
  language c
  immutable
  strict;

create function gin_numeric_cmp(numeric, numeric) returns integer
  language c
  immutable
  strict;

create operator family bit_ops
  using gin;

create operator family bool_ops
  using gin;

create operator family bpchar_ops
  using gin;

create operator family bytea_ops
  using gin;

create operator family char_ops
  using gin;

create operator family cidr_ops
  using gin;

create operator family date_ops
  using gin;

create operator family enum_ops
  using gin;

create operator family float4_ops
  using gin;

create operator family float8_ops
  using gin;

create operator family inet_ops
  using gin;

create operator family int2_ops
  using gin;

create operator family int4_ops
  using gin;

create operator family int8_ops
  using gin;

create operator family interval_ops
  using gin;

create operator family macaddr8_ops
  using gin;

create operator family macaddr_ops
  using gin;

create operator family money_ops
  using gin;

create operator family name_ops
  using gin;

create operator family numeric_ops
  using gin;

create operator family oid_ops
  using gin;

create operator family text_ops
  using gin;

create operator family time_ops
  using gin;

create operator family timestamp_ops
  using gin;

create operator family timestamptz_ops
  using gin;

create operator family timetz_ops
  using gin;

create operator family uuid_ops
  using gin;

create operator family varbit_ops
  using gin;

create operator family varchar_ops
  using gin;

create operator class bit_ops
  default for type bit
  using gin
  family bit_ops as
  storage bit;

create operator class bool_ops
  default for type boolean
  using gin
  family bool_ops as
  storage boolean;

create operator class bpchar_ops
  default for type character
  using gin
  family bpchar_ops as
  storage character;

create operator class bytea_ops
  default for type bytea
  using gin
  family bytea_ops as
  storage bytea;

create operator class char_ops
  default for type "char"
  using gin
  family char_ops as
  storage "char";

create operator class cidr_ops
  default for type cidr
  using gin
  family cidr_ops as
  storage cidr;

create operator class date_ops
  default for type date
  using gin
  family date_ops as
  storage date;

create operator class enum_ops
  default for type anyenum
  using gin
  family enum_ops as
  storage anyenum;

create operator class float4_ops
  default for type real
  using gin
  family float4_ops as
  storage real;

create operator class float8_ops
  default for type double precision
  using gin
  family float8_ops as
  storage double precision;

create operator class inet_ops
  default for type inet
  using gin
  family inet_ops as
  storage inet;

create operator class int2_ops
  default for type smallint
  using gin
  family int2_ops as
  storage smallint;

create operator class int4_ops
  default for type integer
  using gin
  family int4_ops as
  storage integer;

create operator class int8_ops
  default for type bigint
  using gin
  family int8_ops as
  storage bigint;

create operator class interval_ops
  default for type interval
  using gin
  family interval_ops as
  storage interval;

create operator class macaddr8_ops
  default for type macaddr8
  using gin
  family macaddr8_ops as
  storage macaddr8;

create operator class macaddr_ops
  default for type macaddr
  using gin
  family macaddr_ops as
  storage macaddr;

create operator class money_ops
  default for type money
  using gin
  family money_ops as
  storage money;

create operator class name_ops
  default for type name
  using gin
  family name_ops as
  storage name;

create operator class numeric_ops
  default for type numeric
  using gin
  family numeric_ops as
  storage numeric;

create operator class oid_ops
  default for type oid
  using gin
  family oid_ops as
  storage oid;

create operator class text_ops
  default for type text
  using gin
  family text_ops as
  storage text;

create operator class time_ops
  default for type time without time zone
  using gin
  family time_ops as
  storage time without time zone;

create operator class timestamp_ops
  default for type timestamp without time zone
  using gin
  family timestamp_ops as
  storage timestamp without time zone;

create operator class timestamptz_ops
  default for type timestamp with time zone
  using gin
  family timestamptz_ops as
  storage timestamp with time zone;

create operator class timetz_ops
  default for type time with time zone
  using gin
  family timetz_ops as
  storage time with time zone;

create operator class uuid_ops
  default for type uuid
  using gin
  family uuid_ops as
  storage uuid;

create operator class varbit_ops
  default for type bit varying
  using gin
  family varbit_ops as
  storage bit varying;

create operator class varchar_ops
  default for type character varying
  using gin
  family varchar_ops as
  storage character varying;
