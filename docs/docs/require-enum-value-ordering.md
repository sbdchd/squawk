
---
id: require-enum-value-ordering
title: require-enum-value-ordering
---

## problem

Using `ALTER TYPE ... ADD VALUE` without specifying `BEFORE` or `AFTER` appends the new value to the end of the enum type. This can lead to unexpected ordering, especially when enum values have a meaningful order (e.g., status workflows, severity levels).

```sql
-- bad
ALTER TYPE status ADD VALUE 'pending_review';
```

## solution

Explicitly specify where the new value should be inserted using `BEFORE` or `AFTER`.

```sql
-- good
ALTER TYPE status ADD VALUE 'pending_review' BEFORE 'approved';
ALTER TYPE status ADD VALUE 'pending_review' AFTER 'submitted';
```

## links

- https://www.postgresql.org/docs/current/sql-altertype.html
