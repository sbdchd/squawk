---
id: ban-create-policy
title: ban-create-policy
---

`CREATE POLICY` can change access for clients when row level security is enabled. Review the policy command, roles, and conditions before applying it. This rule is opt-in.

```sql
CREATE POLICY p ON accounts TO app_user USING (owner_id = current_user_id());
```

Enable this rule with `--include ban-create-policy`.
