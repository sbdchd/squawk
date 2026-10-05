---
id: ban-drop-schema
title: ban-drop-schema
---

## problem

Dropping a schema may break existing clients. For example, an earlier application revision may still query `s.orders` after a database migration drops `s`. Rolling back the application does not restore the schema, so that query fails. `CASCADE` also drops objects in the schema.

```sql
DROP SCHEMA IF EXISTS s CASCADE;
```

## solution

Update clients to stop using the schema and its objects. Keep the schema until all running application revisions, including any revision used for rollback, no longer depend on it. Drop the schema in a later migration.
