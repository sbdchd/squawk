---
id: ban-drop-domain
title: ban-drop-domain
---

## problem

Clients that use a dropped domain in casts or parameters fail.

```sql
drop domain if exists d cascade;
```

## solution

Move clients to a replacement domain before dropping the old domain.
