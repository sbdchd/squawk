---
id: ban-drop-domain
title: ban-drop-domain
---

## problem

Clients that use a dropped domain in casts or parameters fail.

```sql
DROP DOMAIN IF EXISTS d CASCADE;
```

## solution

Move clients to a replacement domain before dropping the old domain.
