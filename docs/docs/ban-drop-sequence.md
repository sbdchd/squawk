---
id: ban-drop-sequence
title: ban-drop-sequence
---

## problem

Dropping a sequence may break existing clients. For example, an earlier application revision may still call `nextval('s')` after a database migration drops `s`. Rolling back the application does not restore the sequence, so that call fails. `CASCADE` can also remove defaults that depend on the sequence.

```sql
DROP SEQUENCE IF EXISTS s CASCADE;
```

## solution

Update clients to stop using the sequence. Keep it until all running application revisions, including any revision used for rollback, no longer depend on it. Drop the sequence in a later migration.
