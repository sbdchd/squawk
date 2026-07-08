---
id: require-lock-timeout
title: require-lock-timeout
---

## problem

You must configure a `lock_timeout` to safely apply migrations. See ["Safety requirements"](./safe_migrations.md#safety-requirements)

## solution

Configure a `lock_timeout` at the beginning of your migration file:

```sql
-- error, missing lock timeout
alter table t add column c boolean;
```

```sql
-- ok, lock timeout configured before ddl operations
set lock_timeout = '1s';
alter table t add column c boolean;
```

### alternatively

If your database connection is already configured with a lock timeout, you
can safely ignore this rule.

See [`require-statement-timeout`](./require-statement-timeout.md) for the
related `statement_timeout` check.

## links

- [PostgreSQL: SET](https://www.postgresql.org/docs/current/sql-set.html)
- [PostgreSQL: lock_timeout](https://www.postgresql.org/docs/current/runtime-config-client.html#GUC-LOCK-TIMEOUT)
