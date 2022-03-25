---
id: changing-column-type
title: changing-column-type
---

## problem

Changing a column type requires an `ACCESS EXCLUSIVE` lock on the table which blocks reads and writes while the table is written.

Changing the type of the column may also break other clients reading from the
table.

<https://www.postgresql.org/docs/current/sql-altertable.html#SQL-ALTERTABLE-NOTES>


## solution

https://medium.com/paypal-tech/postgresql-at-scale-database-schema-changes-without-downtime-20d3749ed680

VARCHAR can safely be converted to TEXT and shorter `VARCHAR(5)` can be converted to longer `VARCHAR(10)`.

Converting an INT column to BIGINT requires more care.

Add a new column, `new_pk`. Dual write with triggers. Backfill.


```sql
ALTER TABLE users ADD COLUMN new_id BIGINT;


BEGIN;
LOCK users;
ALTER TABLE users RENAME COLUMN


```


https://techcommunity.microsoft.com/t5/azure-database-for-postgresql/postgres-tips-how-to-convert-2-billion-rows-to-bigint-with-citus/ba-p/1490128


TODO