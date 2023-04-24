# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

- Added `timestamp` arg for format fns
  The `Timestamp` type accepts preformatted timestamps in 
  `&str`, `String` and `&[u8]` forms
- Changed the timestamp formatting to use a custom
  formatter that doesn't allocate on the heap.
- Changed formatting a messsage without structured data does not use any
  heap allocations. See the test folder for verifications of this.
- Removed unused Error type.

## [0.2.0] - 2023-04-20

- Added `From<Config> for Formatter` and `impl Default` for `Config` and `Formatter`.
- Removed `fn exe_name_from_env`.
  This can easily be provided by the user.
- Removed `Cargo.lock` as this is a library.

## [0.1.2] - 2023-04-19

- Add `simple_datagram_based_logger` example
- Add CONTRIBUTING.md
- Change rustdoc 
  - rephrase comments that quote the spec
  - include link to spec sections in comments
- Change README
  - Add project goal
  - include link to examples
  - include Contributing section
  - include License section

## [0.1.1] - 2023-04-17

- Moved repo from `bheylin` account to `tandemdrive` org.
- Updated Cargo.toml to Tandemdrive details.

## [0.1.0] - 2023-04-17

- Added formatter for 5424 syslog spec.
