use std::io;

use arrayvec::ArrayVec;
use syslog_fmt::{
    v5424::{self, Timestamp},
    Severity,
};

#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() -> io::Result<()> {
    // the first call to Local::new initializes a thread safe cache within chrono
    let _datetime = chrono::Local::now();

    // the creation of a Formatter allocates on the heaps
    let formatter = v5424::Config {
        app_name: Some("default_config_example"),
        ..Default::default()
    }
    .into_formatter();

    let _profiler = dhat::Profiler::builder().testing().build();

    let mut buf = ArrayVec::<u8, 128>::new();

    formatter.format(
        &mut buf,
        Severity::Info,
        Timestamp::CreateChronoLocal,
        "'su root' failed for lonvick on /dev/pts/8",
        None,
    )?;

    let stats = dhat::HeapStats::get();

    dhat::assert_eq!(stats.total_bytes, 0);

    Ok(())
}
