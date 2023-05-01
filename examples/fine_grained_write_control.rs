use std::io::{self, Write};

use arrayvec::ArrayVec;
use syslog_fmt::{
    v5424::{self, Timestamp},
    Severity,
};

fn main() -> io::Result<()> {
    let formatter = v5424::Config {
        app_name: Some("fine_grained_write_control_example"),
        ..Default::default()
    }
    .into_formatter();

    // This example shows how to use a single buffer to write out a complex
    // syslog message with structured data and variable substitution in the msg section
    // while not allocating on the heap
    let mut buf = ArrayVec::<u8, 1024>::new();
    formatter.write_header(&mut buf, Severity::Info, Timestamp::CreateChronoLocal, None)?;

    // This type of data is supplied by the `log` crate
    let module = "app::connection";
    let lineno = "101";

    // Write the location as structured data.
    // No heap allocation occurs due to the use of arrays as arguments
    // and a preallocated buffer is supplied
    v5424::write_data(
        &mut buf,
        [("location", [("module", module), ("lineno", lineno)])],
    )?;

    // The spec requires that a UTF8 encoded string should be prefixed with a BOM
    v5424::write_utf8_bom(&mut buf)?;

    // write a message manually to the end of the buffer.
    // again no heap allocations occur due to the use of a preallocated buffer
    let command = "su root";
    let device = "/dev/pts/8";
    write!(&mut buf, "'{command}' failed for lonvick on {device}").unwrap();

    println!(
        "{}",
        std::str::from_utf8(&buf).expect("message isn't valid UTF8")
    );

    Ok(())
}
