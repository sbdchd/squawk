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
create table app_user (
  id serial not null primary key,
  email varchar(100) not null
);
```

Use:

```sql
create table app_user (
  id serial not null primary key,
  email text not null
);
alter table app_user add constraint text_size check (length(email) <= 100);
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
