---
id: ban-alter-role-options
title: ban-alter-role-options
---

`ALTER ROLE` options and configuration changes can change access or behaviour for clients that use the role. This rule is opt-in. It does not report role renames.

```sql
ALTER ROLE app_user NOLOGIN;
ALTER ROLE app_user SET search_path TO public;
```

Review clients before changing the role. Enable this rule with `--include ban-alter-role-options`.
