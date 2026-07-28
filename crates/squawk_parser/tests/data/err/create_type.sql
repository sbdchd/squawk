-- WITHOUT OVERLAPS is only valid w/ PRIMARY KEY or UNIQUE constraint
create type ty as (a int without overlaps);
