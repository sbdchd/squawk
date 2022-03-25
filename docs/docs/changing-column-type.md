---
id: changing-column-type
title: changing-column-type
---

## problem

Changing a column type requires an `ACCESS EXCLUSIVE` lock on the table which blocks reads and writes while the table is rewritten.

Changing the type of the column may also break other clients reading from the
table.

<https://www.postgresql.org/docs/current/sql-altertable.html#SQL-ALTERTABLE-NOTES>


## solution

Some "binary coercible" types can be converted without a table rewrite.

`VARCHAR` can safely be converted to `TEXT` and shorter `VARCHAR(5)` can be converted to longer `VARCHAR(10)` because they have the same binary representation on disk.

An `INT` (4 bytes wide) cannot be converted to a `BIGINT` (8 bytes wide) without rewriting the table.


### convert an `INT` column to a `BIGINT` column

Consider a `user_email` table with a column `user_id` that we want to convert from `INT` to `BIGINT`.


Add a new column, `new_user_id`. Dual write with triggers. Backfill.


```sql
ALTER TABLE users ADD COLUMN new_id BIGINT;


BEGIN;
LOCK users;
ALTER TABLE users RENAME COLUMN


```


https://techcommunity.microsoft.com/t5/azure-database-for-postgresql/postgres-tips-how-to-convert-2-billion-rows-to-bigint-with-citus/ba-p/1490128


### convert an `INT` primary key to `BIGINT`

For primary keys and columns with foreign key relations, we must update all related tables pointing to our primary key before making our change.

TODO\