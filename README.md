# squawk [![cargo-badge](https://img.shields.io/crates/v/squawk.svg)](https://crates.io/crates/squawk)

> linter for Postgres migrations

## Why?

Prevent unexpected downtime caused by database migrations.

Also it seemed like a nice project to spend more time with Rust.

## Install

```shell
cargo install squawk
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

### `squawk --help`

```
squawk 0.1.0
Find problems in your SQL

USAGE:
    squawk [FLAGS] [OPTIONS] [--] [paths]...

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
```

## Rules

### foo

## prior art

- <https://github.com/erik/squabble>

### related tools

- <https://github.com/yandex/zero-downtime-migrations>
- <https://github.com/tbicr/django-pg-zero-downtime-migrations>
- <https://github.com/3YOURMIND/django-migration-linter>

## related blog posts / SE Posts / PG Docs

- <https://www.braintreepayments.com/blog/safe-operations-for-high-volume-postgresql/>
- <https://gocardless.com/blog/zero-downtime-postgres-migrations-the-hard-parts/>
- <https://realpython.com/create-django-index-without-downtime/#non-atomic-migrations>
- <https://dba.stackexchange.com/questions/158499/postgres-how-is-set-not-null-more-efficient-than-check-constraint>
- <https://www.postgresql.org/docs/10/sql-altertable.html#SQL-ALTERTABLE-NOTES>
- <https://www.postgresql.org/docs/current/explicit-locking.html>

## dev

```shell
cargo install
cargo test
cargo run
cargo clippy -- -W clippy::nursery
cargo fmt
```

## how it works

squawk wraps calls to [libpg_query-sys](https://github.com/tdbgamer/libpg_query-sys) in a safe
interface and parses the JSON into eaiser to work with structures.
libpg_query-sys in turn uses [bindgen](https://rust-lang.github.io/rust-bindgen/) to bind to
[libpg_query](https://github.com/lfittl/libpg_query), which itself wraps Postgres' SQL
parser in a bit of C code that outputs the parsed AST into a JSON string.

Squawk then runs the rule functions over the parsed AST, gathers and pretty
prints the rule violations.
