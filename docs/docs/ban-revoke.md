---
id: ban-revoke
title: ban-revoke
---

## problem

Revoking privileges can make client queries fail with a permission error. This rule is opt-in.

```sql
REVOKE SELECT ON t FROM app;
```

## solution

Move clients to a role with the required privileges before revoking them.

Enable this rule with `--include ban-revoke` (or add `ban-revoke` to your configured include list).
