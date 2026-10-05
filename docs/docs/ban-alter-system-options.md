---
id: ban-alter-system-options
title: ban-alter-system-options
---

`ALTER SYSTEM SET` and `ALTER SYSTEM RESET` change server configuration for every database and every client. Parameters such as `search_path`, `timezone`, `DateStyle`, `IntervalStyle`, `bytea_output`, and `standard_conforming_strings` change name resolution or result formats for existing clients. This rule is opt-in.

```sql
ALTER SYSTEM SET timezone = 'UTC';
ALTER SYSTEM RESET search_path;
```

Review clients before changing server settings. Enable this rule with `--include ban-alter-system-options`.
