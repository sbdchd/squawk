-- missing alter_table
add column foo boolean;

-- mismatch options
alter table t alter constraint c not deferrable initially deferred;

-- pg 18 only, via: https://www.depesz.com/2025/05/01/waiting-for-postgresql-18-allow-not-null-constraints-to-be-added-as-not-valid/
alter table public.copy_2 add constraint id_not_null not null id not valid;
