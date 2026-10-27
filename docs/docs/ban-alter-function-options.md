---
id: ban-alter-function-options
title: ban-alter-function-options
---

## problem

`ALTER FUNCTION`, `ALTER PROCEDURE`, and `ALTER ROUTINE` options can change execution behavior or security for existing clients that call them. This includes security modes and `SET` or `RESET` configuration options, such as `search_path`. This rule is opt-in. It does not report renames, ownership changes, or schema moves.

```sql
alter function f() security definer;
alter procedure p() set search_path to private;
alter routine f() reset all;
```

## solution

Review callers before changing the options. Enable this rule with `--include ban-alter-function-options`.
