---
id: renaming-object
title: renaming-object
---

## problem

Clients that use an old object name or enum value can fail after a rename.

```sql
ALTER VIEW v RENAME TO v2;
```

## solution

Update clients to use the new name before renaming the object.
