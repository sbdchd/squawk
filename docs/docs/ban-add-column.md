---
id: ban-add-column
title: ban-add-column
---

## problem

Adding a column to an existing table or foreign table changes the shape of `SELECT *` results and can break positional inserts from existing clients. This rule is opt-in. An unconditional `CREATE TABLE` earlier in the file suppresses a warning for that new table.

```sql
ALTER TABLE accounts ADD COLUMN status text;
```

## solution

Check clients that use `SELECT *` or positional inserts before adding the column.

Enable this rule with `--include ban-add-column`.
