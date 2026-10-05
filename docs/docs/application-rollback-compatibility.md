---
id: application-rollback-compatibility
title: Application rollback compatibility
---

A migration can finish without blocking traffic and still break an application-only rollback. An application-only rollback keeps the migrated database and all data written by the new application. Run the old application against that database with its actual database role before declaring the migration compatible.

## Use the compatibility rules

Default rules detect some operations that remove objects or guarantees. Additional rules are opt-in because the effect depends on application writes, reads, permissions, and data. Enable them with `--include <rule-name>`. A `NOT VALID` constraint still rejects new invalid writes. A unique index built `CONCURRENTLY` still enforces uniqueness. Neither option proves rollback compatibility.

The linter checks parsed SQL, not the live database schema. Some opt-in rules skip objects created unconditionally earlier in the same migration; this is only a statement-level approximation. Explicit inclusion, exclusion, and ignore comments still apply. The rule documentation lists each SQL form.

## Verify an application-only rollback

1. Apply the migration to representative data.
2. Start the new application and write representative new data.
3. Start the old application against the same database, with its actual database role.
4. Check reads, writes, result decoding, authorization, identifier generation, and database side effects.
5. Repeat for every application version that remains an allowed rollback target.

Review data migrations (including status strings, JSON formats, destructive updates, plain `TRUNCATE`, and deletes), function and trigger bodies, role membership and privileges, and SQL inside procedural blocks or dynamic SQL separately. Lint results do not prove compatibility. Keep lock-duration and table-rewrite checks separate from application compatibility checks.
