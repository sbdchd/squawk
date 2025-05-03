# squawk-parser

Postgresl SQL parser that wraps
[libpg_query-sys](https://github.com/tdbgamer/libpg_query-sys) in a safe
interface and parses the json from
[libpg_query](https://github.com/lfittl/libpg_query) into easier to work with
structures.

Used by `squawk-linter` for writing lint rules.
