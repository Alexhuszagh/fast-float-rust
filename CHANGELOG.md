# Changelog

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

## 0.2.4

### Added

- Add support for `no-panic` (#28, by @malbarbo)

## 0.2.3

### Fixed

- Use `max_value` and not `MAX` for ancient Rust versions.
- Use `isize` and not `usize` for length checks.

## 0.2.2

### Fixed

- Fix `no_std` support.

### Removed

- Remove most uses of unsafe.
- Remove non-local safety invariants to prevent unsoundness.

## 0.2.1

### Fixed

- Fix undefined behavior in checking the buffer length.

## 0.2.0

### Fixed

- Fixed an edge case where long decimals with trailing zeros were truncated.

### Removed

- Remove the use of unsafe when querying power-of-10 tables.

### Added

- Added float64 roundtrip fuzz target.
- Added tests for the power-of-5 table using num-bigint.

### Changed

- Improvements and new options in the bench tool.
- Minor micro-optimization fixes in the fast path parser.
- Updated benchmark timings, added Apple M1 and AMD Rome timings.

## 0.1.0

Initial release, fully tested and benchmarked.
