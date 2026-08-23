# Release 0.7.2

## What's Changed
* Raise the minimum Rust version to `1.85` by @LucaCappelletti94 in https://github.com/fdeantoni/prost-wkt/pull/87
* Ship the imported `google.protobuf` schemas by @LucaCappelletti94 in https://github.com/fdeantoni/prost-wkt/pull/86
* Add utoipa feature to prost-wkt-types by @AlJohri in https://github.com/fdeantoni/prost-wkt/pull/88
* Corrected the schemars and utoipa schemas: `Duration` accepts negative values and no longer claims `format: duration`, `Timestamp` no longer claims `format: date-time`, and `Any` only requires `@type`. utoipa components are registered as `google.protobuf.*`.
* The `schemars` and `utoipa` features are now built and tested in CI and enabled on docs.rs

## Notes
* `prost-wkt-types` now vendors the `google.protobuf` well-known type schemas (protobuf v35.1), so the generated code no longer depends on the installed `protoc` or on `protox`. These files are BSD-3-Clause, so the crate's license expression is now `Apache-2.0 AND BSD-3-Clause`.
* The minimum supported Rust version is now 1.85.

**Full Changelog**: https://github.com/fdeantoni/prost-wkt/compare/v0.7.1...v0.7.2


# Release 0.7.1

## What's Changed
* Converted to proper cargo workspace layout
* Updated rust-version in line with prost to 1.82
* Updated schema-rs from 0.8.0 to 1.2.0 by @maxwase in https://github.com/fdeantoni/prost-wkt/pull/85

**Full Changelog**: https://github.com/fdeantoni/prost-wkt/compare/v0.7.0...v0.7.1


# Release 0.7.0

## What's Changed
* Prost 0.14 by @jayhf in https://github.com/fdeantoni/prost-wkt/pull/83

**Full Changelog**: https://github.com/fdeantoni/prost-wkt/compare/v0.6.1...v0.7.0


# Release 0.6.1

## What's Changed
* bump protox version to match `prost` version being used. by @GeneralOneill in https://github.com/fdeantoni/prost-wkt/pull/70
* Upgrade protobuf-src from 1.1.0 to 2.1.0 by @daniel-b2c2 in https://github.com/fdeantoni/prost-wkt/pull/71
* feat: Add customization of type url generation by @asyade in https://github.com/fdeantoni/prost-wkt/pull/72
* Add Schemars Support by @jayhf in https://github.com/fdeantoni/prost-wkt/pull/77

**Full Changelog**: https://github.com/fdeantoni/prost-wkt/compare/v0.6.0...v0.6.1


# Release 0.6.0

## What's Changed
* Use an anonymous const rather than a dummy name by @CodingAnarchy in https://github.com/fdeantoni/prost-wkt/pull/65
* Upgrade prost crates to 0.13 by @nickpresta in https://github.com/fdeantoni/prost-wkt/pull/68

## Breaking Changes
* All Prost 0.12 to 0.13 breaking changes
* The `From<DateTime> for Timestamp` has been replaced by `TryFrom`.

**Full Changelog**: https://github.com/fdeantoni/prost-wkt/compare/v0.5.1...v0.6.0


# Release 0.5.2

**Note**: This release was yanked in favour of 0.6.0


# Release 0.5.1

## What's Changed
* Updated Prost to 0.12.3
* Fixed chrono to 0.4.27 minimum
* implement serialize/deserialize for Duration type in accordance with protobuf JSON spec by @chrnorm in https://github.com/fdeantoni/prost-wkt/pull/61
* MSRV is now 1.70

**Full Changelog**: https://github.com/fdeantoni/prost-wkt/compare/v0.5.0...v0.5.1