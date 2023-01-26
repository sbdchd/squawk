---
id: prefer-bigint-over-int
title: prefer-bigint-over-int
---


## problem

Using 32 bit integer fields can result in hitting the max int limit.

## solution

Use 64 bit integer field instead, like `BIGSERIAL` or `BIGINT`.


### serial

Instead of:

```sql
CREATE TABLE posts (
  id serial primary key
)
```

Use:

```sql
CREATE TABLE posts (
  id bigserial primary key
)
```


### int

Instead of:

```sql
CREATE TABLE posts (
  id int primary key
)
```

Use:

```sql
CREATE TABLE posts (
  id bigint primary key
)
```

## related

See ["prefer-bigint-over-smallint"](./prefer-bigint-over-smallint.md).
