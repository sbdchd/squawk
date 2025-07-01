---
id: syntax-error
title: syntax-error
---

## problem

Squawk encountered invalid syntax when parsing.

## examples

trailing comma

```sql
select f(1,2,);
-- error[syntax-error]: unexpected trailing comma
--  --> stdin:1:13
--   |
-- 1 | select f(1,2,);
--   |             ^
--   |
```

missing semicolon

```sql
select * from t
select id from users where email = email;
-- error[syntax-error]: expected SEMICOLON
--  --> stdin:1:16
--   |
-- 1 | select * from t
--   |                ^
--   |
```

## solutions

Fix the syntax error.

:::note
Squawk might be mistaken, if you think that's the case, please [open an issue](https://github.com/sbdchd/squawk/issues/new)!
:::
