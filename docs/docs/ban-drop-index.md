---
id: ban-drop-index
title: ban-drop-index
---

## problem

Dropping an index can remove a unique or exclusion guarantee or change query plans. This rule is opt-in.

```sql
DROP INDEX CONCURRENTLY IF EXISTS i;
```

## solution

Check client queries and constraints before dropping the index.

Enable this rule with `--include ban-drop-index` (or add `ban-drop-index` to your configured include list).
