---
id: prefer-bigint-over-smallint
title: prefer-bigint-over-smallint
---


## problem

Using 16 bit integer fields can result in hitting the max int limit.

## solution

Use 64 bit integer field instead, like `BIGSERIAL` or `BIGINT`.


### smallserial

Instead of:

```sql
CREATE TABLE posts (
  id smallserial primary key
)
```

Use:

```sql
CREATE TABLE posts (
  id bigserial primary key
)
```


### smallint

Instead of:

```sql
CREATE TABLE posts (
  id smallint primary key
)
```

Use:

```sql
CREATE TABLE posts (
  id bigint primary key
)
```

## related

See ["prefer-bigint-over-int"](./prefer-bigint-over-int.md) for a simliar lint rule against 32 bit integers.
