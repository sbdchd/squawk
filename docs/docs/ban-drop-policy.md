---
id: ban-drop-policy
title: ban-drop-policy
---

## problem

Dropping a policy or rule changes access or rewrite behavior for existing clients. This rule is opt-in.

```sql
drop policy if exists p on t;
```

## solution

Update clients before dropping the policy or rule.

Enable this rule with `--include ban-drop-policy` (or add `ban-drop-policy` to your configured include list).
