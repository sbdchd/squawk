# squawk [![cargo-badge](https://img.shields.io/crates/v/squawk.svg)](https://crates.io/crates/squawk) ![Rust CI](https://github.com/sbdchd/squawk/workflows/Rust%20CI/badge.svg)

> linter for Postgres migrations

## Why?

Prevent unexpected downtime caused by database migrations.

Also it seemed like a nice project to spend more time with Rust.

## Install

Note: due to `squawk`'s dependency on
[`libpg_query`](https://github.com/lfittl/libpg_query/issues/44), `squawk`
only supports Linux and macOS

```shell
npm install -g squawk-cli

cargo install squawk

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

### `require-concurrent-index-creation`

Ensure all index creations use the `CONCURRENTLY` option.

This rule ignores indexes added to tables created in the same transaction.

During a normal index creation updates are blocked. `CONCURRENTLY` avoids the
issue of blocking.

<https://www.postgresql.org/docs/current/sql-createindex.html#SQL-CREATEINDEX-CONCURRENTLY>

### `constraint-missing-not-valid`

Check that all new constraints have `NOT VALID`.

By default new constraints require a table scan and block writes to the
table. Using `NOT VALID` with a later `VALIDATE CONSTRAINT` call prevents the
table scan and results in the validation step only requiring a `SHARE UPDATE EXCLUSIVE` lock.

<https://www.postgresql.org/docs/current/sql-altertable.html#SQL-ALTERTABLE-NOTES>

### `adding-field-with-default`

On Postgres versions less than 11, adding a field with a `DEFAULT` requires a
table rewrite with an `ACCESS EXCLUSIVE` lock.

<https://www.postgresql.org/docs/10/sql-altertable.html#SQL-ALTERTABLE-NOTES>

### `changing-column-type`

Changing a column type requires an `ACCESS EXCLUSIVE` lock on the table which blocks reads.

Changing the type of the column may also break other clients reading from the
table.

<https://www.postgresql.org/docs/current/sql-altertable.html#SQL-ALTERTABLE-NOTES>

### `adding-not-nullable-field`

A `NOT NULL` constraint requires a table scan and the `ALTER TABLE` requires
an `ACCESS EXCLUSIVE` lock.

Usually this is paired with a `DEFAULT` which has issues on version less than
\11. See the `adding-field-with-default` rule.

### `renaming-column`

Renaming a column may break existing clients.

### `renaming-table`

Renaming a table may break existing clients.

### `disallowed-unique-constraint`

Adding a `UNIQUE` constraint requires an `ACCESS EXCLUSIVE` lock which blocks reads.

Instead create an index `CONCURRENTLY` and create the `CONSTRAINT` `USING` the index.

<https://www.postgresql.org/docs/current/sql-altertable.html>

### `ban-drop-database`

Dropping a database may break existing clients.

### `prefer-text-field`

Changing the size of a `varchar` field requires an `ACCESS EXCLUSIVE` lock.

Using a text field with a `CHECK CONSTRAINT` makes it easier to change the
max length. See the `constraint-missing-not-valid` rule.

### `prefer-robust-stmts`

Goal of this rule is to make migrations more robust when they fail part way through.

For instance, you may have a migration with two steps. First, the migration
adds a field to a table, then it creates an index concurrently.

Since this second part is concurrent, it can't run in a transaction so the
first part of the migration can succeed, and second part can fail meaning the
first part won't be rolled back.

Then when the migration is run again, it will fail at adding the field since
it already exists.

To appease this rule you can use guards like `IF NOT EXISTS` or wrap all your
statements in a transaction.

## Bot Setup

Squawk works as a CLI tool but can also create comments on GitHub Pull
Requests using the `upload-to-github` subcommand.

Here's an example comment created by `squawk` using the `example.sql` in the repo:

<https://github.com/sbdchd/squawk/pull/14#issuecomment-647009446>

### Create a new app

Squawk needs a corresponding GitHub App so it can talk to GitHub.

1. Create the app

   - head over to <https://github.com/settings/apps/new>

   - add an app name & homepage url

   - Uncheck the `active` checkbox under Webhook

   - add permissions

   | name          | kind  | why               |
   | ------------- | ----- | ----------------- |
   | Pull Requests | Write | to comment on PRs |

   hit create and copy the `App ID` under the "About" section

   url should be: https://github.com/settings/apps/$YOUR_APP_NAME

2. Head down the the bottom of the page under the "Private Keys" section and
   hit "Generate a private key"

   The key should automatically download after a couple seconds. Hold onto this key, we'll need it later.

   Now we have an `App ID` and a `Private Key`, now we need to install the app

3. Install the app & get the Install ID

   Head to <https://github.com/settings/apps/$YOUR_APP_NAME/installations> and hit "Install"

   GitHub should have redirected you to the <https://github.com/settings/installations/$INSTALL_ID> page where `$INSTALL_ID` is some number.

   Save this ID for later.

   Now we have our `SQUAWK_GITHUB_APP_ID`, `SQUAWK_GITHUB_PRIVATE_KEY`,
   `SQUAWK_GITHUB_INSTALL_ID`.

   Squawk needs the pull request related values: `SQUAWK_GITHUB_REPO_NAME`,
   `SQUAWK_GITHUB_REPO_OWNER`, and `SQUAWK_GITHUB_PR_NUMBER`.

   Where to find these varies depending how you're running squawk, but for the
   next step I'm assuming you're running Squawk as a CircleCI job.

4. Finding the Pull Request variables

   ### CircleCI

   <https://circleci.com/docs/2.0/env-vars/#built-in-environment-variables>

   `CIRCLE_PULL_REQUEST` has the content we need

   example: `https://github.com/recipeyak/recipeyak/pull/567`

   Now we need to split this to get the repo name, repo owner, and pull
   requeset id.

   With a bit of help from

   ```sh
   echo "https://github.com/recipeyak/recipeyak/pull/567" | awk -F/ '{print $4 " " $5 " " $7}'

   recipeyak recipeyak 567
   ```

   ```sh
   SQUAWK_GITHUB_REPO_OWNER=$(echo $CIRCLE_PULL_REQUEST | awk -F/ '{print $4}')
   SQUAWK_GITHUB_REPO_NAME=$(echo $CIRCLE_PULL_REQUEST | awk -F/ '{print $5}')
   SQUAWK_GITHUB_PR_NUMBER=$(echo $CIRCLE_PULL_REQUEST | awk -F/ '{print $7}')
   ```

5. Conclusion

   Wrapping it all up we should have the following env vars:

   ```sh
   SQUAWK_GITHUB_APP_ID= # fill in with id found in step 5
   SQUAWK_GITHUB_INSTALL_ID= # fill in with id found in step 7
   # downloaded via step 6, your key will have a different name
   SQUAWK_GITHUB_PRIVATE_KEY=$(cat ./cool-bot-name.private-key.pem)
   # can also use the SQUAWK_GITHUB_PRIVATE_KEY_BASE64 instead ^
   SQUAWK_GITHUB_REPO_OWNER=$(echo $CIRCLE_PULL_REQUEST | awk -F/ '{print $4}')
   SQUAWK_GITHUB_REPO_NAME=$(echo $CIRCLE_PULL_REQUEST | awk -F/ '{print $5}')
   SQUAWK_GITHUB_PR_NUMBER=$(echo $CIRCLE_PULL_REQUEST | awk -F/ '{print $7}')
   ```

   We can pass this into the env before running squawk or we can translate
   them to the command line flag. What's ever easiest for you.

   An example run will look like the following (assuming the env vars are set):

   ```sh
   squawk upload-to-github example.sql
   ```

   which creates a comment like the following:

   <https://github.com/sbdchd/squawk/pull/14#issuecomment-647009446>

## prior art

- <https://github.com/erik/squabble>

### related tools

- <https://github.com/yandex/zero-downtime-migrations>
- <https://github.com/tbicr/django-pg-zero-downtime-migrations>
- <https://github.com/3YOURMIND/django-migration-linter>

## related blog posts / SE Posts / PG Docs

- <https://www.braintreepayments.com/blog/safe-operations-for-high-volume-postgresql/>
- <https://gocardless.com/blog/zero-downtime-postgres-migrations-the-hard-parts/>
- <https://www.citusdata.com/blog/2018/02/22/seven-tips-for-dealing-with-postgres-locks/>
- <https://realpython.com/create-django-index-without-downtime/#non-atomic-migrations>
- <https://dba.stackexchange.com/questions/158499/postgres-how-is-set-not-null-more-efficient-than-check-constraint>
- <https://www.postgresql.org/docs/10/sql-altertable.html#SQL-ALTERTABLE-NOTES>
- <https://www.postgresql.org/docs/current/explicit-locking.html>
- <https://benchling.engineering/move-fast-and-migrate-things-how-we-automated-migrations-in-postgres-d60aba0fc3d4>

## dev

```shell
cargo install
cargo run
./s/test
./s/lint
./s/fmt
```

### releasing a new version

1. update the CHANGELOG.md
2. bump version in all the dependency `Cargo.toml` as well as the CLI `Cargo.toml`
3. create a new release on github - CI will attach the binaries automatically
4. bump version in `package.json` and follow the `npm` steps
5. publish each crate to cargo in a DAG fashion

## how it works

squawk wraps calls to [libpg_query-sys](https://github.com/tdbgamer/libpg_query-sys) in a safe
interface and parses the JSON into eaiser to work with structures.
libpg_query-sys in turn uses [bindgen](https://rust-lang.github.io/rust-bindgen/) to bind to
[libpg_query](https://github.com/lfittl/libpg_query), which itself wraps Postgres' SQL
parser in a bit of C code that outputs the parsed AST into a JSON string.

Squawk then runs the rule functions over the parsed AST, gathers and pretty
prints the rule violations.
