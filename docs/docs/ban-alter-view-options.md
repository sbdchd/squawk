---
id: ban-alter-view-options
title: ban-alter-view-options
---

## problem

`ALTER VIEW ... SET` and `ALTER VIEW ... RESET` change options on an existing view. Existing clients, including an earlier application revision after a rollback, may still use that view after a migration. Rolling back the application does not restore the previous view options.

For example, setting `security_invoker = true` makes PostgreSQL check access to the underlying tables and apply row-level security as the caller rather than the view owner. An earlier application revision that could query the view before the migration may then get a permission error or see different rows. Changing `security_barrier` can change when view conditions are evaluated relative to caller-supplied functions, which can affect information exposure and query performance. Changing `check_option` can reject writes through the view that previously succeeded.

```sql
ALTER VIEW v SET (security_invoker = true);
ALTER VIEW v RESET (security_barrier);
```

This rule is opt-in. It reports view option changes, not view renames or column default changes.

## solution

Review the privileges, row-level security policies, and read and write queries of application revisions that may use the migrated database. Enable this rule with `--include ban-alter-view-options`.
