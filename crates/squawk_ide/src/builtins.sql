-- squawk-ignore-file
-- pg version: 18.0
-- update via:
--   cargo xtask sync-builtins

create schema information_schema;
create schema pg_catalog;
create schema pg_temp;

-- size: 4, align: 4
create type information_schema.cardinal_number;

-- size: -1, align: 4
create type information_schema.character_data;

-- size: 64, align: 1
create type information_schema.sql_identifier;

-- size: 8, align: 8
create type information_schema.time_stamp;

-- size: -1, align: 4
create type information_schema.yes_or_no;

-- size: 16, align: 8
create type pg_catalog.aclitem;

-- size: 4, align: 4
create type pg_catalog.any;

-- size: -1, align: 8
create type pg_catalog.anyarray;

-- size: 4, align: 4
create type pg_catalog.anycompatible;

-- size: -1, align: 8
create type pg_catalog.anycompatiblearray;

-- size: -1, align: 8
create type pg_catalog.anycompatiblemultirange;

-- size: 4, align: 4
create type pg_catalog.anycompatiblenonarray;

-- size: -1, align: 8
create type pg_catalog.anycompatiblerange;

-- size: 4, align: 4
create type pg_catalog.anyelement;

-- size: 4, align: 4
create type pg_catalog.anyenum;

-- size: -1, align: 8
create type pg_catalog.anymultirange;

-- size: 4, align: 4
create type pg_catalog.anynonarray;

-- size: -1, align: 8
create type pg_catalog.anyrange;

-- size: -1, align: 4
create type pg_catalog.bit;

-- size: 1, align: 1
create type pg_catalog.bool;

-- size: 32, align: 8
create type pg_catalog.box;

-- size: -1, align: 4
create type pg_catalog.bpchar;

-- size: -1, align: 4
create type pg_catalog.bytea;

-- size: 1, align: 1
create type pg_catalog.char;

-- size: 4, align: 4
create type pg_catalog.cid;

-- size: -1, align: 4
create type pg_catalog.cidr;

-- size: 24, align: 8
create type pg_catalog.circle;

-- size: -2, align: 1
create type pg_catalog.cstring;

-- size: 4, align: 4
create type pg_catalog.date;

-- size: -1, align: 4
create type pg_catalog.daterange;

-- size: 4, align: 4
create type pg_catalog.event_trigger;

-- size: 4, align: 4
create type pg_catalog.fdw_handler;

-- size: 4, align: 4
create type pg_catalog.float4;

-- size: 8, align: 8
create type pg_catalog.float8;

-- size: -1, align: 4
create type pg_catalog.gtsvector;

-- size: 4, align: 4
create type pg_catalog.index_am_handler;

-- size: -1, align: 4
create type pg_catalog.inet;

-- size: 2, align: 2
create type pg_catalog.int2;

-- size: -1, align: 4
create type pg_catalog.int2vector;

-- size: 4, align: 4
create type pg_catalog.int4;

-- size: -1, align: 4
create type pg_catalog.int4range;

-- size: 8, align: 8
create type pg_catalog.int8;

-- size: -1, align: 8
create type pg_catalog.int8range;

-- size: 8, align: 8
create type pg_catalog.internal;

-- size: 16, align: 8
create type pg_catalog.interval;

-- size: -1, align: 4
create type pg_catalog.json;

-- size: -1, align: 4
create type pg_catalog.jsonb;

-- size: -1, align: 4
create type pg_catalog.jsonpath;

-- size: 4, align: 4
create type pg_catalog.language_handler;

-- size: 24, align: 8
create type pg_catalog.line;

-- size: 32, align: 8
create type pg_catalog.lseg;

-- size: 6, align: 4
create type pg_catalog.macaddr;

-- size: 8, align: 4
create type pg_catalog.macaddr8;

-- size: 8, align: 8
create type pg_catalog.money;

-- size: 64, align: 1
create type pg_catalog.name;

-- size: -1, align: 4
create type pg_catalog.numeric;

-- size: -1, align: 4
create type pg_catalog.numrange;

-- size: 4, align: 4
create type pg_catalog.oid;

-- size: -1, align: 4
create type pg_catalog.oidvector;

-- size: -1, align: 8
create type pg_catalog.path;

-- size: -1, align: 4
create type pg_catalog.pg_brin_bloom_summary;

-- size: -1, align: 4
create type pg_catalog.pg_brin_minmax_multi_summary;

-- size: 8, align: 8
create type pg_catalog.pg_ddl_command;

-- size: -1, align: 4
create type pg_catalog.pg_dependencies;

-- size: 8, align: 8
create type pg_catalog.pg_lsn;

-- size: -1, align: 4
create type pg_catalog.pg_mcv_list;

-- size: -1, align: 4
create type pg_catalog.pg_ndistinct;

-- size: -1, align: 4
create type pg_catalog.pg_node_tree;

-- size: -1, align: 8
create type pg_catalog.pg_snapshot;

-- size: 16, align: 8
create type pg_catalog.point;

-- size: -1, align: 8
create type pg_catalog.polygon;

-- size: -1, align: 8
create type pg_catalog.record;

-- size: -1, align: 4
create type pg_catalog.refcursor;

-- size: 4, align: 4
create type pg_catalog.regclass;

-- size: 4, align: 4
create type pg_catalog.regcollation;

-- size: 4, align: 4
create type pg_catalog.regconfig;

-- size: 4, align: 4
create type pg_catalog.regdictionary;

-- size: 4, align: 4
create type pg_catalog.regnamespace;

-- size: 4, align: 4
create type pg_catalog.regoper;

-- size: 4, align: 4
create type pg_catalog.regoperator;

-- size: 4, align: 4
create type pg_catalog.regproc;

-- size: 4, align: 4
create type pg_catalog.regprocedure;

-- size: 4, align: 4
create type pg_catalog.regrole;

-- size: 4, align: 4
create type pg_catalog.regtype;

-- size: 4, align: 4
create type pg_catalog.table_am_handler;

-- size: -1, align: 4
create type pg_catalog.text;

-- size: 6, align: 2
create type pg_catalog.tid;

-- size: 8, align: 8
create type pg_catalog.time;

-- size: 8, align: 8
create type pg_catalog.timestamp;

-- size: 8, align: 8
create type pg_catalog.timestamptz;

-- size: 12, align: 8
create type pg_catalog.timetz;

-- size: 4, align: 4
create type pg_catalog.trigger;

-- size: 4, align: 4
create type pg_catalog.tsm_handler;

-- size: -1, align: 4
create type pg_catalog.tsquery;

-- size: -1, align: 8
create type pg_catalog.tsrange;

-- size: -1, align: 8
create type pg_catalog.tstzrange;

-- size: -1, align: 4
create type pg_catalog.tsvector;

-- size: -1, align: 8
create type pg_catalog.txid_snapshot;

-- size: -2, align: 1
create type pg_catalog.unknown;

-- size: 16, align: 1
create type pg_catalog.uuid;

-- size: -1, align: 4
create type pg_catalog.varbit;

-- size: -1, align: 4
create type pg_catalog.varchar;

-- size: 4, align: 4
create type pg_catalog.void;

-- size: 4, align: 4
create type pg_catalog.xid;

-- size: 8, align: 8
create type pg_catalog.xid8;

-- size: -1, align: 4
create type pg_catalog.xml;

create table information_schema.sql_features (
  feature_id information_schema.character_data,
  feature_name information_schema.character_data,
  sub_feature_id information_schema.character_data,
  sub_feature_name information_schema.character_data,
  is_supported information_schema.yes_or_no,
  is_verified_by information_schema.character_data,
  comments information_schema.character_data
);

create table information_schema.sql_implementation_info (
  implementation_info_id information_schema.character_data,
  implementation_info_name information_schema.character_data,
  integer_value information_schema.cardinal_number,
  character_value information_schema.character_data,
  comments information_schema.character_data
);

create table information_schema.sql_parts (
  feature_id information_schema.character_data,
  feature_name information_schema.character_data,
  is_supported information_schema.yes_or_no,
  is_verified_by information_schema.character_data,
  comments information_schema.character_data
);

create table information_schema.sql_sizing (
  sizing_id information_schema.cardinal_number,
  sizing_name information_schema.character_data,
  supported_value information_schema.cardinal_number,
  comments information_schema.character_data
);

create table pg_catalog.pg_aggregate (
  aggfnoid regproc,
  aggkind "char",
  aggnumdirectargs smallint,
  aggtransfn regproc,
  aggfinalfn regproc,
  aggcombinefn regproc,
  aggserialfn regproc,
  aggdeserialfn regproc,
  aggmtransfn regproc,
  aggminvtransfn regproc,
  aggmfinalfn regproc,
  aggfinalextra boolean,
  aggmfinalextra boolean,
  aggfinalmodify "char",
  aggmfinalmodify "char",
  aggsortop oid,
  aggtranstype oid,
  aggtransspace integer,
  aggmtranstype oid,
  aggmtransspace integer,
  agginitval text,
  aggminitval text
);

create table pg_catalog.pg_am (
  oid oid,
  amname name,
  amhandler regproc,
  amtype "char"
);

create table pg_catalog.pg_amop (
  oid oid,
  amopfamily oid,
  amoplefttype oid,
  amoprighttype oid,
  amopstrategy smallint,
  amoppurpose "char",
  amopopr oid,
  amopmethod oid,
  amopsortfamily oid
);

create table pg_catalog.pg_amproc (
  oid oid,
  amprocfamily oid,
  amproclefttype oid,
  amprocrighttype oid,
  amprocnum smallint,
  amproc regproc
);

create table pg_catalog.pg_attrdef (
  oid oid,
  adrelid oid,
  adnum smallint,
  adbin pg_node_tree
);

create table pg_catalog.pg_attribute (
  attrelid oid,
  attname name,
  atttypid oid,
  attlen smallint,
  attnum smallint,
  atttypmod integer,
  attndims smallint,
  attbyval boolean,
  attalign "char",
  attstorage "char",
  attcompression "char",
  attnotnull boolean,
  atthasdef boolean,
  atthasmissing boolean,
  attidentity "char",
  attgenerated "char",
  attisdropped boolean,
  attislocal boolean,
  attinhcount smallint,
  attcollation oid,
  attstattarget smallint,
  attacl aclitem[],
  attoptions text[],
  attfdwoptions text[],
  attmissingval anyarray
);

create table pg_catalog.pg_auth_members (
  oid oid,
  roleid oid,
  member oid,
  grantor oid,
  admin_option boolean,
  inherit_option boolean,
  set_option boolean
);

create table pg_catalog.pg_authid (
  oid oid,
  rolname name,
  rolsuper boolean,
  rolinherit boolean,
  rolcreaterole boolean,
  rolcreatedb boolean,
  rolcanlogin boolean,
  rolreplication boolean,
  rolbypassrls boolean,
  rolconnlimit integer,
  rolpassword text,
  rolvaliduntil timestamp with time zone
);

create table pg_catalog.pg_cast (
  oid oid,
  castsource oid,
  casttarget oid,
  castfunc oid,
  castcontext "char",
  castmethod "char"
);

create table pg_catalog.pg_class (
  oid oid,
  relname name,
  relnamespace oid,
  reltype oid,
  reloftype oid,
  relowner oid,
  relam oid,
  relfilenode oid,
  reltablespace oid,
  relpages integer,
  reltuples real,
  relallvisible integer,
  relallfrozen integer,
  reltoastrelid oid,
  relhasindex boolean,
  relisshared boolean,
  relpersistence "char",
  relkind "char",
  relnatts smallint,
  relchecks smallint,
  relhasrules boolean,
  relhastriggers boolean,
  relhassubclass boolean,
  relrowsecurity boolean,
  relforcerowsecurity boolean,
  relispopulated boolean,
  relreplident "char",
  relispartition boolean,
  relrewrite oid,
  relfrozenxid xid,
  relminmxid xid,
  relacl aclitem[],
  reloptions text[],
  relpartbound pg_node_tree
);

create table pg_catalog.pg_collation (
  oid oid,
  collname name,
  collnamespace oid,
  collowner oid,
  collprovider "char",
  collisdeterministic boolean,
  collencoding integer,
  collcollate text,
  collctype text,
  colllocale text,
  collicurules text,
  collversion text
);

create table pg_catalog.pg_constraint (
  oid oid,
  conname name,
  connamespace oid,
  contype "char",
  condeferrable boolean,
  condeferred boolean,
  conenforced boolean,
  convalidated boolean,
  conrelid oid,
  contypid oid,
  conindid oid,
  conparentid oid,
  confrelid oid,
  confupdtype "char",
  confdeltype "char",
  confmatchtype "char",
  conislocal boolean,
  coninhcount smallint,
  connoinherit boolean,
  conperiod boolean,
  conkey smallint[],
  confkey smallint[],
  conpfeqop oid[],
  conppeqop oid[],
  conffeqop oid[],
  confdelsetcols smallint[],
  conexclop oid[],
  conbin pg_node_tree
);

create table pg_catalog.pg_conversion (
  oid oid,
  conname name,
  connamespace oid,
  conowner oid,
  conforencoding integer,
  contoencoding integer,
  conproc regproc,
  condefault boolean
);

create table pg_catalog.pg_database (
  oid oid,
  datname name,
  datdba oid,
  encoding integer,
  datlocprovider "char",
  datistemplate boolean,
  datallowconn boolean,
  dathasloginevt boolean,
  datconnlimit integer,
  datfrozenxid xid,
  datminmxid xid,
  dattablespace oid,
  datcollate text,
  datctype text,
  datlocale text,
  daticurules text,
  datcollversion text,
  datacl aclitem[]
);

create table pg_catalog.pg_db_role_setting (
  setdatabase oid,
  setrole oid,
  setconfig text[]
);

create table pg_catalog.pg_default_acl (
  oid oid,
  defaclrole oid,
  defaclnamespace oid,
  defaclobjtype "char",
  defaclacl aclitem[]
);

create table pg_catalog.pg_depend (
  classid oid,
  objid oid,
  objsubid integer,
  refclassid oid,
  refobjid oid,
  refobjsubid integer,
  deptype "char"
);

create table pg_catalog.pg_description (
  objoid oid,
  classoid oid,
  objsubid integer,
  description text
);

create table pg_catalog.pg_enum (
  oid oid,
  enumtypid oid,
  enumsortorder real,
  enumlabel name
);

create table pg_catalog.pg_event_trigger (
  oid oid,
  evtname name,
  evtevent name,
  evtowner oid,
  evtfoid oid,
  evtenabled "char",
  evttags text[]
);

create table pg_catalog.pg_extension (
  oid oid,
  extname name,
  extowner oid,
  extnamespace oid,
  extrelocatable boolean,
  extversion text,
  extconfig oid[],
  extcondition text[]
);

create table pg_catalog.pg_foreign_data_wrapper (
  oid oid,
  fdwname name,
  fdwowner oid,
  fdwhandler oid,
  fdwvalidator oid,
  fdwacl aclitem[],
  fdwoptions text[]
);

create table pg_catalog.pg_foreign_server (
  oid oid,
  srvname name,
  srvowner oid,
  srvfdw oid,
  srvtype text,
  srvversion text,
  srvacl aclitem[],
  srvoptions text[]
);

create table pg_catalog.pg_foreign_table (
  ftrelid oid,
  ftserver oid,
  ftoptions text[]
);

create table pg_catalog.pg_index (
  indexrelid oid,
  indrelid oid,
  indnatts smallint,
  indnkeyatts smallint,
  indisunique boolean,
  indnullsnotdistinct boolean,
  indisprimary boolean,
  indisexclusion boolean,
  indimmediate boolean,
  indisclustered boolean,
  indisvalid boolean,
  indcheckxmin boolean,
  indisready boolean,
  indislive boolean,
  indisreplident boolean,
  indkey int2vector,
  indcollation oidvector,
  indclass oidvector,
  indoption int2vector,
  indexprs pg_node_tree,
  indpred pg_node_tree
);

create table pg_catalog.pg_inherits (
  inhrelid oid,
  inhparent oid,
  inhseqno integer,
  inhdetachpending boolean
);

create table pg_catalog.pg_init_privs (
  objoid oid,
  classoid oid,
  objsubid integer,
  privtype "char",
  initprivs aclitem[]
);

create table pg_catalog.pg_language (
  oid oid,
  lanname name,
  lanowner oid,
  lanispl boolean,
  lanpltrusted boolean,
  lanplcallfoid oid,
  laninline oid,
  lanvalidator oid,
  lanacl aclitem[]
);

create table pg_catalog.pg_largeobject (
  loid oid,
  pageno integer,
  data bytea
);

create table pg_catalog.pg_largeobject_metadata (
  oid oid,
  lomowner oid,
  lomacl aclitem[]
);

create table pg_catalog.pg_namespace (
  oid oid,
  nspname name,
  nspowner oid,
  nspacl aclitem[]
);

create table pg_catalog.pg_opclass (
  oid oid,
  opcmethod oid,
  opcname name,
  opcnamespace oid,
  opcowner oid,
  opcfamily oid,
  opcintype oid,
  opcdefault boolean,
  opckeytype oid
);

create table pg_catalog.pg_operator (
  oid oid,
  oprname name,
  oprnamespace oid,
  oprowner oid,
  oprkind "char",
  oprcanmerge boolean,
  oprcanhash boolean,
  oprleft oid,
  oprright oid,
  oprresult oid,
  oprcom oid,
  oprnegate oid,
  oprcode regproc,
  oprrest regproc,
  oprjoin regproc
);

create table pg_catalog.pg_opfamily (
  oid oid,
  opfmethod oid,
  opfname name,
  opfnamespace oid,
  opfowner oid
);

create table pg_catalog.pg_parameter_acl (
  oid oid,
  parname text,
  paracl aclitem[]
);

create table pg_catalog.pg_partitioned_table (
  partrelid oid,
  partstrat "char",
  partnatts smallint,
  partdefid oid,
  partattrs int2vector,
  partclass oidvector,
  partcollation oidvector,
  partexprs pg_node_tree
);

create table pg_catalog.pg_policy (
  oid oid,
  polname name,
  polrelid oid,
  polcmd "char",
  polpermissive boolean,
  polroles oid[],
  polqual pg_node_tree,
  polwithcheck pg_node_tree
);

create table pg_catalog.pg_proc (
  oid oid,
  proname name,
  pronamespace oid,
  proowner oid,
  prolang oid,
  procost real,
  prorows real,
  provariadic oid,
  prosupport regproc,
  prokind "char",
  prosecdef boolean,
  proleakproof boolean,
  proisstrict boolean,
  proretset boolean,
  provolatile "char",
  proparallel "char",
  pronargs smallint,
  pronargdefaults smallint,
  prorettype oid,
  proargtypes oidvector,
  proallargtypes oid[],
  proargmodes "char"[],
  proargnames text[],
  proargdefaults pg_node_tree,
  protrftypes oid[],
  prosrc text,
  probin text,
  prosqlbody pg_node_tree,
  proconfig text[],
  proacl aclitem[]
);

create table pg_catalog.pg_publication (
  oid oid,
  pubname name,
  pubowner oid,
  puballtables boolean,
  pubinsert boolean,
  pubupdate boolean,
  pubdelete boolean,
  pubtruncate boolean,
  pubviaroot boolean,
  pubgencols "char"
);

create table pg_catalog.pg_publication_namespace (
  oid oid,
  pnpubid oid,
  pnnspid oid
);

create table pg_catalog.pg_publication_rel (
  oid oid,
  prpubid oid,
  prrelid oid,
  prqual pg_node_tree,
  prattrs int2vector
);

create table pg_catalog.pg_range (
  rngtypid oid,
  rngsubtype oid,
  rngmultitypid oid,
  rngcollation oid,
  rngsubopc oid,
  rngcanonical regproc,
  rngsubdiff regproc
);

create table pg_catalog.pg_replication_origin (
  roident oid,
  roname text
);

create table pg_catalog.pg_rewrite (
  oid oid,
  rulename name,
  ev_class oid,
  ev_type "char",
  ev_enabled "char",
  is_instead boolean,
  ev_qual pg_node_tree,
  ev_action pg_node_tree
);

create table pg_catalog.pg_seclabel (
  objoid oid,
  classoid oid,
  objsubid integer,
  provider text,
  label text
);

create table pg_catalog.pg_sequence (
  seqrelid oid,
  seqtypid oid,
  seqstart bigint,
  seqincrement bigint,
  seqmax bigint,
  seqmin bigint,
  seqcache bigint,
  seqcycle boolean
);

create table pg_catalog.pg_shdepend (
  dbid oid,
  classid oid,
  objid oid,
  objsubid integer,
  refclassid oid,
  refobjid oid,
  deptype "char"
);

create table pg_catalog.pg_shdescription (
  objoid oid,
  classoid oid,
  description text
);

create table pg_catalog.pg_shseclabel (
  objoid oid,
  classoid oid,
  provider text,
  label text
);

create table pg_catalog.pg_statistic (
  starelid oid,
  staattnum smallint,
  stainherit boolean,
  stanullfrac real,
  stawidth integer,
  stadistinct real,
  stakind1 smallint,
  stakind2 smallint,
  stakind3 smallint,
  stakind4 smallint,
  stakind5 smallint,
  staop1 oid,
  staop2 oid,
  staop3 oid,
  staop4 oid,
  staop5 oid,
  stacoll1 oid,
  stacoll2 oid,
  stacoll3 oid,
  stacoll4 oid,
  stacoll5 oid,
  stanumbers1 real[],
  stanumbers2 real[],
  stanumbers3 real[],
  stanumbers4 real[],
  stanumbers5 real[],
  stavalues1 anyarray,
  stavalues2 anyarray,
  stavalues3 anyarray,
  stavalues4 anyarray,
  stavalues5 anyarray
);

create table pg_catalog.pg_statistic_ext (
  oid oid,
  stxrelid oid,
  stxname name,
  stxnamespace oid,
  stxowner oid,
  stxkeys int2vector,
  stxstattarget smallint,
  stxkind "char"[],
  stxexprs pg_node_tree
);

create table pg_catalog.pg_statistic_ext_data (
  stxoid oid,
  stxdinherit boolean,
  stxdndistinct pg_ndistinct,
  stxddependencies pg_dependencies,
  stxdmcv pg_mcv_list,
  stxdexpr pg_statistic[]
);

create table pg_catalog.pg_subscription (
  oid oid,
  subdbid oid,
  subskiplsn pg_lsn,
  subname name,
  subowner oid,
  subenabled boolean,
  subbinary boolean,
  substream "char",
  subtwophasestate "char",
  subdisableonerr boolean,
  subpasswordrequired boolean,
  subrunasowner boolean,
  subfailover boolean,
  subconninfo text,
  subslotname name,
  subsynccommit text,
  subpublications text[],
  suborigin text
);

create table pg_catalog.pg_subscription_rel (
  srsubid oid,
  srrelid oid,
  srsubstate "char",
  srsublsn pg_lsn
);

create table pg_catalog.pg_tablespace (
  oid oid,
  spcname name,
  spcowner oid,
  spcacl aclitem[],
  spcoptions text[]
);

create table pg_catalog.pg_transform (
  oid oid,
  trftype oid,
  trflang oid,
  trffromsql regproc,
  trftosql regproc
);

create table pg_catalog.pg_trigger (
  oid oid,
  tgrelid oid,
  tgparentid oid,
  tgname name,
  tgfoid oid,
  tgtype smallint,
  tgenabled "char",
  tgisinternal boolean,
  tgconstrrelid oid,
  tgconstrindid oid,
  tgconstraint oid,
  tgdeferrable boolean,
  tginitdeferred boolean,
  tgnargs smallint,
  tgattr int2vector,
  tgargs bytea,
  tgqual pg_node_tree,
  tgoldtable name,
  tgnewtable name
);

create table pg_catalog.pg_ts_config (
  oid oid,
  cfgname name,
  cfgnamespace oid,
  cfgowner oid,
  cfgparser oid
);

create table pg_catalog.pg_ts_config_map (
  mapcfg oid,
  maptokentype integer,
  mapseqno integer,
  mapdict oid
);

create table pg_catalog.pg_ts_dict (
  oid oid,
  dictname name,
  dictnamespace oid,
  dictowner oid,
  dicttemplate oid,
  dictinitoption text
);

create table pg_catalog.pg_ts_parser (
  oid oid,
  prsname name,
  prsnamespace oid,
  prsstart regproc,
  prstoken regproc,
  prsend regproc,
  prsheadline regproc,
  prslextype regproc
);

create table pg_catalog.pg_ts_template (
  oid oid,
  tmplname name,
  tmplnamespace oid,
  tmplinit regproc,
  tmpllexize regproc
);

create table pg_catalog.pg_type (
  oid oid,
  typname name,
  typnamespace oid,
  typowner oid,
  typlen smallint,
  typbyval boolean,
  typtype "char",
  typcategory "char",
  typispreferred boolean,
  typisdefined boolean,
  typdelim "char",
  typrelid oid,
  typsubscript regproc,
  typelem oid,
  typarray oid,
  typinput regproc,
  typoutput regproc,
  typreceive regproc,
  typsend regproc,
  typmodin regproc,
  typmodout regproc,
  typanalyze regproc,
  typalign "char",
  typstorage "char",
  typnotnull boolean,
  typbasetype oid,
  typtypmod integer,
  typndims integer,
  typcollation oid,
  typdefaultbin pg_node_tree,
  typdefault text,
  typacl aclitem[]
);

create table pg_catalog.pg_user_mapping (
  oid oid,
  umuser oid,
  umserver oid,
  umoptions text[]
);

create function information_schema._pg_char_max_length(typid oid, typmod integer) returns integer language internal;

create function information_schema._pg_char_octet_length(typid oid, typmod integer) returns integer language internal;

create function information_schema._pg_datetime_precision(typid oid, typmod integer) returns integer language internal;

create function information_schema._pg_expandarray(anyarray, OUT x anyelement, OUT n integer) returns SETOF record language internal;

create function information_schema._pg_index_position(oid, smallint) returns integer language internal;

create function information_schema._pg_interval_type(typid oid, mod integer) returns text language internal;

create function information_schema._pg_numeric_precision(typid oid, typmod integer) returns integer language internal;

create function information_schema._pg_numeric_precision_radix(typid oid, typmod integer) returns integer language internal;

create function information_schema._pg_numeric_scale(typid oid, typmod integer) returns integer language internal;

create function information_schema._pg_truetypid(pg_attribute, pg_type) returns oid language internal;

create function information_schema._pg_truetypmod(pg_attribute, pg_type) returns integer language internal;

create function pg_catalog.RI_FKey_cascade_del() returns trigger language internal;

create function pg_catalog.RI_FKey_cascade_upd() returns trigger language internal;

create function pg_catalog.RI_FKey_check_ins() returns trigger language internal;

create function pg_catalog.RI_FKey_check_upd() returns trigger language internal;

create function pg_catalog.RI_FKey_noaction_del() returns trigger language internal;

create function pg_catalog.RI_FKey_noaction_upd() returns trigger language internal;

create function pg_catalog.RI_FKey_restrict_del() returns trigger language internal;

create function pg_catalog.RI_FKey_restrict_upd() returns trigger language internal;

create function pg_catalog.RI_FKey_setdefault_del() returns trigger language internal;

create function pg_catalog.RI_FKey_setdefault_upd() returns trigger language internal;

create function pg_catalog.RI_FKey_setnull_del() returns trigger language internal;

create function pg_catalog.RI_FKey_setnull_upd() returns trigger language internal;

create function pg_catalog.abbrev(inet) returns text language internal;

create function pg_catalog.abbrev(cidr) returns text language internal;

create function pg_catalog.abs(integer) returns integer language internal;

create function pg_catalog.abs(double precision) returns double precision language internal;

create function pg_catalog.abs(real) returns real language internal;

create function pg_catalog.abs(smallint) returns smallint language internal;

create function pg_catalog.abs(bigint) returns bigint language internal;

create function pg_catalog.abs(numeric) returns numeric language internal;

create function pg_catalog.aclcontains(aclitem[], aclitem) returns boolean language internal;

create function pg_catalog.acldefault("char", oid) returns aclitem[] language internal;

create function pg_catalog.aclexplode(acl aclitem[], OUT grantor oid, OUT grantee oid, OUT privilege_type text, OUT is_grantable boolean) returns SETOF record language internal;

create function pg_catalog.aclinsert(aclitem[], aclitem) returns aclitem[] language internal;

create function pg_catalog.aclitemeq(aclitem, aclitem) returns boolean language internal;

create function pg_catalog.aclitemin(cstring) returns aclitem language internal;

create function pg_catalog.aclitemout(aclitem) returns cstring language internal;

create function pg_catalog.aclremove(aclitem[], aclitem) returns aclitem[] language internal;

create function pg_catalog.acos(double precision) returns double precision language internal;

create function pg_catalog.acosd(double precision) returns double precision language internal;

create function pg_catalog.acosh(double precision) returns double precision language internal;

create function pg_catalog.age(xid) returns integer language internal;

create function pg_catalog.age(timestamp with time zone, timestamp with time zone) returns interval language internal;

create function pg_catalog.age(timestamp without time zone, timestamp without time zone) returns interval language internal;

create function pg_catalog.age(timestamp without time zone) returns interval language internal;

create function pg_catalog.age(timestamp with time zone) returns interval language internal;

create function pg_catalog.amvalidate(oid) returns boolean language internal;

create function pg_catalog.any_in(cstring) returns "any" language internal;

create function pg_catalog.any_out("any") returns cstring language internal;

create function pg_catalog.any_value(anyelement) returns anyelement language internal;

create function pg_catalog.any_value_transfn(anyelement, anyelement) returns anyelement language internal;

create function pg_catalog.anyarray_in(cstring) returns anyarray language internal;

create function pg_catalog.anyarray_out(anyarray) returns cstring language internal;

create function pg_catalog.anyarray_recv(internal) returns anyarray language internal;

create function pg_catalog.anyarray_send(anyarray) returns bytea language internal;

create function pg_catalog.anycompatible_in(cstring) returns anycompatible language internal;

create function pg_catalog.anycompatible_out(anycompatible) returns cstring language internal;

create function pg_catalog.anycompatiblearray_in(cstring) returns anycompatiblearray language internal;

create function pg_catalog.anycompatiblearray_out(anycompatiblearray) returns cstring language internal;

create function pg_catalog.anycompatiblearray_recv(internal) returns anycompatiblearray language internal;

create function pg_catalog.anycompatiblearray_send(anycompatiblearray) returns bytea language internal;

create function pg_catalog.anycompatiblemultirange_in(cstring, oid, integer) returns anycompatiblemultirange language internal;

create function pg_catalog.anycompatiblemultirange_out(anycompatiblemultirange) returns cstring language internal;

create function pg_catalog.anycompatiblenonarray_in(cstring) returns anycompatiblenonarray language internal;

create function pg_catalog.anycompatiblenonarray_out(anycompatiblenonarray) returns cstring language internal;

create function pg_catalog.anycompatiblerange_in(cstring, oid, integer) returns anycompatiblerange language internal;

create function pg_catalog.anycompatiblerange_out(anycompatiblerange) returns cstring language internal;

create function pg_catalog.anyelement_in(cstring) returns anyelement language internal;

create function pg_catalog.anyelement_out(anyelement) returns cstring language internal;

create function pg_catalog.anyenum_in(cstring) returns anyenum language internal;

create function pg_catalog.anyenum_out(anyenum) returns cstring language internal;

create function pg_catalog.anymultirange_in(cstring, oid, integer) returns anymultirange language internal;

create function pg_catalog.anymultirange_out(anymultirange) returns cstring language internal;

create function pg_catalog.anynonarray_in(cstring) returns anynonarray language internal;

create function pg_catalog.anynonarray_out(anynonarray) returns cstring language internal;

create function pg_catalog.anyrange_in(cstring, oid, integer) returns anyrange language internal;

create function pg_catalog.anyrange_out(anyrange) returns cstring language internal;

create function pg_catalog.anytextcat(anynonarray, text) returns text language internal;

create function pg_catalog.area(path) returns double precision language internal;

create function pg_catalog.area(box) returns double precision language internal;

create function pg_catalog.area(circle) returns double precision language internal;

create function pg_catalog.areajoinsel(internal, oid, internal, smallint, internal) returns double precision language internal;

create function pg_catalog.areasel(internal, oid, internal, integer) returns double precision language internal;

create function pg_catalog.array_agg(anynonarray) returns anyarray language internal;

create function pg_catalog.array_agg(anyarray) returns anyarray language internal;

create function pg_catalog.array_agg_array_combine(internal, internal) returns internal language internal;

create function pg_catalog.array_agg_array_deserialize(bytea, internal) returns internal language internal;

create function pg_catalog.array_agg_array_finalfn(internal, anyarray) returns anyarray language internal;

create function pg_catalog.array_agg_array_serialize(internal) returns bytea language internal;

create function pg_catalog.array_agg_array_transfn(internal, anyarray) returns internal language internal;

create function pg_catalog.array_agg_combine(internal, internal) returns internal language internal;

create function pg_catalog.array_agg_deserialize(bytea, internal) returns internal language internal;

create function pg_catalog.array_agg_finalfn(internal, anynonarray) returns anyarray language internal;

create function pg_catalog.array_agg_serialize(internal) returns bytea language internal;

create function pg_catalog.array_agg_transfn(internal, anynonarray) returns internal language internal;

create function pg_catalog.array_append(anycompatiblearray, anycompatible) returns anycompatiblearray language internal;

create function pg_catalog.array_append_support(internal) returns internal language internal;

create function pg_catalog.array_cat(anycompatiblearray, anycompatiblearray) returns anycompatiblearray language internal;

create function pg_catalog.array_dims(anyarray) returns text language internal;

create function pg_catalog.array_eq(anyarray, anyarray) returns boolean language internal;

create function pg_catalog.array_fill(anyelement, integer[]) returns anyarray language internal;

create function pg_catalog.array_fill(anyelement, integer[], integer[]) returns anyarray language internal;

create function pg_catalog.array_ge(anyarray, anyarray) returns boolean language internal;

create function pg_catalog.array_gt(anyarray, anyarray) returns boolean language internal;

create function pg_catalog.array_in(cstring, oid, integer) returns anyarray language internal;

create function pg_catalog.array_larger(anyarray, anyarray) returns anyarray language internal;

create function pg_catalog.array_le(anyarray, anyarray) returns boolean language internal;

create function pg_catalog.array_length(anyarray, integer) returns integer language internal;

create function pg_catalog.array_lower(anyarray, integer) returns integer language internal;

create function pg_catalog.array_lt(anyarray, anyarray) returns boolean language internal;

create function pg_catalog.array_ndims(anyarray) returns integer language internal;

create function pg_catalog.array_ne(anyarray, anyarray) returns boolean language internal;

create function pg_catalog.array_out(anyarray) returns cstring language internal;

create function pg_catalog.array_position(anycompatiblearray, anycompatible) returns integer language internal;

create function pg_catalog.array_position(anycompatiblearray, anycompatible, integer) returns integer language internal;

create function pg_catalog.array_positions(anycompatiblearray, anycompatible) returns integer[] language internal;

create function pg_catalog.array_prepend(anycompatible, anycompatiblearray) returns anycompatiblearray language internal;

create function pg_catalog.array_prepend_support(internal) returns internal language internal;

create function pg_catalog.array_recv(internal, oid, integer) returns anyarray language internal;

create function pg_catalog.array_remove(anycompatiblearray, anycompatible) returns anycompatiblearray language internal;

create function pg_catalog.array_replace(anycompatiblearray, anycompatible, anycompatible) returns anycompatiblearray language internal;

create function pg_catalog.array_reverse(anyarray) returns anyarray language internal;

create function pg_catalog.array_sample(anyarray, integer) returns anyarray language internal;

create function pg_catalog.array_send(anyarray) returns bytea language internal;

create function pg_catalog.array_shuffle(anyarray) returns anyarray language internal;

create function pg_catalog.array_smaller(anyarray, anyarray) returns anyarray language internal;

create function pg_catalog.array_sort("array" anyarray, descending boolean, nulls_first boolean) returns anyarray language internal;

create function pg_catalog.array_sort(anyarray) returns anyarray language internal;

create function pg_catalog.array_sort("array" anyarray, descending boolean) returns anyarray language internal;

create function pg_catalog.array_subscript_handler(internal) returns internal language internal;

create function pg_catalog.array_subscript_handler_support(internal) returns internal language internal;

create function pg_catalog.array_to_json(anyarray, boolean) returns json language internal;

create function pg_catalog.array_to_json(anyarray) returns json language internal;

create function pg_catalog.array_to_string(anyarray, text) returns text language internal;

create function pg_catalog.array_to_string(anyarray, text, text) returns text language internal;

create function pg_catalog.array_to_tsvector(text[]) returns tsvector language internal;

create function pg_catalog.array_typanalyze(internal) returns boolean language internal;

create function pg_catalog.array_unnest_support(internal) returns internal language internal;

create function pg_catalog.array_upper(anyarray, integer) returns integer language internal;

create function pg_catalog.arraycontained(anyarray, anyarray) returns boolean language internal;

create function pg_catalog.arraycontains(anyarray, anyarray) returns boolean language internal;

create function pg_catalog.arraycontjoinsel(internal, oid, internal, smallint, internal) returns double precision language internal;

create function pg_catalog.arraycontsel(internal, oid, internal, integer) returns double precision language internal;

create function pg_catalog.arrayoverlap(anyarray, anyarray) returns boolean language internal;

create function pg_catalog.ascii(text) returns integer language internal;

create function pg_catalog.asin(double precision) returns double precision language internal;

create function pg_catalog.asind(double precision) returns double precision language internal;

create function pg_catalog.asinh(double precision) returns double precision language internal;

create function pg_catalog.atan(double precision) returns double precision language internal;

create function pg_catalog.atan2(double precision, double precision) returns double precision language internal;

create function pg_catalog.atan2d(double precision, double precision) returns double precision language internal;

create function pg_catalog.atand(double precision) returns double precision language internal;

create function pg_catalog.atanh(double precision) returns double precision language internal;

create function pg_catalog.avg(bigint) returns numeric language internal;

create function pg_catalog.avg(numeric) returns numeric language internal;

create function pg_catalog.avg(interval) returns interval language internal;

create function pg_catalog.avg(double precision) returns double precision language internal;

create function pg_catalog.avg(real) returns double precision language internal;

create function pg_catalog.avg(smallint) returns numeric language internal;

create function pg_catalog.avg(integer) returns numeric language internal;

create function pg_catalog.bernoulli(internal) returns tsm_handler language internal;

create function pg_catalog.big5_to_euc_tw(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.big5_to_mic(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.big5_to_utf8(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.binary_upgrade_add_sub_rel_state(text, oid, "char", pg_lsn) returns void language internal;

create function pg_catalog.binary_upgrade_create_empty_extension(text, text, boolean, text, oid[], text[], text[]) returns void language internal;

create function pg_catalog.binary_upgrade_logical_slot_has_caught_up(name) returns boolean language internal;

create function pg_catalog.binary_upgrade_replorigin_advance(text, pg_lsn) returns void language internal;

create function pg_catalog.binary_upgrade_set_missing_value(oid, text, text) returns void language internal;

create function pg_catalog.binary_upgrade_set_next_array_pg_type_oid(oid) returns void language internal;

create function pg_catalog.binary_upgrade_set_next_heap_pg_class_oid(oid) returns void language internal;

create function pg_catalog.binary_upgrade_set_next_heap_relfilenode(oid) returns void language internal;

create function pg_catalog.binary_upgrade_set_next_index_pg_class_oid(oid) returns void language internal;

create function pg_catalog.binary_upgrade_set_next_index_relfilenode(oid) returns void language internal;

create function pg_catalog.binary_upgrade_set_next_multirange_array_pg_type_oid(oid) returns void language internal;

create function pg_catalog.binary_upgrade_set_next_multirange_pg_type_oid(oid) returns void language internal;

create function pg_catalog.binary_upgrade_set_next_pg_authid_oid(oid) returns void language internal;

create function pg_catalog.binary_upgrade_set_next_pg_enum_oid(oid) returns void language internal;

create function pg_catalog.binary_upgrade_set_next_pg_tablespace_oid(oid) returns void language internal;

create function pg_catalog.binary_upgrade_set_next_pg_type_oid(oid) returns void language internal;

create function pg_catalog.binary_upgrade_set_next_toast_pg_class_oid(oid) returns void language internal;

create function pg_catalog.binary_upgrade_set_next_toast_relfilenode(oid) returns void language internal;

create function pg_catalog.binary_upgrade_set_record_init_privs(boolean) returns void language internal;

create function pg_catalog.bit(integer, integer) returns bit language internal;

create function pg_catalog.bit(bit, integer, boolean) returns bit language internal;

create function pg_catalog.bit(bigint, integer) returns bit language internal;

create function pg_catalog.bit_and(integer) returns integer language internal;

create function pg_catalog.bit_and(bit) returns bit language internal;

create function pg_catalog.bit_and(bigint) returns bigint language internal;

create function pg_catalog.bit_and(smallint) returns smallint language internal;

create function pg_catalog.bit_count(bit) returns bigint language internal;

create function pg_catalog.bit_count(bytea) returns bigint language internal;

create function pg_catalog.bit_in(cstring, oid, integer) returns bit language internal;

create function pg_catalog.bit_length(text) returns integer language internal;

create function pg_catalog.bit_length(bit) returns integer language internal;

create function pg_catalog.bit_length(bytea) returns integer language internal;

create function pg_catalog.bit_or(bigint) returns bigint language internal;

create function pg_catalog.bit_or(integer) returns integer language internal;

create function pg_catalog.bit_or(smallint) returns smallint language internal;

create function pg_catalog.bit_or(bit) returns bit language internal;

create function pg_catalog.bit_out(bit) returns cstring language internal;

create function pg_catalog.bit_recv(internal, oid, integer) returns bit language internal;

create function pg_catalog.bit_send(bit) returns bytea language internal;

create function pg_catalog.bit_xor(integer) returns integer language internal;

create function pg_catalog.bit_xor(bigint) returns bigint language internal;

create function pg_catalog.bit_xor(smallint) returns smallint language internal;

create function pg_catalog.bit_xor(bit) returns bit language internal;

create function pg_catalog.bitand(bit, bit) returns bit language internal;

create function pg_catalog.bitcat(bit varying, bit varying) returns bit varying language internal;

create function pg_catalog.bitcmp(bit, bit) returns integer language internal;

create function pg_catalog.biteq(bit, bit) returns boolean language internal;

create function pg_catalog.bitge(bit, bit) returns boolean language internal;

create function pg_catalog.bitgt(bit, bit) returns boolean language internal;

create function pg_catalog.bitle(bit, bit) returns boolean language internal;

create function pg_catalog.bitlt(bit, bit) returns boolean language internal;

create function pg_catalog.bitne(bit, bit) returns boolean language internal;

create function pg_catalog.bitnot(bit) returns bit language internal;

create function pg_catalog.bitor(bit, bit) returns bit language internal;

create function pg_catalog.bitshiftleft(bit, integer) returns bit language internal;

create function pg_catalog.bitshiftright(bit, integer) returns bit language internal;

create function pg_catalog.bittypmodin(cstring[]) returns integer language internal;

create function pg_catalog.bittypmodout(integer) returns cstring language internal;

create function pg_catalog.bitxor(bit, bit) returns bit language internal;

create function pg_catalog.bool(jsonb) returns boolean language internal;

create function pg_catalog.bool(integer) returns boolean language internal;

create function pg_catalog.bool_accum(internal, boolean) returns internal language internal;

create function pg_catalog.bool_accum_inv(internal, boolean) returns internal language internal;

create function pg_catalog.bool_alltrue(internal) returns boolean language internal;

create function pg_catalog.bool_and(boolean) returns boolean language internal;

create function pg_catalog.bool_anytrue(internal) returns boolean language internal;

create function pg_catalog.bool_or(boolean) returns boolean language internal;

create function pg_catalog.booland_statefunc(boolean, boolean) returns boolean language internal;

create function pg_catalog.booleq(boolean, boolean) returns boolean language internal;

create function pg_catalog.boolge(boolean, boolean) returns boolean language internal;

create function pg_catalog.boolgt(boolean, boolean) returns boolean language internal;

create function pg_catalog.boolin(cstring) returns boolean language internal;

create function pg_catalog.boolle(boolean, boolean) returns boolean language internal;

create function pg_catalog.boollt(boolean, boolean) returns boolean language internal;

create function pg_catalog.boolne(boolean, boolean) returns boolean language internal;

create function pg_catalog.boolor_statefunc(boolean, boolean) returns boolean language internal;

create function pg_catalog.boolout(boolean) returns cstring language internal;

create function pg_catalog.boolrecv(internal) returns boolean language internal;

create function pg_catalog.boolsend(boolean) returns bytea language internal;

create function pg_catalog.bound_box(box, box) returns box language internal;

create function pg_catalog.box(circle) returns box language internal;

create function pg_catalog.box(polygon) returns box language internal;

create function pg_catalog.box(point) returns box language internal;

create function pg_catalog.box(point, point) returns box language internal;

create function pg_catalog.box_above(box, box) returns boolean language internal;

create function pg_catalog.box_above_eq(box, box) returns boolean language internal;

create function pg_catalog.box_add(box, point) returns box language internal;

create function pg_catalog.box_below(box, box) returns boolean language internal;

create function pg_catalog.box_below_eq(box, box) returns boolean language internal;

create function pg_catalog.box_center(box) returns point language internal;

create function pg_catalog.box_contain(box, box) returns boolean language internal;

create function pg_catalog.box_contain_pt(box, point) returns boolean language internal;

create function pg_catalog.box_contained(box, box) returns boolean language internal;

create function pg_catalog.box_distance(box, box) returns double precision language internal;

create function pg_catalog.box_div(box, point) returns box language internal;

create function pg_catalog.box_eq(box, box) returns boolean language internal;

create function pg_catalog.box_ge(box, box) returns boolean language internal;

create function pg_catalog.box_gt(box, box) returns boolean language internal;

create function pg_catalog.box_in(cstring) returns box language internal;

create function pg_catalog.box_intersect(box, box) returns box language internal;

create function pg_catalog.box_le(box, box) returns boolean language internal;

create function pg_catalog.box_left(box, box) returns boolean language internal;

create function pg_catalog.box_lt(box, box) returns boolean language internal;

create function pg_catalog.box_mul(box, point) returns box language internal;

create function pg_catalog.box_out(box) returns cstring language internal;

create function pg_catalog.box_overabove(box, box) returns boolean language internal;

create function pg_catalog.box_overbelow(box, box) returns boolean language internal;

create function pg_catalog.box_overlap(box, box) returns boolean language internal;

create function pg_catalog.box_overleft(box, box) returns boolean language internal;

create function pg_catalog.box_overright(box, box) returns boolean language internal;

create function pg_catalog.box_recv(internal) returns box language internal;

create function pg_catalog.box_right(box, box) returns boolean language internal;

create function pg_catalog.box_same(box, box) returns boolean language internal;

create function pg_catalog.box_send(box) returns bytea language internal;

create function pg_catalog.box_sub(box, point) returns box language internal;

create function pg_catalog.bpchar("char") returns character language internal;

create function pg_catalog.bpchar(name) returns character language internal;

create function pg_catalog.bpchar(character, integer, boolean) returns character language internal;

create function pg_catalog.bpchar_larger(character, character) returns character language internal;

create function pg_catalog.bpchar_pattern_ge(character, character) returns boolean language internal;

create function pg_catalog.bpchar_pattern_gt(character, character) returns boolean language internal;

create function pg_catalog.bpchar_pattern_le(character, character) returns boolean language internal;

create function pg_catalog.bpchar_pattern_lt(character, character) returns boolean language internal;

create function pg_catalog.bpchar_smaller(character, character) returns character language internal;

create function pg_catalog.bpchar_sortsupport(internal) returns void language internal;

create function pg_catalog.bpcharcmp(character, character) returns integer language internal;

create function pg_catalog.bpchareq(character, character) returns boolean language internal;

create function pg_catalog.bpcharge(character, character) returns boolean language internal;

create function pg_catalog.bpchargt(character, character) returns boolean language internal;

create function pg_catalog.bpchariclike(character, text) returns boolean language internal;

create function pg_catalog.bpcharicnlike(character, text) returns boolean language internal;

create function pg_catalog.bpcharicregexeq(character, text) returns boolean language internal;

create function pg_catalog.bpcharicregexne(character, text) returns boolean language internal;

create function pg_catalog.bpcharin(cstring, oid, integer) returns character language internal;

create function pg_catalog.bpcharle(character, character) returns boolean language internal;

create function pg_catalog.bpcharlike(character, text) returns boolean language internal;

create function pg_catalog.bpcharlt(character, character) returns boolean language internal;

create function pg_catalog.bpcharne(character, character) returns boolean language internal;

create function pg_catalog.bpcharnlike(character, text) returns boolean language internal;

create function pg_catalog.bpcharout(character) returns cstring language internal;

create function pg_catalog.bpcharrecv(internal, oid, integer) returns character language internal;

create function pg_catalog.bpcharregexeq(character, text) returns boolean language internal;

create function pg_catalog.bpcharregexne(character, text) returns boolean language internal;

create function pg_catalog.bpcharsend(character) returns bytea language internal;

create function pg_catalog.bpchartypmodin(cstring[]) returns integer language internal;

create function pg_catalog.bpchartypmodout(integer) returns cstring language internal;

create function pg_catalog.brin_bloom_add_value(internal, internal, internal, internal) returns boolean language internal;

create function pg_catalog.brin_bloom_consistent(internal, internal, internal, integer) returns boolean language internal;

create function pg_catalog.brin_bloom_opcinfo(internal) returns internal language internal;

create function pg_catalog.brin_bloom_options(internal) returns void language internal;

create function pg_catalog.brin_bloom_summary_in(cstring) returns pg_brin_bloom_summary language internal;

create function pg_catalog.brin_bloom_summary_out(pg_brin_bloom_summary) returns cstring language internal;

create function pg_catalog.brin_bloom_summary_recv(internal) returns pg_brin_bloom_summary language internal;

create function pg_catalog.brin_bloom_summary_send(pg_brin_bloom_summary) returns bytea language internal;

create function pg_catalog.brin_bloom_union(internal, internal, internal) returns boolean language internal;

create function pg_catalog.brin_desummarize_range(regclass, bigint) returns void language internal;

create function pg_catalog.brin_inclusion_add_value(internal, internal, internal, internal) returns boolean language internal;

create function pg_catalog.brin_inclusion_consistent(internal, internal, internal) returns boolean language internal;

create function pg_catalog.brin_inclusion_opcinfo(internal) returns internal language internal;

create function pg_catalog.brin_inclusion_union(internal, internal, internal) returns boolean language internal;

create function pg_catalog.brin_minmax_add_value(internal, internal, internal, internal) returns boolean language internal;

create function pg_catalog.brin_minmax_consistent(internal, internal, internal) returns boolean language internal;

create function pg_catalog.brin_minmax_multi_add_value(internal, internal, internal, internal) returns boolean language internal;

create function pg_catalog.brin_minmax_multi_consistent(internal, internal, internal, integer) returns boolean language internal;

create function pg_catalog.brin_minmax_multi_distance_date(internal, internal) returns double precision language internal;

create function pg_catalog.brin_minmax_multi_distance_float4(internal, internal) returns double precision language internal;

create function pg_catalog.brin_minmax_multi_distance_float8(internal, internal) returns double precision language internal;

create function pg_catalog.brin_minmax_multi_distance_inet(internal, internal) returns double precision language internal;

create function pg_catalog.brin_minmax_multi_distance_int2(internal, internal) returns double precision language internal;

create function pg_catalog.brin_minmax_multi_distance_int4(internal, internal) returns double precision language internal;

create function pg_catalog.brin_minmax_multi_distance_int8(internal, internal) returns double precision language internal;

create function pg_catalog.brin_minmax_multi_distance_interval(internal, internal) returns double precision language internal;

create function pg_catalog.brin_minmax_multi_distance_macaddr(internal, internal) returns double precision language internal;

create function pg_catalog.brin_minmax_multi_distance_macaddr8(internal, internal) returns double precision language internal;

create function pg_catalog.brin_minmax_multi_distance_numeric(internal, internal) returns double precision language internal;

create function pg_catalog.brin_minmax_multi_distance_pg_lsn(internal, internal) returns double precision language internal;

create function pg_catalog.brin_minmax_multi_distance_tid(internal, internal) returns double precision language internal;

create function pg_catalog.brin_minmax_multi_distance_time(internal, internal) returns double precision language internal;

create function pg_catalog.brin_minmax_multi_distance_timestamp(internal, internal) returns double precision language internal;

create function pg_catalog.brin_minmax_multi_distance_timetz(internal, internal) returns double precision language internal;

create function pg_catalog.brin_minmax_multi_distance_uuid(internal, internal) returns double precision language internal;

create function pg_catalog.brin_minmax_multi_opcinfo(internal) returns internal language internal;

create function pg_catalog.brin_minmax_multi_options(internal) returns void language internal;

create function pg_catalog.brin_minmax_multi_summary_in(cstring) returns pg_brin_minmax_multi_summary language internal;

create function pg_catalog.brin_minmax_multi_summary_out(pg_brin_minmax_multi_summary) returns cstring language internal;

create function pg_catalog.brin_minmax_multi_summary_recv(internal) returns pg_brin_minmax_multi_summary language internal;

create function pg_catalog.brin_minmax_multi_summary_send(pg_brin_minmax_multi_summary) returns bytea language internal;

create function pg_catalog.brin_minmax_multi_union(internal, internal, internal) returns boolean language internal;

create function pg_catalog.brin_minmax_opcinfo(internal) returns internal language internal;

create function pg_catalog.brin_minmax_union(internal, internal, internal) returns boolean language internal;

create function pg_catalog.brin_summarize_new_values(regclass) returns integer language internal;

create function pg_catalog.brin_summarize_range(regclass, bigint) returns integer language internal;

create function pg_catalog.brinhandler(internal) returns index_am_handler language internal;

create function pg_catalog.broadcast(inet) returns inet language internal;

create function pg_catalog.btarraycmp(anyarray, anyarray) returns integer language internal;

create function pg_catalog.btboolcmp(boolean, boolean) returns integer language internal;

create function pg_catalog.btboolskipsupport(internal) returns void language internal;

create function pg_catalog.btbpchar_pattern_cmp(character, character) returns integer language internal;

create function pg_catalog.btbpchar_pattern_sortsupport(internal) returns void language internal;

create function pg_catalog.btcharcmp("char", "char") returns integer language internal;

create function pg_catalog.btcharskipsupport(internal) returns void language internal;

create function pg_catalog.btequalimage(oid) returns boolean language internal;

create function pg_catalog.btfloat48cmp(real, double precision) returns integer language internal;

create function pg_catalog.btfloat4cmp(real, real) returns integer language internal;

create function pg_catalog.btfloat4sortsupport(internal) returns void language internal;

create function pg_catalog.btfloat84cmp(double precision, real) returns integer language internal;

create function pg_catalog.btfloat8cmp(double precision, double precision) returns integer language internal;

create function pg_catalog.btfloat8sortsupport(internal) returns void language internal;

create function pg_catalog.bthandler(internal) returns index_am_handler language internal;

create function pg_catalog.btint24cmp(smallint, integer) returns integer language internal;

create function pg_catalog.btint28cmp(smallint, bigint) returns integer language internal;

create function pg_catalog.btint2cmp(smallint, smallint) returns integer language internal;

create function pg_catalog.btint2skipsupport(internal) returns void language internal;

create function pg_catalog.btint2sortsupport(internal) returns void language internal;

create function pg_catalog.btint42cmp(integer, smallint) returns integer language internal;

create function pg_catalog.btint48cmp(integer, bigint) returns integer language internal;

create function pg_catalog.btint4cmp(integer, integer) returns integer language internal;

create function pg_catalog.btint4skipsupport(internal) returns void language internal;

create function pg_catalog.btint4sortsupport(internal) returns void language internal;

create function pg_catalog.btint82cmp(bigint, smallint) returns integer language internal;

create function pg_catalog.btint84cmp(bigint, integer) returns integer language internal;

create function pg_catalog.btint8cmp(bigint, bigint) returns integer language internal;

create function pg_catalog.btint8skipsupport(internal) returns void language internal;

create function pg_catalog.btint8sortsupport(internal) returns void language internal;

create function pg_catalog.btnamecmp(name, name) returns integer language internal;

create function pg_catalog.btnamesortsupport(internal) returns void language internal;

create function pg_catalog.btnametextcmp(name, text) returns integer language internal;

create function pg_catalog.btoidcmp(oid, oid) returns integer language internal;

create function pg_catalog.btoidskipsupport(internal) returns void language internal;

create function pg_catalog.btoidsortsupport(internal) returns void language internal;

create function pg_catalog.btoidvectorcmp(oidvector, oidvector) returns integer language internal;

create function pg_catalog.btrecordcmp(record, record) returns integer language internal;

create function pg_catalog.btrecordimagecmp(record, record) returns integer language internal;

create function pg_catalog.btrim(bytea, bytea) returns bytea language internal;

create function pg_catalog.btrim(text, text) returns text language internal;

create function pg_catalog.btrim(text) returns text language internal;

create function pg_catalog.bttext_pattern_cmp(text, text) returns integer language internal;

create function pg_catalog.bttext_pattern_sortsupport(internal) returns void language internal;

create function pg_catalog.bttextcmp(text, text) returns integer language internal;

create function pg_catalog.bttextnamecmp(text, name) returns integer language internal;

create function pg_catalog.bttextsortsupport(internal) returns void language internal;

create function pg_catalog.bttidcmp(tid, tid) returns integer language internal;

create function pg_catalog.btvarstrequalimage(oid) returns boolean language internal;

create function pg_catalog.bytea(integer) returns bytea language internal;

create function pg_catalog.bytea(smallint) returns bytea language internal;

create function pg_catalog.bytea(bigint) returns bytea language internal;

create function pg_catalog.bytea_larger(bytea, bytea) returns bytea language internal;

create function pg_catalog.bytea_smaller(bytea, bytea) returns bytea language internal;

create function pg_catalog.bytea_sortsupport(internal) returns void language internal;

create function pg_catalog.bytea_string_agg_finalfn(internal) returns bytea language internal;

create function pg_catalog.bytea_string_agg_transfn(internal, bytea, bytea) returns internal language internal;

create function pg_catalog.byteacat(bytea, bytea) returns bytea language internal;

create function pg_catalog.byteacmp(bytea, bytea) returns integer language internal;

create function pg_catalog.byteaeq(bytea, bytea) returns boolean language internal;

create function pg_catalog.byteage(bytea, bytea) returns boolean language internal;

create function pg_catalog.byteagt(bytea, bytea) returns boolean language internal;

create function pg_catalog.byteain(cstring) returns bytea language internal;

create function pg_catalog.byteale(bytea, bytea) returns boolean language internal;

create function pg_catalog.bytealike(bytea, bytea) returns boolean language internal;

create function pg_catalog.bytealt(bytea, bytea) returns boolean language internal;

create function pg_catalog.byteane(bytea, bytea) returns boolean language internal;

create function pg_catalog.byteanlike(bytea, bytea) returns boolean language internal;

create function pg_catalog.byteaout(bytea) returns cstring language internal;

create function pg_catalog.bytearecv(internal) returns bytea language internal;

create function pg_catalog.byteasend(bytea) returns bytea language internal;

create function pg_catalog.cardinality(anyarray) returns integer language internal;

create function pg_catalog.casefold(text) returns text language internal;

create function pg_catalog.cash_cmp(money, money) returns integer language internal;

create function pg_catalog.cash_div_cash(money, money) returns double precision language internal;

create function pg_catalog.cash_div_flt4(money, real) returns money language internal;

create function pg_catalog.cash_div_flt8(money, double precision) returns money language internal;

create function pg_catalog.cash_div_int2(money, smallint) returns money language internal;

create function pg_catalog.cash_div_int4(money, integer) returns money language internal;

create function pg_catalog.cash_div_int8(money, bigint) returns money language internal;

create function pg_catalog.cash_eq(money, money) returns boolean language internal;

create function pg_catalog.cash_ge(money, money) returns boolean language internal;

create function pg_catalog.cash_gt(money, money) returns boolean language internal;

create function pg_catalog.cash_in(cstring) returns money language internal;

create function pg_catalog.cash_le(money, money) returns boolean language internal;

create function pg_catalog.cash_lt(money, money) returns boolean language internal;

create function pg_catalog.cash_mi(money, money) returns money language internal;

create function pg_catalog.cash_mul_flt4(money, real) returns money language internal;

create function pg_catalog.cash_mul_flt8(money, double precision) returns money language internal;

create function pg_catalog.cash_mul_int2(money, smallint) returns money language internal;

create function pg_catalog.cash_mul_int4(money, integer) returns money language internal;

create function pg_catalog.cash_mul_int8(money, bigint) returns money language internal;

create function pg_catalog.cash_ne(money, money) returns boolean language internal;

create function pg_catalog.cash_out(money) returns cstring language internal;

create function pg_catalog.cash_pl(money, money) returns money language internal;

create function pg_catalog.cash_recv(internal) returns money language internal;

create function pg_catalog.cash_send(money) returns bytea language internal;

create function pg_catalog.cash_words(money) returns text language internal;

create function pg_catalog.cashlarger(money, money) returns money language internal;

create function pg_catalog.cashsmaller(money, money) returns money language internal;

create function pg_catalog.cbrt(double precision) returns double precision language internal;

create function pg_catalog.ceil(double precision) returns double precision language internal;

create function pg_catalog.ceil(numeric) returns numeric language internal;

create function pg_catalog.ceiling(double precision) returns double precision language internal;

create function pg_catalog.ceiling(numeric) returns numeric language internal;

create function pg_catalog.center(circle) returns point language internal;

create function pg_catalog.center(box) returns point language internal;

create function pg_catalog.char(text) returns "char" language internal;

create function pg_catalog.char(integer) returns "char" language internal;

create function pg_catalog.char_length(text) returns integer language internal;

create function pg_catalog.char_length(character) returns integer language internal;

create function pg_catalog.character_length(text) returns integer language internal;

create function pg_catalog.character_length(character) returns integer language internal;

create function pg_catalog.chareq("char", "char") returns boolean language internal;

create function pg_catalog.charge("char", "char") returns boolean language internal;

create function pg_catalog.chargt("char", "char") returns boolean language internal;

create function pg_catalog.charin(cstring) returns "char" language internal;

create function pg_catalog.charle("char", "char") returns boolean language internal;

create function pg_catalog.charlt("char", "char") returns boolean language internal;

create function pg_catalog.charne("char", "char") returns boolean language internal;

create function pg_catalog.charout("char") returns cstring language internal;

create function pg_catalog.charrecv(internal) returns "char" language internal;

create function pg_catalog.charsend("char") returns bytea language internal;

create function pg_catalog.chr(integer) returns text language internal;

create function pg_catalog.cideq(cid, cid) returns boolean language internal;

create function pg_catalog.cidin(cstring) returns cid language internal;

create function pg_catalog.cidout(cid) returns cstring language internal;

create function pg_catalog.cidr(inet) returns cidr language internal;

create function pg_catalog.cidr_in(cstring) returns cidr language internal;

create function pg_catalog.cidr_out(cidr) returns cstring language internal;

create function pg_catalog.cidr_recv(internal) returns cidr language internal;

create function pg_catalog.cidr_send(cidr) returns bytea language internal;

create function pg_catalog.cidrecv(internal) returns cid language internal;

create function pg_catalog.cidsend(cid) returns bytea language internal;

create function pg_catalog.circle(point, double precision) returns circle language internal;

create function pg_catalog.circle(polygon) returns circle language internal;

create function pg_catalog.circle(box) returns circle language internal;

create function pg_catalog.circle_above(circle, circle) returns boolean language internal;

create function pg_catalog.circle_add_pt(circle, point) returns circle language internal;

create function pg_catalog.circle_below(circle, circle) returns boolean language internal;

create function pg_catalog.circle_center(circle) returns point language internal;

create function pg_catalog.circle_contain(circle, circle) returns boolean language internal;

create function pg_catalog.circle_contain_pt(circle, point) returns boolean language internal;

create function pg_catalog.circle_contained(circle, circle) returns boolean language internal;

create function pg_catalog.circle_distance(circle, circle) returns double precision language internal;

create function pg_catalog.circle_div_pt(circle, point) returns circle language internal;

create function pg_catalog.circle_eq(circle, circle) returns boolean language internal;

create function pg_catalog.circle_ge(circle, circle) returns boolean language internal;

create function pg_catalog.circle_gt(circle, circle) returns boolean language internal;

create function pg_catalog.circle_in(cstring) returns circle language internal;

create function pg_catalog.circle_le(circle, circle) returns boolean language internal;

create function pg_catalog.circle_left(circle, circle) returns boolean language internal;

create function pg_catalog.circle_lt(circle, circle) returns boolean language internal;

create function pg_catalog.circle_mul_pt(circle, point) returns circle language internal;

create function pg_catalog.circle_ne(circle, circle) returns boolean language internal;

create function pg_catalog.circle_out(circle) returns cstring language internal;

create function pg_catalog.circle_overabove(circle, circle) returns boolean language internal;

create function pg_catalog.circle_overbelow(circle, circle) returns boolean language internal;

create function pg_catalog.circle_overlap(circle, circle) returns boolean language internal;

create function pg_catalog.circle_overleft(circle, circle) returns boolean language internal;

create function pg_catalog.circle_overright(circle, circle) returns boolean language internal;

create function pg_catalog.circle_recv(internal) returns circle language internal;

create function pg_catalog.circle_right(circle, circle) returns boolean language internal;

create function pg_catalog.circle_same(circle, circle) returns boolean language internal;

create function pg_catalog.circle_send(circle) returns bytea language internal;

create function pg_catalog.circle_sub_pt(circle, point) returns circle language internal;

create function pg_catalog.clock_timestamp() returns timestamp with time zone language internal;

create function pg_catalog.close_ls(line, lseg) returns point language internal;

create function pg_catalog.close_lseg(lseg, lseg) returns point language internal;

create function pg_catalog.close_pb(point, box) returns point language internal;

create function pg_catalog.close_pl(point, line) returns point language internal;

create function pg_catalog.close_ps(point, lseg) returns point language internal;

create function pg_catalog.close_sb(lseg, box) returns point language internal;

create function pg_catalog.col_description(oid, integer) returns text language internal;

create function pg_catalog.concat(VARIADIC "any") returns text language internal;

create function pg_catalog.concat_ws(text, VARIADIC "any") returns text language internal;

create function pg_catalog.contjoinsel(internal, oid, internal, smallint, internal) returns double precision language internal;

create function pg_catalog.contsel(internal, oid, internal, integer) returns double precision language internal;

create function pg_catalog.convert(bytea, name, name) returns bytea language internal;

create function pg_catalog.convert_from(bytea, name) returns text language internal;

create function pg_catalog.convert_to(text, name) returns bytea language internal;

create function pg_catalog.corr(double precision, double precision) returns double precision language internal;

create function pg_catalog.cos(double precision) returns double precision language internal;

create function pg_catalog.cosd(double precision) returns double precision language internal;

create function pg_catalog.cosh(double precision) returns double precision language internal;

create function pg_catalog.cot(double precision) returns double precision language internal;

create function pg_catalog.cotd(double precision) returns double precision language internal;

create function pg_catalog.count("any") returns bigint language internal;

create function pg_catalog.count() returns bigint language internal;

create function pg_catalog.covar_pop(double precision, double precision) returns double precision language internal;

create function pg_catalog.covar_samp(double precision, double precision) returns double precision language internal;

create function pg_catalog.crc32(bytea) returns bigint language internal;

create function pg_catalog.crc32c(bytea) returns bigint language internal;

create function pg_catalog.cstring_in(cstring) returns cstring language internal;

create function pg_catalog.cstring_out(cstring) returns cstring language internal;

create function pg_catalog.cstring_recv(internal) returns cstring language internal;

create function pg_catalog.cstring_send(cstring) returns bytea language internal;

create function pg_catalog.cume_dist() returns double precision language internal;

create function pg_catalog.cume_dist_final(internal, VARIADIC "any") returns double precision language internal;

create function pg_catalog.current_database() returns name language internal;

create function pg_catalog.current_query() returns text language internal;

create function pg_catalog.current_schema() returns name language internal;

create function pg_catalog.current_schemas(boolean) returns name[] language internal;

create function pg_catalog.current_setting(text) returns text language internal;

create function pg_catalog.current_setting(text, boolean) returns text language internal;

create function pg_catalog.current_user() returns name language internal;

create function pg_catalog.currtid2(text, tid) returns tid language internal;

create function pg_catalog.currval(regclass) returns bigint language internal;

create function pg_catalog.cursor_to_xml(cursor refcursor, count integer, nulls boolean, tableforest boolean, targetns text) returns xml language internal;

create function pg_catalog.cursor_to_xmlschema(cursor refcursor, nulls boolean, tableforest boolean, targetns text) returns xml language internal;

create function pg_catalog.database_to_xml(nulls boolean, tableforest boolean, targetns text) returns xml language internal;

create function pg_catalog.database_to_xml_and_xmlschema(nulls boolean, tableforest boolean, targetns text) returns xml language internal;

create function pg_catalog.database_to_xmlschema(nulls boolean, tableforest boolean, targetns text) returns xml language internal;

create function pg_catalog.date(timestamp without time zone) returns date language internal;

create function pg_catalog.date(timestamp with time zone) returns date language internal;

create function pg_catalog.date_add(timestamp with time zone, interval) returns timestamp with time zone language internal;

create function pg_catalog.date_add(timestamp with time zone, interval, text) returns timestamp with time zone language internal;

create function pg_catalog.date_bin(interval, timestamp without time zone, timestamp without time zone) returns timestamp without time zone language internal;

create function pg_catalog.date_bin(interval, timestamp with time zone, timestamp with time zone) returns timestamp with time zone language internal;

create function pg_catalog.date_cmp(date, date) returns integer language internal;

create function pg_catalog.date_cmp_timestamp(date, timestamp without time zone) returns integer language internal;

create function pg_catalog.date_cmp_timestamptz(date, timestamp with time zone) returns integer language internal;

create function pg_catalog.date_eq(date, date) returns boolean language internal;

create function pg_catalog.date_eq_timestamp(date, timestamp without time zone) returns boolean language internal;

create function pg_catalog.date_eq_timestamptz(date, timestamp with time zone) returns boolean language internal;

create function pg_catalog.date_ge(date, date) returns boolean language internal;

create function pg_catalog.date_ge_timestamp(date, timestamp without time zone) returns boolean language internal;

create function pg_catalog.date_ge_timestamptz(date, timestamp with time zone) returns boolean language internal;

create function pg_catalog.date_gt(date, date) returns boolean language internal;

create function pg_catalog.date_gt_timestamp(date, timestamp without time zone) returns boolean language internal;

create function pg_catalog.date_gt_timestamptz(date, timestamp with time zone) returns boolean language internal;

create function pg_catalog.date_in(cstring) returns date language internal;

create function pg_catalog.date_larger(date, date) returns date language internal;

create function pg_catalog.date_le(date, date) returns boolean language internal;

create function pg_catalog.date_le_timestamp(date, timestamp without time zone) returns boolean language internal;

create function pg_catalog.date_le_timestamptz(date, timestamp with time zone) returns boolean language internal;

create function pg_catalog.date_lt(date, date) returns boolean language internal;

create function pg_catalog.date_lt_timestamp(date, timestamp without time zone) returns boolean language internal;

create function pg_catalog.date_lt_timestamptz(date, timestamp with time zone) returns boolean language internal;

create function pg_catalog.date_mi(date, date) returns integer language internal;

create function pg_catalog.date_mi_interval(date, interval) returns timestamp without time zone language internal;

create function pg_catalog.date_mii(date, integer) returns date language internal;

create function pg_catalog.date_ne(date, date) returns boolean language internal;

create function pg_catalog.date_ne_timestamp(date, timestamp without time zone) returns boolean language internal;

create function pg_catalog.date_ne_timestamptz(date, timestamp with time zone) returns boolean language internal;

create function pg_catalog.date_out(date) returns cstring language internal;

create function pg_catalog.date_part(text, date) returns double precision language internal;

create function pg_catalog.date_part(text, time with time zone) returns double precision language internal;

create function pg_catalog.date_part(text, interval) returns double precision language internal;

create function pg_catalog.date_part(text, timestamp without time zone) returns double precision language internal;

create function pg_catalog.date_part(text, timestamp with time zone) returns double precision language internal;

create function pg_catalog.date_part(text, time without time zone) returns double precision language internal;

create function pg_catalog.date_pl_interval(date, interval) returns timestamp without time zone language internal;

create function pg_catalog.date_pli(date, integer) returns date language internal;

create function pg_catalog.date_recv(internal) returns date language internal;

create function pg_catalog.date_send(date) returns bytea language internal;

create function pg_catalog.date_skipsupport(internal) returns void language internal;

create function pg_catalog.date_smaller(date, date) returns date language internal;

create function pg_catalog.date_sortsupport(internal) returns void language internal;

create function pg_catalog.date_subtract(timestamp with time zone, interval, text) returns timestamp with time zone language internal;

create function pg_catalog.date_subtract(timestamp with time zone, interval) returns timestamp with time zone language internal;

create function pg_catalog.date_trunc(text, interval) returns interval language internal;

create function pg_catalog.date_trunc(text, timestamp with time zone, text) returns timestamp with time zone language internal;

create function pg_catalog.date_trunc(text, timestamp with time zone) returns timestamp with time zone language internal;

create function pg_catalog.date_trunc(text, timestamp without time zone) returns timestamp without time zone language internal;

create function pg_catalog.datemultirange(VARIADIC daterange[]) returns datemultirange language internal;

create function pg_catalog.datemultirange() returns datemultirange language internal;

create function pg_catalog.datemultirange(daterange) returns datemultirange language internal;

create function pg_catalog.daterange(date, date) returns daterange language internal;

create function pg_catalog.daterange(date, date, text) returns daterange language internal;

create function pg_catalog.daterange_canonical(daterange) returns daterange language internal;

create function pg_catalog.daterange_subdiff(date, date) returns double precision language internal;

create function pg_catalog.datetime_pl(date, time without time zone) returns timestamp without time zone language internal;

create function pg_catalog.datetimetz_pl(date, time with time zone) returns timestamp with time zone language internal;

create function pg_catalog.dcbrt(double precision) returns double precision language internal;

create function pg_catalog.decode(text, text) returns bytea language internal;

create function pg_catalog.degrees(double precision) returns double precision language internal;

create function pg_catalog.dense_rank() returns bigint language internal;

create function pg_catalog.dense_rank_final(internal, VARIADIC "any") returns bigint language internal;

create function pg_catalog.dexp(double precision) returns double precision language internal;

create function pg_catalog.diagonal(box) returns lseg language internal;

create function pg_catalog.diameter(circle) returns double precision language internal;

create function pg_catalog.dispell_init(internal) returns internal language internal;

create function pg_catalog.dispell_lexize(internal, internal, internal, internal) returns internal language internal;

create function pg_catalog.dist_bp(box, point) returns double precision language internal;

create function pg_catalog.dist_bs(box, lseg) returns double precision language internal;

create function pg_catalog.dist_cpoint(circle, point) returns double precision language internal;

create function pg_catalog.dist_cpoly(circle, polygon) returns double precision language internal;

create function pg_catalog.dist_lp(line, point) returns double precision language internal;

create function pg_catalog.dist_ls(line, lseg) returns double precision language internal;

create function pg_catalog.dist_pathp(path, point) returns double precision language internal;

create function pg_catalog.dist_pb(point, box) returns double precision language internal;

create function pg_catalog.dist_pc(point, circle) returns double precision language internal;

create function pg_catalog.dist_pl(point, line) returns double precision language internal;

create function pg_catalog.dist_polyc(polygon, circle) returns double precision language internal;

create function pg_catalog.dist_polyp(polygon, point) returns double precision language internal;

create function pg_catalog.dist_ppath(point, path) returns double precision language internal;

create function pg_catalog.dist_ppoly(point, polygon) returns double precision language internal;

create function pg_catalog.dist_ps(point, lseg) returns double precision language internal;

create function pg_catalog.dist_sb(lseg, box) returns double precision language internal;

create function pg_catalog.dist_sl(lseg, line) returns double precision language internal;

create function pg_catalog.dist_sp(lseg, point) returns double precision language internal;

create function pg_catalog.div(numeric, numeric) returns numeric language internal;

create function pg_catalog.dlog1(double precision) returns double precision language internal;

create function pg_catalog.dlog10(double precision) returns double precision language internal;

create function pg_catalog.domain_in(cstring, oid, integer) returns "any" language internal;

create function pg_catalog.domain_recv(internal, oid, integer) returns "any" language internal;

create function pg_catalog.dpow(double precision, double precision) returns double precision language internal;

create function pg_catalog.dround(double precision) returns double precision language internal;

create function pg_catalog.dsimple_init(internal) returns internal language internal;

create function pg_catalog.dsimple_lexize(internal, internal, internal, internal) returns internal language internal;

create function pg_catalog.dsnowball_init(internal) returns internal language internal;

create function pg_catalog.dsnowball_lexize(internal, internal, internal, internal) returns internal language internal;

create function pg_catalog.dsqrt(double precision) returns double precision language internal;

create function pg_catalog.dsynonym_init(internal) returns internal language internal;

create function pg_catalog.dsynonym_lexize(internal, internal, internal, internal) returns internal language internal;

create function pg_catalog.dtrunc(double precision) returns double precision language internal;

create function pg_catalog.elem_contained_by_multirange(anyelement, anymultirange) returns boolean language internal;

create function pg_catalog.elem_contained_by_range(anyelement, anyrange) returns boolean language internal;

create function pg_catalog.elem_contained_by_range_support(internal) returns internal language internal;

create function pg_catalog.encode(bytea, text) returns text language internal;

create function pg_catalog.enum_cmp(anyenum, anyenum) returns integer language internal;

create function pg_catalog.enum_eq(anyenum, anyenum) returns boolean language internal;

create function pg_catalog.enum_first(anyenum) returns anyenum language internal;

create function pg_catalog.enum_ge(anyenum, anyenum) returns boolean language internal;

create function pg_catalog.enum_gt(anyenum, anyenum) returns boolean language internal;

create function pg_catalog.enum_in(cstring, oid) returns anyenum language internal;

create function pg_catalog.enum_larger(anyenum, anyenum) returns anyenum language internal;

create function pg_catalog.enum_last(anyenum) returns anyenum language internal;

create function pg_catalog.enum_le(anyenum, anyenum) returns boolean language internal;

create function pg_catalog.enum_lt(anyenum, anyenum) returns boolean language internal;

create function pg_catalog.enum_ne(anyenum, anyenum) returns boolean language internal;

create function pg_catalog.enum_out(anyenum) returns cstring language internal;

create function pg_catalog.enum_range(anyenum) returns anyarray language internal;

create function pg_catalog.enum_range(anyenum, anyenum) returns anyarray language internal;

create function pg_catalog.enum_recv(internal, oid) returns anyenum language internal;

create function pg_catalog.enum_send(anyenum) returns bytea language internal;

create function pg_catalog.enum_smaller(anyenum, anyenum) returns anyenum language internal;

create function pg_catalog.eqjoinsel(internal, oid, internal, smallint, internal) returns double precision language internal;

create function pg_catalog.eqsel(internal, oid, internal, integer) returns double precision language internal;

create function pg_catalog.erf(double precision) returns double precision language internal;

create function pg_catalog.erfc(double precision) returns double precision language internal;

create function pg_catalog.euc_cn_to_mic(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.euc_cn_to_utf8(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.euc_jis_2004_to_shift_jis_2004(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.euc_jis_2004_to_utf8(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.euc_jp_to_mic(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.euc_jp_to_sjis(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.euc_jp_to_utf8(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.euc_kr_to_mic(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.euc_kr_to_utf8(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.euc_tw_to_big5(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.euc_tw_to_mic(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.euc_tw_to_utf8(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.event_trigger_in(cstring) returns event_trigger language internal;

create function pg_catalog.event_trigger_out(event_trigger) returns cstring language internal;

create function pg_catalog.every(boolean) returns boolean language internal;

create function pg_catalog.exp(double precision) returns double precision language internal;

create function pg_catalog.exp(numeric) returns numeric language internal;

create function pg_catalog.extract(text, time without time zone) returns numeric language internal;

create function pg_catalog.extract(text, interval) returns numeric language internal;

create function pg_catalog.extract(text, timestamp without time zone) returns numeric language internal;

create function pg_catalog.extract(text, date) returns numeric language internal;

create function pg_catalog.extract(text, time with time zone) returns numeric language internal;

create function pg_catalog.extract(text, timestamp with time zone) returns numeric language internal;

create function pg_catalog.factorial(bigint) returns numeric language internal;

create function pg_catalog.family(inet) returns integer language internal;

create function pg_catalog.fdw_handler_in(cstring) returns fdw_handler language internal;

create function pg_catalog.fdw_handler_out(fdw_handler) returns cstring language internal;

create function pg_catalog.first_value(anyelement) returns anyelement language internal;

create function pg_catalog.float4(double precision) returns real language internal;

create function pg_catalog.float4(smallint) returns real language internal;

create function pg_catalog.float4(bigint) returns real language internal;

create function pg_catalog.float4(numeric) returns real language internal;

create function pg_catalog.float4(jsonb) returns real language internal;

create function pg_catalog.float4(integer) returns real language internal;

create function pg_catalog.float48div(real, double precision) returns double precision language internal;

create function pg_catalog.float48eq(real, double precision) returns boolean language internal;

create function pg_catalog.float48ge(real, double precision) returns boolean language internal;

create function pg_catalog.float48gt(real, double precision) returns boolean language internal;

create function pg_catalog.float48le(real, double precision) returns boolean language internal;

create function pg_catalog.float48lt(real, double precision) returns boolean language internal;

create function pg_catalog.float48mi(real, double precision) returns double precision language internal;

create function pg_catalog.float48mul(real, double precision) returns double precision language internal;

create function pg_catalog.float48ne(real, double precision) returns boolean language internal;

create function pg_catalog.float48pl(real, double precision) returns double precision language internal;

create function pg_catalog.float4_accum(double precision[], real) returns double precision[] language internal;

create function pg_catalog.float4abs(real) returns real language internal;

create function pg_catalog.float4div(real, real) returns real language internal;

create function pg_catalog.float4eq(real, real) returns boolean language internal;

create function pg_catalog.float4ge(real, real) returns boolean language internal;

create function pg_catalog.float4gt(real, real) returns boolean language internal;

create function pg_catalog.float4in(cstring) returns real language internal;

create function pg_catalog.float4larger(real, real) returns real language internal;

create function pg_catalog.float4le(real, real) returns boolean language internal;

create function pg_catalog.float4lt(real, real) returns boolean language internal;

create function pg_catalog.float4mi(real, real) returns real language internal;

create function pg_catalog.float4mul(real, real) returns real language internal;

create function pg_catalog.float4ne(real, real) returns boolean language internal;

create function pg_catalog.float4out(real) returns cstring language internal;

create function pg_catalog.float4pl(real, real) returns real language internal;

create function pg_catalog.float4recv(internal) returns real language internal;

create function pg_catalog.float4send(real) returns bytea language internal;

create function pg_catalog.float4smaller(real, real) returns real language internal;

create function pg_catalog.float4um(real) returns real language internal;

create function pg_catalog.float4up(real) returns real language internal;

create function pg_catalog.float8(numeric) returns double precision language internal;

create function pg_catalog.float8(jsonb) returns double precision language internal;

create function pg_catalog.float8(bigint) returns double precision language internal;

create function pg_catalog.float8(real) returns double precision language internal;

create function pg_catalog.float8(integer) returns double precision language internal;

create function pg_catalog.float8(smallint) returns double precision language internal;

create function pg_catalog.float84div(double precision, real) returns double precision language internal;

create function pg_catalog.float84eq(double precision, real) returns boolean language internal;

create function pg_catalog.float84ge(double precision, real) returns boolean language internal;

create function pg_catalog.float84gt(double precision, real) returns boolean language internal;

create function pg_catalog.float84le(double precision, real) returns boolean language internal;

create function pg_catalog.float84lt(double precision, real) returns boolean language internal;

create function pg_catalog.float84mi(double precision, real) returns double precision language internal;

create function pg_catalog.float84mul(double precision, real) returns double precision language internal;

create function pg_catalog.float84ne(double precision, real) returns boolean language internal;

create function pg_catalog.float84pl(double precision, real) returns double precision language internal;

create function pg_catalog.float8_accum(double precision[], double precision) returns double precision[] language internal;

create function pg_catalog.float8_avg(double precision[]) returns double precision language internal;

create function pg_catalog.float8_combine(double precision[], double precision[]) returns double precision[] language internal;

create function pg_catalog.float8_corr(double precision[]) returns double precision language internal;

create function pg_catalog.float8_covar_pop(double precision[]) returns double precision language internal;

create function pg_catalog.float8_covar_samp(double precision[]) returns double precision language internal;

create function pg_catalog.float8_regr_accum(double precision[], double precision, double precision) returns double precision[] language internal;

create function pg_catalog.float8_regr_avgx(double precision[]) returns double precision language internal;

create function pg_catalog.float8_regr_avgy(double precision[]) returns double precision language internal;

create function pg_catalog.float8_regr_combine(double precision[], double precision[]) returns double precision[] language internal;

create function pg_catalog.float8_regr_intercept(double precision[]) returns double precision language internal;

create function pg_catalog.float8_regr_r2(double precision[]) returns double precision language internal;

create function pg_catalog.float8_regr_slope(double precision[]) returns double precision language internal;

create function pg_catalog.float8_regr_sxx(double precision[]) returns double precision language internal;

create function pg_catalog.float8_regr_sxy(double precision[]) returns double precision language internal;

create function pg_catalog.float8_regr_syy(double precision[]) returns double precision language internal;

create function pg_catalog.float8_stddev_pop(double precision[]) returns double precision language internal;

create function pg_catalog.float8_stddev_samp(double precision[]) returns double precision language internal;

create function pg_catalog.float8_var_pop(double precision[]) returns double precision language internal;

create function pg_catalog.float8_var_samp(double precision[]) returns double precision language internal;

create function pg_catalog.float8abs(double precision) returns double precision language internal;

create function pg_catalog.float8div(double precision, double precision) returns double precision language internal;

create function pg_catalog.float8eq(double precision, double precision) returns boolean language internal;

create function pg_catalog.float8ge(double precision, double precision) returns boolean language internal;

create function pg_catalog.float8gt(double precision, double precision) returns boolean language internal;

create function pg_catalog.float8in(cstring) returns double precision language internal;

create function pg_catalog.float8larger(double precision, double precision) returns double precision language internal;

create function pg_catalog.float8le(double precision, double precision) returns boolean language internal;

create function pg_catalog.float8lt(double precision, double precision) returns boolean language internal;

create function pg_catalog.float8mi(double precision, double precision) returns double precision language internal;

create function pg_catalog.float8mul(double precision, double precision) returns double precision language internal;

create function pg_catalog.float8ne(double precision, double precision) returns boolean language internal;

create function pg_catalog.float8out(double precision) returns cstring language internal;

create function pg_catalog.float8pl(double precision, double precision) returns double precision language internal;

create function pg_catalog.float8recv(internal) returns double precision language internal;

create function pg_catalog.float8send(double precision) returns bytea language internal;

create function pg_catalog.float8smaller(double precision, double precision) returns double precision language internal;

create function pg_catalog.float8um(double precision) returns double precision language internal;

create function pg_catalog.float8up(double precision) returns double precision language internal;

create function pg_catalog.floor(numeric) returns numeric language internal;

create function pg_catalog.floor(double precision) returns double precision language internal;

create function pg_catalog.flt4_mul_cash(real, money) returns money language internal;

create function pg_catalog.flt8_mul_cash(double precision, money) returns money language internal;

create function pg_catalog.fmgr_c_validator(oid) returns void language internal;

create function pg_catalog.fmgr_internal_validator(oid) returns void language internal;

create function pg_catalog.fmgr_sql_validator(oid) returns void language internal;

create function pg_catalog.format(text, VARIADIC "any") returns text language internal;

create function pg_catalog.format(text) returns text language internal;

create function pg_catalog.format_type(oid, integer) returns text language internal;

create function pg_catalog.gamma(double precision) returns double precision language internal;

create function pg_catalog.gb18030_to_utf8(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.gbk_to_utf8(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.gcd(bigint, bigint) returns bigint language internal;

create function pg_catalog.gcd(integer, integer) returns integer language internal;

create function pg_catalog.gcd(numeric, numeric) returns numeric language internal;

create function pg_catalog.gen_random_uuid() returns uuid language internal;

create function pg_catalog.generate_series(numeric, numeric, numeric) returns SETOF numeric language internal;

create function pg_catalog.generate_series(integer, integer, integer) returns SETOF integer language internal;

create function pg_catalog.generate_series(integer, integer) returns SETOF integer language internal;

create function pg_catalog.generate_series(bigint, bigint, bigint) returns SETOF bigint language internal;

create function pg_catalog.generate_series(numeric, numeric) returns SETOF numeric language internal;

create function pg_catalog.generate_series(bigint, bigint) returns SETOF bigint language internal;

create function pg_catalog.generate_series(timestamp with time zone, timestamp with time zone, interval, text) returns SETOF timestamp with time zone language internal;

create function pg_catalog.generate_series(timestamp with time zone, timestamp with time zone, interval) returns SETOF timestamp with time zone language internal;

create function pg_catalog.generate_series(timestamp without time zone, timestamp without time zone, interval) returns SETOF timestamp without time zone language internal;

create function pg_catalog.generate_series_int4_support(internal) returns internal language internal;

create function pg_catalog.generate_series_int8_support(internal) returns internal language internal;

create function pg_catalog.generate_series_numeric_support(internal) returns internal language internal;

create function pg_catalog.generate_series_timestamp_support(internal) returns internal language internal;

create function pg_catalog.generate_subscripts(anyarray, integer, boolean) returns SETOF integer language internal;

create function pg_catalog.generate_subscripts(anyarray, integer) returns SETOF integer language internal;

create function pg_catalog.get_bit(bit, integer) returns integer language internal;

create function pg_catalog.get_bit(bytea, bigint) returns integer language internal;

create function pg_catalog.get_byte(bytea, integer) returns integer language internal;

create function pg_catalog.get_current_ts_config() returns regconfig language internal;

create function pg_catalog.getdatabaseencoding() returns name language internal;

create function pg_catalog.getpgusername() returns name language internal;

create function pg_catalog.gin_clean_pending_list(regclass) returns bigint language internal;

create function pg_catalog.gin_cmp_prefix(text, text, smallint, internal) returns integer language internal;

create function pg_catalog.gin_cmp_tslexeme(text, text) returns integer language internal;

create function pg_catalog.gin_compare_jsonb(text, text) returns integer language internal;

create function pg_catalog.gin_consistent_jsonb(internal, smallint, jsonb, integer, internal, internal, internal, internal) returns boolean language internal;

create function pg_catalog.gin_consistent_jsonb_path(internal, smallint, jsonb, integer, internal, internal, internal, internal) returns boolean language internal;

create function pg_catalog.gin_extract_jsonb(jsonb, internal, internal) returns internal language internal;

create function pg_catalog.gin_extract_jsonb_path(jsonb, internal, internal) returns internal language internal;

create function pg_catalog.gin_extract_jsonb_query(jsonb, internal, smallint, internal, internal, internal, internal) returns internal language internal;

create function pg_catalog.gin_extract_jsonb_query_path(jsonb, internal, smallint, internal, internal, internal, internal) returns internal language internal;

create function pg_catalog.gin_extract_tsquery(tsquery, internal, smallint, internal, internal, internal, internal) returns internal language internal;

create function pg_catalog.gin_extract_tsquery(tsquery, internal, smallint, internal, internal) returns internal language internal;

create function pg_catalog.gin_extract_tsquery(tsvector, internal, smallint, internal, internal, internal, internal) returns internal language internal;

create function pg_catalog.gin_extract_tsvector(tsvector, internal, internal) returns internal language internal;

create function pg_catalog.gin_extract_tsvector(tsvector, internal) returns internal language internal;

create function pg_catalog.gin_triconsistent_jsonb(internal, smallint, jsonb, integer, internal, internal, internal) returns "char" language internal;

create function pg_catalog.gin_triconsistent_jsonb_path(internal, smallint, jsonb, integer, internal, internal, internal) returns "char" language internal;

create function pg_catalog.gin_tsquery_consistent(internal, smallint, tsquery, integer, internal, internal) returns boolean language internal;

create function pg_catalog.gin_tsquery_consistent(internal, smallint, tsvector, integer, internal, internal, internal, internal) returns boolean language internal;

create function pg_catalog.gin_tsquery_consistent(internal, smallint, tsquery, integer, internal, internal, internal, internal) returns boolean language internal;

create function pg_catalog.gin_tsquery_triconsistent(internal, smallint, tsvector, integer, internal, internal, internal) returns "char" language internal;

create function pg_catalog.ginarrayconsistent(internal, smallint, anyarray, integer, internal, internal, internal, internal) returns boolean language internal;

create function pg_catalog.ginarrayextract(anyarray, internal, internal) returns internal language internal;

create function pg_catalog.ginarrayextract(anyarray, internal) returns internal language internal;

create function pg_catalog.ginarraytriconsistent(internal, smallint, anyarray, integer, internal, internal, internal) returns "char" language internal;

create function pg_catalog.ginhandler(internal) returns index_am_handler language internal;

create function pg_catalog.ginqueryarrayextract(anyarray, internal, smallint, internal, internal, internal, internal) returns internal language internal;

create function pg_catalog.gist_box_consistent(internal, box, smallint, oid, internal) returns boolean language internal;

create function pg_catalog.gist_box_distance(internal, box, smallint, oid, internal) returns double precision language internal;

create function pg_catalog.gist_box_penalty(internal, internal, internal) returns internal language internal;

create function pg_catalog.gist_box_picksplit(internal, internal) returns internal language internal;

create function pg_catalog.gist_box_same(box, box, internal) returns internal language internal;

create function pg_catalog.gist_box_union(internal, internal) returns box language internal;

create function pg_catalog.gist_circle_compress(internal) returns internal language internal;

create function pg_catalog.gist_circle_consistent(internal, circle, smallint, oid, internal) returns boolean language internal;

create function pg_catalog.gist_circle_distance(internal, circle, smallint, oid, internal) returns double precision language internal;

create function pg_catalog.gist_point_compress(internal) returns internal language internal;

create function pg_catalog.gist_point_consistent(internal, point, smallint, oid, internal) returns boolean language internal;

create function pg_catalog.gist_point_distance(internal, point, smallint, oid, internal) returns double precision language internal;

create function pg_catalog.gist_point_fetch(internal) returns internal language internal;

create function pg_catalog.gist_point_sortsupport(internal) returns void language internal;

create function pg_catalog.gist_poly_compress(internal) returns internal language internal;

create function pg_catalog.gist_poly_consistent(internal, polygon, smallint, oid, internal) returns boolean language internal;

create function pg_catalog.gist_poly_distance(internal, polygon, smallint, oid, internal) returns double precision language internal;

create function pg_catalog.gist_translate_cmptype_common(integer) returns smallint language internal;

create function pg_catalog.gisthandler(internal) returns index_am_handler language internal;

create function pg_catalog.gtsquery_compress(internal) returns internal language internal;

create function pg_catalog.gtsquery_consistent(internal, internal, integer, oid, internal) returns boolean language internal;

create function pg_catalog.gtsquery_consistent(internal, tsquery, smallint, oid, internal) returns boolean language internal;

create function pg_catalog.gtsquery_penalty(internal, internal, internal) returns internal language internal;

create function pg_catalog.gtsquery_picksplit(internal, internal) returns internal language internal;

create function pg_catalog.gtsquery_same(bigint, bigint, internal) returns internal language internal;

create function pg_catalog.gtsquery_union(internal, internal) returns bigint language internal;

create function pg_catalog.gtsvector_compress(internal) returns internal language internal;

create function pg_catalog.gtsvector_consistent(internal, tsvector, smallint, oid, internal) returns boolean language internal;

create function pg_catalog.gtsvector_consistent(internal, gtsvector, integer, oid, internal) returns boolean language internal;

create function pg_catalog.gtsvector_decompress(internal) returns internal language internal;

create function pg_catalog.gtsvector_options(internal) returns void language internal;

create function pg_catalog.gtsvector_penalty(internal, internal, internal) returns internal language internal;

create function pg_catalog.gtsvector_picksplit(internal, internal) returns internal language internal;

create function pg_catalog.gtsvector_same(gtsvector, gtsvector, internal) returns internal language internal;

create function pg_catalog.gtsvector_union(internal, internal) returns gtsvector language internal;

create function pg_catalog.gtsvectorin(cstring) returns gtsvector language internal;

create function pg_catalog.gtsvectorout(gtsvector) returns cstring language internal;

create function pg_catalog.has_any_column_privilege(oid, oid, text) returns boolean language internal;

create function pg_catalog.has_any_column_privilege(name, text, text) returns boolean language internal;

create function pg_catalog.has_any_column_privilege(name, oid, text) returns boolean language internal;

create function pg_catalog.has_any_column_privilege(oid, text) returns boolean language internal;

create function pg_catalog.has_any_column_privilege(text, text) returns boolean language internal;

create function pg_catalog.has_any_column_privilege(oid, text, text) returns boolean language internal;

create function pg_catalog.has_column_privilege(name, text, text, text) returns boolean language internal;

create function pg_catalog.has_column_privilege(oid, smallint, text) returns boolean language internal;

create function pg_catalog.has_column_privilege(oid, text, text) returns boolean language internal;

create function pg_catalog.has_column_privilege(text, smallint, text) returns boolean language internal;

create function pg_catalog.has_column_privilege(text, text, text) returns boolean language internal;

create function pg_catalog.has_column_privilege(oid, oid, smallint, text) returns boolean language internal;

create function pg_catalog.has_column_privilege(oid, oid, text, text) returns boolean language internal;

create function pg_catalog.has_column_privilege(oid, text, smallint, text) returns boolean language internal;

create function pg_catalog.has_column_privilege(oid, text, text, text) returns boolean language internal;

create function pg_catalog.has_column_privilege(name, oid, smallint, text) returns boolean language internal;

create function pg_catalog.has_column_privilege(name, oid, text, text) returns boolean language internal;

create function pg_catalog.has_column_privilege(name, text, smallint, text) returns boolean language internal;

create function pg_catalog.has_database_privilege(name, text, text) returns boolean language internal;

create function pg_catalog.has_database_privilege(oid, text) returns boolean language internal;

create function pg_catalog.has_database_privilege(text, text) returns boolean language internal;

create function pg_catalog.has_database_privilege(oid, oid, text) returns boolean language internal;

create function pg_catalog.has_database_privilege(oid, text, text) returns boolean language internal;

create function pg_catalog.has_database_privilege(name, oid, text) returns boolean language internal;

create function pg_catalog.has_foreign_data_wrapper_privilege(oid, text) returns boolean language internal;

create function pg_catalog.has_foreign_data_wrapper_privilege(name, text, text) returns boolean language internal;

create function pg_catalog.has_foreign_data_wrapper_privilege(name, oid, text) returns boolean language internal;

create function pg_catalog.has_foreign_data_wrapper_privilege(oid, text, text) returns boolean language internal;

create function pg_catalog.has_foreign_data_wrapper_privilege(oid, oid, text) returns boolean language internal;

create function pg_catalog.has_foreign_data_wrapper_privilege(text, text) returns boolean language internal;

create function pg_catalog.has_function_privilege(oid, text, text) returns boolean language internal;

create function pg_catalog.has_function_privilege(name, text, text) returns boolean language internal;

create function pg_catalog.has_function_privilege(name, oid, text) returns boolean language internal;

create function pg_catalog.has_function_privilege(oid, oid, text) returns boolean language internal;

create function pg_catalog.has_function_privilege(text, text) returns boolean language internal;

create function pg_catalog.has_function_privilege(oid, text) returns boolean language internal;

create function pg_catalog.has_language_privilege(text, text) returns boolean language internal;

create function pg_catalog.has_language_privilege(name, text, text) returns boolean language internal;

create function pg_catalog.has_language_privilege(oid, text, text) returns boolean language internal;

create function pg_catalog.has_language_privilege(oid, text) returns boolean language internal;

create function pg_catalog.has_language_privilege(oid, oid, text) returns boolean language internal;

create function pg_catalog.has_language_privilege(name, oid, text) returns boolean language internal;

create function pg_catalog.has_largeobject_privilege(name, oid, text) returns boolean language internal;

create function pg_catalog.has_largeobject_privilege(oid, text) returns boolean language internal;

create function pg_catalog.has_largeobject_privilege(oid, oid, text) returns boolean language internal;

create function pg_catalog.has_parameter_privilege(name, text, text) returns boolean language internal;

create function pg_catalog.has_parameter_privilege(oid, text, text) returns boolean language internal;

create function pg_catalog.has_parameter_privilege(text, text) returns boolean language internal;

create function pg_catalog.has_schema_privilege(name, oid, text) returns boolean language internal;

create function pg_catalog.has_schema_privilege(name, text, text) returns boolean language internal;

create function pg_catalog.has_schema_privilege(oid, text) returns boolean language internal;

create function pg_catalog.has_schema_privilege(text, text) returns boolean language internal;

create function pg_catalog.has_schema_privilege(oid, oid, text) returns boolean language internal;

create function pg_catalog.has_schema_privilege(oid, text, text) returns boolean language internal;

create function pg_catalog.has_sequence_privilege(oid, oid, text) returns boolean language internal;

create function pg_catalog.has_sequence_privilege(oid, text, text) returns boolean language internal;

create function pg_catalog.has_sequence_privilege(name, oid, text) returns boolean language internal;

create function pg_catalog.has_sequence_privilege(name, text, text) returns boolean language internal;

create function pg_catalog.has_sequence_privilege(oid, text) returns boolean language internal;

create function pg_catalog.has_sequence_privilege(text, text) returns boolean language internal;

create function pg_catalog.has_server_privilege(oid, text) returns boolean language internal;

create function pg_catalog.has_server_privilege(name, oid, text) returns boolean language internal;

create function pg_catalog.has_server_privilege(oid, text, text) returns boolean language internal;

create function pg_catalog.has_server_privilege(text, text) returns boolean language internal;

create function pg_catalog.has_server_privilege(oid, oid, text) returns boolean language internal;

create function pg_catalog.has_server_privilege(name, text, text) returns boolean language internal;

create function pg_catalog.has_table_privilege(oid, text) returns boolean language internal;

create function pg_catalog.has_table_privilege(text, text) returns boolean language internal;

create function pg_catalog.has_table_privilege(oid, oid, text) returns boolean language internal;

create function pg_catalog.has_table_privilege(oid, text, text) returns boolean language internal;

create function pg_catalog.has_table_privilege(name, oid, text) returns boolean language internal;

create function pg_catalog.has_table_privilege(name, text, text) returns boolean language internal;

create function pg_catalog.has_tablespace_privilege(oid, oid, text) returns boolean language internal;

create function pg_catalog.has_tablespace_privilege(name, text, text) returns boolean language internal;

create function pg_catalog.has_tablespace_privilege(oid, text) returns boolean language internal;

create function pg_catalog.has_tablespace_privilege(name, oid, text) returns boolean language internal;

create function pg_catalog.has_tablespace_privilege(oid, text, text) returns boolean language internal;

create function pg_catalog.has_tablespace_privilege(text, text) returns boolean language internal;

create function pg_catalog.has_type_privilege(oid, text, text) returns boolean language internal;

create function pg_catalog.has_type_privilege(name, oid, text) returns boolean language internal;

create function pg_catalog.has_type_privilege(name, text, text) returns boolean language internal;

create function pg_catalog.has_type_privilege(text, text) returns boolean language internal;

create function pg_catalog.has_type_privilege(oid, oid, text) returns boolean language internal;

create function pg_catalog.has_type_privilege(oid, text) returns boolean language internal;

create function pg_catalog.hash_aclitem(aclitem) returns integer language internal;

create function pg_catalog.hash_aclitem_extended(aclitem, bigint) returns bigint language internal;

create function pg_catalog.hash_array(anyarray) returns integer language internal;

create function pg_catalog.hash_array_extended(anyarray, bigint) returns bigint language internal;

create function pg_catalog.hash_multirange(anymultirange) returns integer language internal;

create function pg_catalog.hash_multirange_extended(anymultirange, bigint) returns bigint language internal;

create function pg_catalog.hash_numeric(numeric) returns integer language internal;

create function pg_catalog.hash_numeric_extended(numeric, bigint) returns bigint language internal;

create function pg_catalog.hash_range(anyrange) returns integer language internal;

create function pg_catalog.hash_range_extended(anyrange, bigint) returns bigint language internal;

create function pg_catalog.hash_record(record) returns integer language internal;

create function pg_catalog.hash_record_extended(record, bigint) returns bigint language internal;

create function pg_catalog.hashbool(boolean) returns integer language internal;

create function pg_catalog.hashboolextended(boolean, bigint) returns bigint language internal;

create function pg_catalog.hashbpchar(character) returns integer language internal;

create function pg_catalog.hashbpcharextended(character, bigint) returns bigint language internal;

create function pg_catalog.hashbytea(bytea) returns integer language internal;

create function pg_catalog.hashbyteaextended(bytea, bigint) returns bigint language internal;

create function pg_catalog.hashchar("char") returns integer language internal;

create function pg_catalog.hashcharextended("char", bigint) returns bigint language internal;

create function pg_catalog.hashcid(cid) returns integer language internal;

create function pg_catalog.hashcidextended(cid, bigint) returns bigint language internal;

create function pg_catalog.hashdate(date) returns integer language internal;

create function pg_catalog.hashdateextended(date, bigint) returns bigint language internal;

create function pg_catalog.hashenum(anyenum) returns integer language internal;

create function pg_catalog.hashenumextended(anyenum, bigint) returns bigint language internal;

create function pg_catalog.hashfloat4(real) returns integer language internal;

create function pg_catalog.hashfloat4extended(real, bigint) returns bigint language internal;

create function pg_catalog.hashfloat8(double precision) returns integer language internal;

create function pg_catalog.hashfloat8extended(double precision, bigint) returns bigint language internal;

create function pg_catalog.hashhandler(internal) returns index_am_handler language internal;

create function pg_catalog.hashinet(inet) returns integer language internal;

create function pg_catalog.hashinetextended(inet, bigint) returns bigint language internal;

create function pg_catalog.hashint2(smallint) returns integer language internal;

create function pg_catalog.hashint2extended(smallint, bigint) returns bigint language internal;

create function pg_catalog.hashint4(integer) returns integer language internal;

create function pg_catalog.hashint4extended(integer, bigint) returns bigint language internal;

create function pg_catalog.hashint8(bigint) returns integer language internal;

create function pg_catalog.hashint8extended(bigint, bigint) returns bigint language internal;

create function pg_catalog.hashmacaddr(macaddr) returns integer language internal;

create function pg_catalog.hashmacaddr8(macaddr8) returns integer language internal;

create function pg_catalog.hashmacaddr8extended(macaddr8, bigint) returns bigint language internal;

create function pg_catalog.hashmacaddrextended(macaddr, bigint) returns bigint language internal;

create function pg_catalog.hashname(name) returns integer language internal;

create function pg_catalog.hashnameextended(name, bigint) returns bigint language internal;

create function pg_catalog.hashoid(oid) returns integer language internal;

create function pg_catalog.hashoidextended(oid, bigint) returns bigint language internal;

create function pg_catalog.hashoidvector(oidvector) returns integer language internal;

create function pg_catalog.hashoidvectorextended(oidvector, bigint) returns bigint language internal;

create function pg_catalog.hashtext(text) returns integer language internal;

create function pg_catalog.hashtextextended(text, bigint) returns bigint language internal;

create function pg_catalog.hashtid(tid) returns integer language internal;

create function pg_catalog.hashtidextended(tid, bigint) returns bigint language internal;

create function pg_catalog.hashvarlena(internal) returns integer language internal;

create function pg_catalog.hashvarlenaextended(internal, bigint) returns bigint language internal;

create function pg_catalog.hashxid(xid) returns integer language internal;

create function pg_catalog.hashxid8(xid8) returns integer language internal;

create function pg_catalog.hashxid8extended(xid8, bigint) returns bigint language internal;

create function pg_catalog.hashxidextended(xid, bigint) returns bigint language internal;

create function pg_catalog.heap_tableam_handler(internal) returns table_am_handler language internal;

create function pg_catalog.height(box) returns double precision language internal;

create function pg_catalog.host(inet) returns text language internal;

create function pg_catalog.hostmask(inet) returns inet language internal;

create function pg_catalog.iclikejoinsel(internal, oid, internal, smallint, internal) returns double precision language internal;

create function pg_catalog.iclikesel(internal, oid, internal, integer) returns double precision language internal;

create function pg_catalog.icnlikejoinsel(internal, oid, internal, smallint, internal) returns double precision language internal;

create function pg_catalog.icnlikesel(internal, oid, internal, integer) returns double precision language internal;

create function pg_catalog.icregexeqjoinsel(internal, oid, internal, smallint, internal) returns double precision language internal;

create function pg_catalog.icregexeqsel(internal, oid, internal, integer) returns double precision language internal;

create function pg_catalog.icregexnejoinsel(internal, oid, internal, smallint, internal) returns double precision language internal;

create function pg_catalog.icregexnesel(internal, oid, internal, integer) returns double precision language internal;

create function pg_catalog.icu_unicode_version() returns text language internal;

create function pg_catalog.in_range(time with time zone, time with time zone, interval, boolean, boolean) returns boolean language internal;

create function pg_catalog.in_range(timestamp without time zone, timestamp without time zone, interval, boolean, boolean) returns boolean language internal;

create function pg_catalog.in_range(timestamp with time zone, timestamp with time zone, interval, boolean, boolean) returns boolean language internal;

create function pg_catalog.in_range(interval, interval, interval, boolean, boolean) returns boolean language internal;

create function pg_catalog.in_range(time without time zone, time without time zone, interval, boolean, boolean) returns boolean language internal;

create function pg_catalog.in_range(bigint, bigint, bigint, boolean, boolean) returns boolean language internal;

create function pg_catalog.in_range(integer, integer, bigint, boolean, boolean) returns boolean language internal;

create function pg_catalog.in_range(integer, integer, integer, boolean, boolean) returns boolean language internal;

create function pg_catalog.in_range(integer, integer, smallint, boolean, boolean) returns boolean language internal;

create function pg_catalog.in_range(smallint, smallint, bigint, boolean, boolean) returns boolean language internal;

create function pg_catalog.in_range(smallint, smallint, integer, boolean, boolean) returns boolean language internal;

create function pg_catalog.in_range(smallint, smallint, smallint, boolean, boolean) returns boolean language internal;

create function pg_catalog.in_range(double precision, double precision, double precision, boolean, boolean) returns boolean language internal;

create function pg_catalog.in_range(real, real, double precision, boolean, boolean) returns boolean language internal;

create function pg_catalog.in_range(numeric, numeric, numeric, boolean, boolean) returns boolean language internal;

create function pg_catalog.in_range(date, date, interval, boolean, boolean) returns boolean language internal;

create function pg_catalog.index_am_handler_in(cstring) returns index_am_handler language internal;

create function pg_catalog.index_am_handler_out(index_am_handler) returns cstring language internal;

create function pg_catalog.inet_client_addr() returns inet language internal;

create function pg_catalog.inet_client_port() returns integer language internal;

create function pg_catalog.inet_gist_compress(internal) returns internal language internal;

create function pg_catalog.inet_gist_consistent(internal, inet, smallint, oid, internal) returns boolean language internal;

create function pg_catalog.inet_gist_fetch(internal) returns internal language internal;

create function pg_catalog.inet_gist_penalty(internal, internal, internal) returns internal language internal;

create function pg_catalog.inet_gist_picksplit(internal, internal) returns internal language internal;

create function pg_catalog.inet_gist_same(inet, inet, internal) returns internal language internal;

create function pg_catalog.inet_gist_union(internal, internal) returns inet language internal;

create function pg_catalog.inet_in(cstring) returns inet language internal;

create function pg_catalog.inet_merge(inet, inet) returns cidr language internal;

create function pg_catalog.inet_out(inet) returns cstring language internal;

create function pg_catalog.inet_recv(internal) returns inet language internal;

create function pg_catalog.inet_same_family(inet, inet) returns boolean language internal;

create function pg_catalog.inet_send(inet) returns bytea language internal;

create function pg_catalog.inet_server_addr() returns inet language internal;

create function pg_catalog.inet_server_port() returns integer language internal;

create function pg_catalog.inet_spg_choose(internal, internal) returns void language internal;

create function pg_catalog.inet_spg_config(internal, internal) returns void language internal;

create function pg_catalog.inet_spg_inner_consistent(internal, internal) returns void language internal;

create function pg_catalog.inet_spg_leaf_consistent(internal, internal) returns boolean language internal;

create function pg_catalog.inet_spg_picksplit(internal, internal) returns void language internal;

create function pg_catalog.inetand(inet, inet) returns inet language internal;

create function pg_catalog.inetmi(inet, inet) returns bigint language internal;

create function pg_catalog.inetmi_int8(inet, bigint) returns inet language internal;

create function pg_catalog.inetnot(inet) returns inet language internal;

create function pg_catalog.inetor(inet, inet) returns inet language internal;

create function pg_catalog.inetpl(inet, bigint) returns inet language internal;

create function pg_catalog.initcap(text) returns text language internal;

create function pg_catalog.int2(double precision) returns smallint language internal;

create function pg_catalog.int2(real) returns smallint language internal;

create function pg_catalog.int2(bytea) returns smallint language internal;

create function pg_catalog.int2(numeric) returns smallint language internal;

create function pg_catalog.int2(jsonb) returns smallint language internal;

create function pg_catalog.int2(bigint) returns smallint language internal;

create function pg_catalog.int2(integer) returns smallint language internal;

create function pg_catalog.int24div(smallint, integer) returns integer language internal;

create function pg_catalog.int24eq(smallint, integer) returns boolean language internal;

create function pg_catalog.int24ge(smallint, integer) returns boolean language internal;

create function pg_catalog.int24gt(smallint, integer) returns boolean language internal;

create function pg_catalog.int24le(smallint, integer) returns boolean language internal;

create function pg_catalog.int24lt(smallint, integer) returns boolean language internal;

create function pg_catalog.int24mi(smallint, integer) returns integer language internal;

create function pg_catalog.int24mul(smallint, integer) returns integer language internal;

create function pg_catalog.int24ne(smallint, integer) returns boolean language internal;

create function pg_catalog.int24pl(smallint, integer) returns integer language internal;

create function pg_catalog.int28div(smallint, bigint) returns bigint language internal;

create function pg_catalog.int28eq(smallint, bigint) returns boolean language internal;

create function pg_catalog.int28ge(smallint, bigint) returns boolean language internal;

create function pg_catalog.int28gt(smallint, bigint) returns boolean language internal;

create function pg_catalog.int28le(smallint, bigint) returns boolean language internal;

create function pg_catalog.int28lt(smallint, bigint) returns boolean language internal;

create function pg_catalog.int28mi(smallint, bigint) returns bigint language internal;

create function pg_catalog.int28mul(smallint, bigint) returns bigint language internal;

create function pg_catalog.int28ne(smallint, bigint) returns boolean language internal;

create function pg_catalog.int28pl(smallint, bigint) returns bigint language internal;

create function pg_catalog.int2_accum(internal, smallint) returns internal language internal;

create function pg_catalog.int2_accum_inv(internal, smallint) returns internal language internal;

create function pg_catalog.int2_avg_accum(bigint[], smallint) returns bigint[] language internal;

create function pg_catalog.int2_avg_accum_inv(bigint[], smallint) returns bigint[] language internal;

create function pg_catalog.int2_mul_cash(smallint, money) returns money language internal;

create function pg_catalog.int2_sum(bigint, smallint) returns bigint language internal;

create function pg_catalog.int2abs(smallint) returns smallint language internal;

create function pg_catalog.int2and(smallint, smallint) returns smallint language internal;

create function pg_catalog.int2div(smallint, smallint) returns smallint language internal;

create function pg_catalog.int2eq(smallint, smallint) returns boolean language internal;

create function pg_catalog.int2ge(smallint, smallint) returns boolean language internal;

create function pg_catalog.int2gt(smallint, smallint) returns boolean language internal;

create function pg_catalog.int2in(cstring) returns smallint language internal;

create function pg_catalog.int2int4_sum(bigint[]) returns bigint language internal;

create function pg_catalog.int2larger(smallint, smallint) returns smallint language internal;

create function pg_catalog.int2le(smallint, smallint) returns boolean language internal;

create function pg_catalog.int2lt(smallint, smallint) returns boolean language internal;

create function pg_catalog.int2mi(smallint, smallint) returns smallint language internal;

create function pg_catalog.int2mod(smallint, smallint) returns smallint language internal;

create function pg_catalog.int2mul(smallint, smallint) returns smallint language internal;

create function pg_catalog.int2ne(smallint, smallint) returns boolean language internal;

create function pg_catalog.int2not(smallint) returns smallint language internal;

create function pg_catalog.int2or(smallint, smallint) returns smallint language internal;

create function pg_catalog.int2out(smallint) returns cstring language internal;

create function pg_catalog.int2pl(smallint, smallint) returns smallint language internal;

create function pg_catalog.int2recv(internal) returns smallint language internal;

create function pg_catalog.int2send(smallint) returns bytea language internal;

create function pg_catalog.int2shl(smallint, integer) returns smallint language internal;

create function pg_catalog.int2shr(smallint, integer) returns smallint language internal;

create function pg_catalog.int2smaller(smallint, smallint) returns smallint language internal;

create function pg_catalog.int2um(smallint) returns smallint language internal;

create function pg_catalog.int2up(smallint) returns smallint language internal;

create function pg_catalog.int2vectorin(cstring) returns int2vector language internal;

create function pg_catalog.int2vectorout(int2vector) returns cstring language internal;

create function pg_catalog.int2vectorrecv(internal) returns int2vector language internal;

create function pg_catalog.int2vectorsend(int2vector) returns bytea language internal;

create function pg_catalog.int2xor(smallint, smallint) returns smallint language internal;

create function pg_catalog.int4(bit) returns integer language internal;

create function pg_catalog.int4(bigint) returns integer language internal;

create function pg_catalog.int4(real) returns integer language internal;

create function pg_catalog.int4("char") returns integer language internal;

create function pg_catalog.int4(jsonb) returns integer language internal;

create function pg_catalog.int4(double precision) returns integer language internal;

create function pg_catalog.int4(smallint) returns integer language internal;

create function pg_catalog.int4(bytea) returns integer language internal;

create function pg_catalog.int4(boolean) returns integer language internal;

create function pg_catalog.int4(numeric) returns integer language internal;

create function pg_catalog.int42div(integer, smallint) returns integer language internal;

create function pg_catalog.int42eq(integer, smallint) returns boolean language internal;

create function pg_catalog.int42ge(integer, smallint) returns boolean language internal;

create function pg_catalog.int42gt(integer, smallint) returns boolean language internal;

create function pg_catalog.int42le(integer, smallint) returns boolean language internal;

create function pg_catalog.int42lt(integer, smallint) returns boolean language internal;

create function pg_catalog.int42mi(integer, smallint) returns integer language internal;

create function pg_catalog.int42mul(integer, smallint) returns integer language internal;

create function pg_catalog.int42ne(integer, smallint) returns boolean language internal;

create function pg_catalog.int42pl(integer, smallint) returns integer language internal;

create function pg_catalog.int48div(integer, bigint) returns bigint language internal;

create function pg_catalog.int48eq(integer, bigint) returns boolean language internal;

create function pg_catalog.int48ge(integer, bigint) returns boolean language internal;

create function pg_catalog.int48gt(integer, bigint) returns boolean language internal;

create function pg_catalog.int48le(integer, bigint) returns boolean language internal;

create function pg_catalog.int48lt(integer, bigint) returns boolean language internal;

create function pg_catalog.int48mi(integer, bigint) returns bigint language internal;

create function pg_catalog.int48mul(integer, bigint) returns bigint language internal;

create function pg_catalog.int48ne(integer, bigint) returns boolean language internal;

create function pg_catalog.int48pl(integer, bigint) returns bigint language internal;

create function pg_catalog.int4_accum(internal, integer) returns internal language internal;

create function pg_catalog.int4_accum_inv(internal, integer) returns internal language internal;

create function pg_catalog.int4_avg_accum(bigint[], integer) returns bigint[] language internal;

create function pg_catalog.int4_avg_accum_inv(bigint[], integer) returns bigint[] language internal;

create function pg_catalog.int4_avg_combine(bigint[], bigint[]) returns bigint[] language internal;

create function pg_catalog.int4_mul_cash(integer, money) returns money language internal;

create function pg_catalog.int4_sum(bigint, integer) returns bigint language internal;

create function pg_catalog.int4abs(integer) returns integer language internal;

create function pg_catalog.int4and(integer, integer) returns integer language internal;

create function pg_catalog.int4div(integer, integer) returns integer language internal;

create function pg_catalog.int4eq(integer, integer) returns boolean language internal;

create function pg_catalog.int4ge(integer, integer) returns boolean language internal;

create function pg_catalog.int4gt(integer, integer) returns boolean language internal;

create function pg_catalog.int4in(cstring) returns integer language internal;

create function pg_catalog.int4inc(integer) returns integer language internal;

create function pg_catalog.int4larger(integer, integer) returns integer language internal;

create function pg_catalog.int4le(integer, integer) returns boolean language internal;

create function pg_catalog.int4lt(integer, integer) returns boolean language internal;

create function pg_catalog.int4mi(integer, integer) returns integer language internal;

create function pg_catalog.int4mod(integer, integer) returns integer language internal;

create function pg_catalog.int4mul(integer, integer) returns integer language internal;

create function pg_catalog.int4multirange(VARIADIC int4range[]) returns int4multirange language internal;

create function pg_catalog.int4multirange() returns int4multirange language internal;

create function pg_catalog.int4multirange(int4range) returns int4multirange language internal;

create function pg_catalog.int4ne(integer, integer) returns boolean language internal;

create function pg_catalog.int4not(integer) returns integer language internal;

create function pg_catalog.int4or(integer, integer) returns integer language internal;

create function pg_catalog.int4out(integer) returns cstring language internal;

create function pg_catalog.int4pl(integer, integer) returns integer language internal;

create function pg_catalog.int4range(integer, integer) returns int4range language internal;

create function pg_catalog.int4range(integer, integer, text) returns int4range language internal;

create function pg_catalog.int4range_canonical(int4range) returns int4range language internal;

create function pg_catalog.int4range_subdiff(integer, integer) returns double precision language internal;

create function pg_catalog.int4recv(internal) returns integer language internal;

create function pg_catalog.int4send(integer) returns bytea language internal;

create function pg_catalog.int4shl(integer, integer) returns integer language internal;

create function pg_catalog.int4shr(integer, integer) returns integer language internal;

create function pg_catalog.int4smaller(integer, integer) returns integer language internal;

create function pg_catalog.int4um(integer) returns integer language internal;

create function pg_catalog.int4up(integer) returns integer language internal;

create function pg_catalog.int4xor(integer, integer) returns integer language internal;

create function pg_catalog.int8(smallint) returns bigint language internal;

create function pg_catalog.int8(integer) returns bigint language internal;

create function pg_catalog.int8(double precision) returns bigint language internal;

create function pg_catalog.int8(real) returns bigint language internal;

create function pg_catalog.int8(bit) returns bigint language internal;

create function pg_catalog.int8(oid) returns bigint language internal;

create function pg_catalog.int8(jsonb) returns bigint language internal;

create function pg_catalog.int8(bytea) returns bigint language internal;

create function pg_catalog.int8(numeric) returns bigint language internal;

create function pg_catalog.int82div(bigint, smallint) returns bigint language internal;

create function pg_catalog.int82eq(bigint, smallint) returns boolean language internal;

create function pg_catalog.int82ge(bigint, smallint) returns boolean language internal;

create function pg_catalog.int82gt(bigint, smallint) returns boolean language internal;

create function pg_catalog.int82le(bigint, smallint) returns boolean language internal;

create function pg_catalog.int82lt(bigint, smallint) returns boolean language internal;

create function pg_catalog.int82mi(bigint, smallint) returns bigint language internal;

create function pg_catalog.int82mul(bigint, smallint) returns bigint language internal;

create function pg_catalog.int82ne(bigint, smallint) returns boolean language internal;

create function pg_catalog.int82pl(bigint, smallint) returns bigint language internal;

create function pg_catalog.int84div(bigint, integer) returns bigint language internal;

create function pg_catalog.int84eq(bigint, integer) returns boolean language internal;

create function pg_catalog.int84ge(bigint, integer) returns boolean language internal;

create function pg_catalog.int84gt(bigint, integer) returns boolean language internal;

create function pg_catalog.int84le(bigint, integer) returns boolean language internal;

create function pg_catalog.int84lt(bigint, integer) returns boolean language internal;

create function pg_catalog.int84mi(bigint, integer) returns bigint language internal;

create function pg_catalog.int84mul(bigint, integer) returns bigint language internal;

create function pg_catalog.int84ne(bigint, integer) returns boolean language internal;

create function pg_catalog.int84pl(bigint, integer) returns bigint language internal;

create function pg_catalog.int8_accum(internal, bigint) returns internal language internal;

create function pg_catalog.int8_accum_inv(internal, bigint) returns internal language internal;

create function pg_catalog.int8_avg(bigint[]) returns numeric language internal;

create function pg_catalog.int8_avg_accum(internal, bigint) returns internal language internal;

create function pg_catalog.int8_avg_accum_inv(internal, bigint) returns internal language internal;

create function pg_catalog.int8_avg_combine(internal, internal) returns internal language internal;

create function pg_catalog.int8_avg_deserialize(bytea, internal) returns internal language internal;

create function pg_catalog.int8_avg_serialize(internal) returns bytea language internal;

create function pg_catalog.int8_mul_cash(bigint, money) returns money language internal;

create function pg_catalog.int8_sum(numeric, bigint) returns numeric language internal;

create function pg_catalog.int8abs(bigint) returns bigint language internal;

create function pg_catalog.int8and(bigint, bigint) returns bigint language internal;

create function pg_catalog.int8dec(bigint) returns bigint language internal;

create function pg_catalog.int8dec_any(bigint, "any") returns bigint language internal;

create function pg_catalog.int8div(bigint, bigint) returns bigint language internal;

create function pg_catalog.int8eq(bigint, bigint) returns boolean language internal;

create function pg_catalog.int8ge(bigint, bigint) returns boolean language internal;

create function pg_catalog.int8gt(bigint, bigint) returns boolean language internal;

create function pg_catalog.int8in(cstring) returns bigint language internal;

create function pg_catalog.int8inc(bigint) returns bigint language internal;

create function pg_catalog.int8inc_any(bigint, "any") returns bigint language internal;

create function pg_catalog.int8inc_float8_float8(bigint, double precision, double precision) returns bigint language internal;

create function pg_catalog.int8inc_support(internal) returns internal language internal;

create function pg_catalog.int8larger(bigint, bigint) returns bigint language internal;

create function pg_catalog.int8le(bigint, bigint) returns boolean language internal;

create function pg_catalog.int8lt(bigint, bigint) returns boolean language internal;

create function pg_catalog.int8mi(bigint, bigint) returns bigint language internal;

create function pg_catalog.int8mod(bigint, bigint) returns bigint language internal;

create function pg_catalog.int8mul(bigint, bigint) returns bigint language internal;

create function pg_catalog.int8multirange() returns int8multirange language internal;

create function pg_catalog.int8multirange(int8range) returns int8multirange language internal;

create function pg_catalog.int8multirange(VARIADIC int8range[]) returns int8multirange language internal;

create function pg_catalog.int8ne(bigint, bigint) returns boolean language internal;

create function pg_catalog.int8not(bigint) returns bigint language internal;

create function pg_catalog.int8or(bigint, bigint) returns bigint language internal;

create function pg_catalog.int8out(bigint) returns cstring language internal;

create function pg_catalog.int8pl(bigint, bigint) returns bigint language internal;

create function pg_catalog.int8pl_inet(bigint, inet) returns inet language internal;

create function pg_catalog.int8range(bigint, bigint) returns int8range language internal;

create function pg_catalog.int8range(bigint, bigint, text) returns int8range language internal;

create function pg_catalog.int8range_canonical(int8range) returns int8range language internal;

create function pg_catalog.int8range_subdiff(bigint, bigint) returns double precision language internal;

create function pg_catalog.int8recv(internal) returns bigint language internal;

create function pg_catalog.int8send(bigint) returns bytea language internal;

create function pg_catalog.int8shl(bigint, integer) returns bigint language internal;

create function pg_catalog.int8shr(bigint, integer) returns bigint language internal;

create function pg_catalog.int8smaller(bigint, bigint) returns bigint language internal;

create function pg_catalog.int8um(bigint) returns bigint language internal;

create function pg_catalog.int8up(bigint) returns bigint language internal;

create function pg_catalog.int8xor(bigint, bigint) returns bigint language internal;

create function pg_catalog.integer_pl_date(integer, date) returns date language internal;

create function pg_catalog.inter_lb(line, box) returns boolean language internal;

create function pg_catalog.inter_sb(lseg, box) returns boolean language internal;

create function pg_catalog.inter_sl(lseg, line) returns boolean language internal;

create function pg_catalog.internal_in(cstring) returns internal language internal;

create function pg_catalog.internal_out(internal) returns cstring language internal;

create function pg_catalog.interval(time without time zone) returns interval language internal;

create function pg_catalog.interval(interval, integer) returns interval language internal;

create function pg_catalog.interval_avg(internal) returns interval language internal;

create function pg_catalog.interval_avg_accum(internal, interval) returns internal language internal;

create function pg_catalog.interval_avg_accum_inv(internal, interval) returns internal language internal;

create function pg_catalog.interval_avg_combine(internal, internal) returns internal language internal;

create function pg_catalog.interval_avg_deserialize(bytea, internal) returns internal language internal;

create function pg_catalog.interval_avg_serialize(internal) returns bytea language internal;

create function pg_catalog.interval_cmp(interval, interval) returns integer language internal;

create function pg_catalog.interval_div(interval, double precision) returns interval language internal;

create function pg_catalog.interval_eq(interval, interval) returns boolean language internal;

create function pg_catalog.interval_ge(interval, interval) returns boolean language internal;

create function pg_catalog.interval_gt(interval, interval) returns boolean language internal;

create function pg_catalog.interval_hash(interval) returns integer language internal;

create function pg_catalog.interval_hash_extended(interval, bigint) returns bigint language internal;

create function pg_catalog.interval_in(cstring, oid, integer) returns interval language internal;

create function pg_catalog.interval_larger(interval, interval) returns interval language internal;

create function pg_catalog.interval_le(interval, interval) returns boolean language internal;

create function pg_catalog.interval_lt(interval, interval) returns boolean language internal;

create function pg_catalog.interval_mi(interval, interval) returns interval language internal;

create function pg_catalog.interval_mul(interval, double precision) returns interval language internal;

create function pg_catalog.interval_ne(interval, interval) returns boolean language internal;

create function pg_catalog.interval_out(interval) returns cstring language internal;

create function pg_catalog.interval_pl(interval, interval) returns interval language internal;

create function pg_catalog.interval_pl_date(interval, date) returns timestamp without time zone language internal;

create function pg_catalog.interval_pl_time(interval, time without time zone) returns time without time zone language internal;

create function pg_catalog.interval_pl_timestamp(interval, timestamp without time zone) returns timestamp without time zone language internal;

create function pg_catalog.interval_pl_timestamptz(interval, timestamp with time zone) returns timestamp with time zone language internal;

create function pg_catalog.interval_pl_timetz(interval, time with time zone) returns time with time zone language internal;

create function pg_catalog.interval_recv(internal, oid, integer) returns interval language internal;

create function pg_catalog.interval_send(interval) returns bytea language internal;

create function pg_catalog.interval_smaller(interval, interval) returns interval language internal;

create function pg_catalog.interval_sum(internal) returns interval language internal;

create function pg_catalog.interval_support(internal) returns internal language internal;

create function pg_catalog.interval_um(interval) returns interval language internal;

create function pg_catalog.intervaltypmodin(cstring[]) returns integer language internal;

create function pg_catalog.intervaltypmodout(integer) returns cstring language internal;

create function pg_catalog.is_normalized(text, text DEFAULT 'NFC'::text) returns boolean language internal;

create function pg_catalog.isclosed(path) returns boolean language internal;

create function pg_catalog.isempty(anymultirange) returns boolean language internal;

create function pg_catalog.isempty(anyrange) returns boolean language internal;

create function pg_catalog.isfinite(timestamp without time zone) returns boolean language internal;

create function pg_catalog.isfinite(date) returns boolean language internal;

create function pg_catalog.isfinite(interval) returns boolean language internal;

create function pg_catalog.isfinite(timestamp with time zone) returns boolean language internal;

create function pg_catalog.ishorizontal(line) returns boolean language internal;

create function pg_catalog.ishorizontal(lseg) returns boolean language internal;

create function pg_catalog.ishorizontal(point, point) returns boolean language internal;

create function pg_catalog.iso8859_1_to_utf8(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.iso8859_to_utf8(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.iso_to_koi8r(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.iso_to_mic(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.iso_to_win1251(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.iso_to_win866(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.isopen(path) returns boolean language internal;

create function pg_catalog.isparallel(lseg, lseg) returns boolean language internal;

create function pg_catalog.isparallel(line, line) returns boolean language internal;

create function pg_catalog.isperp(lseg, lseg) returns boolean language internal;

create function pg_catalog.isperp(line, line) returns boolean language internal;

create function pg_catalog.isvertical(point, point) returns boolean language internal;

create function pg_catalog.isvertical(line) returns boolean language internal;

create function pg_catalog.isvertical(lseg) returns boolean language internal;

create function pg_catalog.johab_to_utf8(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.json_agg(anyelement) returns json language internal;

create function pg_catalog.json_agg_finalfn(internal) returns json language internal;

create function pg_catalog.json_agg_strict(anyelement) returns json language internal;

create function pg_catalog.json_agg_strict_transfn(internal, anyelement) returns internal language internal;

create function pg_catalog.json_agg_transfn(internal, anyelement) returns internal language internal;

create function pg_catalog.json_array_element(from_json json, element_index integer) returns json language internal;

create function pg_catalog.json_array_element_text(from_json json, element_index integer) returns text language internal;

create function pg_catalog.json_array_elements(from_json json, OUT value json) returns SETOF json language internal;

create function pg_catalog.json_array_elements_text(from_json json, OUT value text) returns SETOF text language internal;

create function pg_catalog.json_array_length(json) returns integer language internal;

create function pg_catalog.json_build_array() returns json language internal;

create function pg_catalog.json_build_array(VARIADIC "any") returns json language internal;

create function pg_catalog.json_build_object() returns json language internal;

create function pg_catalog.json_build_object(VARIADIC "any") returns json language internal;

create function pg_catalog.json_each(from_json json, OUT key text, OUT value json) returns SETOF record language internal;

create function pg_catalog.json_each_text(from_json json, OUT key text, OUT value text) returns SETOF record language internal;

create function pg_catalog.json_extract_path(from_json json, VARIADIC path_elems text[]) returns json language internal;

create function pg_catalog.json_extract_path_text(from_json json, VARIADIC path_elems text[]) returns text language internal;

create function pg_catalog.json_in(cstring) returns json language internal;

create function pg_catalog.json_object(text[]) returns json language internal;

create function pg_catalog.json_object(text[], text[]) returns json language internal;

create function pg_catalog.json_object_agg(key "any", value "any") returns json language internal;

create function pg_catalog.json_object_agg_finalfn(internal) returns json language internal;

create function pg_catalog.json_object_agg_strict(key "any", value "any") returns json language internal;

create function pg_catalog.json_object_agg_strict_transfn(internal, "any", "any") returns internal language internal;

create function pg_catalog.json_object_agg_transfn(internal, "any", "any") returns internal language internal;

create function pg_catalog.json_object_agg_unique(key "any", value "any") returns json language internal;

create function pg_catalog.json_object_agg_unique_strict(key "any", value "any") returns json language internal;

create function pg_catalog.json_object_agg_unique_strict_transfn(internal, "any", "any") returns internal language internal;

create function pg_catalog.json_object_agg_unique_transfn(internal, "any", "any") returns internal language internal;

create function pg_catalog.json_object_field(from_json json, field_name text) returns json language internal;

create function pg_catalog.json_object_field_text(from_json json, field_name text) returns text language internal;

create function pg_catalog.json_object_keys(json) returns SETOF text language internal;

create function pg_catalog.json_out(json) returns cstring language internal;

create function pg_catalog.json_populate_record(base anyelement, from_json json, use_json_as_text boolean DEFAULT false) returns anyelement language internal;

create function pg_catalog.json_populate_recordset(base anyelement, from_json json, use_json_as_text boolean DEFAULT false) returns SETOF anyelement language internal;

create function pg_catalog.json_recv(internal) returns json language internal;

create function pg_catalog.json_send(json) returns bytea language internal;

create function pg_catalog.json_strip_nulls(target json, strip_in_arrays boolean DEFAULT false) returns json language internal;

create function pg_catalog.json_to_record(json) returns record language internal;

create function pg_catalog.json_to_recordset(json) returns SETOF record language internal;

create function pg_catalog.json_to_tsvector(json, jsonb) returns tsvector language internal;

create function pg_catalog.json_to_tsvector(regconfig, json, jsonb) returns tsvector language internal;

create function pg_catalog.json_typeof(json) returns text language internal;

create function pg_catalog.jsonb_agg(anyelement) returns jsonb language internal;

create function pg_catalog.jsonb_agg_finalfn(internal) returns jsonb language internal;

create function pg_catalog.jsonb_agg_strict(anyelement) returns jsonb language internal;

create function pg_catalog.jsonb_agg_strict_transfn(internal, anyelement) returns internal language internal;

create function pg_catalog.jsonb_agg_transfn(internal, anyelement) returns internal language internal;

create function pg_catalog.jsonb_array_element(from_json jsonb, element_index integer) returns jsonb language internal;

create function pg_catalog.jsonb_array_element_text(from_json jsonb, element_index integer) returns text language internal;

create function pg_catalog.jsonb_array_elements(from_json jsonb, OUT value jsonb) returns SETOF jsonb language internal;

create function pg_catalog.jsonb_array_elements_text(from_json jsonb, OUT value text) returns SETOF text language internal;

create function pg_catalog.jsonb_array_length(jsonb) returns integer language internal;

create function pg_catalog.jsonb_build_array() returns jsonb language internal;

create function pg_catalog.jsonb_build_array(VARIADIC "any") returns jsonb language internal;

create function pg_catalog.jsonb_build_object(VARIADIC "any") returns jsonb language internal;

create function pg_catalog.jsonb_build_object() returns jsonb language internal;

create function pg_catalog.jsonb_cmp(jsonb, jsonb) returns integer language internal;

create function pg_catalog.jsonb_concat(jsonb, jsonb) returns jsonb language internal;

create function pg_catalog.jsonb_contained(jsonb, jsonb) returns boolean language internal;

create function pg_catalog.jsonb_contains(jsonb, jsonb) returns boolean language internal;

create function pg_catalog.jsonb_delete(from_json jsonb, VARIADIC path_elems text[]) returns jsonb language internal;

create function pg_catalog.jsonb_delete(jsonb, integer) returns jsonb language internal;

create function pg_catalog.jsonb_delete(jsonb, text) returns jsonb language internal;

create function pg_catalog.jsonb_delete_path(jsonb, text[]) returns jsonb language internal;

create function pg_catalog.jsonb_each(from_json jsonb, OUT key text, OUT value jsonb) returns SETOF record language internal;

create function pg_catalog.jsonb_each_text(from_json jsonb, OUT key text, OUT value text) returns SETOF record language internal;

create function pg_catalog.jsonb_eq(jsonb, jsonb) returns boolean language internal;

create function pg_catalog.jsonb_exists(jsonb, text) returns boolean language internal;

create function pg_catalog.jsonb_exists_all(jsonb, text[]) returns boolean language internal;

create function pg_catalog.jsonb_exists_any(jsonb, text[]) returns boolean language internal;

create function pg_catalog.jsonb_extract_path(from_json jsonb, VARIADIC path_elems text[]) returns jsonb language internal;

create function pg_catalog.jsonb_extract_path_text(from_json jsonb, VARIADIC path_elems text[]) returns text language internal;

create function pg_catalog.jsonb_ge(jsonb, jsonb) returns boolean language internal;

create function pg_catalog.jsonb_gt(jsonb, jsonb) returns boolean language internal;

create function pg_catalog.jsonb_hash(jsonb) returns integer language internal;

create function pg_catalog.jsonb_hash_extended(jsonb, bigint) returns bigint language internal;

create function pg_catalog.jsonb_in(cstring) returns jsonb language internal;

create function pg_catalog.jsonb_insert(jsonb_in jsonb, path text[], replacement jsonb, insert_after boolean DEFAULT false) returns jsonb language internal;

create function pg_catalog.jsonb_le(jsonb, jsonb) returns boolean language internal;

create function pg_catalog.jsonb_lt(jsonb, jsonb) returns boolean language internal;

create function pg_catalog.jsonb_ne(jsonb, jsonb) returns boolean language internal;

create function pg_catalog.jsonb_object(text[], text[]) returns jsonb language internal;

create function pg_catalog.jsonb_object(text[]) returns jsonb language internal;

create function pg_catalog.jsonb_object_agg(key "any", value "any") returns jsonb language internal;

create function pg_catalog.jsonb_object_agg_finalfn(internal) returns jsonb language internal;

create function pg_catalog.jsonb_object_agg_strict(key "any", value "any") returns jsonb language internal;

create function pg_catalog.jsonb_object_agg_strict_transfn(internal, "any", "any") returns internal language internal;

create function pg_catalog.jsonb_object_agg_transfn(internal, "any", "any") returns internal language internal;

create function pg_catalog.jsonb_object_agg_unique(key "any", value "any") returns jsonb language internal;

create function pg_catalog.jsonb_object_agg_unique_strict(key "any", value "any") returns jsonb language internal;

create function pg_catalog.jsonb_object_agg_unique_strict_transfn(internal, "any", "any") returns internal language internal;

create function pg_catalog.jsonb_object_agg_unique_transfn(internal, "any", "any") returns internal language internal;

create function pg_catalog.jsonb_object_field(from_json jsonb, field_name text) returns jsonb language internal;

create function pg_catalog.jsonb_object_field_text(from_json jsonb, field_name text) returns text language internal;

create function pg_catalog.jsonb_object_keys(jsonb) returns SETOF text language internal;

create function pg_catalog.jsonb_out(jsonb) returns cstring language internal;

create function pg_catalog.jsonb_path_exists(target jsonb, path jsonpath, vars jsonb DEFAULT '{}'::jsonb, silent boolean DEFAULT false) returns boolean language internal;

create function pg_catalog.jsonb_path_exists_opr(jsonb, jsonpath) returns boolean language internal;

create function pg_catalog.jsonb_path_exists_tz(target jsonb, path jsonpath, vars jsonb DEFAULT '{}'::jsonb, silent boolean DEFAULT false) returns boolean language internal;

create function pg_catalog.jsonb_path_match(target jsonb, path jsonpath, vars jsonb DEFAULT '{}'::jsonb, silent boolean DEFAULT false) returns boolean language internal;

create function pg_catalog.jsonb_path_match_opr(jsonb, jsonpath) returns boolean language internal;

create function pg_catalog.jsonb_path_match_tz(target jsonb, path jsonpath, vars jsonb DEFAULT '{}'::jsonb, silent boolean DEFAULT false) returns boolean language internal;

create function pg_catalog.jsonb_path_query(target jsonb, path jsonpath, vars jsonb DEFAULT '{}'::jsonb, silent boolean DEFAULT false) returns SETOF jsonb language internal;

create function pg_catalog.jsonb_path_query_array(target jsonb, path jsonpath, vars jsonb DEFAULT '{}'::jsonb, silent boolean DEFAULT false) returns jsonb language internal;

create function pg_catalog.jsonb_path_query_array_tz(target jsonb, path jsonpath, vars jsonb DEFAULT '{}'::jsonb, silent boolean DEFAULT false) returns jsonb language internal;

create function pg_catalog.jsonb_path_query_first(target jsonb, path jsonpath, vars jsonb DEFAULT '{}'::jsonb, silent boolean DEFAULT false) returns jsonb language internal;

create function pg_catalog.jsonb_path_query_first_tz(target jsonb, path jsonpath, vars jsonb DEFAULT '{}'::jsonb, silent boolean DEFAULT false) returns jsonb language internal;

create function pg_catalog.jsonb_path_query_tz(target jsonb, path jsonpath, vars jsonb DEFAULT '{}'::jsonb, silent boolean DEFAULT false) returns SETOF jsonb language internal;

create function pg_catalog.jsonb_populate_record(anyelement, jsonb) returns anyelement language internal;

create function pg_catalog.jsonb_populate_record_valid(anyelement, jsonb) returns boolean language internal;

create function pg_catalog.jsonb_populate_recordset(anyelement, jsonb) returns SETOF anyelement language internal;

create function pg_catalog.jsonb_pretty(jsonb) returns text language internal;

create function pg_catalog.jsonb_recv(internal) returns jsonb language internal;

create function pg_catalog.jsonb_send(jsonb) returns bytea language internal;

create function pg_catalog.jsonb_set(jsonb_in jsonb, path text[], replacement jsonb, create_if_missing boolean DEFAULT true) returns jsonb language internal;

create function pg_catalog.jsonb_set_lax(jsonb_in jsonb, path text[], replacement jsonb, create_if_missing boolean DEFAULT true, null_value_treatment text DEFAULT 'use_json_null'::text) returns jsonb language internal;

create function pg_catalog.jsonb_strip_nulls(target jsonb, strip_in_arrays boolean DEFAULT false) returns jsonb language internal;

create function pg_catalog.jsonb_subscript_handler(internal) returns internal language internal;

create function pg_catalog.jsonb_to_record(jsonb) returns record language internal;

create function pg_catalog.jsonb_to_recordset(jsonb) returns SETOF record language internal;

create function pg_catalog.jsonb_to_tsvector(jsonb, jsonb) returns tsvector language internal;

create function pg_catalog.jsonb_to_tsvector(regconfig, jsonb, jsonb) returns tsvector language internal;

create function pg_catalog.jsonb_typeof(jsonb) returns text language internal;

create function pg_catalog.jsonpath_in(cstring) returns jsonpath language internal;

create function pg_catalog.jsonpath_out(jsonpath) returns cstring language internal;

create function pg_catalog.jsonpath_recv(internal) returns jsonpath language internal;

create function pg_catalog.jsonpath_send(jsonpath) returns bytea language internal;

create function pg_catalog.justify_days(interval) returns interval language internal;

create function pg_catalog.justify_hours(interval) returns interval language internal;

create function pg_catalog.justify_interval(interval) returns interval language internal;

create function pg_catalog.koi8r_to_iso(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.koi8r_to_mic(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.koi8r_to_utf8(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.koi8r_to_win1251(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.koi8r_to_win866(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.koi8u_to_utf8(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.lag(anyelement) returns anyelement language internal;

create function pg_catalog.lag(anyelement, integer) returns anyelement language internal;

create function pg_catalog.lag(anycompatible, integer, anycompatible) returns anycompatible language internal;

create function pg_catalog.language_handler_in(cstring) returns language_handler language internal;

create function pg_catalog.language_handler_out(language_handler) returns cstring language internal;

create function pg_catalog.last_value(anyelement) returns anyelement language internal;

create function pg_catalog.lastval() returns bigint language internal;

create function pg_catalog.latin1_to_mic(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.latin2_to_mic(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.latin2_to_win1250(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.latin3_to_mic(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.latin4_to_mic(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.lcm(integer, integer) returns integer language internal;

create function pg_catalog.lcm(numeric, numeric) returns numeric language internal;

create function pg_catalog.lcm(bigint, bigint) returns bigint language internal;

create function pg_catalog.lead(anyelement, integer) returns anyelement language internal;

create function pg_catalog.lead(anyelement) returns anyelement language internal;

create function pg_catalog.lead(anycompatible, integer, anycompatible) returns anycompatible language internal;

create function pg_catalog.left(text, integer) returns text language internal;

create function pg_catalog.length(character) returns integer language internal;

create function pg_catalog.length(path) returns double precision language internal;

create function pg_catalog.length(lseg) returns double precision language internal;

create function pg_catalog.length(bytea, name) returns integer language internal;

create function pg_catalog.length(bit) returns integer language internal;

create function pg_catalog.length(tsvector) returns integer language internal;

create function pg_catalog.length(text) returns integer language internal;

create function pg_catalog.length(bytea) returns integer language internal;

create function pg_catalog.lgamma(double precision) returns double precision language internal;

create function pg_catalog.like(name, text) returns boolean language internal;

create function pg_catalog.like(bytea, bytea) returns boolean language internal;

create function pg_catalog.like(text, text) returns boolean language internal;

create function pg_catalog.like_escape(text, text) returns text language internal;

create function pg_catalog.like_escape(bytea, bytea) returns bytea language internal;

create function pg_catalog.likejoinsel(internal, oid, internal, smallint, internal) returns double precision language internal;

create function pg_catalog.likesel(internal, oid, internal, integer) returns double precision language internal;

create function pg_catalog.line(point, point) returns line language internal;

create function pg_catalog.line_distance(line, line) returns double precision language internal;

create function pg_catalog.line_eq(line, line) returns boolean language internal;

create function pg_catalog.line_horizontal(line) returns boolean language internal;

create function pg_catalog.line_in(cstring) returns line language internal;

create function pg_catalog.line_interpt(line, line) returns point language internal;

create function pg_catalog.line_intersect(line, line) returns boolean language internal;

create function pg_catalog.line_out(line) returns cstring language internal;

create function pg_catalog.line_parallel(line, line) returns boolean language internal;

create function pg_catalog.line_perp(line, line) returns boolean language internal;

create function pg_catalog.line_recv(internal) returns line language internal;

create function pg_catalog.line_send(line) returns bytea language internal;

create function pg_catalog.line_vertical(line) returns boolean language internal;

create function pg_catalog.ln(double precision) returns double precision language internal;

create function pg_catalog.ln(numeric) returns numeric language internal;

create function pg_catalog.lo_close(integer) returns integer language internal;

create function pg_catalog.lo_creat(integer) returns oid language internal;

create function pg_catalog.lo_create(oid) returns oid language internal;

create function pg_catalog.lo_export(oid, text) returns integer language internal;

create function pg_catalog.lo_from_bytea(oid, bytea) returns oid language internal;

create function pg_catalog.lo_get(oid, bigint, integer) returns bytea language internal;

create function pg_catalog.lo_get(oid) returns bytea language internal;

create function pg_catalog.lo_import(text) returns oid language internal;

create function pg_catalog.lo_import(text, oid) returns oid language internal;

create function pg_catalog.lo_lseek(integer, integer, integer) returns integer language internal;

create function pg_catalog.lo_lseek64(integer, bigint, integer) returns bigint language internal;

create function pg_catalog.lo_open(oid, integer) returns integer language internal;

create function pg_catalog.lo_put(oid, bigint, bytea) returns void language internal;

create function pg_catalog.lo_tell(integer) returns integer language internal;

create function pg_catalog.lo_tell64(integer) returns bigint language internal;

create function pg_catalog.lo_truncate(integer, integer) returns integer language internal;

create function pg_catalog.lo_truncate64(integer, bigint) returns integer language internal;

create function pg_catalog.lo_unlink(oid) returns integer language internal;

create function pg_catalog.log(numeric) returns numeric language internal;

create function pg_catalog.log(numeric, numeric) returns numeric language internal;

create function pg_catalog.log(double precision) returns double precision language internal;

create function pg_catalog.log10(double precision) returns double precision language internal;

create function pg_catalog.log10(numeric) returns numeric language internal;

create function pg_catalog.loread(integer, integer) returns bytea language internal;

create function pg_catalog.lower(anymultirange) returns anyelement language internal;

create function pg_catalog.lower(text) returns text language internal;

create function pg_catalog.lower(anyrange) returns anyelement language internal;

create function pg_catalog.lower_inc(anymultirange) returns boolean language internal;

create function pg_catalog.lower_inc(anyrange) returns boolean language internal;

create function pg_catalog.lower_inf(anyrange) returns boolean language internal;

create function pg_catalog.lower_inf(anymultirange) returns boolean language internal;

create function pg_catalog.lowrite(integer, bytea) returns integer language internal;

create function pg_catalog.lpad(text, integer, text) returns text language internal;

create function pg_catalog.lpad(text, integer) returns text language internal;

create function pg_catalog.lseg(box) returns lseg language internal;

create function pg_catalog.lseg(point, point) returns lseg language internal;

create function pg_catalog.lseg_center(lseg) returns point language internal;

create function pg_catalog.lseg_distance(lseg, lseg) returns double precision language internal;

create function pg_catalog.lseg_eq(lseg, lseg) returns boolean language internal;

create function pg_catalog.lseg_ge(lseg, lseg) returns boolean language internal;

create function pg_catalog.lseg_gt(lseg, lseg) returns boolean language internal;

create function pg_catalog.lseg_horizontal(lseg) returns boolean language internal;

create function pg_catalog.lseg_in(cstring) returns lseg language internal;

create function pg_catalog.lseg_interpt(lseg, lseg) returns point language internal;

create function pg_catalog.lseg_intersect(lseg, lseg) returns boolean language internal;

create function pg_catalog.lseg_le(lseg, lseg) returns boolean language internal;

create function pg_catalog.lseg_length(lseg) returns double precision language internal;

create function pg_catalog.lseg_lt(lseg, lseg) returns boolean language internal;

create function pg_catalog.lseg_ne(lseg, lseg) returns boolean language internal;

create function pg_catalog.lseg_out(lseg) returns cstring language internal;

create function pg_catalog.lseg_parallel(lseg, lseg) returns boolean language internal;

create function pg_catalog.lseg_perp(lseg, lseg) returns boolean language internal;

create function pg_catalog.lseg_recv(internal) returns lseg language internal;

create function pg_catalog.lseg_send(lseg) returns bytea language internal;

create function pg_catalog.lseg_vertical(lseg) returns boolean language internal;

create function pg_catalog.ltrim(text, text) returns text language internal;

create function pg_catalog.ltrim(bytea, bytea) returns bytea language internal;

create function pg_catalog.ltrim(text) returns text language internal;

create function pg_catalog.macaddr(macaddr8) returns macaddr language internal;

create function pg_catalog.macaddr8(macaddr) returns macaddr8 language internal;

create function pg_catalog.macaddr8_and(macaddr8, macaddr8) returns macaddr8 language internal;

create function pg_catalog.macaddr8_cmp(macaddr8, macaddr8) returns integer language internal;

create function pg_catalog.macaddr8_eq(macaddr8, macaddr8) returns boolean language internal;

create function pg_catalog.macaddr8_ge(macaddr8, macaddr8) returns boolean language internal;

create function pg_catalog.macaddr8_gt(macaddr8, macaddr8) returns boolean language internal;

create function pg_catalog.macaddr8_in(cstring) returns macaddr8 language internal;

create function pg_catalog.macaddr8_le(macaddr8, macaddr8) returns boolean language internal;

create function pg_catalog.macaddr8_lt(macaddr8, macaddr8) returns boolean language internal;

create function pg_catalog.macaddr8_ne(macaddr8, macaddr8) returns boolean language internal;

create function pg_catalog.macaddr8_not(macaddr8) returns macaddr8 language internal;

create function pg_catalog.macaddr8_or(macaddr8, macaddr8) returns macaddr8 language internal;

create function pg_catalog.macaddr8_out(macaddr8) returns cstring language internal;

create function pg_catalog.macaddr8_recv(internal) returns macaddr8 language internal;

create function pg_catalog.macaddr8_send(macaddr8) returns bytea language internal;

create function pg_catalog.macaddr8_set7bit(macaddr8) returns macaddr8 language internal;

create function pg_catalog.macaddr_and(macaddr, macaddr) returns macaddr language internal;

create function pg_catalog.macaddr_cmp(macaddr, macaddr) returns integer language internal;

create function pg_catalog.macaddr_eq(macaddr, macaddr) returns boolean language internal;

create function pg_catalog.macaddr_ge(macaddr, macaddr) returns boolean language internal;

create function pg_catalog.macaddr_gt(macaddr, macaddr) returns boolean language internal;

create function pg_catalog.macaddr_in(cstring) returns macaddr language internal;

create function pg_catalog.macaddr_le(macaddr, macaddr) returns boolean language internal;

create function pg_catalog.macaddr_lt(macaddr, macaddr) returns boolean language internal;

create function pg_catalog.macaddr_ne(macaddr, macaddr) returns boolean language internal;

create function pg_catalog.macaddr_not(macaddr) returns macaddr language internal;

create function pg_catalog.macaddr_or(macaddr, macaddr) returns macaddr language internal;

create function pg_catalog.macaddr_out(macaddr) returns cstring language internal;

create function pg_catalog.macaddr_recv(internal) returns macaddr language internal;

create function pg_catalog.macaddr_send(macaddr) returns bytea language internal;

create function pg_catalog.macaddr_sortsupport(internal) returns void language internal;

create function pg_catalog.make_date(year integer, month integer, day integer) returns date language internal;

create function pg_catalog.make_interval(years integer DEFAULT 0, months integer DEFAULT 0, weeks integer DEFAULT 0, days integer DEFAULT 0, hours integer DEFAULT 0, mins integer DEFAULT 0, secs double precision DEFAULT 0.0) returns interval language internal;

create function pg_catalog.make_time(hour integer, min integer, sec double precision) returns time without time zone language internal;

create function pg_catalog.make_timestamp(year integer, month integer, mday integer, hour integer, min integer, sec double precision) returns timestamp without time zone language internal;

create function pg_catalog.make_timestamptz(year integer, month integer, mday integer, hour integer, min integer, sec double precision) returns timestamp with time zone language internal;

create function pg_catalog.make_timestamptz(year integer, month integer, mday integer, hour integer, min integer, sec double precision, timezone text) returns timestamp with time zone language internal;

create function pg_catalog.makeaclitem(oid, oid, text, boolean) returns aclitem language internal;

create function pg_catalog.masklen(inet) returns integer language internal;

create function pg_catalog.matchingjoinsel(internal, oid, internal, smallint, internal) returns double precision language internal;

create function pg_catalog.matchingsel(internal, oid, internal, integer) returns double precision language internal;

create function pg_catalog.max(anyarray) returns anyarray language internal;

create function pg_catalog.max(record) returns record language internal;

create function pg_catalog.max(character) returns character language internal;

create function pg_catalog.max(tid) returns tid language internal;

create function pg_catalog.max(inet) returns inet language internal;

create function pg_catalog.max(pg_lsn) returns pg_lsn language internal;

create function pg_catalog.max(xid8) returns xid8 language internal;

create function pg_catalog.max(bytea) returns bytea language internal;

create function pg_catalog.max(date) returns date language internal;

create function pg_catalog.max(double precision) returns double precision language internal;

create function pg_catalog.max(real) returns real language internal;

create function pg_catalog.max(integer) returns integer language internal;

create function pg_catalog.max(anyenum) returns anyenum language internal;

create function pg_catalog.max(oid) returns oid language internal;

create function pg_catalog.max(smallint) returns smallint language internal;

create function pg_catalog.max(bigint) returns bigint language internal;

create function pg_catalog.max(time without time zone) returns time without time zone language internal;

create function pg_catalog.max(time with time zone) returns time with time zone language internal;

create function pg_catalog.max(money) returns money language internal;

create function pg_catalog.max(timestamp without time zone) returns timestamp without time zone language internal;

create function pg_catalog.max(timestamp with time zone) returns timestamp with time zone language internal;

create function pg_catalog.max(interval) returns interval language internal;

create function pg_catalog.max(text) returns text language internal;

create function pg_catalog.max(numeric) returns numeric language internal;

create function pg_catalog.md5(text) returns text language internal;

create function pg_catalog.md5(bytea) returns text language internal;

create function pg_catalog.mic_to_big5(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.mic_to_euc_cn(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.mic_to_euc_jp(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.mic_to_euc_kr(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.mic_to_euc_tw(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.mic_to_iso(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.mic_to_koi8r(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.mic_to_latin1(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.mic_to_latin2(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.mic_to_latin3(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.mic_to_latin4(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.mic_to_sjis(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.mic_to_win1250(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.mic_to_win1251(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.mic_to_win866(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.min(timestamp without time zone) returns timestamp without time zone language internal;

create function pg_catalog.min(anyarray) returns anyarray language internal;

create function pg_catalog.min(money) returns money language internal;

create function pg_catalog.min(time with time zone) returns time with time zone language internal;

create function pg_catalog.min(record) returns record language internal;

create function pg_catalog.min(bytea) returns bytea language internal;

create function pg_catalog.min(anyenum) returns anyenum language internal;

create function pg_catalog.min(xid8) returns xid8 language internal;

create function pg_catalog.min(time without time zone) returns time without time zone language internal;

create function pg_catalog.min(date) returns date language internal;

create function pg_catalog.min(double precision) returns double precision language internal;

create function pg_catalog.min(real) returns real language internal;

create function pg_catalog.min(oid) returns oid language internal;

create function pg_catalog.min(smallint) returns smallint language internal;

create function pg_catalog.min(integer) returns integer language internal;

create function pg_catalog.min(bigint) returns bigint language internal;

create function pg_catalog.min(pg_lsn) returns pg_lsn language internal;

create function pg_catalog.min(inet) returns inet language internal;

create function pg_catalog.min(tid) returns tid language internal;

create function pg_catalog.min(text) returns text language internal;

create function pg_catalog.min(character) returns character language internal;

create function pg_catalog.min(interval) returns interval language internal;

create function pg_catalog.min(timestamp with time zone) returns timestamp with time zone language internal;

create function pg_catalog.min(numeric) returns numeric language internal;

create function pg_catalog.min_scale(numeric) returns integer language internal;

create function pg_catalog.mod(bigint, bigint) returns bigint language internal;

create function pg_catalog.mod(numeric, numeric) returns numeric language internal;

create function pg_catalog.mod(integer, integer) returns integer language internal;

create function pg_catalog.mod(smallint, smallint) returns smallint language internal;

create function pg_catalog.mode_final(internal, anyelement) returns anyelement language internal;

create function pg_catalog.money(integer) returns money language internal;

create function pg_catalog.money(bigint) returns money language internal;

create function pg_catalog.money(numeric) returns money language internal;

create function pg_catalog.mul_d_interval(double precision, interval) returns interval language internal;

create function pg_catalog.multirange(anyrange) returns anymultirange language internal;

create function pg_catalog.multirange_adjacent_multirange(anymultirange, anymultirange) returns boolean language internal;

create function pg_catalog.multirange_adjacent_range(anymultirange, anyrange) returns boolean language internal;

create function pg_catalog.multirange_after_multirange(anymultirange, anymultirange) returns boolean language internal;

create function pg_catalog.multirange_after_range(anymultirange, anyrange) returns boolean language internal;

create function pg_catalog.multirange_agg_finalfn(internal, anymultirange) returns anymultirange language internal;

create function pg_catalog.multirange_agg_transfn(internal, anymultirange) returns internal language internal;

create function pg_catalog.multirange_before_multirange(anymultirange, anymultirange) returns boolean language internal;

create function pg_catalog.multirange_before_range(anymultirange, anyrange) returns boolean language internal;

create function pg_catalog.multirange_cmp(anymultirange, anymultirange) returns integer language internal;

create function pg_catalog.multirange_contained_by_multirange(anymultirange, anymultirange) returns boolean language internal;

create function pg_catalog.multirange_contained_by_range(anymultirange, anyrange) returns boolean language internal;

create function pg_catalog.multirange_contains_elem(anymultirange, anyelement) returns boolean language internal;

create function pg_catalog.multirange_contains_multirange(anymultirange, anymultirange) returns boolean language internal;

create function pg_catalog.multirange_contains_range(anymultirange, anyrange) returns boolean language internal;

create function pg_catalog.multirange_eq(anymultirange, anymultirange) returns boolean language internal;

create function pg_catalog.multirange_ge(anymultirange, anymultirange) returns boolean language internal;

create function pg_catalog.multirange_gist_compress(internal) returns internal language internal;

create function pg_catalog.multirange_gist_consistent(internal, anymultirange, smallint, oid, internal) returns boolean language internal;

create function pg_catalog.multirange_gt(anymultirange, anymultirange) returns boolean language internal;

create function pg_catalog.multirange_in(cstring, oid, integer) returns anymultirange language internal;

create function pg_catalog.multirange_intersect(anymultirange, anymultirange) returns anymultirange language internal;

create function pg_catalog.multirange_intersect_agg_transfn(anymultirange, anymultirange) returns anymultirange language internal;

create function pg_catalog.multirange_le(anymultirange, anymultirange) returns boolean language internal;

create function pg_catalog.multirange_lt(anymultirange, anymultirange) returns boolean language internal;

create function pg_catalog.multirange_minus(anymultirange, anymultirange) returns anymultirange language internal;

create function pg_catalog.multirange_ne(anymultirange, anymultirange) returns boolean language internal;

create function pg_catalog.multirange_out(anymultirange) returns cstring language internal;

create function pg_catalog.multirange_overlaps_multirange(anymultirange, anymultirange) returns boolean language internal;

create function pg_catalog.multirange_overlaps_range(anymultirange, anyrange) returns boolean language internal;

create function pg_catalog.multirange_overleft_multirange(anymultirange, anymultirange) returns boolean language internal;

create function pg_catalog.multirange_overleft_range(anymultirange, anyrange) returns boolean language internal;

create function pg_catalog.multirange_overright_multirange(anymultirange, anymultirange) returns boolean language internal;

create function pg_catalog.multirange_overright_range(anymultirange, anyrange) returns boolean language internal;

create function pg_catalog.multirange_recv(internal, oid, integer) returns anymultirange language internal;

create function pg_catalog.multirange_send(anymultirange) returns bytea language internal;

create function pg_catalog.multirange_typanalyze(internal) returns boolean language internal;

create function pg_catalog.multirange_union(anymultirange, anymultirange) returns anymultirange language internal;

create function pg_catalog.multirangesel(internal, oid, internal, integer) returns double precision language internal;

create function pg_catalog.mxid_age(xid) returns integer language internal;

create function pg_catalog.name(character) returns name language internal;

create function pg_catalog.name(text) returns name language internal;

create function pg_catalog.name(character varying) returns name language internal;

create function pg_catalog.nameconcatoid(name, oid) returns name language internal;

create function pg_catalog.nameeq(name, name) returns boolean language internal;

create function pg_catalog.nameeqtext(name, text) returns boolean language internal;

create function pg_catalog.namege(name, name) returns boolean language internal;

create function pg_catalog.namegetext(name, text) returns boolean language internal;

create function pg_catalog.namegt(name, name) returns boolean language internal;

create function pg_catalog.namegttext(name, text) returns boolean language internal;

create function pg_catalog.nameiclike(name, text) returns boolean language internal;

create function pg_catalog.nameicnlike(name, text) returns boolean language internal;

create function pg_catalog.nameicregexeq(name, text) returns boolean language internal;

create function pg_catalog.nameicregexne(name, text) returns boolean language internal;

create function pg_catalog.namein(cstring) returns name language internal;

create function pg_catalog.namele(name, name) returns boolean language internal;

create function pg_catalog.nameletext(name, text) returns boolean language internal;

create function pg_catalog.namelike(name, text) returns boolean language internal;

create function pg_catalog.namelt(name, name) returns boolean language internal;

create function pg_catalog.namelttext(name, text) returns boolean language internal;

create function pg_catalog.namene(name, name) returns boolean language internal;

create function pg_catalog.namenetext(name, text) returns boolean language internal;

create function pg_catalog.namenlike(name, text) returns boolean language internal;

create function pg_catalog.nameout(name) returns cstring language internal;

create function pg_catalog.namerecv(internal) returns name language internal;

create function pg_catalog.nameregexeq(name, text) returns boolean language internal;

create function pg_catalog.nameregexne(name, text) returns boolean language internal;

create function pg_catalog.namesend(name) returns bytea language internal;

create function pg_catalog.neqjoinsel(internal, oid, internal, smallint, internal) returns double precision language internal;

create function pg_catalog.neqsel(internal, oid, internal, integer) returns double precision language internal;

create function pg_catalog.netmask(inet) returns inet language internal;

create function pg_catalog.network(inet) returns cidr language internal;

create function pg_catalog.network_cmp(inet, inet) returns integer language internal;

create function pg_catalog.network_eq(inet, inet) returns boolean language internal;

create function pg_catalog.network_ge(inet, inet) returns boolean language internal;

create function pg_catalog.network_gt(inet, inet) returns boolean language internal;

create function pg_catalog.network_larger(inet, inet) returns inet language internal;

create function pg_catalog.network_le(inet, inet) returns boolean language internal;

create function pg_catalog.network_lt(inet, inet) returns boolean language internal;

create function pg_catalog.network_ne(inet, inet) returns boolean language internal;

create function pg_catalog.network_overlap(inet, inet) returns boolean language internal;

create function pg_catalog.network_smaller(inet, inet) returns inet language internal;

create function pg_catalog.network_sortsupport(internal) returns void language internal;

create function pg_catalog.network_sub(inet, inet) returns boolean language internal;

create function pg_catalog.network_subeq(inet, inet) returns boolean language internal;

create function pg_catalog.network_subset_support(internal) returns internal language internal;

create function pg_catalog.network_sup(inet, inet) returns boolean language internal;

create function pg_catalog.network_supeq(inet, inet) returns boolean language internal;

create function pg_catalog.networkjoinsel(internal, oid, internal, smallint, internal) returns double precision language internal;

create function pg_catalog.networksel(internal, oid, internal, integer) returns double precision language internal;

create function pg_catalog.nextval(regclass) returns bigint language internal;

create function pg_catalog.nlikejoinsel(internal, oid, internal, smallint, internal) returns double precision language internal;

create function pg_catalog.nlikesel(internal, oid, internal, integer) returns double precision language internal;

create function pg_catalog.normalize(text, text DEFAULT 'NFC'::text) returns text language internal;

create function pg_catalog.notlike(text, text) returns boolean language internal;

create function pg_catalog.notlike(bytea, bytea) returns boolean language internal;

create function pg_catalog.notlike(name, text) returns boolean language internal;

create function pg_catalog.now() returns timestamp with time zone language internal;

create function pg_catalog.npoints(polygon) returns integer language internal;

create function pg_catalog.npoints(path) returns integer language internal;

create function pg_catalog.nth_value(anyelement, integer) returns anyelement language internal;

create function pg_catalog.ntile(integer) returns integer language internal;

create function pg_catalog.num_nonnulls(VARIADIC "any") returns integer language internal;

create function pg_catalog.num_nulls(VARIADIC "any") returns integer language internal;

create function pg_catalog.numeric(bigint) returns numeric language internal;

create function pg_catalog.numeric(money) returns numeric language internal;

create function pg_catalog.numeric(jsonb) returns numeric language internal;

create function pg_catalog.numeric(numeric, integer) returns numeric language internal;

create function pg_catalog.numeric(double precision) returns numeric language internal;

create function pg_catalog.numeric(real) returns numeric language internal;

create function pg_catalog.numeric(integer) returns numeric language internal;

create function pg_catalog.numeric(smallint) returns numeric language internal;

create function pg_catalog.numeric_abs(numeric) returns numeric language internal;

create function pg_catalog.numeric_accum(internal, numeric) returns internal language internal;

create function pg_catalog.numeric_accum_inv(internal, numeric) returns internal language internal;

create function pg_catalog.numeric_add(numeric, numeric) returns numeric language internal;

create function pg_catalog.numeric_avg(internal) returns numeric language internal;

create function pg_catalog.numeric_avg_accum(internal, numeric) returns internal language internal;

create function pg_catalog.numeric_avg_combine(internal, internal) returns internal language internal;

create function pg_catalog.numeric_avg_deserialize(bytea, internal) returns internal language internal;

create function pg_catalog.numeric_avg_serialize(internal) returns bytea language internal;

create function pg_catalog.numeric_cmp(numeric, numeric) returns integer language internal;

create function pg_catalog.numeric_combine(internal, internal) returns internal language internal;

create function pg_catalog.numeric_deserialize(bytea, internal) returns internal language internal;

create function pg_catalog.numeric_div(numeric, numeric) returns numeric language internal;

create function pg_catalog.numeric_div_trunc(numeric, numeric) returns numeric language internal;

create function pg_catalog.numeric_eq(numeric, numeric) returns boolean language internal;

create function pg_catalog.numeric_exp(numeric) returns numeric language internal;

create function pg_catalog.numeric_ge(numeric, numeric) returns boolean language internal;

create function pg_catalog.numeric_gt(numeric, numeric) returns boolean language internal;

create function pg_catalog.numeric_in(cstring, oid, integer) returns numeric language internal;

create function pg_catalog.numeric_inc(numeric) returns numeric language internal;

create function pg_catalog.numeric_larger(numeric, numeric) returns numeric language internal;

create function pg_catalog.numeric_le(numeric, numeric) returns boolean language internal;

create function pg_catalog.numeric_ln(numeric) returns numeric language internal;

create function pg_catalog.numeric_log(numeric, numeric) returns numeric language internal;

create function pg_catalog.numeric_lt(numeric, numeric) returns boolean language internal;

create function pg_catalog.numeric_mod(numeric, numeric) returns numeric language internal;

create function pg_catalog.numeric_mul(numeric, numeric) returns numeric language internal;

create function pg_catalog.numeric_ne(numeric, numeric) returns boolean language internal;

create function pg_catalog.numeric_out(numeric) returns cstring language internal;

create function pg_catalog.numeric_pl_pg_lsn(numeric, pg_lsn) returns pg_lsn language internal;

create function pg_catalog.numeric_poly_avg(internal) returns numeric language internal;

create function pg_catalog.numeric_poly_combine(internal, internal) returns internal language internal;

create function pg_catalog.numeric_poly_deserialize(bytea, internal) returns internal language internal;

create function pg_catalog.numeric_poly_serialize(internal) returns bytea language internal;

create function pg_catalog.numeric_poly_stddev_pop(internal) returns numeric language internal;

create function pg_catalog.numeric_poly_stddev_samp(internal) returns numeric language internal;

create function pg_catalog.numeric_poly_sum(internal) returns numeric language internal;

create function pg_catalog.numeric_poly_var_pop(internal) returns numeric language internal;

create function pg_catalog.numeric_poly_var_samp(internal) returns numeric language internal;

create function pg_catalog.numeric_power(numeric, numeric) returns numeric language internal;

create function pg_catalog.numeric_recv(internal, oid, integer) returns numeric language internal;

create function pg_catalog.numeric_send(numeric) returns bytea language internal;

create function pg_catalog.numeric_serialize(internal) returns bytea language internal;

create function pg_catalog.numeric_smaller(numeric, numeric) returns numeric language internal;

create function pg_catalog.numeric_sortsupport(internal) returns void language internal;

create function pg_catalog.numeric_sqrt(numeric) returns numeric language internal;

create function pg_catalog.numeric_stddev_pop(internal) returns numeric language internal;

create function pg_catalog.numeric_stddev_samp(internal) returns numeric language internal;

create function pg_catalog.numeric_sub(numeric, numeric) returns numeric language internal;

create function pg_catalog.numeric_sum(internal) returns numeric language internal;

create function pg_catalog.numeric_support(internal) returns internal language internal;

create function pg_catalog.numeric_uminus(numeric) returns numeric language internal;

create function pg_catalog.numeric_uplus(numeric) returns numeric language internal;

create function pg_catalog.numeric_var_pop(internal) returns numeric language internal;

create function pg_catalog.numeric_var_samp(internal) returns numeric language internal;

create function pg_catalog.numerictypmodin(cstring[]) returns integer language internal;

create function pg_catalog.numerictypmodout(integer) returns cstring language internal;

create function pg_catalog.nummultirange(numrange) returns nummultirange language internal;

create function pg_catalog.nummultirange() returns nummultirange language internal;

create function pg_catalog.nummultirange(VARIADIC numrange[]) returns nummultirange language internal;

create function pg_catalog.numnode(tsquery) returns integer language internal;

create function pg_catalog.numrange(numeric, numeric) returns numrange language internal;

create function pg_catalog.numrange(numeric, numeric, text) returns numrange language internal;

create function pg_catalog.numrange_subdiff(numeric, numeric) returns double precision language internal;

create function pg_catalog.obj_description(oid) returns text language internal;

create function pg_catalog.obj_description(oid, name) returns text language internal;

create function pg_catalog.octet_length(text) returns integer language internal;

create function pg_catalog.octet_length(bytea) returns integer language internal;

create function pg_catalog.octet_length(character) returns integer language internal;

create function pg_catalog.octet_length(bit) returns integer language internal;

create function pg_catalog.oid(bigint) returns oid language internal;

create function pg_catalog.oideq(oid, oid) returns boolean language internal;

create function pg_catalog.oidge(oid, oid) returns boolean language internal;

create function pg_catalog.oidgt(oid, oid) returns boolean language internal;

create function pg_catalog.oidin(cstring) returns oid language internal;

create function pg_catalog.oidlarger(oid, oid) returns oid language internal;

create function pg_catalog.oidle(oid, oid) returns boolean language internal;

create function pg_catalog.oidlt(oid, oid) returns boolean language internal;

create function pg_catalog.oidne(oid, oid) returns boolean language internal;

create function pg_catalog.oidout(oid) returns cstring language internal;

create function pg_catalog.oidrecv(internal) returns oid language internal;

create function pg_catalog.oidsend(oid) returns bytea language internal;

create function pg_catalog.oidsmaller(oid, oid) returns oid language internal;

create function pg_catalog.oidvectoreq(oidvector, oidvector) returns boolean language internal;

create function pg_catalog.oidvectorge(oidvector, oidvector) returns boolean language internal;

create function pg_catalog.oidvectorgt(oidvector, oidvector) returns boolean language internal;

create function pg_catalog.oidvectorin(cstring) returns oidvector language internal;

create function pg_catalog.oidvectorle(oidvector, oidvector) returns boolean language internal;

create function pg_catalog.oidvectorlt(oidvector, oidvector) returns boolean language internal;

create function pg_catalog.oidvectorne(oidvector, oidvector) returns boolean language internal;

create function pg_catalog.oidvectorout(oidvector) returns cstring language internal;

create function pg_catalog.oidvectorrecv(internal) returns oidvector language internal;

create function pg_catalog.oidvectorsend(oidvector) returns bytea language internal;

create function pg_catalog.oidvectortypes(oidvector) returns text language internal;

create function pg_catalog.on_pb(point, box) returns boolean language internal;

create function pg_catalog.on_pl(point, line) returns boolean language internal;

create function pg_catalog.on_ppath(point, path) returns boolean language internal;

create function pg_catalog.on_ps(point, lseg) returns boolean language internal;

create function pg_catalog.on_sb(lseg, box) returns boolean language internal;

create function pg_catalog.on_sl(lseg, line) returns boolean language internal;

create function pg_catalog.ordered_set_transition(internal, "any") returns internal language internal;

create function pg_catalog.ordered_set_transition_multi(internal, VARIADIC "any") returns internal language internal;

create function pg_catalog.overlaps(time without time zone, time without time zone, time without time zone, time without time zone) returns boolean language internal;

create function pg_catalog.overlaps(timestamp with time zone, interval, timestamp with time zone, interval) returns boolean language internal;

create function pg_catalog.overlaps(timestamp with time zone, interval, timestamp with time zone, timestamp with time zone) returns boolean language internal;

create function pg_catalog.overlaps(time with time zone, time with time zone, time with time zone, time with time zone) returns boolean language internal;

create function pg_catalog.overlaps(timestamp without time zone, timestamp without time zone, timestamp without time zone, interval) returns boolean language internal;

create function pg_catalog.overlaps(timestamp with time zone, timestamp with time zone, timestamp with time zone, timestamp with time zone) returns boolean language internal;

create function pg_catalog.overlaps(time without time zone, interval, time without time zone, time without time zone) returns boolean language internal;

create function pg_catalog.overlaps(timestamp without time zone, interval, timestamp without time zone, timestamp without time zone) returns boolean language internal;

create function pg_catalog.overlaps(timestamp without time zone, interval, timestamp without time zone, interval) returns boolean language internal;

create function pg_catalog.overlaps(timestamp with time zone, timestamp with time zone, timestamp with time zone, interval) returns boolean language internal;

create function pg_catalog.overlaps(time without time zone, time without time zone, time without time zone, interval) returns boolean language internal;

create function pg_catalog.overlaps(time without time zone, interval, time without time zone, interval) returns boolean language internal;

create function pg_catalog.overlaps(timestamp without time zone, timestamp without time zone, timestamp without time zone, timestamp without time zone) returns boolean language internal;

create function pg_catalog.overlay(bytea, bytea, integer, integer) returns bytea language internal;

create function pg_catalog.overlay(bytea, bytea, integer) returns bytea language internal;

create function pg_catalog.overlay(text, text, integer) returns text language internal;

create function pg_catalog.overlay(bit, bit, integer, integer) returns bit language internal;

create function pg_catalog.overlay(text, text, integer, integer) returns text language internal;

create function pg_catalog.overlay(bit, bit, integer) returns bit language internal;

create function pg_catalog.parse_ident(str text, strict boolean DEFAULT true) returns text[] language internal;

create function pg_catalog.path(polygon) returns path language internal;

create function pg_catalog.path_add(path, path) returns path language internal;

create function pg_catalog.path_add_pt(path, point) returns path language internal;

create function pg_catalog.path_contain_pt(path, point) returns boolean language internal;

create function pg_catalog.path_distance(path, path) returns double precision language internal;

create function pg_catalog.path_div_pt(path, point) returns path language internal;

create function pg_catalog.path_in(cstring) returns path language internal;

create function pg_catalog.path_inter(path, path) returns boolean language internal;

create function pg_catalog.path_length(path) returns double precision language internal;

create function pg_catalog.path_mul_pt(path, point) returns path language internal;

create function pg_catalog.path_n_eq(path, path) returns boolean language internal;

create function pg_catalog.path_n_ge(path, path) returns boolean language internal;

create function pg_catalog.path_n_gt(path, path) returns boolean language internal;

create function pg_catalog.path_n_le(path, path) returns boolean language internal;

create function pg_catalog.path_n_lt(path, path) returns boolean language internal;

create function pg_catalog.path_npoints(path) returns integer language internal;

create function pg_catalog.path_out(path) returns cstring language internal;

create function pg_catalog.path_recv(internal) returns path language internal;

create function pg_catalog.path_send(path) returns bytea language internal;

create function pg_catalog.path_sub_pt(path, point) returns path language internal;

create function pg_catalog.pclose(path) returns path language internal;

create function pg_catalog.percent_rank() returns double precision language internal;

create function pg_catalog.percent_rank_final(internal, VARIADIC "any") returns double precision language internal;

create function pg_catalog.percentile_cont_float8_final(internal, double precision) returns double precision language internal;

create function pg_catalog.percentile_cont_float8_multi_final(internal, double precision[]) returns double precision[] language internal;

create function pg_catalog.percentile_cont_interval_final(internal, double precision) returns interval language internal;

create function pg_catalog.percentile_cont_interval_multi_final(internal, double precision[]) returns interval[] language internal;

create function pg_catalog.percentile_disc_final(internal, double precision, anyelement) returns anyelement language internal;

create function pg_catalog.percentile_disc_multi_final(internal, double precision[], anyelement) returns anyarray language internal;

create function pg_catalog.pg_advisory_lock(bigint) returns void language internal;

create function pg_catalog.pg_advisory_lock(integer, integer) returns void language internal;

create function pg_catalog.pg_advisory_lock_shared(bigint) returns void language internal;

create function pg_catalog.pg_advisory_lock_shared(integer, integer) returns void language internal;

create function pg_catalog.pg_advisory_unlock(integer, integer) returns boolean language internal;

create function pg_catalog.pg_advisory_unlock(bigint) returns boolean language internal;

create function pg_catalog.pg_advisory_unlock_all() returns void language internal;

create function pg_catalog.pg_advisory_unlock_shared(integer, integer) returns boolean language internal;

create function pg_catalog.pg_advisory_unlock_shared(bigint) returns boolean language internal;

create function pg_catalog.pg_advisory_xact_lock(integer, integer) returns void language internal;

create function pg_catalog.pg_advisory_xact_lock(bigint) returns void language internal;

create function pg_catalog.pg_advisory_xact_lock_shared(integer, integer) returns void language internal;

create function pg_catalog.pg_advisory_xact_lock_shared(bigint) returns void language internal;

create function pg_catalog.pg_available_extension_versions(OUT name name, OUT version text, OUT superuser boolean, OUT trusted boolean, OUT relocatable boolean, OUT schema name, OUT requires name[], OUT comment text) returns SETOF record language internal;

create function pg_catalog.pg_available_extensions(OUT name name, OUT default_version text, OUT comment text) returns SETOF record language internal;

create function pg_catalog.pg_available_wal_summaries(OUT tli bigint, OUT start_lsn pg_lsn, OUT end_lsn pg_lsn) returns SETOF record language internal;

create function pg_catalog.pg_backend_pid() returns integer language internal;

create function pg_catalog.pg_backup_start(label text, fast boolean DEFAULT false) returns pg_lsn language internal;

create function pg_catalog.pg_backup_stop(wait_for_archive boolean DEFAULT true, OUT lsn pg_lsn, OUT labelfile text, OUT spcmapfile text) returns record language internal;

create function pg_catalog.pg_basetype(regtype) returns regtype language internal;

create function pg_catalog.pg_blocking_pids(integer) returns integer[] language internal;

create function pg_catalog.pg_cancel_backend(integer) returns boolean language internal;

create function pg_catalog.pg_char_to_encoding(name) returns integer language internal;

create function pg_catalog.pg_clear_attribute_stats(schemaname text, relname text, attname text, inherited boolean) returns void language internal;

create function pg_catalog.pg_clear_relation_stats(schemaname text, relname text) returns void language internal;

create function pg_catalog.pg_client_encoding() returns name language internal;

create function pg_catalog.pg_collation_actual_version(oid) returns text language internal;

create function pg_catalog.pg_collation_for("any") returns text language internal;

create function pg_catalog.pg_collation_is_visible(oid) returns boolean language internal;

create function pg_catalog.pg_column_compression("any") returns text language internal;

create function pg_catalog.pg_column_is_updatable(regclass, smallint, boolean) returns boolean language internal;

create function pg_catalog.pg_column_size("any") returns integer language internal;

create function pg_catalog.pg_column_toast_chunk_id("any") returns oid language internal;

create function pg_catalog.pg_conf_load_time() returns timestamp with time zone language internal;

create function pg_catalog.pg_config(OUT name text, OUT setting text) returns SETOF record language internal;

create function pg_catalog.pg_control_checkpoint(OUT checkpoint_lsn pg_lsn, OUT redo_lsn pg_lsn, OUT redo_wal_file text, OUT timeline_id integer, OUT prev_timeline_id integer, OUT full_page_writes boolean, OUT next_xid text, OUT next_oid oid, OUT next_multixact_id xid, OUT next_multi_offset xid, OUT oldest_xid xid, OUT oldest_xid_dbid oid, OUT oldest_active_xid xid, OUT oldest_multi_xid xid, OUT oldest_multi_dbid oid, OUT oldest_commit_ts_xid xid, OUT newest_commit_ts_xid xid, OUT checkpoint_time timestamp with time zone) returns record language internal;

create function pg_catalog.pg_control_init(OUT max_data_alignment integer, OUT database_block_size integer, OUT blocks_per_segment integer, OUT wal_block_size integer, OUT bytes_per_wal_segment integer, OUT max_identifier_length integer, OUT max_index_columns integer, OUT max_toast_chunk_size integer, OUT large_object_chunk_size integer, OUT float8_pass_by_value boolean, OUT data_page_checksum_version integer, OUT default_char_signedness boolean) returns record language internal;

create function pg_catalog.pg_control_recovery(OUT min_recovery_end_lsn pg_lsn, OUT min_recovery_end_timeline integer, OUT backup_start_lsn pg_lsn, OUT backup_end_lsn pg_lsn, OUT end_of_backup_record_required boolean) returns record language internal;

create function pg_catalog.pg_control_system(OUT pg_control_version integer, OUT catalog_version_no integer, OUT system_identifier bigint, OUT pg_control_last_modified timestamp with time zone) returns record language internal;

create function pg_catalog.pg_conversion_is_visible(oid) returns boolean language internal;

create function pg_catalog.pg_copy_logical_replication_slot(src_slot_name name, dst_slot_name name, temporary boolean, plugin name, OUT slot_name name, OUT lsn pg_lsn) returns record language internal;

create function pg_catalog.pg_copy_logical_replication_slot(src_slot_name name, dst_slot_name name, OUT slot_name name, OUT lsn pg_lsn) returns record language internal;

create function pg_catalog.pg_copy_logical_replication_slot(src_slot_name name, dst_slot_name name, temporary boolean, OUT slot_name name, OUT lsn pg_lsn) returns record language internal;

create function pg_catalog.pg_copy_physical_replication_slot(src_slot_name name, dst_slot_name name, temporary boolean, OUT slot_name name, OUT lsn pg_lsn) returns record language internal;

create function pg_catalog.pg_copy_physical_replication_slot(src_slot_name name, dst_slot_name name, OUT slot_name name, OUT lsn pg_lsn) returns record language internal;

create function pg_catalog.pg_create_logical_replication_slot(slot_name name, plugin name, temporary boolean DEFAULT false, twophase boolean DEFAULT false, failover boolean DEFAULT false, OUT slot_name name, OUT lsn pg_lsn) returns record language internal;

create function pg_catalog.pg_create_physical_replication_slot(slot_name name, immediately_reserve boolean DEFAULT false, temporary boolean DEFAULT false, OUT slot_name name, OUT lsn pg_lsn) returns record language internal;

create function pg_catalog.pg_create_restore_point(text) returns pg_lsn language internal;

create function pg_catalog.pg_current_logfile(text) returns text language internal;

create function pg_catalog.pg_current_logfile() returns text language internal;

create function pg_catalog.pg_current_snapshot() returns pg_snapshot language internal;

create function pg_catalog.pg_current_wal_flush_lsn() returns pg_lsn language internal;

create function pg_catalog.pg_current_wal_insert_lsn() returns pg_lsn language internal;

create function pg_catalog.pg_current_wal_lsn() returns pg_lsn language internal;

create function pg_catalog.pg_current_xact_id() returns xid8 language internal;

create function pg_catalog.pg_current_xact_id_if_assigned() returns xid8 language internal;

create function pg_catalog.pg_cursor(OUT name text, OUT statement text, OUT is_holdable boolean, OUT is_binary boolean, OUT is_scrollable boolean, OUT creation_time timestamp with time zone) returns SETOF record language internal;

create function pg_catalog.pg_database_collation_actual_version(oid) returns text language internal;

create function pg_catalog.pg_database_size(name) returns bigint language internal;

create function pg_catalog.pg_database_size(oid) returns bigint language internal;

create function pg_catalog.pg_ddl_command_in(cstring) returns pg_ddl_command language internal;

create function pg_catalog.pg_ddl_command_out(pg_ddl_command) returns cstring language internal;

create function pg_catalog.pg_ddl_command_recv(internal) returns pg_ddl_command language internal;

create function pg_catalog.pg_ddl_command_send(pg_ddl_command) returns bytea language internal;

create function pg_catalog.pg_dependencies_in(cstring) returns pg_dependencies language internal;

create function pg_catalog.pg_dependencies_out(pg_dependencies) returns cstring language internal;

create function pg_catalog.pg_dependencies_recv(internal) returns pg_dependencies language internal;

create function pg_catalog.pg_dependencies_send(pg_dependencies) returns bytea language internal;

create function pg_catalog.pg_describe_object(oid, oid, integer) returns text language internal;

create function pg_catalog.pg_drop_replication_slot(name) returns void language internal;

create function pg_catalog.pg_encoding_max_length(integer) returns integer language internal;

create function pg_catalog.pg_encoding_to_char(integer) returns name language internal;

create function pg_catalog.pg_event_trigger_ddl_commands(OUT classid oid, OUT objid oid, OUT objsubid integer, OUT command_tag text, OUT object_type text, OUT schema_name text, OUT object_identity text, OUT in_extension boolean, OUT command pg_ddl_command) returns SETOF record language internal;

create function pg_catalog.pg_event_trigger_dropped_objects(OUT classid oid, OUT objid oid, OUT objsubid integer, OUT original boolean, OUT normal boolean, OUT is_temporary boolean, OUT object_type text, OUT schema_name text, OUT object_name text, OUT object_identity text, OUT address_names text[], OUT address_args text[]) returns SETOF record language internal;

create function pg_catalog.pg_event_trigger_table_rewrite_oid(OUT oid oid) returns oid language internal;

create function pg_catalog.pg_event_trigger_table_rewrite_reason() returns integer language internal;

create function pg_catalog.pg_export_snapshot() returns text language internal;

create function pg_catalog.pg_extension_config_dump(regclass, text) returns void language internal;

create function pg_catalog.pg_extension_update_paths(name name, OUT source text, OUT target text, OUT path text) returns SETOF record language internal;

create function pg_catalog.pg_filenode_relation(oid, oid) returns regclass language internal;

create function pg_catalog.pg_function_is_visible(oid) returns boolean language internal;

create function pg_catalog.pg_get_acl(classid oid, objid oid, objsubid integer) returns aclitem[] language internal;

create function pg_catalog.pg_get_aios(OUT pid integer, OUT io_id integer, OUT io_generation bigint, OUT state text, OUT operation text, OUT off bigint, OUT length bigint, OUT target text, OUT handle_data_len smallint, OUT raw_result integer, OUT result text, OUT target_desc text, OUT f_sync boolean, OUT f_localmem boolean, OUT f_buffered boolean) returns SETOF record language internal;

create function pg_catalog.pg_get_backend_memory_contexts(OUT name text, OUT ident text, OUT type text, OUT level integer, OUT path integer[], OUT total_bytes bigint, OUT total_nblocks bigint, OUT free_bytes bigint, OUT free_chunks bigint, OUT used_bytes bigint) returns SETOF record language internal;

create function pg_catalog.pg_get_catalog_foreign_keys(OUT fktable regclass, OUT fkcols text[], OUT pktable regclass, OUT pkcols text[], OUT is_array boolean, OUT is_opt boolean) returns SETOF record language internal;

create function pg_catalog.pg_get_constraintdef(oid) returns text language internal;

create function pg_catalog.pg_get_constraintdef(oid, boolean) returns text language internal;

create function pg_catalog.pg_get_expr(pg_node_tree, oid) returns text language internal;

create function pg_catalog.pg_get_expr(pg_node_tree, oid, boolean) returns text language internal;

create function pg_catalog.pg_get_function_arg_default(oid, integer) returns text language internal;

create function pg_catalog.pg_get_function_arguments(oid) returns text language internal;

create function pg_catalog.pg_get_function_identity_arguments(oid) returns text language internal;

create function pg_catalog.pg_get_function_result(oid) returns text language internal;

create function pg_catalog.pg_get_function_sqlbody(oid) returns text language internal;

create function pg_catalog.pg_get_functiondef(oid) returns text language internal;

create function pg_catalog.pg_get_indexdef(oid) returns text language internal;

create function pg_catalog.pg_get_indexdef(oid, integer, boolean) returns text language internal;

create function pg_catalog.pg_get_keywords(OUT word text, OUT catcode "char", OUT barelabel boolean, OUT catdesc text, OUT baredesc text) returns SETOF record language internal;

create function pg_catalog.pg_get_loaded_modules(OUT module_name text, OUT version text, OUT file_name text) returns SETOF record language internal;

create function pg_catalog.pg_get_multixact_members(multixid xid, OUT xid xid, OUT mode text) returns SETOF record language internal;

create function pg_catalog.pg_get_object_address(type text, object_names text[], object_args text[], OUT classid oid, OUT objid oid, OUT objsubid integer) returns record language internal;

create function pg_catalog.pg_get_partition_constraintdef(oid) returns text language internal;

create function pg_catalog.pg_get_partkeydef(oid) returns text language internal;

create function pg_catalog.pg_get_publication_tables(VARIADIC pubname text[], OUT pubid oid, OUT relid oid, OUT attrs int2vector, OUT qual pg_node_tree) returns SETOF record language internal;

create function pg_catalog.pg_get_replica_identity_index(regclass) returns regclass language internal;

create function pg_catalog.pg_get_replication_slots(OUT slot_name name, OUT plugin name, OUT slot_type text, OUT datoid oid, OUT temporary boolean, OUT active boolean, OUT active_pid integer, OUT xmin xid, OUT catalog_xmin xid, OUT restart_lsn pg_lsn, OUT confirmed_flush_lsn pg_lsn, OUT wal_status text, OUT safe_wal_size bigint, OUT two_phase boolean, OUT two_phase_at pg_lsn, OUT inactive_since timestamp with time zone, OUT conflicting boolean, OUT invalidation_reason text, OUT failover boolean, OUT synced boolean) returns SETOF record language internal;

create function pg_catalog.pg_get_ruledef(oid) returns text language internal;

create function pg_catalog.pg_get_ruledef(oid, boolean) returns text language internal;

create function pg_catalog.pg_get_sequence_data(sequence_oid regclass, OUT last_value bigint, OUT is_called boolean) returns record language internal;

create function pg_catalog.pg_get_serial_sequence(text, text) returns text language internal;

create function pg_catalog.pg_get_shmem_allocations(OUT name text, OUT off bigint, OUT size bigint, OUT allocated_size bigint) returns SETOF record language internal;

create function pg_catalog.pg_get_shmem_allocations_numa(OUT name text, OUT numa_node integer, OUT size bigint) returns SETOF record language internal;

create function pg_catalog.pg_get_statisticsobjdef(oid) returns text language internal;

create function pg_catalog.pg_get_statisticsobjdef_columns(oid) returns text language internal;

create function pg_catalog.pg_get_statisticsobjdef_expressions(oid) returns text[] language internal;

create function pg_catalog.pg_get_triggerdef(oid) returns text language internal;

create function pg_catalog.pg_get_triggerdef(oid, boolean) returns text language internal;

create function pg_catalog.pg_get_userbyid(oid) returns name language internal;

create function pg_catalog.pg_get_viewdef(oid, boolean) returns text language internal;

create function pg_catalog.pg_get_viewdef(oid, integer) returns text language internal;

create function pg_catalog.pg_get_viewdef(text) returns text language internal;

create function pg_catalog.pg_get_viewdef(oid) returns text language internal;

create function pg_catalog.pg_get_viewdef(text, boolean) returns text language internal;

create function pg_catalog.pg_get_wait_events(OUT type text, OUT name text, OUT description text) returns SETOF record language internal;

create function pg_catalog.pg_get_wal_replay_pause_state() returns text language internal;

create function pg_catalog.pg_get_wal_resource_managers(OUT rm_id integer, OUT rm_name text, OUT rm_builtin boolean) returns SETOF record language internal;

create function pg_catalog.pg_get_wal_summarizer_state(OUT summarized_tli bigint, OUT summarized_lsn pg_lsn, OUT pending_lsn pg_lsn, OUT summarizer_pid integer) returns record language internal;

create function pg_catalog.pg_has_role(name, name, text) returns boolean language internal;

create function pg_catalog.pg_has_role(name, text) returns boolean language internal;

create function pg_catalog.pg_has_role(oid, text) returns boolean language internal;

create function pg_catalog.pg_has_role(name, oid, text) returns boolean language internal;

create function pg_catalog.pg_has_role(oid, name, text) returns boolean language internal;

create function pg_catalog.pg_has_role(oid, oid, text) returns boolean language internal;

create function pg_catalog.pg_hba_file_rules(OUT rule_number integer, OUT file_name text, OUT line_number integer, OUT type text, OUT database text[], OUT user_name text[], OUT address text, OUT netmask text, OUT auth_method text, OUT options text[], OUT error text) returns SETOF record language internal;

create function pg_catalog.pg_ident_file_mappings(OUT map_number integer, OUT file_name text, OUT line_number integer, OUT map_name text, OUT sys_name text, OUT pg_username text, OUT error text) returns SETOF record language internal;

create function pg_catalog.pg_identify_object(classid oid, objid oid, objsubid integer, OUT type text, OUT schema text, OUT name text, OUT identity text) returns record language internal;

create function pg_catalog.pg_identify_object_as_address(classid oid, objid oid, objsubid integer, OUT type text, OUT object_names text[], OUT object_args text[]) returns record language internal;

create function pg_catalog.pg_import_system_collations(regnamespace) returns integer language internal;

create function pg_catalog.pg_index_column_has_property(regclass, integer, text) returns boolean language internal;

create function pg_catalog.pg_index_has_property(regclass, text) returns boolean language internal;

create function pg_catalog.pg_indexam_has_property(oid, text) returns boolean language internal;

create function pg_catalog.pg_indexam_progress_phasename(oid, bigint) returns text language internal;

create function pg_catalog.pg_indexes_size(regclass) returns bigint language internal;

create function pg_catalog.pg_input_error_info(value text, type_name text, OUT message text, OUT detail text, OUT hint text, OUT sql_error_code text) returns record language internal;

create function pg_catalog.pg_input_is_valid(text, text) returns boolean language internal;

create function pg_catalog.pg_is_in_recovery() returns boolean language internal;

create function pg_catalog.pg_is_other_temp_schema(oid) returns boolean language internal;

create function pg_catalog.pg_is_wal_replay_paused() returns boolean language internal;

create function pg_catalog.pg_isolation_test_session_is_blocked(integer, integer[]) returns boolean language internal;

create function pg_catalog.pg_jit_available() returns boolean language internal;

create function pg_catalog.pg_last_committed_xact(OUT xid xid, OUT "timestamp" timestamp with time zone, OUT roident oid) returns record language internal;

create function pg_catalog.pg_last_wal_receive_lsn() returns pg_lsn language internal;

create function pg_catalog.pg_last_wal_replay_lsn() returns pg_lsn language internal;

create function pg_catalog.pg_last_xact_replay_timestamp() returns timestamp with time zone language internal;

create function pg_catalog.pg_listening_channels() returns SETOF text language internal;

create function pg_catalog.pg_lock_status(OUT locktype text, OUT database oid, OUT relation oid, OUT page integer, OUT tuple smallint, OUT virtualxid text, OUT transactionid xid, OUT classid oid, OUT objid oid, OUT objsubid smallint, OUT virtualtransaction text, OUT pid integer, OUT mode text, OUT granted boolean, OUT fastpath boolean, OUT waitstart timestamp with time zone) returns SETOF record language internal;

create function pg_catalog.pg_log_backend_memory_contexts(integer) returns boolean language internal;

create function pg_catalog.pg_log_standby_snapshot() returns pg_lsn language internal;

create function pg_catalog.pg_logical_emit_message(transactional boolean, prefix text, message bytea, flush boolean DEFAULT false) returns pg_lsn language internal;

create function pg_catalog.pg_logical_emit_message(transactional boolean, prefix text, message text, flush boolean DEFAULT false) returns pg_lsn language internal;

create function pg_catalog.pg_logical_slot_get_binary_changes(slot_name name, upto_lsn pg_lsn, upto_nchanges integer, VARIADIC options text[] DEFAULT '{}'::text[], OUT lsn pg_lsn, OUT xid xid, OUT data bytea) returns SETOF record language internal;

create function pg_catalog.pg_logical_slot_get_changes(slot_name name, upto_lsn pg_lsn, upto_nchanges integer, VARIADIC options text[] DEFAULT '{}'::text[], OUT lsn pg_lsn, OUT xid xid, OUT data text) returns SETOF record language internal;

create function pg_catalog.pg_logical_slot_peek_binary_changes(slot_name name, upto_lsn pg_lsn, upto_nchanges integer, VARIADIC options text[] DEFAULT '{}'::text[], OUT lsn pg_lsn, OUT xid xid, OUT data bytea) returns SETOF record language internal;

create function pg_catalog.pg_logical_slot_peek_changes(slot_name name, upto_lsn pg_lsn, upto_nchanges integer, VARIADIC options text[] DEFAULT '{}'::text[], OUT lsn pg_lsn, OUT xid xid, OUT data text) returns SETOF record language internal;

create function pg_catalog.pg_ls_archive_statusdir(OUT name text, OUT size bigint, OUT modification timestamp with time zone) returns SETOF record language internal;

create function pg_catalog.pg_ls_dir(text, boolean, boolean) returns SETOF text language internal;

create function pg_catalog.pg_ls_dir(text) returns SETOF text language internal;

create function pg_catalog.pg_ls_logdir(OUT name text, OUT size bigint, OUT modification timestamp with time zone) returns SETOF record language internal;

create function pg_catalog.pg_ls_logicalmapdir(OUT name text, OUT size bigint, OUT modification timestamp with time zone) returns SETOF record language internal;

create function pg_catalog.pg_ls_logicalsnapdir(OUT name text, OUT size bigint, OUT modification timestamp with time zone) returns SETOF record language internal;

create function pg_catalog.pg_ls_replslotdir(slot_name text, OUT name text, OUT size bigint, OUT modification timestamp with time zone) returns SETOF record language internal;

create function pg_catalog.pg_ls_summariesdir(OUT name text, OUT size bigint, OUT modification timestamp with time zone) returns SETOF record language internal;

create function pg_catalog.pg_ls_tmpdir(OUT name text, OUT size bigint, OUT modification timestamp with time zone) returns SETOF record language internal;

create function pg_catalog.pg_ls_tmpdir(tablespace oid, OUT name text, OUT size bigint, OUT modification timestamp with time zone) returns SETOF record language internal;

create function pg_catalog.pg_ls_waldir(OUT name text, OUT size bigint, OUT modification timestamp with time zone) returns SETOF record language internal;

create function pg_catalog.pg_lsn(numeric) returns pg_lsn language internal;

create function pg_catalog.pg_lsn_cmp(pg_lsn, pg_lsn) returns integer language internal;

create function pg_catalog.pg_lsn_eq(pg_lsn, pg_lsn) returns boolean language internal;

create function pg_catalog.pg_lsn_ge(pg_lsn, pg_lsn) returns boolean language internal;

create function pg_catalog.pg_lsn_gt(pg_lsn, pg_lsn) returns boolean language internal;

create function pg_catalog.pg_lsn_hash(pg_lsn) returns integer language internal;

create function pg_catalog.pg_lsn_hash_extended(pg_lsn, bigint) returns bigint language internal;

create function pg_catalog.pg_lsn_in(cstring) returns pg_lsn language internal;

create function pg_catalog.pg_lsn_larger(pg_lsn, pg_lsn) returns pg_lsn language internal;

create function pg_catalog.pg_lsn_le(pg_lsn, pg_lsn) returns boolean language internal;

create function pg_catalog.pg_lsn_lt(pg_lsn, pg_lsn) returns boolean language internal;

create function pg_catalog.pg_lsn_mi(pg_lsn, pg_lsn) returns numeric language internal;

create function pg_catalog.pg_lsn_mii(pg_lsn, numeric) returns pg_lsn language internal;

create function pg_catalog.pg_lsn_ne(pg_lsn, pg_lsn) returns boolean language internal;

create function pg_catalog.pg_lsn_out(pg_lsn) returns cstring language internal;

create function pg_catalog.pg_lsn_pli(pg_lsn, numeric) returns pg_lsn language internal;

create function pg_catalog.pg_lsn_recv(internal) returns pg_lsn language internal;

create function pg_catalog.pg_lsn_send(pg_lsn) returns bytea language internal;

create function pg_catalog.pg_lsn_smaller(pg_lsn, pg_lsn) returns pg_lsn language internal;

create function pg_catalog.pg_mcv_list_in(cstring) returns pg_mcv_list language internal;

create function pg_catalog.pg_mcv_list_items(mcv_list pg_mcv_list, OUT index integer, OUT "values" text[], OUT nulls boolean[], OUT frequency double precision, OUT base_frequency double precision) returns SETOF record language internal;

create function pg_catalog.pg_mcv_list_out(pg_mcv_list) returns cstring language internal;

create function pg_catalog.pg_mcv_list_recv(internal) returns pg_mcv_list language internal;

create function pg_catalog.pg_mcv_list_send(pg_mcv_list) returns bytea language internal;

create function pg_catalog.pg_my_temp_schema() returns oid language internal;

create function pg_catalog.pg_ndistinct_in(cstring) returns pg_ndistinct language internal;

create function pg_catalog.pg_ndistinct_out(pg_ndistinct) returns cstring language internal;

create function pg_catalog.pg_ndistinct_recv(internal) returns pg_ndistinct language internal;

create function pg_catalog.pg_ndistinct_send(pg_ndistinct) returns bytea language internal;

create function pg_catalog.pg_nextoid(regclass, name, regclass) returns oid language internal;

create function pg_catalog.pg_node_tree_in(cstring) returns pg_node_tree language internal;

create function pg_catalog.pg_node_tree_out(pg_node_tree) returns cstring language internal;

create function pg_catalog.pg_node_tree_recv(internal) returns pg_node_tree language internal;

create function pg_catalog.pg_node_tree_send(pg_node_tree) returns bytea language internal;

create function pg_catalog.pg_notification_queue_usage() returns double precision language internal;

create function pg_catalog.pg_notify(text, text) returns void language internal;

create function pg_catalog.pg_numa_available() returns boolean language internal;

create function pg_catalog.pg_opclass_is_visible(oid) returns boolean language internal;

create function pg_catalog.pg_operator_is_visible(oid) returns boolean language internal;

create function pg_catalog.pg_opfamily_is_visible(oid) returns boolean language internal;

create function pg_catalog.pg_options_to_table(options_array text[], OUT option_name text, OUT option_value text) returns SETOF record language internal;

create function pg_catalog.pg_partition_ancestors(partitionid regclass, OUT relid regclass) returns SETOF regclass language internal;

create function pg_catalog.pg_partition_root(regclass) returns regclass language internal;

create function pg_catalog.pg_partition_tree(rootrelid regclass, OUT relid regclass, OUT parentrelid regclass, OUT isleaf boolean, OUT level integer) returns SETOF record language internal;

create function pg_catalog.pg_postmaster_start_time() returns timestamp with time zone language internal;

create function pg_catalog.pg_prepared_statement(OUT name text, OUT statement text, OUT prepare_time timestamp with time zone, OUT parameter_types regtype[], OUT result_types regtype[], OUT from_sql boolean, OUT generic_plans bigint, OUT custom_plans bigint) returns SETOF record language internal;

create function pg_catalog.pg_prepared_xact(OUT transaction xid, OUT gid text, OUT prepared timestamp with time zone, OUT ownerid oid, OUT dbid oid) returns SETOF record language internal;

create function pg_catalog.pg_promote(wait boolean DEFAULT true, wait_seconds integer DEFAULT 60) returns boolean language internal;

create function pg_catalog.pg_read_binary_file(text) returns bytea language internal;

create function pg_catalog.pg_read_binary_file(text, boolean) returns bytea language internal;

create function pg_catalog.pg_read_binary_file(text, bigint, bigint, boolean) returns bytea language internal;

create function pg_catalog.pg_read_binary_file(text, bigint, bigint) returns bytea language internal;

create function pg_catalog.pg_read_file(text, boolean) returns text language internal;

create function pg_catalog.pg_read_file(text, bigint, bigint, boolean) returns text language internal;

create function pg_catalog.pg_read_file(text, bigint, bigint) returns text language internal;

create function pg_catalog.pg_read_file(text) returns text language internal;

create function pg_catalog.pg_relation_filenode(regclass) returns oid language internal;

create function pg_catalog.pg_relation_filepath(regclass) returns text language internal;

create function pg_catalog.pg_relation_is_publishable(regclass) returns boolean language internal;

create function pg_catalog.pg_relation_is_updatable(regclass, boolean) returns integer language internal;

create function pg_catalog.pg_relation_size(regclass) returns bigint language internal;

create function pg_catalog.pg_relation_size(regclass, text) returns bigint language internal;

create function pg_catalog.pg_reload_conf() returns boolean language internal;

create function pg_catalog.pg_replication_origin_advance(text, pg_lsn) returns void language internal;

create function pg_catalog.pg_replication_origin_create(text) returns oid language internal;

create function pg_catalog.pg_replication_origin_drop(text) returns void language internal;

create function pg_catalog.pg_replication_origin_oid(text) returns oid language internal;

create function pg_catalog.pg_replication_origin_progress(text, boolean) returns pg_lsn language internal;

create function pg_catalog.pg_replication_origin_session_is_setup() returns boolean language internal;

create function pg_catalog.pg_replication_origin_session_progress(boolean) returns pg_lsn language internal;

create function pg_catalog.pg_replication_origin_session_reset() returns void language internal;

create function pg_catalog.pg_replication_origin_session_setup(text) returns void language internal;

create function pg_catalog.pg_replication_origin_xact_reset() returns void language internal;

create function pg_catalog.pg_replication_origin_xact_setup(pg_lsn, timestamp with time zone) returns void language internal;

create function pg_catalog.pg_replication_slot_advance(slot_name name, upto_lsn pg_lsn, OUT slot_name name, OUT end_lsn pg_lsn) returns record language internal;

create function pg_catalog.pg_restore_attribute_stats(VARIADIC kwargs "any") returns boolean language internal;

create function pg_catalog.pg_restore_relation_stats(VARIADIC kwargs "any") returns boolean language internal;

create function pg_catalog.pg_rotate_logfile() returns boolean language internal;

create function pg_catalog.pg_safe_snapshot_blocking_pids(integer) returns integer[] language internal;

create function pg_catalog.pg_sequence_last_value(regclass) returns bigint language internal;

create function pg_catalog.pg_sequence_parameters(sequence_oid oid, OUT start_value bigint, OUT minimum_value bigint, OUT maximum_value bigint, OUT increment bigint, OUT cycle_option boolean, OUT cache_size bigint, OUT data_type oid) returns record language internal;

create function pg_catalog.pg_settings_get_flags(text) returns text[] language internal;

create function pg_catalog.pg_show_all_file_settings(OUT sourcefile text, OUT sourceline integer, OUT seqno integer, OUT name text, OUT setting text, OUT applied boolean, OUT error text) returns SETOF record language internal;

create function pg_catalog.pg_show_all_settings(OUT name text, OUT setting text, OUT unit text, OUT category text, OUT short_desc text, OUT extra_desc text, OUT context text, OUT vartype text, OUT source text, OUT min_val text, OUT max_val text, OUT enumvals text[], OUT boot_val text, OUT reset_val text, OUT sourcefile text, OUT sourceline integer, OUT pending_restart boolean) returns SETOF record language internal;

create function pg_catalog.pg_show_replication_origin_status(OUT local_id oid, OUT external_id text, OUT remote_lsn pg_lsn, OUT local_lsn pg_lsn) returns SETOF record language internal;

create function pg_catalog.pg_size_bytes(text) returns bigint language internal;

create function pg_catalog.pg_size_pretty(bigint) returns text language internal;

create function pg_catalog.pg_size_pretty(numeric) returns text language internal;

create function pg_catalog.pg_sleep(double precision) returns void language internal;

create function pg_catalog.pg_sleep_for(interval) returns void language internal;

create function pg_catalog.pg_sleep_until(timestamp with time zone) returns void language internal;

create function pg_catalog.pg_snapshot_in(cstring) returns pg_snapshot language internal;

create function pg_catalog.pg_snapshot_out(pg_snapshot) returns cstring language internal;

create function pg_catalog.pg_snapshot_recv(internal) returns pg_snapshot language internal;

create function pg_catalog.pg_snapshot_send(pg_snapshot) returns bytea language internal;

create function pg_catalog.pg_snapshot_xip(pg_snapshot) returns SETOF xid8 language internal;

create function pg_catalog.pg_snapshot_xmax(pg_snapshot) returns xid8 language internal;

create function pg_catalog.pg_snapshot_xmin(pg_snapshot) returns xid8 language internal;

create function pg_catalog.pg_split_walfile_name(file_name text, OUT segment_number numeric, OUT timeline_id bigint) returns record language internal;

create function pg_catalog.pg_stat_clear_snapshot() returns void language internal;

create function pg_catalog.pg_stat_file(filename text, OUT size bigint, OUT access timestamp with time zone, OUT modification timestamp with time zone, OUT change timestamp with time zone, OUT creation timestamp with time zone, OUT isdir boolean) returns record language internal;

create function pg_catalog.pg_stat_file(filename text, missing_ok boolean, OUT size bigint, OUT access timestamp with time zone, OUT modification timestamp with time zone, OUT change timestamp with time zone, OUT creation timestamp with time zone, OUT isdir boolean) returns record language internal;

create function pg_catalog.pg_stat_force_next_flush() returns void language internal;

create function pg_catalog.pg_stat_get_activity(pid integer, OUT datid oid, OUT pid integer, OUT usesysid oid, OUT application_name text, OUT state text, OUT query text, OUT wait_event_type text, OUT wait_event text, OUT xact_start timestamp with time zone, OUT query_start timestamp with time zone, OUT backend_start timestamp with time zone, OUT state_change timestamp with time zone, OUT client_addr inet, OUT client_hostname text, OUT client_port integer, OUT backend_xid xid, OUT backend_xmin xid, OUT backend_type text, OUT ssl boolean, OUT sslversion text, OUT sslcipher text, OUT sslbits integer, OUT ssl_client_dn text, OUT ssl_client_serial numeric, OUT ssl_issuer_dn text, OUT gss_auth boolean, OUT gss_princ text, OUT gss_enc boolean, OUT gss_delegation boolean, OUT leader_pid integer, OUT query_id bigint) returns SETOF record language internal;

create function pg_catalog.pg_stat_get_analyze_count(oid) returns bigint language internal;

create function pg_catalog.pg_stat_get_archiver(OUT archived_count bigint, OUT last_archived_wal text, OUT last_archived_time timestamp with time zone, OUT failed_count bigint, OUT last_failed_wal text, OUT last_failed_time timestamp with time zone, OUT stats_reset timestamp with time zone) returns record language internal;

create function pg_catalog.pg_stat_get_autoanalyze_count(oid) returns bigint language internal;

create function pg_catalog.pg_stat_get_autovacuum_count(oid) returns bigint language internal;

create function pg_catalog.pg_stat_get_backend_activity(integer) returns text language internal;

create function pg_catalog.pg_stat_get_backend_activity_start(integer) returns timestamp with time zone language internal;

create function pg_catalog.pg_stat_get_backend_client_addr(integer) returns inet language internal;

create function pg_catalog.pg_stat_get_backend_client_port(integer) returns integer language internal;

create function pg_catalog.pg_stat_get_backend_dbid(integer) returns oid language internal;

create function pg_catalog.pg_stat_get_backend_idset() returns SETOF integer language internal;

create function pg_catalog.pg_stat_get_backend_io(backend_pid integer, OUT backend_type text, OUT object text, OUT context text, OUT reads bigint, OUT read_bytes numeric, OUT read_time double precision, OUT writes bigint, OUT write_bytes numeric, OUT write_time double precision, OUT writebacks bigint, OUT writeback_time double precision, OUT extends bigint, OUT extend_bytes numeric, OUT extend_time double precision, OUT hits bigint, OUT evictions bigint, OUT reuses bigint, OUT fsyncs bigint, OUT fsync_time double precision, OUT stats_reset timestamp with time zone) returns SETOF record language internal;

create function pg_catalog.pg_stat_get_backend_pid(integer) returns integer language internal;

create function pg_catalog.pg_stat_get_backend_start(integer) returns timestamp with time zone language internal;

create function pg_catalog.pg_stat_get_backend_subxact(bid integer, OUT subxact_count integer, OUT subxact_overflowed boolean) returns record language internal;

create function pg_catalog.pg_stat_get_backend_userid(integer) returns oid language internal;

create function pg_catalog.pg_stat_get_backend_wait_event(integer) returns text language internal;

create function pg_catalog.pg_stat_get_backend_wait_event_type(integer) returns text language internal;

create function pg_catalog.pg_stat_get_backend_wal(backend_pid integer, OUT wal_records bigint, OUT wal_fpi bigint, OUT wal_bytes numeric, OUT wal_buffers_full bigint, OUT stats_reset timestamp with time zone) returns record language internal;

create function pg_catalog.pg_stat_get_backend_xact_start(integer) returns timestamp with time zone language internal;

create function pg_catalog.pg_stat_get_bgwriter_buf_written_clean() returns bigint language internal;

create function pg_catalog.pg_stat_get_bgwriter_maxwritten_clean() returns bigint language internal;

create function pg_catalog.pg_stat_get_bgwriter_stat_reset_time() returns timestamp with time zone language internal;

create function pg_catalog.pg_stat_get_blocks_fetched(oid) returns bigint language internal;

create function pg_catalog.pg_stat_get_blocks_hit(oid) returns bigint language internal;

create function pg_catalog.pg_stat_get_buf_alloc() returns bigint language internal;

create function pg_catalog.pg_stat_get_checkpointer_buffers_written() returns bigint language internal;

create function pg_catalog.pg_stat_get_checkpointer_num_performed() returns bigint language internal;

create function pg_catalog.pg_stat_get_checkpointer_num_requested() returns bigint language internal;

create function pg_catalog.pg_stat_get_checkpointer_num_timed() returns bigint language internal;

create function pg_catalog.pg_stat_get_checkpointer_restartpoints_performed() returns bigint language internal;

create function pg_catalog.pg_stat_get_checkpointer_restartpoints_requested() returns bigint language internal;

create function pg_catalog.pg_stat_get_checkpointer_restartpoints_timed() returns bigint language internal;

create function pg_catalog.pg_stat_get_checkpointer_slru_written() returns bigint language internal;

create function pg_catalog.pg_stat_get_checkpointer_stat_reset_time() returns timestamp with time zone language internal;

create function pg_catalog.pg_stat_get_checkpointer_sync_time() returns double precision language internal;

create function pg_catalog.pg_stat_get_checkpointer_write_time() returns double precision language internal;

create function pg_catalog.pg_stat_get_db_active_time(oid) returns double precision language internal;

create function pg_catalog.pg_stat_get_db_blk_read_time(oid) returns double precision language internal;

create function pg_catalog.pg_stat_get_db_blk_write_time(oid) returns double precision language internal;

create function pg_catalog.pg_stat_get_db_blocks_fetched(oid) returns bigint language internal;

create function pg_catalog.pg_stat_get_db_blocks_hit(oid) returns bigint language internal;

create function pg_catalog.pg_stat_get_db_checksum_failures(oid) returns bigint language internal;

create function pg_catalog.pg_stat_get_db_checksum_last_failure(oid) returns timestamp with time zone language internal;

create function pg_catalog.pg_stat_get_db_conflict_all(oid) returns bigint language internal;

create function pg_catalog.pg_stat_get_db_conflict_bufferpin(oid) returns bigint language internal;

create function pg_catalog.pg_stat_get_db_conflict_lock(oid) returns bigint language internal;

create function pg_catalog.pg_stat_get_db_conflict_logicalslot(oid) returns bigint language internal;

create function pg_catalog.pg_stat_get_db_conflict_snapshot(oid) returns bigint language internal;

create function pg_catalog.pg_stat_get_db_conflict_startup_deadlock(oid) returns bigint language internal;

create function pg_catalog.pg_stat_get_db_conflict_tablespace(oid) returns bigint language internal;

create function pg_catalog.pg_stat_get_db_deadlocks(oid) returns bigint language internal;

create function pg_catalog.pg_stat_get_db_idle_in_transaction_time(oid) returns double precision language internal;

create function pg_catalog.pg_stat_get_db_numbackends(oid) returns integer language internal;

create function pg_catalog.pg_stat_get_db_parallel_workers_launched(oid) returns bigint language internal;

create function pg_catalog.pg_stat_get_db_parallel_workers_to_launch(oid) returns bigint language internal;

create function pg_catalog.pg_stat_get_db_session_time(oid) returns double precision language internal;

create function pg_catalog.pg_stat_get_db_sessions(oid) returns bigint language internal;

create function pg_catalog.pg_stat_get_db_sessions_abandoned(oid) returns bigint language internal;

create function pg_catalog.pg_stat_get_db_sessions_fatal(oid) returns bigint language internal;

create function pg_catalog.pg_stat_get_db_sessions_killed(oid) returns bigint language internal;

create function pg_catalog.pg_stat_get_db_stat_reset_time(oid) returns timestamp with time zone language internal;

create function pg_catalog.pg_stat_get_db_temp_bytes(oid) returns bigint language internal;

create function pg_catalog.pg_stat_get_db_temp_files(oid) returns bigint language internal;

create function pg_catalog.pg_stat_get_db_tuples_deleted(oid) returns bigint language internal;

create function pg_catalog.pg_stat_get_db_tuples_fetched(oid) returns bigint language internal;

create function pg_catalog.pg_stat_get_db_tuples_inserted(oid) returns bigint language internal;

create function pg_catalog.pg_stat_get_db_tuples_returned(oid) returns bigint language internal;

create function pg_catalog.pg_stat_get_db_tuples_updated(oid) returns bigint language internal;

create function pg_catalog.pg_stat_get_db_xact_commit(oid) returns bigint language internal;

create function pg_catalog.pg_stat_get_db_xact_rollback(oid) returns bigint language internal;

create function pg_catalog.pg_stat_get_dead_tuples(oid) returns bigint language internal;

create function pg_catalog.pg_stat_get_function_calls(oid) returns bigint language internal;

create function pg_catalog.pg_stat_get_function_self_time(oid) returns double precision language internal;

create function pg_catalog.pg_stat_get_function_total_time(oid) returns double precision language internal;

create function pg_catalog.pg_stat_get_ins_since_vacuum(oid) returns bigint language internal;

create function pg_catalog.pg_stat_get_io(OUT backend_type text, OUT object text, OUT context text, OUT reads bigint, OUT read_bytes numeric, OUT read_time double precision, OUT writes bigint, OUT write_bytes numeric, OUT write_time double precision, OUT writebacks bigint, OUT writeback_time double precision, OUT extends bigint, OUT extend_bytes numeric, OUT extend_time double precision, OUT hits bigint, OUT evictions bigint, OUT reuses bigint, OUT fsyncs bigint, OUT fsync_time double precision, OUT stats_reset timestamp with time zone) returns SETOF record language internal;

create function pg_catalog.pg_stat_get_last_analyze_time(oid) returns timestamp with time zone language internal;

create function pg_catalog.pg_stat_get_last_autoanalyze_time(oid) returns timestamp with time zone language internal;

create function pg_catalog.pg_stat_get_last_autovacuum_time(oid) returns timestamp with time zone language internal;

create function pg_catalog.pg_stat_get_last_vacuum_time(oid) returns timestamp with time zone language internal;

create function pg_catalog.pg_stat_get_lastscan(oid) returns timestamp with time zone language internal;

create function pg_catalog.pg_stat_get_live_tuples(oid) returns bigint language internal;

create function pg_catalog.pg_stat_get_mod_since_analyze(oid) returns bigint language internal;

create function pg_catalog.pg_stat_get_numscans(oid) returns bigint language internal;

create function pg_catalog.pg_stat_get_progress_info(cmdtype text, OUT pid integer, OUT datid oid, OUT relid oid, OUT param1 bigint, OUT param2 bigint, OUT param3 bigint, OUT param4 bigint, OUT param5 bigint, OUT param6 bigint, OUT param7 bigint, OUT param8 bigint, OUT param9 bigint, OUT param10 bigint, OUT param11 bigint, OUT param12 bigint, OUT param13 bigint, OUT param14 bigint, OUT param15 bigint, OUT param16 bigint, OUT param17 bigint, OUT param18 bigint, OUT param19 bigint, OUT param20 bigint) returns SETOF record language internal;

create function pg_catalog.pg_stat_get_recovery_prefetch(OUT stats_reset timestamp with time zone, OUT prefetch bigint, OUT hit bigint, OUT skip_init bigint, OUT skip_new bigint, OUT skip_fpw bigint, OUT skip_rep bigint, OUT wal_distance integer, OUT block_distance integer, OUT io_depth integer) returns SETOF record language internal;

create function pg_catalog.pg_stat_get_replication_slot(slot_name text, OUT slot_name text, OUT spill_txns bigint, OUT spill_count bigint, OUT spill_bytes bigint, OUT stream_txns bigint, OUT stream_count bigint, OUT stream_bytes bigint, OUT total_txns bigint, OUT total_bytes bigint, OUT stats_reset timestamp with time zone) returns record language internal;

create function pg_catalog.pg_stat_get_slru(OUT name text, OUT blks_zeroed bigint, OUT blks_hit bigint, OUT blks_read bigint, OUT blks_written bigint, OUT blks_exists bigint, OUT flushes bigint, OUT truncates bigint, OUT stats_reset timestamp with time zone) returns SETOF record language internal;

create function pg_catalog.pg_stat_get_snapshot_timestamp() returns timestamp with time zone language internal;

create function pg_catalog.pg_stat_get_subscription(subid oid, OUT subid oid, OUT relid oid, OUT pid integer, OUT leader_pid integer, OUT received_lsn pg_lsn, OUT last_msg_send_time timestamp with time zone, OUT last_msg_receipt_time timestamp with time zone, OUT latest_end_lsn pg_lsn, OUT latest_end_time timestamp with time zone, OUT worker_type text) returns SETOF record language internal;

create function pg_catalog.pg_stat_get_subscription_stats(subid oid, OUT subid oid, OUT apply_error_count bigint, OUT sync_error_count bigint, OUT confl_insert_exists bigint, OUT confl_update_origin_differs bigint, OUT confl_update_exists bigint, OUT confl_update_missing bigint, OUT confl_delete_origin_differs bigint, OUT confl_delete_missing bigint, OUT confl_multiple_unique_conflicts bigint, OUT stats_reset timestamp with time zone) returns record language internal;

create function pg_catalog.pg_stat_get_total_analyze_time(oid) returns double precision language internal;

create function pg_catalog.pg_stat_get_total_autoanalyze_time(oid) returns double precision language internal;

create function pg_catalog.pg_stat_get_total_autovacuum_time(oid) returns double precision language internal;

create function pg_catalog.pg_stat_get_total_vacuum_time(oid) returns double precision language internal;

create function pg_catalog.pg_stat_get_tuples_deleted(oid) returns bigint language internal;

create function pg_catalog.pg_stat_get_tuples_fetched(oid) returns bigint language internal;

create function pg_catalog.pg_stat_get_tuples_hot_updated(oid) returns bigint language internal;

create function pg_catalog.pg_stat_get_tuples_inserted(oid) returns bigint language internal;

create function pg_catalog.pg_stat_get_tuples_newpage_updated(oid) returns bigint language internal;

create function pg_catalog.pg_stat_get_tuples_returned(oid) returns bigint language internal;

create function pg_catalog.pg_stat_get_tuples_updated(oid) returns bigint language internal;

create function pg_catalog.pg_stat_get_vacuum_count(oid) returns bigint language internal;

create function pg_catalog.pg_stat_get_wal(OUT wal_records bigint, OUT wal_fpi bigint, OUT wal_bytes numeric, OUT wal_buffers_full bigint, OUT stats_reset timestamp with time zone) returns record language internal;

create function pg_catalog.pg_stat_get_wal_receiver(OUT pid integer, OUT status text, OUT receive_start_lsn pg_lsn, OUT receive_start_tli integer, OUT written_lsn pg_lsn, OUT flushed_lsn pg_lsn, OUT received_tli integer, OUT last_msg_send_time timestamp with time zone, OUT last_msg_receipt_time timestamp with time zone, OUT latest_end_lsn pg_lsn, OUT latest_end_time timestamp with time zone, OUT slot_name text, OUT sender_host text, OUT sender_port integer, OUT conninfo text) returns record language internal;

create function pg_catalog.pg_stat_get_wal_senders(OUT pid integer, OUT state text, OUT sent_lsn pg_lsn, OUT write_lsn pg_lsn, OUT flush_lsn pg_lsn, OUT replay_lsn pg_lsn, OUT write_lag interval, OUT flush_lag interval, OUT replay_lag interval, OUT sync_priority integer, OUT sync_state text, OUT reply_time timestamp with time zone) returns SETOF record language internal;

create function pg_catalog.pg_stat_get_xact_blocks_fetched(oid) returns bigint language internal;

create function pg_catalog.pg_stat_get_xact_blocks_hit(oid) returns bigint language internal;

create function pg_catalog.pg_stat_get_xact_function_calls(oid) returns bigint language internal;

create function pg_catalog.pg_stat_get_xact_function_self_time(oid) returns double precision language internal;

create function pg_catalog.pg_stat_get_xact_function_total_time(oid) returns double precision language internal;

create function pg_catalog.pg_stat_get_xact_numscans(oid) returns bigint language internal;

create function pg_catalog.pg_stat_get_xact_tuples_deleted(oid) returns bigint language internal;

create function pg_catalog.pg_stat_get_xact_tuples_fetched(oid) returns bigint language internal;

create function pg_catalog.pg_stat_get_xact_tuples_hot_updated(oid) returns bigint language internal;

create function pg_catalog.pg_stat_get_xact_tuples_inserted(oid) returns bigint language internal;

create function pg_catalog.pg_stat_get_xact_tuples_newpage_updated(oid) returns bigint language internal;

create function pg_catalog.pg_stat_get_xact_tuples_returned(oid) returns bigint language internal;

create function pg_catalog.pg_stat_get_xact_tuples_updated(oid) returns bigint language internal;

create function pg_catalog.pg_stat_have_stats(text, oid, bigint) returns boolean language internal;

create function pg_catalog.pg_stat_reset() returns void language internal;

create function pg_catalog.pg_stat_reset_backend_stats(integer) returns void language internal;

create function pg_catalog.pg_stat_reset_replication_slot(text) returns void language internal;

create function pg_catalog.pg_stat_reset_shared(target text DEFAULT NULL::text) returns void language internal;

create function pg_catalog.pg_stat_reset_single_function_counters(oid) returns void language internal;

create function pg_catalog.pg_stat_reset_single_table_counters(oid) returns void language internal;

create function pg_catalog.pg_stat_reset_slru(target text DEFAULT NULL::text) returns void language internal;

create function pg_catalog.pg_stat_reset_subscription_stats(oid) returns void language internal;

create function pg_catalog.pg_statistics_obj_is_visible(oid) returns boolean language internal;

create function pg_catalog.pg_stop_making_pinned_objects() returns void language internal;

create function pg_catalog.pg_switch_wal() returns pg_lsn language internal;

create function pg_catalog.pg_sync_replication_slots() returns void language internal;

create function pg_catalog.pg_table_is_visible(oid) returns boolean language internal;

create function pg_catalog.pg_table_size(regclass) returns bigint language internal;

create function pg_catalog.pg_tablespace_databases(oid) returns SETOF oid language internal;

create function pg_catalog.pg_tablespace_location(oid) returns text language internal;

create function pg_catalog.pg_tablespace_size(oid) returns bigint language internal;

create function pg_catalog.pg_tablespace_size(name) returns bigint language internal;

create function pg_catalog.pg_terminate_backend(pid integer, timeout bigint DEFAULT 0) returns boolean language internal;

create function pg_catalog.pg_timezone_abbrevs_abbrevs(OUT abbrev text, OUT utc_offset interval, OUT is_dst boolean) returns SETOF record language internal;

create function pg_catalog.pg_timezone_abbrevs_zone(OUT abbrev text, OUT utc_offset interval, OUT is_dst boolean) returns SETOF record language internal;

create function pg_catalog.pg_timezone_names(OUT name text, OUT abbrev text, OUT utc_offset interval, OUT is_dst boolean) returns SETOF record language internal;

create function pg_catalog.pg_total_relation_size(regclass) returns bigint language internal;

create function pg_catalog.pg_trigger_depth() returns integer language internal;

create function pg_catalog.pg_try_advisory_lock(bigint) returns boolean language internal;

create function pg_catalog.pg_try_advisory_lock(integer, integer) returns boolean language internal;

create function pg_catalog.pg_try_advisory_lock_shared(integer, integer) returns boolean language internal;

create function pg_catalog.pg_try_advisory_lock_shared(bigint) returns boolean language internal;

create function pg_catalog.pg_try_advisory_xact_lock(integer, integer) returns boolean language internal;

create function pg_catalog.pg_try_advisory_xact_lock(bigint) returns boolean language internal;

create function pg_catalog.pg_try_advisory_xact_lock_shared(bigint) returns boolean language internal;

create function pg_catalog.pg_try_advisory_xact_lock_shared(integer, integer) returns boolean language internal;

create function pg_catalog.pg_ts_config_is_visible(oid) returns boolean language internal;

create function pg_catalog.pg_ts_dict_is_visible(oid) returns boolean language internal;

create function pg_catalog.pg_ts_parser_is_visible(oid) returns boolean language internal;

create function pg_catalog.pg_ts_template_is_visible(oid) returns boolean language internal;

create function pg_catalog.pg_type_is_visible(oid) returns boolean language internal;

create function pg_catalog.pg_typeof("any") returns regtype language internal;

create function pg_catalog.pg_visible_in_snapshot(xid8, pg_snapshot) returns boolean language internal;

create function pg_catalog.pg_wal_lsn_diff(pg_lsn, pg_lsn) returns numeric language internal;

create function pg_catalog.pg_wal_replay_pause() returns void language internal;

create function pg_catalog.pg_wal_replay_resume() returns void language internal;

create function pg_catalog.pg_wal_summary_contents(tli bigint, start_lsn pg_lsn, end_lsn pg_lsn, OUT relfilenode oid, OUT reltablespace oid, OUT reldatabase oid, OUT relforknumber smallint, OUT relblocknumber bigint, OUT is_limit_block boolean) returns SETOF record language internal;

create function pg_catalog.pg_walfile_name(pg_lsn) returns text language internal;

create function pg_catalog.pg_walfile_name_offset(lsn pg_lsn, OUT file_name text, OUT file_offset integer) returns record language internal;

create function pg_catalog.pg_xact_commit_timestamp(xid) returns timestamp with time zone language internal;

create function pg_catalog.pg_xact_commit_timestamp_origin(xid xid, OUT "timestamp" timestamp with time zone, OUT roident oid) returns record language internal;

create function pg_catalog.pg_xact_status(xid8) returns text language internal;

create function pg_catalog.phraseto_tsquery(regconfig, text) returns tsquery language internal;

create function pg_catalog.phraseto_tsquery(text) returns tsquery language internal;

create function pg_catalog.pi() returns double precision language internal;

create function pg_catalog.plainto_tsquery(text) returns tsquery language internal;

create function pg_catalog.plainto_tsquery(regconfig, text) returns tsquery language internal;

create function pg_catalog.plpgsql_call_handler() returns language_handler language internal;

create function pg_catalog.plpgsql_inline_handler(internal) returns void language internal;

create function pg_catalog.plpgsql_validator(oid) returns void language internal;

create function pg_catalog.point(box) returns point language internal;

create function pg_catalog.point(double precision, double precision) returns point language internal;

create function pg_catalog.point(lseg) returns point language internal;

create function pg_catalog.point(circle) returns point language internal;

create function pg_catalog.point(polygon) returns point language internal;

create function pg_catalog.point_above(point, point) returns boolean language internal;

create function pg_catalog.point_add(point, point) returns point language internal;

create function pg_catalog.point_below(point, point) returns boolean language internal;

create function pg_catalog.point_distance(point, point) returns double precision language internal;

create function pg_catalog.point_div(point, point) returns point language internal;

create function pg_catalog.point_eq(point, point) returns boolean language internal;

create function pg_catalog.point_horiz(point, point) returns boolean language internal;

create function pg_catalog.point_in(cstring) returns point language internal;

create function pg_catalog.point_left(point, point) returns boolean language internal;

create function pg_catalog.point_mul(point, point) returns point language internal;

create function pg_catalog.point_ne(point, point) returns boolean language internal;

create function pg_catalog.point_out(point) returns cstring language internal;

create function pg_catalog.point_recv(internal) returns point language internal;

create function pg_catalog.point_right(point, point) returns boolean language internal;

create function pg_catalog.point_send(point) returns bytea language internal;

create function pg_catalog.point_sub(point, point) returns point language internal;

create function pg_catalog.point_vert(point, point) returns boolean language internal;

create function pg_catalog.poly_above(polygon, polygon) returns boolean language internal;

create function pg_catalog.poly_below(polygon, polygon) returns boolean language internal;

create function pg_catalog.poly_center(polygon) returns point language internal;

create function pg_catalog.poly_contain(polygon, polygon) returns boolean language internal;

create function pg_catalog.poly_contain_pt(polygon, point) returns boolean language internal;

create function pg_catalog.poly_contained(polygon, polygon) returns boolean language internal;

create function pg_catalog.poly_distance(polygon, polygon) returns double precision language internal;

create function pg_catalog.poly_in(cstring) returns polygon language internal;

create function pg_catalog.poly_left(polygon, polygon) returns boolean language internal;

create function pg_catalog.poly_npoints(polygon) returns integer language internal;

create function pg_catalog.poly_out(polygon) returns cstring language internal;

create function pg_catalog.poly_overabove(polygon, polygon) returns boolean language internal;

create function pg_catalog.poly_overbelow(polygon, polygon) returns boolean language internal;

create function pg_catalog.poly_overlap(polygon, polygon) returns boolean language internal;

create function pg_catalog.poly_overleft(polygon, polygon) returns boolean language internal;

create function pg_catalog.poly_overright(polygon, polygon) returns boolean language internal;

create function pg_catalog.poly_recv(internal) returns polygon language internal;

create function pg_catalog.poly_right(polygon, polygon) returns boolean language internal;

create function pg_catalog.poly_same(polygon, polygon) returns boolean language internal;

create function pg_catalog.poly_send(polygon) returns bytea language internal;

create function pg_catalog.polygon(box) returns polygon language internal;

create function pg_catalog.polygon(path) returns polygon language internal;

create function pg_catalog.polygon(circle) returns polygon language internal;

create function pg_catalog.polygon(integer, circle) returns polygon language internal;

create function pg_catalog.popen(path) returns path language internal;

create function pg_catalog.position(text, text) returns integer language internal;

create function pg_catalog.position(bit, bit) returns integer language internal;

create function pg_catalog.position(bytea, bytea) returns integer language internal;

create function pg_catalog.positionjoinsel(internal, oid, internal, smallint, internal) returns double precision language internal;

create function pg_catalog.positionsel(internal, oid, internal, integer) returns double precision language internal;

create function pg_catalog.postgresql_fdw_validator(text[], oid) returns boolean language internal;

create function pg_catalog.pow(double precision, double precision) returns double precision language internal;

create function pg_catalog.pow(numeric, numeric) returns numeric language internal;

create function pg_catalog.power(numeric, numeric) returns numeric language internal;

create function pg_catalog.power(double precision, double precision) returns double precision language internal;

create function pg_catalog.prefixjoinsel(internal, oid, internal, smallint, internal) returns double precision language internal;

create function pg_catalog.prefixsel(internal, oid, internal, integer) returns double precision language internal;

create function pg_catalog.prsd_end(internal) returns void language internal;

create function pg_catalog.prsd_headline(internal, internal, tsquery) returns internal language internal;

create function pg_catalog.prsd_lextype(internal) returns internal language internal;

create function pg_catalog.prsd_nexttoken(internal, internal, internal) returns internal language internal;

create function pg_catalog.prsd_start(internal, integer) returns internal language internal;

create function pg_catalog.pt_contained_circle(point, circle) returns boolean language internal;

create function pg_catalog.pt_contained_poly(point, polygon) returns boolean language internal;

create function pg_catalog.query_to_xml(query text, nulls boolean, tableforest boolean, targetns text) returns xml language internal;

create function pg_catalog.query_to_xml_and_xmlschema(query text, nulls boolean, tableforest boolean, targetns text) returns xml language internal;

create function pg_catalog.query_to_xmlschema(query text, nulls boolean, tableforest boolean, targetns text) returns xml language internal;

create function pg_catalog.querytree(tsquery) returns text language internal;

create function pg_catalog.quote_ident(text) returns text language internal;

create function pg_catalog.quote_literal(anyelement) returns text language internal;

create function pg_catalog.quote_literal(text) returns text language internal;

create function pg_catalog.quote_nullable(anyelement) returns text language internal;

create function pg_catalog.quote_nullable(text) returns text language internal;

create function pg_catalog.radians(double precision) returns double precision language internal;

create function pg_catalog.radius(circle) returns double precision language internal;

create function pg_catalog.random(min bigint, max bigint) returns bigint language internal;

create function pg_catalog.random(min numeric, max numeric) returns numeric language internal;

create function pg_catalog.random() returns double precision language internal;

create function pg_catalog.random(min integer, max integer) returns integer language internal;

create function pg_catalog.random_normal(mean double precision DEFAULT 0, stddev double precision DEFAULT 1) returns double precision language internal;

create function pg_catalog.range_adjacent(anyrange, anyrange) returns boolean language internal;

create function pg_catalog.range_adjacent_multirange(anyrange, anymultirange) returns boolean language internal;

create function pg_catalog.range_after(anyrange, anyrange) returns boolean language internal;

create function pg_catalog.range_after_multirange(anyrange, anymultirange) returns boolean language internal;

create function pg_catalog.range_agg(anyrange) returns anymultirange language internal;

create function pg_catalog.range_agg(anymultirange) returns anymultirange language internal;

create function pg_catalog.range_agg_finalfn(internal, anyrange) returns anymultirange language internal;

create function pg_catalog.range_agg_transfn(internal, anyrange) returns internal language internal;

create function pg_catalog.range_before(anyrange, anyrange) returns boolean language internal;

create function pg_catalog.range_before_multirange(anyrange, anymultirange) returns boolean language internal;

create function pg_catalog.range_cmp(anyrange, anyrange) returns integer language internal;

create function pg_catalog.range_contained_by(anyrange, anyrange) returns boolean language internal;

create function pg_catalog.range_contained_by_multirange(anyrange, anymultirange) returns boolean language internal;

create function pg_catalog.range_contains(anyrange, anyrange) returns boolean language internal;

create function pg_catalog.range_contains_elem(anyrange, anyelement) returns boolean language internal;

create function pg_catalog.range_contains_elem_support(internal) returns internal language internal;

create function pg_catalog.range_contains_multirange(anyrange, anymultirange) returns boolean language internal;

create function pg_catalog.range_eq(anyrange, anyrange) returns boolean language internal;

create function pg_catalog.range_ge(anyrange, anyrange) returns boolean language internal;

create function pg_catalog.range_gist_consistent(internal, anyrange, smallint, oid, internal) returns boolean language internal;

create function pg_catalog.range_gist_penalty(internal, internal, internal) returns internal language internal;

create function pg_catalog.range_gist_picksplit(internal, internal) returns internal language internal;

create function pg_catalog.range_gist_same(anyrange, anyrange, internal) returns internal language internal;

create function pg_catalog.range_gist_union(internal, internal) returns anyrange language internal;

create function pg_catalog.range_gt(anyrange, anyrange) returns boolean language internal;

create function pg_catalog.range_in(cstring, oid, integer) returns anyrange language internal;

create function pg_catalog.range_intersect(anyrange, anyrange) returns anyrange language internal;

create function pg_catalog.range_intersect_agg(anymultirange) returns anymultirange language internal;

create function pg_catalog.range_intersect_agg(anyrange) returns anyrange language internal;

create function pg_catalog.range_intersect_agg_transfn(anyrange, anyrange) returns anyrange language internal;

create function pg_catalog.range_le(anyrange, anyrange) returns boolean language internal;

create function pg_catalog.range_lt(anyrange, anyrange) returns boolean language internal;

create function pg_catalog.range_merge(anymultirange) returns anyrange language internal;

create function pg_catalog.range_merge(anyrange, anyrange) returns anyrange language internal;

create function pg_catalog.range_minus(anyrange, anyrange) returns anyrange language internal;

create function pg_catalog.range_ne(anyrange, anyrange) returns boolean language internal;

create function pg_catalog.range_out(anyrange) returns cstring language internal;

create function pg_catalog.range_overlaps(anyrange, anyrange) returns boolean language internal;

create function pg_catalog.range_overlaps_multirange(anyrange, anymultirange) returns boolean language internal;

create function pg_catalog.range_overleft(anyrange, anyrange) returns boolean language internal;

create function pg_catalog.range_overleft_multirange(anyrange, anymultirange) returns boolean language internal;

create function pg_catalog.range_overright(anyrange, anyrange) returns boolean language internal;

create function pg_catalog.range_overright_multirange(anyrange, anymultirange) returns boolean language internal;

create function pg_catalog.range_recv(internal, oid, integer) returns anyrange language internal;

create function pg_catalog.range_send(anyrange) returns bytea language internal;

create function pg_catalog.range_sortsupport(internal) returns void language internal;

create function pg_catalog.range_typanalyze(internal) returns boolean language internal;

create function pg_catalog.range_union(anyrange, anyrange) returns anyrange language internal;

create function pg_catalog.rangesel(internal, oid, internal, integer) returns double precision language internal;

create function pg_catalog.rank() returns bigint language internal;

create function pg_catalog.rank_final(internal, VARIADIC "any") returns bigint language internal;

create function pg_catalog.raw_array_subscript_handler(internal) returns internal language internal;

create function pg_catalog.record_eq(record, record) returns boolean language internal;

create function pg_catalog.record_ge(record, record) returns boolean language internal;

create function pg_catalog.record_gt(record, record) returns boolean language internal;

create function pg_catalog.record_image_eq(record, record) returns boolean language internal;

create function pg_catalog.record_image_ge(record, record) returns boolean language internal;

create function pg_catalog.record_image_gt(record, record) returns boolean language internal;

create function pg_catalog.record_image_le(record, record) returns boolean language internal;

create function pg_catalog.record_image_lt(record, record) returns boolean language internal;

create function pg_catalog.record_image_ne(record, record) returns boolean language internal;

create function pg_catalog.record_in(cstring, oid, integer) returns record language internal;

create function pg_catalog.record_larger(record, record) returns record language internal;

create function pg_catalog.record_le(record, record) returns boolean language internal;

create function pg_catalog.record_lt(record, record) returns boolean language internal;

create function pg_catalog.record_ne(record, record) returns boolean language internal;

create function pg_catalog.record_out(record) returns cstring language internal;

create function pg_catalog.record_recv(internal, oid, integer) returns record language internal;

create function pg_catalog.record_send(record) returns bytea language internal;

create function pg_catalog.record_smaller(record, record) returns record language internal;

create function pg_catalog.regclass(text) returns regclass language internal;

create function pg_catalog.regclassin(cstring) returns regclass language internal;

create function pg_catalog.regclassout(regclass) returns cstring language internal;

create function pg_catalog.regclassrecv(internal) returns regclass language internal;

create function pg_catalog.regclasssend(regclass) returns bytea language internal;

create function pg_catalog.regcollationin(cstring) returns regcollation language internal;

create function pg_catalog.regcollationout(regcollation) returns cstring language internal;

create function pg_catalog.regcollationrecv(internal) returns regcollation language internal;

create function pg_catalog.regcollationsend(regcollation) returns bytea language internal;

create function pg_catalog.regconfigin(cstring) returns regconfig language internal;

create function pg_catalog.regconfigout(regconfig) returns cstring language internal;

create function pg_catalog.regconfigrecv(internal) returns regconfig language internal;

create function pg_catalog.regconfigsend(regconfig) returns bytea language internal;

create function pg_catalog.regdictionaryin(cstring) returns regdictionary language internal;

create function pg_catalog.regdictionaryout(regdictionary) returns cstring language internal;

create function pg_catalog.regdictionaryrecv(internal) returns regdictionary language internal;

create function pg_catalog.regdictionarysend(regdictionary) returns bytea language internal;

create function pg_catalog.regexeqjoinsel(internal, oid, internal, smallint, internal) returns double precision language internal;

create function pg_catalog.regexeqsel(internal, oid, internal, integer) returns double precision language internal;

create function pg_catalog.regexnejoinsel(internal, oid, internal, smallint, internal) returns double precision language internal;

create function pg_catalog.regexnesel(internal, oid, internal, integer) returns double precision language internal;

create function pg_catalog.regexp_count(string text, pattern text, start integer) returns integer language internal;

create function pg_catalog.regexp_count(string text, pattern text, start integer, flags text) returns integer language internal;

create function pg_catalog.regexp_count(string text, pattern text) returns integer language internal;

create function pg_catalog.regexp_instr(string text, pattern text, start integer, "N" integer, endoption integer, flags text, subexpr integer) returns integer language internal;

create function pg_catalog.regexp_instr(string text, pattern text, start integer, "N" integer, endoption integer) returns integer language internal;

create function pg_catalog.regexp_instr(string text, pattern text, start integer, "N" integer) returns integer language internal;

create function pg_catalog.regexp_instr(string text, pattern text) returns integer language internal;

create function pg_catalog.regexp_instr(string text, pattern text, start integer, "N" integer, endoption integer, flags text) returns integer language internal;

create function pg_catalog.regexp_instr(string text, pattern text, start integer) returns integer language internal;

create function pg_catalog.regexp_like(string text, pattern text, flags text) returns boolean language internal;

create function pg_catalog.regexp_like(string text, pattern text) returns boolean language internal;

create function pg_catalog.regexp_match(string text, pattern text, flags text) returns text[] language internal;

create function pg_catalog.regexp_match(string text, pattern text) returns text[] language internal;

create function pg_catalog.regexp_matches(string text, pattern text) returns SETOF text[] language internal;

create function pg_catalog.regexp_matches(string text, pattern text, flags text) returns SETOF text[] language internal;

create function pg_catalog.regexp_replace(string text, pattern text, replacement text, start integer) returns text language internal;

create function pg_catalog.regexp_replace(string text, pattern text, replacement text, start integer, "N" integer) returns text language internal;

create function pg_catalog.regexp_replace(string text, pattern text, replacement text, start integer, "N" integer, flags text) returns text language internal;

create function pg_catalog.regexp_replace(string text, pattern text, replacement text, flags text) returns text language internal;

create function pg_catalog.regexp_replace(string text, pattern text, replacement text) returns text language internal;

create function pg_catalog.regexp_split_to_array(string text, pattern text, flags text) returns text[] language internal;

create function pg_catalog.regexp_split_to_array(string text, pattern text) returns text[] language internal;

create function pg_catalog.regexp_split_to_table(string text, pattern text) returns SETOF text language internal;

create function pg_catalog.regexp_split_to_table(string text, pattern text, flags text) returns SETOF text language internal;

create function pg_catalog.regexp_substr(string text, pattern text) returns text language internal;

create function pg_catalog.regexp_substr(string text, pattern text, start integer, "N" integer, flags text) returns text language internal;

create function pg_catalog.regexp_substr(string text, pattern text, start integer) returns text language internal;

create function pg_catalog.regexp_substr(string text, pattern text, start integer, "N" integer) returns text language internal;

create function pg_catalog.regexp_substr(string text, pattern text, start integer, "N" integer, flags text, subexpr integer) returns text language internal;

create function pg_catalog.regnamespacein(cstring) returns regnamespace language internal;

create function pg_catalog.regnamespaceout(regnamespace) returns cstring language internal;

create function pg_catalog.regnamespacerecv(internal) returns regnamespace language internal;

create function pg_catalog.regnamespacesend(regnamespace) returns bytea language internal;

create function pg_catalog.regoperatorin(cstring) returns regoperator language internal;

create function pg_catalog.regoperatorout(regoperator) returns cstring language internal;

create function pg_catalog.regoperatorrecv(internal) returns regoperator language internal;

create function pg_catalog.regoperatorsend(regoperator) returns bytea language internal;

create function pg_catalog.regoperin(cstring) returns regoper language internal;

create function pg_catalog.regoperout(regoper) returns cstring language internal;

create function pg_catalog.regoperrecv(internal) returns regoper language internal;

create function pg_catalog.regopersend(regoper) returns bytea language internal;

create function pg_catalog.regprocedurein(cstring) returns regprocedure language internal;

create function pg_catalog.regprocedureout(regprocedure) returns cstring language internal;

create function pg_catalog.regprocedurerecv(internal) returns regprocedure language internal;

create function pg_catalog.regproceduresend(regprocedure) returns bytea language internal;

create function pg_catalog.regprocin(cstring) returns regproc language internal;

create function pg_catalog.regprocout(regproc) returns cstring language internal;

create function pg_catalog.regprocrecv(internal) returns regproc language internal;

create function pg_catalog.regprocsend(regproc) returns bytea language internal;

create function pg_catalog.regr_avgx(double precision, double precision) returns double precision language internal;

create function pg_catalog.regr_avgy(double precision, double precision) returns double precision language internal;

create function pg_catalog.regr_count(double precision, double precision) returns bigint language internal;

create function pg_catalog.regr_intercept(double precision, double precision) returns double precision language internal;

create function pg_catalog.regr_r2(double precision, double precision) returns double precision language internal;

create function pg_catalog.regr_slope(double precision, double precision) returns double precision language internal;

create function pg_catalog.regr_sxx(double precision, double precision) returns double precision language internal;

create function pg_catalog.regr_sxy(double precision, double precision) returns double precision language internal;

create function pg_catalog.regr_syy(double precision, double precision) returns double precision language internal;

create function pg_catalog.regrolein(cstring) returns regrole language internal;

create function pg_catalog.regroleout(regrole) returns cstring language internal;

create function pg_catalog.regrolerecv(internal) returns regrole language internal;

create function pg_catalog.regrolesend(regrole) returns bytea language internal;

create function pg_catalog.regtypein(cstring) returns regtype language internal;

create function pg_catalog.regtypeout(regtype) returns cstring language internal;

create function pg_catalog.regtyperecv(internal) returns regtype language internal;

create function pg_catalog.regtypesend(regtype) returns bytea language internal;

create function pg_catalog.repeat(text, integer) returns text language internal;

create function pg_catalog.replace(text, text, text) returns text language internal;

create function pg_catalog.reverse(text) returns text language internal;

create function pg_catalog.reverse(bytea) returns bytea language internal;

create function pg_catalog.right(text, integer) returns text language internal;

create function pg_catalog.round(double precision) returns double precision language internal;

create function pg_catalog.round(numeric, integer) returns numeric language internal;

create function pg_catalog.round(numeric) returns numeric language internal;

create function pg_catalog.row_number() returns bigint language internal;

create function pg_catalog.row_security_active(oid) returns boolean language internal;

create function pg_catalog.row_security_active(text) returns boolean language internal;

create function pg_catalog.row_to_json(record) returns json language internal;

create function pg_catalog.row_to_json(record, boolean) returns json language internal;

create function pg_catalog.rpad(text, integer, text) returns text language internal;

create function pg_catalog.rpad(text, integer) returns text language internal;

create function pg_catalog.rtrim(text) returns text language internal;

create function pg_catalog.rtrim(bytea, bytea) returns bytea language internal;

create function pg_catalog.rtrim(text, text) returns text language internal;

create function pg_catalog.satisfies_hash_partition(oid, integer, integer, VARIADIC "any") returns boolean language internal;

create function pg_catalog.scalargejoinsel(internal, oid, internal, smallint, internal) returns double precision language internal;

create function pg_catalog.scalargesel(internal, oid, internal, integer) returns double precision language internal;

create function pg_catalog.scalargtjoinsel(internal, oid, internal, smallint, internal) returns double precision language internal;

create function pg_catalog.scalargtsel(internal, oid, internal, integer) returns double precision language internal;

create function pg_catalog.scalarlejoinsel(internal, oid, internal, smallint, internal) returns double precision language internal;

create function pg_catalog.scalarlesel(internal, oid, internal, integer) returns double precision language internal;

create function pg_catalog.scalarltjoinsel(internal, oid, internal, smallint, internal) returns double precision language internal;

create function pg_catalog.scalarltsel(internal, oid, internal, integer) returns double precision language internal;

create function pg_catalog.scale(numeric) returns integer language internal;

create function pg_catalog.schema_to_xml(schema name, nulls boolean, tableforest boolean, targetns text) returns xml language internal;

create function pg_catalog.schema_to_xml_and_xmlschema(schema name, nulls boolean, tableforest boolean, targetns text) returns xml language internal;

create function pg_catalog.schema_to_xmlschema(schema name, nulls boolean, tableforest boolean, targetns text) returns xml language internal;

create function pg_catalog.session_user() returns name language internal;

create function pg_catalog.set_bit(bytea, bigint, integer) returns bytea language internal;

create function pg_catalog.set_bit(bit, integer, integer) returns bit language internal;

create function pg_catalog.set_byte(bytea, integer, integer) returns bytea language internal;

create function pg_catalog.set_config(text, text, boolean) returns text language internal;

create function pg_catalog.set_masklen(inet, integer) returns inet language internal;

create function pg_catalog.set_masklen(cidr, integer) returns cidr language internal;

create function pg_catalog.setseed(double precision) returns void language internal;

create function pg_catalog.setval(regclass, bigint, boolean) returns bigint language internal;

create function pg_catalog.setval(regclass, bigint) returns bigint language internal;

create function pg_catalog.setweight(tsvector, "char", text[]) returns tsvector language internal;

create function pg_catalog.setweight(tsvector, "char") returns tsvector language internal;

create function pg_catalog.sha224(bytea) returns bytea language internal;

create function pg_catalog.sha256(bytea) returns bytea language internal;

create function pg_catalog.sha384(bytea) returns bytea language internal;

create function pg_catalog.sha512(bytea) returns bytea language internal;

create function pg_catalog.shell_in(cstring) returns void language internal;

create function pg_catalog.shell_out(void) returns cstring language internal;

create function pg_catalog.shift_jis_2004_to_euc_jis_2004(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.shift_jis_2004_to_utf8(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.shobj_description(oid, name) returns text language internal;

create function pg_catalog.sign(double precision) returns double precision language internal;

create function pg_catalog.sign(numeric) returns numeric language internal;

create function pg_catalog.similar_escape(text, text) returns text language internal;

create function pg_catalog.similar_to_escape(text, text) returns text language internal;

create function pg_catalog.similar_to_escape(text) returns text language internal;

create function pg_catalog.sin(double precision) returns double precision language internal;

create function pg_catalog.sind(double precision) returns double precision language internal;

create function pg_catalog.sinh(double precision) returns double precision language internal;

create function pg_catalog.sjis_to_euc_jp(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.sjis_to_mic(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.sjis_to_utf8(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.slope(point, point) returns double precision language internal;

create function pg_catalog.spg_bbox_quad_config(internal, internal) returns void language internal;

create function pg_catalog.spg_box_quad_choose(internal, internal) returns void language internal;

create function pg_catalog.spg_box_quad_config(internal, internal) returns void language internal;

create function pg_catalog.spg_box_quad_inner_consistent(internal, internal) returns void language internal;

create function pg_catalog.spg_box_quad_leaf_consistent(internal, internal) returns boolean language internal;

create function pg_catalog.spg_box_quad_picksplit(internal, internal) returns void language internal;

create function pg_catalog.spg_kd_choose(internal, internal) returns void language internal;

create function pg_catalog.spg_kd_config(internal, internal) returns void language internal;

create function pg_catalog.spg_kd_inner_consistent(internal, internal) returns void language internal;

create function pg_catalog.spg_kd_picksplit(internal, internal) returns void language internal;

create function pg_catalog.spg_poly_quad_compress(polygon) returns box language internal;

create function pg_catalog.spg_quad_choose(internal, internal) returns void language internal;

create function pg_catalog.spg_quad_config(internal, internal) returns void language internal;

create function pg_catalog.spg_quad_inner_consistent(internal, internal) returns void language internal;

create function pg_catalog.spg_quad_leaf_consistent(internal, internal) returns boolean language internal;

create function pg_catalog.spg_quad_picksplit(internal, internal) returns void language internal;

create function pg_catalog.spg_range_quad_choose(internal, internal) returns void language internal;

create function pg_catalog.spg_range_quad_config(internal, internal) returns void language internal;

create function pg_catalog.spg_range_quad_inner_consistent(internal, internal) returns void language internal;

create function pg_catalog.spg_range_quad_leaf_consistent(internal, internal) returns boolean language internal;

create function pg_catalog.spg_range_quad_picksplit(internal, internal) returns void language internal;

create function pg_catalog.spg_text_choose(internal, internal) returns void language internal;

create function pg_catalog.spg_text_config(internal, internal) returns void language internal;

create function pg_catalog.spg_text_inner_consistent(internal, internal) returns void language internal;

create function pg_catalog.spg_text_leaf_consistent(internal, internal) returns boolean language internal;

create function pg_catalog.spg_text_picksplit(internal, internal) returns void language internal;

create function pg_catalog.spghandler(internal) returns index_am_handler language internal;

create function pg_catalog.split_part(text, text, integer) returns text language internal;

create function pg_catalog.sqrt(double precision) returns double precision language internal;

create function pg_catalog.sqrt(numeric) returns numeric language internal;

create function pg_catalog.starts_with(text, text) returns boolean language internal;

create function pg_catalog.statement_timestamp() returns timestamp with time zone language internal;

create function pg_catalog.stddev(integer) returns numeric language internal;

create function pg_catalog.stddev(bigint) returns numeric language internal;

create function pg_catalog.stddev(smallint) returns numeric language internal;

create function pg_catalog.stddev(real) returns double precision language internal;

create function pg_catalog.stddev(double precision) returns double precision language internal;

create function pg_catalog.stddev(numeric) returns numeric language internal;

create function pg_catalog.stddev_pop(integer) returns numeric language internal;

create function pg_catalog.stddev_pop(double precision) returns double precision language internal;

create function pg_catalog.stddev_pop(numeric) returns numeric language internal;

create function pg_catalog.stddev_pop(real) returns double precision language internal;

create function pg_catalog.stddev_pop(smallint) returns numeric language internal;

create function pg_catalog.stddev_pop(bigint) returns numeric language internal;

create function pg_catalog.stddev_samp(double precision) returns double precision language internal;

create function pg_catalog.stddev_samp(smallint) returns numeric language internal;

create function pg_catalog.stddev_samp(numeric) returns numeric language internal;

create function pg_catalog.stddev_samp(real) returns double precision language internal;

create function pg_catalog.stddev_samp(bigint) returns numeric language internal;

create function pg_catalog.stddev_samp(integer) returns numeric language internal;

create function pg_catalog.string_agg(value bytea, delimiter bytea) returns bytea language internal;

create function pg_catalog.string_agg(value text, delimiter text) returns text language internal;

create function pg_catalog.string_agg_combine(internal, internal) returns internal language internal;

create function pg_catalog.string_agg_deserialize(bytea, internal) returns internal language internal;

create function pg_catalog.string_agg_finalfn(internal) returns text language internal;

create function pg_catalog.string_agg_serialize(internal) returns bytea language internal;

create function pg_catalog.string_agg_transfn(internal, text, text) returns internal language internal;

create function pg_catalog.string_to_array(text, text, text) returns text[] language internal;

create function pg_catalog.string_to_array(text, text) returns text[] language internal;

create function pg_catalog.string_to_table(text, text, text) returns SETOF text language internal;

create function pg_catalog.string_to_table(text, text) returns SETOF text language internal;

create function pg_catalog.strip(tsvector) returns tsvector language internal;

create function pg_catalog.strpos(text, text) returns integer language internal;

create function pg_catalog.substr(text, integer) returns text language internal;

create function pg_catalog.substr(text, integer, integer) returns text language internal;

create function pg_catalog.substr(bytea, integer) returns bytea language internal;

create function pg_catalog.substr(bytea, integer, integer) returns bytea language internal;

create function pg_catalog.substring(text, integer) returns text language internal;

create function pg_catalog.substring(text, text, text) returns text language internal;

create function pg_catalog.substring(text, text) returns text language internal;

create function pg_catalog.substring(bytea, integer) returns bytea language internal;

create function pg_catalog.substring(bytea, integer, integer) returns bytea language internal;

create function pg_catalog.substring(bit, integer) returns bit language internal;

create function pg_catalog.substring(text, integer, integer) returns text language internal;

create function pg_catalog.substring(bit, integer, integer) returns bit language internal;

create function pg_catalog.sum(integer) returns bigint language internal;

create function pg_catalog.sum(numeric) returns numeric language internal;

create function pg_catalog.sum(interval) returns interval language internal;

create function pg_catalog.sum(money) returns money language internal;

create function pg_catalog.sum(double precision) returns double precision language internal;

create function pg_catalog.sum(real) returns real language internal;

create function pg_catalog.sum(smallint) returns bigint language internal;

create function pg_catalog.sum(bigint) returns numeric language internal;

create function pg_catalog.suppress_redundant_updates_trigger() returns trigger language internal;

create function pg_catalog.system(internal) returns tsm_handler language internal;

create function pg_catalog.system_user() returns text language internal;

create function pg_catalog.table_am_handler_in(cstring) returns table_am_handler language internal;

create function pg_catalog.table_am_handler_out(table_am_handler) returns cstring language internal;

create function pg_catalog.table_to_xml(tbl regclass, nulls boolean, tableforest boolean, targetns text) returns xml language internal;

create function pg_catalog.table_to_xml_and_xmlschema(tbl regclass, nulls boolean, tableforest boolean, targetns text) returns xml language internal;

create function pg_catalog.table_to_xmlschema(tbl regclass, nulls boolean, tableforest boolean, targetns text) returns xml language internal;

create function pg_catalog.tan(double precision) returns double precision language internal;

create function pg_catalog.tand(double precision) returns double precision language internal;

create function pg_catalog.tanh(double precision) returns double precision language internal;

create function pg_catalog.text(boolean) returns text language internal;

create function pg_catalog.text(name) returns text language internal;

create function pg_catalog.text("char") returns text language internal;

create function pg_catalog.text(inet) returns text language internal;

create function pg_catalog.text(xml) returns text language internal;

create function pg_catalog.text(character) returns text language internal;

create function pg_catalog.text_ge(text, text) returns boolean language internal;

create function pg_catalog.text_gt(text, text) returns boolean language internal;

create function pg_catalog.text_larger(text, text) returns text language internal;

create function pg_catalog.text_le(text, text) returns boolean language internal;

create function pg_catalog.text_lt(text, text) returns boolean language internal;

create function pg_catalog.text_pattern_ge(text, text) returns boolean language internal;

create function pg_catalog.text_pattern_gt(text, text) returns boolean language internal;

create function pg_catalog.text_pattern_le(text, text) returns boolean language internal;

create function pg_catalog.text_pattern_lt(text, text) returns boolean language internal;

create function pg_catalog.text_smaller(text, text) returns text language internal;

create function pg_catalog.text_starts_with_support(internal) returns internal language internal;

create function pg_catalog.textanycat(text, anynonarray) returns text language internal;

create function pg_catalog.textcat(text, text) returns text language internal;

create function pg_catalog.texteq(text, text) returns boolean language internal;

create function pg_catalog.texteqname(text, name) returns boolean language internal;

create function pg_catalog.textgename(text, name) returns boolean language internal;

create function pg_catalog.textgtname(text, name) returns boolean language internal;

create function pg_catalog.texticlike(text, text) returns boolean language internal;

create function pg_catalog.texticlike_support(internal) returns internal language internal;

create function pg_catalog.texticnlike(text, text) returns boolean language internal;

create function pg_catalog.texticregexeq(text, text) returns boolean language internal;

create function pg_catalog.texticregexeq_support(internal) returns internal language internal;

create function pg_catalog.texticregexne(text, text) returns boolean language internal;

create function pg_catalog.textin(cstring) returns text language internal;

create function pg_catalog.textlen(text) returns integer language internal;

create function pg_catalog.textlename(text, name) returns boolean language internal;

create function pg_catalog.textlike(text, text) returns boolean language internal;

create function pg_catalog.textlike_support(internal) returns internal language internal;

create function pg_catalog.textltname(text, name) returns boolean language internal;

create function pg_catalog.textne(text, text) returns boolean language internal;

create function pg_catalog.textnename(text, name) returns boolean language internal;

create function pg_catalog.textnlike(text, text) returns boolean language internal;

create function pg_catalog.textout(text) returns cstring language internal;

create function pg_catalog.textrecv(internal) returns text language internal;

create function pg_catalog.textregexeq(text, text) returns boolean language internal;

create function pg_catalog.textregexeq_support(internal) returns internal language internal;

create function pg_catalog.textregexne(text, text) returns boolean language internal;

create function pg_catalog.textsend(text) returns bytea language internal;

create function pg_catalog.thesaurus_init(internal) returns internal language internal;

create function pg_catalog.thesaurus_lexize(internal, internal, internal, internal) returns internal language internal;

create function pg_catalog.tideq(tid, tid) returns boolean language internal;

create function pg_catalog.tidge(tid, tid) returns boolean language internal;

create function pg_catalog.tidgt(tid, tid) returns boolean language internal;

create function pg_catalog.tidin(cstring) returns tid language internal;

create function pg_catalog.tidlarger(tid, tid) returns tid language internal;

create function pg_catalog.tidle(tid, tid) returns boolean language internal;

create function pg_catalog.tidlt(tid, tid) returns boolean language internal;

create function pg_catalog.tidne(tid, tid) returns boolean language internal;

create function pg_catalog.tidout(tid) returns cstring language internal;

create function pg_catalog.tidrecv(internal) returns tid language internal;

create function pg_catalog.tidsend(tid) returns bytea language internal;

create function pg_catalog.tidsmaller(tid, tid) returns tid language internal;

create function pg_catalog.time(timestamp with time zone) returns time without time zone language internal;

create function pg_catalog.time(interval) returns time without time zone language internal;

create function pg_catalog.time(timestamp without time zone) returns time without time zone language internal;

create function pg_catalog.time(time with time zone) returns time without time zone language internal;

create function pg_catalog.time(time without time zone, integer) returns time without time zone language internal;

create function pg_catalog.time_cmp(time without time zone, time without time zone) returns integer language internal;

create function pg_catalog.time_eq(time without time zone, time without time zone) returns boolean language internal;

create function pg_catalog.time_ge(time without time zone, time without time zone) returns boolean language internal;

create function pg_catalog.time_gt(time without time zone, time without time zone) returns boolean language internal;

create function pg_catalog.time_hash(time without time zone) returns integer language internal;

create function pg_catalog.time_hash_extended(time without time zone, bigint) returns bigint language internal;

create function pg_catalog.time_in(cstring, oid, integer) returns time without time zone language internal;

create function pg_catalog.time_larger(time without time zone, time without time zone) returns time without time zone language internal;

create function pg_catalog.time_le(time without time zone, time without time zone) returns boolean language internal;

create function pg_catalog.time_lt(time without time zone, time without time zone) returns boolean language internal;

create function pg_catalog.time_mi_interval(time without time zone, interval) returns time without time zone language internal;

create function pg_catalog.time_mi_time(time without time zone, time without time zone) returns interval language internal;

create function pg_catalog.time_ne(time without time zone, time without time zone) returns boolean language internal;

create function pg_catalog.time_out(time without time zone) returns cstring language internal;

create function pg_catalog.time_pl_interval(time without time zone, interval) returns time without time zone language internal;

create function pg_catalog.time_recv(internal, oid, integer) returns time without time zone language internal;

create function pg_catalog.time_send(time without time zone) returns bytea language internal;

create function pg_catalog.time_smaller(time without time zone, time without time zone) returns time without time zone language internal;

create function pg_catalog.time_support(internal) returns internal language internal;

create function pg_catalog.timedate_pl(time without time zone, date) returns timestamp without time zone language internal;

create function pg_catalog.timeofday() returns text language internal;

create function pg_catalog.timestamp(timestamp with time zone) returns timestamp without time zone language internal;

create function pg_catalog.timestamp(date, time without time zone) returns timestamp without time zone language internal;

create function pg_catalog.timestamp(timestamp without time zone, integer) returns timestamp without time zone language internal;

create function pg_catalog.timestamp(date) returns timestamp without time zone language internal;

create function pg_catalog.timestamp_cmp(timestamp without time zone, timestamp without time zone) returns integer language internal;

create function pg_catalog.timestamp_cmp_date(timestamp without time zone, date) returns integer language internal;

create function pg_catalog.timestamp_cmp_timestamptz(timestamp without time zone, timestamp with time zone) returns integer language internal;

create function pg_catalog.timestamp_eq(timestamp without time zone, timestamp without time zone) returns boolean language internal;

create function pg_catalog.timestamp_eq_date(timestamp without time zone, date) returns boolean language internal;

create function pg_catalog.timestamp_eq_timestamptz(timestamp without time zone, timestamp with time zone) returns boolean language internal;

create function pg_catalog.timestamp_ge(timestamp without time zone, timestamp without time zone) returns boolean language internal;

create function pg_catalog.timestamp_ge_date(timestamp without time zone, date) returns boolean language internal;

create function pg_catalog.timestamp_ge_timestamptz(timestamp without time zone, timestamp with time zone) returns boolean language internal;

create function pg_catalog.timestamp_gt(timestamp without time zone, timestamp without time zone) returns boolean language internal;

create function pg_catalog.timestamp_gt_date(timestamp without time zone, date) returns boolean language internal;

create function pg_catalog.timestamp_gt_timestamptz(timestamp without time zone, timestamp with time zone) returns boolean language internal;

create function pg_catalog.timestamp_hash(timestamp without time zone) returns integer language internal;

create function pg_catalog.timestamp_hash_extended(timestamp without time zone, bigint) returns bigint language internal;

create function pg_catalog.timestamp_in(cstring, oid, integer) returns timestamp without time zone language internal;

create function pg_catalog.timestamp_larger(timestamp without time zone, timestamp without time zone) returns timestamp without time zone language internal;

create function pg_catalog.timestamp_le(timestamp without time zone, timestamp without time zone) returns boolean language internal;

create function pg_catalog.timestamp_le_date(timestamp without time zone, date) returns boolean language internal;

create function pg_catalog.timestamp_le_timestamptz(timestamp without time zone, timestamp with time zone) returns boolean language internal;

create function pg_catalog.timestamp_lt(timestamp without time zone, timestamp without time zone) returns boolean language internal;

create function pg_catalog.timestamp_lt_date(timestamp without time zone, date) returns boolean language internal;

create function pg_catalog.timestamp_lt_timestamptz(timestamp without time zone, timestamp with time zone) returns boolean language internal;

create function pg_catalog.timestamp_mi(timestamp without time zone, timestamp without time zone) returns interval language internal;

create function pg_catalog.timestamp_mi_interval(timestamp without time zone, interval) returns timestamp without time zone language internal;

create function pg_catalog.timestamp_ne(timestamp without time zone, timestamp without time zone) returns boolean language internal;

create function pg_catalog.timestamp_ne_date(timestamp without time zone, date) returns boolean language internal;

create function pg_catalog.timestamp_ne_timestamptz(timestamp without time zone, timestamp with time zone) returns boolean language internal;

create function pg_catalog.timestamp_out(timestamp without time zone) returns cstring language internal;

create function pg_catalog.timestamp_pl_interval(timestamp without time zone, interval) returns timestamp without time zone language internal;

create function pg_catalog.timestamp_recv(internal, oid, integer) returns timestamp without time zone language internal;

create function pg_catalog.timestamp_send(timestamp without time zone) returns bytea language internal;

create function pg_catalog.timestamp_skipsupport(internal) returns void language internal;

create function pg_catalog.timestamp_smaller(timestamp without time zone, timestamp without time zone) returns timestamp without time zone language internal;

create function pg_catalog.timestamp_sortsupport(internal) returns void language internal;

create function pg_catalog.timestamp_support(internal) returns internal language internal;

create function pg_catalog.timestamptypmodin(cstring[]) returns integer language internal;

create function pg_catalog.timestamptypmodout(integer) returns cstring language internal;

create function pg_catalog.timestamptz(date, time without time zone) returns timestamp with time zone language internal;

create function pg_catalog.timestamptz(timestamp without time zone) returns timestamp with time zone language internal;

create function pg_catalog.timestamptz(date) returns timestamp with time zone language internal;

create function pg_catalog.timestamptz(date, time with time zone) returns timestamp with time zone language internal;

create function pg_catalog.timestamptz(timestamp with time zone, integer) returns timestamp with time zone language internal;

create function pg_catalog.timestamptz_cmp(timestamp with time zone, timestamp with time zone) returns integer language internal;

create function pg_catalog.timestamptz_cmp_date(timestamp with time zone, date) returns integer language internal;

create function pg_catalog.timestamptz_cmp_timestamp(timestamp with time zone, timestamp without time zone) returns integer language internal;

create function pg_catalog.timestamptz_eq(timestamp with time zone, timestamp with time zone) returns boolean language internal;

create function pg_catalog.timestamptz_eq_date(timestamp with time zone, date) returns boolean language internal;

create function pg_catalog.timestamptz_eq_timestamp(timestamp with time zone, timestamp without time zone) returns boolean language internal;

create function pg_catalog.timestamptz_ge(timestamp with time zone, timestamp with time zone) returns boolean language internal;

create function pg_catalog.timestamptz_ge_date(timestamp with time zone, date) returns boolean language internal;

create function pg_catalog.timestamptz_ge_timestamp(timestamp with time zone, timestamp without time zone) returns boolean language internal;

create function pg_catalog.timestamptz_gt(timestamp with time zone, timestamp with time zone) returns boolean language internal;

create function pg_catalog.timestamptz_gt_date(timestamp with time zone, date) returns boolean language internal;

create function pg_catalog.timestamptz_gt_timestamp(timestamp with time zone, timestamp without time zone) returns boolean language internal;

create function pg_catalog.timestamptz_hash(timestamp with time zone) returns integer language internal;

create function pg_catalog.timestamptz_hash_extended(timestamp with time zone, bigint) returns bigint language internal;

create function pg_catalog.timestamptz_in(cstring, oid, integer) returns timestamp with time zone language internal;

create function pg_catalog.timestamptz_larger(timestamp with time zone, timestamp with time zone) returns timestamp with time zone language internal;

create function pg_catalog.timestamptz_le(timestamp with time zone, timestamp with time zone) returns boolean language internal;

create function pg_catalog.timestamptz_le_date(timestamp with time zone, date) returns boolean language internal;

create function pg_catalog.timestamptz_le_timestamp(timestamp with time zone, timestamp without time zone) returns boolean language internal;

create function pg_catalog.timestamptz_lt(timestamp with time zone, timestamp with time zone) returns boolean language internal;

create function pg_catalog.timestamptz_lt_date(timestamp with time zone, date) returns boolean language internal;

create function pg_catalog.timestamptz_lt_timestamp(timestamp with time zone, timestamp without time zone) returns boolean language internal;

create function pg_catalog.timestamptz_mi(timestamp with time zone, timestamp with time zone) returns interval language internal;

create function pg_catalog.timestamptz_mi_interval(timestamp with time zone, interval) returns timestamp with time zone language internal;

create function pg_catalog.timestamptz_ne(timestamp with time zone, timestamp with time zone) returns boolean language internal;

create function pg_catalog.timestamptz_ne_date(timestamp with time zone, date) returns boolean language internal;

create function pg_catalog.timestamptz_ne_timestamp(timestamp with time zone, timestamp without time zone) returns boolean language internal;

create function pg_catalog.timestamptz_out(timestamp with time zone) returns cstring language internal;

create function pg_catalog.timestamptz_pl_interval(timestamp with time zone, interval) returns timestamp with time zone language internal;

create function pg_catalog.timestamptz_recv(internal, oid, integer) returns timestamp with time zone language internal;

create function pg_catalog.timestamptz_send(timestamp with time zone) returns bytea language internal;

create function pg_catalog.timestamptz_smaller(timestamp with time zone, timestamp with time zone) returns timestamp with time zone language internal;

create function pg_catalog.timestamptztypmodin(cstring[]) returns integer language internal;

create function pg_catalog.timestamptztypmodout(integer) returns cstring language internal;

create function pg_catalog.timetypmodin(cstring[]) returns integer language internal;

create function pg_catalog.timetypmodout(integer) returns cstring language internal;

create function pg_catalog.timetz(time with time zone, integer) returns time with time zone language internal;

create function pg_catalog.timetz(time without time zone) returns time with time zone language internal;

create function pg_catalog.timetz(timestamp with time zone) returns time with time zone language internal;

create function pg_catalog.timetz_cmp(time with time zone, time with time zone) returns integer language internal;

create function pg_catalog.timetz_eq(time with time zone, time with time zone) returns boolean language internal;

create function pg_catalog.timetz_ge(time with time zone, time with time zone) returns boolean language internal;

create function pg_catalog.timetz_gt(time with time zone, time with time zone) returns boolean language internal;

create function pg_catalog.timetz_hash(time with time zone) returns integer language internal;

create function pg_catalog.timetz_hash_extended(time with time zone, bigint) returns bigint language internal;

create function pg_catalog.timetz_in(cstring, oid, integer) returns time with time zone language internal;

create function pg_catalog.timetz_larger(time with time zone, time with time zone) returns time with time zone language internal;

create function pg_catalog.timetz_le(time with time zone, time with time zone) returns boolean language internal;

create function pg_catalog.timetz_lt(time with time zone, time with time zone) returns boolean language internal;

create function pg_catalog.timetz_mi_interval(time with time zone, interval) returns time with time zone language internal;

create function pg_catalog.timetz_ne(time with time zone, time with time zone) returns boolean language internal;

create function pg_catalog.timetz_out(time with time zone) returns cstring language internal;

create function pg_catalog.timetz_pl_interval(time with time zone, interval) returns time with time zone language internal;

create function pg_catalog.timetz_recv(internal, oid, integer) returns time with time zone language internal;

create function pg_catalog.timetz_send(time with time zone) returns bytea language internal;

create function pg_catalog.timetz_smaller(time with time zone, time with time zone) returns time with time zone language internal;

create function pg_catalog.timetzdate_pl(time with time zone, date) returns timestamp with time zone language internal;

create function pg_catalog.timetztypmodin(cstring[]) returns integer language internal;

create function pg_catalog.timetztypmodout(integer) returns cstring language internal;

create function pg_catalog.timezone(timestamp with time zone) returns timestamp without time zone language internal;

create function pg_catalog.timezone(timestamp without time zone) returns timestamp with time zone language internal;

create function pg_catalog.timezone(text, timestamp with time zone) returns timestamp without time zone language internal;

create function pg_catalog.timezone(time with time zone) returns time with time zone language internal;

create function pg_catalog.timezone(interval, time with time zone) returns time with time zone language internal;

create function pg_catalog.timezone(text, time with time zone) returns time with time zone language internal;

create function pg_catalog.timezone(text, timestamp without time zone) returns timestamp with time zone language internal;

create function pg_catalog.timezone(interval, timestamp without time zone) returns timestamp with time zone language internal;

create function pg_catalog.timezone(interval, timestamp with time zone) returns timestamp without time zone language internal;

create function pg_catalog.to_ascii(text) returns text language internal;

create function pg_catalog.to_ascii(text, integer) returns text language internal;

create function pg_catalog.to_ascii(text, name) returns text language internal;

create function pg_catalog.to_bin(bigint) returns text language internal;

create function pg_catalog.to_bin(integer) returns text language internal;

create function pg_catalog.to_char(double precision, text) returns text language internal;

create function pg_catalog.to_char(real, text) returns text language internal;

create function pg_catalog.to_char(bigint, text) returns text language internal;

create function pg_catalog.to_char(integer, text) returns text language internal;

create function pg_catalog.to_char(numeric, text) returns text language internal;

create function pg_catalog.to_char(timestamp with time zone, text) returns text language internal;

create function pg_catalog.to_char(timestamp without time zone, text) returns text language internal;

create function pg_catalog.to_char(interval, text) returns text language internal;

create function pg_catalog.to_date(text, text) returns date language internal;

create function pg_catalog.to_hex(bigint) returns text language internal;

create function pg_catalog.to_hex(integer) returns text language internal;

create function pg_catalog.to_json(anyelement) returns json language internal;

create function pg_catalog.to_jsonb(anyelement) returns jsonb language internal;

create function pg_catalog.to_number(text, text) returns numeric language internal;

create function pg_catalog.to_oct(integer) returns text language internal;

create function pg_catalog.to_oct(bigint) returns text language internal;

create function pg_catalog.to_regclass(text) returns regclass language internal;

create function pg_catalog.to_regcollation(text) returns regcollation language internal;

create function pg_catalog.to_regnamespace(text) returns regnamespace language internal;

create function pg_catalog.to_regoper(text) returns regoper language internal;

create function pg_catalog.to_regoperator(text) returns regoperator language internal;

create function pg_catalog.to_regproc(text) returns regproc language internal;

create function pg_catalog.to_regprocedure(text) returns regprocedure language internal;

create function pg_catalog.to_regrole(text) returns regrole language internal;

create function pg_catalog.to_regtype(text) returns regtype language internal;

create function pg_catalog.to_regtypemod(text) returns integer language internal;

create function pg_catalog.to_timestamp(double precision) returns timestamp with time zone language internal;

create function pg_catalog.to_timestamp(text, text) returns timestamp with time zone language internal;

create function pg_catalog.to_tsquery(regconfig, text) returns tsquery language internal;

create function pg_catalog.to_tsquery(text) returns tsquery language internal;

create function pg_catalog.to_tsvector(jsonb) returns tsvector language internal;

create function pg_catalog.to_tsvector(regconfig, text) returns tsvector language internal;

create function pg_catalog.to_tsvector(regconfig, jsonb) returns tsvector language internal;

create function pg_catalog.to_tsvector(regconfig, json) returns tsvector language internal;

create function pg_catalog.to_tsvector(json) returns tsvector language internal;

create function pg_catalog.to_tsvector(text) returns tsvector language internal;

create function pg_catalog.transaction_timestamp() returns timestamp with time zone language internal;

create function pg_catalog.translate(text, text, text) returns text language internal;

create function pg_catalog.trigger_in(cstring) returns trigger language internal;

create function pg_catalog.trigger_out(trigger) returns cstring language internal;

create function pg_catalog.trim_array(anyarray, integer) returns anyarray language internal;

create function pg_catalog.trim_scale(numeric) returns numeric language internal;

create function pg_catalog.trunc(numeric, integer) returns numeric language internal;

create function pg_catalog.trunc(numeric) returns numeric language internal;

create function pg_catalog.trunc(macaddr8) returns macaddr8 language internal;

create function pg_catalog.trunc(macaddr) returns macaddr language internal;

create function pg_catalog.trunc(double precision) returns double precision language internal;

create function pg_catalog.ts_debug(document text, OUT alias text, OUT description text, OUT token text, OUT dictionaries regdictionary[], OUT dictionary regdictionary, OUT lexemes text[]) returns SETOF record language internal;

create function pg_catalog.ts_debug(config regconfig, document text, OUT alias text, OUT description text, OUT token text, OUT dictionaries regdictionary[], OUT dictionary regdictionary, OUT lexemes text[]) returns SETOF record language internal;

create function pg_catalog.ts_delete(tsvector, text) returns tsvector language internal;

create function pg_catalog.ts_delete(tsvector, text[]) returns tsvector language internal;

create function pg_catalog.ts_filter(tsvector, "char"[]) returns tsvector language internal;

create function pg_catalog.ts_headline(json, tsquery, text) returns json language internal;

create function pg_catalog.ts_headline(json, tsquery) returns json language internal;

create function pg_catalog.ts_headline(regconfig, json, tsquery) returns json language internal;

create function pg_catalog.ts_headline(regconfig, json, tsquery, text) returns json language internal;

create function pg_catalog.ts_headline(jsonb, tsquery) returns jsonb language internal;

create function pg_catalog.ts_headline(jsonb, tsquery, text) returns jsonb language internal;

create function pg_catalog.ts_headline(regconfig, jsonb, tsquery) returns jsonb language internal;

create function pg_catalog.ts_headline(regconfig, jsonb, tsquery, text) returns jsonb language internal;

create function pg_catalog.ts_headline(text, tsquery) returns text language internal;

create function pg_catalog.ts_headline(text, tsquery, text) returns text language internal;

create function pg_catalog.ts_headline(regconfig, text, tsquery) returns text language internal;

create function pg_catalog.ts_headline(regconfig, text, tsquery, text) returns text language internal;

create function pg_catalog.ts_lexize(regdictionary, text) returns text[] language internal;

create function pg_catalog.ts_match_qv(tsquery, tsvector) returns boolean language internal;

create function pg_catalog.ts_match_tq(text, tsquery) returns boolean language internal;

create function pg_catalog.ts_match_tt(text, text) returns boolean language internal;

create function pg_catalog.ts_match_vq(tsvector, tsquery) returns boolean language internal;

create function pg_catalog.ts_parse(parser_oid oid, txt text, OUT tokid integer, OUT token text) returns SETOF record language internal;

create function pg_catalog.ts_parse(parser_name text, txt text, OUT tokid integer, OUT token text) returns SETOF record language internal;

create function pg_catalog.ts_rank(tsvector, tsquery) returns real language internal;

create function pg_catalog.ts_rank(real[], tsvector, tsquery, integer) returns real language internal;

create function pg_catalog.ts_rank(real[], tsvector, tsquery) returns real language internal;

create function pg_catalog.ts_rank(tsvector, tsquery, integer) returns real language internal;

create function pg_catalog.ts_rank_cd(real[], tsvector, tsquery) returns real language internal;

create function pg_catalog.ts_rank_cd(tsvector, tsquery) returns real language internal;

create function pg_catalog.ts_rank_cd(real[], tsvector, tsquery, integer) returns real language internal;

create function pg_catalog.ts_rank_cd(tsvector, tsquery, integer) returns real language internal;

create function pg_catalog.ts_rewrite(tsquery, tsquery, tsquery) returns tsquery language internal;

create function pg_catalog.ts_rewrite(tsquery, text) returns tsquery language internal;

create function pg_catalog.ts_stat(query text, weights text, OUT word text, OUT ndoc integer, OUT nentry integer) returns SETOF record language internal;

create function pg_catalog.ts_stat(query text, OUT word text, OUT ndoc integer, OUT nentry integer) returns SETOF record language internal;

create function pg_catalog.ts_token_type(parser_oid oid, OUT tokid integer, OUT alias text, OUT description text) returns SETOF record language internal;

create function pg_catalog.ts_token_type(parser_name text, OUT tokid integer, OUT alias text, OUT description text) returns SETOF record language internal;

create function pg_catalog.ts_typanalyze(internal) returns boolean language internal;

create function pg_catalog.tsm_handler_in(cstring) returns tsm_handler language internal;

create function pg_catalog.tsm_handler_out(tsm_handler) returns cstring language internal;

create function pg_catalog.tsmatchjoinsel(internal, oid, internal, smallint, internal) returns double precision language internal;

create function pg_catalog.tsmatchsel(internal, oid, internal, integer) returns double precision language internal;

create function pg_catalog.tsmultirange() returns tsmultirange language internal;

create function pg_catalog.tsmultirange(tsrange) returns tsmultirange language internal;

create function pg_catalog.tsmultirange(VARIADIC tsrange[]) returns tsmultirange language internal;

create function pg_catalog.tsq_mcontained(tsquery, tsquery) returns boolean language internal;

create function pg_catalog.tsq_mcontains(tsquery, tsquery) returns boolean language internal;

create function pg_catalog.tsquery_and(tsquery, tsquery) returns tsquery language internal;

create function pg_catalog.tsquery_cmp(tsquery, tsquery) returns integer language internal;

create function pg_catalog.tsquery_eq(tsquery, tsquery) returns boolean language internal;

create function pg_catalog.tsquery_ge(tsquery, tsquery) returns boolean language internal;

create function pg_catalog.tsquery_gt(tsquery, tsquery) returns boolean language internal;

create function pg_catalog.tsquery_le(tsquery, tsquery) returns boolean language internal;

create function pg_catalog.tsquery_lt(tsquery, tsquery) returns boolean language internal;

create function pg_catalog.tsquery_ne(tsquery, tsquery) returns boolean language internal;

create function pg_catalog.tsquery_not(tsquery) returns tsquery language internal;

create function pg_catalog.tsquery_or(tsquery, tsquery) returns tsquery language internal;

create function pg_catalog.tsquery_phrase(tsquery, tsquery, integer) returns tsquery language internal;

create function pg_catalog.tsquery_phrase(tsquery, tsquery) returns tsquery language internal;

create function pg_catalog.tsqueryin(cstring) returns tsquery language internal;

create function pg_catalog.tsqueryout(tsquery) returns cstring language internal;

create function pg_catalog.tsqueryrecv(internal) returns tsquery language internal;

create function pg_catalog.tsquerysend(tsquery) returns bytea language internal;

create function pg_catalog.tsrange(timestamp without time zone, timestamp without time zone, text) returns tsrange language internal;

create function pg_catalog.tsrange(timestamp without time zone, timestamp without time zone) returns tsrange language internal;

create function pg_catalog.tsrange_subdiff(timestamp without time zone, timestamp without time zone) returns double precision language internal;

create function pg_catalog.tstzmultirange(tstzrange) returns tstzmultirange language internal;

create function pg_catalog.tstzmultirange(VARIADIC tstzrange[]) returns tstzmultirange language internal;

create function pg_catalog.tstzmultirange() returns tstzmultirange language internal;

create function pg_catalog.tstzrange(timestamp with time zone, timestamp with time zone) returns tstzrange language internal;

create function pg_catalog.tstzrange(timestamp with time zone, timestamp with time zone, text) returns tstzrange language internal;

create function pg_catalog.tstzrange_subdiff(timestamp with time zone, timestamp with time zone) returns double precision language internal;

create function pg_catalog.tsvector_cmp(tsvector, tsvector) returns integer language internal;

create function pg_catalog.tsvector_concat(tsvector, tsvector) returns tsvector language internal;

create function pg_catalog.tsvector_eq(tsvector, tsvector) returns boolean language internal;

create function pg_catalog.tsvector_ge(tsvector, tsvector) returns boolean language internal;

create function pg_catalog.tsvector_gt(tsvector, tsvector) returns boolean language internal;

create function pg_catalog.tsvector_le(tsvector, tsvector) returns boolean language internal;

create function pg_catalog.tsvector_lt(tsvector, tsvector) returns boolean language internal;

create function pg_catalog.tsvector_ne(tsvector, tsvector) returns boolean language internal;

create function pg_catalog.tsvector_to_array(tsvector) returns text[] language internal;

create function pg_catalog.tsvector_update_trigger() returns trigger language internal;

create function pg_catalog.tsvector_update_trigger_column() returns trigger language internal;

create function pg_catalog.tsvectorin(cstring) returns tsvector language internal;

create function pg_catalog.tsvectorout(tsvector) returns cstring language internal;

create function pg_catalog.tsvectorrecv(internal) returns tsvector language internal;

create function pg_catalog.tsvectorsend(tsvector) returns bytea language internal;

create function pg_catalog.txid_current() returns bigint language internal;

create function pg_catalog.txid_current_if_assigned() returns bigint language internal;

create function pg_catalog.txid_current_snapshot() returns txid_snapshot language internal;

create function pg_catalog.txid_snapshot_in(cstring) returns txid_snapshot language internal;

create function pg_catalog.txid_snapshot_out(txid_snapshot) returns cstring language internal;

create function pg_catalog.txid_snapshot_recv(internal) returns txid_snapshot language internal;

create function pg_catalog.txid_snapshot_send(txid_snapshot) returns bytea language internal;

create function pg_catalog.txid_snapshot_xip(txid_snapshot) returns SETOF bigint language internal;

create function pg_catalog.txid_snapshot_xmax(txid_snapshot) returns bigint language internal;

create function pg_catalog.txid_snapshot_xmin(txid_snapshot) returns bigint language internal;

create function pg_catalog.txid_status(bigint) returns text language internal;

create function pg_catalog.txid_visible_in_snapshot(bigint, txid_snapshot) returns boolean language internal;

create function pg_catalog.uhc_to_utf8(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.unicode_assigned(text) returns boolean language internal;

create function pg_catalog.unicode_version() returns text language internal;

create function pg_catalog.unique_key_recheck() returns trigger language internal;

create function pg_catalog.unistr(text) returns text language internal;

create function pg_catalog.unknownin(cstring) returns unknown language internal;

create function pg_catalog.unknownout(unknown) returns cstring language internal;

create function pg_catalog.unknownrecv(internal) returns unknown language internal;

create function pg_catalog.unknownsend(unknown) returns bytea language internal;

create function pg_catalog.unnest(anymultirange) returns SETOF anyrange language internal;

create function pg_catalog.unnest(tsvector tsvector, OUT lexeme text, OUT positions smallint[], OUT weights text[]) returns SETOF record language internal;

create function pg_catalog.unnest(anyarray) returns SETOF anyelement language internal;

create function pg_catalog.upper(text) returns text language internal;

create function pg_catalog.upper(anymultirange) returns anyelement language internal;

create function pg_catalog.upper(anyrange) returns anyelement language internal;

create function pg_catalog.upper_inc(anyrange) returns boolean language internal;

create function pg_catalog.upper_inc(anymultirange) returns boolean language internal;

create function pg_catalog.upper_inf(anymultirange) returns boolean language internal;

create function pg_catalog.upper_inf(anyrange) returns boolean language internal;

create function pg_catalog.utf8_to_big5(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.utf8_to_euc_cn(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.utf8_to_euc_jis_2004(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.utf8_to_euc_jp(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.utf8_to_euc_kr(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.utf8_to_euc_tw(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.utf8_to_gb18030(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.utf8_to_gbk(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.utf8_to_iso8859(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.utf8_to_iso8859_1(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.utf8_to_johab(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.utf8_to_koi8r(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.utf8_to_koi8u(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.utf8_to_shift_jis_2004(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.utf8_to_sjis(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.utf8_to_uhc(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.utf8_to_win(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.uuid_cmp(uuid, uuid) returns integer language internal;

create function pg_catalog.uuid_eq(uuid, uuid) returns boolean language internal;

create function pg_catalog.uuid_extract_timestamp(uuid) returns timestamp with time zone language internal;

create function pg_catalog.uuid_extract_version(uuid) returns smallint language internal;

create function pg_catalog.uuid_ge(uuid, uuid) returns boolean language internal;

create function pg_catalog.uuid_gt(uuid, uuid) returns boolean language internal;

create function pg_catalog.uuid_hash(uuid) returns integer language internal;

create function pg_catalog.uuid_hash_extended(uuid, bigint) returns bigint language internal;

create function pg_catalog.uuid_in(cstring) returns uuid language internal;

create function pg_catalog.uuid_le(uuid, uuid) returns boolean language internal;

create function pg_catalog.uuid_lt(uuid, uuid) returns boolean language internal;

create function pg_catalog.uuid_ne(uuid, uuid) returns boolean language internal;

create function pg_catalog.uuid_out(uuid) returns cstring language internal;

create function pg_catalog.uuid_recv(internal) returns uuid language internal;

create function pg_catalog.uuid_send(uuid) returns bytea language internal;

create function pg_catalog.uuid_skipsupport(internal) returns void language internal;

create function pg_catalog.uuid_sortsupport(internal) returns void language internal;

create function pg_catalog.uuidv4() returns uuid language internal;

create function pg_catalog.uuidv7(shift interval) returns uuid language internal;

create function pg_catalog.uuidv7() returns uuid language internal;

create function pg_catalog.var_pop(bigint) returns numeric language internal;

create function pg_catalog.var_pop(double precision) returns double precision language internal;

create function pg_catalog.var_pop(real) returns double precision language internal;

create function pg_catalog.var_pop(smallint) returns numeric language internal;

create function pg_catalog.var_pop(integer) returns numeric language internal;

create function pg_catalog.var_pop(numeric) returns numeric language internal;

create function pg_catalog.var_samp(numeric) returns numeric language internal;

create function pg_catalog.var_samp(bigint) returns numeric language internal;

create function pg_catalog.var_samp(integer) returns numeric language internal;

create function pg_catalog.var_samp(smallint) returns numeric language internal;

create function pg_catalog.var_samp(real) returns double precision language internal;

create function pg_catalog.var_samp(double precision) returns double precision language internal;

create function pg_catalog.varbit(bit varying, integer, boolean) returns bit varying language internal;

create function pg_catalog.varbit_in(cstring, oid, integer) returns bit varying language internal;

create function pg_catalog.varbit_out(bit varying) returns cstring language internal;

create function pg_catalog.varbit_recv(internal, oid, integer) returns bit varying language internal;

create function pg_catalog.varbit_send(bit varying) returns bytea language internal;

create function pg_catalog.varbit_support(internal) returns internal language internal;

create function pg_catalog.varbitcmp(bit varying, bit varying) returns integer language internal;

create function pg_catalog.varbiteq(bit varying, bit varying) returns boolean language internal;

create function pg_catalog.varbitge(bit varying, bit varying) returns boolean language internal;

create function pg_catalog.varbitgt(bit varying, bit varying) returns boolean language internal;

create function pg_catalog.varbitle(bit varying, bit varying) returns boolean language internal;

create function pg_catalog.varbitlt(bit varying, bit varying) returns boolean language internal;

create function pg_catalog.varbitne(bit varying, bit varying) returns boolean language internal;

create function pg_catalog.varbittypmodin(cstring[]) returns integer language internal;

create function pg_catalog.varbittypmodout(integer) returns cstring language internal;

create function pg_catalog.varchar(name) returns character varying language internal;

create function pg_catalog.varchar(character varying, integer, boolean) returns character varying language internal;

create function pg_catalog.varchar_support(internal) returns internal language internal;

create function pg_catalog.varcharin(cstring, oid, integer) returns character varying language internal;

create function pg_catalog.varcharout(character varying) returns cstring language internal;

create function pg_catalog.varcharrecv(internal, oid, integer) returns character varying language internal;

create function pg_catalog.varcharsend(character varying) returns bytea language internal;

create function pg_catalog.varchartypmodin(cstring[]) returns integer language internal;

create function pg_catalog.varchartypmodout(integer) returns cstring language internal;

create function pg_catalog.variance(bigint) returns numeric language internal;

create function pg_catalog.variance(integer) returns numeric language internal;

create function pg_catalog.variance(smallint) returns numeric language internal;

create function pg_catalog.variance(real) returns double precision language internal;

create function pg_catalog.variance(double precision) returns double precision language internal;

create function pg_catalog.variance(numeric) returns numeric language internal;

create function pg_catalog.version() returns text language internal;

create function pg_catalog.void_in(cstring) returns void language internal;

create function pg_catalog.void_out(void) returns cstring language internal;

create function pg_catalog.void_recv(internal) returns void language internal;

create function pg_catalog.void_send(void) returns bytea language internal;

create function pg_catalog.websearch_to_tsquery(text) returns tsquery language internal;

create function pg_catalog.websearch_to_tsquery(regconfig, text) returns tsquery language internal;

create function pg_catalog.width(box) returns double precision language internal;

create function pg_catalog.width_bucket(numeric, numeric, numeric, integer) returns integer language internal;

create function pg_catalog.width_bucket(double precision, double precision, double precision, integer) returns integer language internal;

create function pg_catalog.width_bucket(anycompatible, anycompatiblearray) returns integer language internal;

create function pg_catalog.win1250_to_latin2(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.win1250_to_mic(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.win1251_to_iso(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.win1251_to_koi8r(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.win1251_to_mic(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.win1251_to_win866(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.win866_to_iso(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.win866_to_koi8r(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.win866_to_mic(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.win866_to_win1251(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.win_to_utf8(integer, integer, cstring, internal, integer, boolean) returns integer language internal;

create function pg_catalog.window_cume_dist_support(internal) returns internal language internal;

create function pg_catalog.window_dense_rank_support(internal) returns internal language internal;

create function pg_catalog.window_ntile_support(internal) returns internal language internal;

create function pg_catalog.window_percent_rank_support(internal) returns internal language internal;

create function pg_catalog.window_rank_support(internal) returns internal language internal;

create function pg_catalog.window_row_number_support(internal) returns internal language internal;

create function pg_catalog.xid(xid8) returns xid language internal;

create function pg_catalog.xid8_larger(xid8, xid8) returns xid8 language internal;

create function pg_catalog.xid8_smaller(xid8, xid8) returns xid8 language internal;

create function pg_catalog.xid8cmp(xid8, xid8) returns integer language internal;

create function pg_catalog.xid8eq(xid8, xid8) returns boolean language internal;

create function pg_catalog.xid8ge(xid8, xid8) returns boolean language internal;

create function pg_catalog.xid8gt(xid8, xid8) returns boolean language internal;

create function pg_catalog.xid8in(cstring) returns xid8 language internal;

create function pg_catalog.xid8le(xid8, xid8) returns boolean language internal;

create function pg_catalog.xid8lt(xid8, xid8) returns boolean language internal;

create function pg_catalog.xid8ne(xid8, xid8) returns boolean language internal;

create function pg_catalog.xid8out(xid8) returns cstring language internal;

create function pg_catalog.xid8recv(internal) returns xid8 language internal;

create function pg_catalog.xid8send(xid8) returns bytea language internal;

create function pg_catalog.xideq(xid, xid) returns boolean language internal;

create function pg_catalog.xideqint4(xid, integer) returns boolean language internal;

create function pg_catalog.xidin(cstring) returns xid language internal;

create function pg_catalog.xidneq(xid, xid) returns boolean language internal;

create function pg_catalog.xidneqint4(xid, integer) returns boolean language internal;

create function pg_catalog.xidout(xid) returns cstring language internal;

create function pg_catalog.xidrecv(internal) returns xid language internal;

create function pg_catalog.xidsend(xid) returns bytea language internal;

create function pg_catalog.xml(text) returns xml language internal;

create function pg_catalog.xml_in(cstring) returns xml language internal;

create function pg_catalog.xml_is_well_formed(text) returns boolean language internal;

create function pg_catalog.xml_is_well_formed_content(text) returns boolean language internal;

create function pg_catalog.xml_is_well_formed_document(text) returns boolean language internal;

create function pg_catalog.xml_out(xml) returns cstring language internal;

create function pg_catalog.xml_recv(internal) returns xml language internal;

create function pg_catalog.xml_send(xml) returns bytea language internal;

create function pg_catalog.xmlagg(xml) returns xml language internal;

create function pg_catalog.xmlcomment(text) returns xml language internal;

create function pg_catalog.xmlconcat2(xml, xml) returns xml language internal;

create function pg_catalog.xmlexists(text, xml) returns boolean language internal;

create function pg_catalog.xmltext(text) returns xml language internal;

create function pg_catalog.xmlvalidate(xml, text) returns boolean language internal;

create function pg_catalog.xpath(text, xml, text[]) returns xml[] language internal;

create function pg_catalog.xpath(text, xml) returns xml[] language internal;

create function pg_catalog.xpath_exists(text, xml) returns boolean language internal;

create function pg_catalog.xpath_exists(text, xml, text[]) returns boolean language internal;

create operator pg_catalog.!! (rightarg = tsquery);

create operator pg_catalog.!~ (leftarg = text, rightarg = text);

create operator pg_catalog.!~ (leftarg = character, rightarg = text);

create operator pg_catalog.!~ (leftarg = name, rightarg = text);

create operator pg_catalog.!~* (leftarg = name, rightarg = text);

create operator pg_catalog.!~* (leftarg = text, rightarg = text);

create operator pg_catalog.!~* (leftarg = character, rightarg = text);

create operator pg_catalog.!~~ (leftarg = character, rightarg = text);

create operator pg_catalog.!~~ (leftarg = name, rightarg = text);

create operator pg_catalog.!~~ (leftarg = bytea, rightarg = bytea);

create operator pg_catalog.!~~ (leftarg = text, rightarg = text);

create operator pg_catalog.!~~* (leftarg = text, rightarg = text);

create operator pg_catalog.!~~* (leftarg = character, rightarg = text);

create operator pg_catalog.!~~* (leftarg = name, rightarg = text);

create operator pg_catalog.# (leftarg = lseg, rightarg = lseg);

create operator pg_catalog.# (leftarg = bigint, rightarg = bigint);

create operator pg_catalog.# (leftarg = integer, rightarg = integer);

create operator pg_catalog.# (leftarg = box, rightarg = box);

create operator pg_catalog.# (rightarg = path);

create operator pg_catalog.# (leftarg = line, rightarg = line);

create operator pg_catalog.# (leftarg = bit, rightarg = bit);

create operator pg_catalog.# (leftarg = smallint, rightarg = smallint);

create operator pg_catalog.# (rightarg = polygon);

create operator pg_catalog.## (leftarg = lseg, rightarg = lseg);

create operator pg_catalog.## (leftarg = point, rightarg = line);

create operator pg_catalog.## (leftarg = point, rightarg = lseg);

create operator pg_catalog.## (leftarg = point, rightarg = box);

create operator pg_catalog.## (leftarg = lseg, rightarg = box);

create operator pg_catalog.## (leftarg = line, rightarg = lseg);

create operator pg_catalog.#- (leftarg = jsonb, rightarg = text[]);

create operator pg_catalog.#> (leftarg = jsonb, rightarg = text[]);

create operator pg_catalog.#> (leftarg = json, rightarg = text[]);

create operator pg_catalog.#>> (leftarg = json, rightarg = text[]);

create operator pg_catalog.#>> (leftarg = jsonb, rightarg = text[]);

create operator pg_catalog.% (leftarg = integer, rightarg = integer);

create operator pg_catalog.% (leftarg = bigint, rightarg = bigint);

create operator pg_catalog.% (leftarg = numeric, rightarg = numeric);

create operator pg_catalog.% (leftarg = smallint, rightarg = smallint);

create operator pg_catalog.& (leftarg = macaddr8, rightarg = macaddr8);

create operator pg_catalog.& (leftarg = bigint, rightarg = bigint);

create operator pg_catalog.& (leftarg = integer, rightarg = integer);

create operator pg_catalog.& (leftarg = smallint, rightarg = smallint);

create operator pg_catalog.& (leftarg = macaddr, rightarg = macaddr);

create operator pg_catalog.& (leftarg = inet, rightarg = inet);

create operator pg_catalog.& (leftarg = bit, rightarg = bit);

create operator pg_catalog.&& (leftarg = anyrange, rightarg = anymultirange);

create operator pg_catalog.&& (leftarg = tsquery, rightarg = tsquery);

create operator pg_catalog.&& (leftarg = anyarray, rightarg = anyarray);

create operator pg_catalog.&& (leftarg = inet, rightarg = inet);

create operator pg_catalog.&& (leftarg = box, rightarg = box);

create operator pg_catalog.&& (leftarg = circle, rightarg = circle);

create operator pg_catalog.&& (leftarg = anyrange, rightarg = anyrange);

create operator pg_catalog.&& (leftarg = anymultirange, rightarg = anymultirange);

create operator pg_catalog.&& (leftarg = anymultirange, rightarg = anyrange);

create operator pg_catalog.&& (leftarg = polygon, rightarg = polygon);

create operator pg_catalog.&< (leftarg = circle, rightarg = circle);

create operator pg_catalog.&< (leftarg = anyrange, rightarg = anymultirange);

create operator pg_catalog.&< (leftarg = anymultirange, rightarg = anyrange);

create operator pg_catalog.&< (leftarg = anymultirange, rightarg = anymultirange);

create operator pg_catalog.&< (leftarg = anyrange, rightarg = anyrange);

create operator pg_catalog.&< (leftarg = polygon, rightarg = polygon);

create operator pg_catalog.&< (leftarg = box, rightarg = box);

create operator pg_catalog.&<| (leftarg = polygon, rightarg = polygon);

create operator pg_catalog.&<| (leftarg = circle, rightarg = circle);

create operator pg_catalog.&<| (leftarg = box, rightarg = box);

create operator pg_catalog.&> (leftarg = circle, rightarg = circle);

create operator pg_catalog.&> (leftarg = polygon, rightarg = polygon);

create operator pg_catalog.&> (leftarg = box, rightarg = box);

create operator pg_catalog.&> (leftarg = anymultirange, rightarg = anymultirange);

create operator pg_catalog.&> (leftarg = anyrange, rightarg = anymultirange);

create operator pg_catalog.&> (leftarg = anymultirange, rightarg = anyrange);

create operator pg_catalog.&> (leftarg = anyrange, rightarg = anyrange);

create operator pg_catalog.* (leftarg = box, rightarg = point);

create operator pg_catalog.* (leftarg = real, rightarg = double precision);

create operator pg_catalog.* (leftarg = anymultirange, rightarg = anymultirange);

create operator pg_catalog.* (leftarg = numeric, rightarg = numeric);

create operator pg_catalog.* (leftarg = integer, rightarg = integer);

create operator pg_catalog.* (leftarg = smallint, rightarg = smallint);

create operator pg_catalog.* (leftarg = smallint, rightarg = integer);

create operator pg_catalog.* (leftarg = integer, rightarg = smallint);

create operator pg_catalog.* (leftarg = real, rightarg = real);

create operator pg_catalog.* (leftarg = double precision, rightarg = double precision);

create operator pg_catalog.* (leftarg = double precision, rightarg = interval);

create operator pg_catalog.* (leftarg = interval, rightarg = double precision);

create operator pg_catalog.* (leftarg = anyrange, rightarg = anyrange);

create operator pg_catalog.* (leftarg = circle, rightarg = point);

create operator pg_catalog.* (leftarg = bigint, rightarg = bigint);

create operator pg_catalog.* (leftarg = bigint, rightarg = integer);

create operator pg_catalog.* (leftarg = integer, rightarg = bigint);

create operator pg_catalog.* (leftarg = bigint, rightarg = smallint);

create operator pg_catalog.* (leftarg = smallint, rightarg = bigint);

create operator pg_catalog.* (leftarg = point, rightarg = point);

create operator pg_catalog.* (leftarg = path, rightarg = point);

create operator pg_catalog.* (leftarg = double precision, rightarg = real);

create operator pg_catalog.* (leftarg = money, rightarg = real);

create operator pg_catalog.* (leftarg = real, rightarg = money);

create operator pg_catalog.* (leftarg = money, rightarg = double precision);

create operator pg_catalog.* (leftarg = money, rightarg = bigint);

create operator pg_catalog.* (leftarg = money, rightarg = integer);

create operator pg_catalog.* (leftarg = money, rightarg = smallint);

create operator pg_catalog.* (leftarg = double precision, rightarg = money);

create operator pg_catalog.* (leftarg = bigint, rightarg = money);

create operator pg_catalog.* (leftarg = integer, rightarg = money);

create operator pg_catalog.* (leftarg = smallint, rightarg = money);

create operator pg_catalog.*< (leftarg = record, rightarg = record);

create operator pg_catalog.*<= (leftarg = record, rightarg = record);

create operator pg_catalog.*<> (leftarg = record, rightarg = record);

create operator pg_catalog.*= (leftarg = record, rightarg = record);

create operator pg_catalog.*> (leftarg = record, rightarg = record);

create operator pg_catalog.*>= (leftarg = record, rightarg = record);

create operator pg_catalog.+ (leftarg = bigint, rightarg = bigint);

create operator pg_catalog.+ (leftarg = real, rightarg = double precision);

create operator pg_catalog.+ (leftarg = pg_lsn, rightarg = numeric);

create operator pg_catalog.+ (leftarg = numeric, rightarg = pg_lsn);

create operator pg_catalog.+ (leftarg = aclitem[], rightarg = aclitem);

create operator pg_catalog.+ (leftarg = timestamp without time zone, rightarg = interval);

create operator pg_catalog.+ (leftarg = date, rightarg = integer);

create operator pg_catalog.+ (leftarg = date, rightarg = interval);

create operator pg_catalog.+ (leftarg = box, rightarg = point);

create operator pg_catalog.+ (leftarg = integer, rightarg = date);

create operator pg_catalog.+ (leftarg = interval, rightarg = timestamp with time zone);

create operator pg_catalog.+ (leftarg = circle, rightarg = point);

create operator pg_catalog.+ (leftarg = double precision, rightarg = real);

create operator pg_catalog.+ (leftarg = timestamp with time zone, rightarg = interval);

create operator pg_catalog.+ (rightarg = bigint);

create operator pg_catalog.+ (leftarg = interval, rightarg = interval);

create operator pg_catalog.+ (leftarg = money, rightarg = money);

create operator pg_catalog.+ (leftarg = date, rightarg = time without time zone);

create operator pg_catalog.+ (leftarg = date, rightarg = time with time zone);

create operator pg_catalog.+ (leftarg = smallint, rightarg = smallint);

create operator pg_catalog.+ (leftarg = integer, rightarg = integer);

create operator pg_catalog.+ (leftarg = smallint, rightarg = integer);

create operator pg_catalog.+ (leftarg = integer, rightarg = smallint);

create operator pg_catalog.+ (leftarg = inet, rightarg = bigint);

create operator pg_catalog.+ (leftarg = time with time zone, rightarg = interval);

create operator pg_catalog.+ (leftarg = bigint, rightarg = inet);

create operator pg_catalog.+ (leftarg = interval, rightarg = time without time zone);

create operator pg_catalog.+ (leftarg = time without time zone, rightarg = interval);

create operator pg_catalog.+ (leftarg = real, rightarg = real);

create operator pg_catalog.+ (leftarg = numeric, rightarg = numeric);

create operator pg_catalog.+ (leftarg = time without time zone, rightarg = date);

create operator pg_catalog.+ (leftarg = time with time zone, rightarg = date);

create operator pg_catalog.+ (leftarg = double precision, rightarg = double precision);

create operator pg_catalog.+ (leftarg = path, rightarg = point);

create operator pg_catalog.+ (leftarg = anymultirange, rightarg = anymultirange);

create operator pg_catalog.+ (rightarg = smallint);

create operator pg_catalog.+ (rightarg = integer);

create operator pg_catalog.+ (leftarg = smallint, rightarg = bigint);

create operator pg_catalog.+ (leftarg = path, rightarg = path);

create operator pg_catalog.+ (leftarg = anyrange, rightarg = anyrange);

create operator pg_catalog.+ (leftarg = point, rightarg = point);

create operator pg_catalog.+ (leftarg = interval, rightarg = date);

create operator pg_catalog.+ (leftarg = interval, rightarg = time with time zone);

create operator pg_catalog.+ (leftarg = interval, rightarg = timestamp without time zone);

create operator pg_catalog.+ (rightarg = real);

create operator pg_catalog.+ (rightarg = double precision);

create operator pg_catalog.+ (rightarg = numeric);

create operator pg_catalog.+ (leftarg = bigint, rightarg = smallint);

create operator pg_catalog.+ (leftarg = integer, rightarg = bigint);

create operator pg_catalog.+ (leftarg = bigint, rightarg = integer);

create operator pg_catalog.- (leftarg = inet, rightarg = bigint);

create operator pg_catalog.- (leftarg = time with time zone, rightarg = interval);

create operator pg_catalog.- (leftarg = smallint, rightarg = smallint);

create operator pg_catalog.- (leftarg = integer, rightarg = integer);

create operator pg_catalog.- (leftarg = smallint, rightarg = integer);

create operator pg_catalog.- (leftarg = integer, rightarg = smallint);

create operator pg_catalog.- (rightarg = integer);

create operator pg_catalog.- (rightarg = smallint);

create operator pg_catalog.- (rightarg = real);

create operator pg_catalog.- (rightarg = double precision);

create operator pg_catalog.- (leftarg = real, rightarg = real);

create operator pg_catalog.- (leftarg = money, rightarg = money);

create operator pg_catalog.- (leftarg = double precision, rightarg = double precision);

create operator pg_catalog.- (leftarg = aclitem[], rightarg = aclitem);

create operator pg_catalog.- (leftarg = jsonb, rightarg = text);

create operator pg_catalog.- (leftarg = pg_lsn, rightarg = pg_lsn);

create operator pg_catalog.- (leftarg = pg_lsn, rightarg = numeric);

create operator pg_catalog.- (leftarg = timestamp without time zone, rightarg = timestamp without time zone);

create operator pg_catalog.- (leftarg = timestamp without time zone, rightarg = interval);

create operator pg_catalog.- (leftarg = anyrange, rightarg = anyrange);

create operator pg_catalog.- (leftarg = circle, rightarg = point);

create operator pg_catalog.- (leftarg = date, rightarg = date);

create operator pg_catalog.- (leftarg = date, rightarg = integer);

create operator pg_catalog.- (leftarg = bigint, rightarg = bigint);

create operator pg_catalog.- (leftarg = bigint, rightarg = integer);

create operator pg_catalog.- (leftarg = double precision, rightarg = real);

create operator pg_catalog.- (leftarg = integer, rightarg = bigint);

create operator pg_catalog.- (leftarg = bigint, rightarg = smallint);

create operator pg_catalog.- (leftarg = smallint, rightarg = bigint);

create operator pg_catalog.- (leftarg = time without time zone, rightarg = time without time zone);

create operator pg_catalog.- (leftarg = point, rightarg = point);

create operator pg_catalog.- (leftarg = path, rightarg = point);

create operator pg_catalog.- (leftarg = interval, rightarg = interval);

create operator pg_catalog.- (rightarg = interval);

create operator pg_catalog.- (leftarg = timestamp with time zone, rightarg = interval);

create operator pg_catalog.- (leftarg = timestamp with time zone, rightarg = timestamp with time zone);

create operator pg_catalog.- (leftarg = date, rightarg = interval);

create operator pg_catalog.- (leftarg = box, rightarg = point);

create operator pg_catalog.- (leftarg = jsonb, rightarg = integer);

create operator pg_catalog.- (leftarg = jsonb, rightarg = text[]);

create operator pg_catalog.- (leftarg = real, rightarg = double precision);

create operator pg_catalog.- (leftarg = anymultirange, rightarg = anymultirange);

create operator pg_catalog.- (leftarg = numeric, rightarg = numeric);

create operator pg_catalog.- (rightarg = numeric);

create operator pg_catalog.- (rightarg = bigint);

create operator pg_catalog.- (leftarg = time without time zone, rightarg = interval);

create operator pg_catalog.- (leftarg = inet, rightarg = inet);

create operator pg_catalog.-> (leftarg = jsonb, rightarg = integer);

create operator pg_catalog.-> (leftarg = json, rightarg = integer);

create operator pg_catalog.-> (leftarg = json, rightarg = text);

create operator pg_catalog.-> (leftarg = jsonb, rightarg = text);

create operator pg_catalog.->> (leftarg = json, rightarg = text);

create operator pg_catalog.->> (leftarg = jsonb, rightarg = text);

create operator pg_catalog.->> (leftarg = jsonb, rightarg = integer);

create operator pg_catalog.->> (leftarg = json, rightarg = integer);

create operator pg_catalog.-|- (leftarg = anyrange, rightarg = anymultirange);

create operator pg_catalog.-|- (leftarg = anymultirange, rightarg = anymultirange);

create operator pg_catalog.-|- (leftarg = anyrange, rightarg = anyrange);

create operator pg_catalog.-|- (leftarg = anymultirange, rightarg = anyrange);

create operator pg_catalog./ (leftarg = money, rightarg = double precision);

create operator pg_catalog./ (leftarg = money, rightarg = integer);

create operator pg_catalog./ (leftarg = bigint, rightarg = bigint);

create operator pg_catalog./ (leftarg = bigint, rightarg = integer);

create operator pg_catalog./ (leftarg = money, rightarg = smallint);

create operator pg_catalog./ (leftarg = integer, rightarg = bigint);

create operator pg_catalog./ (leftarg = numeric, rightarg = numeric);

create operator pg_catalog./ (leftarg = bigint, rightarg = smallint);

create operator pg_catalog./ (leftarg = double precision, rightarg = real);

create operator pg_catalog./ (leftarg = smallint, rightarg = bigint);

create operator pg_catalog./ (leftarg = interval, rightarg = double precision);

create operator pg_catalog./ (leftarg = point, rightarg = point);

create operator pg_catalog./ (leftarg = double precision, rightarg = double precision);

create operator pg_catalog./ (leftarg = path, rightarg = point);

create operator pg_catalog./ (leftarg = real, rightarg = real);

create operator pg_catalog./ (leftarg = integer, rightarg = smallint);

create operator pg_catalog./ (leftarg = smallint, rightarg = integer);

create operator pg_catalog./ (leftarg = integer, rightarg = integer);

create operator pg_catalog./ (leftarg = smallint, rightarg = smallint);

create operator pg_catalog./ (leftarg = money, rightarg = money);

create operator pg_catalog./ (leftarg = box, rightarg = point);

create operator pg_catalog./ (leftarg = money, rightarg = real);

create operator pg_catalog./ (leftarg = real, rightarg = double precision);

create operator pg_catalog./ (leftarg = money, rightarg = bigint);

create operator pg_catalog./ (leftarg = circle, rightarg = point);

create operator pg_catalog.< (leftarg = date, rightarg = date);

create operator pg_catalog.< (leftarg = xid8, rightarg = xid8);

create operator pg_catalog.< (leftarg = time without time zone, rightarg = time without time zone);

create operator pg_catalog.< (leftarg = uuid, rightarg = uuid);

create operator pg_catalog.< (leftarg = text, rightarg = name);

create operator pg_catalog.< (leftarg = text, rightarg = text);

create operator pg_catalog.< (leftarg = anyrange, rightarg = anyrange);

create operator pg_catalog.< (leftarg = path, rightarg = path);

create operator pg_catalog.< (leftarg = real, rightarg = double precision);

create operator pg_catalog.< (leftarg = numeric, rightarg = numeric);

create operator pg_catalog.< (leftarg = interval, rightarg = interval);

create operator pg_catalog.< (leftarg = time with time zone, rightarg = time with time zone);

create operator pg_catalog.< (leftarg = anyenum, rightarg = anyenum);

create operator pg_catalog.< (leftarg = anymultirange, rightarg = anymultirange);

create operator pg_catalog.< (leftarg = timestamp with time zone, rightarg = timestamp without time zone);

create operator pg_catalog.< (leftarg = tid, rightarg = tid);

create operator pg_catalog.< (leftarg = timestamp with time zone, rightarg = timestamp with time zone);

create operator pg_catalog.< (leftarg = boolean, rightarg = boolean);

create operator pg_catalog.< (leftarg = macaddr, rightarg = macaddr);

create operator pg_catalog.< (leftarg = bigint, rightarg = bigint);

create operator pg_catalog.< (leftarg = timestamp without time zone, rightarg = timestamp without time zone);

create operator pg_catalog.< (leftarg = integer, rightarg = smallint);

create operator pg_catalog.< (leftarg = record, rightarg = record);

create operator pg_catalog.< (leftarg = bit varying, rightarg = bit varying);

create operator pg_catalog.< (leftarg = bigint, rightarg = integer);

create operator pg_catalog.< (leftarg = integer, rightarg = bigint);

create operator pg_catalog.< (leftarg = bigint, rightarg = smallint);

create operator pg_catalog.< (leftarg = macaddr8, rightarg = macaddr8);

create operator pg_catalog.< (leftarg = double precision, rightarg = double precision);

create operator pg_catalog.< (leftarg = smallint, rightarg = bigint);

create operator pg_catalog.< (leftarg = real, rightarg = real);

create operator pg_catalog.< (leftarg = inet, rightarg = inet);

create operator pg_catalog.< (leftarg = tsvector, rightarg = tsvector);

create operator pg_catalog.< (leftarg = bytea, rightarg = bytea);

create operator pg_catalog.< (leftarg = date, rightarg = timestamp without time zone);

create operator pg_catalog.< (leftarg = "char", rightarg = "char");

create operator pg_catalog.< (leftarg = smallint, rightarg = integer);

create operator pg_catalog.< (leftarg = pg_lsn, rightarg = pg_lsn);

create operator pg_catalog.< (leftarg = bit, rightarg = bit);

create operator pg_catalog.< (leftarg = date, rightarg = timestamp with time zone);

create operator pg_catalog.< (leftarg = timestamp without time zone, rightarg = date);

create operator pg_catalog.< (leftarg = timestamp with time zone, rightarg = date);

create operator pg_catalog.< (leftarg = name, rightarg = text);

create operator pg_catalog.< (leftarg = money, rightarg = money);

create operator pg_catalog.< (leftarg = smallint, rightarg = smallint);

create operator pg_catalog.< (leftarg = timestamp without time zone, rightarg = timestamp with time zone);

create operator pg_catalog.< (leftarg = integer, rightarg = integer);

create operator pg_catalog.< (leftarg = lseg, rightarg = lseg);

create operator pg_catalog.< (leftarg = box, rightarg = box);

create operator pg_catalog.< (leftarg = oid, rightarg = oid);

create operator pg_catalog.< (leftarg = anyarray, rightarg = anyarray);

create operator pg_catalog.< (leftarg = double precision, rightarg = real);

create operator pg_catalog.< (leftarg = circle, rightarg = circle);

create operator pg_catalog.< (leftarg = oidvector, rightarg = oidvector);

create operator pg_catalog.< (leftarg = name, rightarg = name);

create operator pg_catalog.< (leftarg = jsonb, rightarg = jsonb);

create operator pg_catalog.< (leftarg = character, rightarg = character);

create operator pg_catalog.< (leftarg = tsquery, rightarg = tsquery);

create operator pg_catalog.<-> (leftarg = tsquery, rightarg = tsquery);

create operator pg_catalog.<-> (leftarg = point, rightarg = point);

create operator pg_catalog.<-> (leftarg = point, rightarg = line);

create operator pg_catalog.<-> (leftarg = line, rightarg = point);

create operator pg_catalog.<-> (leftarg = point, rightarg = lseg);

create operator pg_catalog.<-> (leftarg = lseg, rightarg = point);

create operator pg_catalog.<-> (leftarg = point, rightarg = box);

create operator pg_catalog.<-> (leftarg = box, rightarg = point);

create operator pg_catalog.<-> (leftarg = lseg, rightarg = line);

create operator pg_catalog.<-> (leftarg = line, rightarg = lseg);

create operator pg_catalog.<-> (leftarg = lseg, rightarg = box);

create operator pg_catalog.<-> (leftarg = box, rightarg = lseg);

create operator pg_catalog.<-> (leftarg = point, rightarg = path);

create operator pg_catalog.<-> (leftarg = path, rightarg = point);

create operator pg_catalog.<-> (leftarg = box, rightarg = box);

create operator pg_catalog.<-> (leftarg = path, rightarg = path);

create operator pg_catalog.<-> (leftarg = line, rightarg = line);

create operator pg_catalog.<-> (leftarg = lseg, rightarg = lseg);

create operator pg_catalog.<-> (leftarg = polygon, rightarg = polygon);

create operator pg_catalog.<-> (leftarg = circle, rightarg = circle);

create operator pg_catalog.<-> (leftarg = point, rightarg = circle);

create operator pg_catalog.<-> (leftarg = circle, rightarg = point);

create operator pg_catalog.<-> (leftarg = point, rightarg = polygon);

create operator pg_catalog.<-> (leftarg = polygon, rightarg = point);

create operator pg_catalog.<-> (leftarg = circle, rightarg = polygon);

create operator pg_catalog.<-> (leftarg = polygon, rightarg = circle);

create operator pg_catalog.<< (leftarg = anymultirange, rightarg = anyrange);

create operator pg_catalog.<< (leftarg = anyrange, rightarg = anyrange);

create operator pg_catalog.<< (leftarg = integer, rightarg = integer);

create operator pg_catalog.<< (leftarg = bigint, rightarg = integer);

create operator pg_catalog.<< (leftarg = anyrange, rightarg = anymultirange);

create operator pg_catalog.<< (leftarg = polygon, rightarg = polygon);

create operator pg_catalog.<< (leftarg = anymultirange, rightarg = anymultirange);

create operator pg_catalog.<< (leftarg = smallint, rightarg = integer);

create operator pg_catalog.<< (leftarg = point, rightarg = point);

create operator pg_catalog.<< (leftarg = inet, rightarg = inet);

create operator pg_catalog.<< (leftarg = circle, rightarg = circle);

create operator pg_catalog.<< (leftarg = box, rightarg = box);

create operator pg_catalog.<< (leftarg = bit, rightarg = integer);

create operator pg_catalog.<<= (leftarg = inet, rightarg = inet);

create operator pg_catalog.<<| (leftarg = circle, rightarg = circle);

create operator pg_catalog.<<| (leftarg = polygon, rightarg = polygon);

create operator pg_catalog.<<| (leftarg = box, rightarg = box);

create operator pg_catalog.<<| (leftarg = point, rightarg = point);

create operator pg_catalog.<= (leftarg = jsonb, rightarg = jsonb);

create operator pg_catalog.<= (leftarg = boolean, rightarg = boolean);

create operator pg_catalog.<= (leftarg = double precision, rightarg = real);

create operator pg_catalog.<= (leftarg = record, rightarg = record);

create operator pg_catalog.<= (leftarg = integer, rightarg = integer);

create operator pg_catalog.<= (leftarg = smallint, rightarg = smallint);

create operator pg_catalog.<= (leftarg = bigint, rightarg = integer);

create operator pg_catalog.<= (leftarg = numeric, rightarg = numeric);

create operator pg_catalog.<= (leftarg = bit, rightarg = bit);

create operator pg_catalog.<= (leftarg = box, rightarg = box);

create operator pg_catalog.<= (leftarg = date, rightarg = date);

create operator pg_catalog.<= (leftarg = money, rightarg = money);

create operator pg_catalog.<= (leftarg = timestamp with time zone, rightarg = timestamp without time zone);

create operator pg_catalog.<= (leftarg = anyarray, rightarg = anyarray);

create operator pg_catalog.<= (leftarg = pg_lsn, rightarg = pg_lsn);

create operator pg_catalog.<= (leftarg = timestamp with time zone, rightarg = timestamp with time zone);

create operator pg_catalog.<= (leftarg = timestamp without time zone, rightarg = timestamp with time zone);

create operator pg_catalog.<= (leftarg = anymultirange, rightarg = anymultirange);

create operator pg_catalog.<= (leftarg = timestamp with time zone, rightarg = date);

create operator pg_catalog.<= (leftarg = interval, rightarg = interval);

create operator pg_catalog.<= (leftarg = name, rightarg = text);

create operator pg_catalog.<= (leftarg = path, rightarg = path);

create operator pg_catalog.<= (leftarg = timestamp without time zone, rightarg = date);

create operator pg_catalog.<= (leftarg = date, rightarg = timestamp with time zone);

create operator pg_catalog.<= (leftarg = date, rightarg = timestamp without time zone);

create operator pg_catalog.<= (leftarg = circle, rightarg = circle);

create operator pg_catalog.<= (leftarg = tsquery, rightarg = tsquery);

create operator pg_catalog.<= (leftarg = double precision, rightarg = double precision);

create operator pg_catalog.<= (leftarg = integer, rightarg = bigint);

create operator pg_catalog.<= (leftarg = time without time zone, rightarg = time without time zone);

create operator pg_catalog.<= (leftarg = text, rightarg = name);

create operator pg_catalog.<= (leftarg = text, rightarg = text);

create operator pg_catalog.<= (leftarg = timestamp without time zone, rightarg = timestamp without time zone);

create operator pg_catalog.<= (leftarg = name, rightarg = name);

create operator pg_catalog.<= (leftarg = uuid, rightarg = uuid);

create operator pg_catalog.<= (leftarg = bytea, rightarg = bytea);

create operator pg_catalog.<= (leftarg = "char", rightarg = "char");

create operator pg_catalog.<= (leftarg = real, rightarg = double precision);

create operator pg_catalog.<= (leftarg = real, rightarg = real);

create operator pg_catalog.<= (leftarg = anyenum, rightarg = anyenum);

create operator pg_catalog.<= (leftarg = oidvector, rightarg = oidvector);

create operator pg_catalog.<= (leftarg = oid, rightarg = oid);

create operator pg_catalog.<= (leftarg = xid8, rightarg = xid8);

create operator pg_catalog.<= (leftarg = lseg, rightarg = lseg);

create operator pg_catalog.<= (leftarg = tsvector, rightarg = tsvector);

create operator pg_catalog.<= (leftarg = character, rightarg = character);

create operator pg_catalog.<= (leftarg = integer, rightarg = smallint);

create operator pg_catalog.<= (leftarg = macaddr, rightarg = macaddr);

create operator pg_catalog.<= (leftarg = smallint, rightarg = integer);

create operator pg_catalog.<= (leftarg = tid, rightarg = tid);

create operator pg_catalog.<= (leftarg = time with time zone, rightarg = time with time zone);

create operator pg_catalog.<= (leftarg = macaddr8, rightarg = macaddr8);

create operator pg_catalog.<= (leftarg = bigint, rightarg = smallint);

create operator pg_catalog.<= (leftarg = smallint, rightarg = bigint);

create operator pg_catalog.<= (leftarg = anyrange, rightarg = anyrange);

create operator pg_catalog.<= (leftarg = inet, rightarg = inet);

create operator pg_catalog.<= (leftarg = bit varying, rightarg = bit varying);

create operator pg_catalog.<= (leftarg = bigint, rightarg = bigint);

create operator pg_catalog.<> (leftarg = oid, rightarg = oid);

create operator pg_catalog.<> (leftarg = smallint, rightarg = smallint);

create operator pg_catalog.<> (leftarg = character, rightarg = character);

create operator pg_catalog.<> (leftarg = tsvector, rightarg = tsvector);

create operator pg_catalog.<> (leftarg = macaddr, rightarg = macaddr);

create operator pg_catalog.<> (leftarg = text, rightarg = name);

create operator pg_catalog.<> (leftarg = anyrange, rightarg = anyrange);

create operator pg_catalog.<> (leftarg = lseg, rightarg = lseg);

create operator pg_catalog.<> (leftarg = timestamp with time zone, rightarg = date);

create operator pg_catalog.<> (leftarg = anymultirange, rightarg = anymultirange);

create operator pg_catalog.<> (leftarg = smallint, rightarg = integer);

create operator pg_catalog.<> (leftarg = anyarray, rightarg = anyarray);

create operator pg_catalog.<> (leftarg = interval, rightarg = interval);

create operator pg_catalog.<> (leftarg = smallint, rightarg = bigint);

create operator pg_catalog.<> (leftarg = boolean, rightarg = boolean);

create operator pg_catalog.<> (leftarg = timestamp without time zone, rightarg = date);

create operator pg_catalog.<> (leftarg = tsquery, rightarg = tsquery);

create operator pg_catalog.<> (leftarg = text, rightarg = text);

create operator pg_catalog.<> (leftarg = time with time zone, rightarg = time with time zone);

create operator pg_catalog.<> (leftarg = date, rightarg = timestamp with time zone);

create operator pg_catalog.<> (leftarg = point, rightarg = point);

create operator pg_catalog.<> (leftarg = date, rightarg = timestamp without time zone);

create operator pg_catalog.<> (leftarg = real, rightarg = double precision);

create operator pg_catalog.<> (leftarg = circle, rightarg = circle);

create operator pg_catalog.<> (leftarg = name, rightarg = text);

create operator pg_catalog.<> (leftarg = integer, rightarg = integer);

create operator pg_catalog.<> (leftarg = bigint, rightarg = smallint);

create operator pg_catalog.<> (leftarg = double precision, rightarg = double precision);

create operator pg_catalog.<> (leftarg = record, rightarg = record);

create operator pg_catalog.<> (leftarg = uuid, rightarg = uuid);

create operator pg_catalog.<> (leftarg = name, rightarg = name);

create operator pg_catalog.<> (leftarg = timestamp without time zone, rightarg = timestamp without time zone);

create operator pg_catalog.<> (leftarg = integer, rightarg = bigint);

create operator pg_catalog.<> (leftarg = bigint, rightarg = bigint);

create operator pg_catalog.<> (leftarg = numeric, rightarg = numeric);

create operator pg_catalog.<> (leftarg = pg_lsn, rightarg = pg_lsn);

create operator pg_catalog.<> (leftarg = tid, rightarg = tid);

create operator pg_catalog.<> (leftarg = bigint, rightarg = integer);

create operator pg_catalog.<> (leftarg = bit varying, rightarg = bit varying);

create operator pg_catalog.<> (leftarg = "char", rightarg = "char");

create operator pg_catalog.<> (leftarg = bytea, rightarg = bytea);

create operator pg_catalog.<> (leftarg = integer, rightarg = smallint);

create operator pg_catalog.<> (leftarg = xid, rightarg = xid);

create operator pg_catalog.<> (leftarg = real, rightarg = real);

create operator pg_catalog.<> (leftarg = double precision, rightarg = real);

create operator pg_catalog.<> (leftarg = anyenum, rightarg = anyenum);

create operator pg_catalog.<> (leftarg = date, rightarg = date);

create operator pg_catalog.<> (leftarg = bit, rightarg = bit);

create operator pg_catalog.<> (leftarg = jsonb, rightarg = jsonb);

create operator pg_catalog.<> (leftarg = time without time zone, rightarg = time without time zone);

create operator pg_catalog.<> (leftarg = xid, rightarg = integer);

create operator pg_catalog.<> (leftarg = timestamp with time zone, rightarg = timestamp without time zone);

create operator pg_catalog.<> (leftarg = inet, rightarg = inet);

create operator pg_catalog.<> (leftarg = xid8, rightarg = xid8);

create operator pg_catalog.<> (leftarg = money, rightarg = money);

create operator pg_catalog.<> (leftarg = oidvector, rightarg = oidvector);

create operator pg_catalog.<> (leftarg = timestamp with time zone, rightarg = timestamp with time zone);

create operator pg_catalog.<> (leftarg = timestamp without time zone, rightarg = timestamp with time zone);

create operator pg_catalog.<> (leftarg = macaddr8, rightarg = macaddr8);

create operator pg_catalog.<@ (leftarg = anyelement, rightarg = anymultirange);

create operator pg_catalog.<@ (leftarg = lseg, rightarg = box);

create operator pg_catalog.<@ (leftarg = point, rightarg = path);

create operator pg_catalog.<@ (leftarg = anymultirange, rightarg = anyrange);

create operator pg_catalog.<@ (leftarg = point, rightarg = box);

create operator pg_catalog.<@ (leftarg = box, rightarg = box);

create operator pg_catalog.<@ (leftarg = point, rightarg = circle);

create operator pg_catalog.<@ (leftarg = anyarray, rightarg = anyarray);

create operator pg_catalog.<@ (leftarg = tsquery, rightarg = tsquery);

create operator pg_catalog.<@ (leftarg = anyrange, rightarg = anymultirange);

create operator pg_catalog.<@ (leftarg = anymultirange, rightarg = anymultirange);

create operator pg_catalog.<@ (leftarg = point, rightarg = lseg);

create operator pg_catalog.<@ (leftarg = lseg, rightarg = line);

create operator pg_catalog.<@ (leftarg = jsonb, rightarg = jsonb);

create operator pg_catalog.<@ (leftarg = point, rightarg = polygon);

create operator pg_catalog.<@ (leftarg = anyelement, rightarg = anyrange);

create operator pg_catalog.<@ (leftarg = circle, rightarg = circle);

create operator pg_catalog.<@ (leftarg = anyrange, rightarg = anyrange);

create operator pg_catalog.<@ (leftarg = polygon, rightarg = polygon);

create operator pg_catalog.<@ (leftarg = point, rightarg = line);

create operator pg_catalog.<^ (leftarg = point, rightarg = point);

create operator pg_catalog.<^ (leftarg = box, rightarg = box);

create operator pg_catalog.= (leftarg = anymultirange, rightarg = anymultirange);

create operator pg_catalog.= (leftarg = time with time zone, rightarg = time with time zone);

create operator pg_catalog.= (leftarg = date, rightarg = date);

create operator pg_catalog.= (leftarg = real, rightarg = double precision);

create operator pg_catalog.= (leftarg = anyarray, rightarg = anyarray);

create operator pg_catalog.= (leftarg = double precision, rightarg = real);

create operator pg_catalog.= (leftarg = character, rightarg = character);

create operator pg_catalog.= (leftarg = aclitem, rightarg = aclitem);

create operator pg_catalog.= (leftarg = jsonb, rightarg = jsonb);

create operator pg_catalog.= (leftarg = money, rightarg = money);

create operator pg_catalog.= (leftarg = timestamp with time zone, rightarg = timestamp with time zone);

create operator pg_catalog.= (leftarg = interval, rightarg = interval);

create operator pg_catalog.= (leftarg = path, rightarg = path);

create operator pg_catalog.= (leftarg = circle, rightarg = circle);

create operator pg_catalog.= (leftarg = double precision, rightarg = double precision);

create operator pg_catalog.= (leftarg = pg_lsn, rightarg = pg_lsn);

create operator pg_catalog.= (leftarg = lseg, rightarg = lseg);

create operator pg_catalog.= (leftarg = real, rightarg = real);

create operator pg_catalog.= (leftarg = oidvector, rightarg = oidvector);

create operator pg_catalog.= (leftarg = oid, rightarg = oid);

create operator pg_catalog.= (leftarg = line, rightarg = line);

create operator pg_catalog.= (leftarg = macaddr, rightarg = macaddr);

create operator pg_catalog.= (leftarg = macaddr8, rightarg = macaddr8);

create operator pg_catalog.= (leftarg = inet, rightarg = inet);

create operator pg_catalog.= (leftarg = integer, rightarg = smallint);

create operator pg_catalog.= (leftarg = smallint, rightarg = integer);

create operator pg_catalog.= (leftarg = anyrange, rightarg = anyrange);

create operator pg_catalog.= (leftarg = numeric, rightarg = numeric);

create operator pg_catalog.= (leftarg = box, rightarg = box);

create operator pg_catalog.= (leftarg = bit, rightarg = bit);

create operator pg_catalog.= (leftarg = bigint, rightarg = integer);

create operator pg_catalog.= (leftarg = bit varying, rightarg = bit varying);

create operator pg_catalog.= (leftarg = smallint, rightarg = bigint);

create operator pg_catalog.= (leftarg = bigint, rightarg = smallint);

create operator pg_catalog.= (leftarg = bigint, rightarg = bigint);

create operator pg_catalog.= (leftarg = tid, rightarg = tid);

create operator pg_catalog.= (leftarg = record, rightarg = record);

create operator pg_catalog.= (leftarg = cid, rightarg = cid);

create operator pg_catalog.= (leftarg = boolean, rightarg = boolean);

create operator pg_catalog.= (leftarg = xid8, rightarg = xid8);

create operator pg_catalog.= (leftarg = xid, rightarg = integer);

create operator pg_catalog.= (leftarg = bytea, rightarg = bytea);

create operator pg_catalog.= (leftarg = xid, rightarg = xid);

create operator pg_catalog.= (leftarg = timestamp without time zone, rightarg = timestamp without time zone);

create operator pg_catalog.= (leftarg = tsquery, rightarg = tsquery);

create operator pg_catalog.= (leftarg = text, rightarg = name);

create operator pg_catalog.= (leftarg = date, rightarg = timestamp without time zone);

create operator pg_catalog.= (leftarg = date, rightarg = timestamp with time zone);

create operator pg_catalog.= (leftarg = timestamp without time zone, rightarg = date);

create operator pg_catalog.= (leftarg = timestamp with time zone, rightarg = date);

create operator pg_catalog.= (leftarg = name, rightarg = text);

create operator pg_catalog.= (leftarg = timestamp without time zone, rightarg = timestamp with time zone);

create operator pg_catalog.= (leftarg = integer, rightarg = bigint);

create operator pg_catalog.= (leftarg = timestamp with time zone, rightarg = timestamp without time zone);

create operator pg_catalog.= (leftarg = text, rightarg = text);

create operator pg_catalog.= (leftarg = integer, rightarg = integer);

create operator pg_catalog.= (leftarg = smallint, rightarg = smallint);

create operator pg_catalog.= (leftarg = name, rightarg = name);

create operator pg_catalog.= (leftarg = "char", rightarg = "char");

create operator pg_catalog.= (leftarg = tsvector, rightarg = tsvector);

create operator pg_catalog.= (leftarg = anyenum, rightarg = anyenum);

create operator pg_catalog.= (leftarg = uuid, rightarg = uuid);

create operator pg_catalog.= (leftarg = time without time zone, rightarg = time without time zone);

create operator pg_catalog.> (leftarg = macaddr8, rightarg = macaddr8);

create operator pg_catalog.> (leftarg = record, rightarg = record);

create operator pg_catalog.> (leftarg = anyrange, rightarg = anyrange);

create operator pg_catalog.> (leftarg = tid, rightarg = tid);

create operator pg_catalog.> (leftarg = boolean, rightarg = boolean);

create operator pg_catalog.> (leftarg = macaddr, rightarg = macaddr);

create operator pg_catalog.> (leftarg = anyarray, rightarg = anyarray);

create operator pg_catalog.> (leftarg = time with time zone, rightarg = time with time zone);

create operator pg_catalog.> (leftarg = lseg, rightarg = lseg);

create operator pg_catalog.> (leftarg = oid, rightarg = oid);

create operator pg_catalog.> (leftarg = date, rightarg = date);

create operator pg_catalog.> (leftarg = xid8, rightarg = xid8);

create operator pg_catalog.> (leftarg = oidvector, rightarg = oidvector);

create operator pg_catalog.> (leftarg = real, rightarg = real);

create operator pg_catalog.> (leftarg = jsonb, rightarg = jsonb);

create operator pg_catalog.> (leftarg = "char", rightarg = "char");

create operator pg_catalog.> (leftarg = bytea, rightarg = bytea);

create operator pg_catalog.> (leftarg = uuid, rightarg = uuid);

create operator pg_catalog.> (leftarg = pg_lsn, rightarg = pg_lsn);

create operator pg_catalog.> (leftarg = name, rightarg = name);

create operator pg_catalog.> (leftarg = timestamp without time zone, rightarg = timestamp without time zone);

create operator pg_catalog.> (leftarg = text, rightarg = name);

create operator pg_catalog.> (leftarg = text, rightarg = text);

create operator pg_catalog.> (leftarg = tsquery, rightarg = tsquery);

create operator pg_catalog.> (leftarg = double precision, rightarg = real);

create operator pg_catalog.> (leftarg = tsvector, rightarg = tsvector);

create operator pg_catalog.> (leftarg = circle, rightarg = circle);

create operator pg_catalog.> (leftarg = double precision, rightarg = double precision);

create operator pg_catalog.> (leftarg = character, rightarg = character);

create operator pg_catalog.> (leftarg = date, rightarg = timestamp without time zone);

create operator pg_catalog.> (leftarg = name, rightarg = text);

create operator pg_catalog.> (leftarg = anymultirange, rightarg = anymultirange);

create operator pg_catalog.> (leftarg = date, rightarg = timestamp with time zone);

create operator pg_catalog.> (leftarg = path, rightarg = path);

create operator pg_catalog.> (leftarg = timestamp without time zone, rightarg = date);

create operator pg_catalog.> (leftarg = interval, rightarg = interval);

create operator pg_catalog.> (leftarg = timestamp with time zone, rightarg = date);

create operator pg_catalog.> (leftarg = timestamp with time zone, rightarg = timestamp with time zone);

create operator pg_catalog.> (leftarg = timestamp without time zone, rightarg = timestamp with time zone);

create operator pg_catalog.> (leftarg = time without time zone, rightarg = time without time zone);

create operator pg_catalog.> (leftarg = real, rightarg = double precision);

create operator pg_catalog.> (leftarg = box, rightarg = box);

create operator pg_catalog.> (leftarg = bit, rightarg = bit);

create operator pg_catalog.> (leftarg = numeric, rightarg = numeric);

create operator pg_catalog.> (leftarg = integer, rightarg = bigint);

create operator pg_catalog.> (leftarg = smallint, rightarg = smallint);

create operator pg_catalog.> (leftarg = integer, rightarg = integer);

create operator pg_catalog.> (leftarg = bigint, rightarg = integer);

create operator pg_catalog.> (leftarg = money, rightarg = money);

create operator pg_catalog.> (leftarg = anyenum, rightarg = anyenum);

create operator pg_catalog.> (leftarg = bit varying, rightarg = bit varying);

create operator pg_catalog.> (leftarg = inet, rightarg = inet);

create operator pg_catalog.> (leftarg = bigint, rightarg = bigint);

create operator pg_catalog.> (leftarg = smallint, rightarg = integer);

create operator pg_catalog.> (leftarg = integer, rightarg = smallint);

create operator pg_catalog.> (leftarg = smallint, rightarg = bigint);

create operator pg_catalog.> (leftarg = timestamp with time zone, rightarg = timestamp without time zone);

create operator pg_catalog.> (leftarg = bigint, rightarg = smallint);

create operator pg_catalog.>= (leftarg = inet, rightarg = inet);

create operator pg_catalog.>= (leftarg = bigint, rightarg = smallint);

create operator pg_catalog.>= (leftarg = tid, rightarg = tid);

create operator pg_catalog.>= (leftarg = money, rightarg = money);

create operator pg_catalog.>= (leftarg = character, rightarg = character);

create operator pg_catalog.>= (leftarg = date, rightarg = timestamp with time zone);

create operator pg_catalog.>= (leftarg = macaddr, rightarg = macaddr);

create operator pg_catalog.>= (leftarg = bigint, rightarg = integer);

create operator pg_catalog.>= (leftarg = anymultirange, rightarg = anymultirange);

create operator pg_catalog.>= (leftarg = smallint, rightarg = integer);

create operator pg_catalog.>= (leftarg = name, rightarg = text);

create operator pg_catalog.>= (leftarg = record, rightarg = record);

create operator pg_catalog.>= (leftarg = integer, rightarg = smallint);

create operator pg_catalog.>= (leftarg = lseg, rightarg = lseg);

create operator pg_catalog.>= (leftarg = date, rightarg = date);

create operator pg_catalog.>= (leftarg = time with time zone, rightarg = time with time zone);

create operator pg_catalog.>= (leftarg = xid8, rightarg = xid8);

create operator pg_catalog.>= (leftarg = timestamp without time zone, rightarg = date);

create operator pg_catalog.>= (leftarg = oid, rightarg = oid);

create operator pg_catalog.>= (leftarg = macaddr8, rightarg = macaddr8);

create operator pg_catalog.>= (leftarg = interval, rightarg = interval);

create operator pg_catalog.>= (leftarg = bit varying, rightarg = bit varying);

create operator pg_catalog.>= (leftarg = anyarray, rightarg = anyarray);

create operator pg_catalog.>= (leftarg = oidvector, rightarg = oidvector);

create operator pg_catalog.>= (leftarg = path, rightarg = path);

create operator pg_catalog.>= (leftarg = jsonb, rightarg = jsonb);

create operator pg_catalog.>= (leftarg = boolean, rightarg = boolean);

create operator pg_catalog.>= (leftarg = real, rightarg = real);

create operator pg_catalog.>= (leftarg = anyenum, rightarg = anyenum);

create operator pg_catalog.>= (leftarg = timestamp with time zone, rightarg = date);

create operator pg_catalog.>= (leftarg = smallint, rightarg = smallint);

create operator pg_catalog.>= (leftarg = bytea, rightarg = bytea);

create operator pg_catalog.>= (leftarg = "char", rightarg = "char");

create operator pg_catalog.>= (leftarg = uuid, rightarg = uuid);

create operator pg_catalog.>= (leftarg = timestamp with time zone, rightarg = timestamp with time zone);

create operator pg_catalog.>= (leftarg = time without time zone, rightarg = time without time zone);

create operator pg_catalog.>= (leftarg = pg_lsn, rightarg = pg_lsn);

create operator pg_catalog.>= (leftarg = anyrange, rightarg = anyrange);

create operator pg_catalog.>= (leftarg = name, rightarg = name);

create operator pg_catalog.>= (leftarg = real, rightarg = double precision);

create operator pg_catalog.>= (leftarg = timestamp without time zone, rightarg = timestamp without time zone);

create operator pg_catalog.>= (leftarg = timestamp without time zone, rightarg = timestamp with time zone);

create operator pg_catalog.>= (leftarg = text, rightarg = name);

create operator pg_catalog.>= (leftarg = integer, rightarg = integer);

create operator pg_catalog.>= (leftarg = timestamp with time zone, rightarg = timestamp without time zone);

create operator pg_catalog.>= (leftarg = tsquery, rightarg = tsquery);

create operator pg_catalog.>= (leftarg = text, rightarg = text);

create operator pg_catalog.>= (leftarg = double precision, rightarg = real);

create operator pg_catalog.>= (leftarg = circle, rightarg = circle);

create operator pg_catalog.>= (leftarg = bigint, rightarg = bigint);

create operator pg_catalog.>= (leftarg = box, rightarg = box);

create operator pg_catalog.>= (leftarg = tsvector, rightarg = tsvector);

create operator pg_catalog.>= (leftarg = smallint, rightarg = bigint);

create operator pg_catalog.>= (leftarg = integer, rightarg = bigint);

create operator pg_catalog.>= (leftarg = numeric, rightarg = numeric);

create operator pg_catalog.>= (leftarg = date, rightarg = timestamp without time zone);

create operator pg_catalog.>= (leftarg = bit, rightarg = bit);

create operator pg_catalog.>= (leftarg = double precision, rightarg = double precision);

create operator pg_catalog.>> (leftarg = anymultirange, rightarg = anymultirange);

create operator pg_catalog.>> (leftarg = polygon, rightarg = polygon);

create operator pg_catalog.>> (leftarg = box, rightarg = box);

create operator pg_catalog.>> (leftarg = point, rightarg = point);

create operator pg_catalog.>> (leftarg = circle, rightarg = circle);

create operator pg_catalog.>> (leftarg = inet, rightarg = inet);

create operator pg_catalog.>> (leftarg = bit, rightarg = integer);

create operator pg_catalog.>> (leftarg = smallint, rightarg = integer);

create operator pg_catalog.>> (leftarg = integer, rightarg = integer);

create operator pg_catalog.>> (leftarg = bigint, rightarg = integer);

create operator pg_catalog.>> (leftarg = anyrange, rightarg = anyrange);

create operator pg_catalog.>> (leftarg = anyrange, rightarg = anymultirange);

create operator pg_catalog.>> (leftarg = anymultirange, rightarg = anyrange);

create operator pg_catalog.>>= (leftarg = inet, rightarg = inet);

create operator pg_catalog.>^ (leftarg = box, rightarg = box);

create operator pg_catalog.>^ (leftarg = point, rightarg = point);

create operator pg_catalog.? (leftarg = jsonb, rightarg = text);

create operator pg_catalog.?# (leftarg = lseg, rightarg = box);

create operator pg_catalog.?# (leftarg = line, rightarg = line);

create operator pg_catalog.?# (leftarg = box, rightarg = box);

create operator pg_catalog.?# (leftarg = path, rightarg = path);

create operator pg_catalog.?# (leftarg = lseg, rightarg = lseg);

create operator pg_catalog.?# (leftarg = lseg, rightarg = line);

create operator pg_catalog.?# (leftarg = line, rightarg = box);

create operator pg_catalog.?& (leftarg = jsonb, rightarg = text[]);

create operator pg_catalog.?- (rightarg = lseg);

create operator pg_catalog.?- (leftarg = point, rightarg = point);

create operator pg_catalog.?- (rightarg = line);

create operator pg_catalog.?-| (leftarg = line, rightarg = line);

create operator pg_catalog.?-| (leftarg = lseg, rightarg = lseg);

create operator pg_catalog.?| (rightarg = line);

create operator pg_catalog.?| (leftarg = point, rightarg = point);

create operator pg_catalog.?| (leftarg = jsonb, rightarg = text[]);

create operator pg_catalog.?| (rightarg = lseg);

create operator pg_catalog.?|| (leftarg = line, rightarg = line);

create operator pg_catalog.?|| (leftarg = lseg, rightarg = lseg);

create operator pg_catalog.@ (rightarg = integer);

create operator pg_catalog.@ (rightarg = numeric);

create operator pg_catalog.@ (rightarg = smallint);

create operator pg_catalog.@ (rightarg = double precision);

create operator pg_catalog.@ (rightarg = real);

create operator pg_catalog.@ (rightarg = bigint);

create operator pg_catalog.@-@ (rightarg = path);

create operator pg_catalog.@-@ (rightarg = lseg);

create operator pg_catalog.@> (leftarg = polygon, rightarg = point);

create operator pg_catalog.@> (leftarg = circle, rightarg = point);

create operator pg_catalog.@> (leftarg = tsquery, rightarg = tsquery);

create operator pg_catalog.@> (leftarg = path, rightarg = point);

create operator pg_catalog.@> (leftarg = anymultirange, rightarg = anyelement);

create operator pg_catalog.@> (leftarg = box, rightarg = box);

create operator pg_catalog.@> (leftarg = box, rightarg = point);

create operator pg_catalog.@> (leftarg = jsonb, rightarg = jsonb);

create operator pg_catalog.@> (leftarg = circle, rightarg = circle);

create operator pg_catalog.@> (leftarg = polygon, rightarg = polygon);

create operator pg_catalog.@> (leftarg = anyarray, rightarg = anyarray);

create operator pg_catalog.@> (leftarg = anyrange, rightarg = anymultirange);

create operator pg_catalog.@> (leftarg = aclitem[], rightarg = aclitem);

create operator pg_catalog.@> (leftarg = anymultirange, rightarg = anymultirange);

create operator pg_catalog.@> (leftarg = anyrange, rightarg = anyelement);

create operator pg_catalog.@> (leftarg = anyrange, rightarg = anyrange);

create operator pg_catalog.@> (leftarg = anymultirange, rightarg = anyrange);

create operator pg_catalog.@? (leftarg = jsonb, rightarg = jsonpath);

create operator pg_catalog.@@ (leftarg = tsvector, rightarg = tsquery);

create operator pg_catalog.@@ (leftarg = tsquery, rightarg = tsvector);

create operator pg_catalog.@@ (rightarg = circle);

create operator pg_catalog.@@ (rightarg = polygon);

create operator pg_catalog.@@ (leftarg = text, rightarg = text);

create operator pg_catalog.@@ (rightarg = box);

create operator pg_catalog.@@ (leftarg = text, rightarg = tsquery);

create operator pg_catalog.@@ (leftarg = jsonb, rightarg = jsonpath);

create operator pg_catalog.@@ (rightarg = lseg);

create operator pg_catalog.@@@ (leftarg = tsvector, rightarg = tsquery);

create operator pg_catalog.@@@ (leftarg = tsquery, rightarg = tsvector);

create operator pg_catalog.^ (leftarg = double precision, rightarg = double precision);

create operator pg_catalog.^ (leftarg = numeric, rightarg = numeric);

create operator pg_catalog.^@ (leftarg = text, rightarg = text);

create operator pg_catalog.| (leftarg = smallint, rightarg = smallint);

create operator pg_catalog.| (leftarg = bit, rightarg = bit);

create operator pg_catalog.| (leftarg = inet, rightarg = inet);

create operator pg_catalog.| (leftarg = bigint, rightarg = bigint);

create operator pg_catalog.| (leftarg = macaddr8, rightarg = macaddr8);

create operator pg_catalog.| (leftarg = macaddr, rightarg = macaddr);

create operator pg_catalog.| (leftarg = integer, rightarg = integer);

create operator pg_catalog.|&> (leftarg = circle, rightarg = circle);

create operator pg_catalog.|&> (leftarg = box, rightarg = box);

create operator pg_catalog.|&> (leftarg = polygon, rightarg = polygon);

create operator pg_catalog.|/ (rightarg = double precision);

create operator pg_catalog.|>> (leftarg = box, rightarg = box);

create operator pg_catalog.|>> (leftarg = polygon, rightarg = polygon);

create operator pg_catalog.|>> (leftarg = point, rightarg = point);

create operator pg_catalog.|>> (leftarg = circle, rightarg = circle);

create operator pg_catalog.|| (leftarg = text, rightarg = text);

create operator pg_catalog.|| (leftarg = bit varying, rightarg = bit varying);

create operator pg_catalog.|| (leftarg = tsvector, rightarg = tsvector);

create operator pg_catalog.|| (leftarg = jsonb, rightarg = jsonb);

create operator pg_catalog.|| (leftarg = anycompatiblearray, rightarg = anycompatiblearray);

create operator pg_catalog.|| (leftarg = anycompatible, rightarg = anycompatiblearray);

create operator pg_catalog.|| (leftarg = text, rightarg = anynonarray);

create operator pg_catalog.|| (leftarg = anynonarray, rightarg = text);

create operator pg_catalog.|| (leftarg = bytea, rightarg = bytea);

create operator pg_catalog.|| (leftarg = tsquery, rightarg = tsquery);

create operator pg_catalog.|| (leftarg = anycompatiblearray, rightarg = anycompatible);

create operator pg_catalog.||/ (rightarg = double precision);

create operator pg_catalog.~ (leftarg = character, rightarg = text);

create operator pg_catalog.~ (rightarg = macaddr8);

create operator pg_catalog.~ (leftarg = name, rightarg = text);

create operator pg_catalog.~ (rightarg = macaddr);

create operator pg_catalog.~ (rightarg = inet);

create operator pg_catalog.~ (leftarg = text, rightarg = text);

create operator pg_catalog.~ (rightarg = bigint);

create operator pg_catalog.~ (rightarg = bit);

create operator pg_catalog.~ (rightarg = smallint);

create operator pg_catalog.~ (rightarg = integer);

create operator pg_catalog.~* (leftarg = name, rightarg = text);

create operator pg_catalog.~* (leftarg = text, rightarg = text);

create operator pg_catalog.~* (leftarg = character, rightarg = text);

create operator pg_catalog.~<=~ (leftarg = character, rightarg = character);

create operator pg_catalog.~<=~ (leftarg = text, rightarg = text);

create operator pg_catalog.~<~ (leftarg = text, rightarg = text);

create operator pg_catalog.~<~ (leftarg = character, rightarg = character);

create operator pg_catalog.~= (leftarg = point, rightarg = point);

create operator pg_catalog.~= (leftarg = circle, rightarg = circle);

create operator pg_catalog.~= (leftarg = box, rightarg = box);

create operator pg_catalog.~= (leftarg = polygon, rightarg = polygon);

create operator pg_catalog.~>=~ (leftarg = text, rightarg = text);

create operator pg_catalog.~>=~ (leftarg = character, rightarg = character);

create operator pg_catalog.~>~ (leftarg = character, rightarg = character);

create operator pg_catalog.~>~ (leftarg = text, rightarg = text);

create operator pg_catalog.~~ (leftarg = bytea, rightarg = bytea);

create operator pg_catalog.~~ (leftarg = name, rightarg = text);

create operator pg_catalog.~~ (leftarg = text, rightarg = text);

create operator pg_catalog.~~ (leftarg = character, rightarg = text);

create operator pg_catalog.~~* (leftarg = character, rightarg = text);

create operator pg_catalog.~~* (leftarg = text, rightarg = text);

create operator pg_catalog.~~* (leftarg = name, rightarg = text);

