-- squawk-ignore-file
-- pg version: 18.3
-- update via:
--   cargo xtask sync-builtins

-- size: 16, align: 4
create type gbtreekey16;

-- size: 2, align: 4
create type gbtreekey2;

-- size: 32, align: 4
create type gbtreekey32;

-- size: 4, align: 4
create type gbtreekey4;

-- size: 8, align: 4
create type gbtreekey8;

-- size: -1, align: 4
create type gbtreekey_var;

create function cash_dist(money, money) returns money
  language c
  immutable
  strict
  parallel safe;

create function date_dist(date, date) returns integer
  language c
  immutable
  strict
  parallel safe;

create function float4_dist(real, real) returns real
  language c
  immutable
  strict
  parallel safe;

create function float8_dist(
  double precision,
  double precision
) returns double precision
  language c
  immutable
  strict
  parallel safe;

create function gbt_bit_compress(internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_bit_consistent(
  internal,
  bit,
  smallint,
  oid,
  internal
) returns boolean
  language c
  immutable
  strict
  parallel safe;

create function gbt_bit_penalty(internal, internal, internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_bit_picksplit(internal, internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_bit_same(
  gbtreekey_var,
  gbtreekey_var,
  internal
) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_bit_sortsupport(internal) returns void
  language c
  immutable
  strict
  parallel safe;

create function gbt_bit_union(internal, internal) returns gbtreekey_var
  language c
  immutable
  strict
  parallel safe;

create function gbt_bool_compress(internal) returns internal
  language c
  immutable
  strict;

create function gbt_bool_consistent(
  internal,
  boolean,
  smallint,
  oid,
  internal
) returns boolean
  language c
  immutable
  strict;

create function gbt_bool_fetch(internal) returns internal
  language c
  immutable
  strict;

create function gbt_bool_penalty(internal, internal, internal) returns internal
  language c
  immutable
  strict;

create function gbt_bool_picksplit(internal, internal) returns internal
  language c
  immutable
  strict;

create function gbt_bool_same(gbtreekey2, gbtreekey2, internal) returns internal
  language c
  immutable
  strict;

create function gbt_bool_sortsupport(internal) returns void
  language c
  immutable
  strict
  parallel safe;

create function gbt_bool_union(internal, internal) returns gbtreekey2
  language c
  immutable
  strict;

create function gbt_bpchar_compress(internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_bpchar_consistent(
  internal,
  character,
  smallint,
  oid,
  internal
) returns boolean
  language c
  immutable
  strict
  parallel safe;

create function gbt_bpchar_sortsupport(internal) returns void
  language c
  immutable
  strict
  parallel safe;

create function gbt_bytea_compress(internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_bytea_consistent(
  internal,
  bytea,
  smallint,
  oid,
  internal
) returns boolean
  language c
  immutable
  strict
  parallel safe;

create function gbt_bytea_penalty(internal, internal, internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_bytea_picksplit(internal, internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_bytea_same(
  gbtreekey_var,
  gbtreekey_var,
  internal
) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_bytea_sortsupport(internal) returns void
  language c
  immutable
  strict
  parallel safe;

create function gbt_bytea_union(internal, internal) returns gbtreekey_var
  language c
  immutable
  strict
  parallel safe;

create function gbt_cash_compress(internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_cash_consistent(
  internal,
  money,
  smallint,
  oid,
  internal
) returns boolean
  language c
  immutable
  strict
  parallel safe;

create function gbt_cash_distance(
  internal,
  money,
  smallint,
  oid,
  internal
) returns double precision
  language c
  immutable
  strict
  parallel safe;

create function gbt_cash_fetch(internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_cash_penalty(internal, internal, internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_cash_picksplit(internal, internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_cash_same(
  gbtreekey16,
  gbtreekey16,
  internal
) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_cash_sortsupport(internal) returns void
  language c
  immutable
  strict
  parallel safe;

create function gbt_cash_union(internal, internal) returns gbtreekey16
  language c
  immutable
  strict
  parallel safe;

create function gbt_date_compress(internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_date_consistent(
  internal,
  date,
  smallint,
  oid,
  internal
) returns boolean
  language c
  immutable
  strict
  parallel safe;

create function gbt_date_distance(
  internal,
  date,
  smallint,
  oid,
  internal
) returns double precision
  language c
  immutable
  strict
  parallel safe;

create function gbt_date_fetch(internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_date_penalty(internal, internal, internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_date_picksplit(internal, internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_date_same(gbtreekey8, gbtreekey8, internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_date_sortsupport(internal) returns void
  language c
  immutable
  strict
  parallel safe;

create function gbt_date_union(internal, internal) returns gbtreekey8
  language c
  immutable
  strict
  parallel safe;

create function gbt_decompress(internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_enum_compress(internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_enum_consistent(
  internal,
  anyenum,
  smallint,
  oid,
  internal
) returns boolean
  language c
  immutable
  strict
  parallel safe;

create function gbt_enum_fetch(internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_enum_penalty(internal, internal, internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_enum_picksplit(internal, internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_enum_same(gbtreekey8, gbtreekey8, internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_enum_sortsupport(internal) returns void
  language c
  immutable
  strict
  parallel safe;

create function gbt_enum_union(internal, internal) returns gbtreekey8
  language c
  immutable
  strict
  parallel safe;

create function gbt_float4_compress(internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_float4_consistent(
  internal,
  real,
  smallint,
  oid,
  internal
) returns boolean
  language c
  immutable
  strict
  parallel safe;

create function gbt_float4_distance(
  internal,
  real,
  smallint,
  oid,
  internal
) returns double precision
  language c
  immutable
  strict
  parallel safe;

create function gbt_float4_fetch(internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_float4_penalty(
  internal,
  internal,
  internal
) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_float4_picksplit(internal, internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_float4_same(
  gbtreekey8,
  gbtreekey8,
  internal
) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_float4_sortsupport(internal) returns void
  language c
  immutable
  strict
  parallel safe;

create function gbt_float4_union(internal, internal) returns gbtreekey8
  language c
  immutable
  strict
  parallel safe;

create function gbt_float8_compress(internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_float8_consistent(
  internal,
  double precision,
  smallint,
  oid,
  internal
) returns boolean
  language c
  immutable
  strict
  parallel safe;

create function gbt_float8_distance(
  internal,
  double precision,
  smallint,
  oid,
  internal
) returns double precision
  language c
  immutable
  strict
  parallel safe;

create function gbt_float8_fetch(internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_float8_penalty(
  internal,
  internal,
  internal
) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_float8_picksplit(internal, internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_float8_same(
  gbtreekey16,
  gbtreekey16,
  internal
) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_float8_sortsupport(internal) returns void
  language c
  immutable
  strict
  parallel safe;

create function gbt_float8_union(internal, internal) returns gbtreekey16
  language c
  immutable
  strict
  parallel safe;

create function gbt_inet_compress(internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_inet_consistent(
  internal,
  inet,
  smallint,
  oid,
  internal
) returns boolean
  language c
  immutable
  strict
  parallel safe;

create function gbt_inet_penalty(internal, internal, internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_inet_picksplit(internal, internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_inet_same(
  gbtreekey16,
  gbtreekey16,
  internal
) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_inet_sortsupport(internal) returns void
  language c
  immutable
  strict
  parallel safe;

create function gbt_inet_union(internal, internal) returns gbtreekey16
  language c
  immutable
  strict
  parallel safe;

create function gbt_int2_compress(internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_int2_consistent(
  internal,
  smallint,
  smallint,
  oid,
  internal
) returns boolean
  language c
  immutable
  strict
  parallel safe;

create function gbt_int2_distance(
  internal,
  smallint,
  smallint,
  oid,
  internal
) returns double precision
  language c
  immutable
  strict
  parallel safe;

create function gbt_int2_fetch(internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_int2_penalty(internal, internal, internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_int2_picksplit(internal, internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_int2_same(gbtreekey4, gbtreekey4, internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_int2_sortsupport(internal) returns void
  language c
  immutable
  strict
  parallel safe;

create function gbt_int2_union(internal, internal) returns gbtreekey4
  language c
  immutable
  strict
  parallel safe;

create function gbt_int4_compress(internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_int4_consistent(
  internal,
  integer,
  smallint,
  oid,
  internal
) returns boolean
  language c
  immutable
  strict
  parallel safe;

create function gbt_int4_distance(
  internal,
  integer,
  smallint,
  oid,
  internal
) returns double precision
  language c
  immutable
  strict
  parallel safe;

create function gbt_int4_fetch(internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_int4_penalty(internal, internal, internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_int4_picksplit(internal, internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_int4_same(gbtreekey8, gbtreekey8, internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_int4_sortsupport(internal) returns void
  language c
  immutable
  strict
  parallel safe;

create function gbt_int4_union(internal, internal) returns gbtreekey8
  language c
  immutable
  strict
  parallel safe;

create function gbt_int8_compress(internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_int8_consistent(
  internal,
  bigint,
  smallint,
  oid,
  internal
) returns boolean
  language c
  immutable
  strict
  parallel safe;

create function gbt_int8_distance(
  internal,
  bigint,
  smallint,
  oid,
  internal
) returns double precision
  language c
  immutable
  strict
  parallel safe;

create function gbt_int8_fetch(internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_int8_penalty(internal, internal, internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_int8_picksplit(internal, internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_int8_same(
  gbtreekey16,
  gbtreekey16,
  internal
) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_int8_sortsupport(internal) returns void
  language c
  immutable
  strict
  parallel safe;

create function gbt_int8_union(internal, internal) returns gbtreekey16
  language c
  immutable
  strict
  parallel safe;

create function gbt_intv_compress(internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_intv_consistent(
  internal,
  interval,
  smallint,
  oid,
  internal
) returns boolean
  language c
  immutable
  strict
  parallel safe;

create function gbt_intv_decompress(internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_intv_distance(
  internal,
  interval,
  smallint,
  oid,
  internal
) returns double precision
  language c
  immutable
  strict
  parallel safe;

create function gbt_intv_fetch(internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_intv_penalty(internal, internal, internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_intv_picksplit(internal, internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_intv_same(
  gbtreekey32,
  gbtreekey32,
  internal
) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_intv_sortsupport(internal) returns void
  language c
  immutable
  strict
  parallel safe;

create function gbt_intv_union(internal, internal) returns gbtreekey32
  language c
  immutable
  strict
  parallel safe;

create function gbt_macad8_compress(internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_macad8_consistent(
  internal,
  macaddr8,
  smallint,
  oid,
  internal
) returns boolean
  language c
  immutable
  strict
  parallel safe;

create function gbt_macad8_fetch(internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_macad8_penalty(
  internal,
  internal,
  internal
) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_macad8_picksplit(internal, internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_macad8_same(
  gbtreekey16,
  gbtreekey16,
  internal
) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_macad8_sortsupport(internal) returns void
  language c
  immutable
  strict
  parallel safe;

create function gbt_macad8_union(internal, internal) returns gbtreekey16
  language c
  immutable
  strict
  parallel safe;

create function gbt_macad_compress(internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_macad_consistent(
  internal,
  macaddr,
  smallint,
  oid,
  internal
) returns boolean
  language c
  immutable
  strict
  parallel safe;

create function gbt_macad_fetch(internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_macad_penalty(internal, internal, internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_macad_picksplit(internal, internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_macad_same(
  gbtreekey16,
  gbtreekey16,
  internal
) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_macad_union(internal, internal) returns gbtreekey16
  language c
  immutable
  strict
  parallel safe;

create function gbt_macaddr_sortsupport(internal) returns void
  language c
  immutable
  strict
  parallel safe;

create function gbt_numeric_compress(internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_numeric_consistent(
  internal,
  numeric,
  smallint,
  oid,
  internal
) returns boolean
  language c
  immutable
  strict
  parallel safe;

create function gbt_numeric_penalty(
  internal,
  internal,
  internal
) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_numeric_picksplit(internal, internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_numeric_same(
  gbtreekey_var,
  gbtreekey_var,
  internal
) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_numeric_sortsupport(internal) returns void
  language c
  immutable
  strict
  parallel safe;

create function gbt_numeric_union(internal, internal) returns gbtreekey_var
  language c
  immutable
  strict
  parallel safe;

create function gbt_oid_compress(internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_oid_consistent(
  internal,
  oid,
  smallint,
  oid,
  internal
) returns boolean
  language c
  immutable
  strict
  parallel safe;

create function gbt_oid_distance(
  internal,
  oid,
  smallint,
  oid,
  internal
) returns double precision
  language c
  immutable
  strict
  parallel safe;

create function gbt_oid_fetch(internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_oid_penalty(internal, internal, internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_oid_picksplit(internal, internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_oid_same(gbtreekey8, gbtreekey8, internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_oid_sortsupport(internal) returns void
  language c
  immutable
  strict
  parallel safe;

create function gbt_oid_union(internal, internal) returns gbtreekey8
  language c
  immutable
  strict
  parallel safe;

create function gbt_text_compress(internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_text_consistent(
  internal,
  text,
  smallint,
  oid,
  internal
) returns boolean
  language c
  immutable
  strict
  parallel safe;

create function gbt_text_penalty(internal, internal, internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_text_picksplit(internal, internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_text_same(
  gbtreekey_var,
  gbtreekey_var,
  internal
) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_text_sortsupport(internal) returns void
  language c
  immutable
  strict
  parallel safe;

create function gbt_text_union(internal, internal) returns gbtreekey_var
  language c
  immutable
  strict
  parallel safe;

create function gbt_time_compress(internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_time_consistent(
  internal,
  time without time zone,
  smallint,
  oid,
  internal
) returns boolean
  language c
  immutable
  strict
  parallel safe;

create function gbt_time_distance(
  internal,
  time without time zone,
  smallint,
  oid,
  internal
) returns double precision
  language c
  immutable
  strict
  parallel safe;

create function gbt_time_fetch(internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_time_penalty(internal, internal, internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_time_picksplit(internal, internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_time_same(
  gbtreekey16,
  gbtreekey16,
  internal
) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_time_sortsupport(internal) returns void
  language c
  immutable
  strict
  parallel safe;

create function gbt_time_union(internal, internal) returns gbtreekey16
  language c
  immutable
  strict
  parallel safe;

create function gbt_timetz_compress(internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_timetz_consistent(
  internal,
  time with time zone,
  smallint,
  oid,
  internal
) returns boolean
  language c
  immutable
  strict
  parallel safe;

create function gbt_ts_compress(internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_ts_consistent(
  internal,
  timestamp without time zone,
  smallint,
  oid,
  internal
) returns boolean
  language c
  immutable
  strict
  parallel safe;

create function gbt_ts_distance(
  internal,
  timestamp without time zone,
  smallint,
  oid,
  internal
) returns double precision
  language c
  immutable
  strict
  parallel safe;

create function gbt_ts_fetch(internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_ts_penalty(internal, internal, internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_ts_picksplit(internal, internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_ts_same(gbtreekey16, gbtreekey16, internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_ts_sortsupport(internal) returns void
  language c
  immutable
  strict
  parallel safe;

create function gbt_ts_union(internal, internal) returns gbtreekey16
  language c
  immutable
  strict
  parallel safe;

create function gbt_tstz_compress(internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_tstz_consistent(
  internal,
  timestamp with time zone,
  smallint,
  oid,
  internal
) returns boolean
  language c
  immutable
  strict
  parallel safe;

create function gbt_tstz_distance(
  internal,
  timestamp with time zone,
  smallint,
  oid,
  internal
) returns double precision
  language c
  immutable
  strict
  parallel safe;

create function gbt_uuid_compress(internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_uuid_consistent(
  internal,
  uuid,
  smallint,
  oid,
  internal
) returns boolean
  language c
  immutable
  strict
  parallel safe;

create function gbt_uuid_fetch(internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_uuid_penalty(internal, internal, internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_uuid_picksplit(internal, internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_uuid_same(
  gbtreekey32,
  gbtreekey32,
  internal
) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_uuid_sortsupport(internal) returns void
  language c
  immutable
  strict
  parallel safe;

create function gbt_uuid_union(internal, internal) returns gbtreekey32
  language c
  immutable
  strict
  parallel safe;

create function gbt_var_decompress(internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_var_fetch(internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gbt_varbit_sortsupport(internal) returns void
  language c
  immutable
  strict
  parallel safe;

create function gbtreekey16_in(cstring) returns gbtreekey16
  language c
  immutable
  strict
  parallel safe;

create function gbtreekey16_out(gbtreekey16) returns cstring
  language c
  immutable
  strict
  parallel safe;

create function gbtreekey2_in(cstring) returns gbtreekey2
  language c
  immutable
  strict
  parallel safe;

create function gbtreekey2_out(gbtreekey2) returns cstring
  language c
  immutable
  strict
  parallel safe;

create function gbtreekey32_in(cstring) returns gbtreekey32
  language c
  immutable
  strict
  parallel safe;

create function gbtreekey32_out(gbtreekey32) returns cstring
  language c
  immutable
  strict
  parallel safe;

create function gbtreekey4_in(cstring) returns gbtreekey4
  language c
  immutable
  strict
  parallel safe;

create function gbtreekey4_out(gbtreekey4) returns cstring
  language c
  immutable
  strict
  parallel safe;

create function gbtreekey8_in(cstring) returns gbtreekey8
  language c
  immutable
  strict
  parallel safe;

create function gbtreekey8_out(gbtreekey8) returns cstring
  language c
  immutable
  strict
  parallel safe;

create function gbtreekey_var_in(cstring) returns gbtreekey_var
  language c
  immutable
  strict
  parallel safe;

create function gbtreekey_var_out(gbtreekey_var) returns cstring
  language c
  immutable
  strict
  parallel safe;

create function gist_translate_cmptype_btree(integer) returns smallint
  language c
  immutable
  strict
  parallel safe;

create function int2_dist(smallint, smallint) returns smallint
  language c
  immutable
  strict
  parallel safe;

create function int4_dist(integer, integer) returns integer
  language c
  immutable
  strict
  parallel safe;

create function int8_dist(bigint, bigint) returns bigint
  language c
  immutable
  strict
  parallel safe;

create function interval_dist(interval, interval) returns interval
  language c
  immutable
  strict
  parallel safe;

create function oid_dist(oid, oid) returns oid
  language c
  immutable
  strict
  parallel safe;

create function time_dist(
  time without time zone,
  time without time zone
) returns interval
  language c
  immutable
  strict
  parallel safe;

create function ts_dist(
  timestamp without time zone,
  timestamp without time zone
) returns interval
  language c
  immutable
  strict
  parallel safe;

create function tstz_dist(
  timestamp with time zone,
  timestamp with time zone
) returns interval
  language c
  immutable
  strict
  parallel safe;

create type gbtreekey16 (
  input = gbtreekey16_in,
  output = gbtreekey16_out
);

create type gbtreekey2 (
  input = gbtreekey2_in,
  output = gbtreekey2_out
);

create type gbtreekey32 (
  input = gbtreekey32_in,
  output = gbtreekey32_out
);

create type gbtreekey4 (
  input = gbtreekey4_in,
  output = gbtreekey4_out
);

create type gbtreekey8 (
  input = gbtreekey8_in,
  output = gbtreekey8_out
);

create type gbtreekey_var (
  input = gbtreekey_var_in,
  output = gbtreekey_var_out
);

create operator <-> (
  leftarg = bigint,
  rightarg = bigint,
  function = int8_dist,
  commutator = <->
);

create operator <-> (
  leftarg = date,
  rightarg = date,
  function = date_dist,
  commutator = <->
);

create operator <-> (
  leftarg = double precision,
  rightarg = double precision,
  function = float8_dist,
  commutator = <->
);

create operator <-> (
  leftarg = integer,
  rightarg = integer,
  function = int4_dist,
  commutator = <->
);

create operator <-> (
  leftarg = interval,
  rightarg = interval,
  function = interval_dist,
  commutator = <->
);

create operator <-> (
  leftarg = money,
  rightarg = money,
  function = cash_dist,
  commutator = <->
);

create operator <-> (
  leftarg = oid,
  rightarg = oid,
  function = oid_dist,
  commutator = <->
);

create operator <-> (
  leftarg = real,
  rightarg = real,
  function = float4_dist,
  commutator = <->
);

create operator <-> (
  leftarg = smallint,
  rightarg = smallint,
  function = int2_dist,
  commutator = <->
);

create operator <-> (
  leftarg = time without time zone,
  rightarg = time without time zone,
  function = time_dist,
  commutator = <->
);

create operator <-> (
  leftarg = timestamp with time zone,
  rightarg = timestamp with time zone,
  function = tstz_dist,
  commutator = <->
);

create operator <-> (
  leftarg = timestamp without time zone,
  rightarg = timestamp without time zone,
  function = ts_dist,
  commutator = <->
);

create operator family gist_bit_ops
  using gist;

create operator family gist_bool_ops
  using gist;

create operator family gist_bpchar_ops
  using gist;

create operator family gist_bytea_ops
  using gist;

create operator family gist_cash_ops
  using gist;

create operator family gist_cidr_ops
  using gist;

create operator family gist_date_ops
  using gist;

create operator family gist_enum_ops
  using gist;

create operator family gist_float4_ops
  using gist;

create operator family gist_float8_ops
  using gist;

create operator family gist_inet_ops
  using gist;

create operator family gist_int2_ops
  using gist;

create operator family gist_int4_ops
  using gist;

create operator family gist_int8_ops
  using gist;

create operator family gist_interval_ops
  using gist;

create operator family gist_macaddr8_ops
  using gist;

create operator family gist_macaddr_ops
  using gist;

create operator family gist_numeric_ops
  using gist;

create operator family gist_oid_ops
  using gist;

create operator family gist_text_ops
  using gist;

create operator family gist_time_ops
  using gist;

create operator family gist_timestamp_ops
  using gist;

create operator family gist_timestamptz_ops
  using gist;

create operator family gist_timetz_ops
  using gist;

create operator family gist_uuid_ops
  using gist;

create operator family gist_vbit_ops
  using gist;

create operator class gist_bit_ops
  default for type bit
  using gist
  family gist_bit_ops as
    function 1 (bit, bit) gbt_bit_consistent(
      internal,
      bit,
      smallint,
      oid,
      internal
    ),
    function 2 (bit, bit) gbt_bit_union(internal, internal),
    function 5 (bit, bit) gbt_bit_penalty(internal, internal, internal),
    function 6 (bit, bit) gbt_bit_picksplit(internal, internal),
    function 7 (bit, bit) gbt_bit_same(gbtreekey_var, gbtreekey_var, internal),
    storage gbtreekey_var;

create operator class gist_bool_ops
  default for type boolean
  using gist
  family gist_bool_ops as
    function 1 (boolean, boolean) gbt_bool_consistent(
      internal,
      boolean,
      smallint,
      oid,
      internal
    ),
    function 2 (boolean, boolean) gbt_bool_union(internal, internal),
    function 5 (boolean, boolean) gbt_bool_penalty(
      internal,
      internal,
      internal
    ),
    function 6 (boolean, boolean) gbt_bool_picksplit(internal, internal),
    function 7 (boolean, boolean) gbt_bool_same(
      gbtreekey2,
      gbtreekey2,
      internal
    ),
    storage gbtreekey2;

create operator class gist_bpchar_ops
  default for type character
  using gist
  family gist_bpchar_ops as
    function 1 (character, character) gbt_bpchar_consistent(
      internal,
      character,
      smallint,
      oid,
      internal
    ),
    function 2 (character, character) gbt_text_union(internal, internal),
    function 5 (character, character) gbt_text_penalty(
      internal,
      internal,
      internal
    ),
    function 6 (character, character) gbt_text_picksplit(internal, internal),
    function 7 (character, character) gbt_text_same(
      gbtreekey_var,
      gbtreekey_var,
      internal
    ),
    storage gbtreekey_var;

create operator class gist_bytea_ops
  default for type bytea
  using gist
  family gist_bytea_ops as
    function 1 (bytea, bytea) gbt_bytea_consistent(
      internal,
      bytea,
      smallint,
      oid,
      internal
    ),
    function 2 (bytea, bytea) gbt_bytea_union(internal, internal),
    function 5 (bytea, bytea) gbt_bytea_penalty(internal, internal, internal),
    function 6 (bytea, bytea) gbt_bytea_picksplit(internal, internal),
    function 7 (bytea, bytea) gbt_bytea_same(
      gbtreekey_var,
      gbtreekey_var,
      internal
    ),
    storage gbtreekey_var;

create operator class gist_cash_ops
  default for type money
  using gist
  family gist_cash_ops as
    function 1 (money, money) gbt_cash_consistent(
      internal,
      money,
      smallint,
      oid,
      internal
    ),
    function 2 (money, money) gbt_cash_union(internal, internal),
    function 5 (money, money) gbt_cash_penalty(internal, internal, internal),
    function 6 (money, money) gbt_cash_picksplit(internal, internal),
    function 7 (money, money) gbt_cash_same(gbtreekey16, gbtreekey16, internal),
    storage gbtreekey16;

create operator class gist_cidr_ops
  default for type cidr
  using gist
  family gist_cidr_ops as
    function 1 (cidr, cidr) gbt_inet_consistent(
      internal,
      inet,
      smallint,
      oid,
      internal
    ),
    function 2 (cidr, cidr) gbt_inet_union(internal, internal),
    function 5 (cidr, cidr) gbt_inet_penalty(internal, internal, internal),
    function 6 (cidr, cidr) gbt_inet_picksplit(internal, internal),
    function 7 (cidr, cidr) gbt_inet_same(gbtreekey16, gbtreekey16, internal),
    storage gbtreekey16;

create operator class gist_date_ops
  default for type date
  using gist
  family gist_date_ops as
    function 1 (date, date) gbt_date_consistent(
      internal,
      date,
      smallint,
      oid,
      internal
    ),
    function 2 (date, date) gbt_date_union(internal, internal),
    function 5 (date, date) gbt_date_penalty(internal, internal, internal),
    function 6 (date, date) gbt_date_picksplit(internal, internal),
    function 7 (date, date) gbt_date_same(gbtreekey8, gbtreekey8, internal),
    storage gbtreekey8;

create operator class gist_enum_ops
  default for type anyenum
  using gist
  family gist_enum_ops as
    function 1 (anyenum, anyenum) gbt_enum_consistent(
      internal,
      anyenum,
      smallint,
      oid,
      internal
    ),
    function 2 (anyenum, anyenum) gbt_enum_union(internal, internal),
    function 5 (anyenum, anyenum) gbt_enum_penalty(
      internal,
      internal,
      internal
    ),
    function 6 (anyenum, anyenum) gbt_enum_picksplit(internal, internal),
    function 7 (anyenum, anyenum) gbt_enum_same(
      gbtreekey8,
      gbtreekey8,
      internal
    ),
    storage gbtreekey8;

create operator class gist_float4_ops
  default for type real
  using gist
  family gist_float4_ops as
    function 1 (real, real) gbt_float4_consistent(
      internal,
      real,
      smallint,
      oid,
      internal
    ),
    function 2 (real, real) gbt_float4_union(internal, internal),
    function 5 (real, real) gbt_float4_penalty(internal, internal, internal),
    function 6 (real, real) gbt_float4_picksplit(internal, internal),
    function 7 (real, real) gbt_float4_same(gbtreekey8, gbtreekey8, internal),
    storage gbtreekey8;

create operator class gist_float8_ops
  default for type double precision
  using gist
  family gist_float8_ops as
    function 1 (double precision, double precision) gbt_float8_consistent(
      internal,
      double precision,
      smallint,
      oid,
      internal
    ),
    function 2 (double precision, double precision) gbt_float8_union(
      internal,
      internal
    ),
    function 5 (double precision, double precision) gbt_float8_penalty(
      internal,
      internal,
      internal
    ),
    function 6 (double precision, double precision) gbt_float8_picksplit(
      internal,
      internal
    ),
    function 7 (double precision, double precision) gbt_float8_same(
      gbtreekey16,
      gbtreekey16,
      internal
    ),
    storage gbtreekey16;

create operator class gist_inet_ops
  default for type inet
  using gist
  family gist_inet_ops as
    function 1 (inet, inet) gbt_inet_consistent(
      internal,
      inet,
      smallint,
      oid,
      internal
    ),
    function 2 (inet, inet) gbt_inet_union(internal, internal),
    function 5 (inet, inet) gbt_inet_penalty(internal, internal, internal),
    function 6 (inet, inet) gbt_inet_picksplit(internal, internal),
    function 7 (inet, inet) gbt_inet_same(gbtreekey16, gbtreekey16, internal),
    storage gbtreekey16;

create operator class gist_int2_ops
  default for type smallint
  using gist
  family gist_int2_ops as
    function 1 (smallint, smallint) gbt_int2_consistent(
      internal,
      smallint,
      smallint,
      oid,
      internal
    ),
    function 2 (smallint, smallint) gbt_int2_union(internal, internal),
    function 5 (smallint, smallint) gbt_int2_penalty(
      internal,
      internal,
      internal
    ),
    function 6 (smallint, smallint) gbt_int2_picksplit(internal, internal),
    function 7 (smallint, smallint) gbt_int2_same(
      gbtreekey4,
      gbtreekey4,
      internal
    ),
    storage gbtreekey4;

create operator class gist_int4_ops
  default for type integer
  using gist
  family gist_int4_ops as
    function 1 (integer, integer) gbt_int4_consistent(
      internal,
      integer,
      smallint,
      oid,
      internal
    ),
    function 2 (integer, integer) gbt_int4_union(internal, internal),
    function 5 (integer, integer) gbt_int4_penalty(
      internal,
      internal,
      internal
    ),
    function 6 (integer, integer) gbt_int4_picksplit(internal, internal),
    function 7 (integer, integer) gbt_int4_same(
      gbtreekey8,
      gbtreekey8,
      internal
    ),
    storage gbtreekey8;

create operator class gist_int8_ops
  default for type bigint
  using gist
  family gist_int8_ops as
    function 1 (bigint, bigint) gbt_int8_consistent(
      internal,
      bigint,
      smallint,
      oid,
      internal
    ),
    function 2 (bigint, bigint) gbt_int8_union(internal, internal),
    function 5 (bigint, bigint) gbt_int8_penalty(internal, internal, internal),
    function 6 (bigint, bigint) gbt_int8_picksplit(internal, internal),
    function 7 (bigint, bigint) gbt_int8_same(
      gbtreekey16,
      gbtreekey16,
      internal
    ),
    storage gbtreekey16;

create operator class gist_interval_ops
  default for type interval
  using gist
  family gist_interval_ops as
    function 1 (interval, interval) gbt_intv_consistent(
      internal,
      interval,
      smallint,
      oid,
      internal
    ),
    function 2 (interval, interval) gbt_intv_union(internal, internal),
    function 5 (interval, interval) gbt_intv_penalty(
      internal,
      internal,
      internal
    ),
    function 6 (interval, interval) gbt_intv_picksplit(internal, internal),
    function 7 (interval, interval) gbt_intv_same(
      gbtreekey32,
      gbtreekey32,
      internal
    ),
    storage gbtreekey32;

create operator class gist_macaddr8_ops
  default for type macaddr8
  using gist
  family gist_macaddr8_ops as
    function 1 (macaddr8, macaddr8) gbt_macad8_consistent(
      internal,
      macaddr8,
      smallint,
      oid,
      internal
    ),
    function 2 (macaddr8, macaddr8) gbt_macad8_union(internal, internal),
    function 5 (macaddr8, macaddr8) gbt_macad8_penalty(
      internal,
      internal,
      internal
    ),
    function 6 (macaddr8, macaddr8) gbt_macad8_picksplit(internal, internal),
    function 7 (macaddr8, macaddr8) gbt_macad8_same(
      gbtreekey16,
      gbtreekey16,
      internal
    ),
    storage gbtreekey16;

create operator class gist_macaddr_ops
  default for type macaddr
  using gist
  family gist_macaddr_ops as
    function 1 (macaddr, macaddr) gbt_macad_consistent(
      internal,
      macaddr,
      smallint,
      oid,
      internal
    ),
    function 2 (macaddr, macaddr) gbt_macad_union(internal, internal),
    function 5 (macaddr, macaddr) gbt_macad_penalty(
      internal,
      internal,
      internal
    ),
    function 6 (macaddr, macaddr) gbt_macad_picksplit(internal, internal),
    function 7 (macaddr, macaddr) gbt_macad_same(
      gbtreekey16,
      gbtreekey16,
      internal
    ),
    storage gbtreekey16;

create operator class gist_numeric_ops
  default for type numeric
  using gist
  family gist_numeric_ops as
    function 1 (numeric, numeric) gbt_numeric_consistent(
      internal,
      numeric,
      smallint,
      oid,
      internal
    ),
    function 2 (numeric, numeric) gbt_numeric_union(internal, internal),
    function 5 (numeric, numeric) gbt_numeric_penalty(
      internal,
      internal,
      internal
    ),
    function 6 (numeric, numeric) gbt_numeric_picksplit(internal, internal),
    function 7 (numeric, numeric) gbt_numeric_same(
      gbtreekey_var,
      gbtreekey_var,
      internal
    ),
    storage gbtreekey_var;

create operator class gist_oid_ops
  default for type oid
  using gist
  family gist_oid_ops as
    function 1 (oid, oid) gbt_oid_consistent(
      internal,
      oid,
      smallint,
      oid,
      internal
    ),
    function 2 (oid, oid) gbt_oid_union(internal, internal),
    function 5 (oid, oid) gbt_oid_penalty(internal, internal, internal),
    function 6 (oid, oid) gbt_oid_picksplit(internal, internal),
    function 7 (oid, oid) gbt_oid_same(gbtreekey8, gbtreekey8, internal),
    storage gbtreekey8;

create operator class gist_text_ops
  default for type text
  using gist
  family gist_text_ops as
    function 1 (text, text) gbt_text_consistent(
      internal,
      text,
      smallint,
      oid,
      internal
    ),
    function 2 (text, text) gbt_text_union(internal, internal),
    function 5 (text, text) gbt_text_penalty(internal, internal, internal),
    function 6 (text, text) gbt_text_picksplit(internal, internal),
    function 7 (text, text) gbt_text_same(
      gbtreekey_var,
      gbtreekey_var,
      internal
    ),
    storage gbtreekey_var;

create operator class gist_time_ops
  default for type time without time zone
  using gist
  family gist_time_ops as
    function 1 (
      time without time zone,
      time without time zone
    ) gbt_time_consistent(
      internal,
      time without time zone,
      smallint,
      oid,
      internal
    ),
    function 2 (time without time zone, time without time zone) gbt_time_union(
      internal,
      internal
    ),
    function 5 (
      time without time zone,
      time without time zone
    ) gbt_time_penalty(
      internal,
      internal,
      internal
    ),
    function 6 (
      time without time zone,
      time without time zone
    ) gbt_time_picksplit(
      internal,
      internal
    ),
    function 7 (time without time zone, time without time zone) gbt_time_same(
      gbtreekey16,
      gbtreekey16,
      internal
    ),
    storage gbtreekey16;

create operator class gist_timestamp_ops
  default for type timestamp without time zone
  using gist
  family gist_timestamp_ops as
    function 1 (
      timestamp without time zone,
      timestamp without time zone
    ) gbt_ts_consistent(
      internal,
      timestamp without time zone,
      smallint,
      oid,
      internal
    ),
    function 2 (
      timestamp without time zone,
      timestamp without time zone
    ) gbt_ts_union(
      internal,
      internal
    ),
    function 5 (
      timestamp without time zone,
      timestamp without time zone
    ) gbt_ts_penalty(
      internal,
      internal,
      internal
    ),
    function 6 (
      timestamp without time zone,
      timestamp without time zone
    ) gbt_ts_picksplit(
      internal,
      internal
    ),
    function 7 (
      timestamp without time zone,
      timestamp without time zone
    ) gbt_ts_same(
      gbtreekey16,
      gbtreekey16,
      internal
    ),
    storage gbtreekey16;

create operator class gist_timestamptz_ops
  default for type timestamp with time zone
  using gist
  family gist_timestamptz_ops as
    function 1 (
      timestamp with time zone,
      timestamp with time zone
    ) gbt_tstz_consistent(
      internal,
      timestamp with time zone,
      smallint,
      oid,
      internal
    ),
    function 2 (
      timestamp with time zone,
      timestamp with time zone
    ) gbt_ts_union(
      internal,
      internal
    ),
    function 5 (
      timestamp with time zone,
      timestamp with time zone
    ) gbt_ts_penalty(
      internal,
      internal,
      internal
    ),
    function 6 (
      timestamp with time zone,
      timestamp with time zone
    ) gbt_ts_picksplit(
      internal,
      internal
    ),
    function 7 (timestamp with time zone, timestamp with time zone) gbt_ts_same(
      gbtreekey16,
      gbtreekey16,
      internal
    ),
    storage gbtreekey16;

create operator class gist_timetz_ops
  default for type time with time zone
  using gist
  family gist_timetz_ops as
    function 1 (time with time zone, time with time zone) gbt_timetz_consistent(
      internal,
      time with time zone,
      smallint,
      oid,
      internal
    ),
    function 2 (time with time zone, time with time zone) gbt_time_union(
      internal,
      internal
    ),
    function 5 (time with time zone, time with time zone) gbt_time_penalty(
      internal,
      internal,
      internal
    ),
    function 6 (time with time zone, time with time zone) gbt_time_picksplit(
      internal,
      internal
    ),
    function 7 (time with time zone, time with time zone) gbt_time_same(
      gbtreekey16,
      gbtreekey16,
      internal
    ),
    storage gbtreekey16;

create operator class gist_uuid_ops
  default for type uuid
  using gist
  family gist_uuid_ops as
    function 1 (uuid, uuid) gbt_uuid_consistent(
      internal,
      uuid,
      smallint,
      oid,
      internal
    ),
    function 2 (uuid, uuid) gbt_uuid_union(internal, internal),
    function 5 (uuid, uuid) gbt_uuid_penalty(internal, internal, internal),
    function 6 (uuid, uuid) gbt_uuid_picksplit(internal, internal),
    function 7 (uuid, uuid) gbt_uuid_same(gbtreekey32, gbtreekey32, internal),
    storage gbtreekey32;

create operator class gist_vbit_ops
  default for type bit varying
  using gist
  family gist_vbit_ops as
    function 1 (bit varying, bit varying) gbt_bit_consistent(
      internal,
      bit,
      smallint,
      oid,
      internal
    ),
    function 2 (bit varying, bit varying) gbt_bit_union(internal, internal),
    function 5 (bit varying, bit varying) gbt_bit_penalty(
      internal,
      internal,
      internal
    ),
    function 6 (bit varying, bit varying) gbt_bit_picksplit(internal, internal),
    function 7 (bit varying, bit varying) gbt_bit_same(
      gbtreekey_var,
      gbtreekey_var,
      internal
    ),
    storage gbtreekey_var;

alter operator family gist_bit_ops
  using gist
  add
    operator 1 pg_catalog.< (bit, bit),
    operator 2 pg_catalog.<= (bit, bit),
    operator 3 pg_catalog.= (bit, bit),
    operator 4 pg_catalog.>= (bit, bit),
    operator 5 pg_catalog.> (bit, bit),
    operator 6 pg_catalog.<> (bit, bit),
    function 3 (bit, bit) gbt_bit_compress(internal),
    function 4 (bit, bit) gbt_var_decompress(internal),
    function 9 (bit, bit) gbt_var_fetch(internal),
    function 11 (bit, bit) gbt_bit_sortsupport(internal),
    function 12 ("any", "any") gist_translate_cmptype_btree(integer);

alter operator family gist_bool_ops
  using gist
  add
    operator 1 pg_catalog.< (boolean, boolean),
    operator 2 pg_catalog.<= (boolean, boolean),
    operator 3 pg_catalog.= (boolean, boolean),
    operator 4 pg_catalog.>= (boolean, boolean),
    operator 5 pg_catalog.> (boolean, boolean),
    operator 6 pg_catalog.<> (boolean, boolean),
    function 3 (boolean, boolean) gbt_bool_compress(internal),
    function 4 (boolean, boolean) gbt_decompress(internal),
    function 9 (boolean, boolean) gbt_bool_fetch(internal),
    function 11 (boolean, boolean) gbt_bool_sortsupport(internal),
    function 12 ("any", "any") gist_translate_cmptype_btree(integer);

alter operator family gist_bpchar_ops
  using gist
  add
    operator 1 pg_catalog.< (character, character),
    operator 2 pg_catalog.<= (character, character),
    operator 3 pg_catalog.= (character, character),
    operator 4 pg_catalog.>= (character, character),
    operator 5 pg_catalog.> (character, character),
    operator 6 pg_catalog.<> (character, character),
    function 3 (character, character) gbt_bpchar_compress(internal),
    function 4 (character, character) gbt_var_decompress(internal),
    function 9 (character, character) gbt_var_fetch(internal),
    function 11 (character, character) gbt_bpchar_sortsupport(internal),
    function 12 ("any", "any") gist_translate_cmptype_btree(integer);

alter operator family gist_bytea_ops
  using gist
  add
    operator 1 pg_catalog.< (bytea, bytea),
    operator 2 pg_catalog.<= (bytea, bytea),
    operator 3 pg_catalog.= (bytea, bytea),
    operator 4 pg_catalog.>= (bytea, bytea),
    operator 5 pg_catalog.> (bytea, bytea),
    operator 6 pg_catalog.<> (bytea, bytea),
    function 3 (bytea, bytea) gbt_bytea_compress(internal),
    function 4 (bytea, bytea) gbt_var_decompress(internal),
    function 9 (bytea, bytea) gbt_var_fetch(internal),
    function 11 (bytea, bytea) gbt_bytea_sortsupport(internal),
    function 12 ("any", "any") gist_translate_cmptype_btree(integer);

alter operator family gist_cash_ops
  using gist
  add
    operator 1 pg_catalog.< (money, money),
    operator 2 pg_catalog.<= (money, money),
    operator 3 pg_catalog.= (money, money),
    operator 4 pg_catalog.>= (money, money),
    operator 5 pg_catalog.> (money, money),
    operator 6 pg_catalog.<> (money, money),
    operator 15 <-> (money, money) for order by pg_catalog.money_ops,
    function 3 (money, money) gbt_cash_compress(internal),
    function 4 (money, money) gbt_decompress(internal),
    function 8 (money, money) gbt_cash_distance(
      internal,
      money,
      smallint,
      oid,
      internal
    ),
    function 9 (money, money) gbt_cash_fetch(internal),
    function 11 (money, money) gbt_cash_sortsupport(internal),
    function 12 ("any", "any") gist_translate_cmptype_btree(integer);

alter operator family gist_cidr_ops
  using gist
  add
    operator 1 pg_catalog.< (inet, inet),
    operator 2 pg_catalog.<= (inet, inet),
    operator 3 pg_catalog.= (inet, inet),
    operator 4 pg_catalog.>= (inet, inet),
    operator 5 pg_catalog.> (inet, inet),
    operator 6 pg_catalog.<> (inet, inet),
    function 3 (cidr, cidr) gbt_inet_compress(internal),
    function 4 (cidr, cidr) gbt_decompress(internal),
    function 11 (cidr, cidr) gbt_inet_sortsupport(internal),
    function 12 ("any", "any") gist_translate_cmptype_btree(integer);

alter operator family gist_date_ops
  using gist
  add
    operator 1 pg_catalog.< (date, date),
    operator 2 pg_catalog.<= (date, date),
    operator 3 pg_catalog.= (date, date),
    operator 4 pg_catalog.>= (date, date),
    operator 5 pg_catalog.> (date, date),
    operator 6 pg_catalog.<> (date, date),
    operator 15 <-> (date, date) for order by pg_catalog.integer_ops,
    function 3 (date, date) gbt_date_compress(internal),
    function 4 (date, date) gbt_decompress(internal),
    function 8 (date, date) gbt_date_distance(
      internal,
      date,
      smallint,
      oid,
      internal
    ),
    function 9 (date, date) gbt_date_fetch(internal),
    function 11 (date, date) gbt_date_sortsupport(internal),
    function 12 ("any", "any") gist_translate_cmptype_btree(integer);

alter operator family gist_enum_ops
  using gist
  add
    operator 1 pg_catalog.< (anyenum, anyenum),
    operator 2 pg_catalog.<= (anyenum, anyenum),
    operator 3 pg_catalog.= (anyenum, anyenum),
    operator 4 pg_catalog.>= (anyenum, anyenum),
    operator 5 pg_catalog.> (anyenum, anyenum),
    operator 6 pg_catalog.<> (anyenum, anyenum),
    function 3 (anyenum, anyenum) gbt_enum_compress(internal),
    function 4 (anyenum, anyenum) gbt_decompress(internal),
    function 9 (anyenum, anyenum) gbt_enum_fetch(internal),
    function 11 (anyenum, anyenum) gbt_enum_sortsupport(internal),
    function 12 ("any", "any") gist_translate_cmptype_btree(integer);

alter operator family gist_float4_ops
  using gist
  add
    operator 1 pg_catalog.< (real, real),
    operator 2 pg_catalog.<= (real, real),
    operator 3 pg_catalog.= (real, real),
    operator 4 pg_catalog.>= (real, real),
    operator 5 pg_catalog.> (real, real),
    operator 6 pg_catalog.<> (real, real),
    operator 15 <-> (real, real) for order by pg_catalog.float_ops,
    function 3 (real, real) gbt_float4_compress(internal),
    function 4 (real, real) gbt_decompress(internal),
    function 8 (real, real) gbt_float4_distance(
      internal,
      real,
      smallint,
      oid,
      internal
    ),
    function 9 (real, real) gbt_float4_fetch(internal),
    function 11 (real, real) gbt_float4_sortsupport(internal),
    function 12 ("any", "any") gist_translate_cmptype_btree(integer);

alter operator family gist_float8_ops
  using gist
  add
    operator 1 pg_catalog.< (double precision, double precision),
    operator 2 pg_catalog.<= (double precision, double precision),
    operator 3 pg_catalog.= (double precision, double precision),
    operator 4 pg_catalog.>= (double precision, double precision),
    operator 5 pg_catalog.> (double precision, double precision),
    operator 6 pg_catalog.<> (double precision, double precision),
    operator 15 <-> (double precision, double precision)
    for order by pg_catalog.float_ops,
    function 3 (double precision, double precision) gbt_float8_compress(
      internal
    ),
    function 4 (double precision, double precision) gbt_decompress(internal),
    function 8 (double precision, double precision) gbt_float8_distance(
      internal,
      double precision,
      smallint,
      oid,
      internal
    ),
    function 9 (double precision, double precision) gbt_float8_fetch(internal),
    function 11 (double precision, double precision) gbt_float8_sortsupport(
      internal
    ),
    function 12 ("any", "any") gist_translate_cmptype_btree(integer);

alter operator family gist_inet_ops
  using gist
  add
    operator 1 pg_catalog.< (inet, inet),
    operator 2 pg_catalog.<= (inet, inet),
    operator 3 pg_catalog.= (inet, inet),
    operator 4 pg_catalog.>= (inet, inet),
    operator 5 pg_catalog.> (inet, inet),
    operator 6 pg_catalog.<> (inet, inet),
    function 3 (inet, inet) gbt_inet_compress(internal),
    function 4 (inet, inet) gbt_decompress(internal),
    function 11 (inet, inet) gbt_inet_sortsupport(internal),
    function 12 ("any", "any") gist_translate_cmptype_btree(integer);

alter operator family gist_int2_ops
  using gist
  add
    operator 1 pg_catalog.< (smallint, smallint),
    operator 2 pg_catalog.<= (smallint, smallint),
    operator 3 pg_catalog.= (smallint, smallint),
    operator 4 pg_catalog.>= (smallint, smallint),
    operator 5 pg_catalog.> (smallint, smallint),
    operator 6 pg_catalog.<> (smallint, smallint),
    operator 15 <-> (smallint, smallint) for order by pg_catalog.integer_ops,
    function 3 (smallint, smallint) gbt_int2_compress(internal),
    function 4 (smallint, smallint) gbt_decompress(internal),
    function 8 (smallint, smallint) gbt_int2_distance(
      internal,
      smallint,
      smallint,
      oid,
      internal
    ),
    function 9 (smallint, smallint) gbt_int2_fetch(internal),
    function 11 (smallint, smallint) gbt_int2_sortsupport(internal),
    function 12 ("any", "any") gist_translate_cmptype_btree(integer);

alter operator family gist_int4_ops
  using gist
  add
    operator 1 pg_catalog.< (integer, integer),
    operator 2 pg_catalog.<= (integer, integer),
    operator 3 pg_catalog.= (integer, integer),
    operator 4 pg_catalog.>= (integer, integer),
    operator 5 pg_catalog.> (integer, integer),
    operator 6 pg_catalog.<> (integer, integer),
    operator 15 <-> (integer, integer) for order by pg_catalog.integer_ops,
    function 3 (integer, integer) gbt_int4_compress(internal),
    function 4 (integer, integer) gbt_decompress(internal),
    function 8 (integer, integer) gbt_int4_distance(
      internal,
      integer,
      smallint,
      oid,
      internal
    ),
    function 9 (integer, integer) gbt_int4_fetch(internal),
    function 11 (integer, integer) gbt_int4_sortsupport(internal),
    function 12 ("any", "any") gist_translate_cmptype_btree(integer);

alter operator family gist_int8_ops
  using gist
  add
    operator 1 pg_catalog.< (bigint, bigint),
    operator 2 pg_catalog.<= (bigint, bigint),
    operator 3 pg_catalog.= (bigint, bigint),
    operator 4 pg_catalog.>= (bigint, bigint),
    operator 5 pg_catalog.> (bigint, bigint),
    operator 6 pg_catalog.<> (bigint, bigint),
    operator 15 <-> (bigint, bigint) for order by pg_catalog.integer_ops,
    function 3 (bigint, bigint) gbt_int8_compress(internal),
    function 4 (bigint, bigint) gbt_decompress(internal),
    function 8 (bigint, bigint) gbt_int8_distance(
      internal,
      bigint,
      smallint,
      oid,
      internal
    ),
    function 9 (bigint, bigint) gbt_int8_fetch(internal),
    function 11 (bigint, bigint) gbt_int8_sortsupport(internal),
    function 12 ("any", "any") gist_translate_cmptype_btree(integer);

alter operator family gist_interval_ops
  using gist
  add
    operator 1 pg_catalog.< (interval, interval),
    operator 2 pg_catalog.<= (interval, interval),
    operator 3 pg_catalog.= (interval, interval),
    operator 4 pg_catalog.>= (interval, interval),
    operator 5 pg_catalog.> (interval, interval),
    operator 6 pg_catalog.<> (interval, interval),
    operator 15 <-> (interval, interval) for order by pg_catalog.interval_ops,
    function 3 (interval, interval) gbt_intv_compress(internal),
    function 4 (interval, interval) gbt_intv_decompress(internal),
    function 8 (interval, interval) gbt_intv_distance(
      internal,
      interval,
      smallint,
      oid,
      internal
    ),
    function 9 (interval, interval) gbt_intv_fetch(internal),
    function 11 (interval, interval) gbt_intv_sortsupport(internal),
    function 12 ("any", "any") gist_translate_cmptype_btree(integer);

alter operator family gist_macaddr8_ops
  using gist
  add
    operator 1 pg_catalog.< (macaddr8, macaddr8),
    operator 2 pg_catalog.<= (macaddr8, macaddr8),
    operator 3 pg_catalog.= (macaddr8, macaddr8),
    operator 4 pg_catalog.>= (macaddr8, macaddr8),
    operator 5 pg_catalog.> (macaddr8, macaddr8),
    operator 6 pg_catalog.<> (macaddr8, macaddr8),
    function 3 (macaddr8, macaddr8) gbt_macad8_compress(internal),
    function 4 (macaddr8, macaddr8) gbt_decompress(internal),
    function 9 (macaddr8, macaddr8) gbt_macad8_fetch(internal),
    function 11 (macaddr8, macaddr8) gbt_macad8_sortsupport(internal),
    function 12 ("any", "any") gist_translate_cmptype_btree(integer);

alter operator family gist_macaddr_ops
  using gist
  add
    operator 1 pg_catalog.< (macaddr, macaddr),
    operator 2 pg_catalog.<= (macaddr, macaddr),
    operator 3 pg_catalog.= (macaddr, macaddr),
    operator 4 pg_catalog.>= (macaddr, macaddr),
    operator 5 pg_catalog.> (macaddr, macaddr),
    operator 6 pg_catalog.<> (macaddr, macaddr),
    function 3 (macaddr, macaddr) gbt_macad_compress(internal),
    function 4 (macaddr, macaddr) gbt_decompress(internal),
    function 9 (macaddr, macaddr) gbt_macad_fetch(internal),
    function 11 (macaddr, macaddr) gbt_macaddr_sortsupport(internal),
    function 12 ("any", "any") gist_translate_cmptype_btree(integer);

alter operator family gist_numeric_ops
  using gist
  add
    operator 1 pg_catalog.< (numeric, numeric),
    operator 2 pg_catalog.<= (numeric, numeric),
    operator 3 pg_catalog.= (numeric, numeric),
    operator 4 pg_catalog.>= (numeric, numeric),
    operator 5 pg_catalog.> (numeric, numeric),
    operator 6 pg_catalog.<> (numeric, numeric),
    function 3 (numeric, numeric) gbt_numeric_compress(internal),
    function 4 (numeric, numeric) gbt_var_decompress(internal),
    function 9 (numeric, numeric) gbt_var_fetch(internal),
    function 11 (numeric, numeric) gbt_numeric_sortsupport(internal),
    function 12 ("any", "any") gist_translate_cmptype_btree(integer);

alter operator family gist_oid_ops
  using gist
  add
    operator 1 pg_catalog.< (oid, oid),
    operator 2 pg_catalog.<= (oid, oid),
    operator 3 pg_catalog.= (oid, oid),
    operator 4 pg_catalog.>= (oid, oid),
    operator 5 pg_catalog.> (oid, oid),
    operator 6 pg_catalog.<> (oid, oid),
    operator 15 <-> (oid, oid) for order by pg_catalog.oid_ops,
    function 3 (oid, oid) gbt_oid_compress(internal),
    function 4 (oid, oid) gbt_decompress(internal),
    function 8 (oid, oid) gbt_oid_distance(
      internal,
      oid,
      smallint,
      oid,
      internal
    ),
    function 9 (oid, oid) gbt_oid_fetch(internal),
    function 11 (oid, oid) gbt_oid_sortsupport(internal),
    function 12 ("any", "any") gist_translate_cmptype_btree(integer);

alter operator family gist_text_ops
  using gist
  add
    operator 1 pg_catalog.< (text, text),
    operator 2 pg_catalog.<= (text, text),
    operator 3 pg_catalog.= (text, text),
    operator 4 pg_catalog.>= (text, text),
    operator 5 pg_catalog.> (text, text),
    operator 6 pg_catalog.<> (text, text),
    function 3 (text, text) gbt_text_compress(internal),
    function 4 (text, text) gbt_var_decompress(internal),
    function 9 (text, text) gbt_var_fetch(internal),
    function 11 (text, text) gbt_text_sortsupport(internal),
    function 12 ("any", "any") gist_translate_cmptype_btree(integer);

alter operator family gist_time_ops
  using gist
  add
    operator 1 pg_catalog.< (time without time zone, time without time zone),
    operator 2 pg_catalog.<= (time without time zone, time without time zone),
    operator 3 pg_catalog.= (time without time zone, time without time zone),
    operator 4 pg_catalog.>= (time without time zone, time without time zone),
    operator 5 pg_catalog.> (time without time zone, time without time zone),
    operator 6 pg_catalog.<> (time without time zone, time without time zone),
    operator 15 <-> (time without time zone, time without time zone)
    for order by pg_catalog.interval_ops,
    function 3 (
      time without time zone,
      time without time zone
    ) gbt_time_compress(
      internal
    ),
    function 4 (time without time zone, time without time zone) gbt_decompress(
      internal
    ),
    function 8 (
      time without time zone,
      time without time zone
    ) gbt_time_distance(
      internal,
      time without time zone,
      smallint,
      oid,
      internal
    ),
    function 9 (time without time zone, time without time zone) gbt_time_fetch(
      internal
    ),
    function 11 (
      time without time zone,
      time without time zone
    ) gbt_time_sortsupport(
      internal
    ),
    function 12 ("any", "any") gist_translate_cmptype_btree(integer);

alter operator family gist_timestamp_ops
  using gist
  add
    operator 1 pg_catalog.< (
      timestamp without time zone,
      timestamp without time zone
    ),
    operator 2 pg_catalog.<= (
      timestamp without time zone,
      timestamp without time zone
    ),
    operator 3 pg_catalog.= (
      timestamp without time zone,
      timestamp without time zone
    ),
    operator 4 pg_catalog.>= (
      timestamp without time zone,
      timestamp without time zone
    ),
    operator 5 pg_catalog.> (
      timestamp without time zone,
      timestamp without time zone
    ),
    operator 6 pg_catalog.<> (
      timestamp without time zone,
      timestamp without time zone
    ),
    operator 15 <-> (timestamp without time zone, timestamp without time zone)
    for order by pg_catalog.interval_ops,
    function 3 (
      timestamp without time zone,
      timestamp without time zone
    ) gbt_ts_compress(
      internal
    ),
    function 4 (
      timestamp without time zone,
      timestamp without time zone
    ) gbt_decompress(
      internal
    ),
    function 8 (
      timestamp without time zone,
      timestamp without time zone
    ) gbt_ts_distance(
      internal,
      timestamp without time zone,
      smallint,
      oid,
      internal
    ),
    function 9 (
      timestamp without time zone,
      timestamp without time zone
    ) gbt_ts_fetch(
      internal
    ),
    function 11 (
      timestamp without time zone,
      timestamp without time zone
    ) gbt_ts_sortsupport(
      internal
    ),
    function 12 ("any", "any") gist_translate_cmptype_btree(integer);

alter operator family gist_timestamptz_ops
  using gist
  add
    operator 1 pg_catalog.< (
      timestamp with time zone,
      timestamp with time zone
    ),
    operator 2 pg_catalog.<= (
      timestamp with time zone,
      timestamp with time zone
    ),
    operator 3 pg_catalog.= (
      timestamp with time zone,
      timestamp with time zone
    ),
    operator 4 pg_catalog.>= (
      timestamp with time zone,
      timestamp with time zone
    ),
    operator 5 pg_catalog.> (
      timestamp with time zone,
      timestamp with time zone
    ),
    operator 6 pg_catalog.<> (
      timestamp with time zone,
      timestamp with time zone
    ),
    operator 15 <-> (timestamp with time zone, timestamp with time zone)
    for order by pg_catalog.interval_ops,
    function 3 (
      timestamp with time zone,
      timestamp with time zone
    ) gbt_tstz_compress(
      internal
    ),
    function 4 (
      timestamp with time zone,
      timestamp with time zone
    ) gbt_decompress(
      internal
    ),
    function 8 (
      timestamp with time zone,
      timestamp with time zone
    ) gbt_tstz_distance(
      internal,
      timestamp with time zone,
      smallint,
      oid,
      internal
    ),
    function 9 (
      timestamp with time zone,
      timestamp with time zone
    ) gbt_ts_fetch(
      internal
    ),
    function 11 (
      timestamp with time zone,
      timestamp with time zone
    ) gbt_ts_sortsupport(
      internal
    ),
    function 12 ("any", "any") gist_translate_cmptype_btree(integer);

alter operator family gist_timetz_ops
  using gist
  add
    operator 1 pg_catalog.< (time with time zone, time with time zone),
    operator 2 pg_catalog.<= (time with time zone, time with time zone),
    operator 3 pg_catalog.= (time with time zone, time with time zone),
    operator 4 pg_catalog.>= (time with time zone, time with time zone),
    operator 5 pg_catalog.> (time with time zone, time with time zone),
    operator 6 pg_catalog.<> (time with time zone, time with time zone),
    function 3 (time with time zone, time with time zone) gbt_timetz_compress(
      internal
    ),
    function 4 (time with time zone, time with time zone) gbt_decompress(
      internal
    ),
    function 11 (time with time zone, time with time zone) gbt_time_sortsupport(
      internal
    ),
    function 12 ("any", "any") gist_translate_cmptype_btree(integer);

alter operator family gist_uuid_ops
  using gist
  add
    operator 1 pg_catalog.< (uuid, uuid),
    operator 2 pg_catalog.<= (uuid, uuid),
    operator 3 pg_catalog.= (uuid, uuid),
    operator 4 pg_catalog.>= (uuid, uuid),
    operator 5 pg_catalog.> (uuid, uuid),
    operator 6 pg_catalog.<> (uuid, uuid),
    function 3 (uuid, uuid) gbt_uuid_compress(internal),
    function 4 (uuid, uuid) gbt_decompress(internal),
    function 9 (uuid, uuid) gbt_uuid_fetch(internal),
    function 11 (uuid, uuid) gbt_uuid_sortsupport(internal),
    function 12 ("any", "any") gist_translate_cmptype_btree(integer);

alter operator family gist_vbit_ops
  using gist
  add
    operator 1 pg_catalog.< (bit varying, bit varying),
    operator 2 pg_catalog.<= (bit varying, bit varying),
    operator 3 pg_catalog.= (bit varying, bit varying),
    operator 4 pg_catalog.>= (bit varying, bit varying),
    operator 5 pg_catalog.> (bit varying, bit varying),
    operator 6 pg_catalog.<> (bit varying, bit varying),
    function 3 (bit varying, bit varying) gbt_bit_compress(internal),
    function 4 (bit varying, bit varying) gbt_var_decompress(internal),
    function 9 (bit varying, bit varying) gbt_var_fetch(internal),
    function 11 (bit varying, bit varying) gbt_varbit_sortsupport(internal),
    function 12 ("any", "any") gist_translate_cmptype_btree(integer);
