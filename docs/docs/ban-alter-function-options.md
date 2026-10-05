---
id: ban-alter-function-options
title: ban-alter-function-options
---

`ALTER FUNCTION` options can change execution behaviour or security for existing callers. This rule is opt-in. It does not report renames, ownership changes, or schema moves.

```sql
ALTER FUNCTION f() SECURITY DEFINER;
```

Review callers before changing the options. Enable this rule with `--include ban-alter-function-options`.
