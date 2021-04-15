---
id: prefer-robust-stmts
title: prefer-robust-stmts
---

The goal of this rule is to make migrations more robust when they fail part way through.

For instance, you may have a migration with two steps. First, the migration
adds a field to a table, then it creates an index concurrently.

Since this second part is concurrent, it can't run in a transaction so the
first part of the migration can succeed, and second part can fail meaning the
first part won't be rolled back.

Then when the migration is run again, it will fail at adding the field since
it already exists.

To appease this rule you can use guards like `IF NOT EXISTS` or wrap all your
statements in a transaction.

## examples

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
ALTER TABLE "app_user" ADD "email_constraint";

-- use:
ALTER TABLE "app_user" DROP CONSTRAINT IF EXISTS "email_constraint";
ALTER TABLE "app_user" ADD "email_constraint";
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
