-- squawk-ignore-file
-- pg version: 18.0
-- update via:
--   cargo xtask sync-builtins

-- size: 16, align: 8
create type aclitem;

-- size: 4, align: 4
create type any;

-- size: -1, align: 8
create type anyarray;

-- size: 4, align: 4
create type anycompatible;

-- size: -1, align: 8
create type anycompatiblearray;

-- size: -1, align: 8
create type anycompatiblemultirange;

-- size: 4, align: 4
create type anycompatiblenonarray;

-- size: -1, align: 8
create type anycompatiblerange;

-- size: 4, align: 4
create type anyelement;

-- size: 4, align: 4
create type anyenum;

-- size: -1, align: 8
create type anymultirange;

-- size: 4, align: 4
create type anynonarray;

-- size: -1, align: 8
create type anyrange;

-- size: -1, align: 4
create type bit;

-- size: 1, align: 1
create type bool;

-- size: 32, align: 8
create type box;

-- size: -1, align: 4
create type bpchar;

-- size: -1, align: 4
create type bytea;

-- size: 1, align: 1
create type char;

-- size: 4, align: 4
create type cid;

-- size: -1, align: 4
create type cidr;

-- size: 24, align: 8
create type circle;

-- size: -2, align: 1
create type cstring;

-- size: 4, align: 4
create type date;

-- size: -1, align: 4
create type daterange;

-- size: 4, align: 4
create type event_trigger;

-- size: 4, align: 4
create type fdw_handler;

-- size: 4, align: 4
create type float4;

-- size: 8, align: 8
create type float8;

-- size: -1, align: 4
create type gtsvector;

-- size: 4, align: 4
create type index_am_handler;

-- size: -1, align: 4
create type inet;

-- size: 2, align: 2
create type int2;

-- size: -1, align: 4
create type int2vector;

-- size: 4, align: 4
create type int4;

-- size: -1, align: 4
create type int4range;

-- size: 8, align: 8
create type int8;

-- size: -1, align: 8
create type int8range;

-- size: 8, align: 8
create type internal;

-- size: 16, align: 8
create type interval;

-- size: -1, align: 4
create type json;

-- size: -1, align: 4
create type jsonb;

-- size: -1, align: 4
create type jsonpath;

-- size: 4, align: 4
create type language_handler;

-- size: 24, align: 8
create type line;

-- size: 32, align: 8
create type lseg;

-- size: 6, align: 4
create type macaddr;

-- size: 8, align: 4
create type macaddr8;

-- size: 8, align: 8
create type money;

-- size: 64, align: 1
create type name;

-- size: -1, align: 4
create type numeric;

-- size: -1, align: 4
create type numrange;

-- size: 4, align: 4
create type oid;

-- size: -1, align: 4
create type oidvector;

-- size: -1, align: 8
create type path;

-- size: -1, align: 4
create type pg_brin_bloom_summary;

-- size: -1, align: 4
create type pg_brin_minmax_multi_summary;

-- size: 8, align: 8
create type pg_ddl_command;

-- size: -1, align: 4
create type pg_dependencies;

-- size: 8, align: 8
create type pg_lsn;

-- size: -1, align: 4
create type pg_mcv_list;

-- size: -1, align: 4
create type pg_ndistinct;

-- size: -1, align: 4
create type pg_node_tree;

-- size: -1, align: 8
create type pg_snapshot;

-- size: 16, align: 8
create type point;

-- size: -1, align: 8
create type polygon;

-- size: -1, align: 8
create type record;

-- size: -1, align: 4
create type refcursor;

-- size: 4, align: 4
create type regclass;

-- size: 4, align: 4
create type regcollation;

-- size: 4, align: 4
create type regconfig;

-- size: 4, align: 4
create type regdictionary;

-- size: 4, align: 4
create type regnamespace;

-- size: 4, align: 4
create type regoper;

-- size: 4, align: 4
create type regoperator;

-- size: 4, align: 4
create type regproc;

-- size: 4, align: 4
create type regprocedure;

-- size: 4, align: 4
create type regrole;

-- size: 4, align: 4
create type regtype;

-- size: 4, align: 4
create type table_am_handler;

-- size: -1, align: 4
create type text;

-- size: 6, align: 2
create type tid;

-- size: 8, align: 8
create type time;

-- size: 8, align: 8
create type timestamp;

-- size: 8, align: 8
create type timestamptz;

-- size: 12, align: 8
create type timetz;

-- size: 4, align: 4
create type trigger;

-- size: 4, align: 4
create type tsm_handler;

-- size: -1, align: 4
create type tsquery;

-- size: -1, align: 8
create type tsrange;

-- size: -1, align: 8
create type tstzrange;

-- size: -1, align: 4
create type tsvector;

-- size: -1, align: 8
create type txid_snapshot;

-- size: -2, align: 1
create type unknown;

-- size: 16, align: 1
create type uuid;

-- size: -1, align: 4
create type varbit;

-- size: -1, align: 4
create type varchar;

-- size: 4, align: 4
create type void;

-- size: 4, align: 4
create type xid;

-- size: 8, align: 8
create type xid8;

-- size: -1, align: 4
create type xml;

