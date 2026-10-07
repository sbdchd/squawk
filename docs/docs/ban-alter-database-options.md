---
id: ban-alter-database-options
title: ban-alter-database-options
---

`ALTER DATABASE` options and configuration changes can change connection behaviour, name resolution, or results for existing clients in the database. This rule is opt-in. It does not report database renames, owner changes, or tablespace changes.

```sql
ALTER DATABASE app SET search_path TO public;
ALTER DATABASE app WITH CONNECTION LIMIT 20;
```

Review clients before changing database settings. Enable this rule with `--include ban-alter-database-options`.
