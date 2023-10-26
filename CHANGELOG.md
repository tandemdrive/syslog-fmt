# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

- Added the Cargo.lock file to the git repo
  See: https://blog.rust-lang.org/2023/08/29/committing-lockfiles.html
- Upgrade dependencies

## [0.3.1] - 2023-05-01

### Added

- Example showing how to have fine grained control over the writing of syslog messages

### Changed

- Fixed the test links in the README

## [0.3.0] - 2023-05-01

### Added 

- `timestamp` arg for format fns
  The `Timestamp` type accepts preformatted timestamps in 
  `&str`, `String` and `&[u8]` forms
- Fine grained fns for writing header, structured data and message UTF8 BOM.
  These can be used to write/format a message piece by piece without heap allocations

### Changed

- Renamed `format_*` fns to `write_*` to emphasize that they utilize a Writer.
- timestamp formatting uses a custom formatter that doesn't allocate on the heap.
- formatting a message with and without structured data does not perform any
  heap allocations. See the test folder for verifications of this.

### Removed

- Unused Error type.

## [0.2.0] - 2023-04-20

### Added 

- `From<Config> for Formatter` and `impl Default` for `Config` and `Formatter`.

### Removed 

- `fn exe_name_from_env`.
  This can easily be provided by the user.
- `Cargo.lock` as this is a library.

## [0.1.2] - 2023-04-19

### Added

- `simple_datagram_based_logger` example
- CONTRIBUTING.md

### Changed

- Rephrased rustdoc comments that quote the spec and include link to spec sections in comments
- Improve docs
  - Add project goal to README
  - include link to examples
  - include Contributing section
  - include License section

## [0.1.1] - 2023-04-17

### Changed

- Moved repo from `bheylin` account to `tandemdrive` org.
- Updated Cargo.toml to Tandemdrive details.

## [0.1.0] - 2023-04-17

### Added

- formatter for 5424 syslog spec.
