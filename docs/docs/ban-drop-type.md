---
id: ban-drop-type
title: ban-drop-type
---

## problem

Dropping a type, cast, operator, operator class, or operator family may break existing clients. Casts and parameters that name the type can fail with `42704 undefined_object`. `CASCADE` can also drop columns of that type.

## solution

Update your application code to no longer use the type, then drop it in a later migration.
