//! Buffer control/information messages.

use vtenc::{ToAnsi, AnsiEncode};

use vtio_control_base::EscapeSequenceParam;
use vtio_control_derive::{VTControl, terminal_mode};

terminal_mode!(
    /// Insert mode (`IRM`).
    ///
    /// When enabled, newly printed characters are inserted at the cursor
    /// position, shifting existing characters to the right.
    ///
    /// See <https://terminalguide.namepad.de/mode/p4/> for terminal
    /// support specifics.
    InsertMode, params = ["4"]
);

terminal_mode!(
    /// Cursor blinking mode (`ATT610_BLINK`).
    ///
    /// If set, the cursor is blinking.
    ///
    /// See also select cursor style for a more widely supported
    /// alternative.
    ///
    /// See <https://terminalguide.namepad.de/mode/p12/> for terminal
    /// support specifics.
    EchoMode, params = ["12"]
);

terminal_mode!(
    /// Linefeed/Newline mode (`LNM`).
    ///
    /// Controls whether line feed characters also perform a carriage
    /// return.
    ///
    /// See <https://terminalguide.namepad.de/mode/p20/> for terminal
    /// support specifics.
    LinefeedMode, params = ["20"]
);

terminal_mode!(
    /// Reserved for VT52 emulators (`DECANM`).
    ///
    /// Reserved for VT52 emulation.
    ///
    /// See <https://terminalguide.namepad.de/mode/p2/> for terminal
    /// support specifics.
    VT52Mode, private = '?', params = ["2"]
);

terminal_mode!(
    /// 132 column mode (`DECCOLM`).
    ///
    /// Change terminal width between 80 and 132 column mode.
    ///
    /// This mode only is supported when enable support for 132 column
    /// mode is set.
    ///
    /// Modern terminals don't have a fixed width and users generally
    /// expect the terminal to keep the size they assigned to the
    /// terminal. This control violates that expectation.
    ///
    /// If set the terminal is resized to 132 columns wide. If unset
    /// the terminal is resized to 80 columns wide.
    ///
    /// If do not clear screen on 132 column mode change is not set,
    /// the screen is cleared.
    ///
    /// The cursor is moved as invoking set cursor position with
    /// `column` and `row` set to 1.
    ///
    /// If the mode is set, left and right margin is reset.
    ///
    /// See <https://terminalguide.namepad.de/mode/p3/> for terminal
    /// support specifics.
    HundredThirtyTwoColumnMode, private = '?', params = ["3"]
);

terminal_mode!(
    /// Enable support for 132 column mode (`132COLS`).
    ///
    /// Enables support for 132 column mode.
    ///
    /// See <https://terminalguide.namepad.de/mode/p40/> for terminal
    /// support specifics.
    EnableSupportForHundredThirtyTwoColumnMode, private = '?', params = ["40"]
);

terminal_mode!(
    /// Do not clear screen on 132 column mode change (`DECNCSM`).
    ///
    /// Do not clear screen on change of 132 column mode.
    ///
    /// Only available in xterm VT level 5 or above (non-default level).
    ///
    /// See <https://terminalguide.namepad.de/mode/p95/> for terminal
    /// support specifics.
    KeepScreenOnHundredThirtyTwoColumnChangeMode, private = '?', params = ["95"]
);

terminal_mode!(
    /// Reverse display colors (`DECSCNM`).
    ///
    /// Reverses the foreground and background colors of some cells.
    ///
    /// Exact behavior is implementation specific. Most terminals swap
    /// default (unnamed) background and foreground colors when
    /// rendering.
    ///
    /// See <https://terminalguide.namepad.de/mode/p5/> for terminal
    /// support specifics.
    ReverseDisplayColorsMode, private = '?', params = ["5"]
);

terminal_mode!(
    /// Wraparound mode (`DECAWM`).
    ///
    /// Enable or disable automatic line wrapping.
    ///
    /// If disabled, cursor will stop advancing on right-most column of
    /// the scroll region or screen. Printing additional characters will
    /// (repeatedly) overwrite the cell at the cursor position.
    ///
    /// If enabled, printing to the last cell in the scroll region or
    /// screen will leave the cursor at that cell and set the pending
    /// wrap state of the cursor. Printing while the pending wrap state
    /// of the cursor is set will wrap back to the left-most column in
    /// the scroll region, unset the pending wrap state and invoke
    /// index. In some terminals it also saves the information that the
    /// line was wrapped for resize and clipboard heuristics.
    ///
    /// See <https://terminalguide.namepad.de/mode/p7/> for terminal
    /// support specifics.
    LineWraparoundMode, private = '?', params = ["7"]
);

terminal_mode!(
    /// Scrollbar visibility (`RXVT_SCROLLBAR`).
    ///
    /// Show scrollbar.
    ///
    /// See <https://terminalguide.namepad.de/mode/p30/> for terminal
    /// support specifics.
    ScrollbarVisibilityMode, private = '?', params = ["30"]
);

terminal_mode!(
    /// Alternate screen buffer (`ALTBUF`).
    ///
    /// Switch to alternate screen buffer.
    ///
    /// Terminals supporting this mode offer an alternate screen buffer
    /// in addition to the primary buffer. The primary buffer usually
    /// supports scroll-back. The alternate buffer is for full screen
    /// applications. It does not support scroll-back (or displays
    /// scroll-back from the primary screen). Switching to the alternate
    /// screen buffer for fullscreen applications allows visually
    /// switching back to the contents of the primary buffer after the
    /// application terminates.
    ///
    /// Both buffers are partially independent. They have a separate
    /// cell matrix and cursor save state.
    ///
    /// See <https://terminalguide.namepad.de/mode/p47/> for terminal
    /// support specifics.
    AlternateScreenBasicMode, private = '?', params = ["47"]
);

terminal_mode!(
    /// Alternate screen buffer with clear on exit.
    ///
    /// Like alternate screen buffer but clears the alternate buffer on
    /// reset.
    ///
    /// The clear of the alternate buffer fills all cells in the
    /// alternate buffer with space and the current SGR state.
    ///
    /// Leaving this mode might clear the text selection in terminals
    /// that support copy and paste.
    ///
    /// See <https://terminalguide.namepad.de/mode/p1047/> for terminal
    /// support specifics.
    AlternateScreenClearOnExitMode, private = '?', params = ["1047"]
);

terminal_mode!(
    /// Send Cursor Keys on Mouse Wheel on Alternate Screen.
    ///
    /// When the alternate screen is active and the mouse wheel is used
    /// send arrow up and down.
    ///
    /// The number of arrow up or arrow down sequences that are
    /// transmitted is implementation defined.
    ///
    /// All mouse reporting modes suppress this and report in their
    /// specific format instead.
    ///
    /// See <https://terminalguide.namepad.de/mode/p1007/> for terminal
    /// support specifics.
    CursorKeysOnMouseWheelOnAlternateScreenMode, private = '?', params = ["1007"]
);

terminal_mode!(
    /// Alternate screen buffer with cursor save and clear on enter.
    ///
    /// Like alternate screen buffer but saves the cursor and clears the
    /// alternate buffer on activation.
    ///
    /// The clear of the alternate buffer fills all cells in the
    /// alternate buffer with space and the current SGR state.
    ///
    /// The cursor is saved before switching to alternate mode as if
    /// save cursor was invoked. On reset the cursor is restored after
    /// switching to the primary screen buffer as if restore cursor was
    /// invoked.
    ///
    /// See <https://terminalguide.namepad.de/mode/p1049/> for terminal
    /// support specifics.
    AlternateScreenMode, private = '?', params = ["1049"]
);

terminal_mode!(
    /// Report focus change.
    ///
    /// When the terminal gains focus emit `ESC [ I`.
    ///
    /// When the terminal loses focus emit `ESC [ O`.
    ///
    /// See <https://terminalguide.namepad.de/mode/p1004/> for terminal
    /// support specifics.
    ReportFocusChangeMode, private = '?', params = ["1004"]
);

terminal_mode!(
    /// Inhibit scroll on application output.
    ///
    /// Disable automatic scroll to bottom when the application outputs
    /// a printable character.
    ///
    /// Note: xterm implements inverted behavior.
    ///
    /// See <https://terminalguide.namepad.de/mode/p1010/> for terminal
    /// support specifics.
    InhibitScrollOnApplicationOutputMode, private = '?', params = ["1010"]
);

terminal_mode!(
    /// Scroll on keyboard input.
    ///
    /// If set, scrolls to the bottom on every keypress.
    ///
    /// See <https://terminalguide.namepad.de/mode/p1011/> for terminal
    /// support specifics.
    ScrollOnKeyboardInputMode, private = '?', params = ["1011"]
);

terminal_mode!(
    /// Bold/blinking cells are also bright.
    ///
    /// If a cell is rendered in bold, and its foreground color is one
    /// of the 8 'named' dark colors, force that cell's foreground to be
    /// its corresponding bright named color.
    ///
    /// If a cell is rendered as blinking, and its background color is
    /// one of the 8 'named' dark colors, force that cell's background
    /// to be its corresponding bright named color.
    ///
    /// See <https://terminalguide.namepad.de/mode/p1021/> for terminal
    /// support specifics.
    BoldBlinkingBrightMode, private = '?', params = ["1021"]
);

terminal_mode!(
    /// Bracketed paste mode.
    ///
    /// Bracket clipboard paste contents in delimiter sequences.
    ///
    /// When pasting from the (e.g. system) clipboard add `ESC [ 200 ~`
    /// before the clipboard contents and `ESC [ 201 ~` after the
    /// clipboard contents. This allows applications to distinguish
    /// clipboard contents from manually typed text.
    ///
    /// See <https://terminalguide.namepad.de/mode/p2004/> for terminal
    /// support specifics.
    BracketedPasteMode, private = '?', params = ["2004"]
);

terminal_mode!(
    /// Synchronized update mode.
    ///
    /// When the synchronization mode is enabled following render calls
    /// will keep rendering the last rendered state. The terminal
    /// keeps processing incoming text and sequences. When the
    /// synchronized update mode is disabled again the renderer may fetch
    /// the latest screen buffer state again, effectively avoiding the
    /// tearing effect by unintentionally rendering in the middle a of
    /// an application screen update.
    ///
    /// See <https://gitlab.com/gnachman/iterm2/-/wikis/synchronized-updates-spec>
    /// for more details and <https://terminalguide.namepad.de/mode/p2026/>
    /// for terminal support specifics.
    SynchronizedUpdateMode, private = '?', params = ["2006"]
);

/// Request default foreground color.
///
/// Change/read special text default foreground color.
///
/// This is a color in addition to the palette and direct colors which
/// applies to all text that has not otherwise been assigned a
/// foreground color.
///
/// See <https://terminalguide.namepad.de/seq/osc-10/> for terminal
/// support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash, VTControl)]
#[vtctl(osc, data = "10;?")]
pub struct RequestDefaultForeground;

/// Request default background color.
///
/// Change/read special text default background color.
///
/// This is a color in addition to the palette and direct colors which
/// applies to all text that has not otherwise been assigned a
/// background color.
///
/// See <https://terminalguide.namepad.de/seq/osc-11/> for terminal
/// support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash, VTControl)]
#[vtctl(osc, data = "11;?")]
pub struct RequestDefaultBackground;

/// Request text attributes (SGR) using `DECRQSS`.
///
/// Query SGR state using DEC Request Status String.
///
/// The terminal replies with the current SGR attributes.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash, VTControl)]
#[vtctl(dcs, intermediate = "$", finalbyte = 'q', data = "m")]
pub struct RequestTextAttributes;

/// Full Reset (`RIS`).
///
/// Full reset of the terminal state.
///
/// This resets palette colors, switches to primary screen, clears the
/// screen and scrollback buffer, moves cursor to (1, 1), resets SGR
/// attributes, makes cursor visible, resets cursor shape and
/// blinking, resets cursor origin mode, resets scrolling region,
/// resets character sets, disables all mouse tracking modes, resets
/// tab stops, and reverts many other terminal settings to their
/// initial state.
///
/// See <https://terminalguide.namepad.de/seq/a_esc_sc/> for terminal
/// support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash, VTControl)]
#[vtctl(esc, finalbyte = 'c')]
pub struct FullReset;

/// Request Terminal ID (`DECID`).
///
/// Same as primary device attributes without parameters.
///
/// See <https://terminalguide.namepad.de/seq/a_esc_cz/> for terminal
/// support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash, VTControl)]
#[vtctl(esc, finalbyte = 'Z')]
pub struct RequestTerminalID;

/// Request primary device attributes (`DA1`).
///
/// Query the terminal's primary device attributes.
///
/// The response depends on the terminal implementation and
/// configuration. Different terminals report different capabilities
/// and version information.
///
/// See <https://terminalguide.namepad.de/seq/csi_sc/> for terminal
/// support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash, VTControl)]
#[vtctl(csi, finalbyte = 'c')]
pub struct RequestPrimaryDeviceAttributes;

/// Request secondary device attributes (`DA2`).
///
/// Query the terminal's secondary device attributes.
///
/// This typically returns terminal type and version information.
/// Different terminals return different identification codes.
///
/// See <https://terminalguide.namepad.de/seq/> for terminal support
/// specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash, VTControl)]
#[vtctl(csi, intermediate = ">", finalbyte = 'c')]
pub struct RequestSecondaryDeviceAttributes;

/// Request tertiary device attributes (`DA3`).
///
/// Query the terminal's tertiary device attributes.
///
/// This is less commonly supported than DA1 and DA2. When supported,
/// it typically returns additional terminal identification
/// information.
///
/// See <https://terminalguide.namepad.de/seq/> for terminal support
/// specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash, VTControl)]
#[vtctl(csi, intermediate = "=", finalbyte = 'c')]
pub struct RequestTertiaryDeviceAttributes;

/// Terminal conformance level for DA1 response.
///
/// The first parameter in a DA1 response indicates the terminal's
/// conformance level.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ConformanceLevel {
    /// VT100 compatibility (Level 1).
    VT100 = 1,
    /// VT102 compatibility (Level 1).
    VT102 = 6,
    /// VT220 compatibility (Level 2).
    VT220 = 62,
    /// VT320 compatibility (Level 3).
    VT320 = 63,
    /// VT420/VT510/VT525 compatibility (Level 4).
    VT420 = 64,
}

impl Default for ConformanceLevel {
    fn default() -> Self {
        Self::VT100
    }
}

impl ToAnsi for ConformanceLevel {
    fn to_ansi(&self) -> impl AnsiEncode {
        *self as u8
    }
}

impl From<u8> for ConformanceLevel {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::VT100,
            6 => Self::VT102,
            62 => Self::VT220,
            63 => Self::VT320,
            64 => Self::VT420,
            _ => Self::default(),
        }
    }
}

impl From<EscapeSequenceParam> for ConformanceLevel {
    fn from(param: EscapeSequenceParam) -> Self {
        Self::from(param.first())
    }
}

impl From<&EscapeSequenceParam> for ConformanceLevel {
    fn from(param: &EscapeSequenceParam) -> Self {
        Self::from(param.first())
    }
}

/// Terminal capability flags for DA1 response.
///
/// These flags indicate which features the terminal supports.
/// Multiple capabilities can be combined in a single response.
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
#[repr(u8)]
pub enum TerminalCapability {
    /// 132 columns mode (`DECCOLM`).
    Columns132 = 1,
    /// Printer port.
    Printer = 2,
    /// `ReGIS` graphics.
    ReGISGraphics = 3,
    /// `SIXEL` graphics.
    SixelGraphics = 4,
    /// Selective erase (`DECSED`, `DECSEL`).
    SelectiveErase = 6,
    /// Soft character sets (`DRCS` - Dynamic Redefinable Character Sets).
    SoftCharacterSets = 7,
    /// User-defined keys (`DECUDK`).
    UserDefinedKeys = 8,
    /// National Replacement Character sets (`NRC`).
    NationalReplacementCharsets = 9,
    /// Blink attribute (`SGR 5`).
    Blink = 12,
    /// Technical character set.
    TechnicalCharset = 15,
    /// Locator (Mouse) device.
    LocatorDevice = 16,
    /// User-defined keys (extended).
    UserDefinedKeysExtended = 17,
    /// National Replacement Character sets (extended).
    NationalReplacementCharsetsExtended = 18,
    /// 24 or more lines.
    MoreThan24Lines = 19,
    /// Multiple pages / horizontal scrolling.
    HorizontalScrolling = 21,
    /// ANSI color support.
    Color = 22,
    /// Soft key labels.
    SoftKeyLabels = 23,
    /// Rectangular area operations (`DECCRA`, `DECFRA`).
    RectangularAreaOps = 24,
    /// Locator events (motion/button).
    LocatorEvents = 29,
    /// Windowing extensions (`DECRQCRA`).
    WindowingExtensions = 42,
    /// Cursor position report format.
    CursorPositionReportFormat = 44,
    /// RGB color / extended color.
    ExtendedColor = 46,
    /// xterm/VT525-like (older xterm-style)
    VT525Xterm = 52,
    /// Modern xterm/VT525-like
    VT525ModernXterm = 67,
    /// Sentinel
    Unrecognized(u8),
}

impl From<&TerminalCapability> for u8 {
    fn from(value: &TerminalCapability) -> Self {
        match value {
            TerminalCapability::Columns132 => 1,
            TerminalCapability::Printer => 2,
            TerminalCapability::ReGISGraphics => 3,
            TerminalCapability::SixelGraphics => 4,
            TerminalCapability::SelectiveErase => 6,
            TerminalCapability::SoftCharacterSets => 7,
            TerminalCapability::UserDefinedKeys => 8,
            TerminalCapability::NationalReplacementCharsets => 9,
            TerminalCapability::Blink => 12,
            TerminalCapability::TechnicalCharset => 15,
            TerminalCapability::LocatorDevice => 16,
            TerminalCapability::UserDefinedKeysExtended => 17,
            TerminalCapability::NationalReplacementCharsetsExtended => 18,
            TerminalCapability::MoreThan24Lines => 19,
            TerminalCapability::HorizontalScrolling => 21,
            TerminalCapability::Color => 22,
            TerminalCapability::SoftKeyLabels => 23,
            TerminalCapability::RectangularAreaOps => 24,
            TerminalCapability::LocatorEvents => 29,
            TerminalCapability::WindowingExtensions => 42,
            TerminalCapability::CursorPositionReportFormat => 44,
            TerminalCapability::ExtendedColor => 46,
            TerminalCapability::VT525Xterm => 52,
            TerminalCapability::VT525ModernXterm => 67,
            TerminalCapability::Unrecognized(b) => *b,
        }
    }
}

impl From<TerminalCapability> for u8 {
    fn from(value: TerminalCapability) -> Self {
        Self::from(&value)
    }
}

impl ToAnsi for TerminalCapability {
    fn to_ansi(&self) -> impl AnsiEncode {
        u8::from(self)
    }
}

impl From<u8> for TerminalCapability {
    fn from(value: u8) -> Self {
        match value {
            1 => TerminalCapability::Columns132,
            2 => TerminalCapability::Printer,
            3 => TerminalCapability::ReGISGraphics,
            4 => TerminalCapability::SixelGraphics,
            6 => TerminalCapability::SelectiveErase,
            7 => TerminalCapability::SoftCharacterSets,
            8 => TerminalCapability::UserDefinedKeys,
            9 => TerminalCapability::NationalReplacementCharsets,
            12 => TerminalCapability::Blink,
            15 => TerminalCapability::TechnicalCharset,
            16 => TerminalCapability::LocatorDevice,
            17 => TerminalCapability::UserDefinedKeysExtended,
            18 => TerminalCapability::NationalReplacementCharsetsExtended,
            19 => TerminalCapability::MoreThan24Lines,
            21 => TerminalCapability::HorizontalScrolling,
            22 => TerminalCapability::Color,
            23 => TerminalCapability::SoftKeyLabels,
            24 => TerminalCapability::RectangularAreaOps,
            29 => TerminalCapability::LocatorEvents,
            42 => TerminalCapability::WindowingExtensions,
            44 => TerminalCapability::CursorPositionReportFormat,
            46 => TerminalCapability::ExtendedColor,
            52 => TerminalCapability::VT525Xterm,
            67 => TerminalCapability::VT525ModernXterm,
            n => TerminalCapability::Unrecognized(n),
        }
    }
}

impl From<&EscapeSequenceParam> for TerminalCapability {
    fn from(param: &EscapeSequenceParam) -> Self {
        Self::from(param.first())
    }
}

impl From<EscapeSequenceParam> for TerminalCapability {
    fn from(param: EscapeSequenceParam) -> Self {
        Self::from(&param)
    }
}

/// Terminal capabilities wrapper for encoding.
///
/// Encodes a vector of terminal capabilities as a semicolon-separated list.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Capabilities(pub Vec<TerminalCapability>);

impl Capabilities {
    /// Create from a vector of terminal capabilities.
    #[must_use]
    pub fn new(capabilities: Vec<TerminalCapability>) -> Self {
        Self(capabilities)
    }

    /// Create from a slice of terminal capabilities.
    #[must_use]
    pub fn from_slice(capabilities: &[TerminalCapability]) -> Self {
        Self(capabilities.to_vec())
    }
}

impl ToAnsi for Capabilities {
    fn to_ansi(&self) -> impl AnsiEncode {
        self.0
            .iter()
            .map(|cap| u8::from(cap).to_string())
            .collect::<Vec<String>>()
            .join(";")
    }
}

impl From<Vec<TerminalCapability>> for Capabilities {
    fn from(caps: Vec<TerminalCapability>) -> Self {
        Self(caps)
    }
}

impl From<EscapeSequenceParam> for Capabilities {
    fn from(param: EscapeSequenceParam) -> Self {
        // Parse semicolon-separated capabilities
        let s = String::from_utf8_lossy(&param);
        let caps: Vec<TerminalCapability> = s
            .split(';')
            .filter_map(|s| s.parse::<u8>().ok())
            .map(TerminalCapability::from)
            .collect();
        Self(caps)
    }
}

impl From<&EscapeSequenceParam> for Capabilities {
    fn from(param: &EscapeSequenceParam) -> Self {
        // Parse semicolon-separated capabilities
        let s = String::from_utf8_lossy(param);
        let caps: Vec<TerminalCapability> = s
            .split(';')
            .filter_map(|s| s.parse::<u8>().ok())
            .map(TerminalCapability::from)
            .collect();
        Self(caps)
    }
}

/// Response to primary device attributes request (`DA1`).
///
/// Send terminal capabilities in response to a DA1 query.
///
/// The response format is `CSI ? [level] ; [cap1] ; [cap2] ; ... c`.
///
/// See <https://terminalguide.namepad.de/seq/csi_sc/> for terminal
/// support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Hash, VTControl)]
#[vtctl(csi, private = '?', finalbyte = 'c')]
pub struct PrimaryDeviceAttributesResponse {
    /// Conformance level (VT100, VT220, etc.).
    pub conformance_level: ConformanceLevel,
    /// Terminal capabilities to report.
    pub capabilities: Capabilities,
}

/// Response to secondary device attributes request (`DA2`).
///
/// Send terminal type and version information in response to a DA2
/// query.
///
/// The response format is `CSI > [terminal_type] ; [version] ; [extra] c`.
///
/// Common terminal type codes:
/// - 0: VT100
/// - 1: VT220
/// - 2: VT240
/// - 18: VT330
/// - 19: VT340
/// - 24: VT320
/// - 41: VT420
/// - 61: VT510
/// - 64: VT520
/// - 65: VTE-based (e.g., GNOME Terminal)
///
/// The version field typically contains the terminal version or patch
/// level.
///
/// See <https://terminalguide.namepad.de/seq/csi_sc__q/> for terminal support
/// specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash, VTControl)]
#[vtctl(csi, private = '>', finalbyte = 'c')]
pub struct SecondaryDeviceAttributesResponse {
    pub terminal_type: u16,
    pub version: u16,
    pub extra: Option<u16>,
}

/// Unit ID wrapper for hex encoding.
///
/// Encodes a 4-byte unit ID as an 8-character hexadecimal string.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct UnitId(pub [u8; 4]);

impl UnitId {
    /// Create from a 4-byte array.
    #[must_use]
    pub const fn new(bytes: [u8; 4]) -> Self {
        Self(bytes)
    }

    /// Create from a string (takes first 4 bytes).
    #[must_use]
    pub fn from_string(s: &str) -> Self {
        let bytes = s.as_bytes();
        let mut id = [0u8; 4];
        let len = bytes.len().min(4);
        id[..len].copy_from_slice(&bytes[..len]);
        Self(id)
    }
}

impl ToAnsi for UnitId {
    fn to_ansi(&self) -> impl AnsiEncode {
        const HEX: &[u8; 16] = b"0123456789ABCDEF";
        let mut hex = [0u8; 8];

        for (i, &b) in self.0.iter().enumerate() {
            hex[2 * i] = HEX[(b >> 4) as usize];
            hex[2 * i + 1] = HEX[(b & 0x0F) as usize];
        }

        // SAFETY: we are hexlifying bytes above, so `hex`
        // is always a valid ASCII string.
        unsafe { std::str::from_utf8_unchecked(&hex).to_string() }
    }
}

impl From<[u8; 4]> for UnitId {
    fn from(bytes: [u8; 4]) -> Self {
        Self(bytes)
    }
}

impl From<EscapeSequenceParam> for UnitId {
    fn from(param: EscapeSequenceParam) -> Self {
        let s = String::from_utf8_lossy(&param);
        let mut bytes = [0u8; 4];

        // Parse hex string back to bytes
        for (i, chunk) in s.as_bytes().chunks(2).enumerate().take(4) {
            if chunk.len() == 2
                && let Ok(b) = u8::from_str_radix(std::str::from_utf8(chunk).unwrap_or("00"), 16)
            {
                bytes[i] = b;
            }
        }

        Self(bytes)
    }
}

impl From<&EscapeSequenceParam> for UnitId {
    fn from(param: &EscapeSequenceParam) -> Self {
        let s = String::from_utf8_lossy(param);
        let mut bytes = [0u8; 4];

        // Parse hex string back to bytes
        for (i, chunk) in s.as_bytes().chunks(2).enumerate().take(4) {
            if chunk.len() == 2
                && let Ok(b) = u8::from_str_radix(std::str::from_utf8(chunk).unwrap_or("00"), 16)
            {
                bytes[i] = b;
            }
        }

        Self(bytes)
    }
}

/// Response to tertiary device attributes request (`DECRPTUI`).
///
/// Send terminal unit ID in response to a DA3 query.
///
/// The response format is `DCS ! | [hex_string] ST` where `hex_string`
/// is the terminal's unit ID encoded as hexadecimal pairs.
///
/// This is less commonly supported than DA1 and DA2. When supported,
/// the unit ID is typically a fixed string identifying the terminal
/// hardware or implementation.
///
/// # Examples
///
/// Different terminals return different unit IDs encoded as hexadecimal:
///
/// - xterm (v336+): `DCS ! | 00000000 ST`
/// - VTE (GNOME Terminal): `DCS ! | 7E565445 ST` ("~VTE")
/// - Konsole: `DCS ! | 7E4B4445 ST` ("~KDE")
/// - iTerm2: `DCS ! | 6954726D ST` ("iTrm")
///
/// See <https://terminalguide.namepad.de/seq/csi_sc__r/> for terminal
/// support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Hash, VTControl)]
#[vtctl(dcs, intermediate = "!", finalbyte = '|')]
pub struct TertiaryDeviceAttributesResponse {
    pub unit_id: UnitId,
}

/// Select VT-XXX Conformance Level (`DECSCL`).
///
/// Set the conformance level and encoding for C1 controls in terminal
/// replies.
///
/// If `level` < 61 or higher than the configured maximum this sequence
/// does nothing.
///
/// Otherwise `level` - 60 is the VT-xxx conformance level to activate
/// (i.e. `level` = 64 -> VT-4xx conformance).
///
/// If `level` > 61, the parameter `c1_encoding` is used to set the
/// encoding for C1 controls. If `c1_encoding` = 1 then use 7-bit
/// controls. If `c1_encoding` is 0 or 2 then use 8-bit controls. If
/// `c1_encoding` is explicitly set to any other value the encoding is
/// not changed.
///
/// See <https://terminalguide.namepad.de/seq/csi_sp_t_quote/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash, VTControl)]
#[vtctl(csi, intermediate = "\"", finalbyte = 'p')]
pub struct SelectVTConformanceLevel {
    pub level: u16,
    pub c1_encoding: Option<u8>,
}

/// Request VT-xxx Conformance Level and C1 Encoding.
///
/// Query state settable with select vt-xxx conformance level.
///
/// The terminal replies with:
///
/// `DCS $ r level ; c1_encoding ST`
///
/// Where `level` is the vt level plus 60 (i.e. 64 for vt level 4) and
/// `c1_encoding` is set to 1 if 7bit encoding of C1 controls is
/// selected.
///
/// See <https://terminalguide.namepad.de/seq/dcs-dollar-q-quote-p/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash, VTControl)]
#[vtctl(dcs, intermediate = "$", finalbyte = 'q', data = "\"p")]
pub struct RequestVTConformanceLevel;

#[cfg(test)]
mod tests {
    use super::*;
    use vtio_control_base::AnsiEncode2;

    #[test]
    fn test_primary_device_attributes_response_encoding() {
        let mut response = PrimaryDeviceAttributesResponse {
            conformance_level: ConformanceLevel::VT220,
            capabilities: Capabilities(vec![
                TerminalCapability::Columns132,
                TerminalCapability::SixelGraphics,
                TerminalCapability::Color,
            ]),
        };

        let mut buf = Vec::new();
        response.encode_ansi_into(&mut buf).unwrap();
        let encoded = String::from_utf8(buf).unwrap();

        assert_eq!(encoded, "\x1b[?62;1;4;22c");
    }

    #[test]
    fn test_secondary_device_attributes_response_encoding() {
        let mut response = SecondaryDeviceAttributesResponse {
            terminal_type: 65,
            version: 6800,
            extra: Some(1),
        };

        let mut buf = Vec::new();
        response.encode_ansi_into(&mut buf).unwrap();
        let encoded = String::from_utf8(buf).unwrap();

        assert_eq!(encoded, "\x1b[>65;6800;1c");
    }

    #[test]
    fn test_secondary_device_attributes_response_encoding_no_extra() {
        let mut response = SecondaryDeviceAttributesResponse {
            terminal_type: 1,
            version: 0,
            extra: None,
        };

        let mut buf = Vec::new();
        response.encode_ansi_into(&mut buf).unwrap();
        let encoded = String::from_utf8(buf).unwrap();

        assert_eq!(encoded, "\x1b[>1;0c");
    }

    #[test]
    fn test_tertiary_device_attributes_response_encoding() {
        let mut response = TertiaryDeviceAttributesResponse {
            unit_id: UnitId([0x7E, 0x56, 0x54, 0x45]), // "~VTE"
        };

        let mut buf = Vec::new();
        response.encode_ansi_into(&mut buf).unwrap();
        let encoded = String::from_utf8(buf).unwrap();

        assert_eq!(encoded, "\x1bP!|7E565445\x1b\\");
    }

    #[test]
    fn test_select_vt_conformance_level_encoding() {
        let mut cmd = SelectVTConformanceLevel {
            level: 64,
            c1_encoding: Some(1),
        };

        let mut buf = Vec::new();
        cmd.encode_ansi_into(&mut buf).unwrap();
        let encoded = String::from_utf8(buf).unwrap();

        assert_eq!(encoded, "\x1b[64;1\"p");
    }

    #[test]
    fn test_select_vt_conformance_level_encoding_no_c1() {
        let mut cmd = SelectVTConformanceLevel {
            level: 62,
            c1_encoding: None,
        };

        let mut buf = Vec::new();
        cmd.encode_ansi_into(&mut buf).unwrap();
        let encoded = String::from_utf8(buf).unwrap();

        assert_eq!(encoded, "\x1b[62\"p");
    }
}
