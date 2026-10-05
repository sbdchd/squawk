---
id: ban-drop-default
title: ban-drop-default
---

## problem

Dropping a table, foreign table, view column, or domain default may break existing clients. Inserts that omit a `NOT NULL` column can fail with `23502 not_null_violation`. Inserts that omit a nullable column can write `NULL`.

## solution

Update your application code to provide a value for the column, then drop the default in a later migration.
