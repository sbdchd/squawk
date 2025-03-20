---
id: adding-not-nullable-field
title: adding-not-nullable-field
---

Use a check constraint instead of setting a column as `NOT NULL`.

## problem

Adding a column as `NOT NULL` is no longer covered by this rule. See ["adding-required-field (set a non-volatile default)"](adding-required-field.md#set-a-non-volatile-default) for more information on how to add new columns with `NOT NULL`.


Modifying a column to be `NOT NULL` may fail if the column contains records with a `NULL` value, requiring a full table scan to check before executing. Old application code may also try to write `NULL` values to the table.

`ALTER TABLE` also requires an `ACCESS EXCLUSIVE` lock which will disable reads and writes while this statement is running.

## solutions

### setting an existing column as non-nullable

Instead of:

```sql
ALTER TABLE "recipe" ALTER COLUMN "view_count" SET NOT NULL;
```

Use:

```sql
ALTER TABLE "recipe" ADD CONSTRAINT view_count_not_null
    CHECK ("view_count" IS NOT NULL) NOT VALID;

-- Backfill column so it's not null
-- Update ...

ALTER TABLE "recipe" VALIDATE CONSTRAINT view_count_not_null;

-- If pg version >= 12, set not null without doing a table scan
ALTER TABLE table_name ALTER COLUMN column_name SET NOT NULL;
```
For each step, note that:
1. Adding the constraint acquires an `ACCESS EXCLUSIVE` lock, but this is done extremely quickly.
2. You MUST backfill the column before validating the constraint
3. Validating the constraint does require a full table scan, but only acquires a [`SHARE UPDATE EXCLUSIVE`](https://www.postgresql.org/docs/current/sql-altertable.html#SQL-ALTERTABLE-DESC-VALIDATE-CONSTRAINT) lock which allows for normal operations to continue.
4. If using postgres version 12 or later, it forgoes the full table scan by checking the existing constraint.

See ["How not valid constraints work"](constraint-missing-not-valid.md#how-not-valid-validate-works) for more information on adding constraints as `NOT VALID`.


## solution for alembic and sqlalchemy

Instead of:

```python
# migrations/*.py
from alembic import op
import sqlalchemy as sa

def schema_upgrades():
    op.alter_column(
        "recipe",
        "view_count",
        existing_type=sa.BigInteger(),
        nullable=False,
    )

def schema_downgrades():
    op.alter_column(
        "recipe",
        "view_count",
        existing_type=sa.BigInteger(),
        nullable=True,
    )
```

```python
# migrations/*.py
from alembic import op
import sqlalchemy as sa

def schema_upgrades():
    op.create_check_constraint(
        constraint_name="view_count_not_null",
        table_name="recipe",
        condition="view_count IS NOT NULL",
        postgresql_not_valid=True,
    )
    # Backfill existing rows to get rid of any NULL values. You have have to split
    # this into two migrations
    op.execute(
        sa.text("ALTER TABLE recipe VALIDATE CONSTRAINT view_count_not_null"),
    )
    op.alter_column( # only include this step on pg version >= 12
        "recipe",
        "view_count",
        existing_type=sa.BigInteger(),
        nullable=False,
    )


def schema_downgrades():
    op.alter_column( # only include this step on pg version >= 12
        "recipe",
        "view_count",
        existing_type=sa.BigInteger(),
        nullable=True,
    )
    op.drop_constraint(
        constraint_name="view_count_not_null",
        table_name="recipe",
    )
```
