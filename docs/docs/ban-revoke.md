---
id: ban-revoke
title: ban-revoke
---

## problem

Revoking privileges, removing role membership with `ALTER GROUP ... DROP USER`, dropping a role or user, changing object ownership with `OWNER TO` or `REASSIGN OWNED`, or running `DROP OWNED` can make queries from existing clients fail. `DROP OWNED` can also remove objects owned by the role. This rule is opt-in.

```sql
REVOKE SELECT ON t FROM app;
ALTER GROUP writers DROP USER app;
```

## solution

Move clients to a role with the required privileges before revoking them.

Enable this rule with `--include ban-revoke` (or add `ban-revoke` to your configured include list).
