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

<details><summary><code>example-migration.sql</code></summary>

<br/>

```bash
# create example SQL migration
cat <<EOF > example-migration.sql
BEGIN;
ALTER TABLE table_name ADD CONSTRAINT field_name_constraint UNIQUE (field_name);
COMMIT;
EOF

# lint with squawk
squawk example-migration.sql
```

</details>

<br/>

```shell
❯ squawk example-migration.sql
example-migration.sql:2:1: warning: disallowed-unique-constraint

   2 | ALTER TABLE table_name ADD CONSTRAINT field_name_constraint UNIQUE (field_name);

  note: Adding a UNIQUE constraint requires an ACCESS EXCLUSIVE lock which blocks reads.
  help: Create an index CONCURRENTLY and create the constraint using the index.
```

See ["Running Migrations"](./safe_migrations.md#safety-requirements) for information about safely applying migrations in Postgres.

The [CLI docs](./cli.md) have more information about the `squawk` CLI tool and the [GitHub Integration docs](./github_app.md) outline configuring Squawk to work with GitHub pull requests.
