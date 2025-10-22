//! Buffer control/information messages.

use vtenc::{
    ConstEncode, ConstEncodedLen, Encode, EncodeError, csi, dcs, esc, osc, write_csi, write_dcs,
    write_int, write_str_into,
};

use crate::terminal_mode;

terminal_mode!(
    /// Insert mode (`IRM`).
    ///
    /// When enabled, newly printed characters are inserted at the cursor
    /// position, shifting existing characters to the right.
    ///
    /// See <https://terminalguide.namepad.de/mode/p4/> for terminal
    /// support specifics.
    InsertMode,
    "4"
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
    EchoMode,
    "12"
);

terminal_mode!(
    /// Linefeed/Newline mode (`LNM`).
    ///
    /// Controls whether line feed characters also perform a carriage
    /// return.
    ///
    /// See <https://terminalguide.namepad.de/mode/p20/> for terminal
    /// support specifics.
    LinefeedMode,
    "20"
);

terminal_mode!(
    /// Reserved for VT52 emulators (`DECANM`).
    ///
    /// Reserved for VT52 emulation.
    ///
    /// See <https://terminalguide.namepad.de/mode/p2/> for terminal
    /// support specifics.
    VT52Mode,
    "?2"
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
    HundredThirtyTwoColumnMode,
    "?3"
);

terminal_mode!(
    /// Enable support for 132 column mode (`132COLS`).
    ///
    /// Enables support for 132 column mode.
    ///
    /// See <https://terminalguide.namepad.de/mode/p40/> for terminal
    /// support specifics.
    EnableSupportForHundredThirtyTwoColumnMode,
    "?40"
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
    KeepScreenOnHundredThirtyTwoColumnChangeMode,
    "?95"
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
    ReverseDisplayColorsMode,
    "?5"
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
    LineWraparoundMode,
    "?7"
);

terminal_mode!(
    /// Scrollbar visibility (`RXVT_SCROLLBAR`).
    ///
    /// Show scrollbar.
    ///
    /// See <https://terminalguide.namepad.de/mode/p30/> for terminal
    /// support specifics.
    ScrollbarVisibilityMode,
    "?30"
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
    AlternateScreenBasicMode,
    "?47"
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
    AlternateScreenClearOnExitMode,
    "?1047"
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
    CursorKeysOnMouseWheelOnAlternateScreenMode,
    "?1007"
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
    AlternateScreenMode,
    "?1049"
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
    ReportFocusChangeMode,
    "?1004"
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
    InhibitScrollOnApplicationOutputMode,
    "?1010"
);

terminal_mode!(
    /// Scroll on keyboard input.
    ///
    /// If set, scrolls to the bottom on every keypress.
    ///
    /// See <https://terminalguide.namepad.de/mode/p1011/> for terminal
    /// support specifics.
    ScrollOnKeyboardInputMode,
    "?1011"
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
    BoldBlinkingBrightMode,
    "?1021"
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
    BracketedPasteMode,
    "?2004"
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
    SynchronizedUpdateMode,
    "?2006"
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
pub struct RequestDefaultForeground;

impl ConstEncode for RequestDefaultForeground {
    const STR: &'static str = osc!("10;?");
}

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
pub struct RequestDefaultBackground;

impl ConstEncode for RequestDefaultBackground {
    const STR: &'static str = osc!("11;?");
}

/// Request text attributes (SGR) using `DECRQSS`.
///
/// Query SGR state using DEC Request Status String.
///
/// The terminal replies with the current SGR attributes.
pub struct RequestTextAttributes;

impl ConstEncode for RequestTextAttributes {
    const STR: &'static str = dcs!("$qm");
}

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
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct FullReset;

impl ConstEncode for FullReset {
    const STR: &'static str = esc!("c");
}

/// Request Terminal ID (`DECID`).
///
/// Same as primary device attributes without parameters.
///
/// See <https://terminalguide.namepad.de/seq/a_esc_cz/> for terminal
/// support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct RequestTerminalID;

impl ConstEncode for RequestTerminalID {
    const STR: &'static str = esc!("Z");
}

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
pub struct RequestPrimaryDeviceAttributes;

impl ConstEncode for RequestPrimaryDeviceAttributes {
    const STR: &'static str = csi!("c");
}

/// Request secondary device attributes (`DA2`).
///
/// Query the terminal's secondary device attributes.
///
/// This typically returns terminal type and version information.
/// Different terminals return different identification codes.
///
/// See <https://terminalguide.namepad.de/seq/> for terminal support
/// specifics.
pub struct RequestSecondaryDeviceAttributes;

impl ConstEncode for RequestSecondaryDeviceAttributes {
    const STR: &'static str = csi!(">c");
}

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
pub struct RequestTertiaryDeviceAttributes;

impl ConstEncode for RequestTertiaryDeviceAttributes {
    const STR: &'static str = csi!("=c");
}

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

/// Terminal capability flags for DA1 response.
///
/// These flags indicate which features the terminal supports.
/// Multiple capabilities can be combined in a single response.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
#[repr(u16)]
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
}

/// Response to primary device attributes request (`DA1`).
///
/// Send terminal capabilities in response to a DA1 query.
///
/// The response format is `CSI ? [level] ; [cap1] ; [cap2] ; ... c`.
///
/// See <https://terminalguide.namepad.de/seq/csi_sc/> for terminal
/// support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Hash)]
pub struct PrimaryDeviceAttributesResponse {
    /// Conformance level (VT100, VT220, etc.).
    pub conformance_level: ConformanceLevel,
    /// Terminal capabilities to report.
    pub capabilities: Vec<TerminalCapability>,
}

impl PrimaryDeviceAttributesResponse {
    #[inline]
    #[must_use]
    pub fn new(conformance_level: ConformanceLevel, capabilities: Vec<TerminalCapability>) -> Self {
        Self {
            conformance_level,
            capabilities,
        }
    }
}

impl Encode for PrimaryDeviceAttributesResponse {
    #[inline]
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        let mut written = write_csi!(buf; "?", self.conformance_level as i16)?;

        for cap in &self.capabilities {
            written += write_str_into(buf, ";")?;
            written += write_int(buf, *cap as u16)?;
        }

        written += write_str_into(buf, "c")?;
        Ok(written)
    }
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
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct SecondaryDeviceAttributesResponse {
    pub terminal_type: u16,
    pub version: u16,
    pub extra: Option<u16>,
}

impl Encode for SecondaryDeviceAttributesResponse {
    #[inline]
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        let mut written = write_csi!(buf; ">", self.terminal_type, ";", self.version)?;
        if let Some(extra) = self.extra {
            written += write_str_into(buf, ";")?;
            written += write_int(buf, extra)?;
        }
        written += write_str_into(buf, "c")?;
        Ok(written)
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
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Hash)]
pub struct TertiaryDeviceAttributesResponse {
    pub unit_id: [u8; 4],
}

impl Encode for TertiaryDeviceAttributesResponse {
    #[inline]
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        const HEX: &[u8; 16] = b"0123456789ABCDEF";
        let mut hex = [0u8; 8];
        let s;

        for (i, &b) in self.unit_id.iter().enumerate() {
            hex[2 * i] = HEX[(b >> 4) as usize];
            hex[2 * i + 1] = HEX[(b & 0x0F) as usize];
        }

        // SAFETY: we are hexlifying bytes above, so `hex`
        // is always a valid ASCII string.
        unsafe {
            s = std::str::from_utf8_unchecked(&hex);
        }

        write_dcs!(buf; "!|", s)
    }
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
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct SelectVTConformanceLevel {
    pub level: u16,
    pub c1_encoding: Option<u8>,
}

impl ConstEncodedLen for SelectVTConformanceLevel {
    // CSI (2) + max u16 digits (5) + ";" (1) + max u8 digits (3) + "\"p" (2) = 13
    const ENCODED_LEN: usize = 13;
}

impl Encode for SelectVTConformanceLevel {
    #[inline]
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        match self.c1_encoding {
            Some(encoding) => {
                write_csi!(buf; self.level, ";", encoding, "\"p")
            }
            None => {
                write_csi!(buf; self.level, "\"p")
            }
        }
    }
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
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct RequestVTConformanceLevel;

impl ConstEncode for RequestVTConformanceLevel {
    const STR: &'static str = dcs!("$q\"p");
}
