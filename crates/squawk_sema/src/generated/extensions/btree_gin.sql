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
    function 2 (bit, bit) gin_extract_value_bit(bit, internal),
    function 3 (
      bit,
      bit
    )
    gin_extract_query_bit(
      bit,
      internal,
      smallint,
      internal,
      internal
    ),
    storage bit;

create operator class bool_ops
  default for type boolean
  using gin
  family bool_ops as
    function 2 (boolean, boolean) gin_extract_value_bool(boolean, internal),
    function 3 (
      boolean,
      boolean
    )
    gin_extract_query_bool(
      boolean,
      internal,
      smallint,
      internal,
      internal
    ),
    storage boolean;

create operator class bpchar_ops
  default for type character
  using gin
  family bpchar_ops as
    function 2 (
      character,
      character
    )
    gin_extract_value_bpchar(
      character,
      internal
    ),
    function 3 (
      character,
      character
    )
    gin_extract_query_bpchar(
      character,
      internal,
      smallint,
      internal,
      internal
    ),
    storage character;

create operator class bytea_ops
  default for type bytea
  using gin
  family bytea_ops as
    function 2 (bytea, bytea) gin_extract_value_bytea(bytea, internal),
    function 3 (
      bytea,
      bytea
    )
    gin_extract_query_bytea(
      bytea,
      internal,
      smallint,
      internal,
      internal
    ),
    storage bytea;

create operator class char_ops
  default for type "char"
  using gin
  family char_ops as
    function 2 ("char", "char") gin_extract_value_char("char", internal),
    function 3 (
      "char",
      "char"
    )
    gin_extract_query_char(
      "char",
      internal,
      smallint,
      internal,
      internal
    ),
    storage "char";

create operator class cidr_ops
  default for type cidr
  using gin
  family cidr_ops as
    function 2 (cidr, cidr) gin_extract_value_cidr(cidr, internal),
    function 3 (
      cidr,
      cidr
    )
    gin_extract_query_cidr(
      cidr,
      internal,
      smallint,
      internal,
      internal
    ),
    storage cidr;

create operator class date_ops
  default for type date
  using gin
  family date_ops as
    function 2 (date, date) gin_extract_value_date(date, internal),
    function 3 (
      date,
      date
    )
    gin_extract_query_date(
      date,
      internal,
      smallint,
      internal,
      internal
    ),
    storage date;

create operator class enum_ops
  default for type anyenum
  using gin
  family enum_ops as
    function 2 (anyenum, anyenum) gin_extract_value_anyenum(anyenum, internal),
    function 3 (
      anyenum,
      anyenum
    )
    gin_extract_query_anyenum(
      anyenum,
      internal,
      smallint,
      internal,
      internal
    ),
    storage anyenum;

create operator class float4_ops
  default for type real
  using gin
  family float4_ops as
    function 2 (real, real) gin_extract_value_float4(real, internal),
    function 3 (
      real,
      real
    )
    gin_extract_query_float4(
      real,
      internal,
      smallint,
      internal,
      internal
    ),
    storage real;

create operator class float8_ops
  default for type double precision
  using gin
  family float8_ops as
    function 2 (
      double precision,
      double precision
    )
    gin_extract_value_float8(
      double precision,
      internal
    ),
    function 3 (
      double precision,
      double precision
    )
    gin_extract_query_float8(
      double precision,
      internal,
      smallint,
      internal,
      internal
    ),
    storage double precision;

create operator class inet_ops
  default for type inet
  using gin
  family inet_ops as
    function 2 (inet, inet) gin_extract_value_inet(inet, internal),
    function 3 (
      inet,
      inet
    )
    gin_extract_query_inet(
      inet,
      internal,
      smallint,
      internal,
      internal
    ),
    storage inet;

create operator class int2_ops
  default for type smallint
  using gin
  family int2_ops as
    function 2 (smallint, smallint) gin_extract_value_int2(smallint, internal),
    function 3 (
      smallint,
      smallint
    )
    gin_extract_query_int2(
      smallint,
      internal,
      smallint,
      internal,
      internal
    ),
    storage smallint;

create operator class int4_ops
  default for type integer
  using gin
  family int4_ops as
    function 2 (integer, integer) gin_extract_value_int4(integer, internal),
    function 3 (
      integer,
      integer
    )
    gin_extract_query_int4(
      integer,
      internal,
      smallint,
      internal,
      internal
    ),
    storage integer;

create operator class int8_ops
  default for type bigint
  using gin
  family int8_ops as
    function 2 (bigint, bigint) gin_extract_value_int8(bigint, internal),
    function 3 (
      bigint,
      bigint
    )
    gin_extract_query_int8(
      bigint,
      internal,
      smallint,
      internal,
      internal
    ),
    storage bigint;

create operator class interval_ops
  default for type interval
  using gin
  family interval_ops as
    function 2 (
      interval,
      interval
    )
    gin_extract_value_interval(
      interval,
      internal
    ),
    function 3 (
      interval,
      interval
    )
    gin_extract_query_interval(
      interval,
      internal,
      smallint,
      internal,
      internal
    ),
    storage interval;

create operator class macaddr8_ops
  default for type macaddr8
  using gin
  family macaddr8_ops as
    function 2 (
      macaddr8,
      macaddr8
    )
    gin_extract_value_macaddr8(
      macaddr8,
      internal
    ),
    function 3 (
      macaddr8,
      macaddr8
    )
    gin_extract_query_macaddr8(
      macaddr8,
      internal,
      smallint,
      internal,
      internal
    ),
    storage macaddr8;

create operator class macaddr_ops
  default for type macaddr
  using gin
  family macaddr_ops as
    function 2 (macaddr, macaddr) gin_extract_value_macaddr(macaddr, internal),
    function 3 (
      macaddr,
      macaddr
    )
    gin_extract_query_macaddr(
      macaddr,
      internal,
      smallint,
      internal,
      internal
    ),
    storage macaddr;

create operator class money_ops
  default for type money
  using gin
  family money_ops as
    function 2 (money, money) gin_extract_value_money(money, internal),
    function 3 (
      money,
      money
    )
    gin_extract_query_money(
      money,
      internal,
      smallint,
      internal,
      internal
    ),
    storage money;

create operator class name_ops
  default for type name
  using gin
  family name_ops as
    function 2 (name, name) gin_extract_value_name(name, internal),
    function 3 (
      name,
      name
    )
    gin_extract_query_name(
      name,
      internal,
      smallint,
      internal,
      internal
    ),
    storage name;

create operator class numeric_ops
  default for type numeric
  using gin
  family numeric_ops as
    function 2 (numeric, numeric) gin_extract_value_numeric(numeric, internal),
    function 3 (
      numeric,
      numeric
    )
    gin_extract_query_numeric(
      numeric,
      internal,
      smallint,
      internal,
      internal
    ),
    storage numeric;

create operator class oid_ops
  default for type oid
  using gin
  family oid_ops as
    function 2 (oid, oid) gin_extract_value_oid(oid, internal),
    function 3 (
      oid,
      oid
    )
    gin_extract_query_oid(
      oid,
      internal,
      smallint,
      internal,
      internal
    ),
    storage oid;

create operator class text_ops
  default for type text
  using gin
  family text_ops as
    function 2 (text, text) gin_extract_value_text(text, internal),
    function 3 (
      text,
      text
    )
    gin_extract_query_text(
      text,
      internal,
      smallint,
      internal,
      internal
    ),
    storage text;

create operator class time_ops
  default for type time without time zone
  using gin
  family time_ops as
    function 2 (
      time without time zone,
      time without time zone
    )
    gin_extract_value_time(
      time without time zone,
      internal
    ),
    function 3 (
      time without time zone,
      time without time zone
    )
    gin_extract_query_time(
      time without time zone,
      internal,
      smallint,
      internal,
      internal
    ),
    storage time without time zone;

create operator class timestamp_ops
  default for type timestamp without time zone
  using gin
  family timestamp_ops as
    function 2 (
      timestamp without time zone,
      timestamp without time zone
    )
    gin_extract_value_timestamp(
      timestamp without time zone,
      internal
    ),
    function 3 (
      timestamp without time zone,
      timestamp without time zone
    )
    gin_extract_query_timestamp(
      timestamp without time zone,
      internal,
      smallint,
      internal,
      internal
    ),
    storage timestamp without time zone;

create operator class timestamptz_ops
  default for type timestamp with time zone
  using gin
  family timestamptz_ops as
    function 2 (
      timestamp with time zone,
      timestamp with time zone
    )
    gin_extract_value_timestamptz(
      timestamp with time zone,
      internal
    ),
    function 3 (
      timestamp with time zone,
      timestamp with time zone
    )
    gin_extract_query_timestamptz(
      timestamp with time zone,
      internal,
      smallint,
      internal,
      internal
    ),
    storage timestamp with time zone;

create operator class timetz_ops
  default for type time with time zone
  using gin
  family timetz_ops as
    function 2 (
      time with time zone,
      time with time zone
    )
    gin_extract_value_timetz(
      time with time zone,
      internal
    ),
    function 3 (
      time with time zone,
      time with time zone
    )
    gin_extract_query_timetz(
      time with time zone,
      internal,
      smallint,
      internal,
      internal
    ),
    storage time with time zone;

create operator class uuid_ops
  default for type uuid
  using gin
  family uuid_ops as
    function 2 (uuid, uuid) gin_extract_value_uuid(uuid, internal),
    function 3 (
      uuid,
      uuid
    )
    gin_extract_query_uuid(
      uuid,
      internal,
      smallint,
      internal,
      internal
    ),
    storage uuid;

create operator class varbit_ops
  default for type bit varying
  using gin
  family varbit_ops as
    function 2 (
      bit varying,
      bit varying
    )
    gin_extract_value_varbit(
      bit varying,
      internal
    ),
    function 3 (
      bit varying,
      bit varying
    )
    gin_extract_query_varbit(
      bit varying,
      internal,
      smallint,
      internal,
      internal
    ),
    storage bit varying;

create operator class varchar_ops
  default for type character varying
  using gin
  family varchar_ops as
    function 2 (
      character varying,
      character varying
    )
    gin_extract_value_text(
      text,
      internal
    ),
    function 3 (
      character varying,
      character varying
    )
    gin_extract_query_text(
      text,
      internal,
      smallint,
      internal,
      internal
    ),
    storage character varying;

alter operator family bit_ops
  using gin
  add
    operator 1 pg_catalog.< (bit, bit),
    operator 2 pg_catalog.<= (bit, bit),
    operator 3 pg_catalog.= (bit, bit),
    operator 4 pg_catalog.>= (bit, bit),
    operator 5 pg_catalog.> (bit, bit),
    function 1 (bit, bit) pg_catalog.bitcmp(bit, bit),
    function 4 (
      bit,
      bit
    )
    gin_btree_consistent(
      internal,
      smallint,
      anyelement,
      integer,
      internal,
      internal
    ),
    function 5 (bit, bit) gin_compare_prefix_bit(bit, bit, smallint, internal);

alter operator family bool_ops
  using gin
  add
    operator 1 pg_catalog.< (boolean, boolean),
    operator 2 pg_catalog.<= (boolean, boolean),
    operator 3 pg_catalog.= (boolean, boolean),
    operator 4 pg_catalog.>= (boolean, boolean),
    operator 5 pg_catalog.> (boolean, boolean),
    function 1 (boolean, boolean) pg_catalog.btboolcmp(boolean, boolean),
    function 4 (
      boolean,
      boolean
    )
    gin_btree_consistent(
      internal,
      smallint,
      anyelement,
      integer,
      internal,
      internal
    ),
    function 5 (
      boolean,
      boolean
    )
    gin_compare_prefix_bool(
      boolean,
      boolean,
      smallint,
      internal
    );

alter operator family bpchar_ops
  using gin
  add
    operator 1 pg_catalog.< (character, character),
    operator 2 pg_catalog.<= (character, character),
    operator 3 pg_catalog.= (character, character),
    operator 4 pg_catalog.>= (character, character),
    operator 5 pg_catalog.> (character, character),
    function 1 (
      character,
      character
    )
    pg_catalog.bpcharcmp(
      character,
      character
    ),
    function 4 (
      character,
      character
    )
    gin_btree_consistent(
      internal,
      smallint,
      anyelement,
      integer,
      internal,
      internal
    ),
    function 5 (
      character,
      character
    )
    gin_compare_prefix_bpchar(
      character,
      character,
      smallint,
      internal
    );

alter operator family bytea_ops
  using gin
  add
    operator 1 pg_catalog.< (bytea, bytea),
    operator 2 pg_catalog.<= (bytea, bytea),
    operator 3 pg_catalog.= (bytea, bytea),
    operator 4 pg_catalog.>= (bytea, bytea),
    operator 5 pg_catalog.> (bytea, bytea),
    function 1 (bytea, bytea) pg_catalog.byteacmp(bytea, bytea),
    function 4 (
      bytea,
      bytea
    )
    gin_btree_consistent(
      internal,
      smallint,
      anyelement,
      integer,
      internal,
      internal
    ),
    function 5 (
      bytea,
      bytea
    )
    gin_compare_prefix_bytea(
      bytea,
      bytea,
      smallint,
      internal
    );

alter operator family char_ops
  using gin
  add
    operator 1 pg_catalog.< ("char", "char"),
    operator 2 pg_catalog.<= ("char", "char"),
    operator 3 pg_catalog.= ("char", "char"),
    operator 4 pg_catalog.>= ("char", "char"),
    operator 5 pg_catalog.> ("char", "char"),
    function 1 ("char", "char") pg_catalog.btcharcmp("char", "char"),
    function 4 (
      "char",
      "char"
    )
    gin_btree_consistent(
      internal,
      smallint,
      anyelement,
      integer,
      internal,
      internal
    ),
    function 5 (
      "char",
      "char"
    )
    gin_compare_prefix_char(
      "char",
      "char",
      smallint,
      internal
    );

alter operator family cidr_ops
  using gin
  add
    operator 1 pg_catalog.< (inet, inet),
    operator 2 pg_catalog.<= (inet, inet),
    operator 3 pg_catalog.= (inet, inet),
    operator 4 pg_catalog.>= (inet, inet),
    operator 5 pg_catalog.> (inet, inet),
    function 1 (cidr, cidr) pg_catalog.network_cmp(inet, inet),
    function 4 (
      cidr,
      cidr
    )
    gin_btree_consistent(
      internal,
      smallint,
      anyelement,
      integer,
      internal,
      internal
    ),
    function 5 (
      cidr,
      cidr
    )
    gin_compare_prefix_cidr(
      cidr,
      cidr,
      smallint,
      internal
    );

alter operator family date_ops
  using gin
  add
    operator 1 pg_catalog.< (date, date),
    operator 2 pg_catalog.<= (date, date),
    operator 3 pg_catalog.= (date, date),
    operator 4 pg_catalog.>= (date, date),
    operator 5 pg_catalog.> (date, date),
    function 1 (date, date) pg_catalog.date_cmp(date, date),
    function 4 (
      date,
      date
    )
    gin_btree_consistent(
      internal,
      smallint,
      anyelement,
      integer,
      internal,
      internal
    ),
    function 5 (
      date,
      date
    )
    gin_compare_prefix_date(
      date,
      date,
      smallint,
      internal
    );

alter operator family enum_ops
  using gin
  add
    operator 1 pg_catalog.< (anyenum, anyenum),
    operator 2 pg_catalog.<= (anyenum, anyenum),
    operator 3 pg_catalog.= (anyenum, anyenum),
    operator 4 pg_catalog.>= (anyenum, anyenum),
    operator 5 pg_catalog.> (anyenum, anyenum),
    function 1 (anyenum, anyenum) gin_enum_cmp(anyenum, anyenum),
    function 4 (
      anyenum,
      anyenum
    )
    gin_btree_consistent(
      internal,
      smallint,
      anyelement,
      integer,
      internal,
      internal
    ),
    function 5 (
      anyenum,
      anyenum
    )
    gin_compare_prefix_anyenum(
      anyenum,
      anyenum,
      smallint,
      internal
    );

alter operator family float4_ops
  using gin
  add
    operator 1 pg_catalog.< (real, real),
    operator 2 pg_catalog.<= (real, real),
    operator 3 pg_catalog.= (real, real),
    operator 4 pg_catalog.>= (real, real),
    operator 5 pg_catalog.> (real, real),
    function 1 (real, real) pg_catalog.btfloat4cmp(real, real),
    function 4 (
      real,
      real
    )
    gin_btree_consistent(
      internal,
      smallint,
      anyelement,
      integer,
      internal,
      internal
    ),
    function 5 (
      real,
      real
    )
    gin_compare_prefix_float4(
      real,
      real,
      smallint,
      internal
    );

alter operator family float8_ops
  using gin
  add
    operator 1 pg_catalog.< (double precision, double precision),
    operator 2 pg_catalog.<= (double precision, double precision),
    operator 3 pg_catalog.= (double precision, double precision),
    operator 4 pg_catalog.>= (double precision, double precision),
    operator 5 pg_catalog.> (double precision, double precision),
    function 1 (
      double precision,
      double precision
    )
    pg_catalog.btfloat8cmp(
      double precision,
      double precision
    ),
    function 4 (
      double precision,
      double precision
    )
    gin_btree_consistent(
      internal,
      smallint,
      anyelement,
      integer,
      internal,
      internal
    ),
    function 5 (
      double precision,
      double precision
    )
    gin_compare_prefix_float8(
      double precision,
      double precision,
      smallint,
      internal
    );

alter operator family inet_ops
  using gin
  add
    operator 1 pg_catalog.< (inet, inet),
    operator 2 pg_catalog.<= (inet, inet),
    operator 3 pg_catalog.= (inet, inet),
    operator 4 pg_catalog.>= (inet, inet),
    operator 5 pg_catalog.> (inet, inet),
    function 1 (inet, inet) pg_catalog.network_cmp(inet, inet),
    function 4 (
      inet,
      inet
    )
    gin_btree_consistent(
      internal,
      smallint,
      anyelement,
      integer,
      internal,
      internal
    ),
    function 5 (
      inet,
      inet
    )
    gin_compare_prefix_inet(
      inet,
      inet,
      smallint,
      internal
    );

alter operator family int2_ops
  using gin
  add
    operator 1 pg_catalog.< (smallint, smallint),
    operator 2 pg_catalog.<= (smallint, smallint),
    operator 3 pg_catalog.= (smallint, smallint),
    operator 4 pg_catalog.>= (smallint, smallint),
    operator 5 pg_catalog.> (smallint, smallint),
    function 1 (smallint, smallint) pg_catalog.btint2cmp(smallint, smallint),
    function 4 (
      smallint,
      smallint
    )
    gin_btree_consistent(
      internal,
      smallint,
      anyelement,
      integer,
      internal,
      internal
    ),
    function 5 (
      smallint,
      smallint
    )
    gin_compare_prefix_int2(
      smallint,
      smallint,
      smallint,
      internal
    );

alter operator family int4_ops
  using gin
  add
    operator 1 pg_catalog.< (integer, integer),
    operator 2 pg_catalog.<= (integer, integer),
    operator 3 pg_catalog.= (integer, integer),
    operator 4 pg_catalog.>= (integer, integer),
    operator 5 pg_catalog.> (integer, integer),
    function 1 (integer, integer) pg_catalog.btint4cmp(integer, integer),
    function 4 (
      integer,
      integer
    )
    gin_btree_consistent(
      internal,
      smallint,
      anyelement,
      integer,
      internal,
      internal
    ),
    function 5 (
      integer,
      integer
    )
    gin_compare_prefix_int4(
      integer,
      integer,
      smallint,
      internal
    );

alter operator family int8_ops
  using gin
  add
    operator 1 pg_catalog.< (bigint, bigint),
    operator 2 pg_catalog.<= (bigint, bigint),
    operator 3 pg_catalog.= (bigint, bigint),
    operator 4 pg_catalog.>= (bigint, bigint),
    operator 5 pg_catalog.> (bigint, bigint),
    function 1 (bigint, bigint) pg_catalog.btint8cmp(bigint, bigint),
    function 4 (
      bigint,
      bigint
    )
    gin_btree_consistent(
      internal,
      smallint,
      anyelement,
      integer,
      internal,
      internal
    ),
    function 5 (
      bigint,
      bigint
    )
    gin_compare_prefix_int8(
      bigint,
      bigint,
      smallint,
      internal
    );

alter operator family interval_ops
  using gin
  add
    operator 1 pg_catalog.< (interval, interval),
    operator 2 pg_catalog.<= (interval, interval),
    operator 3 pg_catalog.= (interval, interval),
    operator 4 pg_catalog.>= (interval, interval),
    operator 5 pg_catalog.> (interval, interval),
    function 1 (interval, interval) pg_catalog.interval_cmp(interval, interval),
    function 4 (
      interval,
      interval
    )
    gin_btree_consistent(
      internal,
      smallint,
      anyelement,
      integer,
      internal,
      internal
    ),
    function 5 (
      interval,
      interval
    )
    gin_compare_prefix_interval(
      interval,
      interval,
      smallint,
      internal
    );

alter operator family macaddr8_ops
  using gin
  add
    operator 1 pg_catalog.< (macaddr8, macaddr8),
    operator 2 pg_catalog.<= (macaddr8, macaddr8),
    operator 3 pg_catalog.= (macaddr8, macaddr8),
    operator 4 pg_catalog.>= (macaddr8, macaddr8),
    operator 5 pg_catalog.> (macaddr8, macaddr8),
    function 1 (macaddr8, macaddr8) pg_catalog.macaddr8_cmp(macaddr8, macaddr8),
    function 4 (
      macaddr8,
      macaddr8
    )
    gin_btree_consistent(
      internal,
      smallint,
      anyelement,
      integer,
      internal,
      internal
    ),
    function 5 (
      macaddr8,
      macaddr8
    )
    gin_compare_prefix_macaddr8(
      macaddr8,
      macaddr8,
      smallint,
      internal
    );

alter operator family macaddr_ops
  using gin
  add
    operator 1 pg_catalog.< (macaddr, macaddr),
    operator 2 pg_catalog.<= (macaddr, macaddr),
    operator 3 pg_catalog.= (macaddr, macaddr),
    operator 4 pg_catalog.>= (macaddr, macaddr),
    operator 5 pg_catalog.> (macaddr, macaddr),
    function 1 (macaddr, macaddr) pg_catalog.macaddr_cmp(macaddr, macaddr),
    function 4 (
      macaddr,
      macaddr
    )
    gin_btree_consistent(
      internal,
      smallint,
      anyelement,
      integer,
      internal,
      internal
    ),
    function 5 (
      macaddr,
      macaddr
    )
    gin_compare_prefix_macaddr(
      macaddr,
      macaddr,
      smallint,
      internal
    );

alter operator family money_ops
  using gin
  add
    operator 1 pg_catalog.< (money, money),
    operator 2 pg_catalog.<= (money, money),
    operator 3 pg_catalog.= (money, money),
    operator 4 pg_catalog.>= (money, money),
    operator 5 pg_catalog.> (money, money),
    function 1 (money, money) pg_catalog.cash_cmp(money, money),
    function 4 (
      money,
      money
    )
    gin_btree_consistent(
      internal,
      smallint,
      anyelement,
      integer,
      internal,
      internal
    ),
    function 5 (
      money,
      money
    )
    gin_compare_prefix_money(
      money,
      money,
      smallint,
      internal
    );

alter operator family name_ops
  using gin
  add
    operator 1 pg_catalog.< (name, name),
    operator 2 pg_catalog.<= (name, name),
    operator 3 pg_catalog.= (name, name),
    operator 4 pg_catalog.>= (name, name),
    operator 5 pg_catalog.> (name, name),
    function 1 (name, name) pg_catalog.btnamecmp(name, name),
    function 4 (
      name,
      name
    )
    gin_btree_consistent(
      internal,
      smallint,
      anyelement,
      integer,
      internal,
      internal
    ),
    function 5 (
      name,
      name
    )
    gin_compare_prefix_name(
      name,
      name,
      smallint,
      internal
    );

alter operator family numeric_ops
  using gin
  add
    operator 1 pg_catalog.< (numeric, numeric),
    operator 2 pg_catalog.<= (numeric, numeric),
    operator 3 pg_catalog.= (numeric, numeric),
    operator 4 pg_catalog.>= (numeric, numeric),
    operator 5 pg_catalog.> (numeric, numeric),
    function 1 (numeric, numeric) gin_numeric_cmp(numeric, numeric),
    function 4 (
      numeric,
      numeric
    )
    gin_btree_consistent(
      internal,
      smallint,
      anyelement,
      integer,
      internal,
      internal
    ),
    function 5 (
      numeric,
      numeric
    )
    gin_compare_prefix_numeric(
      numeric,
      numeric,
      smallint,
      internal
    );

alter operator family oid_ops
  using gin
  add
    operator 1 pg_catalog.< (oid, oid),
    operator 2 pg_catalog.<= (oid, oid),
    operator 3 pg_catalog.= (oid, oid),
    operator 4 pg_catalog.>= (oid, oid),
    operator 5 pg_catalog.> (oid, oid),
    function 1 (oid, oid) pg_catalog.btoidcmp(oid, oid),
    function 4 (
      oid,
      oid
    )
    gin_btree_consistent(
      internal,
      smallint,
      anyelement,
      integer,
      internal,
      internal
    ),
    function 5 (oid, oid) gin_compare_prefix_oid(oid, oid, smallint, internal);

alter operator family text_ops
  using gin
  add
    operator 1 pg_catalog.< (text, text),
    operator 2 pg_catalog.<= (text, text),
    operator 3 pg_catalog.= (text, text),
    operator 4 pg_catalog.>= (text, text),
    operator 5 pg_catalog.> (text, text),
    function 1 (text, text) pg_catalog.bttextcmp(text, text),
    function 4 (
      text,
      text
    )
    gin_btree_consistent(
      internal,
      smallint,
      anyelement,
      integer,
      internal,
      internal
    ),
    function 5 (
      text,
      text
    )
    gin_compare_prefix_text(
      text,
      text,
      smallint,
      internal
    );

alter operator family time_ops
  using gin
  add
    operator 1 pg_catalog.< (time without time zone, time without time zone),
    operator 2 pg_catalog.<= (time without time zone, time without time zone),
    operator 3 pg_catalog.= (time without time zone, time without time zone),
    operator 4 pg_catalog.>= (time without time zone, time without time zone),
    operator 5 pg_catalog.> (time without time zone, time without time zone),
    function 1 (
      time without time zone,
      time without time zone
    )
    pg_catalog.time_cmp(
      time without time zone,
      time without time zone
    ),
    function 4 (
      time without time zone,
      time without time zone
    )
    gin_btree_consistent(
      internal,
      smallint,
      anyelement,
      integer,
      internal,
      internal
    ),
    function 5 (
      time without time zone,
      time without time zone
    )
    gin_compare_prefix_time(
      time without time zone,
      time without time zone,
      smallint,
      internal
    );

alter operator family timestamp_ops
  using gin
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
    function 1 (
      timestamp without time zone,
      timestamp without time zone
    )
    pg_catalog.timestamp_cmp(
      timestamp without time zone,
      timestamp without time zone
    ),
    function 4 (
      timestamp without time zone,
      timestamp without time zone
    )
    gin_btree_consistent(
      internal,
      smallint,
      anyelement,
      integer,
      internal,
      internal
    ),
    function 5 (
      timestamp without time zone,
      timestamp without time zone
    )
    gin_compare_prefix_timestamp(
      timestamp without time zone,
      timestamp without time zone,
      smallint,
      internal
    );

alter operator family timestamptz_ops
  using gin
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
    function 1 (
      timestamp with time zone,
      timestamp with time zone
    )
    pg_catalog.timestamptz_cmp(
      timestamp with time zone,
      timestamp with time zone
    ),
    function 4 (
      timestamp with time zone,
      timestamp with time zone
    )
    gin_btree_consistent(
      internal,
      smallint,
      anyelement,
      integer,
      internal,
      internal
    ),
    function 5 (
      timestamp with time zone,
      timestamp with time zone
    )
    gin_compare_prefix_timestamptz(
      timestamp with time zone,
      timestamp with time zone,
      smallint,
      internal
    );

alter operator family timetz_ops
  using gin
  add
    operator 1 pg_catalog.< (time with time zone, time with time zone),
    operator 2 pg_catalog.<= (time with time zone, time with time zone),
    operator 3 pg_catalog.= (time with time zone, time with time zone),
    operator 4 pg_catalog.>= (time with time zone, time with time zone),
    operator 5 pg_catalog.> (time with time zone, time with time zone),
    function 1 (
      time with time zone,
      time with time zone
    )
    pg_catalog.timetz_cmp(
      time with time zone,
      time with time zone
    ),
    function 4 (
      time with time zone,
      time with time zone
    )
    gin_btree_consistent(
      internal,
      smallint,
      anyelement,
      integer,
      internal,
      internal
    ),
    function 5 (
      time with time zone,
      time with time zone
    )
    gin_compare_prefix_timetz(
      time with time zone,
      time with time zone,
      smallint,
      internal
    );

alter operator family uuid_ops
  using gin
  add
    operator 1 pg_catalog.< (uuid, uuid),
    operator 2 pg_catalog.<= (uuid, uuid),
    operator 3 pg_catalog.= (uuid, uuid),
    operator 4 pg_catalog.>= (uuid, uuid),
    operator 5 pg_catalog.> (uuid, uuid),
    function 1 (uuid, uuid) pg_catalog.uuid_cmp(uuid, uuid),
    function 4 (
      uuid,
      uuid
    )
    gin_btree_consistent(
      internal,
      smallint,
      anyelement,
      integer,
      internal,
      internal
    ),
    function 5 (
      uuid,
      uuid
    )
    gin_compare_prefix_uuid(
      uuid,
      uuid,
      smallint,
      internal
    );

alter operator family varbit_ops
  using gin
  add
    operator 1 pg_catalog.< (bit varying, bit varying),
    operator 2 pg_catalog.<= (bit varying, bit varying),
    operator 3 pg_catalog.= (bit varying, bit varying),
    operator 4 pg_catalog.>= (bit varying, bit varying),
    operator 5 pg_catalog.> (bit varying, bit varying),
    function 1 (
      bit varying,
      bit varying
    )
    pg_catalog.varbitcmp(
      bit varying,
      bit varying
    ),
    function 4 (
      bit varying,
      bit varying
    )
    gin_btree_consistent(
      internal,
      smallint,
      anyelement,
      integer,
      internal,
      internal
    ),
    function 5 (
      bit varying,
      bit varying
    )
    gin_compare_prefix_varbit(
      bit varying,
      bit varying,
      smallint,
      internal
    );

alter operator family varchar_ops
  using gin
  add
    operator 1 pg_catalog.< (text, text),
    operator 2 pg_catalog.<= (text, text),
    operator 3 pg_catalog.= (text, text),
    operator 4 pg_catalog.>= (text, text),
    operator 5 pg_catalog.> (text, text),
    function 1 (
      character varying,
      character varying
    )
    pg_catalog.bttextcmp(
      text,
      text
    ),
    function 4 (
      character varying,
      character varying
    )
    gin_btree_consistent(
      internal,
      smallint,
      anyelement,
      integer,
      internal,
      internal
    ),
    function 5 (
      character varying,
      character varying
    )
    gin_compare_prefix_text(
      text,
      text,
      smallint,
      internal
    );
