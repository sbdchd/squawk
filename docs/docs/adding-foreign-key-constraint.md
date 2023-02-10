---
id: adding-foreign-key-constraint
title: adding-foreign-key-constraint
---

## problem

Adding a foreign key constraint requires a table scan and a `SHARE ROW EXCLUSIVE` lock on both tables, which blocks writes to each table.

This means no writes will be allowed to either table while the table you're altering is scanned to validate the constraint.

## solution

To prevent blocking writes to tables, add the constraint as `NOT VALID` in one transaction, then `VALIDATE CONSTRAINT` in another.

While `NOT VALID` prevents row updates while running, it commits instantly if it can get a lock (see ["Safety requirements"](./safe_migrations.md#safety-requirements)). `VALIDATE CONSTRAINT` allows row updates while it scans
the table.

See ["How not valid constraints work"](constraint-missing-not-valid.md#how-not-valid-validate-works) for more information on adding constraints as `NOT VALID`.

### adding constraint to existing table

Instead of:

```sql
-- blocks writes to "email" and "user" while Postgres checks rows in "email" have user_id mapping to "user".id (slow)
ALTER TABLE "email" ADD CONSTRAINT "fk_user"
    FOREIGN KEY ("user_id") REFERENCES "user" ("id");
```

Use:

```sql
-- blocks writes to "email" and "user" while Postgres updates table schema (fast)
ALTER TABLE "email" ADD CONSTRAINT "fk_user"
    FOREIGN KEY ("user_id") REFERENCES "user" ("id") NOT VALID;
-- non-blocking while existing rows are checked.
ALTER TABLE "email" VALIDATE CONSTRAINT "fk_user";
```

Add the foreign key constraint as `NOT VALID` to prevent locking the `"email"` and `"user"` tables while "email" rows are checked against "user".

Run `VALIDATE CONSTRAINT` to scan the `"email"` table in the background while reads and writes continue.

### adding constraint to new table

Both of these examples have the same amount of locking. Since the newly created table has no rows, you don't need to add a foreign key with `NOT VALID`. 

Adding foreign key constraint in `create table` statement.

```sql
-- blocks writes to "user" while Postgres updates table schema.
CREATE TABLE email (
    id BIGINT GENERATED ALWAYS AS IDENTITY,
    user_id BIGINT,
    email TEXT,
    PRIMARY KEY(id),
    CONSTRAINT fk_user
        FOREIGN KEY ("user_id")
        REFERENCES "user" ("id")
);
```

Using `not valid...validate`:

```sql
-- no references to lock.
CREATE TABLE email (
    id BIGINT GENERATED ALWAYS AS IDENTITY,
    user_id BIGINT,
    email TEXT,
    PRIMARY KEY(id)
);

-- blocks writes to "email" and "user" while Postgres updates table schema (fast)
ALTER TABLE "email" ADD CONSTRAINT "fk_user"
    FOREIGN KEY ("user_id") REFERENCES "user" ("id") NOT VALID;
-- non-blocking while existing rows are checked.
ALTER TABLE "email" VALIDATE CONSTRAINT "fk_user";
```


## solution for alembic and sqlalchemy

### adding constraint to existing table ###

Instead of:

```python
# models.py
import sqlalchemy as sa


class Email(BaseModel):
    __tablename__ = "email"
    ...
    user_id = sa.Column(sa.BigInteger, sa.ForeignKey("user.id"))
    ...


class User(BaseModel):
    __tablename__ = "user"

    id = sa.Column(sa.BigInteger, primary_key=True, autoincrement=True)
    ...
```

```python
# migrations/*.py
import sqlalchemy as sa
from alembic import op


def schema_upgrades():
    op.add_column("email", sa.Column("user_id", sa.BigInteger(), nullable=True))
    op.create_foreign_key(None, "email", "user", ["user_id"], ["id"])


def schema_downgrades():
    op.drop_constraint(None, "email", type_="foreignkey")
    op.drop_column("email", "user_id")
```

Use:

```python
# models.py
import sqlalchemy as sa


class Email(BaseModel):
    __tablename__ = "email"
    ...
    user_id = sa.Column(sa.BigInteger, sa.ForeignKey("user.id", name="fk_user"))
    ...


class User(BaseModel):
    __tablename__ = "user"

    id = sa.Column(sa.BigInteger, primary_key=True, autoincrement=True)
    ...
```

```python
# migrations/*.py
# First migration
import sqlalchemy as sa
from alembic import op


def schema_upgrades():
    op.add_column("email", sa.Column("user_id", sa.BigInteger(), nullable=True))
    op.create_foreign_key(
        "fk_user", 
        "email", 
        "user", 
        ["user_id"], 
        ["id"], 
        postgresql_not_valid=True
    )


def schema_downgrades():
    op.drop_constraint(
        "fk_user", 
        "email", 
        type_="foreignkey"
    )
    op.drop_column("email", "user_id")
```

```python
# migrations/*.py
# Second migration
import sqlalchemy as sa
from alembic import op


def schema_upgrades():
    op.execute(
        sa.text(
            """ALTER TABLE email VALIDATE CONSTRAINT fk_user"""
        ),
    )
```


### adding constraint to new table

Instead of:

```python
# models.py
import sqlalchemy as sa


class User(BaseModel):
    __tablename__ = "user"
    ...


class Email(BaseModel):
    __tablename__ = "email"

    id = sa.Column(sa.BigInteger, primary_key=True, autoincrement=True)

    user_id = sa.Column(sa.BigInteger, sa.ForeignKey("user.id"))
```

```python
# migrations/*.py
import sqlalchemy as sa
from alembic import op


def schema_upgrades():
    op.create_table("email",
    sa.Column("id", sa.BigInteger(), autoincrement=True, nullable=False),
    sa.Column("user_id", sa.BigInteger(), nullable=True),
    sa.ForeignKeyConstraint(["user_id"], ["user.id"], ),
    sa.PrimaryKeyConstraint("id")
    )


def schema_downgrades():
    op.drop_table("new_table")
```

Use:

```python
# models.py
import sqlalchemy as sa

class User(BaseModel):
    __tablename__ = "user"
    ...


class Email(BaseModel):
    __tablename__ = "email"

    id = sa.Column(sa.BigInteger, primary_key=True, autoincrement=True)

    user_id = sa.Column(sa.BigInteger, sa.ForeignKey("user.id", name="fk_user"))
```

```python
# migrations/*.py
# First migration
import sqlalchemy as sa
from alembic import op


def schema_upgrades():
    op.create_table("email",
    sa.Column("id", sa.BigInteger(), autoincrement=True, nullable=False),
    sa.Column("user_id", sa.BigInteger(), nullable=True),
    sa.PrimaryKeyConstraint("id")
    )
    op.create_foreign_key(
        "fk_user",
        "email",
        "user",
        ["user_id"],
        ["id"],
        postgresql_not_valid=True
    )


def schema_downgrades():
    op.drop_table("new_table")
```

```python
# migrations/*.py
# Second migration
import sqlalchemy as sa
from alembic import op


def schema_upgrades():
    op.execute(
        sa.text(
            """ALTER TABLE email VALIDATE CONSTRAINT fk_user"""
        ),
    )

```

## links

- https://travisofthenorth.com/blog/2017/2/2/postgres-adding-foreign-keys-with-zero-downtime