//! Buffer control/information messages.

use vtenc::{ConstEncode, csi, dcs, esc, osc};

use crate::terminal_mode;

terminal_mode!(
    #[doc = "Insert mode (`IRM`)."]
    InsertMode,
    "4"
);

terminal_mode!(
    #[doc = "Echo mode."]
    EchoMode,
    "12"
);

terminal_mode!(
    #[doc = "Linefeed mode."]
    LinefeedMode,
    "20"
);

terminal_mode!(
    #[doc = "VT52 mode (`DECANM`)."]
    VT52Mode,
    "?2"
);

terminal_mode!(
    #[doc = "132 column mode (`DECCOLM`)."]
    HundredThirtyTwoColumnMode,
    "?3"
);

terminal_mode!(
    #[doc = "Enable support for 132 column mode (`132COLS`)."]
    EnableSupportForHundredThirtyTwoColumnMode,
    "?40"
);

terminal_mode!(
    #[doc = "Do not clear screen on 132 column mode change (`DECNCSM`)."]
    KeepScreenOnHundredThirtyTwoColumnChangeMode,
    "?95"
);

terminal_mode!(
    #[doc = "Reverse display colors (`DECSCNM`)."]
    ReverseDisplayColorsMode,
    "?5"
);

terminal_mode!(
    #[doc = "Wraparound mode (`DECAWM`)."]
    LineWraparoundMode,
    "?7"
);

terminal_mode!(
    #[doc = "Scrollbar visibility (`RXVT_SCROLLBAR`)."]
    ScrollbarVisibilityMode,
    "?30"
);

terminal_mode!(
    #[doc = "Alternate screen buffer (ALTBUF). \
            Switch to alternate screen buffer."]
    AlternateScreenBasicMode,
    "?47"
);

terminal_mode!(
    #[doc = "Alternate screen buffer with clear on exit."]
    AlternateScreenClearOnExitMode,
    "?1047"
);

terminal_mode!(
    #[doc = "Send Cursor Keys on Mouse Wheel on Alternate Screen. \n\n\
            When the alternate screen is active and the mouse wheel \
            is used send arrow up and down. \n\n\
            The number of arrow up or arrow down sequences that are \
            transmitted is implementation defined."]
    CursorKeysOnMouseWheelOnAlternateScreenMode,
    "?1007"
);

terminal_mode!(
    #[doc = "Alternate screen."]
    AlternateScreenMode,
    "?1049"
);

terminal_mode!(
    #[doc = "Focus monitoring."]
    ReportFocusChangeMode,
    "?1004"
);

terminal_mode!(
    #[doc = "Inhibit Scroll on Application Output.\n\n\
            Disable automatic scroll to bottom when the application \
            outputs a printable character."]
    InhibitScrollOnApplicationOutputMode,
    "?1010"
);

terminal_mode!(
    #[doc = "Scroll on Keyboard Input.\n\n\
            If set scrolls to the bottom on every keypress."]
    ScrollOnKeyboardInputMode,
    "?1011"
);

terminal_mode!(
    #[doc = "Bold/Blinking cells are also bright.\n\n\
            If a cell is rendered in bold, and it's foreground color is one \
            of the 8 'named' dark colors force that cell's foreground to be \
            its corresponding bright named color.\n\n\
            If a cell is rendered as blinking, and it's background color is \
            one of the 8 'named' dark colors force that cell's background \
            to be its corresponding bright named color."]
    BoldBlinkingBrightMode,
    "?1021"
);

terminal_mode!(
    #[doc = "Bracketed Paste Mode.\n\n\
            Bracket clipboard paste contents in delimiter sequences.\n\n\
            When pasting from the (e.g. system) clipboard add `ESC[200~`
            before the clipboard contents and `ESC[201~` after the
            clipboard contents. This allows applications to distinguish
            clipboard contents from manually typed text."]
    BracketedPasteMode,
    "?2004"
);

terminal_mode!(
    #[doc = "Synchronized update mode.\n\n\
            When the synchronization mode is enabled following render calls \
            will keep rendering the last rendered state. The terminal \
            keeps processing incoming text and sequences. When the \
            synchronized update mode is disabled again the renderer may fetch \
            the latest screen buffer state again, effectively avoiding the \
            tearing effect by unintentionally rendering in the middle a of \
            an application screen update. \n\n\
            See <https://gitlab.com/gnachman/iterm2/-/wikis/synchronized-updates-spec> \
            for more details and <https://terminalguide.namepad.de/mode/p2026/> \
            for terminal support specifics."]
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
