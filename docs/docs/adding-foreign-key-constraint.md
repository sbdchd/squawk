---
id: adding-foreign-key-constraint
title: adding-foreign-key-constraint
---

A foreign key constraint should be added with `NOT VALID`.

Adding a foreign key constraint requires a table scan and a SHARE ROW EXCLUSIVE lock on both tables, which blocks writes.

Adding the constraint as NOT VALID in one transaction and then using
VALIDATE in another transaction will allow writes when adding the
constraint.

## problem

Adding a foreign key constraint requires a table scan and a `SHARE ROW EXCLUSIVE` lock on both tables, which blocks writes to each table.

This means no writes will be allowed to either table while the table you're altering is scanned to validate the constraint.

## solution

To prevent blocking writes to tables, add the constraint as `NOT VALID` in one transaction, then `VALIDATE CONSTRAINT` in another.

While `NOT VALID` prevents row updates while running, it commits instantly if it can get a lock (see ["Safety requirements"](./safe_migrations.md#safety-requirements)). `VALIDATE CONSTRAINT` allows row updates while it scans
the table.

### adding constraint to existing table

Instead of:

```sql
ALTER TABLE "email" ADD CONSTRAINT "fk_user"
    FOREIGN KEY ("user_id") REFERENCES "user" ("id");
```

Use:

```sql
ALTER TABLE "email" ADD CONSTRAINT "fk_user"
    FOREIGN KEY ("user_id") REFERENCES "user" ("id") NOT VALID;
ALTER TABLE "email" VALIDATE CONSTRAINT "fk_user";
```

Add the foreign key constraint as `NOT VALID` to prevent locking the `"email"` and `"user"` tables.

Run `VALIDATE CONSTRAINT` to scan the `"email"` table in the background while reads and writes continue.

### adding constraint to new table

Instead of:

```sql
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

Use:

```sql
CREATE TABLE email (
    id BIGINT GENERATED ALWAYS AS IDENTITY,
    user_id BIGINT,
    email TEXT,
    PRIMARY KEY(id)
);

ALTER TABLE "email" ADD CONSTRAINT "fk_user"
    FOREIGN KEY ("user_id") REFERENCES "user" ("id") NOT VALID;
ALTER TABLE "email" VALIDATE CONSTRAINT "fk_user";
```

Create the table, add the foreign key constraint as `NOT VALID`, then `VALIDATE` the constraint.
