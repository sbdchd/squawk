---
id: adding-field-with-default
title: adding-field-with-default
---

:::note Postgres Version

This lint only applies to Postgres versions less than 11.
:::

## problem

On Postgres versions less than 11, adding a field with a `DEFAULT` requires a
table rewrite with an `ACCESS EXCLUSIVE` lock.

<https://www.postgresql.org/docs/10/sql-altertable.html#SQL-ALTERTABLE-NOTES>

An `ACCESS EXCLUSIVE` lock blocks reads / writes while the statement is running.

## solution

Add the field as nullable, then set a default, backfill, and remove nullabilty.

Instead of:

```sql
ALTER TABLE "core_recipe" ADD COLUMN "foo" integer DEFAULT 10 NOT NULL;
```

Use:

```sql
ALTER TABLE "core_recipe" ADD COLUMN "foo" integer;
ALTER TABLE "core_recipe" ALTER COLUMN "foo" SET DEFAULT 10;
-- backfill column in batches
ALTER TABLE "core_recipe" ALTER COLUMN "foo" SET NOT NULL;
```

We add our column as nullable, set a default for new rows, backfill our column (ideally done in batches to limit locking), and finally remove nullability.
