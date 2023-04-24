use std::io;

use arrayvec::ArrayVec;
use syslog_fmt::{
    v5424::{self, Timestamp},
    Severity,
};

#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() -> io::Result<()> {
    // The first call to Local::new initializes a thread safe cache within chrono
    let _datetime = chrono::Local::now();

    // the creation of a Formatter allocates on the heaps
    let formatter = v5424::Config {
        app_name: Some("default_config_example"),
        ..Default::default()
    }
    .into_formatter();

    let _profiler = dhat::Profiler::builder().testing().build();

    let mut buf = ArrayVec::<u8, 256>::new();

    formatter
        .format_with_data(
            &mut buf,
            Severity::Info,
            Timestamp::CreateChronoLocal,
            "'su root' failed for lonvick on /dev/pts/8",
            None,
            vec![(
                "exampleSDID@32473",
                vec![
                    ("iut", "3"),
                    ("eventSource", "Application"),
                    ("eventID", "1011"),
                ],
            )],
        )
        .unwrap();

    let stats = dhat::HeapStats::get();

    dhat::assert_eq!(stats.total_bytes, 589);

    Ok(())
}
