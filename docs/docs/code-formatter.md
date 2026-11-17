---
id: code-formatter
title: Code Formatter
---

Squawk supports formatting SQL via `squawk format`.

Try it out in the [Squawk Playground](https://play.squawkhq.com/?mode=format).

## Usage

Format in-place:

```bash
squawk format migration_001.sql 'migrations/*.sql'
```

Use `--check` to exit with a non-zero status code if any files aren't formatted:

```bash
squawk format --check 'migrations/*.sql'
```

`--diff` is similiar to `--check`, but it also prints the changes that would be made:

```bash
squawk format --diff 'migrations/*.sql'
```

## Style guide

Squawk format only changes whitespace, casing, and identifier quoting.

## Editor integration

The [VSCode extension](https://marketplace.visualstudio.com/items?itemName=sbdchd.squawk) supports code formatting powered by the Squawk Language Server.
