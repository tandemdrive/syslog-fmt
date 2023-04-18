[<img alt="crates.io" src="https://img.shields.io/crates/v/syslog-fmt.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/syslog-fmt)

[<img alt="docs.rs" src="https://img.shields.io/docsrs/syslog_fmt/latest?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/syslog-fmt)

[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/tandemdrive/syslog-fmt/ci.yml?branch=main&logo=github&style=for-the-badge">
](https://github.com/tandemdrive/syslog-fmt/actions?query=branch%3Amaster)

Formatter for the [5424](https://datatracker.ietf.org/doc/html/rfc5424) syslog standard.

This crate aims to provide a quality formatter for the 5424 spec.
We consciously limit the crate to the task of formatting to avoid entangling 
the separate concerns of formatting and transport.

Read through the [examples](examples) to see basic usages of the formatter with various transports.
