# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
