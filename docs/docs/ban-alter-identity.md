---
id: ban-alter-identity
title: ban-alter-identity
---

## problem

Changing identity can make inserts with explicit or omitted values fail. This rule is opt-in because compatibility depends on how clients insert values.

```sql
ALTER TABLE t ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY;
```

## solution

Update client inserts before changing the identity setting.

Enable this rule with `--include ban-alter-identity` (or add `ban-alter-identity` to your configured include list).
