---
id: require-concurrent-index-creation
title: require-concurrent-index-creation
---

## problem

During a normal index creation, table updates are blocked, but reads are still allowed. `CONCURRENTLY` avoids locking the table against writes during index creation.

<https://www.postgresql.org/docs/current/sql-createindex.html#SQL-CREATEINDEX-CONCURRENTLY>

## solution

Ensure all index creations use the `CONCURRENTLY` option.

This rule ignores indexes added to tables created in the same transaction.

### create index

Instead of:

```sql
-- blocks writes to table while index is built
CREATE INDEX "email_idx" ON "app_user" ("email");
```

Use:

```sql
-- allows reads and writes while index is built
CREATE INDEX CONCURRENTLY "email_idx" ON "app_user" ("email");
```


## solution for alembic and sqlalchemy

Instead of:

```python
# migrations/*.py
from alembic import op

def schema_upgrades():
    op.create_index(
        "email_idx",
        "app_user",
        ["email"],
        schema="app",
        unique=False,
    )

def schema_downgrades():
    op.drop_index(
        "email_idx",
        schema="app",
    )
```

Use:

```python
# migrations/*.py
from alembic import op

def schema_upgrades():
    with op.get_context().autocommit_block():
        op.create_index(
            op.f("email_idx"),
            "app_user",
            ["email"],
            unique=False,
            postgresql_concurrently=True,
        )

def schema_downgrades():
    with op.get_context().autocommit_block():
        op.drop_index(
            "email_idx",
            table_name="app_user",
            postgresql_concurrently=True,
        )
```

**Notes:**

In a concurrent index build, the index is actually entered as an “invalid” 
([details](https://www.postgresql.org/docs/current/sql-createindex.html#:~:text=In%20a%20concurrent%20index%20build,modified%20the%20table%20to%20terminate.))
