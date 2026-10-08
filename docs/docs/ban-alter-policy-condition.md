---
id: ban-alter-policy-condition
title: ban-alter-policy-condition
---

## problem

`ALTER POLICY` with `USING` or `WITH CHECK` changes which rows existing clients can read or write. This rule is opt-in. It does not report policy renames or role-only changes.

```sql
ALTER POLICY p ON accounts USING (owner_id = current_user_id());
```

## solution

Review the condition and update clients before applying it. Enable this rule with `--include ban-alter-policy-condition`.
