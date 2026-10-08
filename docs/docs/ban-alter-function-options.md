---
id: ban-alter-function-options
title: ban-alter-function-options
---

## problem

`ALTER FUNCTION`, `ALTER PROCEDURE`, and `ALTER ROUTINE` options can change execution behaviour or security for existing clients that call them. This includes security modes and `SET` or `RESET` configuration options, such as `search_path`. This rule is opt-in. It does not report renames, ownership changes, or schema moves.

```sql
ALTER FUNCTION f() SECURITY DEFINER;
ALTER PROCEDURE p() SET search_path TO private;
ALTER ROUTINE f() RESET ALL;
```

## solution

Review callers before changing the options. Enable this rule with `--include ban-alter-function-options`.
