---
id: ban-drop-index
title: ban-drop-index
---

## problem

Dropping an index can remove a unique or exclusion guarantee or change query plans for existing clients. This rule is opt-in.

```sql
drop index concurrently if exists i;
```

## solution

Check client queries and constraints before dropping the index.

Enable this rule with `--include ban-drop-index` (or add `ban-drop-index` to your configured include list).
