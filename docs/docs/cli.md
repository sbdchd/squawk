---
id: cli
title: CLI
---

## Usage

```bash
# lint a file or multiple
squawk migration_001.sql migration_002.sql migration_003.sql

# lint from standard in
cat migration.sql | squawk
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

        --verbose
            Enable debug logging output


OPTIONS:
        --dump-ast <dump-ast>
            Output AST in JSON [possible values: Raw, Parsed, Debug]

    -e, --exclude <exclude>...
            Exclude specific warnings

            For example: --exclude=require-concurrent-index-creation,ban-drop-database
        --explain <explain>
            Provide documentation on the given rule

        --reporter <reporter>
            Style of error reporting [possible values: Tty, Gcc, Json]

        --stdin-filepath <stdin-filepath>
            Path to use in reporting for stdin


ARGS:
    <paths>...
            Paths to search


SUBCOMMANDS:
    help                Prints this message or the help of the given subcommand(s)
    upload-to-github    Comment on a PR with Squawk's results
```
