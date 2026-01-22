# Changelog

All user-facing changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased] - ReleaseDate

## [1.0.2] - 2026-01-22

### Changed

- Add `#[must_use]` to `EnvGuard`

## [1.0.1] - 2025-08-11

### Fixed

- Acquire cwd lock before reading its previous value. This prevents some race conditions and interactions between tests
- Add `CurrentDirError` wrapper for `lock_current_dir` to provide more context for failures

## [1.0.0] - 2025-08-11

### Added

- Add `lock_current_dir` function to set and lock the current working directory for a process

## [0.1.2] - 2024-08-19

### Fixed

- Don't panic on subsequent tests after one test fails

## [0.1.1] - 2024-08-15

Downgrade MSRV to 1.70

## [0.1.0] - 2024-08-15

Initial release!
