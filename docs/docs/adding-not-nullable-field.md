---
id: adding-not-nullable-field
title: adding-not-nullable-field
---

Use a check constraint instead of setting a column as `NOT NULL`.

:::note Postgres Version

In Postgres versions 11 of later, adding a non-null column with a default will complete without a table scan.
:::

## problem

Adding a column as `NOT NULL` requires a table scan and the `ALTER TABLE` requires
an `ACCESS EXCLUSIVE` lock. Reads and writes will be disabled while this statement is running.

## solutions

### adding a non-nullable column

Add a column as nullable and use a check constraint to verify integrity. The check constraint should be added as `NOT NULL` and then validated.

Instead of:

```sql
ALTER TABLE "core_recipe" ADD COLUMN "foo" integer DEFAULT 10 NOT NULL;
```

Use:

```sql
ALTER TABLE "core_recipe" ADD COLUMN "foo" integer DEFAULT 10;
ALTER TABLE "core_recipe" ADD CONSTRAINT foo_not_null
    CHECK ("foo" IS NOT NULL) NOT VALID;
-- backfill column so it's not null
ALTER TABLE "core_recipe" VALIDATE CONSTRAINT foo_not_null;
```

Add the column as nullable, add a check constraint as `NOT VALID` to verify new rows and updates are, backfill the column so it no longer contains null values, validate the constraint to verify existing rows are valid.

See ["How not valid constraints work"](constraint-missing-not-valid.md#how-not-valid-validate-works) for more information on adding constraints as `NOT VALID`.

### setting an existing column as non-nullable

Instead of:

```sql
ALTER TABLE "core_recipe" ALTER COLUMN "foo" SET NOT NULL;
```

Use:

```sql
ALTER TABLE "core_recipe" ADD CONSTRAINT foo_not_null
    CHECK ("foo" IS NOT NULL) NOT VALID;
-- backfill column so it's not null
ALTER TABLE "core_recipe" VALIDATE CONSTRAINT foo_not_null;
```

Add a check constraint as `NOT VALID` to verify new rows and updates are, backfill the column so it no longer contains null values, validate the constraint to verify existing rows are valid.

See ["How not valid constraints work"](constraint-missing-not-valid.md#how-not-valid-validate-works) for more information on adding constraints as `NOT VALID`.
