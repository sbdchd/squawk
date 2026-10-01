---
id: ban-alter-generated-expression
title: ban-alter-generated-expression
---

## problem

A stored generated column rejects inserts that supply its value. Dropping its expression changes the column semantics. This rule is opt-in because compatibility depends on how clients use the column.

```sql
ALTER TABLE t ALTER COLUMN c DROP EXPRESSION;
```

## solution

Update client inserts before adding or changing a generated column.

Enable this rule with `--include ban-alter-generated-expression` (or add `ban-alter-generated-expression` to your configured include list).
