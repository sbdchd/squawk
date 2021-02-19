---
id: constraint-missing-not-valid
title: constraint-missing-not-valid
---

Check that all new constraints have `NOT VALID`.

By default new constraints require a table scan and block writes to the
table. Using `NOT VALID` with a later `VALIDATE CONSTRAINT` call prevents the
table scan and results in the validation step only requiring a `SHARE UPDATE EXCLUSIVE` lock.

<https://www.postgresql.org/docs/current/sql-altertable.html#SQL-ALTERTABLE-NOTES>

Instead of:

```sql
ALTER TABLE "accounts" ADD CONSTRAINT "positive_balance" CHECK ("balance" >= 0);
```

Use:

```sql
ALTER TABLE "accounts" ADD CONSTRAINT "positive_balance" CHECK ("balance" >= 0) NOT VALID;
ALTER TABLE accounts VALIDATE CONSTRAINT positive_balance;
```
