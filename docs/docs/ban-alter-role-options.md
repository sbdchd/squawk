---
id: ban-alter-role-options
title: ban-alter-role-options
---

## problem

`ALTER ROLE` and its alias `ALTER USER` can change access or behaviour for existing clients that use the role. This rule checks role options and `SET` or `RESET` configuration options, including changes for a specific database. This rule is opt-in. It does not report role renames.

```sql
ALTER ROLE app_user NOLOGIN;
ALTER ROLE app_user SET search_path TO public;
ALTER USER app_user IN DATABASE app_db RESET search_path;
```

## solution

Review clients before changing the role. Enable this rule with `--include ban-alter-role-options`.
