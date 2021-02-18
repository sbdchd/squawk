---
id: quick_start
title: Quick Start
sidebar_label: Quick Start
slug: /
---

1. install via NPM

   ```bash
   npm install squawk-cli
   ```

   Squawk can also be downloaded from the [GitHub Release page](https://github.com/sbdchd/squawk/releases).

2. lint a Postgres SQL migration

   ```bash
   # create a fake migration
   echo 'ALTER TABLE app_user
   ADD CONSTRAINT app_user_uniq_email UNIQUE (email);' > bad_migration.sql

   # lint migration with squawk
   squawk bad_migration.sql

   # squawk will print the following output:

   # bad_migration.sql:1:0: warning: disallowed-unique-constraint

   #    1 | ALTER TABLE app_user
   #    2 |    ADD CONSTRAINT app_user_uniq_email UNIQUE (email);

   #   note: Adding a UNIQUE constraint requires an ACCESS EXCLUSIVE lock which blocks reads.
   #   help: Create an index CONCURRENTLY and create the constraint using the index.
   ```

   Each rule has documentation with more examples and reasoning if you have any questions (see the left sidebar).

3. Update your migration to address Squawk's suggestions.

   ```
   ❯ squawk fixed_migration.sql
   Found 0 issues in 1 file 🎉
   ```

4. Your migration is now ready to be applied! See ["Running Migrations"](./safe_migrations.md#safety-requirements) for safety information about applying migrations in Postgres.

The [CLI docs](./cli.md) have more information about the `squawk` CLI tool and the [GitHub Integration docs](./github_app.md) outline configuring Squawk to work with GitHub pull requests.
