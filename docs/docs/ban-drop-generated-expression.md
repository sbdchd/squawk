---
id: ban-drop-generated-expression
title: ban-drop-generated-expression
---

## What it does

Detects `ALTER TABLE ... ALTER COLUMN ... DROP EXPRESSION` by default.

## Why

Dropping an expression changes a stored generated column into an ordinary column. Existing stored values remain, but PostgreSQL no longer computes the value for future inserts or updates. An insert that omits the column can return `NULL` instead of a computed value. Review how the old application reads and writes this column before removing the expression.

Adding a generated column and replacing an expression have different risks. The opt-in `ban-alter-generated-expression` rule covers those operations.

## Example

```sql
ALTER TABLE line_items ALTER COLUMN total DROP EXPRESSION;
```
