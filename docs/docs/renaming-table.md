---
id: renaming-table
title: renaming-table
---

## problem

Renaming a table may break existing clients that depend on the old table name.

During deployments, you can have multiple versions of your app running at the same time. If you rename a table that the old version of your app depends on, you'll

## solutions

### don't rename the table

This is the simplest solution. If you're using an ORM (Object Relational Mapper), you can rename the object in your code, but leave the SQL table name as is.

### rename with minor locking via database views

We have a table with the name `user_stars` that we want to rename to `user_favorites`. Database [views](https://www.postgresql.org/docs/devel/sql-createview.html) allow us to rename a table with temporary locking during the rename.

1. Create a view with the name of your desired table using our original table

```sql
CREATE VIEW user_favorites as SELECT * FROM user_stars;
```

2. Deploy your new code that references `user_favorites` and remove your old deployment that references `user_stars`

3. Delete our view and rename the table now that `user_stars` is no longer queried directly.

```sql
BEGIN;
DROP VIEW user_favorites;
ALTER TABLE user_stars RENAME TO user_favorites;
COMMIT;
```

This transaction will acquire an `ACCESS EXCLUSIVE` lock on the `user_stars` table, blocking all reads and writes to the table while the table is renamed. This should effectively be instantaneous.

### use a shadow table for zero locking

A complicated solution that eliminates the need for locking is to create a new table with triggers to keep both tables in sync. Backfill the new table from the old table and then transition reads/writes to the new table. Once all reads/writes are transitioned, delete the old table. See ["Hot Swapping Production Tables for Safe Database Backfills"](https://doordash.engineering/2020/10/21/hot-swapping-production-data-tables/) for more information.
