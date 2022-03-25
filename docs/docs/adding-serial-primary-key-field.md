---
id: adding-serial-primary-key-field
title: adding-serial-primary-key-field
---

## problem

Adding a primary key constraint requires an `ACCESS EXCLUSIVE` lock that will block all reads and writes to the table while the primary key index is built.

## solution

Instead of creating the constraint directly, create the
`CONSTRAINT` `USING` an index.

The index will be created in the background and an `ACCESS EXCLUSIVE` lock will only be acquired when updating the table metadata with the `ADD CONSTRAINT ... USING` statement. See ["disallowed-unique-constraint"](./disallowed-unique-constraint.md) for more examples.

Instead of:

```sql
ALTER TABLE items ADD PRIMARY KEY (id);
```

Use:

```sql
CREATE UNIQUE INDEX CONCURRENTLY items_pk_idx ON items (id);
ALTER TABLE items ADD CONSTRAINT items_pk PRIMARY KEY USING INDEX items_pk;
```

## further reading

[Citus' 2018 post on tips for Postgres
locking](https://www.citusdata.com/blog/2018/02/22/seven-tips-for-dealing-with-postgres-locks/) and [the Postgres "ALTER TABLE" docs](https://www.postgresql.org/docs/current/sql-altertable.html). 