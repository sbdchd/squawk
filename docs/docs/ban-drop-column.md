---
id: ban-drop-column
title: ban-drop-column
---

## problem

Dropping a column may break existing clients.

## solution

Update your application code to no longer read or write the column.

You can leave the column as nullable or delete the column once queries no longer select or modify the column.