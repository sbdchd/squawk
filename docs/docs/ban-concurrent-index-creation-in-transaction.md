---
id: ban-concurrent-index-creation-in-transaction
title: ban-concurrent-index-creation-in-transaction
---

## problem

While regular index creation can happen inside a transaction, this is not allowed when the `CONCURRENTLY` option is used.

https://www.postgresql.org/docs/current/sql-createindex.html#SQL-CREATEINDEX-CONCURRENTLY

## solution

Remove surrounding transaction markers if any.
For migrations that are implicitly wrapped in a transaction, ensure that the `CREATE INDEX` command is the only command in the migration to allow migration tool to detect that no transaction is needed.

Instead of:

```sql
BEGIN;
-- <any other commands being run transactionally>
CREATE INDEX CONCURRENTLY "email_idx" ON "app_user" ("email");
COMMIT;
```

Use:

```sql
BEGIN;
-- <any other commands being run transactionally>
COMMIT;

CREATE INDEX CONCURRENTLY "email_idx" ON "app_user" ("email");
```

If you use a migration tool, it may be configured to automatically wrap commands in transactions; if that's the case, check if it supports running commands in a non-transactional context.
For example, with `alembic`:

```python
# migrations/*.py
from alembic import op

def schema_upgrades():
    # <any other commands being run transactionally>

    # alembic allows non-transactional operations using autocommit
    with op.get_context().autocommit_block():
        op.create_index(
            "email_idx",
            "app_user",
            ["email"],
            schema="app",
            unique=False,
            postgresql_concurrently=True,
        )
    
    # <any other commands being run transactionally>

def schema_downgrades():
    # <any other downgrade commands>

    op.drop_index(
        "email_idx",
        schema="app",
    )

    # <any other downgrade commands>
```

Or alternatively:

```python
# migrations/*.py
from alembic import op

def schema_upgrades():
    # <any other commands being run transactionally>

    # you can also execute BEGIN/COMMIT to delineate transactions
    op.execute("COMMIT;")
    op.execute("SET statement_timeout = 0;")
    op.create_index(
        "email_idx",
        "app_user",
        ["email"],
        schema="app",
        unique=False,
        postgresql_concurrently=True,
    )

    op.execute("BEGIN;")
    # <any other commands being run transactionally>

def schema_downgrades():
    # <any other downgrade commands>

    op.drop_index(
        "email_idx",
        schema="app",
    )

    # <any other downgrade commands>
```

`golang-migrate` wraps migrations in transactions but is clever enough to perform the migration if the migration file does not contain further commands next to the `CREATE INDEX` command.

## links

https://www.postgresql.org/docs/current/sql-createindex.html#SQL-CREATEINDEX-CONCURRENTLY