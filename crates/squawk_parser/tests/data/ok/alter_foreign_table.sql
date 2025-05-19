-- rename
alter foreign table t
  rename to u;

-- only_and_asterisk
alter foreign table only t
  rename to u;

alter foreign table t *
  rename to u;

-- rename_column
alter foreign table t
  rename column c to d;

alter foreign table t
  rename c to d;

-- set_schema
alter foreign table f.t
  set schema s;

-- action_add_column
alter foreign table t
  add column c int;

alter foreign table t
  add c int collate "fr_FR" not null check (a > 10);

-- action_drop_column
alter foreign table t
  drop c cascade;

alter foreign table t
  drop column if exists c cascade;

alter foreign table t
  drop column if exists c restrict;

-- action_column_type
alter foreign table t
  alter column c type int;

alter foreign table t
  alter column c set data type int collate "fr_FR";

-- action_set_default
alter foreign table t
  alter column c set default 10 * 10;

alter foreign table t
  alter c set default 10 * 10;

-- action_drop_default
alter foreign table t
  alter column c drop default;

alter foreign table t
  alter c drop default;

-- action_not_null
alter foreign table t
  alter column c drop not null;

alter foreign table t
  alter c set not null;

-- action_set_statistics
alter foreign table t
  alter column c set statistics 1;

alter foreign table t
  alter c set statistics 1;

-- action_set
alter foreign table t
  alter column c set (a, b = 1);

alter foreign table t
  alter c set (a, b = 1);

-- action_reset
alter foreign table t
  alter column c reset (a, b);

alter foreign table t
  alter c reset (a);

-- action_set_storage
alter foreign table t
  alter column c set storage plain;

alter foreign table t
  alter c set storage external;

alter foreign table t
  alter c set storage extended;

alter foreign table t
  alter c set storage main;

alter foreign table t
  alter c set storage default;

-- action_options
alter foreign table t
  alter column c options(b '', add c 'c', set x '', drop x);

alter foreign table t
  alter c options(set x '');

-- action_add_table_constraint
alter foreign table t
  add constraint c check (a > b) not valid;

alter foreign table t
  add constraint c check (a > b);

-- action_validate_constraint
alter foreign table t
  validate constraint c;

-- action_drop_constraint
alter foreign table t
  drop constraint if exists c restrict;

alter foreign table t
  drop constraint c cascade;

-- action_disable_trigger
alter foreign table t
  disable trigger t;

alter foreign table t
  disable trigger all;

alter foreign table t
  disable trigger user;

-- action_enable_replica_trigger
alter foreign table t
  enable replica trigger t;

-- action_enable_always_trigger
alter foreign table t
  enable always trigger t;

-- action_set_without_oids
alter foreign table t
  set without oids;

-- action_inherit
alter foreign table t
  inherit u;

alter foreign table s.t
  inherit s.u;

-- action_no_inherit
alter foreign table t
  no inherit u;

alter foreign table s.t
  no inherit s.u;

-- action_owner_to
alter foreign table t
  owner to u;

alter foreign table t
  owner to current_user;

-- multiple_actions
alter foreign table t
  add column c int,
  drop column d cascade,
  alter column e set not null;

-- action_owner
alter foreign table if exists t
  owner to u;

