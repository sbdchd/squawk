---
id: ban-alter-generated-expression
title: ban-alter-generated-expression
---

## problem

Adding a generated column can reject inserts that supply its value. `SET EXPRESSION` changes the values returned for existing rows when PostgreSQL rewrites the table. This rule is opt-in because compatibility depends on how clients use the column.

```sql
ALTER TABLE t ALTER COLUMN c SET EXPRESSION AS (id + 1);
```

## solution

Update client inserts before adding a generated column. Check client reads before changing its expression. `DROP EXPRESSION` is not checked by this rule.

Enable this rule with `--include ban-alter-generated-expression` (or add `ban-alter-generated-expression` to your configured include list).
