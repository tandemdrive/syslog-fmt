Formatter for the [5424](https://datatracker.ietf.org/doc/html/rfc5424) syslog standard.

This crate aims to provide a quality formatter for the 5424 spec.
We consciously limit the crate to the task of formatting to avoid entangling 
the separate concerns of formatting and transport.

Read through the [examples](examples) to see basic usages of the formatter with various transports.
