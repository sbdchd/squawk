---
id: adding-not-nullable-field
title: adding-not-nullable-field
---

Use a check constraint instead of setting a column as `NOT NULL`.

:::note Postgres Version

In Postgres versions 11 of later, adding a non-null column with a default will complete without a table scan.
:::

## problem

Adding a column as `NOT NULL` requires a table scan and the `ALTER TABLE` requires
an `ACCESS EXCLUSIVE` lock. Reads and writes will be disabled while this statement is running.

## solutions

### adding a non-nullable column

Add a column as nullable and use a check constraint to verify integrity. The check constraint should be added as `NOT NULL` and then validated.

Instead of:

```sql
ALTER TABLE "recipe" ADD COLUMN "view_count" integer DEFAULT 10 NOT NULL;
```

Use:

```sql
ALTER TABLE "recipe" ADD COLUMN "view_count" integer DEFAULT 10;
ALTER TABLE "recipe" ADD CONSTRAINT view_count_not_null
    CHECK ("view_count" IS NOT NULL) NOT VALID;
-- backfill column so it's not null
ALTER TABLE "recipe" VALIDATE CONSTRAINT view_count_not_null;
```

Add the column as nullable, add a check constraint as `NOT VALID` to verify new rows and updates are, backfill the column so it no longer contains null values, validate the constraint to verify existing rows are valid.

See ["How not valid constraints work"](constraint-missing-not-valid.md#how-not-valid-validate-works) for more information on adding constraints as `NOT VALID`.

### setting an existing column as non-nullable

Instead of:

```sql
ALTER TABLE "recipe" ALTER COLUMN "view_count" SET NOT NULL;
```

Use:

```sql
ALTER TABLE "recipe" ADD CONSTRAINT view_count_not_null
    CHECK ("view_count" IS NOT NULL) NOT VALID;
-- backfill column so it's not null
ALTER TABLE "recipe" VALIDATE CONSTRAINT view_count_not_null;
```

Add a check constraint as `NOT VALID` to verify new rows and updates are, backfill the column so it no longer contains null values, validate the constraint to verify existing rows are valid.

See ["How not valid constraints work"](constraint-missing-not-valid.md#how-not-valid-validate-works) for more information on adding constraints as `NOT VALID`.


## solution for alembic and sqlalchemy

Instead of:

```python
# models.py
import sqlalchemy as sa

class Recipe(BaseModel):
    view_count = sa.Column(sa.BigInteger, default=10, nullable=False)
```

Use:

```python
# models.py
import sqlalchemy as sa

class Recipe(BaseModel):
    __table_args__ = (
        sa.CheckConstraint(
            "view_count IS NOT NULL",
            name="view_count_not_null",
        )
    )
    view_count = sa.Column(sa.BigInteger, default=10, nullable=True)
```

Create Alembic migration manually, because Alembic not creates migration for constraints automatically. See the related [GitHub Issue here](https://github.com/sqlalchemy/alembic/issues/508).

```python
# migrations/*.py
from alembic import op
import sqlalchemy as sa

def schema_upgrades():
    op.add_column(
        "recipe",
        sa.Column("view_count", sa.BigInteger(), nullable=True),
    )
    op.create_check_constraint(
        constraint_name="view_count_not_null",
        table_name="recipe",
        condition="view_count IS NOT NULL",
        postgresql_not_valid=True,
    )
    op.execute(
        sa.text("ALTER TABLE recipe VALIDATE CONSTRAINT view_count_not_null"),
    )


def schema_downgrades():
    op.drop_constraint(
        constraint_name="view_count_not_null",
        table_name="recipe",
    )
    op.drop_column("recipe", "view_count")
```
