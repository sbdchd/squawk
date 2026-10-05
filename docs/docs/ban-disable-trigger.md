---
id: ban-disable-trigger
title: ban-disable-trigger
---

## problem

Disabling a trigger or rule, enabling a replica or always trigger or rule, or changing row level security enforcement (including `NO FORCE ROW LEVEL SECURITY`) can change behaviour without a client error. This rule is opt-in.

```sql
ALTER TABLE t DISABLE TRIGGER trg;
```

## solution

Update clients before changing trigger, rule, or row level security enforcement.

Enable this rule with `--include ban-disable-trigger` (or add `ban-disable-trigger` to your configured include list).
