---
id: require-statement-timeout
title: require-statement-timeout
---

## problem

A `statement_timeout` helps prevent long migrations that consume too many database resources.

## solution

Configure a `statement_timeout` at the beginning of your migration file:

```sql
-- error, missing statement timeout
alter table t add column c boolean;
```

```sql
-- ok, statement timeout configured before ddl operations
set statement_timeout = '5s';
alter table t add column c boolean;
```

### alternatively

If your database connection is already configured with a statement timeout,
you can safely ignore this rule.

See [`require-lock-timeout`](./require-lock-timeout.md) for the related
`lock_timeout` check.

## links

- [PostgreSQL: SET](https://www.postgresql.org/docs/current/sql-set.html)
- [PostgreSQL: statement_timeout](https://www.postgresql.org/docs/current/runtime-config-client.html#GUC-STATEMENT-TIMEOUT)
