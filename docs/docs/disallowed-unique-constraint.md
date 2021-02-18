---
id: disallowed-unique-constraint
title: disallowed-unique-constraint
---

Adding a `UNIQUE` constraint requires an `ACCESS EXCLUSIVE` lock which blocks reads.

Instead create an index `CONCURRENTLY` and create the `CONSTRAINT` `USING` the index.

<https://www.postgresql.org/docs/current/sql-altertable.html>

Instead of:

```sql
ALTER TABLE table_name ADD CONSTRAINT field_name_constraint UNIQUE (field_name);
```

Use:

```sql
CREATE UNIQUE INDEX CONCURRENTLY dist_id_temp_idx ON distributors (dist_id);
ALTER TABLE distributors
    DROP CONSTRAINT distributors_pkey,
    ADD CONSTRAINT distributors_pkey PRIMARY KEY USING INDEX dist_id_temp_idx;
```
