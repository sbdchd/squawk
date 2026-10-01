---
id: ban-drop-sequence
title: ban-drop-sequence
---

## problem

Clients that call `nextval` on a dropped sequence fail.

```sql
DROP SEQUENCE IF EXISTS s CASCADE;
```

## solution

Move clients to a new sequence before dropping the old sequence.
