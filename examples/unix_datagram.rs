use std::io;

fn main() -> io::Result<()> {
    #[cfg(unix)]
    unix::run()?;

    Ok(())
}

#[cfg(unix)]
mod unix {
    use std::{io, os::unix::net::UnixDatagram};

    use syslog_fmt::{
        v5424::{self, Timestamp},
        Facility, Severity,
    };

    pub fn run() -> io::Result<()> {
        const UNIX_SOCK_PATHS: [&str; 3] = ["/dev/log", "/var/run/syslog", "/var/run/log"];

        let socket = any_datagram_socket(&UNIX_SOCK_PATHS)?;

        let formatter = v5424::Config {
            facility: Facility::Local0,
            hostname: Some("localhost"),
            app_name: Some("unix_datagram_example"),
            proc_id: std::process::id().to_string().as_str().into(),
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

        socket.send(&buf)?;

        Ok(())
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
