---
id: prefer-robust-stmts
title: prefer-robust-stmts
---

## problem

A non-robust migration that fails after partially applying may fail again when retried.

### example

You may have a migration with two steps:

1. add new column named `billing_email` to the `account` table.
2. create index `CONCURRENTLY` on `billing_email`.

Since step 2 is concurrent, we cannot run this migration in a transaction.

Without a transaction, step 1 can succeed while step 2 fails. This would leave
us with the new `billing_email` column, but without the index from step 2.

When we rerun our migration, the migration will fail at step 1, since the field
`billing_email` field already exists.

To make this change robust, we can follow the ["add column"](#add-column) and
["add index"](#add-index) examples shown below and use `IF NOT EXISTS` when
adding our column and index.

With this change, we can run our migration multiple times without erroring.

## solutions

To appease this rule you can use guards like `IF NOT EXISTS` or wrap all your
statements in a transaction.

### add table

```sql
-- instead of:
create table foo_tbl (
  id serial not null primary key,
  modified timestamp with time zone not null,
  created timestamp with time zone not null
);

-- use:
create table if not exists foo_tbl (
  id serial not null primary key,
  modified timestamp with time zone not null,
  created timestamp with time zone not null
);
```

### add column

```sql
-- instead of:
ALTER TABLE "app_user" ADD COLUMN "email" integer NULL;

-- use:
ALTER TABLE "app_user" ADD COLUMN "email" IF NOT EXISTS integer NULL;
```

### add constraint

```sql
-- instead of:
ALTER TABLE "app_user" ADD "email_constraint";

-- use:
ALTER TABLE "app_user" DROP CONSTRAINT IF EXISTS "email_constraint";
ALTER TABLE "app_user" ADD "email_constraint";
```

### add index

```sql
-- instead of:
create index concurrently email_idx on app_user (email);

-- use:
create index concurrently if not exists email_idx on app_user (email);
```

`CREATE INDEX CONCURRENTLY` failing will leave behind [`INVALID` index objects](https://www.postgresql.org/docs/current/sql-createindex.html#SQL-CREATEINDEX-CONCURRENTLY)
that still occupy their respective names. This means that if a name is not
specified, a failing migration being run multiple times will create multiple
duplicate indexes with consecutive names, all of which are `INVALID`.

If you provide a name and `IF NOT EXISTS`, the second run will falsely succeed
because the index exists, even though it is invalid. This is another footgun,
albeit a checkable (see the `pg_index` catalog) one.

Thus:

```sql
-- instead of:
create index concurrently on app_user (email);

-- use:
create index concurrently if not exists email_idx on app_user (email);
```

### remove table

```sql
-- instead of:
drop table foo_tbl;

-- use:
drop table if exists foo_tbl;
```

### remove column

```sql
-- instead of:
alter table app_user drop column col_name;

-- use:
alter table app_user drop column if exists col_name;
```

### remove constraint

```sql
-- instead of:
alter table foo_tbl drop constraint foo_constraint;

-- use:
alter table foo_tbl drop constraint if exists foo_constraint;
```

### remove index

```sql
-- instead of:
drop index foo_idx;

-- use:
drop index if exists foo_idx;
```
