---
id: ban-alter-extension
title: ban-alter-extension
---

## problem

`ALTER EXTENSION ... UPDATE` runs the extension's upgrade scripts. These scripts can remove functions, change function signatures, or change results for existing clients. `ALTER EXTENSION ... DROP` removes an object from the extension so that later extension changes no longer manage it. This rule is opt-in. It does not report `ALTER EXTENSION ... ADD`. `ALTER EXTENSION ... SET SCHEMA` is reported by `ban-set-schema`.

```sql
ALTER EXTENSION postgis UPDATE TO '3.4.0';
ALTER EXTENSION postgis DROP FUNCTION legacy_fn();
```

## solution

Read the extension's upgrade notes and update clients before updating the extension. Enable this rule with `--include ban-alter-extension`.
