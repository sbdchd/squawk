---
id: ban-replace-view-function
title: ban-replace-view-function
---

## problem

`CREATE OR REPLACE` can change returned values or side effects without changing client SQL. This rule is opt-in.

```sql
CREATE OR REPLACE VIEW v AS SELECT 1 AS id;
```

## solution

Deploy compatible clients before replacing the view, function, or procedure.

Enable this rule with `--include ban-replace-view-function` (or add `ban-replace-view-function` to your configured include list).
