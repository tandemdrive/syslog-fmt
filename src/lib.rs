//! Formatters to convert a message into a valid syslog message for both the [3164](https://datatracker.ietf.org/doc/html/rfc3164) and [5425](https://datatracker.ietf.org/doc/html/rfc5424) syslog protocols.
//!
//! This crate does not provide a transport method to get the message to the syslog daemon.
//! The focus is to correctly format a message to transport.
pub mod v5424;

use std::{fmt, io};

pub struct Error {}

impl std::error::Error for Error {}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("error").finish()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Error")
    }
}

/// The Priority value is calculated by first multiplying the Facility
/// number by 8 and then adding the numerical value of the Severity. For
/// example, a kernel message (Facility=0) with a Severity of Emergency
/// (Severity=0) would have a Priority value of 0. Also, a "local use 4"
/// message (Facility=20) with a Severity of Notice (Severity=5) would
/// have a Priority value of 165. In the PRI of a syslog message, these
/// values would be placed between the angle brackets as <0> and <165>
/// respectively. The only time a value of "0" follows the "<" is for
/// the Priority value of "0". Otherwise, leading "0"s MUST NOT be used.
pub type Priority = u8;

///  Facility values MUST be in the range of 0 to 23 inclusive
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Facility {
    /// kernel messages
    Kern = 0 << 3,
    /// user-level messages
    User = 1 << 3,
    /// mail system
    Mail = 2 << 3,
    /// system daemons
    Daemon = 3 << 3,
    /// security/authorization messages
    Auth = 4 << 3,
    /// messages generated internally by syslogd
    Syslog = 5 << 3,
    /// line printer subsystem
    Lpr = 6 << 3,
    /// network news subsystem
    News = 7 << 3,
    /// UUCP subsystem
    Uucp = 8 << 3,
    /// clock daemon
    Cron = 9 << 3,
    /// security/authorization messages
    Authpriv = 10 << 3,
    /// FTP daemon
    Ftp = 11 << 3,
    /// local use 0  (local0)
    Local0 = 16 << 3,
    /// local use 1  (local1)
    Local1 = 17 << 3,
    /// local use 2  (local2)
    Local2 = 18 << 3,
    /// local use 3  (local3)
    Local3 = 19 << 3,
    /// local use 4  (local4)
    Local4 = 20 << 3,
    /// local use 5  (local5)
    Local5 = 21 << 3,
    /// local use 6  (local6)
    Local6 = 22 << 3,
    /// local use 7  (local7)
    Local7 = 23 << 3,
}

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum Severity {
    /// system is unusable
    Emerg,
    /// action must be taken immediately
    Alert,
    /// critical conditions
    Crit,
    /// error conditions
    Err,
    /// warning conditions
    Warning,
    /// normal but significant condition
    Notice,
    /// informational messages
    Info,
    /// debug-level messages
    Debug,
}

/// Return the executable name.
pub fn exe_name_from_env() -> io::Result<String> {
    std::env::current_exe()?
        .file_name()
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "exe path has no filename"))?
        .to_str()
        .map(String::from)
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "exe name is not valid UTF8"))
}
