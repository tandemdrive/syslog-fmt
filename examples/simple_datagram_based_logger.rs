use std::io;

fn main() -> io::Result<()> {
    #[cfg(unix)]
    unix::run()?;

    Ok(())
}

#[cfg(unix)]
mod unix {
    use std::{
        io::{self, Write},
        os::unix::net::UnixDatagram,
    };

    use arrayvec::ArrayVec;
    use is_terminal::IsTerminal;
    use parking_lot::Mutex;
    use syslog_fmt::{v5424, Facility, Severity};

    const SYSLOG_MSG_BUFFER_LEN: usize = 1024;

    struct DatagramLogger {
        socket: UnixDatagram,
        formatter: v5424::Formatter,
        buf: Mutex<ArrayVec<u8, SYSLOG_MSG_BUFFER_LEN>>,
        log_level: log::LevelFilter,
    }

    impl log::Log for DatagramLogger {
        fn enabled(&self, metadata: &log::Metadata<'_>) -> bool {
            metadata.level() <= self.log_level
        }

        fn log(&self, record: &log::Record<'_>) {
            if self.enabled(record.metadata()) {
                let mut buf = self.buf.lock();

                let res = self
                    .formatter
                    .format(&mut *buf, Severity::Info, record.args(), None);

                if let Err(e) = res {
                    // ignore when the buffer runs over capcity
                    // write as much as you can and drop the rest
                    if e.kind() != io::ErrorKind::WriteZero {
                        eprintln!("{e}");
                    }
                }

                if let Err(e) = self.socket.send(&buf) {
                    eprintln!("{e}")
                }
            }
        }

        fn flush(&self) {}
    }

    struct StdErrLogger {
        formatter: v5424::Formatter,
        buf: Mutex<ArrayVec<u8, SYSLOG_MSG_BUFFER_LEN>>,
        log_level: log::LevelFilter,
    }

    impl log::Log for StdErrLogger {
        fn enabled(&self, metadata: &log::Metadata<'_>) -> bool {
            metadata.level() <= self.log_level
        }

        fn log(&self, record: &log::Record<'_>) {
            if self.enabled(record.metadata()) {
                let mut buf = self.buf.lock();

                let res = self
                    .formatter
                    .format(&mut *buf, Severity::Info, record.args(), None);

                if let Err(e) = res {
                    // ignore when the buffer runs over capcity
                    // write as much as you can and drop the rest
                    if e.kind() != io::ErrorKind::WriteZero {
                        eprintln!("{e}");
                    }
                }

                std::io::stderr().write(&buf).unwrap();
            }
        }

        fn flush(&self) {}
    }

    pub fn run() -> io::Result<()> {
        setup_logger()?;

        log::info!("'su root' failed for lonvick on /dev/pts/8");
        log::debug!("'su root' failed for lonvick on /dev/pts/8");

        Ok(())
    }

    fn setup_logger() -> io::Result<()> {
        if std::io::stderr().is_terminal() {
            setup_stderr_logger()
        } else {
            setup_datagram_logger()
        }
    }

    fn setup_stderr_logger() -> io::Result<()> {
        let formatter = setup_syslog_formatter();
        let logger = StdErrLogger {
            formatter,
            buf: Mutex::new(ArrayVec::new()),
            log_level: log::LevelFilter::Info,
        };

        log::set_max_level(logger.log_level);
        log::set_boxed_logger(Box::new(logger))
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        Ok(())
    }

    fn setup_datagram_logger() -> io::Result<()> {
        const UNIX_SOCK_PATHS: [&str; 3] = ["/dev/log", "/var/run/syslog", "/var/run/log"];

        let socket = any_datagram_socket(&UNIX_SOCK_PATHS)?;
        let formatter = setup_syslog_formatter();

        let logger = DatagramLogger {
            socket,
            formatter,
            buf: Mutex::new(ArrayVec::new()),
            log_level: log::LevelFilter::Info,
        };

        log::set_max_level(logger.log_level);
        log::set_boxed_logger(Box::new(logger))
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        Ok(())
    }

    fn setup_syslog_formatter() -> v5424::Formatter {
        v5424::Formatter::from_config(v5424::Config {
            facility: Facility::Auth,
            hostname: Some("localhost"),
            app_name: Some("unix_datagram_example"),
            proc_id: std::process::id().to_string().as_str().into(),
        })
    }

    /// Try to connect as a datagram socket to any of the given paths.
    ///
    /// It's quite likely that a datagram socket is being used for a syslog setup, as
    /// syslog messages should not be large enough to justify streaming.
    fn any_datagram_socket(paths: &[&str]) -> Result<UnixDatagram, io::Error> {
        for path in paths {
            if let Ok(socket) = UnixDatagram::unbound() {
                if socket.connect(path).is_ok() {
                    return Ok(socket);
                };
            }
        }

        Err(io::Error::new(
            io::ErrorKind::Other,
            "No datagram socket could be found",
        ))
    }
}
