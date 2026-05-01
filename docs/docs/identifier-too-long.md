---
id: identifier-too-long
title: identifier-too-long
---

## problem

Postgres will truncate identifiers longer than 63 bytes and return a notice.

This can result in suprising behavior as the names you provide aren't the names that Postgres uses.

```sql
-- table name is 65 bytes and will be silently truncated to 63
create table table_very_long_very_long_very_long_very_long_very_long_very_long (
  -- column name is 66 bytes and will also be silently truncated
  column_very_long_very_long_very_long_very_long_very_long_very_long bigint
);
```

```
NOTICE:  identifier "table_very_long_very_long_very_long_very_long_very_long_very_long" will be truncated to "table_very_long_very_long_very_long_very_long_very_long_very_lo"
NOTICE:  identifier "column_very_long_very_long_very_long_very_long_very_long_very_long" will be truncated to "column_very_long_very_long_very_long_very_long_very_long_very_l"

Query 1 OK: CREATE TABLE
```

## solution

Shorten the identifier!

```sql
create table a_shorter_table_name (
  a_shorter_column_name bigint
);
```

## links

- [Identifiers and Key Words](https://www.postgresql.org/docs/current/sql-syntax-lexical.html#SQL-SYNTAX-IDENTIFIERS)
