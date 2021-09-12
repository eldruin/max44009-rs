# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

...

## [0.2.0] - 2021-09-12

### Added
- Setting the configuration mode.
- Setting the integration time.
- Setting the current division ratio.
- Reading the integration time.
- Reading the current division ratio.
- Enabling/disabling interrupt generation.
- Checking if an interrupt has happened.
- Note about compatibility with MAX44007.

### Changed
- [breaking-change] Remove implementing `Default` for `Max44009`.
- Make types `ConfigurationMode`, `CurrentDivisionRatio`, `IntegrationTime`,
  `MeasurementMode` and `SlaveAddr` implement `Copy`.

## 0.1.0 - 2018-10-19

This is the initial release to crates.io. All changes will be documented in
this CHANGELOG.

[Unreleased]: https://github.com/eldruin/max44009-rs/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/eldruin/max44009-rs/compare/v0.1.0...v0.2.0

