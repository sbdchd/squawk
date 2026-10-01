---
id: ban-replica-identity
title: ban-replica-identity
---

## problem

Changing replica identity changes the row data available to logical replication consumers. This rule is opt-in.

```sql
ALTER TABLE t REPLICA IDENTITY FULL;
```

## solution

Update replication consumers before changing replica identity.

Enable this rule with `--include ban-replica-identity` (or add `ban-replica-identity` to your configured include list).
