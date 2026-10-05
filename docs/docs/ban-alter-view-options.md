---
id: ban-alter-view-options
title: ban-alter-view-options
---

`ALTER VIEW ... SET` and `ALTER VIEW ... RESET` change view options such as `security_barrier` or `security_invoker`. This rule is opt-in. It does not report view renames or column default changes.

```sql
ALTER VIEW v SET (security_invoker = true);
```

Review client access before changing view options. Enable this rule with `--include ban-alter-view-options`.
