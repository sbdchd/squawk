---
id: require-concurrent-partition-detach
title: require-concurrent-partition-detach
---

## problem

Detaching a partition via `ALTER TABLE t DETACH PARTITION y;` requires an `ACCESS EXCLUSIVE` lock, which prevents reads and writes to the table.

## solution

Ensure all partition detaches use the `CONCURRENTLY` option.

Instead of:

```sql
-- blocks reads and writes
ALTER TABLE t DETACH PARTITION p;
```

Use:

```sql
-- allows reads and writes while detaching
ALTER TABLE t DETACH PARTITION p CONCURRENTLY;
```

## links

- <https://www.postgresql.org/docs/current/ddl-partitioning.html>
- <https://www.postgresql.org/docs/current/sql-altertable.html#SQL-ALTERTABLE-DETACH-PARTITION>
