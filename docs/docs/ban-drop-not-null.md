---
id: ban-drop-not-null
title: ban-drop-not-null
---

## problem

Dropping a NOT NULL constraint may break existing clients.

Application code or code written in procedural languages like PL/SQL or PL/pgSQL may not expect NULL values for the column that was previously guaranteed to be NOT NULL and therefore may fail to process them correctly.

## solution

Consider using a marker value that represents a "null" value and using that throughout application code. If NULL is truly desired, consider creating a new table similiar to the current one but that doesn't have the NOT NULL constraint, copy data to that, and create a view to replace the old table that either filters out NULL values or substitutes an appropriate marker value.
