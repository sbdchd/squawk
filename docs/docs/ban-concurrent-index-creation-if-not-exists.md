---
id: ban-concurrent-index-creation-if-not-exists
title: ban-concurrent-index-creation-if-not-exists
---

## problem

If `CREATE INDEX CONCURRENTLY` fails or is cancelled, for example because of a `statement_timeout`, a deadlock, or a uniqueness violation, Postgres leaves behind an `INVALID` index with the name you gave it.

Postgres doesn't use an invalid index for queries, but may still update it on every write. If the build fails late, an invalid unique index can still enforce its uniqueness constraint.

`IF NOT EXISTS` only checks the name of the index, not whether the index is valid. When the migration is rerun, `CREATE INDEX CONCURRENTLY IF NOT EXISTS` skips over the invalid index and the migration succeeds without a usable index.

This rule is opt-in.

## solution

The [Postgres docs](https://www.postgresql.org/docs/current/sql-createindex.html#SQL-CREATEINDEX-CONCURRENTLY) say:

> The recommended recovery method in such cases is to drop the index and try again to perform `CREATE INDEX CONCURRENTLY`.

Drop the index before creating it, so a rerun of the migration removes an invalid index left behind by an earlier attempt.

Instead of:

```sql
create index concurrently if not exists email_idx on app_user (email);
```

Use:

```sql
drop index concurrently if exists email_idx;
create index concurrently email_idx on app_user (email);
```

If a later statement in the migration fails, the rerun drops and rebuilds an index that was already valid, which costs another full index build.

An index is created in the schema of its table, so use the same schema in the `drop index`:

```sql
drop index concurrently if exists app.email_idx;
create index concurrently email_idx on app.app_user (email);
```

This rule allows `if not exists` when the same migration drops the index first.

If you enabled the opt-in [ban-drop-index](ban-drop-index.md) rule, it reports the inserted `drop index`. Add `-- squawk-ignore ban-drop-index` on the line above it.

Some migration tools can't run the drop and the create from one file:

- tools that wrap a file in a transaction unless it has only one statement. Squawk models this with `--assume-in-transaction`.
- tools that send the whole file to Postgres as one query string. Postgres runs a query string with more than one statement as one implicit transaction, so `DROP INDEX CONCURRENTLY` fails.

A drop in an earlier migration of its own doesn't help, because the tool records that migration as applied and doesn't run it again when you rerun the failed migration. With these tools, use the mode of your tool that runs statements one at a time, if it has one. Otherwise, after a failed `CREATE INDEX CONCURRENTLY`, find the invalid index and drop it by hand before you rerun the migration:

```sql
select indexrelid::regclass from pg_index where not indisvalid;
```

This query also lists indexes that are being built right now, and `_ccnew` / `_ccold` indexes from `REINDEX CONCURRENTLY`, so make sure no index build is running before you drop an index by hand.

Enable this rule with `--include ban-concurrent-index-creation-if-not-exists` (or add `ban-concurrent-index-creation-if-not-exists` to your configured include list).

## links

- <https://www.postgresql.org/docs/current/sql-createindex.html#SQL-CREATEINDEX-CONCURRENTLY>
- <https://www.postgresql.org/docs/current/sql-dropindex.html>
