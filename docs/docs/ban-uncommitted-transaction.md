---
id: ban-uncommitted-transaction
title: ban-uncommitted-transaction
---

## problem

If you start a transaction and forget to add a `commit` statement, then the migration will run successfully, but it [won't have actually applied the changes](https://github.com/sbdchd/squawk/issues/697#issue-3570671498).

```sql
begin;
create table users (id bigint);
-- missing `commit`!
```

## solution

Add a `commit` statement to complete your transaction!

```sql
begin;
create table users (id bigint);
commit;
```
