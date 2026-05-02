---
id: prefer-repack
title: prefer-repack
---

## problem

`vacuum full` and `cluster` both require an `ACCESS EXCLUSIVE` lock which blocks reads and writes to the table.

```sql
-- blocks reads and writes
vacuum full t;

-- blocks reads and writes
cluster t using t_pkey;
```

## solution

As of Postgres 19, use the `repack` command, which replaces `vacuum` and `cluster` and adds a `concurrently` option which doesn't require an `ACCESS EXCLUSIVE` lock.

```sql
-- repack without ordering (equivalent to vacuum full)
repack (concurrently) t;

-- repack with ordering (equivalent to cluster)
repack (concurrently) t using index t_pkey;
```

## links

- [PostgreSQL's Docs on `repack`](https://www.postgresql.org/docs/devel/sql-repack.html)
- [PostgreSQL 19: The "repack" command](https://www.dbi-services.com/blog/postgresql-19-the-repack-command/)
- [River Queue's Post on Repack Concurrently](https://riverqueue.com/blog/repack-concurrently)
