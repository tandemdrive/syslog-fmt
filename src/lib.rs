//! Formatter to convert a message into a valid syslog message for the [5425](https://datatracker.ietf.org/doc/html/rfc5424) syslog protocol.
//!
//! This crate does not provide a transport method to get the message to the syslog daemon.
//! The focus is to correctly format a message ready for transport.
pub mod v5424;

/// The Priority value is calculated by first multiplying the Facility
/// number by 8 and then adding the numerical value of the Severity.
///
/// For example, a kernel message (Facility=0) with a Severity of Emergency
/// (Severity=0) would have a Priority value of 0. A "local use 4"
/// message (Facility=20) with a Severity of Notice (Severity=5) would
/// have a Priority value of 165.
///
/// [spec](https://datatracker.ietf.org/doc/html/rfc5424#section-6.2.1)
type Priority = u8;

/// The facility argument is used to specify what type of program is logging the message.
/// This lets the configuration file specify that messages from different facilities will be handled differently.
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

impl Default for Facility {
    fn default() -> Self {
        Self::Local0
    }
}

/// The severity of the message
#[derive(Copy, Clone)]
#[repr(u8)]
pub enum Severity {
    /// System is unusable.
    /// For example: a panic condition.
    Emerg,
    /// Action must be taken immediately.
    /// For example: A condition that should be corrected immediately, such as a corrupted system database.
    Alert,
    /// Critical conditions
    /// For example: Hard device errors
    Crit,
    /// Error conditions.
    Err,
    /// Warning conditions.
    Warning,
    /// Normal but significant condition.
    /// For example: Conditions that are not error conditions, but that may require special handling.
    Notice,
    /// Informational messages.
    /// For example: Confirmation that the program is working as expected.
    Info,
    /// Debug-level messages.
    /// For example: Messages that contain information normally of use only when debugging a program.
    Debug,
}
