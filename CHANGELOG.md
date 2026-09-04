# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.1](https://github.com/rustfoundation/typomania/compare/v0.2.0...v0.2.1) - 2026-09-04

### Fixed

- *(deps)* update rust crate itertools to 0.15.0 ([#26](https://github.com/rustfoundation/typomania/pull/26))

### Other

- *(deps)* lock file maintenance ([#25](https://github.com/rustfoundation/typomania/pull/25))
- *(deps)* update actions/checkout action to v7 ([#27](https://github.com/rustfoundation/typomania/pull/27))

## [0.2.0](https://github.com/rustfoundation/typomania/compare/v0.1.2...v0.2.0) - 2026-06-12

### Fixed

- *(checks)* handle multi-byte UTF-8 in package names
- *(deps)* update rust crate itertools to 0.14.0 ([#18](https://github.com/rustfoundation/typomania/pull/18))
- *(deps)* update rust crate thiserror to v2 ([#20](https://github.com/rustfoundation/typomania/pull/20))

### Other

- *(deps)* lock file maintenance ([#24](https://github.com/rustfoundation/typomania/pull/24))
- *(deps)* Pin `dtolnay/rust-toolchain` action with explicit `toolchain: stable`
- *(deps)* pin dependencies
- *(deps)* update rust crate criterion to 0.8 ([#21](https://github.com/rustfoundation/typomania/pull/21))
- Use `config:best-practices` Renovate preset
- *(checks)* add proptest no-panic coverage
- *(checks)* push repeated char directly in `Repeated` check
- *(checks)* push swapped chars directly in `Characters` check
- *(checks)* move `rebuild_name()` into test module
- *(checks)* reuse a single buffer in `Characters` check
- *(checks)* reuse a single buffer in `Repeated` check
- *(checks)* push delimiter char directly instead of storing strings
- *(checks)* reuse a single buffer when generating variants
- add criterion benchmarks for checks
- *(deps)* update actions/checkout action to v6 ([#19](https://github.com/rustfoundation/typomania/pull/19))
- *(deps)* update rust crate rayon to v1.12.0 ([#17](https://github.com/rustfoundation/typomania/pull/17))
- *(deps)* update rust crate clap to v4.6.1 ([#16](https://github.com/rustfoundation/typomania/pull/16))
- *(deps)* update rust crate thiserror to v1.0.69 ([#13](https://github.com/rustfoundation/typomania/pull/13))
- *(deps)* update rust crate tracing to v0.1.44 ([#14](https://github.com/rustfoundation/typomania/pull/14))
- Add renovate.json ([#12](https://github.com/rustfoundation/typomania/pull/12))
- fix clippy lints on stable ([#10](https://github.com/rustfoundation/typomania/pull/10))
- run release-plz each commit

## [0.1.2](https://github.com/rustfoundation/typomania/compare/v0.1.1...v0.1.2) - 2023-11-10

### Fixed
- *(swapped)* limit the number of word permutations

### Other
- update lockfile

## [0.1.1](https://github.com/rustfoundation/typomania/compare/v0.1.0...v0.1.1) - 2023-10-13

### New features

- allow direct extraction of the squatted package

## [0.1.0](https://github.com/rustfoundation/typomania/releases/tag/v0.1.0) - 2023-09-27

### Other
- Initial release.
