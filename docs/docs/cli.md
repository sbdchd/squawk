---
id: cli
title: CLI
---

## usage

```bash
# lint a file or multiple
squawk migration_001.sql migration_002.sql migration_003.sql

# lint from standard in
cat migration.sql | squawk
```

### rules

Individual rules can be disabled via the `--exclude` flag

```shell
squawk --exclude=adding-field-with-default,disallowed-unique-constraint example.sql
```

### `.squawk.toml` configuration file

Rules can be disabled with a configuration file.

By default, Squawk will traverse up from the current directory to find a `.squawk.toml` configuration file. You may specify a custom path with the `-c` or `--config` flag.

```shell
squawk --config=~/.squawk.toml example.sql
```

The `--exclude` flag will always be prioritized over the configuration file.

**Example `.squawk.toml`**

```toml
excluded_rules = [
    "require-concurrent-index-creation",
    "require-concurrent-index-deletion",
]
```

See the [Squawk website](https://squawkhq.com/docs/rules) for documentation on each rule with examples and reasoning.


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
    -c, --config <config-path>               
            Path to the squawk config file (.squawk.toml)

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
