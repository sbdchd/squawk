---
id: adding-serial-primary-key-field
title: adding-serial-primary-key-field
---

Outlined in [Citus' 2018 post on tips for Postgres
locking](https://www.citusdata.com/blog/2018/02/22/seven-tips-for-dealing-with-postgres-locks/)
as well as [the Postgres docs](https://www.postgresql.org/docs/current/sql-altertable.html), adding a primary key constraint is a blocking
operation.

Instead of creating the constraint directly, consider creating the
`CONSTRAINT` `USING` an index.

From the Postgres docs:

> To recreate a primary key constraint, without blocking updates while the
> index is rebuilt:

```sql
CREATE UNIQUE INDEX CONCURRENTLY dist_id_temp_idx ON distributors (dist_id);
ALTER TABLE distributors DROP CONSTRAINT distributors_pkey,
    ADD CONSTRAINT distributors_pkey PRIMARY KEY USING INDEX dist_id_temp_idx;
```

<https://www.postgresql.org/docs/current/sql-altertable.html>
