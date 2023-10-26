---
id: troubleshooting
title: Troubleshooting
---

If you encounter an issue using Squawk that can't be resolved with these docs, please [open an issue](https://github.com/sbdchd/squawk/issues/new).


## common error messages

### postgres failed to parse query

Squawk was unable to parse the query using the PostgreSQL parser.

Usually the provided statement contains an error and isn't a valid Postgres statement.

Squawk uses the [libpg_query-sys](https://github.com/tdbgamer/libpg_query-sys) Rust bindings to [libpg_query](https://github.com/pganalyze/libpg_query) to parse Postgres queries using the Postgres parser.
