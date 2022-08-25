---
id: adding-field-with-default
title: adding-field-with-default
---

## problem

Adding a field with a default can cause table rewrites, which will take an [`ACCESS EXCLUSIVE`](https://www.postgresql.org/docs/10/sql-altertable.html#SQL-ALTERTABLE-NOTES) lock on the table, blocking reads / writes while the statement is running.

In Postgres version 11 and later, adding a field with a non-`VOLATILE` `DEFAULT` will not require a table rewrite. Adding a field with a [`VOLATILE` `DEFAULT` will cause a table rewrite](https://www.postgresql.org/docs/14/sql-altertable.html#SQL-ALTERTABLE-NOTES).

## solutions

### adding a non-volatile default in Postgres 11+

:::note
This statement is only safe when your default is non-volatile.
:::

```sql
-- blocks reads and writes while schema is changed (fast)
ALTER TABLE "account" ADD COLUMN "foo" integer DEFAULT 10;
```

### adding a volatile default

Add the field without a default, set the default, and then backfill existing rows in batches.

Instead of:

```sql
-- blocks reads and writes while table is rewritten (slow)
ALTER TABLE "account" ADD COLUMN "ab_group" integer DEFAULT random();
```

Use:

```sql
-- blocks reads and writes while schema is changed (fast)
ALTER TABLE "account" ADD COLUMN "ab_group" integer;
-- blocks reads and writes while schema is changed (fast)
ALTER TABLE "account" ALTER COLUMN "ab_group" SET DEFAULT random();

-- backfill existing rows in batches to set the "ab_group" column
```

See ["How not valid constraints work"](constraint-missing-not-valid.md#how-not-valid-validate-works) for more information on adding constraints as `NOT VALID`.
