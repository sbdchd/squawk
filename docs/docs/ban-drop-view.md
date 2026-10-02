---
id: ban-drop-view
title: ban-drop-view
---

## problem

Dropping a view or materialized view may break existing clients. Queries against the view can fail with `42P01 undefined_table`.

## solution

Update your application code to no longer use the view, then drop it in a later migration.
