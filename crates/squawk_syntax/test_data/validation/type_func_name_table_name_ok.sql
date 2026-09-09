create table "left" (a int);
create table s.left (a int);
create foreign table s.left (a int) server s;
select 1 into s.left;
alter table t merge partitions (a, b) into s.left;
