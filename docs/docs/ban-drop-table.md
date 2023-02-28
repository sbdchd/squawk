---
id: ban-drop-table
title: ban-drop-table
---

## problem

Dropping a table may break existing clients.

## solution

Update your application code to no longer read or write the table.

Once the table is no longer needed, you can delete it by running the command "DROP TABLE mytable;". 

This command will permanently remove the table from the database and all its contents. 

Be sure to back up the table before deleting it, just in case you need to restore it in the future.
