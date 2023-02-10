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


## solution for alembic and sqlalchemy

Instead of:

```python
# migrations/*.py
from alembic import op

def schema_upgrades():
    op.drop_index(
        "email_idx",
        schema="app",
    )

def schema_downgrades():
    op.create_index(
        "email_idx",
        "app_user",
        ["email"],
        schema="app",
        unique=False,
    )
```

Use:

```python
# migrations/*.py
from alembic import op

def schema_upgrades():
    with op.get_context().autocommit_block():
        op.drop_index(
            "email_idx",
            table_name="app_user",
            postgresql_concurrently=True,
        )

def schema_downgrades():
    with op.get_context().autocommit_block():
        op.create_index(
            op.f("email_idx"),
            "app_user",
            ["email"],
            unique=False,
            postgresql_concurrently=True,
        )
```
