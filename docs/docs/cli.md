---
id: cli
title: CLI
---

## Usage

```bash
# lint a file or multiple
squawk migration_001.sql migration_002.sql migration_003.sql 'migrations/*.sql'

# lint from standard in
cat migration.sql | squawk
```

## Rules

Individual rules can be disabled via the `--exclude` flag

```shell
squawk --exclude=adding-field-with-default,disallowed-unique-constraint example.sql
```

Rules that are disabled by default can be enabled via the `--include` flag

```shell
squawk --include=require-table-schema example.sql
```

Note: `--exclude` takes precedence over `--include`.

### Disabling rules via comments

Rule violations can be ignored via the `squawk-ignore` comment:

```sql
-- squawk-ignore ban-drop-column
alter table t drop column c cascade;
```

You can also ignore multiple rules by making a comma seperated list:

```sql
-- squawk-ignore ban-drop-column, renaming-column,ban-drop-database
alter table t drop column c cascade;
```

To ignore a rule for the entire file, use `squawk-ignore-file`:

```sql
-- squawk-ignore-file ban-drop-column
alter table t drop column c cascade;
-- also ignored!
alter table t drop column d cascade;
```

Or leave off the rule names to ignore all rules for the file

```sql
-- squawk-ignore-file
alter table t drop column c cascade;
create table t (a int);
```

## Files

Files can be excluded from linting via the `--exclude-path` flag. Glob matching is supported and the flag can be provided multiple times.

```shell
squawk --exclude-path=005_user_ids.sql --exclude-path='*user_ids.sql' 'migrations/*.sql'
```

## `.squawk.toml` configuration file

Rules can be disabled with a configuration file.

By default, Squawk will traverse up from the current directory to find a `.squawk.toml` configuration file. You may specify a custom path with the `-c` or `--config` flag.

```shell
squawk --config=~/.squawk.toml example.sql
```

The `--exclude`, `--include`, `--exclude-path`, and `--pg-version` flags will always be prioritized over the configuration file.

## Example `.squawk.toml` configurations

### Excluding rules

```toml
# .squawk.toml
excluded_rules = [
    "require-concurrent-index-creation",
    "require-concurrent-index-deletion",
]
```

### Including rules

Rules that are disabled by default can be enabled via `included_rules`.

```toml
# .squawk.toml
included_rules = [
    "require-table-schema",
]
```

Note: `excluded_rules` takes precedence over `included_rules`.

### Specifying postgres version

```toml
# .squawk.toml
pg_version = "11.0"
```

### Specifying whether SQL files will be wrapped in a transaction

```toml
# .squawk.toml
assume_in_transaction = true
```

### Using all options

```toml
# .squawk.toml
pg_version = "11.0"
excluded_rules = [
    "require-concurrent-index-creation",
    "require-concurrent-index-deletion",
]
included_rules = [
    "require-table-schema",
]
assume_in_transaction = true
excluded_paths = [
    "005_user_ids.sql",
    "*user_ids.sql",
]
[upload_to_github]
fail_on_violations = true
```

See the [Squawk website](https://squawkhq.com/docs/rules) for documentation on each rule with examples and reasoning.

## `squawk --help`

```
Find problems in your SQL

Usage: squawk [OPTIONS] [path]... [COMMAND]

Commands:
  server            Run the language server
  upload-to-github  Comment on a PR with Squawk's results
  help              Print this message or the help of the given subcommand(s)

Arguments:
  [path]...
          Paths or patterns to search

Options:
      --exclude-path <EXCLUDED_PATH>
          Paths to exclude

          For example:

          `--exclude-path=005_user_ids.sql --exclude-path=009_account_emails.sql`

          `--exclude-path='*user_ids.sql'`

  -e, --exclude <rule>
          Exclude specific warnings

          For example: --exclude=require-concurrent-index-creation,ban-drop-database

  -i, --include <rule>
          Include opt-in rules that are disabled by default

          Rules listed in --exclude take precedence over --include.

          For example: --include=require-table-schema

      --pg-version <PG_VERSION>
          Specify postgres version

          For example: --pg-version=13.0

      --debug <format>
          Output debug format

          [possible values: lex, parse, ast]

      --reporter <REPORTER>
          Style of error reporting

          [possible values: tty, gcc, json, gitlab]

      --stdin-filepath <filepath>
          Path to use in reporting for stdin

      --verbose
          Enable debug logging output

  -c, --config <CONFIG_PATH>
          Path to the squawk config file (.squawk.toml)

      --assume-in-transaction
          Assume that a transaction will wrap each SQL file when run by a migration tool

          Use --no-assume-in-transaction to override any config file that sets this

      --no-error-on-unmatched-pattern
          Do not exit with an error when provided path patterns do not match any files

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
