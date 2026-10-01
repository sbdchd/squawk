---
id: ban-drop-policy
title: ban-drop-policy
---

## problem

Dropping a policy or rule changes access or rewrite behaviour for clients. This rule is opt-in.

```sql
DROP POLICY IF EXISTS p ON t;
```

## solution

Update clients before dropping the policy or rule.

Enable this rule with `--include ban-drop-policy` (or add `ban-drop-policy` to your configured include list).
