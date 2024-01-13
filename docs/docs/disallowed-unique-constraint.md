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


Instead of:

```sql
-- blocks reads and writes to table_name while constraint is validated.
ALTER TABLE distributors
    ADD COLUMN dist_id text CONSTRAINT dist_id_uniq UNIQUE (dist_id);
```

Use:

```sql
-- add nullable column separately to prevent blocking read/writes.
ALTER TABLE distributors ADD COLUMN dist_id text;
-- allows reads and writes while index is built
CREATE UNIQUE INDEX CONCURRENTLY dist_id_uniq ON distributors (dist_id);
```

## solution for alembic and sqlalchemy

```python
# migrations/*.py
from alembic import op

def schema_upgrades():
    with op.get_context().autocommit_block():
        op.create_index(
            op.f("dist_id_uniq"),
            "distributors",
            ["dist_id"],
            unique=True,
            postgresql_concurrently=True,
        )

def schema_downgrades():
    with op.get_context().autocommit_block():
        op.drop_index(
            op.f("dist_id_uniq"),
            postgresql_concurrently=True,
        )
```
