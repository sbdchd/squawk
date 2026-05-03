---
id: require-concurrent-reindex
title: require-concurrent-reindex
---

## problem

`REINDEX` without `CONCURRENTLY` requires an `ACCESS EXCLUSIVE` lock on the index's table, blocking reads and writes.

## solution

Use `REINDEX ... CONCURRENTLY`.

Instead of:

```sql
-- blocks reads and writes
REINDEX TABLE foo;
REINDEX INDEX foo_idx;
```

Use:

```sql
-- avoids blocking reads and writes
REINDEX TABLE CONCURRENTLY foo;
REINDEX INDEX CONCURRENTLY foo_idx;
```

## links

- <https://www.postgresql.org/docs/current/sql-reindex.html#SQL-REINDEX-CONCURRENTLY>
