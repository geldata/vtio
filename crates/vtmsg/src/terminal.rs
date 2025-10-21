//! Buffer control/information messages.

use vtenc::{ConstEncode, csi, dcs, esc, osc};

use crate::terminal_mode;

terminal_mode!(
    /// Insert mode (`IRM`).
    InsertMode,
    "4"
);

terminal_mode!(
    /// Echo mode.
    EchoMode,
    "12"
);

terminal_mode!(
    /// Linefeed mode.
    LinefeedMode,
    "20"
);

terminal_mode!(
    /// VT52 mode (`DECANM`).
    VT52Mode,
    "?2"
);

terminal_mode!(
    /// 132 column mode (`DECCOLM`).
    HundredThirtyTwoColumnMode,
    "?3"
);

terminal_mode!(
    /// Enable support for 132 column mode (`132COLS`).
    EnableSupportForHundredThirtyTwoColumnMode,
    "?40"
);

terminal_mode!(
    /// Do not clear screen on 132 column mode change (`DECNCSM`).
    KeepScreenOnHundredThirtyTwoColumnChangeMode,
    "?95"
);

terminal_mode!(
    /// Reverse display colors (`DECSCNM`).
    ReverseDisplayColorsMode,
    "?5"
);

terminal_mode!(
    /// Wraparound mode (`DECAWM`).
    LineWraparoundMode,
    "?7"
);

terminal_mode!(
    /// Scrollbar visibility (`RXVT_SCROLLBAR`).
    ScrollbarVisibilityMode,
    "?30"
);

terminal_mode!(
    /// Alternate screen buffer (ALTBUF).
    ///
    /// Switch to alternate screen buffer.
    AlternateScreenBasicMode,
    "?47"
);

terminal_mode!(
    /// Alternate screen buffer with clear on exit.
    AlternateScreenClearOnExitMode,
    "?1047"
);

terminal_mode!(
    /// Send Cursor Keys on Mouse Wheel on Alternate Screen.
    ///
    /// When the alternate screen is active and the mouse wheel
    /// is used send arrow up and down.
    ///
    /// The number of arrow up or arrow down sequences that are
    /// transmitted is implementation defined.
    CursorKeysOnMouseWheelOnAlternateScreenMode,
    "?1007"
);

terminal_mode!(
    /// Alternate screen.
    AlternateScreenMode,
    "?1049"
);

terminal_mode!(
    /// Focus monitoring.
    ReportFocusChangeMode,
    "?1004"
);

terminal_mode!(
    /// Inhibit Scroll on Application Output.
    ///
    /// Disable automatic scroll to bottom when the application
    /// outputs a printable character.
    InhibitScrollOnApplicationOutputMode,
    "?1010"
);

terminal_mode!(
    /// Scroll on Keyboard Input.
    ///
    /// If set scrolls to the bottom on every keypress.
    ScrollOnKeyboardInputMode,
    "?1011"
);

terminal_mode!(
    /// Bold/Blinking cells are also bright.
    ///
    /// If a cell is rendered in bold, and it's foreground color is one
    /// of the 8 'named' dark colors force that cell's foreground to be
    /// its corresponding bright named color.
    ///
    /// If a cell is rendered as blinking, and it's background color is
    /// one of the 8 'named' dark colors force that cell's background
    /// to be its corresponding bright named color.
    BoldBlinkingBrightMode,
    "?1021"
);

terminal_mode!(
    /// Bracketed Paste Mode.
    ///
    /// Bracket clipboard paste contents in delimiter sequences.
    ///
    /// When pasting from the (e.g. system) clipboard add `ESC[200~`
    /// before the clipboard contents and `ESC[201~` after the
    /// clipboard contents. This allows applications to distinguish
    /// clipboard contents from manually typed text.
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
pub struct RequestDefaultForeground;

impl ConstEncode for RequestDefaultForeground {
    const STR: &'static str = osc!("10;?");
}

/// Request default background color.
pub struct RequestDefaultBackground;

impl ConstEncode for RequestDefaultBackground {
    const STR: &'static str = osc!("11;?");
}

/// Request text attributes (SGR) using DECRQSS.
pub struct RequestTextAttributes;

impl ConstEncode for RequestTextAttributes {
    const STR: &'static str = dcs!("$qm");
}

/// Full Reset (`RIS`).
///
/// See <https://terminalguide.namepad.de/seq/a_esc_sc/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct FullReset;

impl ConstEncode for FullReset {
    const STR: &'static str = esc!("c");
}

/// Request Terminal ID (`DECID`).
///
/// Same as [`RequestPrimaryDeviceAttributes`] without parameters.
///
/// See <https://terminalguide.namepad.de/seq/a_esc_cz/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct RequestTerminalID;

impl ConstEncode for RequestTerminalID {
    const STR: &'static str = esc!("Z");
}

/// Request primary device attributes (DA1).
pub struct RequestDeviceAttributes;

impl ConstEncode for RequestDeviceAttributes {
    const STR: &'static str = csi!("c");
}

/// Request secondary device attributes (DA2).
pub struct RequestSecondaryDeviceAttributes;

impl ConstEncode for RequestSecondaryDeviceAttributes {
    const STR: &'static str = csi!(">c");
}

/// Request tertiary device attributes (DA3).
pub struct RequestTertiaryDeviceAttributes;

impl ConstEncode for RequestTertiaryDeviceAttributes {
    const STR: &'static str = csi!("=c");
}
