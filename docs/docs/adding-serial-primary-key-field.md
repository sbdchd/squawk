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
-- blocks reads and writes to "account" table while constraint is added.
ALTER TABLE account ADD PRIMARY KEY (id);
```

Use:

```sql
-- allows reads and writes while index is built
CREATE UNIQUE INDEX CONCURRENTLY account_pk_idx ON account (id);
-- blocks reads and writes while table schema is updated (fast)
ALTER TABLE account ADD CONSTRAINT account_pk PRIMARY KEY USING INDEX account_pk_idx;
```

## further reading

[Citus' 2018 post on tips for Postgres
locking](https://www.citusdata.com/blog/2018/02/22/seven-tips-for-dealing-with-postgres-locks/) and [the Postgres "ALTER TABLE" docs](https://www.postgresql.org/docs/current/sql-altertable.html). 
