---
id: disallowed-unique-constraint
title: disallowed-unique-constraint
---

## problem

Adding a `UNIQUE` constraint requires an `ACCESS EXCLUSIVE` lock which blocks reads and writes to the table while the index is built.


## solution

Instead create an index `CONCURRENTLY` and create the `CONSTRAINT` `USING` the index.

<https://www.postgresql.org/docs/current/sql-altertable.html>

Instead of:

```sql
-- blocks reads and writes to table_name while constraint is validated.
ALTER TABLE distributors ADD CONSTRAINT dist_id_uniq UNIQUE (dist_id);
```

Use:

```sql
-- allows reads and writes while index is built
CREATE UNIQUE INDEX CONCURRENTLY dist_id_uniq ON distributors (dist_id);
```
