# Release 0.6.0

## What's Changed
* Use an anonymous const rather than a dummy name by @CodingAnarchy in https://github.com/fdeantoni/prost-wkt/pull/65
* Upgrade prost crates to 0.13 by @nickpresta in https://github.com/fdeantoni/prost-wkt/pull/68

## Breakting Changes
* All Prost 0.12 to 0.13 breaking changes
* The `From<DateTime> for Timestamp` has been replaced by `TryFrom`.

**Full Changelog**: https://github.com/fdeantoni/prost-wkt/compare/v0.5.1...v0.6.0


# Releaes 0.5.2

**Note**: This release was yanked in favour of 0.6.0


# Release 0.5.1

## What's Changed
* Updated Prost to 0.12.3
* Fixed chrono to 0.4.27 minimum
* implement serialize/deserialize for Duration type in accordance with protobuf JSON spec by @chrnorm in https://github.com/fdeantoni/prost-wkt/pull/61
* MSRV is now 1.70

**Full Changelog**: https://github.com/fdeantoni/prost-wkt/compare/v0.5.0...v0.5.1