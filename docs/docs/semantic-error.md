---
id: semantic-error
title: semantic-error
---

## problem

The syntax is valid, but there was an error identified while checking the SQL.

## examples

undefined column

```sql
with t as (select 1 a)
select a, b from t;
-- error[semantic-error]: column `b` does not exist
```

duplicate table

```sql
create table t(a int);
create table t(b int);
-- error[semantic-error]: relation `t` already exists
```

## solutions

Delete the invalid reference, add the necessary definition, delete the duplicate, etc.

There are many possible errors under `semantic-error`, but they have similar solutions.

:::note
Squawk might be mistaken, if you think that's the case, please [open an issue](https://github.com/sbdchd/squawk/issues/new)!
:::

## related

[`syntax-error`](./syntax-error.md)
