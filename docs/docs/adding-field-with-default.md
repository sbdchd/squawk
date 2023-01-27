---
id: adding-field-with-default
title: adding-field-with-default
---

## problem

Adding a field with a default can cause table rewrites, which will take an [`ACCESS EXCLUSIVE`](https://www.postgresql.org/docs/10/sql-altertable.html#SQL-ALTERTABLE-NOTES) lock on the table, blocking reads / writes while the statement is running.

In Postgres version 11 and later, adding a field with a non-`VOLATILE` `DEFAULT` will not require a table rewrite. Adding a field with a [`VOLATILE` `DEFAULT` will cause a table rewrite](https://www.postgresql.org/docs/14/sql-altertable.html#SQL-ALTERTABLE-NOTES).

## solutions

### adding a non-volatile default in Postgres 11+

:::note
This statement is only safe when your default is non-volatile.
:::

```sql
ALTER TABLE "core_recipe" ADD COLUMN "foo" integer DEFAULT 10 NOT NULL;
```

### adding a volatile default

Add the field as nullable, then set a default, backfill, and remove nullabilty.

Instead of:

```sql
ALTER TABLE "core_recipe" ADD COLUMN "foo" integer DEFAULT 10 NOT NULL;
```

Use:

```sql
ALTER TABLE "core_recipe" ADD COLUMN "foo" integer;
ALTER TABLE "core_recipe" ALTER COLUMN "foo" SET DEFAULT 10;
-- backfill column in batches
ALTER TABLE "core_recipe" ALTER COLUMN "foo" SET NOT NULL;
```

We add our column as nullable, set a default for new rows, backfill our column (ideally done in batches to limit locking), and finally remove nullability.

See ["How not valid constraints work"](constraint-missing-not-valid.md#how-not-valid-validate-works) for more information on adding constraints as `NOT VALID`.


## solution for alembic and sqlalchemy

Instead of:

```python
# models.py
import sqlalchemy as sa

class CoreRecipe(BaseModel):
    __tablename__ = "core_recipe"
    ...
    foo = sa.Column(sa.BigInteger, server_default="10", nullable=False)
```

Use:

```python
# models.py
import sqlalchemy as sa

class CoreRecipe(BaseModel):
    __tablename__ = "core_recipe"
    ...
    foo = sa.Column(sa.BigInteger, default=10, nullable=True)
```

```python
# migrations/*.py
import sqlalchemy as sa
from alembic import op

def schema_upgrades():
    op.add_column("core_recipe", sa.Column("foo", sa.BigInteger(), nullable=True))
    op.execute(
        sa.text(
            """
            ALTER TABLE "core_recipe" ALTER COLUMN "foo" SET DEFAULT 10;
            """
        ),
    )
    # if you have the big table use batch update
    op.execute(
        sa.text("""UPDATE core_recipe SET foo = 10"""),
    )
    

def schema_downgrades():
    op.drop_column("core_recipe", "foo")
```
