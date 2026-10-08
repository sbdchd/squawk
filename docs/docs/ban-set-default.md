---
id: ban-set-default
title: ban-set-default
---

## problem

A changed default can make existing clients write different values when they omit a column of a table, foreign table, or view, or a domain value. This rule is opt-in.

```sql
ALTER TABLE t ALTER COLUMN c SET DEFAULT 1;
```

## solution

Update clients to supply explicit values before changing the default.

Enable this rule with `--include ban-set-default` (or add `ban-set-default` to your configured include list).
