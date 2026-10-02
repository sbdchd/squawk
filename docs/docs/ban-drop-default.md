---
id: ban-drop-default
title: ban-drop-default
---

## problem

Dropping a column default may break existing clients. Inserts that omit a `NOT NULL` column fail with `23502 not_null_violation`. Inserts that omit a nullable column silently write `NULL`.

## solution

Update your application code to provide a value for the column, then drop the default in a later migration.
