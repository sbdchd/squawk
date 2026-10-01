---
id: ban-set-default
title: ban-set-default
---

## problem

Inserts that omit a column write different values after its default changes. This rule is opt-in.

```sql
ALTER TABLE t ALTER COLUMN c SET DEFAULT 1;
```

## solution

Update clients to supply explicit values before changing the default.

Enable this rule with `--include ban-set-default` (or add `ban-set-default` to your configured include list).
