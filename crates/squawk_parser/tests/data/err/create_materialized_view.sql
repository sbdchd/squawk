-- WITHOUT OVERLAPS is only valid w/ PRIMARY KEY or UNIQUE constraint
create materialized view mv (a without overlaps) as select 1;
