---
id: ban-add-enum-value
title: ban-add-enum-value
---

## problem

A new enum value can reach existing clients that do not handle it. This rule is opt-in. An unconditional CREATE earlier in the file suppresses a warning for that new object.

```sql
ALTER TYPE mood ADD VALUE 'happy';
```

## solution

Update clients to handle the new enum value before adding it.

Enable this rule with `--include ban-add-enum-value`.
