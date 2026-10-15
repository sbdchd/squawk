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
  language c immutable strict parallel safe;

create function date_dist(date, date) returns integer
  language c immutable strict parallel safe;

create function float4_dist(real, real) returns real
  language c immutable strict parallel safe;

create function float8_dist(double precision, double precision) returns double precision
  language c immutable strict parallel safe;

create function gbt_bit_compress(internal) returns internal
  language c immutable strict parallel safe;

create function gbt_bit_consistent(internal, bit, smallint, oid, internal) returns boolean
  language c immutable strict parallel safe;

create function gbt_bit_penalty(internal, internal, internal) returns internal
  language c immutable strict parallel safe;

create function gbt_bit_picksplit(internal, internal) returns internal
  language c immutable strict parallel safe;

create function gbt_bit_same(gbtreekey_var, gbtreekey_var, internal) returns internal
  language c immutable strict parallel safe;

create function gbt_bit_sortsupport(internal) returns void
  language c immutable strict parallel safe;

create function gbt_bit_union(internal, internal) returns gbtreekey_var
  language c immutable strict parallel safe;

create function gbt_bool_compress(internal) returns internal
  language c immutable strict;

create function gbt_bool_consistent(internal, boolean, smallint, oid, internal) returns boolean
  language c immutable strict;

create function gbt_bool_fetch(internal) returns internal
  language c immutable strict;

create function gbt_bool_penalty(internal, internal, internal) returns internal
  language c immutable strict;

create function gbt_bool_picksplit(internal, internal) returns internal
  language c immutable strict;

create function gbt_bool_same(gbtreekey2, gbtreekey2, internal) returns internal
  language c immutable strict;

create function gbt_bool_sortsupport(internal) returns void
  language c immutable strict parallel safe;

create function gbt_bool_union(internal, internal) returns gbtreekey2
  language c immutable strict;

create function gbt_bpchar_compress(internal) returns internal
  language c immutable strict parallel safe;

create function gbt_bpchar_consistent(internal, character, smallint, oid, internal) returns boolean
  language c immutable strict parallel safe;

create function gbt_bpchar_sortsupport(internal) returns void
  language c immutable strict parallel safe;

create function gbt_bytea_compress(internal) returns internal
  language c immutable strict parallel safe;

create function gbt_bytea_consistent(internal, bytea, smallint, oid, internal) returns boolean
  language c immutable strict parallel safe;

create function gbt_bytea_penalty(internal, internal, internal) returns internal
  language c immutable strict parallel safe;

create function gbt_bytea_picksplit(internal, internal) returns internal
  language c immutable strict parallel safe;

create function gbt_bytea_same(gbtreekey_var, gbtreekey_var, internal) returns internal
  language c immutable strict parallel safe;

create function gbt_bytea_sortsupport(internal) returns void
  language c immutable strict parallel safe;

create function gbt_bytea_union(internal, internal) returns gbtreekey_var
  language c immutable strict parallel safe;

create function gbt_cash_compress(internal) returns internal
  language c immutable strict parallel safe;

create function gbt_cash_consistent(internal, money, smallint, oid, internal) returns boolean
  language c immutable strict parallel safe;

create function gbt_cash_distance(internal, money, smallint, oid, internal) returns double precision
  language c immutable strict parallel safe;

create function gbt_cash_fetch(internal) returns internal
  language c immutable strict parallel safe;

create function gbt_cash_penalty(internal, internal, internal) returns internal
  language c immutable strict parallel safe;

create function gbt_cash_picksplit(internal, internal) returns internal
  language c immutable strict parallel safe;

create function gbt_cash_same(gbtreekey16, gbtreekey16, internal) returns internal
  language c immutable strict parallel safe;

create function gbt_cash_sortsupport(internal) returns void
  language c immutable strict parallel safe;

create function gbt_cash_union(internal, internal) returns gbtreekey16
  language c immutable strict parallel safe;

create function gbt_date_compress(internal) returns internal
  language c immutable strict parallel safe;

create function gbt_date_consistent(internal, date, smallint, oid, internal) returns boolean
  language c immutable strict parallel safe;

create function gbt_date_distance(internal, date, smallint, oid, internal) returns double precision
  language c immutable strict parallel safe;

create function gbt_date_fetch(internal) returns internal
  language c immutable strict parallel safe;

create function gbt_date_penalty(internal, internal, internal) returns internal
  language c immutable strict parallel safe;

create function gbt_date_picksplit(internal, internal) returns internal
  language c immutable strict parallel safe;

create function gbt_date_same(gbtreekey8, gbtreekey8, internal) returns internal
  language c immutable strict parallel safe;

create function gbt_date_sortsupport(internal) returns void
  language c immutable strict parallel safe;

create function gbt_date_union(internal, internal) returns gbtreekey8
  language c immutable strict parallel safe;

create function gbt_decompress(internal) returns internal
  language c immutable strict parallel safe;

create function gbt_enum_compress(internal) returns internal
  language c immutable strict parallel safe;

create function gbt_enum_consistent(internal, anyenum, smallint, oid, internal) returns boolean
  language c immutable strict parallel safe;

create function gbt_enum_fetch(internal) returns internal
  language c immutable strict parallel safe;

create function gbt_enum_penalty(internal, internal, internal) returns internal
  language c immutable strict parallel safe;

create function gbt_enum_picksplit(internal, internal) returns internal
  language c immutable strict parallel safe;

create function gbt_enum_same(gbtreekey8, gbtreekey8, internal) returns internal
  language c immutable strict parallel safe;

create function gbt_enum_sortsupport(internal) returns void
  language c immutable strict parallel safe;

create function gbt_enum_union(internal, internal) returns gbtreekey8
  language c immutable strict parallel safe;

create function gbt_float4_compress(internal) returns internal
  language c immutable strict parallel safe;

create function gbt_float4_consistent(internal, real, smallint, oid, internal) returns boolean
  language c immutable strict parallel safe;

create function gbt_float4_distance(internal, real, smallint, oid, internal) returns double precision
  language c immutable strict parallel safe;

create function gbt_float4_fetch(internal) returns internal
  language c immutable strict parallel safe;

create function gbt_float4_penalty(internal, internal, internal) returns internal
  language c immutable strict parallel safe;

create function gbt_float4_picksplit(internal, internal) returns internal
  language c immutable strict parallel safe;

create function gbt_float4_same(gbtreekey8, gbtreekey8, internal) returns internal
  language c immutable strict parallel safe;

create function gbt_float4_sortsupport(internal) returns void
  language c immutable strict parallel safe;

create function gbt_float4_union(internal, internal) returns gbtreekey8
  language c immutable strict parallel safe;

create function gbt_float8_compress(internal) returns internal
  language c immutable strict parallel safe;

create function gbt_float8_consistent(internal, double precision, smallint, oid, internal) returns boolean
  language c immutable strict parallel safe;

create function gbt_float8_distance(internal, double precision, smallint, oid, internal) returns double precision
  language c immutable strict parallel safe;

create function gbt_float8_fetch(internal) returns internal
  language c immutable strict parallel safe;

create function gbt_float8_penalty(internal, internal, internal) returns internal
  language c immutable strict parallel safe;

create function gbt_float8_picksplit(internal, internal) returns internal
  language c immutable strict parallel safe;

create function gbt_float8_same(gbtreekey16, gbtreekey16, internal) returns internal
  language c immutable strict parallel safe;

create function gbt_float8_sortsupport(internal) returns void
  language c immutable strict parallel safe;

create function gbt_float8_union(internal, internal) returns gbtreekey16
  language c immutable strict parallel safe;

create function gbt_inet_compress(internal) returns internal
  language c immutable strict parallel safe;

create function gbt_inet_consistent(internal, inet, smallint, oid, internal) returns boolean
  language c immutable strict parallel safe;

create function gbt_inet_penalty(internal, internal, internal) returns internal
  language c immutable strict parallel safe;

create function gbt_inet_picksplit(internal, internal) returns internal
  language c immutable strict parallel safe;

create function gbt_inet_same(gbtreekey16, gbtreekey16, internal) returns internal
  language c immutable strict parallel safe;

create function gbt_inet_sortsupport(internal) returns void
  language c immutable strict parallel safe;

create function gbt_inet_union(internal, internal) returns gbtreekey16
  language c immutable strict parallel safe;

create function gbt_int2_compress(internal) returns internal
  language c immutable strict parallel safe;

create function gbt_int2_consistent(internal, smallint, smallint, oid, internal) returns boolean
  language c immutable strict parallel safe;

create function gbt_int2_distance(internal, smallint, smallint, oid, internal) returns double precision
  language c immutable strict parallel safe;

create function gbt_int2_fetch(internal) returns internal
  language c immutable strict parallel safe;

create function gbt_int2_penalty(internal, internal, internal) returns internal
  language c immutable strict parallel safe;

create function gbt_int2_picksplit(internal, internal) returns internal
  language c immutable strict parallel safe;

create function gbt_int2_same(gbtreekey4, gbtreekey4, internal) returns internal
  language c immutable strict parallel safe;

create function gbt_int2_sortsupport(internal) returns void
  language c immutable strict parallel safe;

create function gbt_int2_union(internal, internal) returns gbtreekey4
  language c immutable strict parallel safe;

create function gbt_int4_compress(internal) returns internal
  language c immutable strict parallel safe;

create function gbt_int4_consistent(internal, integer, smallint, oid, internal) returns boolean
  language c immutable strict parallel safe;

create function gbt_int4_distance(internal, integer, smallint, oid, internal) returns double precision
  language c immutable strict parallel safe;

create function gbt_int4_fetch(internal) returns internal
  language c immutable strict parallel safe;

create function gbt_int4_penalty(internal, internal, internal) returns internal
  language c immutable strict parallel safe;

create function gbt_int4_picksplit(internal, internal) returns internal
  language c immutable strict parallel safe;

create function gbt_int4_same(gbtreekey8, gbtreekey8, internal) returns internal
  language c immutable strict parallel safe;

create function gbt_int4_sortsupport(internal) returns void
  language c immutable strict parallel safe;

create function gbt_int4_union(internal, internal) returns gbtreekey8
  language c immutable strict parallel safe;

create function gbt_int8_compress(internal) returns internal
  language c immutable strict parallel safe;

create function gbt_int8_consistent(internal, bigint, smallint, oid, internal) returns boolean
  language c immutable strict parallel safe;

create function gbt_int8_distance(internal, bigint, smallint, oid, internal) returns double precision
  language c immutable strict parallel safe;

create function gbt_int8_fetch(internal) returns internal
  language c immutable strict parallel safe;

create function gbt_int8_penalty(internal, internal, internal) returns internal
  language c immutable strict parallel safe;

create function gbt_int8_picksplit(internal, internal) returns internal
  language c immutable strict parallel safe;

create function gbt_int8_same(gbtreekey16, gbtreekey16, internal) returns internal
  language c immutable strict parallel safe;

create function gbt_int8_sortsupport(internal) returns void
  language c immutable strict parallel safe;

create function gbt_int8_union(internal, internal) returns gbtreekey16
  language c immutable strict parallel safe;

create function gbt_intv_compress(internal) returns internal
  language c immutable strict parallel safe;

create function gbt_intv_consistent(internal, interval, smallint, oid, internal) returns boolean
  language c immutable strict parallel safe;

create function gbt_intv_decompress(internal) returns internal
  language c immutable strict parallel safe;

create function gbt_intv_distance(internal, interval, smallint, oid, internal) returns double precision
  language c immutable strict parallel safe;

create function gbt_intv_fetch(internal) returns internal
  language c immutable strict parallel safe;

create function gbt_intv_penalty(internal, internal, internal) returns internal
  language c immutable strict parallel safe;

create function gbt_intv_picksplit(internal, internal) returns internal
  language c immutable strict parallel safe;

create function gbt_intv_same(gbtreekey32, gbtreekey32, internal) returns internal
  language c immutable strict parallel safe;

create function gbt_intv_sortsupport(internal) returns void
  language c immutable strict parallel safe;

create function gbt_intv_union(internal, internal) returns gbtreekey32
  language c immutable strict parallel safe;

create function gbt_macad8_compress(internal) returns internal
  language c immutable strict parallel safe;

create function gbt_macad8_consistent(internal, macaddr8, smallint, oid, internal) returns boolean
  language c immutable strict parallel safe;

create function gbt_macad8_fetch(internal) returns internal
  language c immutable strict parallel safe;

create function gbt_macad8_penalty(internal, internal, internal) returns internal
  language c immutable strict parallel safe;

create function gbt_macad8_picksplit(internal, internal) returns internal
  language c immutable strict parallel safe;

create function gbt_macad8_same(gbtreekey16, gbtreekey16, internal) returns internal
  language c immutable strict parallel safe;

create function gbt_macad8_sortsupport(internal) returns void
  language c immutable strict parallel safe;

create function gbt_macad8_union(internal, internal) returns gbtreekey16
  language c immutable strict parallel safe;

create function gbt_macad_compress(internal) returns internal
  language c immutable strict parallel safe;

create function gbt_macad_consistent(internal, macaddr, smallint, oid, internal) returns boolean
  language c immutable strict parallel safe;

create function gbt_macad_fetch(internal) returns internal
  language c immutable strict parallel safe;

create function gbt_macad_penalty(internal, internal, internal) returns internal
  language c immutable strict parallel safe;

create function gbt_macad_picksplit(internal, internal) returns internal
  language c immutable strict parallel safe;

create function gbt_macad_same(gbtreekey16, gbtreekey16, internal) returns internal
  language c immutable strict parallel safe;

create function gbt_macad_union(internal, internal) returns gbtreekey16
  language c immutable strict parallel safe;

create function gbt_macaddr_sortsupport(internal) returns void
  language c immutable strict parallel safe;

create function gbt_numeric_compress(internal) returns internal
  language c immutable strict parallel safe;

create function gbt_numeric_consistent(internal, numeric, smallint, oid, internal) returns boolean
  language c immutable strict parallel safe;

create function gbt_numeric_penalty(internal, internal, internal) returns internal
  language c immutable strict parallel safe;

create function gbt_numeric_picksplit(internal, internal) returns internal
  language c immutable strict parallel safe;

create function gbt_numeric_same(gbtreekey_var, gbtreekey_var, internal) returns internal
  language c immutable strict parallel safe;

create function gbt_numeric_sortsupport(internal) returns void
  language c immutable strict parallel safe;

create function gbt_numeric_union(internal, internal) returns gbtreekey_var
  language c immutable strict parallel safe;

create function gbt_oid_compress(internal) returns internal
  language c immutable strict parallel safe;

create function gbt_oid_consistent(internal, oid, smallint, oid, internal) returns boolean
  language c immutable strict parallel safe;

create function gbt_oid_distance(internal, oid, smallint, oid, internal) returns double precision
  language c immutable strict parallel safe;

create function gbt_oid_fetch(internal) returns internal
  language c immutable strict parallel safe;

create function gbt_oid_penalty(internal, internal, internal) returns internal
  language c immutable strict parallel safe;

create function gbt_oid_picksplit(internal, internal) returns internal
  language c immutable strict parallel safe;

create function gbt_oid_same(gbtreekey8, gbtreekey8, internal) returns internal
  language c immutable strict parallel safe;

create function gbt_oid_sortsupport(internal) returns void
  language c immutable strict parallel safe;

create function gbt_oid_union(internal, internal) returns gbtreekey8
  language c immutable strict parallel safe;

create function gbt_text_compress(internal) returns internal
  language c immutable strict parallel safe;

create function gbt_text_consistent(internal, text, smallint, oid, internal) returns boolean
  language c immutable strict parallel safe;

create function gbt_text_penalty(internal, internal, internal) returns internal
  language c immutable strict parallel safe;

create function gbt_text_picksplit(internal, internal) returns internal
  language c immutable strict parallel safe;

create function gbt_text_same(gbtreekey_var, gbtreekey_var, internal) returns internal
  language c immutable strict parallel safe;

create function gbt_text_sortsupport(internal) returns void
  language c immutable strict parallel safe;

create function gbt_text_union(internal, internal) returns gbtreekey_var
  language c immutable strict parallel safe;

create function gbt_time_compress(internal) returns internal
  language c immutable strict parallel safe;

create function gbt_time_consistent(internal, time without time zone, smallint, oid, internal) returns boolean
  language c immutable strict parallel safe;

create function gbt_time_distance(internal, time without time zone, smallint, oid, internal) returns double precision
  language c immutable strict parallel safe;

create function gbt_time_fetch(internal) returns internal
  language c immutable strict parallel safe;

create function gbt_time_penalty(internal, internal, internal) returns internal
  language c immutable strict parallel safe;

create function gbt_time_picksplit(internal, internal) returns internal
  language c immutable strict parallel safe;

create function gbt_time_same(gbtreekey16, gbtreekey16, internal) returns internal
  language c immutable strict parallel safe;

create function gbt_time_sortsupport(internal) returns void
  language c immutable strict parallel safe;

create function gbt_time_union(internal, internal) returns gbtreekey16
  language c immutable strict parallel safe;

create function gbt_timetz_compress(internal) returns internal
  language c immutable strict parallel safe;

create function gbt_timetz_consistent(internal, time with time zone, smallint, oid, internal) returns boolean
  language c immutable strict parallel safe;

create function gbt_ts_compress(internal) returns internal
  language c immutable strict parallel safe;

create function gbt_ts_consistent(internal, timestamp without time zone, smallint, oid, internal) returns boolean
  language c immutable strict parallel safe;

create function gbt_ts_distance(internal, timestamp without time zone, smallint, oid, internal) returns double precision
  language c immutable strict parallel safe;

create function gbt_ts_fetch(internal) returns internal
  language c immutable strict parallel safe;

create function gbt_ts_penalty(internal, internal, internal) returns internal
  language c immutable strict parallel safe;

create function gbt_ts_picksplit(internal, internal) returns internal
  language c immutable strict parallel safe;

create function gbt_ts_same(gbtreekey16, gbtreekey16, internal) returns internal
  language c immutable strict parallel safe;

create function gbt_ts_sortsupport(internal) returns void
  language c immutable strict parallel safe;

create function gbt_ts_union(internal, internal) returns gbtreekey16
  language c immutable strict parallel safe;

create function gbt_tstz_compress(internal) returns internal
  language c immutable strict parallel safe;

create function gbt_tstz_consistent(internal, timestamp with time zone, smallint, oid, internal) returns boolean
  language c immutable strict parallel safe;

create function gbt_tstz_distance(internal, timestamp with time zone, smallint, oid, internal) returns double precision
  language c immutable strict parallel safe;

create function gbt_uuid_compress(internal) returns internal
  language c immutable strict parallel safe;

create function gbt_uuid_consistent(internal, uuid, smallint, oid, internal) returns boolean
  language c immutable strict parallel safe;

create function gbt_uuid_fetch(internal) returns internal
  language c immutable strict parallel safe;

create function gbt_uuid_penalty(internal, internal, internal) returns internal
  language c immutable strict parallel safe;

create function gbt_uuid_picksplit(internal, internal) returns internal
  language c immutable strict parallel safe;

create function gbt_uuid_same(gbtreekey32, gbtreekey32, internal) returns internal
  language c immutable strict parallel safe;

create function gbt_uuid_sortsupport(internal) returns void
  language c immutable strict parallel safe;

create function gbt_uuid_union(internal, internal) returns gbtreekey32
  language c immutable strict parallel safe;

create function gbt_var_decompress(internal) returns internal
  language c immutable strict parallel safe;

create function gbt_var_fetch(internal) returns internal
  language c immutable strict parallel safe;

create function gbt_varbit_sortsupport(internal) returns void
  language c immutable strict parallel safe;

create function gbtreekey16_in(cstring) returns gbtreekey16
  language c immutable strict parallel safe;

create function gbtreekey16_out(gbtreekey16) returns cstring
  language c immutable strict parallel safe;

create function gbtreekey2_in(cstring) returns gbtreekey2
  language c immutable strict parallel safe;

create function gbtreekey2_out(gbtreekey2) returns cstring
  language c immutable strict parallel safe;

create function gbtreekey32_in(cstring) returns gbtreekey32
  language c immutable strict parallel safe;

create function gbtreekey32_out(gbtreekey32) returns cstring
  language c immutable strict parallel safe;

create function gbtreekey4_in(cstring) returns gbtreekey4
  language c immutable strict parallel safe;

create function gbtreekey4_out(gbtreekey4) returns cstring
  language c immutable strict parallel safe;

create function gbtreekey8_in(cstring) returns gbtreekey8
  language c immutable strict parallel safe;

create function gbtreekey8_out(gbtreekey8) returns cstring
  language c immutable strict parallel safe;

create function gbtreekey_var_in(cstring) returns gbtreekey_var
  language c immutable strict parallel safe;

create function gbtreekey_var_out(gbtreekey_var) returns cstring
  language c immutable strict parallel safe;

create function gist_translate_cmptype_btree(integer) returns smallint
  language c immutable strict parallel safe;

create function int2_dist(smallint, smallint) returns smallint
  language c immutable strict parallel safe;

create function int4_dist(integer, integer) returns integer
  language c immutable strict parallel safe;

create function int8_dist(bigint, bigint) returns bigint
  language c immutable strict parallel safe;

create function interval_dist(interval, interval) returns interval
  language c immutable strict parallel safe;

create function oid_dist(oid, oid) returns oid
  language c immutable strict parallel safe;

create function time_dist(time without time zone, time without time zone) returns interval
  language c immutable strict parallel safe;

create function ts_dist(timestamp without time zone, timestamp without time zone) returns interval
  language c immutable strict parallel safe;

create function tstz_dist(timestamp with time zone, timestamp with time zone) returns interval
  language c immutable strict parallel safe;

create type gbtreekey16 (input = gbtreekey16_in, output = gbtreekey16_out);

create type gbtreekey2 (input = gbtreekey2_in, output = gbtreekey2_out);

create type gbtreekey32 (input = gbtreekey32_in, output = gbtreekey32_out);

create type gbtreekey4 (input = gbtreekey4_in, output = gbtreekey4_out);

create type gbtreekey8 (input = gbtreekey8_in, output = gbtreekey8_out);

create type gbtreekey_var (input = gbtreekey_var_in, output = gbtreekey_var_out);

create operator <-> (
  leftarg = bigint,
  rightarg = bigint,
  function = int8_dist
);

create operator <-> (
  leftarg = date,
  rightarg = date,
  function = date_dist
);

create operator <-> (
  leftarg = double precision,
  rightarg = double precision,
  function = float8_dist
);

create operator <-> (
  leftarg = integer,
  rightarg = integer,
  function = int4_dist
);

create operator <-> (
  leftarg = interval,
  rightarg = interval,
  function = interval_dist
);

create operator <-> (
  leftarg = money,
  rightarg = money,
  function = cash_dist
);

create operator <-> (
  leftarg = oid,
  rightarg = oid,
  function = oid_dist
);

create operator <-> (
  leftarg = real,
  rightarg = real,
  function = float4_dist
);

create operator <-> (
  leftarg = smallint,
  rightarg = smallint,
  function = int2_dist
);

create operator <-> (
  leftarg = time without time zone,
  rightarg = time without time zone,
  function = time_dist
);

create operator <-> (
  leftarg = timestamp with time zone,
  rightarg = timestamp with time zone,
  function = tstz_dist
);

create operator <-> (
  leftarg = timestamp without time zone,
  rightarg = timestamp without time zone,
  function = ts_dist
);

create operator family gist_bit_ops using gist;

create operator family gist_bool_ops using gist;

create operator family gist_bpchar_ops using gist;

create operator family gist_bytea_ops using gist;

create operator family gist_cash_ops using gist;

create operator family gist_cidr_ops using gist;

create operator family gist_date_ops using gist;

create operator family gist_enum_ops using gist;

create operator family gist_float4_ops using gist;

create operator family gist_float8_ops using gist;

create operator family gist_inet_ops using gist;

create operator family gist_int2_ops using gist;

create operator family gist_int4_ops using gist;

create operator family gist_int8_ops using gist;

create operator family gist_interval_ops using gist;

create operator family gist_macaddr8_ops using gist;

create operator family gist_macaddr_ops using gist;

create operator family gist_numeric_ops using gist;

create operator family gist_oid_ops using gist;

create operator family gist_text_ops using gist;

create operator family gist_time_ops using gist;

create operator family gist_timestamp_ops using gist;

create operator family gist_timestamptz_ops using gist;

create operator family gist_timetz_ops using gist;

create operator family gist_uuid_ops using gist;

create operator family gist_vbit_ops using gist;

create operator class gist_bit_ops default for type bit using gist family gist_bit_ops as storage gbtreekey_var;

create operator class gist_bool_ops default for type boolean using gist family gist_bool_ops as storage gbtreekey2;

create operator class gist_bpchar_ops default for type character using gist family gist_bpchar_ops as storage gbtreekey_var;

create operator class gist_bytea_ops default for type bytea using gist family gist_bytea_ops as storage gbtreekey_var;

create operator class gist_cash_ops default for type money using gist family gist_cash_ops as storage gbtreekey16;

create operator class gist_cidr_ops default for type cidr using gist family gist_cidr_ops as storage gbtreekey16;

create operator class gist_date_ops default for type date using gist family gist_date_ops as storage gbtreekey8;

create operator class gist_enum_ops default for type anyenum using gist family gist_enum_ops as storage gbtreekey8;

create operator class gist_float4_ops default for type real using gist family gist_float4_ops as storage gbtreekey8;

create operator class gist_float8_ops default for type double precision using gist family gist_float8_ops as storage gbtreekey16;

create operator class gist_inet_ops default for type inet using gist family gist_inet_ops as storage gbtreekey16;

create operator class gist_int2_ops default for type smallint using gist family gist_int2_ops as storage gbtreekey4;

create operator class gist_int4_ops default for type integer using gist family gist_int4_ops as storage gbtreekey8;

create operator class gist_int8_ops default for type bigint using gist family gist_int8_ops as storage gbtreekey16;

create operator class gist_interval_ops default for type interval using gist family gist_interval_ops as storage gbtreekey32;

create operator class gist_macaddr8_ops default for type macaddr8 using gist family gist_macaddr8_ops as storage gbtreekey16;

create operator class gist_macaddr_ops default for type macaddr using gist family gist_macaddr_ops as storage gbtreekey16;

create operator class gist_numeric_ops default for type numeric using gist family gist_numeric_ops as storage gbtreekey_var;

create operator class gist_oid_ops default for type oid using gist family gist_oid_ops as storage gbtreekey8;

create operator class gist_text_ops default for type text using gist family gist_text_ops as storage gbtreekey_var;

create operator class gist_time_ops default for type time without time zone using gist family gist_time_ops as storage gbtreekey16;

create operator class gist_timestamp_ops default for type timestamp without time zone using gist family gist_timestamp_ops as storage gbtreekey16;

create operator class gist_timestamptz_ops default for type timestamp with time zone using gist family gist_timestamptz_ops as storage gbtreekey16;

create operator class gist_timetz_ops default for type time with time zone using gist family gist_timetz_ops as storage gbtreekey16;

create operator class gist_uuid_ops default for type uuid using gist family gist_uuid_ops as storage gbtreekey32;

create operator class gist_vbit_ops default for type bit varying using gist family gist_vbit_ops as storage gbtreekey_var;

