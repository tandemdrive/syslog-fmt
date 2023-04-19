[<img alt="crates.io" src="https://img.shields.io/crates/v/syslog-fmt.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/syslog-fmt)
[<img alt="docs.rs" src="https://img.shields.io/docsrs/syslog_fmt/latest?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/syslog-fmt)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/tandemdrive/syslog-fmt/ci.yml?branch=main&logo=github&style=for-the-badge" height="20">
](https://github.com/tandemdrive/syslog-fmt/actions?query=branch%3Amaster)

Formatter for the [5424](https://datatracker.ietf.org/doc/html/rfc5424) syslog standard.

This crate aims to provide a quality formatter for the 5424 spec.
We consciously limit the crate to the task of formatting to avoid entangling 
the separate concerns of formatting and transport.

Read through the [examples](examples) to see basic usages of the formatter with various transports.


## Contributing

We welcome community contributions to this project.

Please read our [Contributor Terms](CONTRIBUTING.md#contributor-terms) before
you make any contributions.

Any contribution intentionally submitted for inclusion, shall comply with the
Rust standard licensing model (MIT OR Apache 2.0) and therefore be dual licensed
as described below, without any additional terms or conditions:


### License

This contribution is dual licensed under EITHER OF

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

For clarity, "your" refers to TandemDrive or any other licensee/user of the contribution.

