![docs](https://img.shields.io/badge/docs.rs-syslog-fmt?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64)
![crates](https://img.shields.io/crates/v/syslog-fmt.svg?style=for-the-badge&color=fc8d62&logo=rust)
![build status](https://img.shields.io/github/actions/workflow/status/bheylin/syslog-fmt/ci.yml?logo=github&style=for-the-badge)


Formatter for the [5424](https://datatracker.ietf.org/doc/html/rfc5424) syslog standard.

This crate aims to provide a quality formatter for the 5424 spec.
We consciously limit the crate to the task of formatting to avoid entangling 
the separate concerns of formatting and transport.

Read through the [examples](examples) to see basic usages of the formatter with various transports.
