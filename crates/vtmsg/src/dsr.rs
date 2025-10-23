//! Device Status Report (DSR) sequences.
//!
//! These sequences allow applications to request status information from
//! the terminal.
//!
//! See <https://terminalguide.namepad.de/seq/csi_sn/> for details.

use vtderive::csi;

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
#[csi(params = ["5"], finalbyte = 'n')]
pub struct RequestOperatingStatus;

/// Operating Status Report (`DSR`).
///
/// Response from the terminal to [`RequestOperatingStatus`].
///
/// This report indicates that the terminal is operating correctly.
/// The status code is always 0.
///
/// See <https://terminalguide.namepad.de/seq/csi_sn-5/> for terminal
/// support specifics.
#[csi(params = ["0"], finalbyte = 'n')]
pub struct OperatingStatusReport;

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
#[csi(private = '?', params = ["5"], finalbyte = 'n')]
pub struct RequestOperatingStatusPrivate;

/// Operating Status Report (private mode) (`DSR`).
///
/// Response from the terminal to [`RequestOperatingStatusPrivate`].
///
/// This report indicates that the terminal is operating correctly.
/// The status code is always 0.
#[csi(private = '?', params = ["0"], finalbyte = 'n')]
pub struct OperatingStatusReportPrivate;

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
#[csi(private = '?', params = ["15"], finalbyte = 'n')]
pub struct RequestPrinterStatus;

/// Printer Status Report (`DSR`).
///
/// Response indicating printer status.
///
/// The status code typically indicates:
/// - 10: printer ready
/// - 11: printer not ready
/// - 13: no printer
#[csi(private = '?', finalbyte = 'n')]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrinterStatusReport {
    /// Printer status code.
    pub status: u8,
}

impl PrinterStatusReport {
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
#[csi(private = '?', params = ["25"], finalbyte = 'n')]
pub struct RequestUdkStatus;

/// User Defined Key Status Report (`DSR`).
///
/// Response indicating user-defined key status.
///
/// The status code typically indicates:
/// - 20: UDK locked
/// - 21: UDK unlocked
#[csi(private = '?', finalbyte = 'n')]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UdkStatusReport {
    /// UDK status code.
    pub status: u8,
}

impl UdkStatusReport {
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
#[csi(private = '?', params = ["26"], finalbyte = 'n')]
pub struct RequestKeyboardStatus;

/// Keyboard Status Report (`DSR`).
///
/// Response indicating keyboard status and dialect.
///
/// The status code is typically 27, and the dialect identifies the
/// keyboard language/layout.
#[csi(private = '?', params = ["27"], finalbyte = 'n')]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyboardStatusReport {
    /// Keyboard dialect code.
    pub dialect: u8,
}

/// Request DEC Locator Status (`DSR`).
///
/// Request the status of the DEC locator (mouse).
///
/// The terminal replies with the locator status.
///
/// See <https://terminalguide.namepad.de/seq/csi_sn/> for terminal
/// support specifics.
#[csi(private = '?', params = ["55"], finalbyte = 'n')]
pub struct RequestLocatorStatus;

/// DEC Locator Status Report (`DSR`).
///
/// Response indicating the status of the DEC locator.
///
/// The status code indicates:
/// - 50: no locator
/// - 53: locator available
#[csi(private = '?', finalbyte = 'n')]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LocatorStatusReport {
    /// Locator status code.
    pub status: u8,
}

impl LocatorStatusReport {
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

/// Request DEC Locator Type (`DSR`).
///
/// Request the type of the DEC locator (mouse).
///
/// The terminal replies with the locator type.
///
/// See <https://terminalguide.namepad.de/seq/csi_sn/> for terminal
/// support specifics.
#[csi(private = '?', params = ["56"], finalbyte = 'n')]
pub struct RequestLocatorType;

/// DEC Locator Type Report (`DSR`).
///
/// Response indicating the type of the DEC locator.
///
/// The type code indicates the locator device type.
#[csi(private = '?', params = ["57"], finalbyte = 'n')]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LocatorTypeReport {
    /// Locator type code.
    pub locator_type: u8,
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
#[csi(private = '?', params = ["75"], finalbyte = 'n')]
pub struct RequestDataIntegrityStatus;

/// Data Integrity Status Report (`DSR`).
///
/// Response indicating data integrity status.
///
/// The status code typically indicates:
/// - 70: ready, no malfunction detected
/// - 71: malfunction detected
#[csi(private = '?', finalbyte = 'n')]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DataIntegrityStatusReport {
    /// Data integrity status code.
    pub status: u8,
}

impl DataIntegrityStatusReport {
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
#[csi(private = '?', params = ["85"], finalbyte = 'n')]
pub struct RequestMultipleSessionStatus;

/// Multiple Session Status Report (`DSR`).
///
/// Response indicating multiple session status.
///
/// The status code typically indicates:
/// - 80: sessions available
/// - 81: no sessions available
/// - 83: not configured for multiple sessions
#[csi(private = '?', finalbyte = 'n')]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MultipleSessionStatusReport {
    /// Multiple session status code.
    pub status: u8,
}

impl MultipleSessionStatusReport {
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

/// Request Macro Space Status (`DSR`).
///
/// Request the available macro space (historically DSR 62).
///
/// The terminal replies with the amount of available space.
///
/// See <https://terminalguide.namepad.de/seq/csi_sn/> for terminal
/// support specifics.
#[csi(private = '?', params = ["62"], finalbyte = 'n')]
pub struct RequestMacroSpaceStatus;

/// Macro Space Status Report (`DSR`).
///
/// Response indicating available macro space.
#[csi(private = '?', params = ["63"], finalbyte = 'n')]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MacroSpaceStatusReport {
    /// Available macro space in bytes.
    pub space: u16,
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
#[csi(private = '?', params = ["63"], finalbyte = 'n')]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RequestMemoryChecksum {
    /// Identifier for the memory region to checksum.
    pub id: u16,
}

/// Memory Checksum Report (`DSR`).
///
/// Response containing the memory checksum.
#[csi(private = '?', params = ["63"], finalbyte = 'n')]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemoryChecksumReport {
    /// Memory region identifier.
    pub id: u16,
    /// Checksum value.
    pub checksum: u16,
}
