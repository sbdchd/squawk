-- squawk-ignore-file
-- pg version: 18.3
-- update via:
--   cargo xtask sync-builtins

-- size: 16, align: 4
create type public.gbtreekey16;

-- size: 2, align: 4
create type public.gbtreekey2;

-- size: 32, align: 4
create type public.gbtreekey32;

-- size: 4, align: 4
create type public.gbtreekey4;

-- size: 8, align: 4
create type public.gbtreekey8;

-- size: -1, align: 4
create type public.gbtreekey_var;

create function public.cash_dist(money, money) returns money
  language c;

create function public.date_dist(date, date) returns integer
  language c;

create function public.float4_dist(real, real) returns real
  language c;

create function public.float8_dist(double precision, double precision) returns double precision
  language c;

create function public.gbt_bit_compress(internal) returns internal
  language c;

create function public.gbt_bit_consistent(internal, bit, smallint, oid, internal) returns boolean
  language c;

create function public.gbt_bit_penalty(internal, internal, internal) returns internal
  language c;

create function public.gbt_bit_picksplit(internal, internal) returns internal
  language c;

create function public.gbt_bit_same(gbtreekey_var, gbtreekey_var, internal) returns internal
  language c;

create function public.gbt_bit_sortsupport(internal) returns void
  language c;

create function public.gbt_bit_union(internal, internal) returns gbtreekey_var
  language c;

create function public.gbt_bool_compress(internal) returns internal
  language c;

create function public.gbt_bool_consistent(internal, boolean, smallint, oid, internal) returns boolean
  language c;

create function public.gbt_bool_fetch(internal) returns internal
  language c;

create function public.gbt_bool_penalty(internal, internal, internal) returns internal
  language c;

create function public.gbt_bool_picksplit(internal, internal) returns internal
  language c;

create function public.gbt_bool_same(gbtreekey2, gbtreekey2, internal) returns internal
  language c;

create function public.gbt_bool_sortsupport(internal) returns void
  language c;

create function public.gbt_bool_union(internal, internal) returns gbtreekey2
  language c;

create function public.gbt_bpchar_compress(internal) returns internal
  language c;

create function public.gbt_bpchar_consistent(internal, character, smallint, oid, internal) returns boolean
  language c;

create function public.gbt_bpchar_sortsupport(internal) returns void
  language c;

create function public.gbt_bytea_compress(internal) returns internal
  language c;

create function public.gbt_bytea_consistent(internal, bytea, smallint, oid, internal) returns boolean
  language c;

create function public.gbt_bytea_penalty(internal, internal, internal) returns internal
  language c;

create function public.gbt_bytea_picksplit(internal, internal) returns internal
  language c;

create function public.gbt_bytea_same(gbtreekey_var, gbtreekey_var, internal) returns internal
  language c;

create function public.gbt_bytea_sortsupport(internal) returns void
  language c;

create function public.gbt_bytea_union(internal, internal) returns gbtreekey_var
  language c;

create function public.gbt_cash_compress(internal) returns internal
  language c;

create function public.gbt_cash_consistent(internal, money, smallint, oid, internal) returns boolean
  language c;

create function public.gbt_cash_distance(internal, money, smallint, oid, internal) returns double precision
  language c;

create function public.gbt_cash_fetch(internal) returns internal
  language c;

create function public.gbt_cash_penalty(internal, internal, internal) returns internal
  language c;

create function public.gbt_cash_picksplit(internal, internal) returns internal
  language c;

create function public.gbt_cash_same(gbtreekey16, gbtreekey16, internal) returns internal
  language c;

create function public.gbt_cash_sortsupport(internal) returns void
  language c;

create function public.gbt_cash_union(internal, internal) returns gbtreekey16
  language c;

create function public.gbt_date_compress(internal) returns internal
  language c;

create function public.gbt_date_consistent(internal, date, smallint, oid, internal) returns boolean
  language c;

create function public.gbt_date_distance(internal, date, smallint, oid, internal) returns double precision
  language c;

create function public.gbt_date_fetch(internal) returns internal
  language c;

create function public.gbt_date_penalty(internal, internal, internal) returns internal
  language c;

create function public.gbt_date_picksplit(internal, internal) returns internal
  language c;

create function public.gbt_date_same(gbtreekey8, gbtreekey8, internal) returns internal
  language c;

create function public.gbt_date_sortsupport(internal) returns void
  language c;

create function public.gbt_date_union(internal, internal) returns gbtreekey8
  language c;

create function public.gbt_decompress(internal) returns internal
  language c;

create function public.gbt_enum_compress(internal) returns internal
  language c;

create function public.gbt_enum_consistent(internal, anyenum, smallint, oid, internal) returns boolean
  language c;

create function public.gbt_enum_fetch(internal) returns internal
  language c;

create function public.gbt_enum_penalty(internal, internal, internal) returns internal
  language c;

create function public.gbt_enum_picksplit(internal, internal) returns internal
  language c;

create function public.gbt_enum_same(gbtreekey8, gbtreekey8, internal) returns internal
  language c;

create function public.gbt_enum_sortsupport(internal) returns void
  language c;

create function public.gbt_enum_union(internal, internal) returns gbtreekey8
  language c;

create function public.gbt_float4_compress(internal) returns internal
  language c;

create function public.gbt_float4_consistent(internal, real, smallint, oid, internal) returns boolean
  language c;

create function public.gbt_float4_distance(internal, real, smallint, oid, internal) returns double precision
  language c;

create function public.gbt_float4_fetch(internal) returns internal
  language c;

create function public.gbt_float4_penalty(internal, internal, internal) returns internal
  language c;

create function public.gbt_float4_picksplit(internal, internal) returns internal
  language c;

create function public.gbt_float4_same(gbtreekey8, gbtreekey8, internal) returns internal
  language c;

create function public.gbt_float4_sortsupport(internal) returns void
  language c;

create function public.gbt_float4_union(internal, internal) returns gbtreekey8
  language c;

create function public.gbt_float8_compress(internal) returns internal
  language c;

create function public.gbt_float8_consistent(internal, double precision, smallint, oid, internal) returns boolean
  language c;

create function public.gbt_float8_distance(internal, double precision, smallint, oid, internal) returns double precision
  language c;

create function public.gbt_float8_fetch(internal) returns internal
  language c;

create function public.gbt_float8_penalty(internal, internal, internal) returns internal
  language c;

create function public.gbt_float8_picksplit(internal, internal) returns internal
  language c;

create function public.gbt_float8_same(gbtreekey16, gbtreekey16, internal) returns internal
  language c;

create function public.gbt_float8_sortsupport(internal) returns void
  language c;

create function public.gbt_float8_union(internal, internal) returns gbtreekey16
  language c;

create function public.gbt_inet_compress(internal) returns internal
  language c;

create function public.gbt_inet_consistent(internal, inet, smallint, oid, internal) returns boolean
  language c;

create function public.gbt_inet_penalty(internal, internal, internal) returns internal
  language c;

create function public.gbt_inet_picksplit(internal, internal) returns internal
  language c;

create function public.gbt_inet_same(gbtreekey16, gbtreekey16, internal) returns internal
  language c;

create function public.gbt_inet_sortsupport(internal) returns void
  language c;

create function public.gbt_inet_union(internal, internal) returns gbtreekey16
  language c;

create function public.gbt_int2_compress(internal) returns internal
  language c;

create function public.gbt_int2_consistent(internal, smallint, smallint, oid, internal) returns boolean
  language c;

create function public.gbt_int2_distance(internal, smallint, smallint, oid, internal) returns double precision
  language c;

create function public.gbt_int2_fetch(internal) returns internal
  language c;

create function public.gbt_int2_penalty(internal, internal, internal) returns internal
  language c;

create function public.gbt_int2_picksplit(internal, internal) returns internal
  language c;

create function public.gbt_int2_same(gbtreekey4, gbtreekey4, internal) returns internal
  language c;

create function public.gbt_int2_sortsupport(internal) returns void
  language c;

create function public.gbt_int2_union(internal, internal) returns gbtreekey4
  language c;

create function public.gbt_int4_compress(internal) returns internal
  language c;

create function public.gbt_int4_consistent(internal, integer, smallint, oid, internal) returns boolean
  language c;

create function public.gbt_int4_distance(internal, integer, smallint, oid, internal) returns double precision
  language c;

create function public.gbt_int4_fetch(internal) returns internal
  language c;

create function public.gbt_int4_penalty(internal, internal, internal) returns internal
  language c;

create function public.gbt_int4_picksplit(internal, internal) returns internal
  language c;

create function public.gbt_int4_same(gbtreekey8, gbtreekey8, internal) returns internal
  language c;

create function public.gbt_int4_sortsupport(internal) returns void
  language c;

create function public.gbt_int4_union(internal, internal) returns gbtreekey8
  language c;

create function public.gbt_int8_compress(internal) returns internal
  language c;

create function public.gbt_int8_consistent(internal, bigint, smallint, oid, internal) returns boolean
  language c;

create function public.gbt_int8_distance(internal, bigint, smallint, oid, internal) returns double precision
  language c;

create function public.gbt_int8_fetch(internal) returns internal
  language c;

create function public.gbt_int8_penalty(internal, internal, internal) returns internal
  language c;

create function public.gbt_int8_picksplit(internal, internal) returns internal
  language c;

create function public.gbt_int8_same(gbtreekey16, gbtreekey16, internal) returns internal
  language c;

create function public.gbt_int8_sortsupport(internal) returns void
  language c;

create function public.gbt_int8_union(internal, internal) returns gbtreekey16
  language c;

create function public.gbt_intv_compress(internal) returns internal
  language c;

create function public.gbt_intv_consistent(internal, interval, smallint, oid, internal) returns boolean
  language c;

create function public.gbt_intv_decompress(internal) returns internal
  language c;

create function public.gbt_intv_distance(internal, interval, smallint, oid, internal) returns double precision
  language c;

create function public.gbt_intv_fetch(internal) returns internal
  language c;

create function public.gbt_intv_penalty(internal, internal, internal) returns internal
  language c;

create function public.gbt_intv_picksplit(internal, internal) returns internal
  language c;

create function public.gbt_intv_same(gbtreekey32, gbtreekey32, internal) returns internal
  language c;

create function public.gbt_intv_sortsupport(internal) returns void
  language c;

create function public.gbt_intv_union(internal, internal) returns gbtreekey32
  language c;

create function public.gbt_macad8_compress(internal) returns internal
  language c;

create function public.gbt_macad8_consistent(internal, macaddr8, smallint, oid, internal) returns boolean
  language c;

create function public.gbt_macad8_fetch(internal) returns internal
  language c;

create function public.gbt_macad8_penalty(internal, internal, internal) returns internal
  language c;

create function public.gbt_macad8_picksplit(internal, internal) returns internal
  language c;

create function public.gbt_macad8_same(gbtreekey16, gbtreekey16, internal) returns internal
  language c;

create function public.gbt_macad8_sortsupport(internal) returns void
  language c;

create function public.gbt_macad8_union(internal, internal) returns gbtreekey16
  language c;

create function public.gbt_macad_compress(internal) returns internal
  language c;

create function public.gbt_macad_consistent(internal, macaddr, smallint, oid, internal) returns boolean
  language c;

create function public.gbt_macad_fetch(internal) returns internal
  language c;

create function public.gbt_macad_penalty(internal, internal, internal) returns internal
  language c;

create function public.gbt_macad_picksplit(internal, internal) returns internal
  language c;

create function public.gbt_macad_same(gbtreekey16, gbtreekey16, internal) returns internal
  language c;

create function public.gbt_macad_union(internal, internal) returns gbtreekey16
  language c;

create function public.gbt_macaddr_sortsupport(internal) returns void
  language c;

create function public.gbt_numeric_compress(internal) returns internal
  language c;

create function public.gbt_numeric_consistent(internal, numeric, smallint, oid, internal) returns boolean
  language c;

create function public.gbt_numeric_penalty(internal, internal, internal) returns internal
  language c;

create function public.gbt_numeric_picksplit(internal, internal) returns internal
  language c;

create function public.gbt_numeric_same(gbtreekey_var, gbtreekey_var, internal) returns internal
  language c;

create function public.gbt_numeric_sortsupport(internal) returns void
  language c;

create function public.gbt_numeric_union(internal, internal) returns gbtreekey_var
  language c;

create function public.gbt_oid_compress(internal) returns internal
  language c;

create function public.gbt_oid_consistent(internal, oid, smallint, oid, internal) returns boolean
  language c;

create function public.gbt_oid_distance(internal, oid, smallint, oid, internal) returns double precision
  language c;

create function public.gbt_oid_fetch(internal) returns internal
  language c;

create function public.gbt_oid_penalty(internal, internal, internal) returns internal
  language c;

create function public.gbt_oid_picksplit(internal, internal) returns internal
  language c;

create function public.gbt_oid_same(gbtreekey8, gbtreekey8, internal) returns internal
  language c;

create function public.gbt_oid_sortsupport(internal) returns void
  language c;

create function public.gbt_oid_union(internal, internal) returns gbtreekey8
  language c;

create function public.gbt_text_compress(internal) returns internal
  language c;

create function public.gbt_text_consistent(internal, text, smallint, oid, internal) returns boolean
  language c;

create function public.gbt_text_penalty(internal, internal, internal) returns internal
  language c;

create function public.gbt_text_picksplit(internal, internal) returns internal
  language c;

create function public.gbt_text_same(gbtreekey_var, gbtreekey_var, internal) returns internal
  language c;

create function public.gbt_text_sortsupport(internal) returns void
  language c;

create function public.gbt_text_union(internal, internal) returns gbtreekey_var
  language c;

create function public.gbt_time_compress(internal) returns internal
  language c;

create function public.gbt_time_consistent(internal, time without time zone, smallint, oid, internal) returns boolean
  language c;

create function public.gbt_time_distance(internal, time without time zone, smallint, oid, internal) returns double precision
  language c;

create function public.gbt_time_fetch(internal) returns internal
  language c;

create function public.gbt_time_penalty(internal, internal, internal) returns internal
  language c;

create function public.gbt_time_picksplit(internal, internal) returns internal
  language c;

create function public.gbt_time_same(gbtreekey16, gbtreekey16, internal) returns internal
  language c;

create function public.gbt_time_sortsupport(internal) returns void
  language c;

create function public.gbt_time_union(internal, internal) returns gbtreekey16
  language c;

create function public.gbt_timetz_compress(internal) returns internal
  language c;

create function public.gbt_timetz_consistent(internal, time with time zone, smallint, oid, internal) returns boolean
  language c;

create function public.gbt_ts_compress(internal) returns internal
  language c;

create function public.gbt_ts_consistent(internal, timestamp without time zone, smallint, oid, internal) returns boolean
  language c;

create function public.gbt_ts_distance(internal, timestamp without time zone, smallint, oid, internal) returns double precision
  language c;

create function public.gbt_ts_fetch(internal) returns internal
  language c;

create function public.gbt_ts_penalty(internal, internal, internal) returns internal
  language c;

create function public.gbt_ts_picksplit(internal, internal) returns internal
  language c;

create function public.gbt_ts_same(gbtreekey16, gbtreekey16, internal) returns internal
  language c;

create function public.gbt_ts_sortsupport(internal) returns void
  language c;

create function public.gbt_ts_union(internal, internal) returns gbtreekey16
  language c;

create function public.gbt_tstz_compress(internal) returns internal
  language c;

create function public.gbt_tstz_consistent(internal, timestamp with time zone, smallint, oid, internal) returns boolean
  language c;

create function public.gbt_tstz_distance(internal, timestamp with time zone, smallint, oid, internal) returns double precision
  language c;

create function public.gbt_uuid_compress(internal) returns internal
  language c;

create function public.gbt_uuid_consistent(internal, uuid, smallint, oid, internal) returns boolean
  language c;

create function public.gbt_uuid_fetch(internal) returns internal
  language c;

create function public.gbt_uuid_penalty(internal, internal, internal) returns internal
  language c;

create function public.gbt_uuid_picksplit(internal, internal) returns internal
  language c;

create function public.gbt_uuid_same(gbtreekey32, gbtreekey32, internal) returns internal
  language c;

create function public.gbt_uuid_sortsupport(internal) returns void
  language c;

create function public.gbt_uuid_union(internal, internal) returns gbtreekey32
  language c;

create function public.gbt_var_decompress(internal) returns internal
  language c;

create function public.gbt_var_fetch(internal) returns internal
  language c;

create function public.gbt_varbit_sortsupport(internal) returns void
  language c;

create function public.gbtreekey16_in(cstring) returns gbtreekey16
  language c;

create function public.gbtreekey16_out(gbtreekey16) returns cstring
  language c;

create function public.gbtreekey2_in(cstring) returns gbtreekey2
  language c;

create function public.gbtreekey2_out(gbtreekey2) returns cstring
  language c;

create function public.gbtreekey32_in(cstring) returns gbtreekey32
  language c;

create function public.gbtreekey32_out(gbtreekey32) returns cstring
  language c;

create function public.gbtreekey4_in(cstring) returns gbtreekey4
  language c;

create function public.gbtreekey4_out(gbtreekey4) returns cstring
  language c;

create function public.gbtreekey8_in(cstring) returns gbtreekey8
  language c;

create function public.gbtreekey8_out(gbtreekey8) returns cstring
  language c;

create function public.gbtreekey_var_in(cstring) returns gbtreekey_var
  language c;

create function public.gbtreekey_var_out(gbtreekey_var) returns cstring
  language c;

create function public.gist_translate_cmptype_btree(integer) returns smallint
  language c;

create function public.int2_dist(smallint, smallint) returns smallint
  language c;

create function public.int4_dist(integer, integer) returns integer
  language c;

create function public.int8_dist(bigint, bigint) returns bigint
  language c;

create function public.interval_dist(interval, interval) returns interval
  language c;

create function public.oid_dist(oid, oid) returns oid
  language c;

create function public.time_dist(time without time zone, time without time zone) returns interval
  language c;

create function public.ts_dist(timestamp without time zone, timestamp without time zone) returns interval
  language c;

create function public.tstz_dist(timestamp with time zone, timestamp with time zone) returns interval
  language c;

create operator public.<-> (
  leftarg = bigint,
  rightarg = bigint,
  function = public.int8_dist
);

create operator public.<-> (
  leftarg = date,
  rightarg = date,
  function = public.date_dist
);

create operator public.<-> (
  leftarg = double precision,
  rightarg = double precision,
  function = public.float8_dist
);

create operator public.<-> (
  leftarg = integer,
  rightarg = integer,
  function = public.int4_dist
);

create operator public.<-> (
  leftarg = interval,
  rightarg = interval,
  function = public.interval_dist
);

create operator public.<-> (
  leftarg = money,
  rightarg = money,
  function = public.cash_dist
);

create operator public.<-> (
  leftarg = oid,
  rightarg = oid,
  function = public.oid_dist
);

create operator public.<-> (
  leftarg = real,
  rightarg = real,
  function = public.float4_dist
);

create operator public.<-> (
  leftarg = smallint,
  rightarg = smallint,
  function = public.int2_dist
);

create operator public.<-> (
  leftarg = time without time zone,
  rightarg = time without time zone,
  function = public.time_dist
);

create operator public.<-> (
  leftarg = timestamp with time zone,
  rightarg = timestamp with time zone,
  function = public.tstz_dist
);

create operator public.<-> (
  leftarg = timestamp without time zone,
  rightarg = timestamp without time zone,
  function = public.ts_dist
);

