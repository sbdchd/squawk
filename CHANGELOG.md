# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
