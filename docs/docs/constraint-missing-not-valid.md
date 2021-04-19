---
id: constraint-missing-not-valid
title: constraint-missing-not-valid
---

Check that all new constraints have `NOT VALID`.

By default new constraints require a table scan and block writes to the table
while that scan occurs. Using `NOT VALID` with a later `VALIDATE CONSTRAINT`
call prevents the table scan and results in the validation step only requiring a
`SHARE UPDATE EXCLUSIVE` lock.

<https://www.postgresql.org/docs/current/sql-altertable.html#SQL-ALTERTABLE-NOTES>

## solution

Instead of:

```sql
ALTER TABLE "accounts" ADD CONSTRAINT "positive_balance" CHECK ("balance" >= 0);
```

Use:

```sql
ALTER TABLE "accounts" ADD CONSTRAINT "positive_balance" CHECK ("balance" >= 0) NOT VALID;
ALTER TABLE accounts VALIDATE CONSTRAINT positive_balance;
```

## how "not valid, validate" works

When we add this constraint, writes to the `accounts` table will be blocked while the table is scanned to verify all `positive_balance` column entries match the check constraint `"balance" >= 0`.

```sql
ALTER TABLE "accounts" ADD CONSTRAINT "positive_balance" CHECK ("balance" >= 0);
```

By adding the check constraint as `NOT VALID`, existing columns are not checked against the constraint, only new rows or updates to existing rows. This constraint can be applied without blocking writes.

Afterwards, we can call `VALIDATE CONSTRAINT` to verify that the existing `positive_balance` entries meet the constraint. This will require a table scan of `accounts`, but writes will not be blocked.

Postgres doesn't need to block writes for `VALIDATE CONSTRAINT` since Postgres will prevent any new rows or modifications from violating the constraint. So Postgres can scan `accounts` and depend on other transactions enforcing the constraint in concurrent updates.
