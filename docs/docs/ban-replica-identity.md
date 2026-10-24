---
id: ban-replica-identity
title: ban-replica-identity
---

## problem

Changing replica identity changes the row data available to logical replication consumers. Dropping a publication or changing its published tables or options can stop or change replication for those consumers. Existing clients that depend on the replicated data may then see stale, missing, or changed data. This rule is opt-in.

```sql
alter table t replica identity full;
alter publication p drop table t;
```

## solution

Update replication consumers before changing replica identity or publication membership and options.

Enable this rule with `--include ban-replica-identity` (or add `ban-replica-identity` to your configured include list).
