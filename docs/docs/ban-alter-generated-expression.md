---
id: ban-alter-generated-expression
title: ban-alter-generated-expression
---

## problem

Adding a generated column changes the result shape for strict `SELECT *` decoders and can affect positional inserts. Inserts with explicit column lists do not inherently fail. `SET EXPRESSION` changes values returned to existing clients. For stored generated columns, PostgreSQL rewrites stored values; assess that rewrite separately as an availability concern. This rule is opt-in because compatibility depends on how clients use the column.

```sql
ALTER TABLE t ALTER COLUMN c SET EXPRESSION AS (id + 1);
```

## solution

Check client reads and positional inserts before adding a generated column. Check client reads before changing its expression. The default [`ban-drop-generated-expression`](./ban-drop-generated-expression.md) rule checks `DROP EXPRESSION`.

Enable this rule with `--include ban-alter-generated-expression` (or add `ban-alter-generated-expression` to your configured include list).
