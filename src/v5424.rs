//! A Formatter and associated types that converts a message and optional structured data
//! into an [RFC 5424](https://datatracker.ietf.org/doc/html/rfc5424) compliant message.
use core::fmt;
use std::{borrow::Cow, io};

use crate::{Facility, Priority, Severity};

/// Configuration for the building a `Formatter`
#[derive(Default)]
pub struct Config<'a> {
    pub facility: Facility,
    pub hostname: Option<&'a Hostname>,
    pub app_name: Option<&'a AppName>,
    pub proc_id: Option<&'a ProcId>,
}

impl<'a> Config<'a> {
    pub fn into_formatter(self) -> Formatter {
        self.into()
    }
}

impl<'a> From<Config<'a>> for Formatter {
    fn from(config: Config<'a>) -> Self {
        Formatter::from_config(config)
    }
}

/// Formats a message and optional structured data into a into an [RFC 5424](https://datatracker.ietf.org/doc/html/rfc5424) compliant message.
#[derive(Clone, Debug)]
pub struct Formatter {
    facility: Facility,

    /// The hostname, app_name and pid substring can be preformatted
    /// given that they don't change per syslog session
    host_app_proc_id: Box<str>,
}

impl Default for Formatter {
    fn default() -> Self {
        Config::default().into_formatter()
    }
}

impl Formatter {
    /// Create a new syslog 5424 formatter
    ///
    /// Even though the hostname is optional, it's considered highly unlikely that you can't supply one.
    /// See <https://datatracker.ietf.org/doc/html/rfc5424#section-6.2.4>.
    /// A warning will be logged if no hostname is provided.
    pub fn from_config(config: Config<'_>) -> Self {
        let hostname = config.hostname;
        let app_name = config.app_name;
        let proc_id = config.proc_id;

        let hostname = hostname.unwrap_or(NILVALUE);
        let app_name = app_name.unwrap_or(NILVALUE);
        let proc_id = proc_id.unwrap_or(NILVALUE);

        let host_app_proc_id = format!("{hostname} {app_name} {proc_id}").into_boxed_str();

        Self {
            facility: config.facility,
            host_app_proc_id,
        }
    }

    /// Format a syslog 5424 message with structured data.
    ///
    /// This method is a special case as the use of structured data is less likely than providing a simple string message.
    ///
    /// ```rust
    /// use std::io::Write;
    ///
    /// use syslog_fmt::{Severity, Facility, v5424::{Config, Formatter, Timestamp}};
    ///
    /// let mut buf = Vec::<u8>::new();
    /// let formatter = Config {
    ///     facility: Facility::Local7,
    ///     hostname: Some("localhost"),
    ///     app_name: Some("app-name"),
    ///     proc_id: Some("proc-id"),
    /// }
    /// .into_formatter();
    /// formatter.format_with_data(
    ///     &mut buf,
    ///     Severity::Info,
    ///     Timestamp::CreateChronoLocal,
    ///     "this is a message",
    ///     Some("msg-id"),
    ///     vec![("elem-a", vec![("param-a", "value-a")])]
    /// );
    /// ```
    pub fn format_with_data<'a, W, TS, M, I, P>(
        &self,
        w: &mut W,
        severity: Severity,
        timestamp: TS,
        msg: M,
        msg_id: Option<&MsgId>,
        data: I,
    ) -> io::Result<()>
    where
        W: io::Write,
        TS: Into<Timestamp<'a>>,
        M: Into<Msg<'a>>,
        I: IntoIterator<Item = (&'a SdId, P)>,
        P: IntoIterator<Item = SdParam<'a>>,
    {
        let data = data
            .into_iter()
            .map(|(id, params)| SdElement {
                id,
                params: params.into_iter().collect::<Vec<_>>(),
            })
            .collect::<Vec<_>>();

        self.format_items(w, severity, timestamp, msg, msg_id, Some(data))
    }

    /// Format a syslog 5424 message given a simple string message.
    /// An optional MSG-ID can be provided by using a two string tuple for the msg param:
    ///
    /// ```rust
    /// use std::io::Write;
    ///
    /// use syslog_fmt::{Severity, Facility, v5424::{Config, Formatter, Timestamp}};
    ///
    /// let mut buf = Vec::<u8>::new();
    /// let formatter = Config {
    ///     facility: Facility::Local7,
    ///     hostname: Some("localhost"),
    ///     app_name: Some("app-name"),
    ///     proc_id: Some("proc-id"),
    /// }
    /// .into_formatter();
    /// formatter.format(
    ///     &mut buf,
    ///     Severity::Info,
    ///     Timestamp::CreateChronoLocal,
    ///     "this is a message",
    ///     Some("msg-id")
    /// );
    /// ```
    pub fn format<'a, W, TS, M>(
        &self,
        w: &mut W,
        severity: Severity,
        timestamp: TS,
        msg: M,
        msg_id: Option<&MsgId>,
    ) -> io::Result<()>
    where
        W: io::Write,
        TS: Into<Timestamp<'a>>,
        M: Into<Msg<'a>>,
    {
        self.format_items(w, severity, timestamp, msg, msg_id, None)
    }

    /// Format a syslog [5424](https://datatracker.ietf.org/doc/html/rfc5424#section-6) message
    fn format_items<'a, W, TS, M>(
        &self,
        w: &mut W,
        severity: Severity,
        timestamp: TS,
        msg: M,
        msg_id: Option<&MsgId>,
        data: Option<StructuredData<'a>>,
    ) -> io::Result<()>
    where
        W: io::Write,
        TS: Into<Timestamp<'a>>,
        M: Into<Msg<'a>>,
    {
        let Self {
            facility,
            host_app_proc_id,
        } = self;

        let prio = encode_priority(severity, *facility);
        let msg_id = msg_id.unwrap_or(NILVALUE);

        let data: Cow<'a, str> = if let Some(data) = data {
            if data.is_empty() {
                NILVALUE.into()
            } else {
                data_to_string(data).into()
            }
        } else {
            NILVALUE.into()
        };

        write!(w, "<{prio}>{VERSION} ")?;

        let timestamp = timestamp.into();

        match timestamp {
            #[cfg(feature = "chrono")]
            Timestamp::Chrono(datetime) => {
                format_chrono_datetime(w, datetime)?;
            }
            #[cfg(feature = "chrono")]
            Timestamp::CreateChronoLocal => {
                let datetime = chrono::Local::now();
                format_chrono_datetime(w, &datetime)?;
            }
            Timestamp::PreformattedStr(s) => w.write_all(s.as_bytes())?,
            Timestamp::PreformattedString(s) => w.write_all(s.as_bytes())?,
            Timestamp::None => w.write_all(NILVALUE.as_bytes())?,
        };

        write!(w, " {host_app_proc_id} {msg_id} {data}")?;

        let msg = msg.into();

        match msg {
            Msg::Utf8Str(s) => write_str_msg(w, s)?,
            Msg::Utf8String(s) => write_str_msg(w, &s)?,
            Msg::NonUnicodeBytes(bytes) => w.write(bytes).map(|_| ())?,
            Msg::FmtArguments(args) => write!(w, " {args}")?,
            Msg::FmtArgumentsRef(args) => write!(w, " {args}")?,
        };

        Ok(())
    }
}

#[cfg(feature = "chrono")]
fn format_chrono_datetime<W: io::Write>(w: &mut W, datetime: &ChronoLocalTime) -> io::Result<()> {
    use chrono::Timelike;

    const MILLI_IN_NANO: u32 = 1000;
    const SEC_IN_HOUR: i32 = 3600;
    const PLUS: &str = "+";
    const MIN: &str = "-";

    // reuse chrono `Debug` impls which already print ISO 8601 format.
    let date = datetime.date_naive();
    let time = datetime.time();
    let h = time.hour();
    let m = time.minute();
    let s = time.second();
    let ms = time.nanosecond() / MILLI_IN_NANO;
    let offset_hour = datetime.offset().local_minus_utc() / SEC_IN_HOUR;
    let sign = if offset_hour >= 0 { PLUS } else { MIN };

    write!(
        w,
        "{date:?}T{h:02}:{m:02}:{s:02}.{ms:06}{sign}{offset_hour:02}:00"
    )?;

    Ok(())
}

/// Write a UTF8 string with a BOM prefixed as stated in the spec
fn write_str_msg<W: io::Write>(w: &mut W, s: &str) -> io::Result<()> {
    if !s.is_empty() {
        // the BOM is prefixed by an ASCII space
        const BOM: [u8; 4] = [0x20, 0xEF, 0xBB, 0xBF];

        w.write_all(&BOM)?;
        w.write_all(s.as_bytes())?;
    }

    Ok(())
}

const NILVALUE: &str = "-";

/// The VERSION field denotes the version of the syslog protocol
/// specification. The version number MUST be incremented for any new
/// syslog protocol specification that changes any part of the HEADER
/// format.
///
/// [spec](https://datatracker.ietf.org/doc/html/rfc5424#section-6.2.2)
const VERSION: &str = "1";

#[cfg(feature = "chrono")]
type ChronoLocalTime = chrono::DateTime<chrono::Local>;

/// The TIMESTAMP field is a formalized timestamp derived from [RFC3339](https://datatracker.ietf.org/doc/html/rfc3339).
///
/// Whereas [RFC3339](https://datatracker.ietf.org/doc/html/rfc3339) makes allowances for multiple syntaxes,
/// the Syslog 5424 spec imposes further restrictions. The TIMESTAMP value MUST
/// follow these restrictions:
///
/// *  The "T" and "Z" characters in this syntax MUST be upper case.
/// *  Usage of the "T" character is REQUIRED.
/// *  Leap seconds MUST NOT be used.
///
/// A syslog application MUST use the NILVALUE as TIMESTAMP if the syslog
/// application is incapable of obtaining system time.
///
/// [spec](https://datatracker.ietf.org/doc/html/rfc5424#section-6.2.3)
pub enum Timestamp<'a> {
    /// Provide a datatime to be formatted.
    /// A custom formatter is used that does not perform any heap allcations
    #[cfg(feature = "chrono")]
    Chrono(&'a ChronoLocalTime),
    /// The formatter will create a new chrono::DateTime<Local>
    /// A custom formatter is used that does not perform any heap allcations
    #[cfg(feature = "chrono")]
    CreateChronoLocal,
    /// Provide a preformatted timestamp.
    /// This string is not validated. The onus is on the provider to verify it as an RFC3339 timestamp
    /// See the [Timestamp] docs above for details on how to format a timestamp.
    PreformattedStr(&'a str),
    /// Provide a preformatted timestamp.
    /// This string is not validated. The onus is on the provider to verify it as an RFC3339 timestamp
    /// See the [Timestamp] docs above for details on how to format a timestamp.
    PreformattedString(String),
    /// No timestamp can be provided.
    None,
}

impl<'a> From<&'a str> for Timestamp<'a> {
    fn from(s: &'a str) -> Self {
        Self::PreformattedStr(s)
    }
}

impl<'a> From<String> for Timestamp<'a> {
    fn from(s: String) -> Self {
        Self::PreformattedString(s)
    }
}

#[cfg(feature = "chrono")]
impl<'a> From<&'a ChronoLocalTime> for Timestamp<'a> {
    fn from(datetime: &'a ChronoLocalTime) -> Self {
        Self::Chrono(datetime)
    }
}

/// The HOSTNAME field identifies the machine that originally sent the syslog message.
///
/// The HOSTNAME field SHOULD contain the hostname and the domain name of
/// the originator in the format specified in STD 13 [RFC1034](https://datatracker.ietf.org/doc/html/rfc1034).
/// This format is called a Fully Qualified Domain Name (FQDN).
///
/// In practice, not all syslog applications are able to provide an FQDN.
/// As such, other values MAY also be present in HOSTNAME. Below are
/// provisions for using other values in such situations. A syslog
/// application SHOULD provide the most specific available value first.
/// The order of preference for the contents of the HOSTNAME field is as
/// follows:
///
/// 1. FQDN
/// 2. Static IP address
/// 3. hostname
/// 4. Dynamic IP address
/// 5. the NILVALUE
///
/// If an IPv4 address is used, it MUST be in the format of the dotted
/// decimal notation as used in STD 13 [RFC1035](https://datatracker.ietf.org/doc/html/rfc1035).
/// If an IPv6 address is used, a valid textual representation as described in
/// [RFC4291, Section 2.2](https://datatracker.ietf.org/doc/html/rfc4291#section-2.2), MUST be used.
///
/// Syslog applications SHOULD consistently use the same value in the
/// HOSTNAME field for as long as possible.
///
/// The NILVALUE SHOULD only be used when the syslog application has no
/// way to obtain its real hostname. This situation is considered highly
/// unlikely.
///
/// [spec](https://datatracker.ietf.org/doc/html/rfc5424#section-6.2.4)
type Hostname = str;

/// The APP-NAME field SHOULD identify the device or application that
/// originated the message. It is a string without further semantics.
/// It is intended for filtering messages on a relay or collector.
///
/// The NILVALUE MAY be used when the syslog application has no idea of
/// its APP-NAME or cannot provide that information. It may be that a
/// device is unable to provide that information either because of a
/// local policy decision, or because the information is not available,
/// or not applicable, on the device.
///
/// This field MAY be operator-assigned.
///
/// [spec](https://datatracker.ietf.org/doc/html/rfc5424#section-6.2.5)
type AppName = str;

/// PROCID is a value that is included in the message, having no
/// interoperable meaning, except that a change in the value indicates
/// there has been a discontinuity in syslog reporting. The field does
/// not have any specific syntax or semantics; the value is
/// implementation-dependent and/or operator-assigned. The NILVALUE MAY
/// be used when no value is provided.
///
/// The PROCID field is often used to provide the process name or process
/// ID associated with a syslog system. The NILVALUE might be used when
/// a process ID is not available. On an embedded system without any
/// operating system process ID, PROCID might be a reboot ID.
///
/// PROCID can enable log analyzers to detect discontinuities in syslog
/// reporting by detecting a change in the syslog process ID. However,
/// PROCID is not a reliable identification of a restarted process since
/// the restarted syslog process might be assigned the same process ID as
/// the previous syslog process.
///
/// PROCID can also be used to identify which messages belong to a group
/// of messages. For example, an SMTP mail transfer agent might put its
/// SMTP transaction ID into PROCID, which would allow the collector or
/// relay to group messages based on the SMTP transaction.
///
/// [spec](https://datatracker.ietf.org/doc/html/rfc5424#section-6.2.6)
type ProcId = str;

/// The MSGID SHOULD identify the type of message. For example, a
/// firewall might use the MSGID "TCPIN" for incoming TCP traffic and the
/// MSGID "TCPOUT" for outgoing TCP traffic. Messages with the same
/// MSGID should reflect events of the same semantics. The MSGID itself
/// is a string without further semantics. It is intended for filtering
/// messages on a relay or collector.
///
/// [spec](https://datatracker.ietf.org/doc/html/rfc5424#section-6.2.7)
type MsgId = str;

/// The MSG part contains a free-form message that provides information
/// about the event.
///
/// The character set used in MSG SHOULD be UNICODE, encoded using UTF-8
/// as specified in [RFC3629](https://datatracker.ietf.org/doc/html/rfc3629).
/// If the syslog application cannot encode
/// the MSG in Unicode, it MAY use any other encoding.
///
/// The syslog application SHOULD avoid octet values below 32 (the
/// traditional US-ASCII control character range except DEL). These
/// values are legal, but a syslog application MAY modify these
/// characters upon reception. For example, it might change them into an
/// escape sequence (e.g., value 0 may be changed to "\0"). A syslog
/// application SHOULD NOT modify any other octet values.
///
/// If a syslog application encodes MSG in UTF-8, the string MUST start
/// with the Unicode byte order mask (BOM), which for UTF-8 is ABNF
/// %xEF.BB.BF. The syslog application MUST encode in the "shortest
/// form" and MAY use any valid UTF-8 sequence.
///
/// [spec](https://datatracker.ietf.org/doc/html/rfc5424#section-6.4)
pub enum Msg<'a> {
    /// A BOM will be prefixed to UTF8 encoded strings
    Utf8Str(&'a str),
    /// A BOM will be prefixed to UTF8 encoded strings
    Utf8String(String),
    /// Bytes not encoded as Unicode not be prefixed by a BOM
    NonUnicodeBytes(&'a [u8]),
    /// Accepting fmt::Arguments can make life easier when working with logging frameworks
    FmtArguments(fmt::Arguments<'a>),
    /// Accepting fmt::Arguments can make life easier when working with logging frameworks
    FmtArgumentsRef(&'a fmt::Arguments<'a>),
}

impl<'a> From<&'a str> for Msg<'a> {
    fn from(s: &'a str) -> Self {
        Self::Utf8Str(s)
    }
}

impl<'a> From<String> for Msg<'a> {
    fn from(s: String) -> Self {
        Self::Utf8String(s)
    }
}

impl<'a> From<&'a [u8]> for Msg<'a> {
    fn from(bytes: &'a [u8]) -> Self {
        Self::NonUnicodeBytes(bytes)
    }
}

impl<'a> From<fmt::Arguments<'a>> for Msg<'a> {
    fn from(args: fmt::Arguments<'a>) -> Self {
        Self::FmtArguments(args)
    }
}

impl<'a> From<&'a fmt::Arguments<'a>> for Msg<'a> {
    fn from(args: &'a fmt::Arguments<'a>) -> Self {
        Self::FmtArgumentsRef(args)
    }
}

/// STRUCTURED-DATA provides a mechanism to express information in a well
/// defined, easily parseable and interpretable data format. There are
/// multiple usage scenarios. For example, it may express meta-
/// information about the syslog message or application-specific
/// information such as traffic counters or IP addresses.
///
/// STRUCTURED-DATA can contain zero, one, or multiple structured data
/// elements, [SdElement].
///
/// In case of zero structured data elements, the STRUCTURED-DATA field
/// MUST contain the NILVALUE.
///
/// The character set used in STRUCTURED-DATA MUST be seven-bit ASCII in
/// an eight-bit field as described in [RFC5234](https://datatracker.ietf.org/doc/html/rfc5234).
/// These are the ASCII codes as defined in "USA Standard Code for Information Interchange"
/// [ANSI.X3-4.1968](https://datatracker.ietf.org/doc/html/rfc5424#ref-ANSI.X3-4.1968).
/// An exception is the PARAM-VALUE field, in which UTF-8 encoding MUST be used.
///
/// [spec](https://datatracker.ietf.org/doc/html/rfc5424#section-6.3)
type StructuredData<'a> = Vec<SdElement<'a>>;

/// An SD-ELEMENT consists of a name and parameter name-value pairs. The
/// name is referred to as SD-ID. The name-value pairs are referred to
/// as [SdParam].
///
/// [spec](https://datatracker.ietf.org/doc/html/rfc5424#section-6.3.1)
#[derive(Debug)]
struct SdElement<'a> {
    id: &'a SdId,
    params: Vec<SdParam<'a>>,
}

/// [SdId]s are case-sensitive and uniquely identify the type and purpose
/// of the [SdElement]. The same [SdId] MUST NOT exist more than once in a
/// message.
///
/// There are two formats for [SdId] names:
///
/// - Names that do not contain an at-sign ("@", ABNF %d64) are reserved
/// to be assigned by IETF Review as described in BCP26 [RFC5226](https://datatracker.ietf.org/doc/html/rfc5226).
/// Currently, these are the names defined in Section 7. Names of
/// this format are only valid if they are first registered with the
/// IANA. Registered names MUST NOT contain an at-sign ('@', ABNF %d64),
/// an equal-sign ('=', ABNF %d61), a closing brace (']', ABNF
/// %d93), a quote-character ('"', ABNF %d34), whitespace, or control
/// characters (ASCII code 127 and codes 32 or less).
///
/// - Anyone can define additional SD-IDs using names in the format
/// `name@<private enterprise number>`, e.g., "ourSDID@32473". The
/// format of the part preceding the at-sign is not specified;
/// however, these names MUST be printable US-ASCII strings, and MUST
/// NOT contain an at-sign ('@', ABNF %d64), an equal-sign ('=', ABNF
/// %d61), a closing brace (']', ABNF %d93), a quote-character ('"',
/// ABNF %d34), whitespace, or control characters. The part following
/// the at-sign MUST be a private enterprise number as specified in
/// Section 7.2.2. Please note that throughout this document the
/// value of 32473 is used for all private enterprise numbers. This
/// value has been reserved by IANA to be used as an example number in
/// documentation. Implementors will need to use their own private
/// enterprise number for the enterpriseId parameter, and when
/// creating locally extensible SD-ID names.
///
/// [spec](https://datatracker.ietf.org/doc/html/rfc5424#section-6.3.2)
type SdId = str;

/// Each SD-PARAM consists of a name, referred to as PARAM-NAME, and a
/// value, referred to as PARAM-VALUE.
///
/// PARAM-NAME is case-sensitive. IANA controls all PARAM-NAMEs, with
/// the exception of those in SD-IDs whose names contain an at-sign. The
/// PARAM-NAME scope is within a specific SD-ID. Thus, equally named
/// PARAM-NAME values contained in two different SD-IDs are not the same.
///
/// To support international characters, the PARAM-VALUE field MUST be
/// encoded using UTF-8. A syslog application MAY issue any valid UTF-8
/// sequence. A syslog application MUST accept any valid UTF-8 sequence
/// in the "shortest form". It MUST NOT fail if control characters are
/// present in PARAM-VALUE. The syslog application MAY modify messages
/// containing control characters (e.g., by changing an octet with value
/// 0 (USASCII NUL) to the four characters "#000"). For the reasons
/// outlined in UNICODE TR36 [UNICODE-TR36, section 3.1](https://datatracker.ietf.org/doc/html/rfc5424#ref-UNICODE-TR36),
/// an originator MUST encode messages in the "shortest form" and a collector or relay
/// MUST NOT interpret messages in the "non-shortest form".
///
/// Inside PARAM-VALUE, the characters '"' (ABNF %d34), '\' (ABNF %d92),
/// and ']' (ABNF %d93) MUST be escaped. This is necessary to avoid
/// parsing errors. Escaping ']' would not strictly be necessary but is
/// REQUIRED by this specification to avoid syslog application
/// implementation errors. Each of these three characters MUST be
/// escaped as '\"', '\\', and '\]' respectively. The backslash is used
/// for control character escaping for consistency with its use for
/// escaping in other parts of the syslog message as well as in
/// traditional syslog.
///
/// A backslash ('\') followed by none of the three described characters
/// is considered an invalid escape sequence. In this case, the
/// backslash MUST be treated as a regular backslash and the following
/// character as a regular character. Thus, the invalid sequence MUST
/// not be altered.
///
/// An SD-PARAM MAY be repeated multiple times inside an SD-ELEMENT.
///
/// [spec](https://datatracker.ietf.org/doc/html/rfc5424#section-6.3.3)
type SdParam<'a> = (ParamName<'a>, ParamValue<'a>);
type ParamName<'a> = &'a str;
type ParamValue<'a> = &'a str;

fn data_to_string(data: Vec<SdElement<'_>>) -> String {
    let elements = data
        .into_iter()
        .map(|elem| {
            let SdElement { id, params } = elem;

            if params.is_empty() {
                format!("[{id}]")
            } else {
                let params = params
                    .into_iter()
                    .map(|(name, value)| format!("{name}=\"{value}\""))
                    .collect::<Vec<_>>()
                    .join(" ");

                format!("[{id} {params}]")
            }
        })
        .collect::<Vec<_>>();

    elements.join("")
}

fn encode_priority(severity: Severity, facility: Facility) -> Priority {
    facility as u8 | severity as u8
}

#[cfg(test)]
mod tests {
    use std::io::ErrorKind;

    use assert_matches::assert_matches;

    use super::*;

    #[test]
    #[cfg(feature = "chrono")]
    fn should_format_date_like_chrono() {
        let datetime = chrono::Local::now();
        let use_z = false;
        let chrono_s = datetime.to_rfc3339_opts(chrono::SecondsFormat::Micros, use_z);

        let mut buf = Vec::with_capacity(32);
        format_chrono_datetime(&mut buf, &datetime).unwrap();
        let s = String::from_utf8(buf).unwrap();

        assert_eq!(
            chrono_s, s,
            "syslog-fmt date formatter should be char for char equal to Chrono"
        );
    }

    #[test]
    fn should_format_message_without_msg_id() {
        let hostname = "mymachine.example.com";
        let app_name = "su";
        let severity = Severity::Crit;
        let msg = "'su root' failed for lonvick on /dev/pts/8";
        let fmt = Config {
            facility: Facility::Auth,
            hostname: hostname.into(),
            app_name: app_name.into(),
            proc_id: None,
        }
        .into_formatter();
        let mut buf = vec![];
        fmt.format(&mut buf, severity, Timestamp::CreateChronoLocal, msg, None)
            .unwrap();

        let parts = parse_syslog_message(&buf);

        assert_matches!(
            parts,
            Parts {
                prio: "<34>1",
                timestamp: _,
                hostname,
                app_name,
                proc_id: NILVALUE,
                msg_id: NILVALUE,
                data: NILVALUE,
                msg
            } if hostname == hostname && app_name == app_name && msg == msg
        );
    }

    #[test]
    fn should_format_message_with_msg_id() {
        let hostname = "mymachine.example.com";
        let app_name = "su";
        let severity = Severity::Crit;
        let msg_id = "ID47";
        let msg = "'su root' failed for lonvick on /dev/pts/8";
        let fmt = Config {
            facility: Facility::Auth,
            hostname: hostname.into(),
            app_name: app_name.into(),
            proc_id: None,
        }
        .into_formatter();
        let mut buf = vec![];

        fmt.format(
            &mut buf,
            severity,
            Timestamp::CreateChronoLocal,
            msg,
            Some(msg_id),
        )
        .unwrap();

        let parts = parse_syslog_message(&buf);
        assert_matches!(
            parts,
            Parts {
                prio: "<34>1",
                timestamp: _,
                hostname: "mymachine.example.com",
                app_name: "su",
                proc_id: NILVALUE,
                msg_id,
                data: NILVALUE,
                msg
            } if msg_id == msg_id && msg == msg
        );
    }

    #[test]
    fn should_format_message_with_structured_data_and_message() {
        let hostname = "mymachine.example.com";
        let app_name = "evntslog";
        let severity = Severity::Notice;
        let msg_id = "ID47";
        let msg = "An application event log entry...";
        let fmt = Config {
            facility: Facility::Local4,
            hostname: hostname.into(),
            app_name: app_name.into(),
            proc_id: None,
        }
        .into_formatter();
        let mut buf = vec![];

        fmt.format_with_data(
            &mut buf,
            severity,
            Timestamp::CreateChronoLocal,
            msg,
            Some(msg_id),
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

        let parts = parse_syslog_message(&buf);

        assert_matches!(
            parts,
            Parts {
                prio: "<165>1",
                timestamp: _,
                hostname: "mymachine.example.com",
                app_name: "evntslog",
                proc_id: NILVALUE,
                msg_id,
                data: r#"[exampleSDID@32473 iut="3" eventSource="Application" eventID="1011"]"#,
                msg
            } if hostname == hostname && app_name == app_name && msg_id == msg_id && msg == msg
        );
    }

    #[test]
    fn should_format_message_with_structured_data_and_no_message() {
        let hostname = "mymachine.example.com";
        let app_name = "evntslog";
        let severity = Severity::Notice;
        let msg_id = "ID47";
        let msg = "";
        let fmt = Config {
            facility: Facility::Local4,
            hostname: hostname.into(),
            app_name: app_name.into(),
            proc_id: None,
        }
        .into_formatter();
        let mut buf = vec![];

        fmt.format_with_data(
            &mut buf,
            severity,
            Timestamp::CreateChronoLocal,
            msg,
            Some(msg_id),
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

        let parts = parse_syslog_message(&buf);

        assert_matches!(
            parts,
            Parts {
                prio: "<165>1",
                timestamp: _,
                hostname,
                app_name,
                proc_id: NILVALUE,
                msg_id,
                data: r#"[exampleSDID@32473 iut="3" eventSource="Application" eventID="1011"]"#,
                msg
            } if hostname == hostname && app_name == app_name && msg_id == msg_id && msg == msg
        );
    }

    #[test]
    fn should_truncate_message_to_buffer_size() {
        use arrayvec::ArrayVec;

        let timestamp = "1985-04-12T23:20:50.52Z";
        let hostname = "mymachine.example.com";
        let app_name = "su";
        let severity = Severity::Crit;
        let msg = "'su root' failed for lonvick on /dev/pts/8";
        let fmt = Config {
            facility: Facility::Auth,
            hostname: hostname.into(),
            app_name: app_name.into(),
            proc_id: None,
        }
        .into_formatter();
        let mut buf = ArrayVec::<u8, 100>::new();

        let err = fmt
            .format_items(&mut buf, severity, timestamp, msg, None, None)
            .unwrap_err();

        assert_eq!(
            err.kind(),
            ErrorKind::WriteZero,
            "The given buffer is too small for the message. But the formatter should write as much as possible"
        );

        let parts = parse_syslog_message(&buf);

        assert_matches!(
            parts,
            Parts {
                prio: "<34>1",
                timestamp,
                hostname,
                app_name,
                proc_id: NILVALUE,
                msg_id: NILVALUE,
                data: NILVALUE,
                msg: "'su root' failed for lonvick on /dev"
            } if timestamp == timestamp && hostname == hostname && app_name == app_name
        );
    }

    #[test]
    fn should_fmt_structured_data() {
        assert_eq!(data_to_string(vec![]), "");

        assert_eq!(
            data_to_string(vec![SdElement {
                id: "first",
                params: vec![],
            }]),
            "[first]"
        );

        assert_eq!(
            data_to_string(vec![
                SdElement {
                    id: "first",
                    params: vec![],
                },
                SdElement {
                    id: "second",
                    params: vec![],
                }
            ]),
            "[first][second]"
        );

        assert_eq!(
            data_to_string(vec![SdElement {
                id: "first",
                params: vec![("p-one", "pv-one")],
            }]),
            r#"[first p-one="pv-one"]"#
        );

        assert_eq!(
            data_to_string(vec![SdElement {
                id: "first",
                params: vec![("p-one", "pv-one"), ("p-two", "pv-two")],
            }]),
            r#"[first p-one="pv-one" p-two="pv-two"]"#
        );

        assert_eq!(
            data_to_string(vec![
                SdElement {
                    id: "first",
                    params: vec![("p-one", "pv-one"), ("p-two", "pv-two")],
                },
                SdElement {
                    id: "second",
                    params: vec![("p-one", "pv-one"), ("p-two", "pv-two")],
                }
            ]),
            r#"[first p-one="pv-one" p-two="pv-two"][second p-one="pv-one" p-two="pv-two"]"#
        );
    }

    #[derive(Debug)]
    struct Parts<'a> {
        prio: &'a str,
        timestamp: &'a str,
        hostname: &'a str,
        app_name: &'a str,
        proc_id: &'a str,
        msg_id: &'a str,
        data: &'a str,
        msg: &'a str,
    }

    fn parse_syslog_message(buf: &[u8]) -> Parts<'_> {
        const DELIM: char = ' ';
        const UTF8_BOM: char = '\u{feff}';

        let s = std::str::from_utf8(buf).unwrap();
        let (prio, s) = s.split_once(DELIM).unwrap();
        let (timestamp, s) = s.split_once(DELIM).unwrap();
        let (hostname, s) = s.split_once(DELIM).unwrap();
        let (app_name, s) = s.split_once(DELIM).unwrap();
        let (proc_id, s) = s.split_once(DELIM).unwrap();
        let (msg_id, s) = s.split_once(DELIM).unwrap();

        let (data, msg) = if s.starts_with('[') {
            let index = s.rfind(']').expect("There should be a closing delimiter");
            let (data, s) = s.split_at(index + 1);
            let s = s.trim();

            (data, s.strip_prefix(UTF8_BOM).unwrap_or(s))
        } else {
            let (data, s) = s.split_once(DELIM).unwrap();
            (data, s.strip_prefix(UTF8_BOM).unwrap_or(s))
        };

        Parts {
            prio,
            timestamp,
            hostname,
            app_name,
            proc_id,
            msg_id,
            data,
            msg,
        }
    }

    // See: <https://datatracker.ietf.org/doc/html/rfc5424#section-6.5>
    #[test]
    fn should_parse_example_1_with_no_structured_data() {
        let msg_buf= b"<34>1 2003-10-11T22:14:15.003Z mymachine.example.com su - ID47 - 'su root' failed for lonvick on /dev/pts/8";
        let parts = parse_syslog_message(msg_buf);

        assert_matches!(
            parts,
            Parts {
                prio: "<34>1",
                timestamp: "2003-10-11T22:14:15.003Z",
                hostname: "mymachine.example.com",
                app_name: "su",
                proc_id: NILVALUE,
                msg_id: "ID47",
                data: NILVALUE,
                msg: "'su root' failed for lonvick on /dev/pts/8"
            }
        );
    }

    // See: <https://datatracker.ietf.org/doc/html/rfc5424#section-6.5>
    #[test]
    fn should_parse_example_2_with_no_structured_data() {
        let msg_buf= b"<165>1 2003-08-24T05:14:15.000003-07:00 192.0.2.1 myproc 8710 - - %% It's time to make the do-nuts.";
        let parts = parse_syslog_message(msg_buf);

        assert_matches!(
            parts,
            Parts {
                prio: "<165>1",
                timestamp: "2003-08-24T05:14:15.000003-07:00",
                hostname: "192.0.2.1",
                app_name: "myproc",
                proc_id: "8710",
                msg_id: NILVALUE,
                data: NILVALUE,
                msg: "%% It's time to make the do-nuts."
            }
        );
    }

    // See: <https://datatracker.ietf.org/doc/html/rfc5424#section-6.5>
    #[test]
    fn should_parse_example_3_with_structured_data() {
        let msg_buf= br#"<165>1 2003-10-11T22:14:15.003Z mymachine.example.com evntslog - ID47 [exampleSDID@32473 iut="3" eventSource="Application" eventID="1011"] An application event log entry..."#;
        let parts = parse_syslog_message(msg_buf);

        assert_matches!(
            parts,
            Parts {
                prio: "<165>1",
                timestamp: "2003-10-11T22:14:15.003Z",
                hostname: "mymachine.example.com",
                app_name: "evntslog",
                proc_id: NILVALUE,
                msg_id: "ID47",
                data: r#"[exampleSDID@32473 iut="3" eventSource="Application" eventID="1011"]"#,
                msg: "An application event log entry..."
            }
        );
    }

    // See: <https://datatracker.ietf.org/doc/html/rfc5424#section-6.5>
    #[test]
    fn should_parse_example_4_structured_data_only() {
        let msg_buf= br#"<165>1 2003-10-11T22:14:15.003Z mymachine.example.com evntslog - ID47 [exampleSDID@32473 iut="3" eventSource="Application" eventID="1011"][examplePriority@32473 class="high"]"#;
        let parts = parse_syslog_message(msg_buf);

        assert_matches!(
            parts,
            Parts {
                prio: "<165>1",
                timestamp: "2003-10-11T22:14:15.003Z",
                hostname: "mymachine.example.com",
                app_name: "evntslog",
                proc_id: NILVALUE,
                msg_id: "ID47",
                data: r#"[exampleSDID@32473 iut="3" eventSource="Application" eventID="1011"][examplePriority@32473 class="high"]"#,
                msg: ""
            }
        );
    }
}
