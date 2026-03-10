---
id: require-enum-value-ordering
title: require-enum-value-ordering
---

## problem

Using `ALTER TYPE ... ADD VALUE` without specifying `BEFORE` or `AFTER` appends the new value to the end of the enum type.

This can lead to unexpected ordering, especially when enum values have a meaningful order (e.g., workflow status, severity, priority).

```sql
-- existing type
CREATE TYPE status AS ENUM (
  'draft',
  'submitted',
  'approved',
  'rejected'
);
-- existing query to find all not yet approved
select * from papers where status < 'approved';

-- bad
ALTER TYPE status ADD VALUE 'pending_review';

-- now our query isn't including any of the 'pending_review' items!
```

## solution

Explicitly specify where the new value should be inserted using `BEFORE` or `AFTER`.

```sql
-- existing type
CREATE TYPE status AS ENUM (
  'draft',
  'submitted',
  'approved',
  'rejected'
);

-- existing query to find all not yet approved
select * from papers where status < 'approved';

-- good
ALTER TYPE status ADD VALUE 'pending_review' BEFORE 'approved';
-- or
ALTER TYPE status ADD VALUE 'pending_review' AFTER 'submitted';
```

## links

- https://www.postgresql.org/docs/current/sql-altertype.html
