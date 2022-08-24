---
id: safe_migrations
title: Applying migrations safely
---

A migration that passes Squawk's lint is not automatically safe to run.

To safely apply a migration you must set a `lock_timeout` in Postgres. See below for more information.

## safety requirements

1. Use Squawk to lint your migrations. Follow the advice.
2. Set a short `lock_timeout` (e.g. 2 seconds) within Postgres when running your migrations. If you hit the `lock_timeout`, great, retry your migration until it succeeds.

### `lock_timeout`

You must configure a `lock_timeout` within Postgres while running your migrations. Without this your migration could block other writes while trying to grab a lock on the table.

You can set this timeout in a few ways:

- globally in Postgres (see below)
- for a specific database role
- via the options connection parameter of your client
- with the `PGOPTIONS` environment variable if your client supports it. Python's `pyscogpg2` supports this option. (see below)

```sql
-- set the global Postgres lock timeout to 1 second
SET lock_timeout = '1s';
```

```bash
PGOPTIONS='-c lock_timeout=10s' ./migrate_db
```

With a short `lock_timeout` of 1 second, queries will be blocked for up to 1 second. If your migration hits the lock timeout, it will be cancelled and error, allowing the waiting queries to proceed. You should retry a migration that hits the lock timeout until it succeeds.

### example

This statement requires an `AccessExclusiveLock` lock on `"accounts"`. Reads and writes to `"accounts"` will be blocked while this statement is running.

```sql
ALTER TABLE "accounts" ADD CONSTRAINT "positive_balance" CHECK ("balance" >= 0) NOT VALID
```

If there is a long running query or open transaction, this `ALTER TABLE` statement could be blocked while it waits for an `AccessExclusiveLock` lock. While the statement waits for a lock, all other reads and writes sent after will wait for this statement too, bringing your application to a halt.

With a short `lock_timeout` of 1 second, queries will be blocked for up to 1 second. If your migration hits the lock timeout, it will be cancelled and error, allowing the waiting queries to proceed. You should retry a migration that hits the lock timeout until it succeeds.


## further reading

Benchling's ["Move fast and migrate things: how we automated migrations in Postgres"](https://benchling.engineering/move-fast-and-migrate-things-how-we-automated-migrations-in-postgres-d60aba0fc3d4) and GoCardless's ["Zero-downtime Postgres migrations - the hard parts"](https://gocardless.com/blog/zero-downtime-postgres-migrations-the-hard-parts/) provide more background on `lock_timeout` and `statement_timeout` in a production environment.

## experiementing with locks

Create some example

```sql
-- create table
create table "account" (
    id bigint generated always as identity primary key,
    created_at timestamptz not null default now()
);

-- open a transaction
begin;

-- run your migration
alter table app.users

-- check locks
select
    locktype,
    relation::regclass,
    mode,
    transactionid as tid,
    virtualtransaction as vtid,
    pid,
    granted
from pg_locks;

```

https://travisofthenorth.com/blog/2017/2/2/postgres-adding-foreign-keys-with-zero-downtime
