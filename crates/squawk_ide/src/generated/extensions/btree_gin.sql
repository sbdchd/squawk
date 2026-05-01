-- squawk-ignore-file
-- pg version: 18.3
-- update via:
--   cargo xtask sync-builtins

create function public.gin_btree_consistent(internal, smallint, anyelement, integer, internal, internal) returns boolean
  language c;

create function public.gin_compare_prefix_anyenum(anyenum, anyenum, smallint, internal) returns integer
  language c;

create function public.gin_compare_prefix_bit(bit, bit, smallint, internal) returns integer
  language c;

create function public.gin_compare_prefix_bool(boolean, boolean, smallint, internal) returns integer
  language c;

create function public.gin_compare_prefix_bpchar(character, character, smallint, internal) returns integer
  language c;

create function public.gin_compare_prefix_bytea(bytea, bytea, smallint, internal) returns integer
  language c;

create function public.gin_compare_prefix_char("char", "char", smallint, internal) returns integer
  language c;

create function public.gin_compare_prefix_cidr(cidr, cidr, smallint, internal) returns integer
  language c;

create function public.gin_compare_prefix_date(date, date, smallint, internal) returns integer
  language c;

create function public.gin_compare_prefix_float4(real, real, smallint, internal) returns integer
  language c;

create function public.gin_compare_prefix_float8(double precision, double precision, smallint, internal) returns integer
  language c;

create function public.gin_compare_prefix_inet(inet, inet, smallint, internal) returns integer
  language c;

create function public.gin_compare_prefix_int2(smallint, smallint, smallint, internal) returns integer
  language c;

create function public.gin_compare_prefix_int4(integer, integer, smallint, internal) returns integer
  language c;

create function public.gin_compare_prefix_int8(bigint, bigint, smallint, internal) returns integer
  language c;

create function public.gin_compare_prefix_interval(interval, interval, smallint, internal) returns integer
  language c;

create function public.gin_compare_prefix_macaddr(macaddr, macaddr, smallint, internal) returns integer
  language c;

create function public.gin_compare_prefix_macaddr8(macaddr8, macaddr8, smallint, internal) returns integer
  language c;

create function public.gin_compare_prefix_money(money, money, smallint, internal) returns integer
  language c;

create function public.gin_compare_prefix_name(name, name, smallint, internal) returns integer
  language c;

create function public.gin_compare_prefix_numeric(numeric, numeric, smallint, internal) returns integer
  language c;

create function public.gin_compare_prefix_oid(oid, oid, smallint, internal) returns integer
  language c;

create function public.gin_compare_prefix_text(text, text, smallint, internal) returns integer
  language c;

create function public.gin_compare_prefix_time(time without time zone, time without time zone, smallint, internal) returns integer
  language c;

create function public.gin_compare_prefix_timestamp(timestamp without time zone, timestamp without time zone, smallint, internal) returns integer
  language c;

create function public.gin_compare_prefix_timestamptz(timestamp with time zone, timestamp with time zone, smallint, internal) returns integer
  language c;

create function public.gin_compare_prefix_timetz(time with time zone, time with time zone, smallint, internal) returns integer
  language c;

create function public.gin_compare_prefix_uuid(uuid, uuid, smallint, internal) returns integer
  language c;

create function public.gin_compare_prefix_varbit(bit varying, bit varying, smallint, internal) returns integer
  language c;

create function public.gin_enum_cmp(anyenum, anyenum) returns integer
  language c;

create function public.gin_extract_query_anyenum(anyenum, internal, smallint, internal, internal) returns internal
  language c;

create function public.gin_extract_query_bit(bit, internal, smallint, internal, internal) returns internal
  language c;

create function public.gin_extract_query_bool(boolean, internal, smallint, internal, internal) returns internal
  language c;

create function public.gin_extract_query_bpchar(character, internal, smallint, internal, internal) returns internal
  language c;

create function public.gin_extract_query_bytea(bytea, internal, smallint, internal, internal) returns internal
  language c;

create function public.gin_extract_query_char("char", internal, smallint, internal, internal) returns internal
  language c;

create function public.gin_extract_query_cidr(cidr, internal, smallint, internal, internal) returns internal
  language c;

create function public.gin_extract_query_date(date, internal, smallint, internal, internal) returns internal
  language c;

create function public.gin_extract_query_float4(real, internal, smallint, internal, internal) returns internal
  language c;

create function public.gin_extract_query_float8(double precision, internal, smallint, internal, internal) returns internal
  language c;

create function public.gin_extract_query_inet(inet, internal, smallint, internal, internal) returns internal
  language c;

create function public.gin_extract_query_int2(smallint, internal, smallint, internal, internal) returns internal
  language c;

create function public.gin_extract_query_int4(integer, internal, smallint, internal, internal) returns internal
  language c;

create function public.gin_extract_query_int8(bigint, internal, smallint, internal, internal) returns internal
  language c;

create function public.gin_extract_query_interval(interval, internal, smallint, internal, internal) returns internal
  language c;

create function public.gin_extract_query_macaddr(macaddr, internal, smallint, internal, internal) returns internal
  language c;

create function public.gin_extract_query_macaddr8(macaddr8, internal, smallint, internal, internal) returns internal
  language c;

create function public.gin_extract_query_money(money, internal, smallint, internal, internal) returns internal
  language c;

create function public.gin_extract_query_name(name, internal, smallint, internal, internal) returns internal
  language c;

create function public.gin_extract_query_numeric(numeric, internal, smallint, internal, internal) returns internal
  language c;

create function public.gin_extract_query_oid(oid, internal, smallint, internal, internal) returns internal
  language c;

create function public.gin_extract_query_text(text, internal, smallint, internal, internal) returns internal
  language c;

create function public.gin_extract_query_time(time without time zone, internal, smallint, internal, internal) returns internal
  language c;

create function public.gin_extract_query_timestamp(timestamp without time zone, internal, smallint, internal, internal) returns internal
  language c;

create function public.gin_extract_query_timestamptz(timestamp with time zone, internal, smallint, internal, internal) returns internal
  language c;

create function public.gin_extract_query_timetz(time with time zone, internal, smallint, internal, internal) returns internal
  language c;

create function public.gin_extract_query_uuid(uuid, internal, smallint, internal, internal) returns internal
  language c;

create function public.gin_extract_query_varbit(bit varying, internal, smallint, internal, internal) returns internal
  language c;

create function public.gin_extract_value_anyenum(anyenum, internal) returns internal
  language c;

create function public.gin_extract_value_bit(bit, internal) returns internal
  language c;

create function public.gin_extract_value_bool(boolean, internal) returns internal
  language c;

create function public.gin_extract_value_bpchar(character, internal) returns internal
  language c;

create function public.gin_extract_value_bytea(bytea, internal) returns internal
  language c;

create function public.gin_extract_value_char("char", internal) returns internal
  language c;

create function public.gin_extract_value_cidr(cidr, internal) returns internal
  language c;

create function public.gin_extract_value_date(date, internal) returns internal
  language c;

create function public.gin_extract_value_float4(real, internal) returns internal
  language c;

create function public.gin_extract_value_float8(double precision, internal) returns internal
  language c;

create function public.gin_extract_value_inet(inet, internal) returns internal
  language c;

create function public.gin_extract_value_int2(smallint, internal) returns internal
  language c;

create function public.gin_extract_value_int4(integer, internal) returns internal
  language c;

create function public.gin_extract_value_int8(bigint, internal) returns internal
  language c;

create function public.gin_extract_value_interval(interval, internal) returns internal
  language c;

create function public.gin_extract_value_macaddr(macaddr, internal) returns internal
  language c;

create function public.gin_extract_value_macaddr8(macaddr8, internal) returns internal
  language c;

create function public.gin_extract_value_money(money, internal) returns internal
  language c;

create function public.gin_extract_value_name(name, internal) returns internal
  language c;

create function public.gin_extract_value_numeric(numeric, internal) returns internal
  language c;

create function public.gin_extract_value_oid(oid, internal) returns internal
  language c;

create function public.gin_extract_value_text(text, internal) returns internal
  language c;

create function public.gin_extract_value_time(time without time zone, internal) returns internal
  language c;

create function public.gin_extract_value_timestamp(timestamp without time zone, internal) returns internal
  language c;

create function public.gin_extract_value_timestamptz(timestamp with time zone, internal) returns internal
  language c;

create function public.gin_extract_value_timetz(time with time zone, internal) returns internal
  language c;

create function public.gin_extract_value_uuid(uuid, internal) returns internal
  language c;

create function public.gin_extract_value_varbit(bit varying, internal) returns internal
  language c;

create function public.gin_numeric_cmp(numeric, numeric) returns integer
  language c;

