---
id: prefer-robust-stmts
title: prefer-robust-stmts
---

The goal of this rule is to make migrations more robust when they fail part way through.

To appease this rule you can use guards like `IF NOT EXISTS` or wrap all your
statements in a transaction.

## examples

You may have a migration with two steps:

1. add new column named `billing_email` to the `account` table.
2. create index `CONCURRENTLY` on `billing_email`.

Since step 2 is concurrent, we cannot run this migration in a transaction.

Without a transaction, step 1 can succeed while step 2 fails. This would leave
us with the new `billing_email` column, but without the index from step 2.

When we rerun our migration, the migration will fail at step 1, since the field
`billing_email` field already exists.

To make this change robust, we can follow the ["add column"](#add-column) and
["add index"](#add-index) examples shown below and use `IF NOT EXISTS` when
adding our column and index.

With this change, we can run our migration multiple times without erroring.

### add table

```sql
-- instead of:
CREATE TABLE "foo_tbl" (
    "id" serial NOT NULL PRIMARY KEY,
    "modified" timestamp with time zone NOT NULL,
    "created" timestamp with time zone NOT NULL
);

-- use:
CREATE TABLE IF NOT EXISTS "foo_tbl" (
    "id" serial NOT NULL PRIMARY KEY,
    "modified" timestamp with time zone NOT NULL,
    "created" timestamp with time zone NOT NULL
);
```

### add column

```sql
-- instead of:
ALTER TABLE "app_user" ADD COLUMN "email" integer NULL;

-- use:
ALTER TABLE "app_user" ADD COLUMN "email" IF NOT EXISTS integer NULL;
```

### add constraint

```sql
-- instead of:
ALTER TABLE "accounts" ADD CONSTRAINT "positive_balance" CHECK ("balance" >= 0);

-- use:
ALTER TABLE "accounts" DROP CONSTRAINT IF EXISTS "positive_balance";
ALTER TABLE "accounts" ADD CONSTRAINT "positive_balance" CHECK ("balance" >= 0);
```

### validate constraint

```sql
-- instead of:
ALTER TABLE "accounts" VALIDATE CONSTRAINT "positive_balance";

-- use:
ALTER TABLE "accounts" VALIDATE CONSTRAINT "positive_balance";
```

### add index

```sql
-- instead of:
CREATE INDEX CONCURRENTLY "email_idx" ON "app_user" ("email");

-- use:
CREATE INDEX CONCURRENTLY IF NOT EXISTS "email_idx" ON "app_user" ("email");
```

### remove table

```sql
-- instead of:
DROP TABLE "foo_tbl";

-- use:
DROP TABLE IF EXISTS "foo_tbl";
```

### remove column

```sql
-- instead of:
ALTER TABLE DROP "col_name";

-- use:
DROP TABLE IF EXISTS "foo_tbl";
```

### remove constraint

```sql
-- instead of:
ALTER TABLE "foo_tbl" DROP CONSTRAINT "foo_constraint";

-- use:
ALTER TABLE "foo_tbl" DROP CONSTRAINT IF EXISTS "foo_constraint";
```

### remove index

```sql
-- instead of:
DROP INDEX "foo_idx";

-- use:
DROP INDEX "foo_idx" IF EXISTS;
```
