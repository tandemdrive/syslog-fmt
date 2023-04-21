//! Formatter to convert a message into a valid syslog message for the [5425](https://datatracker.ietf.org/doc/html/rfc5424) syslog protocol.
//!
//! This crate does not provide a transport method to get the message to the syslog daemon.
//! The focus is to correctly format a message ready for transport.

use core::{fmt, marker::PhantomData};
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

impl fmt::Display for Facility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Facility::Kern => "Kern",
            Facility::User => "User",
            Facility::Mail => "Mail",
            Facility::Daemon => "Daemon",
            Facility::Auth => "Auth",
            Facility::Syslog => "Syslog",
            Facility::Lpr => "Lpr",
            Facility::News => "News",
            Facility::Uucp => "Uucp",
            Facility::Cron => "Cron",
            Facility::Authpriv => "Authpriv",
            Facility::Ftp => "Ftp",
            Facility::Local0 => "Local0",
            Facility::Local1 => "Local1",
            Facility::Local2 => "Local2",
            Facility::Local3 => "Local3",
            Facility::Local4 => "Local4",
            Facility::Local5 => "Local5",
            Facility::Local6 => "Local6",
            Facility::Local7 => "Local7",
        };

        f.write_str(s)
    }
}

impl<T> fmt::Display for IntToEnumError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let enum_name: &'static str = std::any::type_name::<T>();
        write!(f, "Failed to convert {} to {}", self.value, enum_name)
    }
}

impl TryFrom<u8> for Facility {
    type Error = IntToEnumError<Self>;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Into::<i32>::into(value).try_into()
    }
}

/// Try convert a i32 (libc::c_int) into a Facility
impl TryFrom<i32> for Facility {
    type Error = IntToEnumError<Self>;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let variant = match value {
            0 => Self::Kern,
            1 => Self::User,
            2 => Self::Mail,
            3 => Self::Daemon,
            4 => Self::Auth,
            5 => Self::Syslog,
            6 => Self::Lpr,
            7 => Self::News,
            8 => Self::Uucp,
            9 => Self::Cron,
            10 => Self::Authpriv,
            11 => Self::Ftp,
            16 => Self::Local0,
            17 => Self::Local1,
            18 => Self::Local2,
            19 => Self::Local3,
            20 => Self::Local4,
            21 => Self::Local5,
            22 => Self::Local6,
            23 => Self::Local7,
            _ => {
                return Err(IntToEnumError {
                    value,
                    target: PhantomData,
                })
            }
        };

        Ok(variant)
    }
}

/// The severity of the message
#[derive(Copy, Clone, Debug)]
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

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Severity::Emerg => "Emerg",
            Severity::Alert => "Alert",
            Severity::Crit => "Crit",
            Severity::Err => "Err",
            Severity::Warning => "Warning",
            Severity::Notice => "Notice",
            Severity::Info => "Info",
            Severity::Debug => "Debug",
        };

        f.write_str(s)
    }
}

impl TryFrom<u8> for Severity {
    type Error = IntToEnumError<Self>;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Into::<i32>::into(value).try_into()
    }
}

/// Try convert a i32 (libc::c_int) into a Severity
impl TryFrom<i32> for Severity {
    type Error = IntToEnumError<Self>;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let variant = match value {
            0 => Self::Emerg,
            1 => Self::Alert,
            2 => Self::Crit,
            3 => Self::Err,
            4 => Self::Warning,
            5 => Self::Notice,
            6 => Self::Info,
            7 => Self::Debug,
            _ => {
                return Err(IntToEnumError {
                    value,
                    target: PhantomData,
                })
            }
        };

        Ok(variant)
    }
}

/// Error returned if converting from an integer to a u8 based enum fails
pub struct IntToEnumError<T> {
    value: i32,
    target: PhantomData<T>,
}

impl<T> fmt::Debug for IntToEnumError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IntToEnumError")
            .field("value", &self.value)
            .field("target", &std::any::type_name::<T>())
            .finish()
    }
}
