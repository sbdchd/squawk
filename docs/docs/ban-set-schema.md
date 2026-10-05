---
id: ban-set-schema
title: ban-set-schema
---

## problem

Moving an object to another schema may break existing clients that use its old schema-qualified name. For example, an earlier application revision may still query `public.t` after a migration moves the table to `s`. Rolling back the application does not move the table back, so that query fails. This also applies to foreign tables, procedures, and routines.

```sql
ALTER TABLE t SET SCHEMA s;
```

## solution

Update clients to use the new schema-qualified name. Keep the object at its old name until all running application revisions, including any revision used for rollback, no longer depend on it. Move the object in a later migration.
