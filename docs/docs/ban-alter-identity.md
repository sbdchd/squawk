---
id: ban-alter-identity
title: ban-alter-identity
---

## problem

Changing identity can make inserts with explicit or omitted values fail. This rule is enabled by default. Check how the old application inserts values before changing identity.

```sql
ALTER TABLE t ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY;
```

## solution

Update client inserts before changing the identity setting.

Exclude this rule with `--exclude ban-alter-identity` after checking application compatibility.
