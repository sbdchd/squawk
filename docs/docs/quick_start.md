---
id: quick_start
title: Quick Start
sidebar_label: Quick Start
slug: /
---

## install

Squawk is distributed via NPM but can also be downloaded from the [GitHub Release page](https://github.com/sbdchd/squawk/releases).

1. install via NPM

   ```bash
   npm install squawk-cli
   ```

2. lint a Postgres SQL migration

   ```bash
   echo 'ALTER TABLE app_user
   ADD CONSTRAINT app_user_uniq_email UNIQUE (email);' > bad_migration.sql

   squawk bad_migration.sql
   ```

   ```
   bad_migration.sql:1:0: warning: disallowed-unique-constraint

     1 | ALTER TABLE app_user ADD CONSTRAINT app_user_uniq_email UNIQUE (email);

     note: Adding a UNIQUE constraint requires an ACCESS EXCLUSIVE lock which blocks reads.
     help: Create an index CONCURRENTLY and create the constraint using the index.

   bad_migration.sql:1:0: warning: prefer-robust-stmts

     1 | ALTER TABLE app_user ADD CONSTRAINT app_user_uniq_email UNIQUE (email);

     help: Consider wrapping in a transaction or adding a IF NOT EXISTS clause.

   ```
