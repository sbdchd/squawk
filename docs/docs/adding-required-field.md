---
id: adding-required-field
title: adding-required-field
---

Make new columns optional when adding them to an existing table.

## problem

Adding a new column that is `NOT NULL` and has no default value to an existing table effectively makes it required. For example:

```
ALTER TABLE "recipe" ADD COLUMN "public" boolean NOT NULL;
```

This will fail immediately upon running for any populated table. Furthermore, old application code that is unaware of this column will fail to `INSERT` to this table.

## solution

### allow null values initially

Make new columns optional by omitting the `NOT NULL` constraint until all existing data and application code has been updated to set appropriate values for the column. Then set it to `NOT NULL`, following the guidance in the [adding-not-nullable-field](./adding-not-nullable-field) docs.

For example:

```
ALTER TABLE "recipe" ADD COLUMN "public" boolean;
```

### set a non-volatile default

If using Postgres version 11 or later, add a `DEFAULT` value that is not volatile, as described in [adding-field-with-default](./adding-field-with-default). This allows the column to keep its `NOT NULL` constraint.

For example:

```
ALTER TABLE "recipe" ADD COLUMN "public" boolean NOT NULL DEFAULT false;
```
