---
id: troubleshooting
title: troubleshooting
---

If you encounter an issue using Squawk that can't be resolved with these docs, please [open an issue](https://github.com/sbdchd/squawk/issues/new).

## Common error messages

### Postgres failed to parse query

Squawk was unable to parse the query using the PostgreSQL parser.

Usually the provided statement contains an error and isn't a valid Postgres statement.

Squawk uses the [libpg_query-sys](https://github.com/tdbgamer/libpg_query-sys) Rust bindings to [libpg_query](https://github.com/pganalyze/libpg_query) 10-1.0.2 to parse Postgres queries using the Postgres parser.
