# Changelog

All user-visible changes to this library will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/), as described
for Rust libraries in [RFC #1105](https://github.com/rust-lang/rfcs/blob/master/text/1105-api-evolution.md)

## [0.1.4] – 2024-12-22

The main goals of this release are:

- Stabilize API for more stringent compliance with the recommendations of IEEE 754 and IEEE 854 standards.
- Refactor decimal module and simplify context usage.

### Breaking changes

* Remove [`Context`] from a most methods argument list and put it into decimal control block for better context
  handling.
  Now [`Context`] is a property of any Decimal number instance.

### Added

* Extend test coverage.

### Changed

* Documentation improvements.
* Deprecate and replace some outdated methods to align with the updated structure and functionality.
* Benchmarks were modularized into separate operations, enhancing clarity and maintainability.

## [0.1.3] – 2024-12-16

### Added

* Extend test coverage.

### Fixed

* `sqlx` support for `PostgreSQL`.

### Changed

* Minor documentation fixes.
* Remove `const_str` dependency.

## [0.1.2] – 2024-12-15

### Added

* Added `signals!` macro.
* Extend test coverage.

### Changed

* Deprecated `with_scale!` in favor of `quantum!`.
* Documentation improvements.

## [0.1.1] – 2024-12-11

* Minor fixes.

## [0.1.0] – 2024-12-11

### Breaking changes

* Internal decimal representation for signed and unsigned decimal types.
* Reduce exponent from `64` bit to `16` bit.
* Remove `DecimalResult` and unwrap methods.
* Replace `ArithmeticPolicy`, `OverflowPolicy`, `RoundingPolicy` with `Context`.

### Added

* Add `NaN` and `±Infinity` special values.
* Add `Flags` and `Signal`.
* Add `Context` and `SignalingTraps`.
* Add `with_context!` macro.

### Fixed

* Fix panic shift with overflow.

### Changed

* Stabilize API.
* Re-implement most methods.
* Documentation improvements.

## [0.0.14] – 2024-11-27

* Fix rounding subtraction.
* Extend test coverage for `numtraits` feature.
* Documentation improvements.

## [0.0.13] – 2024-11-24

* Stabilize API

## [0.0.12] – 2024-11-23

## [0.0.11] – 2024-11-23

## [0.0.10] – 2024-11-23

## [0.0.9] – 2024-11-23

* Stabilize API

## [0.0.8] – 2024-11-19

* Bump `utoipa` to 5.0.x version

## [0.0.7] – 2024-11-19

* Stabilize API
* Improve docs

## [0.0.6] – 2024-11-19

* Stabilize API

## [0.0.6]

## [0.0.5]

## [0.0.4]

## [0.0.3]

* Technical releases

## [0.0.1] – 2024-10-21

* Initial commit