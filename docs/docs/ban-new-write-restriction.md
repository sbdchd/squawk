---
id: ban-new-write-restriction
title: ban-new-write-restriction
---

## problem

New table, foreign-table, and domain constraints, `NOT VALID` constraints, `SET NOT NULL`, named `NOT NULL` constraints, constrained added columns (including inline primary keys), changed constraint timing, and unique indexes (including `CONCURRENTLY`) can reject writes from existing clients. This rule is opt-in. An unconditional `CREATE TABLE` earlier in the file suppresses a warning for that table. The rule does not know whether an object existed before the migration.

```sql
alter table t add constraint c check (id > 0) not valid;
alter table t add constraint id_required not null id;
alter table t add column external_id bigint primary key;
alter foreign table ft add column external_id bigint not null;
```

## solution

Update client writes before adding the restriction.

Enable this rule with `--include ban-new-write-restriction`.
