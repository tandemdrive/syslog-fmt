use std::io;

use syslog_fmt::{
    v5424::{self, Timestamp},
    Severity,
};

fn main() -> io::Result<()> {
    let formatter = v5424::Config {
        app_name: Some("default_config_example"),
        ..Default::default()
    }
    .into_formatter();

    let mut buf = Vec::<u8>::new();
    formatter.write_without_data(
        &mut buf,
        Severity::Info,
        Timestamp::CreateChronoLocal,
        "'su root' failed for lonvick on /dev/pts/8",
        None,
    )?;

    println!(
        "{}",
        std::str::from_utf8(&buf).expect("message isn't valid UTF8")
    );

    Ok(())
}
