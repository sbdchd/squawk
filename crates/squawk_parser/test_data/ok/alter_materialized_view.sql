-- rename_view
alter materialized view v rename to n;
alter materialized view if exists s.v rename to n;

-- rename_column
alter materialized view v
  rename c to n;
alter materialized view if exists s.v 
  rename column c to n;

-- set_schema
alter materialized view v set schema s;
alter materialized view if exists v set schema s;

-- depends_extension
alter materialized view v depends on extension e;
alter materialized view v no depends on extension e;

-- set_tablespace
alter materialized view all in tablespace t
  set tablespace n;
alter materialized view all in tablespace t
  owned by current_user, u
  set tablespace n nowait;

-- action_alter_col_stats
alter materialized view v 
  alter c set statistics 100;

alter materialized view v
  alter column c set statistics -1;

alter materialized view v
  alter column c set statistics default;

-- action_alter_col_set
alter materialized view v
  alter column c set (n_distinct = 1.0);

alter materialized view v
  alter c set (a = true, b = 1);

-- action_alter_col_reset
alter materialized view v alter c reset (n_distinct);
alter materialized view v alter column c reset (n_distinct, n_distinct_inherited);

-- action_alter_col_storage
alter materialized view v
  alter c set storage plain;

alter materialized view v
  alter c set storage external;

alter materialized view v
  alter c set storage extended;

alter materialized view v
  alter c set storage main;

alter materialized view s.v 
  alter c set storage default;

-- action_alter_col_compression
alter materialized view v
  alter c set compression pglz;

alter materialized view v
  alter column c set compression pglz;

-- action_cluster
alter materialized view v
  cluster on i;

-- action_set_without_cluster
alter materialized view v
  set without cluster;

