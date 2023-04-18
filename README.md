<img alt="GitHub Workflow Status (with branch)" src="https://img.shields.io/github/actions/workflow/status/tandemdrive/syslog-fmt/ci.yml?branch=main&logo=github&style=for-the-badge">
![crates](https://img.shields.io/crates/v/syslog_fmt.svg?style=for-the-badge&color=fc8d62&logo=rust)
![build status](https://img.shields.io/github/actions/workflow/status/bheylin/syslog-fmt/ci.yml?logo=github&style=for-the-badge)


Formatter for the [5424](https://datatracker.ietf.org/doc/html/rfc5424) syslog standard.

This crate aims to provide a quality formatter for the 5424 spec.
We consciously limit the crate to the task of formatting to avoid entangling 
the separate concerns of formatting and transport.

Read through the [examples](examples) to see basic usages of the formatter with various transports.
