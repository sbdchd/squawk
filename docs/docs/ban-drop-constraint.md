---
id: ban-drop-constraint
title: ban-drop-constraint
---

## problem

Dropping a constraint removes a foreign key, check, or uniqueness guarantee that clients can depend on. This rule is opt-in because compatibility depends on whether clients rely on the constraint.

```sql
ALTER TABLE t DROP CONSTRAINT IF EXISTS c;
```

## solution

Update clients to not depend on the constraint before dropping it.

Enable this rule with `--include ban-drop-constraint` (or add `ban-drop-constraint` to your configured include list).
