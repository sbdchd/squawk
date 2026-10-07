---
id: ban-disable-trigger
title: ban-disable-trigger
---

## problem

Disabling or enabling a trigger or rule, enabling replica-only or always firing, or changing row level security enforcement (including `NO FORCE ROW LEVEL SECURITY`) can change data or reject access. `ENABLE REPLICA TRIGGER` and `ENABLE REPLICA RULE` stop the trigger or rule from firing for normal application writes. An earlier application revision may depend on the side effects that those writes previously produced. Some enforcement changes cause explicit errors. This rule is opt-in.

```sql
ALTER TABLE t DISABLE TRIGGER trg;
```

## solution

Test the earlier application revision against the migrated database before changing trigger, rule, or row level security enforcement. Keep any trigger or rule that its normal writes require active for normal application sessions.

Enable this rule with `--include ban-disable-trigger` (or add `ban-disable-trigger` to your configured include list).
