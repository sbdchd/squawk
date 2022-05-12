# squawk [![npm](https://img.shields.io/npm/v/squawk-cli)](https://www.npmjs.com/package/squawk-cli) ![Rust CI](https://github.com/sbdchd/squawk/workflows/Rust%20CI/badge.svg)

> linter for Postgres migrations

[quick start](https://squawkhq.com/docs/) | [rules documentation](https://squawkhq.com/docs/rules) | [github integration](https://squawkhq.com/docs/github_app)

## Why?

Prevent unexpected downtime caused by database migrations and encourage best
practices around Postgres schemas and SQL.

Also it seemed like a nice project to spend more time with Rust.

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
```

### `squawk --help`

```
squawk
Find problems in your SQL

USAGE:
    squawk [FLAGS] [OPTIONS] [paths]... [SUBCOMMAND]

FLAGS:
    -h, --help
            Prints help information

        --list-rules
            List all available rules

    -V, --version
            Prints version information


OPTIONS:
        --dump-ast <dump-ast>
            Output AST in JSON [possible values: Raw, Parsed]

    -e, --exclude <exclude>...
            Exclude specific warnings

            For example: --exclude=require-concurrent-index-creation,ban-drop-database
        --explain <explain>
            Provide documentation on the given rule

        --reporter <reporter>
            Style of error reporting [possible values: Tty, Gcc, Json]


ARGS:
    <paths>...
            Paths to search


SUBCOMMANDS:
    help                Prints this message or the help of the given subcommand(s)
    upload-to-github    Comment on a PR with Squawk's results
```

## Rules

Individual rules can be disabled via the `--exclude` flag

```shell
squawk --exclude=adding-field-with-default,disallowed-unique-constraint example.sql
```

### Configuration file

Rules can also be disabled by using a configuration file, which can be used via the `-c` or `--config` flag

```shell
squawk --config=~/.squawk.toml example.sql
```
The `--exclude` flag will always be prioritized over the configuration file

If the config flag it's not present or the file doesn't exist, squawk will try to look for a configuration file automatically at these locations (ordered by priority):

1.  `./.squawk.toml`
2. `$GIT_REPO_ROOT/.squawk.toml`
3. `~/.squawk.toml`

**Example `.squawk.toml`**
```toml
excluded_rules = ["require-concurrent-index-creation", "require-concurrent-index-deletion"]
```

See the [Squawk website](https://squawkhq.com/docs/rules) for documentation on each rule with examples and reasoning.

## Bot Setup

Squawk works as a CLI tool but can also create comments on GitHub Pull
Requests using the `upload-to-github` subcommand.

Here's an example comment created by `squawk` using the `example.sql` in the repo:

<https://github.com/sbdchd/squawk/pull/14#issuecomment-647009446>

See the ["GitHub Integration" docs](https://squawkhq.com/docs/github_app) for more information.

## `pre-commit` hook

Integrate Squawk into Git workflow with [pre-commit](https://pre-commit.com/). Add the following
to your project's `.pre-commit-config.yaml`:

```
repos:
  - repo: https://github.com/sbdchd/squawk
    rev: v0.10.0
    hooks:
     - id: squawk
       files: path/to/postres/migrations/written/in/sql
```

Note the `files` parameter as it specifies the location of the files to be linted.

## prior art

- <https://github.com/erik/squabble>

### related tools

- <https://github.com/yandex/zero-downtime-migrations>
- <https://github.com/tbicr/django-pg-zero-downtime-migrations>
- <https://github.com/3YOURMIND/django-migration-linter>
- <https://github.com/ankane/strong_migrations>

## related blog posts / SE Posts / PG Docs

- <https://www.braintreepayments.com/blog/safe-operations-for-high-volume-postgresql/>
- <https://gocardless.com/blog/zero-downtime-postgres-migrations-the-hard-parts/>
- <https://www.citusdata.com/blog/2018/02/22/seven-tips-for-dealing-with-postgres-locks/>
- <https://realpython.com/create-django-index-without-downtime/#non-atomic-migrations>
- <https://dba.stackexchange.com/questions/158499/postgres-how-is-set-not-null-more-efficient-than-check-constraint>
- <https://www.postgresql.org/docs/10/sql-altertable.html#SQL-ALTERTABLE-NOTES>
- <https://www.postgresql.org/docs/current/explicit-locking.html>
- <https://benchling.engineering/move-fast-and-migrate-things-how-we-automated-migrations-in-postgres-d60aba0fc3d4>
- <https://medium.com/paypal-tech/postgresql-at-scale-database-schema-changes-without-downtime-20d3749ed680>

## dev

```shell
cargo install
cargo run
./s/test
./s/lint
./s/fmt
```

### releasing a new version

1. update the CHANGELOG.md and bump version in the cli `Cargo.toml`, ensure the
   lock file is updated, and update `package.json` and commit the changes

   ```bash
   # update version in Cargo.toml files and package.json to 4.5.3
   s/update-version 4.5.3
   ```

2. create a new release on github - CI will attach the binaries automatically
3. wait for build artifacts to be attached to release.
4. login to `npm` and publish new version.

   ```bash
   npm login
   npm publish
   ```

## how it works

squawk wraps calls to [libpg_query-sys](https://github.com/tdbgamer/libpg_query-sys) in a safe
interface and parses the JSON into easier to work with structures.
libpg_query-sys in turn uses [bindgen](https://rust-lang.github.io/rust-bindgen/) to bind to
[libpg_query](https://github.com/lfittl/libpg_query), which itself wraps Postgres' SQL
parser in a bit of C code that outputs the parsed AST into a JSON string.

Squawk then runs the rule functions over the parsed AST, gathers and pretty
prints the rule violations.
