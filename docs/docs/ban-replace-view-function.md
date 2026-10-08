---
id: ban-replace-view-function
title: ban-replace-view-function
---

## problem

`CREATE OR REPLACE` on a view, function, procedure, trigger, rule, or aggregate can change returned values or side effects for existing clients without changing their SQL. This rule is opt-in.

```sql
CREATE OR REPLACE VIEW v AS SELECT 1 AS id;
```

## solution

Deploy compatible clients before replacing the view, function, procedure, trigger, rule, or aggregate.

Enable this rule with `--include ban-replace-view-function` (or add `ban-replace-view-function` to your configured include list).
