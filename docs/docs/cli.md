---
id: cli
title: CLI
---

## usage

```bash
# lint a file or multiple
squawk migration_001.sql migration_002.sql migration_003.sql 'migrations/*.sql'

# lint from standard in
cat migration.sql | squawk
```

## rules

Individual rules can be disabled via the `--exclude` flag

```shell
squawk --exclude=adding-field-with-default,disallowed-unique-constraint example.sql
```

## files

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

The `--exclude`, `--exclude-path`, and `--pg-version` flags will always be prioritized over the configuration file.


## example `.squawk.toml` configurations

### excluding rules

```toml
# .squawk.toml
excluded_rules = [
    "require-concurrent-index-creation",
    "require-concurrent-index-deletion",
]
```

### specifying postgres version

```toml
# .squawk.toml
pg_version = "11.0"
```

### specifying whether SQL files will be wrapped in a transaction

```toml
# .squawk.toml
assume_in_transaction = true
```

### using all options

```toml
# .squawk.toml
pg_version = "11.0"
excluded_rules = [
    "require-concurrent-index-creation",
    "require-concurrent-index-deletion",
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
squawk
Find problems in your SQL

USAGE:
    squawk [FLAGS] [OPTIONS] [path]... [SUBCOMMAND]

FLAGS:
        --assume-in-transaction
            Assume that a transaction will wrap each SQL file when run by a migration tool

            Use --no-assume-in-transaction to override any config file that sets this
    -h, --help
            Prints help information

        --list-rules
            List all available rules

    -V, --version
            Prints version information

        --verbose
            Enable debug logging output


OPTIONS:
    -c, --config <config-path>
            Path to the squawk config file (.squawk.toml)

        --dump-ast <ast-format>
            Output AST in JSON [possible values: Raw, Parsed, Debug]

    -e, --exclude <rule>...
            Exclude specific warnings

            For example: --exclude=require-concurrent-index-creation,ban-drop-database
        --explain <rule>
            Provide documentation on the given rule

        --pg-version <pg-version>
            Specify postgres version

            For example: --pg-version=13.0
        --reporter <reporter>
            Style of error reporting [possible values: Tty, Gcc, Json]

        --stdin-filepath <filepath>
            Path to use in reporting for stdin


ARGS:
    <path>...
            Paths to search


SUBCOMMANDS:
    help                Prints this message or the help of the given subcommand(s)
    upload-to-github    Comment on a PR with Squawk's results
```
