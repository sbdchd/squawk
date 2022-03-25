---
id: ban-drop-column
title: ban-drop-column
---

## problem

Dropping a column may break existing clients.

## solution

Update your application code to no longer read or write the column. You can delete the column once that change has been rolled out.