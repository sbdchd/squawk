---
id: require-timeout-settings
title: require-timeout-settings
---

:::note Deprecated

This rule has been split into [`require-lock-timeout`](./require-lock-timeout.md) and [`require-statement-timeout`](./require-statement-timeout.md), which can be enabled and disabled individually.

`require-timeout-settings` still works as an alias for both rules in configuration files and `squawk-ignore` comments.
:::

## problem

You must configure a `lock_timeout` to safely apply migrations. See ["Safety requirements"](./safe_migrations.md#safety-requirements)

Additionally, a statement_timeout also helps prevent long migrations that consume too many database resources.

## solution

Configure both timeout settings at the beginning of your migration file:

```sql
-- error, missing timeouts
alter table t add column c boolean;
```

```sql
-- ok, timeouts configured before ddl operations
set lock_timeout = '1s';
set statement_timeout = '5s';
alter table t add column c boolean;
```

### alternatively

If you're database connection is already configured with lock and statement
timeouts, you can safely ignore this rule.

## links

- [PostgreSQL: SET](https://www.postgresql.org/docs/current/sql-set.html)
- [PostgreSQL: lock_timeout](https://www.postgresql.org/docs/current/runtime-config-client.html#GUC-LOCK-TIMEOUT)
- [PostgreSQL: statement_timeout](https://www.postgresql.org/docs/current/runtime-config-client.html#GUC-STATEMENT-TIMEOUT)
