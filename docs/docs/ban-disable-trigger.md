---
id: ban-disable-trigger
title: ban-disable-trigger
---

## problem

Disabling or enabling a trigger or rule, or enabling replica-only or always firing, can change data or reject access for existing clients. `ENABLE REPLICA TRIGGER` and `ENABLE REPLICA RULE` stop the trigger or rule from firing for normal application writes. An earlier application revision may depend on the side effects that those writes previously produced. Some enforcement changes cause explicit errors. Row level security changes are covered by [`ban-alter-row-level-security`](ban-alter-row-level-security.md). This rule is opt-in.

```sql
alter table t disable trigger trg;
```

## solution

Test the earlier application revision against the migrated database before changing trigger or rule enforcement. Keep any trigger or rule that its normal writes require active for normal application sessions.

Enable this rule with `--include ban-disable-trigger` (or add `ban-disable-trigger` to your configured include list).
