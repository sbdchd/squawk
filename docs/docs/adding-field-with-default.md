---
id: adding-field-with-default
title: adding-field-with-default
---

:::note

This lint only applies to Postgres versions less than 11.
:::

## problem

Adding a field with a DEFAULT requires a table rewrite with an ACCESS EXCLUSIVE lock, which blocks reads / writes while the statement is running.

## solution

Add the field as nullable, then set a default, backfill, and remove nullabilty.

```sql
-- instead of
ALTER TABLE "core_recipe" ADD COLUMN "foo" integer DEFAULT 10;
-- use
ALTER TABLE "core_recipe" ADD COLUMN "foo" integer;
ALTER TABLE "core_recipe" ALTER COLUMN "foo" SET DEFAULT 10;
-- backfill
-- remove nullability
```
