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

Instead of:

```sql
ALTER TABLE "app_user" ADD COLUMN "email" integer NULL;
CREATE INDEX CONCURRENTLY "email_idx" ON "app_user" ("email");
```

Use:

```sql
ALTER TABLE "app_user" ADD COLUMN IF NOT EXISTS "email" integer NULL;
CREATE INDEX CONCURRENTLY IF NOT EXISTS "email_idx" ON "app_user" ("email");
```
