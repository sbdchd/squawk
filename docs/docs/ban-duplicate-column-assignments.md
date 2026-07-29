---
id: ban-duplicate-column-assignments
title: ban-duplicate-column-assignments
---

## problem

Assigning to a column more than once in Postgres results in a runtime error.

```sql
create table t(a int);
update t set a = 1, a = 2;
```

gives:

```
Query 1 ERROR at Line 1: : ERROR:  multiple assignments to same column "a"
```

## solution

Remove your dupe assignment:

```sql
update t set a = 2;
```
