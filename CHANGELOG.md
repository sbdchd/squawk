# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## v1.5.0

### Added

- Added warning to `adding-field-with-default` to warn about generated columns (#397).

### Fixed

- Fixed bug in `adding-required-field` where generated columns were incorrectly flagged (#397).

## v1.4.1

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
