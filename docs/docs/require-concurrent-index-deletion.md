---
id: require-concurrent-index-deletion
title: require-concurrent-index-deletion
---

## problem

A normal `DROP INDEX` acquires an `ACCESS EXCLUSIVE` lock on the table, blocking other accesses until the index drop can be completed. 

## solution

Ensure all index deletions use the `CONCURRENTLY` option. `CONCURRENTLY` waits until conflicting transactions have completed.

<https://www.postgresql.org/docs/10/sql-dropindex.html>

### drop index

Instead of:

```sql
DROP INDEX "email_idx" ON "app"."user" ("email");
```

Use:

```sql
DROP INDEX CONCURRENTLY "app"."email_idx";
```
