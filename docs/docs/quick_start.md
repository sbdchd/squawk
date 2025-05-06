---
id: quick_start
title: Quick Start
sidebar_label: Quick Start
slug: /
---

## Install

```shell
npm install -g squawk-cli

# or install binaries directly via the releases page
https://github.com/sbdchd/squawk/releases
```

**NOTE**: You can also try Squawk in the browser via the [Squawk Playground](https://play.squawkhq.com)!

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

```
❯ squawk example-migration.sql
warning[constraint-missing-not-valid]: By default new constraints require a table scan and block writes to the table while that scan occurs.
 --> example-migration.sql:2:24
  |
2 | ALTER TABLE table_name ADD CONSTRAINT field_name_constraint UNIQUE (field_name);
  |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
warning[disallowed-unique-constraint]: Adding a `UNIQUE` constraint requires an `ACCESS EXCLUSIVE` lock which blocks reads and writes to the table while the index is built.
 --> example-migration.sql:2:28
  |
2 | ALTER TABLE table_name ADD CONSTRAINT field_name_constraint UNIQUE (field_name);
  |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = help: Create an index CONCURRENTLY and create the constraint using the index.

Find detailed examples and solutions for each rule at https://squawkhq.com/docs/rules
Found 2 issues in 1 file (checked 1 source file)
```

See ["Running Migrations"](./safe_migrations.md#safety-requirements) for information about safely applying migrations in Postgres.

The [CLI docs](./cli.md) have more information about the `squawk` CLI tool and the [GitHub Integration docs](./github_app.md) outline configuring Squawk to work with GitHub pull requests.

## Playground

You can also lint your SQL in the [Squawk Playground](https://play.squawkhq.com) using the WASM compiled version!

## GitHub Action

```yml
# .github/workflows/lint-migrations.yml
name: Lint Migrations

on: pull_request

jobs:
  lint_migrations:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v1
      - name: Find modified migrations
        run: |
          modified_migrations=$(git diff --name-only origin/$GITHUB_BASE_REF...origin/$GITHUB_HEAD_REF 'migrations/*.sql')
          echo "$modified_migrations"
          echo "::set-output name=file_names::$modified_migrations"
        id: modified-migrations
      - uses: sbdchd/squawk-action@v1
        with:
          pattern: ${{ steps.modified-migrations.outputs.file_names }}
```

The [GitHub Integration docs](./github_app.md) has more information on using Squawk with GitHub.
