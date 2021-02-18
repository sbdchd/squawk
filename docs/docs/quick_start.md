---
id: quick_start
title: Quick Start
sidebar_label: Quick Start
slug: /
---

## Install

Note: due to `squawk`'s dependency on
[`libpg_query`](https://github.com/lfittl/libpg_query/issues/44), `squawk`
only supports Linux and macOS

```shell
npm install -g squawk-cli

# or install binaries directly via the releases page
https://github.com/sbdchd/squawk/releases
```

## Usage

```shell
❯ squawk example.sql
example.sql:2:1: warning: prefer-text-field

   2 | --
   3 | -- Create model Bar
   4 | --
   5 | CREATE TABLE "core_bar" (
   6 |     "id" serial NOT NULL PRIMARY KEY,
   7 |     "alpha" varchar(100) NOT NULL
   8 | );

  note: Changing the size of a varchar field requires an ACCESS EXCLUSIVE lock.
  help: Use a text field with a check constraint.

example.sql:9:2: warning: require-concurrent-index-creation

   9 |
  10 | CREATE INDEX "field_name_idx" ON "table_name" ("field_name");

  note: Creating an index blocks writes.
  note: Create the index CONCURRENTLY.

example.sql:11:2: warning: disallowed-unique-constraint

  11 |
  12 | ALTER TABLE table_name ADD CONSTRAINT field_name_constraint UNIQUE (field_name);

  note: Adding a UNIQUE constraint requires an ACCESS EXCLUSIVE lock which blocks reads.
  help: Create an index CONCURRENTLY and create the constraint using the index.

example.sql:13:2: warning: adding-field-with-default

  13 |
  14 | ALTER TABLE "core_recipe" ADD COLUMN "foo" integer DEFAULT 10;

  note: In Postgres versions <11 adding a field with a DEFAULT requires a table rewrite with an ACCESS EXCLUSIVE lock.
  help: Add the field as nullable, then set a default, backfill, and remove nullabilty.
```

See ["Running Migrations"](./safe_migrations.md#safety-requirements) for information about safely applying migrations in Postgres.

The [CLI docs](./cli.md) have more information about the `squawk` CLI tool and the [GitHub Integration docs](./github_app.md) outline configuring Squawk to work with GitHub pull requests.
