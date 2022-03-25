---
id: postgres-locks
title: Postgres locks and further reading
---

The Postgres [docs on "Explicit Locking"](https://www.postgresql.org/docs/current/explicit-locking.html) provide excellent information on Postgres lock levels in and their interactions.

Braintree's ["PostgreSQL at Scale: Database Schema Changes Without Downtime"](https://medium.com/paypal-tech/postgresql-at-scale-database-schema-changes-without-downtime-20d3749ed680) has practical examples of different schema changes, the locks required for them and how to safely make those changes. Their earlier ["Safe Operations For High Volume PostgreSQL"](https://www.braintreepayments.com/blog/safe-operations-for-high-volume-postgresql/) provides more concise examples of schema changes and their workarounds to make them safe.



Please see ["Applying migrations safely"](./safe_migrations.md) for information about setting the necessary lock timeouts to safely apply migrations in a production environment.