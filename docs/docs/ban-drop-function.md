---
id: ban-drop-function
title: ban-drop-function
---

## problem

Dropping a function or procedure may break existing clients. Calls can fail with `42883 undefined_function`.

## solution

Update your application code to no longer call the function or procedure, then drop it in a later migration.
