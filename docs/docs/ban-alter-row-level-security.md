---
id: ban-alter-row-level-security
title: ban-alter-row-level-security
---

`ALTER TABLE` with `ENABLE`, `DISABLE`, `FORCE`, or `NO FORCE ROW LEVEL SECURITY` changes access for existing clients. This rule is opt-in.

```sql
ALTER TABLE accounts ENABLE ROW LEVEL SECURITY;
```

Review table policies and client access before applying the change. Enable this rule with `--include ban-alter-row-level-security`.
