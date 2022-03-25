---
id: renaming-column
title: renaming-column
---

## problem

Renaming a column may break existing clients.

## solution

There isn't an easy way to rename a column.

If you're using an ORM (object relational mapper), you can rename the field in your application level struct while leaving the column name unchanged.

A more complicated solution using a new column with triggers and backfill to sync the two columns may work.