---
id: ban-drop-trigger
title: ban-drop-trigger
---

## problem

Dropping a trigger may silently change behaviour for existing clients. For example, updates may stop setting `updated_at` or writing audit rows. This rule is opt-in.

## solution

Update your application code to no longer depend on the trigger, then drop it in a later migration.

Enable this rule with `--include ban-drop-trigger` (or add `ban-drop-trigger` to your configured include list).
