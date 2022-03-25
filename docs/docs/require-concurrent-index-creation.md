---
id: require-concurrent-index-creation
title: require-concurrent-index-creation
---

## problem

During a normal index creation, table updates are blocked. `CONCURRENTLY` avoids the
issue of locking.

<https://www.postgresql.org/docs/current/sql-createindex.html#SQL-CREATEINDEX-CONCURRENTLY>

## solution

Ensure all index creations use the `CONCURRENTLY` option.

This rule ignores indexes added to tables created in the same transaction.

### create index

Instead of:

```sql
CREATE INDEX "email_idx" ON "app_user" ("email");
```

Use:

```sql
CREATE INDEX CONCURRENTLY "email_idx" ON "app_user" ("email");
```