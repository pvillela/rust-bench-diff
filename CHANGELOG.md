# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.6] - 2025-05-21

### Changed

- Replaced internal implementation of statistical functions with `basic_stats` library.
- Reorganized private features.
- Other internal housekeeping.

## [1.0.5] - 2025-04-26

### Changed

- Refactored to avoid allocation of histograms subsequent to warm-up.
- Deprecated `statistics` module, replacing it with equivalent `stats_types` module. This is the only publicly visible change and it does not impact compatibility with previous versions.
- Renamed all private features, prefixing them with '_'.
- Other internal housekeeping.

## [1.0.4] - 2025-04-25

### Changed

- Fixed incorrect dates in CHANGELOG.md.
- Minor update to documentation to reflect that this library is typically used as a dev-dependency.
- Added one more exclusion (guarded by a private feature) in Cargo.toml.
- Minor clean-up of benches and scripts.

## [1.0.3] - 2025-04-23

### Changed

- Updated the Quick Start section of lib doc with a dependencies paragraph.
- Reverted the Cargo.toml exclusions that had been removed in the previous yanked version.

## [1.0.2] - 2025-04-23

**This version was yanked.**

### Changed

- Removed some exclusions in Cargo.toml.

## [1.0.1] - 2025-04-23

### Changed

- Minor update to README.md.

## [1.0.0] - 2025-04-23

First release.
