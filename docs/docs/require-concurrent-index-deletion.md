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
-- blocks reads and writes to table
DROP INDEX "app"."email_idx";
```

Use:

```sql
-- allows reads and writes to table while Postgres waits for conflicting transactions to finish
DROP INDEX CONCURRENTLY "app"."email_idx";
```
