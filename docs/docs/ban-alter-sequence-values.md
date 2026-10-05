---
id: ban-alter-sequence-values
title: ban-alter-sequence-values
---

## problem

Changes to standalone and identity sequence options can change generated values. This rule is opt-in. An unconditional CREATE earlier in the file suppresses a warning for that new object.

```sql
ALTER SEQUENCE ids RESTART WITH 1;
```

## solution

Coordinate sequence changes with clients that use generated values.

Enable this rule with `--include ban-alter-sequence-values`.
