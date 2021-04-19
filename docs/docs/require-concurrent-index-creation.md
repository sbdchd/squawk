---
id: require-concurrent-index-creation
title: require-concurrent-index-creation
---

Ensure all index creations use the `CONCURRENTLY` option.

This rule ignores indexes added to tables created in the same transaction.

During a normal index creation updates are blocked. `CONCURRENTLY` avoids the
issue of blocking.

<https://www.postgresql.org/docs/current/sql-createindex.html#SQL-CREATEINDEX-CONCURRENTLY>

## solutions

### create index

Instead of:

```sql
CREATE INDEX "email_idx" ON "app_user" ("email");
```

Use:

```sql
CREATE INDEX CONCURRENTLY "email_idx" ON "app_user" ("email");
```

### drop index

Instead of:

```sql
DROP INDEX "email_idx" ON "app_user" ("email");
```

Use:

```sql
DROP INDEX CONCURRENTLY "email_idx" ON "app_user" ("email");
```
