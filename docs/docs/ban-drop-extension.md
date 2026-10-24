---
id: ban-drop-extension
title: ban-drop-extension
---

## problem

`DROP EXTENSION` removes the extension and its objects. Existing clients can depend on those objects. This rule is enabled by default.

```sql
drop extension if exists hstore;
```

## solution

Update clients before dropping the extension. To disable this rule, use `--exclude ban-drop-extension`.
