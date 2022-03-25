---
id: postgres-locks
title: Postgres locks and further reading
---

The Postgres [docs on "Explicit Locking"](https://www.postgresql.org/docs/current/explicit-locking.html) provide excellent information on Postgres lock levels in and their interactions.

Braintree's ["PostgreSQL at Scale: Database Schema Changes Without Downtime"](https://medium.com/paypal-tech/postgresql-at-scale-database-schema-changes-without-downtime-20d3749ed680) has practical examples of different schema changes, the locks required for them and how to safely make those changes. Their earlier ["Safe Operations For High Volume PostgreSQL"](https://www.braintreepayments.com/blog/safe-operations-for-high-volume-postgresql/) provides more concise examples of schema changes and their workarounds to make them safe.

Citus's ["When Postgres blocks: 7 tips for dealing with locks"](https://www.citusdata.com/blog/2018/02/22/seven-tips-for-dealing-with-postgres-locks/) has some examples of schema changes and how to make them safely.

GoCardless's ["Zero-downtime Postgres migrations - the hard parts"](https://gocardless.com/blog/zero-downtime-postgres-migrations-the-hard-parts/) and Benchling's ["Move fast and migrate things: how we automated migrations in Postgres"](https://benchling.engineering/move-fast-and-migrate-things-how-we-automated-migrations-in-postgres-d60aba0fc3d4) documents how to set lock timeouts to safely apply migrations.

The Postgres [docs on "Alter Table"](https://www.postgresql.org/docs/10/sql-altertable.html#SQL-ALTERTABLE-NOTES) have notes on the database behavior of table modifications. This is an excellent resource for understanding how certain schema changes will be applied at the database. 


Please see ["Applying migrations safely"](./safe_migrations.md) for information about setting the necessary lock timeouts to safely apply migrations in a production environment.