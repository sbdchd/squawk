---
id: prefer-text-field
title: prefer-text-field
---

Changing the size of a `varchar` field requires an `ACCESS EXCLUSIVE` lock.

Using a text field with a `CHECK CONSTRAINT` makes it easier to change the
max length. See the [`constraint-missing-not-valid` rule](./constraint-missing-not-valid.md).

Instead of:

```sql
CREATE TABLE "app_user" (
    "id" serial NOT NULL PRIMARY KEY,
    "email" varchar(100) NOT NULL
);
```

Use:

```sql
CREATE TABLE "app_user" (
    "id" serial NOT NULL PRIMARY KEY,
    "email" TEXT NOT NULL
);
ALTER TABLE "app_user" ADD CONSTRAINT "text_size" CHECK (LENGTH("email") <= 100);
```
