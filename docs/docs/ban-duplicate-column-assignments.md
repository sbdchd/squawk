---
id: ban-duplicate-column-assignments
title: ban-duplicate-column-assignments
---

## problem

Assigning/declaring a column more than once results in a runtime error in
Postgres.

```sql
create table t(a int, a text);
create view v (a, a) as select 1, 2;
update t set a = 1, a = 2;
```

## solution

Remove duplicate assignments/declarations:

```sql
create table t(a int);
create view v (a, b) as select 1, 2;
update t set a = 2;
```
