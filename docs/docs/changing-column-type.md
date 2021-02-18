---
id: changing-column-type
title: changing-column-type
---

Changing a column type requires an `ACCESS EXCLUSIVE` lock on the table which blocks reads.

Changing the type of the column may also break other clients reading from the
table.

<https://www.postgresql.org/docs/current/sql-altertable.html#SQL-ALTERTABLE-NOTES>
