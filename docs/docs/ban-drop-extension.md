---
id: ban-drop-extension
title: ban-drop-extension
---

`DROP EXTENSION` removes the extension and its objects. Existing clients can depend on those objects. This rule is enabled by default.

```sql
DROP EXTENSION IF EXISTS hstore;
```

Update clients before dropping the extension. To disable this rule, use `--exclude ban-drop-extension`.
