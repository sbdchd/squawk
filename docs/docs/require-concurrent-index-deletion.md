---
id: require-concurrent-index-deletion
title: require-concurrent-index-deletion
---

Ensure all index deletions use the `CONCURRENTLY` option.

A normal `DROP INDEX` acquires an `ACCESS EXCLUSIVE` lock on the table, blocking other accesses until the index drop can be completed. 
`CONCURRENTLY` waits until conflicting transactions have completed.

<https://www.postgresql.org/docs/10/sql-dropindex.html>

## solutions

### drop index

Instead of:

```sql
DROP INDEX "email_idx" ON "app_user" ("email");
```

Use:

```sql
DROP INDEX CONCURRENTLY "email_idx" ON "app_user" ("email");
```
