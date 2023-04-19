use std::io;

use syslog_fmt::{v5424, Severity};

fn main() -> io::Result<()> {
    let formatter = v5424::Config {
        app_name: Some("unix_datagram_example"),
        ..Default::default()
    }
    .into_formatter();

    let mut buf = Vec::<u8>::new();
    formatter.format(
        &mut buf,
        Severity::Info,
        "'su root' failed for lonvick on /dev/pts/8",
        None,
    )?;

    println!(
        "{}",
        std::str::from_utf8(&buf).expect("message isn't valid UTF8")
    );

    Ok(())
}
