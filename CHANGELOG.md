# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## v2.21.1 - 2025-07-23

## Fixed

- parser: bugged warning for invalid casts that was warning about all casts inside function calls. (#598)

## v2.21.0 - 2025-07-23

## Added

- parser: warn about `try_cast` and don't fail to parse casts that are invalid. (#593)

## Fixed

- vscode: invalid binary path on windows causing extension not to load. (#595)

## v2.20.0 - 2025-07-12

## Added

- GitHub annotations when run with `GITHUB_ACTIONS` set. (#589), (#590)

  This can be disabled by setting `SQUAWK_DISABLE_GITHUB_ANNOTATIONS`.

- Docs for `squawk-ignore-file`. (#588)

- Publishing to open vsx in CI. (#587)

## v2.19.0 - 2025-07-09

## Added

- linter: file level rule violation ignore comments. (#585)

  Now you can ignore all violations in a file via:

  ```sql
  -- squawk-ignore-file
  ```

  or ignore a specific rule for the file:

  ```sql
  -- squawk-ignore-file prefer-robust-stmts
  ```

  This works with the existing line level ignores:

  ```sql
  create table t (id int);
  -- squawk-ignore prefer-robust-stmts
  create table u (id int);
  ```

- vscode: incremental sync. (#583)

  No more debouncing of file updates making squawk in vscode snappier!

- vscode: tracing setting `squawk.trace.server` & output channel. (#584), (#585)

## v2.18.0 - 2025-07-03

## Added

- vscode: `Show tokens` command to get lexer output. (#579)
- vscode: `Show server` & `Stop server` commands. (#576)
- vscode: `show client logs` & `show syntax tree` commands. (#575)

## v2.17.0 - 2025-06-30

## Added

- Basic VSCode extension. (#567), (#569), (#571), (#572)

- Improved `json_table` parsing. (#564)

## v2.16.0 - 2025-06-27

### Added

- Parsing non-standard placeholders with format `:name`. (#560), (#561)
- Error recovery for `drop table` when there's extra commas or missing commas. (#556)
- Improved `order by` parsing. (#556)
- Error recovery for array exprs & improvements for constraints. (#557)
- Improved CTE parsing. (#558)
- Error recovery for type args & more forgiving create table args parsing. (#559)

## Changed

- Internal: bumped rust to 1.88.0

## v2.15.0 - 2025-06-21

### Added

- validation for missing types in `create table` args. (#550)

  The following now parses with an error:

  ```sql
  create table t (
    x int,
    description
  );
  ```

  ```
  error[syntax-error]: Missing column type
   --> stdin:3:14
    |
  3 |   description
    |              ^
    |
  ```

- Make `alter table` actions robust to missing commas. (#549)

  The following now parses with an error:

  ```sql
  alter table t
    validate constraint foo
    validate constraint b;
  ```

  ```
  error[syntax-error]: missing comma
   --> stdin:2:26
    |
  2 |   validate constraint foo
    |                          ^
    |
  ```

### Fixed

- Crash with trailing comma in select target list. (#551)

  The following now parses with an error:

  ```sql
  select a, from t;
  ```

  ```
  error[syntax-error]: unexpected trailing comma
  --> stdin:1:9
  |
  1 | select a, from t;
  |         ^
  |
  ```

- Parsing idents with `uescape`. (#533)

  The following now parses:

  ```sql
  select U&"d!0061t!+000061" UESCAPE '!';
  ```

## v2.14.0 - 2025-06-17

### Added

- The npm install script now checks an env var (`SQUAWK_LOCAL_CDNURL`), that
  defaults to `https://github.com/sbdchd/squawk/releases/download`, for
  downloading binaries. This should help when you want to use a cache or in case
  GitHub is down.

### Fixed

- The last of the pg regression suite errors. (#543), (#542)

- Precendence when parsing compound select statements. (#544)

  ```sql
  SELECT foo UNION SELECT bar ORDER BY baz;
  -- equal to:
  (SELECT foo UNION SELECT bar) ORDER BY baz;
  ```

## v2.13.0 - 2025-06-15

### Fixed

- parsing compound select statements & their trailing clauses, i.e. (#539)

  ```sql
  (select 1) limit 1;

  select * from (
    (select 1)
    union all
    (select 1)
  );
  ```

- join parsing to be more error resilent: (#538)

  ```
  error[syntax-error]: Join missing condition.
   --> stdin:2:23
    |
  2 | select * from t join u;
    |                       ^
    |
  error[syntax-error]: Join `using` clause is not allowed for cross joins.
    --> stdin:16:30
     |
  16 | select * from t cross join u using (id);
     |                              ^^^^^^^^^^
     |
  error[syntax-error]: Join condition is not allowed for cross joins.
    --> stdin:18:30
     |
  18 | select * from t cross join u on true;
     |                              ^^^^^^^
     |
  ```

## v2.12.0 - 2025-06-09

### Changed

- Relicensed: gpl3 -> apache or mit (#534)

### Fixed

- `json_table`, `table`, `values`, `group by` (#536)
- Cast expression issues (#533)
- Float lexing (#532)
- Unicode escaped quoted idents (#531)
- `xmltable` (#530)
- Many parser fixes for the pg regression test suite (#529), (#527), (#526), (#524), (#523), (#522)

## v2.11.0 - 2025-05-30

### Fixed

- Fix regression in linter rule `adding-not-nullable-field` from v2 rewrite. It wasn't erroring when it should have. (#520)

- Fix parser panic related to `select json_array(select from t);` (#515)

- Fix more parser errors in PG regression suite. `notnull` now parses! (#516)

- Fix more parser errors in PG regression suite part 2. Notably `ilike` is now supported. (#518)

## v2.10.0 - 2025-05-27

### Fixed

- Fix typo in rule name parsing causing `unused-ignore` to warn for `prefer-timestamp-tz`. (#511)

- Fix parsing window definition. (#510)

  Previously the following would error:

  ```sql
  WITH ranked_notifications AS (
    SELECT
      notification_id,
      ROW_NUMBER() OVER (
        PARTITION BY user_id, board_id ORDER BY created_at DESC
      ) AS rn
    FROM public.notification
    WHERE android_channel_id = 'watchlist'
  )
  UPDATE public.notification
  SET dismissed_at = current_timestamp
  WHERE notification_id IN (
    SELECT notification_id FROM ranked_notifications WHERE rn > 1
  );
  ```

## v2.9.0 - 2025-05-25

### Added

- Added back basic dump ast support via `squawk --debug=ast`. (#505)

## v2.8.0 - 2025-05-25

### Fixed

- Fix parsing `select select`. (#499)

  Previously, the following would panic:

  ```sql
  select select
  ```

- Fix displaying warnings as errors. (#502)

  We were highlighting warnings using the error syntax (red `^` instead of yellow `-`). We also were rendering syntax errors as warnings instead of errors!

- Fix parsing aggregate args with modifiers like `distinct` or `all`. (#498) Thanks @psteinroe!

  Previously the following would error:

  ```sql
  select string_agg(distinct c, ',')
    filter (where length(c) > 1)
    from t;
  ```

- Fix parsing data sources within params. (#497) Thanks @psteinroe!

  Previously the following would error:

  ```sql
  select f1, count(*) from
    t1 x(x0,x1) left join (t1 left join t2 using(f1)) on (x0 = 0)
  ```

- Fix parsing alternative boolean keywords. (#493) Thanks @psteinroe!

  Previously the following would error:

  ```sql
  explain (costs off) select;
  ```

## Changed

- Renamed `ast::Item` to `ast::Stmt`. (#483)
- Split `select` into `select`, `tables`, and `values` statements. (#484)

## v2.7.0 - 2025-05-14

### Fixed

- Fix parsing table constraint foreign key actions. (#480)

  Previously, the following would error:

  ```sql
  alter table foo
  add constraint foo_bar_id_fkey foreign key (bar_id)
  references bar (id) on update cascade on delete cascade;
  ```

## v2.6.0 - 2025-05-14

### Fixed

- Fix the GitHub commenting overwriting comments made by other bots sharing the same name, i.e., the default GitHub Action bot. (#477) Thanks @danxmoran!

## v2.5.0 - 2025-05-14

### Fixed

- Fixed parsing `create or replace view` (#474).

  Previously, the following would error:

  ```sql
  create or replace view my_view as
    select x from foo;
  ```

## v2.4.0 - 2025-05-13

### Fixed

- Fixed parsing `create view` with nested parens (#468).

## v2.3.0 - 2025-05-13

### Fixed

- Parsing `grant` and `revoke` statements with schema specified names, aka the following parses correctly: (#469)

  ```sql
  GRANT ALL ON SEQUENCE public.s TO u;
  ```

## v2.2.0 - 2025-05-12

### Added

- Style guide for linter error messages

### Fixed

- Error messages for `prefer-robust-statements` to be less confusing. We were saying to add `if not exists` every time, even when the statement didn't have that option.

- Fixed parsing nested compound select that has parens (#464).
  The following now parses:

  ```sql
  SELECT (
    (SELECT id FROM code_categories WHERE "language" = @language::char(4) ORDER BY "id" ASC LIMIT 1)
    UNION
    (SELECT id FROM code_categories WHERE "language" = 'nl-NL' ORDER BY "id" ASC LIMIT 1)
  ) LIMIT 1;
  ```

## v2.1.0 - 2025-05-08

### Added

- `ban-truncate-cascade` rule (#453)

## v2.0.0 - 2025-05-07

### Added

- [Ignore comments to disable a rule inline](https://squawkhq.com/docs/cli#disabling-rules-via-comments).
- Fancier lint violation formatting in the terminal using [annotate-snippets](https://github.com/rust-lang/annotate-snippets-rs).
- [WASM powered playground](https://play.squawkhq.com) to run Squawk locally in the browser.
- [New parser with error recovery](https://github.com/sbdchd/squawk/blob/master/crates/squawk_parser/src/grammar.rs) -- it doesn't fail if you're missing a comma, semicolon, etc! This also means we've removed the dependency on libpg_query.
- Dockerfile to run Squawk in a container (#422). Thanks @alphayax!

### Removed

- `prefer-big-int` rule, which has been deprecated in favor of `prefer-bigint-over-int` and `prefer-bigint-over-smallint` for a while now.

## v1.6.1 - 2025-05-02

### Fixed

- Fixed panic when formatting violations (#426).

## v1.6.0 - 2025-04-02

### Added

- Added `ban-alter-domain-with-add-constraint` and `ban-create-domain-with-constraint` (#418). Thanks @johnmastro!

## v1.5.5 - 2025-03-20

### Fixed

- Fixed adding-not-nullable-field incorrectly failing to check for nulls (#412). Thanks @schew2381!

## v1.5.4 - 2025-01-14

### Fixed

- Fixed building macOS Python wheels for x86 (#406). Thanks @ermakov-oleg!

## v1.5.3 - 2025-01-12

### Fixed

- Bump package version to set latest (#404).

## v1.5.2 - 2025-01-11

### Fixed

- Fixed accidental glibc upgrade (#401).

## v1.5.1 - 2025-01-10

### Fixed

- Fixed support for custom LIBPG_QUERY_PATH (#399).

## v1.5.0 - 2025-01-09

### Added

- Added warning to `adding-field-with-default` to warn about generated columns (#397).

### Fixed

- Fixed bug in `adding-required-field` where generated columns were incorrectly flagged (#397).

## v1.4.1 - 2024-10-10

### Fixed

- fix `start transaction` support (#386). Thanks @bmbferreira!

## v1.4.0 - 2024-09-24

### Added

- added support for linux arm64 builds (#382).

## v1.3.0 - 2024-09-21

### Added

- Add file violation count to summary report output (#378). Thanks @ermakov-oleg!

## v1.2.0 - 2024-09-08

### Added

- added support windows builds (#368). Thanks @ermakov-oleg!
- improved docs for "disallowed-unique-constraint" (#370). Thanks @teeyingyap!

### Fixed

- fixed "prefer-robust-stmts" to work with RLS (#367)

## v1.1.2 - 2024-06-26

### Fixed

- fix install for darwin-x86 (#361)

## v1.1.1 - 2024-06-16

### Fixed

- fix build for macos arm64 (#356)

## v1.1.0 - 2024-06-13

### Changed

- support configuration for "fail_on_violations" via `upload_to_github.fail_on_violations` (#353)
- support root flags within upload-to-github subcommand (#353)

## v1.0.0 - 2024-06-11

### Changed

- provided paths now support glob matching. `squawk 'migrations/*.sql'` (#352)

## v0.29.0 - 2024-05-30

### Added

- added `--excluded-paths` flag and `excluded_paths = ["example.sql"]` configuration option to ignore paths when searching for files (#350). Thanks @rdaniels6813!

## v0.28.0 - 2024-01-12

### Changed

- add exceptions for `ban-concurrent-index-creation-in-transaction` to handle golang-migrate. Thanks @janrueth! (#339)
- improve `disallowed-unique-constraint` to handle `alter table...add column... unique`. (#337)

## v0.27.0 - 2024-01-11

### Added

- added `ban-concurrent-index-creation-in-transaction` rule. Thanks @alixlahuec! (#335)

## v0.26.0 - 2023-12-12

### Changed

- `squawk upload-to-github` will always leave a pull request comment if files are evaluated. Previously if violations were resolved, stale warnings would be left in a comment. (#330)

## v0.25.0 - 2023-12-09

### Added

- added `squawk upload-to-github --fail-on-violations` flag to exit with error if violations are found (#327). Thanks @wmartins!

## v0.24.2 - 2023-11-07

### Fixed

- support parsing `alter database refresh collation` statements (#324)

## v0.24.1 - 2023-10-24

### Fixed

- support parsing `alter table set` statements (#321)

## v0.24.0 - 2023-04-11

### Added

- added `transaction-nesting` rule. Thanks @andrewsmith! (#303)
- added `adding-required-field` rule from `adding-not-nullable-field`. Thanks @andrewsmith! (#301)

### Changed

- functionality in `adding-not-nullable-field` for adding non-nullable fields was moved into `adding-required-field`. Thanks @andrewsmith! (#301)

## v0.23.0 - 2023-03-30

### Changed

- Only read from stdin when file paths are not provided. (#295)

## v0.22.0 - 2023-03-25

### Added

- added `ban-drop-table` rule. Thanks @borisrozumnuk! (#286)
- added `not-null-constraint` rule. Thanks @andrewsmith! (#288)

### Fixed

- Fixed building Squawk on platforms where `c_char` is unsigned. Thanks @ods! (#285)
- Fixed Squawk compatiblity with Nix. Thanks @andrewsmith! (#287)
- Fixed regression in parsing union queries. Fixed parsing call statement. (#293)

### Changed

- Upgrade libpg_query from 13 to 15. Thanks @andrewsmith! (#291)

## v0.21.0 - 2023-02-14

### Fixed

- Better support non-volatile defaults for `adding-field-with-default`. (#278)
- Static link for openssl and glibc. (#283)

## v0.20.0 - 2023-01-31

### Added

- added `prefer-bigint-over-int` and `prefer-bigint-over-smallint` to replace `prefer-big-int`. Thanks @aldenquimby! (#273)

### Fixed

- Support TableLikeClause in table creation. Thanks @qoelet! (#271)
- Recognize most DROP statements within a transaction as robust. Thanks @andrewsmith! (#276)

## v0.19.0 - 2023-01-24

### Added

- added `--assume-in-transaction` flag and configuration option to indicate that each SQL file is wrapped in a transaction by an external tool. Thanks @andrewsmith! (#264)

## v0.18.0 - 2023-01-06

## Fixed

- error parsing multiple transactions in one file (#259)

## v0.17.0 - 2022-07-27

## Added

- added better error handling of invalid syntax. (#245)

## v0.16.0 - 2022-07-22

## Added

- added `prefer-big-int` rule to prefer 64 bit types over 32 bit types. (#238)
- added `prefer-identity` rule to prefer `identity` columns over `serial` columns. (#243)

## v0.15.0 - 2022-07-17

## Added

- added `prefer-timestamptz` rule to warn about using `timestamp` instead of `timestamptz`. (#230)

## Changed

- only apply `prefer-robust-stmts` to files with more than one SQL statement. (#231)

## Fixed

- catch another way to add foreign keys for `adding-foreign-key-constraint`. Thanks @adamrdavid! (#228)

## v0.14.0 - 2022-07-10

## Added

- added `pg_version` configuration option to support ignoring rules by Postgres version. Thanks @adamrdavid! (#219)

## v0.13.2 - 2022-06-16

## Changed

- internal: use rule IDs based on serde annotations of `RuleViolationKind`. (#218)

## v0.13.1 - 2022-06-15

## Fixed

- fix parsing enum values from configuration file. (#217)

## v0.13.0 - 2022-06-06

## Added

- added validation for configured rules. (#216)

### Changed

- Change help menu value names. (#216)

## v0.12.0 - 2022-05-27

## Added

- added configuration file (`.squawk.toml`) to specify excluded rules. (#213)

## v0.11.3 - 2022-05-11

## Fixed

- incorrectly silenced stdin

## v0.11.2 - 2022-05-11

## Fixed

- silence stdin if it's empty (#210)

## v0.11.1 - 2022-05-11

## Fixed

- duplicate messages being written when using GitHub Actions token authentication. (#209)

## v0.11.0 - 2022-05-11

## Added

- Support GitHub API authentication via GitHub Actions tokens. (#207)

## v0.10.0 - 2022-04-25

## Changed

- differentiate between `VOLATILE` and non-`VOLATILE` defaults in `adding-field-with-default` documentation (#201)

## v0.9.0 - 2022-02-26

## Added

- require-concurrent-index-deletion rule and removed deletion warnings from
  require-concurrency-index-creation. Thanks @K-Phoen! (#177)

## v0.8.2 - 2022-01-07

### Fixed

- errors parsing PG12 and later query syntax by upgrading to PG13 parser (#174)
- parsing alter constraint statement (#173)

## v0.8.1 - 2021-11-30

### Fixed

- copy pasta of rename-table into the ban-drop-column rule (#168)
- parsing an index without a name specified (#169)

## v0.8.0 - 2021-10-31

### Fixed

- false positives with disallowed-unique-constraint and adding-serial-primary-key-field. Thanks @qoelet! (#161)

## v0.7.3 - 2021-08-11

### Fixed

- false positives with require-concurrent-index rule (#157)

## v0.7.2 - 2021-08-02

### Fixed

- incorrect internal schema for "alter table" statements with function calls. Thanks @qoelet! (#154)

## v0.7.1 - 2021-05-30

### Fixed

- incorrect internal schema for "create partition" statements. (#146)
- `upload-to-github` command not obeying top level `--exclude`s. (#142)

### Changed

- allowing adding not null column with default for `adding-not-null-field`. (#144)

## v0.7.0 - 2021-05-19

### Added

- link to website for tty reporter when there are lint errors. (#120)
- `DROP INDEX` support for "require-concurrent-index-creation" and "prefer-robust-stmts". (#124)
- github comment now includes Squawk's version number. (#131)
- new `ban-drop-column` rule (#132)

### Changed

- updated "adding-not-null-field" to warn about making a column non-nullable with `NOT NULL`. See the ["adding-not-null-field" docs](https://squawkhq.com/docs/adding-not-nullable-field) for more information. (#101)

### Fixed

- false positive with `prefer-text-field` that wasn't allowing `varchar`
  without a length specified

## v0.6.0 - 2021-02-19

### Added

- added "debug" option for `--dump-ast` to print out tree using Rust's `Debug` formatter (#92)
- added new rule, "adding-foreign-key-constraint", to provide suggestions for safely adding foreign key constraints (#91)

### Changed

- updated "robust-stmts" suggestions with caveat about `IF NOT EXISTS` (#95)
- updated "constraint-missing-not-valid" to warn about adding a constraint as NOT VALID and then using "VALIDATE CONSTRAINT" in the same transaction (#97)
- updated "prefer-robust-stmts" with exception for using `DROP CONSTRAINT IF EXISTS` before `ADD CONSTRAINT` (#99)

### Fixed

- error reporting is more user friendly (#89, #90)

## v0.5.4 - 2021-01-08

### Fixed

- parsing of RangeVar with missing inh field

## v0.5.3 - 2021-01-04

### Fixed

- parsing of alter table with type cast

## v0.5.2 - 2020-10-26

### Fixed

- parsing of create table with primary key constraint on two fields

## v0.5.1 - 2020-10-07

### Fixed

- run `prefer-text-field` on alter table statments

## v0.5.0 - 2020-09-08

### Added

- new rule `adding-primary-key-field`

### Fixed

- parsing `->>` operator

## v0.4.1 - 2020-08-19

### Fixed

- parse function calls in alter table statements

## v0.4.0 - 2020-07-19

### Added

- new rule `ban-char-type`

## v0.3.0 - 2020-07-10

### Changed

- upload-to-github comment formatting to hopefully be more clear
- docs on crates.io for sub crates

## v0.2.3 - 2020-07-09

### Added

- new rule `prefer-robust-stmts`

## v0.2.2 - 2020-06-25

### Fixed

- upload-to-github commenting on PRs when no files provided (#30)

## v0.2.1 - 2020-06-25

### Changed

- remove `SQUAWK_GITHUB_BOT_NAME` env var for github upload, no longer needed (#27)

### Fixed

- false positive in unique constraint rule (#28)

## v0.2.0 - 2020-06-23

### Added

- logging, mainly around upload-to-github (#24)
- `--stdin-filepath` argument (#23)
- output a success message for CLI tty reporter (#22)

### Changed

- prefix env vars with SQUAWK\_ (#21)

### Fixed

- error level HTTP status codes not being errors (#20)

## v0.1.4 - 2020-06-21

### Added

- `upload-to-github` subcommand for outputing squawk results in a GitHub PR
  comment.
- print help menu when no options provided
- automatically detect stdin instead of using the `-` path

### Fixed

- off by one error in slicing problem SQL for the tty reporter

## v0.1.3 - 2020-06-12

### Added

- documentation for rules
- release binaries
- CI

## v0.1.0 - 2020-06-05

### Added

- Initial release
