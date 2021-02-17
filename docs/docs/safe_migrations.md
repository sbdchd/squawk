---
id: safe_migrations
title: Running migrations
---

Squawk checks Postgres migrations for statements that could prevent reads or writes to tables while the migration is running. _Squawk alone is not enough to make a migration safe._

## Safety requirements

1. Use Squawk to lint your migrations. Follow the advice.
2. Set a short `lock_timeout` (e.g. 2 seconds) within Postgres when running your migrations. If you hit the `lock_timeout`, great, retry your migration until it succeeds.

### `lock_timeout`

You must configure a `lock_timeout` within Postgres while running your migrations. Without this your migration could block other writes while trying to grab a lock on the table.

For example, `ALTER TABLE "accounts" ADD CONSTRAINT "positive_balance" CHECK ("balance" >= 0) NOT VALID;` requires an `AccessExclusiveLock` lock on `"accounts"`. Reads and writes to "accounts" will be blocked while this statement is running.

If there is a long running query or open transaction, this `ALTER TABLE` statement could be blocked while it waits for an `AccessExclusiveLock` lock. While this query waits, all other reads and writes sent after this query will wait for this query too, bringing your application to a halt.

Setting a short `lock_timeout` of 1 second will limit this query to blocking access to the table for only 1 second. If your migration hits the lock timeout, it will be cancelled and error. You should retry this migration until it succeeds.
