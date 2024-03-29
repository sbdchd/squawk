---
id: prefer-identity
title: prefer-identity
---

## problem

Serial types make permissions and schema management difficult. Identity columns are standard SQL and have more features and better usability.

## solution

Instead of:

```sql
create table app.users
(
    id  bigserial
)
```

Use:

```sql
create table app.users
(
    id  bigint generated by default as identity primary key
)
```

## solution for alembic and sqlalchemy

Built-in support for rendering of `IDENTITY` is not available yet, 
however the following compilation hook may be used to replace occurrences of `SERIAL` with `IDENTITY`. See the [SQLAlchemy docs for more information](https://docs.sqlalchemy.org/en/13/dialects/postgresql.html#postgresql-10-identity-columns).

```python
from sqlalchemy.schema import CreateColumn
from sqlalchemy.ext.compiler import compiles

@compiles(CreateColumn, "postgresql")
def use_identity(element, compiler, **kw):
    text = compiler.visit_create_column(element, **kw)
    text = text.replace("SERIAL", "INT GENERATED BY DEFAULT AS IDENTITY")
    return text
```


## links

- https://wiki.postgresql.org/wiki/Don%27t_Do_This#Don.27t_use_serial
- https://www.enterprisedb.com/blog/postgresql-10-identity-columns-explained
