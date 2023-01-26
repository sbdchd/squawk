---
id: prefer-text-field
title: prefer-text-field
---

## problem

Changing the size of a `varchar` field requires an `ACCESS EXCLUSIVE` lock, that will prevent all reads and writes to the table.

## solution

Use a text field with a `CHECK CONSTRAINT` makes it easier to change the
max length. See the [`constraint-missing-not-valid` rule](./constraint-missing-not-valid.md).

Instead of:

```sql
CREATE TABLE "app_user" (
    "id" serial NOT NULL PRIMARY KEY,
    "email" varchar(100) NOT NULL
);
```

Use:

```sql
CREATE TABLE "app_user" (
    "id" serial NOT NULL PRIMARY KEY,
    "email" TEXT NOT NULL
);
ALTER TABLE "app_user" ADD CONSTRAINT "text_size" CHECK (LENGTH("email") <= 100);
```


## solution for alembic and sqlalchemy

Instead of:

```python
# models.py
import sqlalchemy as sa

class User(BaseModel):
    email = sa.Column(sa.String(length=100))
```

Use:

```python
# models.py
import sqlalchemy as sa

class User(BaseModel):
    __table_args__ = (sa.CheckConstraint('length("email") <= 100'),)
    email = sa.Column(sa.Text)
```
