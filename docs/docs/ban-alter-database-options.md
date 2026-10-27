---
id: ban-alter-database-options
title: ban-alter-database-options
---

## problem

`ALTER DATABASE` options and configuration changes can change connection behavior, name resolution, or results for existing clients in the database. This rule is opt-in. It does not report database renames, owner changes, or tablespace changes.

```sql
alter database app set search_path to public;
alter database app with connection limit 20;
```

## solution

Review clients before changing database settings. Enable this rule with `--include ban-alter-database-options`.
