//! Device Status Report (DSR) sequences.
//!
//! These sequences allow applications to request status information from
//! the terminal.
//!
//! See <https://terminalguide.namepad.de/seq/csi_sn/> for details.

use vtenc::{ConstEncode, ConstEncodedLen, Encode, EncodeError, format_csi, write_csi};

/// Request Operating Status (`DSR`).
///
/// Request the terminal's operating status.
///
/// The terminal always replies with:
///
/// `CSI 0 n` (operating correctly)
///
/// This is a basic status check that indicates the terminal is
/// functioning and able to respond to commands.
///
/// See <https://terminalguide.namepad.de/seq/csi_sn-5/> for terminal
/// support specifics.
pub struct RequestOperatingStatus;

impl ConstEncode for RequestOperatingStatus {
    const STR: &'static str = format_csi!("5n");
}

/// Operating Status Report (`DSR`).
///
/// Response from the terminal to [`RequestOperatingStatus`].
///
/// This report indicates that the terminal is operating correctly.
/// The status code is always 0.
///
/// See <https://terminalguide.namepad.de/seq/csi_sn-5/> for terminal
/// support specifics.
pub struct OperatingStatusReport;

impl ConstEncode for OperatingStatusReport {
    const STR: &'static str = format_csi!("0n");
}

/// Request Operating Status (private mode) (`DSR`).
///
/// Request the terminal's operating status using the DEC private mode
/// variant.
///
/// The terminal replies with:
///
/// `CSI ? 0 n` (operating correctly)
///
/// See <https://terminalguide.namepad.de/seq/csi_sn/> for terminal
/// support specifics.
pub struct RequestOperatingStatusPrivate;

impl ConstEncode for RequestOperatingStatusPrivate {
    const STR: &'static str = format_csi!("?5n");
}

/// Operating Status Report (private mode) (`DSR`).
///
/// Response from the terminal to [`RequestOperatingStatusPrivate`].
///
/// This report indicates that the terminal is operating correctly.
/// The status code is always 0.
pub struct OperatingStatusReportPrivate;

impl ConstEncode for OperatingStatusReportPrivate {
    const STR: &'static str = format_csi!("?0n");
}

/// Request Printer Status (`DSR`).
///
/// Request the printer status (historically DSR 13).
///
/// In modern terminals, this typically replies with:
///
/// `CSI ? 13 n` (no printer)
///
/// See <https://terminalguide.namepad.de/seq/csi_sn/> for terminal
/// support specifics.
pub struct RequestPrinterStatus;

impl ConstEncode for RequestPrinterStatus {
    const STR: &'static str = format_csi!("?15n");
}

/// Printer Status Report (`DSR`).
///
/// Response indicating printer status.
///
/// The status code typically indicates:
/// - 10: printer ready
/// - 11: printer not ready
/// - 13: no printer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrinterStatusReport {
    /// Printer status code.
    pub status: u8,
}

impl PrinterStatusReport {
    /// Create a printer status report with the specified status code.
    #[must_use]
    pub const fn new(status: u8) -> Self {
        Self { status }
    }

    /// Create a report indicating no printer is available.
    #[must_use]
    pub const fn no_printer() -> Self {
        Self { status: 13 }
    }

    /// Create a report indicating the printer is ready.
    #[must_use]
    pub const fn ready() -> Self {
        Self { status: 10 }
    }

    /// Create a report indicating the printer is not ready.
    #[must_use]
    pub const fn not_ready() -> Self {
        Self { status: 11 }
    }
}

impl ConstEncodedLen for PrinterStatusReport {
    // CSI (2) + "?" (1) + max status (2) + "n" (1) = 6
    const ENCODED_LEN: usize = 6;
}

impl Encode for PrinterStatusReport {
    #[inline]
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        write_csi!(buf; "?", self.status, "n")
    }
}

/// Request User Defined Key Status (`DSR`).
///
/// Request the status of user-defined keys (historically DSR 25).
///
/// In modern terminals, this typically replies with:
///
/// `CSI ? 20 n` (UDK locked)
///
/// See <https://terminalguide.namepad.de/seq/csi_sn/> for terminal
/// support specifics.
pub struct RequestUdkStatus;

impl ConstEncode for RequestUdkStatus {
    const STR: &'static str = format_csi!("?25n");
}

/// User Defined Key Status Report (`DSR`).
///
/// Response indicating user-defined key status.
///
/// The status code typically indicates:
/// - 20: UDK locked
/// - 21: UDK unlocked
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UdkStatusReport {
    /// UDK status code.
    pub status: u8,
}

impl UdkStatusReport {
    /// Create a UDK status report with the specified status code.
    #[must_use]
    pub const fn new(status: u8) -> Self {
        Self { status }
    }

    /// Create a report indicating UDK is locked.
    #[must_use]
    pub const fn locked() -> Self {
        Self { status: 20 }
    }

    /// Create a report indicating UDK is unlocked.
    #[must_use]
    pub const fn unlocked() -> Self {
        Self { status: 21 }
    }
}

impl ConstEncodedLen for UdkStatusReport {
    // CSI (2) + "?" (1) + max status (2) + "n" (1) = 6
    const ENCODED_LEN: usize = 6;
}

impl Encode for UdkStatusReport {
    #[inline]
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        write_csi!(buf; "?", self.status, "n")
    }
}

/// Request Keyboard Status (`DSR`).
///
/// Request the keyboard status (historically DSR 26).
///
/// In modern terminals, this typically replies with:
///
/// `CSI ? 27 ; keyboard_dialect n`
///
/// See <https://terminalguide.namepad.de/seq/csi_sn/> for terminal
/// support specifics.
pub struct RequestKeyboardStatus;

impl ConstEncode for RequestKeyboardStatus {
    const STR: &'static str = format_csi!("?26n");
}

/// Keyboard Status Report (`DSR`).
///
/// Response indicating keyboard status and dialect.
///
/// The status code is typically 27, and the dialect identifies the
/// keyboard language/layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyboardStatusReport {
    /// Keyboard dialect code.
    pub dialect: u8,
}

impl KeyboardStatusReport {
    /// Create a keyboard status report with the specified dialect.
    #[must_use]
    pub const fn new(dialect: u8) -> Self {
        Self { dialect }
    }
}

impl ConstEncodedLen for KeyboardStatusReport {
    // CSI (2) + "?" (1) + "27" (2) + ";" (1) + max dialect (3) + "n"
    // (1) = 10
    const ENCODED_LEN: usize = 10;
}

impl Encode for KeyboardStatusReport {
    #[inline]
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        write_csi!(buf; "?27;", self.dialect, "n")
    }
}

/// Request DEC Locator Status (`DSR`).
///
/// Request the status of the DEC locator (mouse).
///
/// The terminal replies with the locator status.
///
/// See <https://terminalguide.namepad.de/seq/csi_sn/> for terminal
/// support specifics.
pub struct RequestLocatorStatus;

impl ConstEncode for RequestLocatorStatus {
    const STR: &'static str = format_csi!("?55n");
}

/// DEC Locator Status Report (`DSR`).
///
/// Response indicating the status of the DEC locator.
///
/// The status code indicates:
/// - 50: no locator
/// - 53: locator available
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LocatorStatusReport {
    /// Locator status code.
    pub status: u8,
}

impl LocatorStatusReport {
    /// Create a locator status report with the specified status code.
    #[must_use]
    pub const fn new(status: u8) -> Self {
        Self { status }
    }

    /// Create a report indicating no locator is available.
    #[must_use]
    pub const fn no_locator() -> Self {
        Self { status: 50 }
    }

    /// Create a report indicating a locator is available.
    #[must_use]
    pub const fn available() -> Self {
        Self { status: 53 }
    }
}

impl ConstEncodedLen for LocatorStatusReport {
    // CSI (2) + "?" (1) + max status (2) + "n" (1) = 6
    const ENCODED_LEN: usize = 6;
}

impl Encode for LocatorStatusReport {
    #[inline]
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        write_csi!(buf; "?", self.status, "n")
    }
}

/// Request DEC Locator Type (`DSR`).
///
/// Request the type of the DEC locator (mouse).
///
/// The terminal replies with the locator type.
///
/// See <https://terminalguide.namepad.de/seq/csi_sn/> for terminal
/// support specifics.
pub struct RequestLocatorType;

impl ConstEncode for RequestLocatorType {
    const STR: &'static str = format_csi!("?56n");
}

/// DEC Locator Type Report (`DSR`).
///
/// Response indicating the type of the DEC locator.
///
/// The type code indicates the locator device type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LocatorTypeReport {
    /// Locator type code.
    pub locator_type: u8,
}

impl LocatorTypeReport {
    /// Create a locator type report with the specified type code.
    #[must_use]
    pub const fn new(locator_type: u8) -> Self {
        Self { locator_type }
    }
}

impl ConstEncodedLen for LocatorTypeReport {
    // CSI (2) + "?" (1) + "57" (2) + ";" (1) + max type (3) + "n" (1)
    // = 10
    const ENCODED_LEN: usize = 10;
}

impl Encode for LocatorTypeReport {
    #[inline]
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        write_csi!(buf; "?57;", self.locator_type, "n")
    }
}

/// Request Data Integrity Status (`DSR`).
///
/// Request data integrity status (historically DSR 75).
///
/// In modern terminals, this typically replies with:
///
/// `CSI ? 70 n` (ready, no malfunction detected)
///
/// See <https://terminalguide.namepad.de/seq/csi_sn/> for terminal
/// support specifics.
pub struct RequestDataIntegrityStatus;

impl ConstEncode for RequestDataIntegrityStatus {
    const STR: &'static str = format_csi!("?75n");
}

/// Data Integrity Status Report (`DSR`).
///
/// Response indicating data integrity status.
///
/// The status code typically indicates:
/// - 70: ready, no malfunction detected
/// - 71: malfunction detected
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DataIntegrityStatusReport {
    /// Data integrity status code.
    pub status: u8,
}

impl DataIntegrityStatusReport {
    /// Create a data integrity status report with the specified status
    /// code.
    #[must_use]
    pub const fn new(status: u8) -> Self {
        Self { status }
    }

    /// Create a report indicating no malfunction was detected.
    #[must_use]
    pub const fn ready() -> Self {
        Self { status: 70 }
    }

    /// Create a report indicating a malfunction was detected.
    #[must_use]
    pub const fn malfunction() -> Self {
        Self { status: 71 }
    }
}

impl ConstEncodedLen for DataIntegrityStatusReport {
    // CSI (2) + "?" (1) + max status (2) + "n" (1) = 6
    const ENCODED_LEN: usize = 6;
}

impl Encode for DataIntegrityStatusReport {
    #[inline]
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        write_csi!(buf; "?", self.status, "n")
    }
}

/// Request Multiple Session Status (`DSR`).
///
/// Request multiple session status (historically DSR 85).
///
/// In modern terminals, this typically replies with:
///
/// `CSI ? 83 n` (not configured)
///
/// See <https://terminalguide.namepad.de/seq/csi_sn/> for terminal
/// support specifics.
pub struct RequestMultipleSessionStatus;

impl ConstEncode for RequestMultipleSessionStatus {
    const STR: &'static str = format_csi!("?85n");
}

/// Multiple Session Status Report (`DSR`).
///
/// Response indicating multiple session status.
///
/// The status code typically indicates:
/// - 80: sessions available
/// - 81: no sessions available
/// - 83: not configured for multiple sessions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MultipleSessionStatusReport {
    /// Multiple session status code.
    pub status: u8,
}

impl MultipleSessionStatusReport {
    /// Create a multiple session status report with the specified
    /// status code.
    #[must_use]
    pub const fn new(status: u8) -> Self {
        Self { status }
    }

    /// Create a report indicating sessions are available.
    #[must_use]
    pub const fn available() -> Self {
        Self { status: 80 }
    }

    /// Create a report indicating no sessions are available.
    #[must_use]
    pub const fn not_available() -> Self {
        Self { status: 81 }
    }

    /// Create a report indicating not configured for multiple
    /// sessions.
    #[must_use]
    pub const fn not_configured() -> Self {
        Self { status: 83 }
    }
}

impl ConstEncodedLen for MultipleSessionStatusReport {
    // CSI (2) + "?" (1) + max status (2) + "n" (1) = 6
    const ENCODED_LEN: usize = 6;
}

impl Encode for MultipleSessionStatusReport {
    #[inline]
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        write_csi!(buf; "?", self.status, "n")
    }
}

/// Request Macro Space Status (`DSR`).
///
/// Request the available macro space (historically DSR 62).
///
/// The terminal replies with the amount of available space.
///
/// See <https://terminalguide.namepad.de/seq/csi_sn/> for terminal
/// support specifics.
pub struct RequestMacroSpaceStatus;

impl ConstEncode for RequestMacroSpaceStatus {
    const STR: &'static str = format_csi!("?62n");
}

/// Macro Space Status Report (`DSR`).
///
/// Response indicating available macro space.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MacroSpaceStatusReport {
    /// Available macro space in bytes.
    pub space: u16,
}

impl MacroSpaceStatusReport {
    /// Create a macro space status report with the specified available
    /// space.
    #[must_use]
    pub const fn new(space: u16) -> Self {
        Self { space }
    }
}

impl ConstEncodedLen for MacroSpaceStatusReport {
    // CSI (2) + "?" (1) + "63" (2) + ";" (1) + max space (5) + "n"
    // (1) = 12
    const ENCODED_LEN: usize = 12;
}

impl Encode for MacroSpaceStatusReport {
    #[inline]
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        write_csi!(buf; "?63;", self.space, "n")
    }
}

/// Request Memory Checksum (`DSR`).
///
/// Request a memory checksum (historically DSR 63).
///
/// The terminal replies with a checksum of the specified memory
/// region.
///
/// See <https://terminalguide.namepad.de/seq/csi_sn/> for terminal
/// support specifics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RequestMemoryChecksum {
    /// Identifier for the memory region to checksum.
    pub id: u16,
}

impl RequestMemoryChecksum {
    /// Create a memory checksum request for the specified region.
    #[must_use]
    pub const fn new(id: u16) -> Self {
        Self { id }
    }
}

impl ConstEncodedLen for RequestMemoryChecksum {
    // CSI (2) + "?" (1) + "63" (2) + ";" (1) + max id (5) + "n" (1) =
    // 12
    const ENCODED_LEN: usize = 12;
}

impl Encode for RequestMemoryChecksum {
    #[inline]
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        write_csi!(buf; "?63;", self.id, "n")
    }
}

/// Memory Checksum Report (`DSR`).
///
/// Response containing the memory checksum.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemoryChecksumReport {
    /// Memory region identifier.
    pub id: u16,
    /// Checksum value.
    pub checksum: u16,
}

impl MemoryChecksumReport {
    /// Create a memory checksum report with the specified ID and
    /// checksum.
    #[must_use]
    pub const fn new(id: u16, checksum: u16) -> Self {
        Self { id, checksum }
    }
}

impl ConstEncodedLen for MemoryChecksumReport {
    // CSI (2) + "?" (1) + "63" (2) + ";" (1) + max id (5) + ";" (1) +
    // max checksum (5) + "n" (1) = 18
    const ENCODED_LEN: usize = 18;
}

impl Encode for MemoryChecksumReport {
    #[inline]
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        write_csi!(buf; "?63;", self.id, ";", self.checksum, "n")
    }
}
