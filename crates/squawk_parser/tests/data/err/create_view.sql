-- WITHOUT OVERLAPS is only valid w/ PRIMARY KEY or UNIQUE constraint
create view v (a without overlaps) as select 1;

-- leading comma in column list
create view v (,) as select 1;

-- leading comma with columns
create view v (,a, b) as select 1;
