---
id: ban-alter-role-options
title: ban-alter-role-options
---

## problem

`ALTER ROLE` and its alias `ALTER USER` can change access or behaviour for existing clients that use the role. This rule checks role options and `SET` or `RESET` configuration options, including changes for a specific database. This rule is opt-in. It does not report role renames.

```sql
alter role app_user nologin;
alter role app_user set search_path to public;
alter user app_user in database app_db reset search_path;
```

## solution

Review clients before changing the role. Enable this rule with `--include ban-alter-role-options`.
