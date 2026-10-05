---
id: ban-add-composite-attribute
title: ban-add-composite-attribute
---

## problem

A new composite attribute changes the structure of values that clients receive. This rule is opt-in. An unconditional CREATE earlier in the file suppresses a warning for that new object.

```sql
ALTER TYPE address ADD ATTRIBUTE street text;
```

## solution

Update clients to handle the new attribute before adding it.

Enable this rule with `--include ban-add-composite-attribute`.
