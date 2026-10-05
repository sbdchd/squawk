---
id: ban-alter-policy-roles
title: ban-alter-policy-roles
---

`ALTER POLICY ... TO` changes which roles the policy applies to. This can change client access. This rule is opt-in. It does not report policy renames or condition-only changes.

```sql
ALTER POLICY p ON accounts TO app_user;
```

Review the affected roles before applying the change. Enable this rule with `--include ban-alter-policy-roles`.
