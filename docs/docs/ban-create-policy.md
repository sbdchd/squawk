---
id: ban-create-policy
title: ban-create-policy
---

## problem

`CREATE POLICY` can change access for existing clients when row level security is enabled. This rule is opt-in.

```sql
create policy p on accounts to app_user using (owner_id = current_user_id());
```

## solution

Review the policy command, roles, and conditions before applying it.

Enable this rule with `--include ban-create-policy`.
