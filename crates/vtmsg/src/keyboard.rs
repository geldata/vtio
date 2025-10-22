//! Keyboard-related messages.

use bitflags::bitflags;
use vtenc::{ConstEncode, Encode, EncodeError, csi, esc, write_csi};

use crate::terminal_mode;

terminal_mode!(
    /// Disable Keyboard Input (`KAM`).
    ///
    /// When this mode is active it disables all keyboard input.
    ///
    /// See <https://terminalguide.namepad.de/mode/2/> for
    /// terminal support specifics.
    KeyboardInputDisabledMode,
    "2"
);

terminal_mode!(
    /// Cursor Key Format (`DECCKM`).
    ///
    /// Switches reporting format for cursor type keys.
    ///
    /// This changes the reported sequence for:
    /// - up, down, right, left
    /// - numpad up, down, right, left (for terminals that use ESC [ A etc
    ///   sequences for these keys)
    /// - home, end (for terminals that use ESC [ H etc sequences for these
    ///   keys)
    ///
    /// With the mode active these report with sequences beginning with ESC O.
    /// With the mode reset these report with sequences beginning with ESC [.
    ///
    /// See <https://terminalguide.namepad.de/mode/p1/> for
    /// terminal support specifics.
    CursorKeysMode,
    "?1"
);

terminal_mode!(
    /// Repeat Held Keys.
    ///
    /// Repeat keys while being held down. If enabled a key held down
    /// automatically repeats in an implementation specific interval.
    ///
    /// See <https://terminalguide.namepad.de/mode/p8/> for
    /// terminal support specifics.
    HeldKeysRepeatMode,
    "?8"
);

terminal_mode!(
    /// Application Keypad Mode (`DECNKM`).
    ///
    /// This is a mirror of [`SetApplicationKeypadMode`].
    ///
    /// See <https://terminalguide.namepad.de/mode/p66/> for
    /// terminal support specifics.
    ApplicationKeypadMode,
    "?66"
);

terminal_mode!(
    /// Backspace Sends Delete (`DECBKM`).
    ///
    /// When set the backspace key sends BS, when reset it sends DEL.
    ///
    /// See <https://terminalguide.namepad.de/mode/p67/> for
    /// terminal support specifics.
    BackspaceSendsDeleteMode,
    "?67"
);

terminal_mode!(
    /// Alt + Key Sends Character with High Bit Set.
    ///
    /// Use high bit to signal keypresses with alt modifier held.
    ///
    /// When reporting key presses use the high (eighth) bit to indicate
    /// alt modifier. (This will collide with non ASCII characters)
    ///
    /// See <https://terminalguide.namepad.de/mode/p1034/> for
    /// terminal support specifics.
    AltKeyHighBitSetMode,
    "?1034"
);

terminal_mode!(
    /// Ignore Keypad Application Mode When Numlock is Active.
    ///
    /// If application keypad mode is active:
    /// - The mathematical operations keys (/, *, -, +) are sent to the
    ///   application as escape sequence regardless of num lock state if this
    ///   mode is off. If this mode is on, the keys send their printable
    ///   character when num lock is active.
    /// - With num lock active, the number / edit keys send escape sequences
    ///   if this mode is off if shift is not pressed. When this mode is on
    ///   they send the printable character for their number. (In both
    ///   settings the keys send the same escape sequences when shift is held)
    /// - With num lock active, the enter key on the num pad sends escape
    ///   sequences if this mode is off. When this mode is on it sends CR.
    ///
    /// See <https://terminalguide.namepad.de/mode/p1035/> for
    /// terminal support specifics.
    IgnoreKeypadApplicationModeOnNumlockMode,
    "?1035"
);

terminal_mode!(
    /// Alt + Key Sends Esc as Prefix.
    ///
    /// When this mode is active Alt + Key sends ESC + the key for
    /// printable inputs instead of forcing the 8th bit of the character
    /// to high.
    ///
    /// See <https://terminalguide.namepad.de/mode/p1036/> for
    /// terminal support specifics.
    AltKeySendsEscPrefixMode,
    "?1036"
);

terminal_mode!(
    /// Delete Key sends DEL.
    ///
    /// If set use legacy DEL for the delete key instead of an
    /// escape sequence.
    ///
    /// See <https://terminalguide.namepad.de/mode/p1037/> for
    /// terminal support specifics.
    DeleteKeySendsDELMode,
    "?1037"
);

terminal_mode!(
    /// Additional Modifier + Key Sends Esc as Prefix.
    ///
    /// This is similar to [`AltKeySendsEscPrefixMode`], but for
    /// an additionally configured modifier.
    ///
    /// See <https://terminalguide.namepad.de/mode/p1039/> for
    /// terminal support specifics.
    AdditionalModifierKeySendsEscPrefix,
    "?1039"
);

bitflags! {
    /// Keyboard enhancement flags for the kitty keyboard protocol.
    ///
    /// Represents special flags that tell compatible terminals to add extra
    /// information to keyboard events.
    ///
    /// See <https://sw.kovidgoyal.net/kitty/keyboard-protocol/#progressive-enhancement>
    /// for more information.
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(transparent))]
    #[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
    pub struct KeyboardEnhancementFlags: u8 {
        /// Represent Escape and modified keys using CSI-u sequences.
        ///
        /// This allows them to be unambiguously read.
        const DISAMBIGUATE_ESCAPE_CODES = 0b0000_0001;
        /// Add extra events with repeat or release event types.
        ///
        /// Add extra events when keys are autorepeated or released.
        const REPORT_EVENT_TYPES = 0b0000_0010;
        /// Send alternate keycodes in addition to the base keycode.
        ///
        /// Send alternate keycodes as described in the kitty keyboard
        /// protocol. The alternate keycode overrides the base keycode in
        /// resulting key events.
        const REPORT_ALTERNATE_KEYS = 0b0000_0100;
        /// Represent all keyboard events as CSI-u sequences.
        ///
        /// This is required to get repeat/release events for plain-text keys.
        const REPORT_ALL_KEYS_AS_ESCAPE_CODES = 0b0000_1000;
        /// Send the Unicode codepoint as well as the keycode.
        const REPORT_ASSOCIATED_TEXT = 0b0001_0000;
    }
}

impl Encode for KeyboardEnhancementFlags {
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        write_csi!(buf; "?", self.0.bits(), "u")
    }
}

/// Push Keyboard Enhancement Flags.
///
/// Enable the kitty keyboard protocol, which adds extra information to
/// keyboard events and removes ambiguity for modifier keys.
///
/// It should be paired with [`PopKeyboardEnhancementFlags`] to restore the
/// previous state.
///
/// See <https://sw.kovidgoyal.net/kitty/keyboard-protocol/> for more
/// information.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PushKeyboardEnhancementFlags(pub KeyboardEnhancementFlags);

impl Encode for PushKeyboardEnhancementFlags {
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        write_csi!(buf; ">", self.0.bits(), "u")
    }
}

/// Pop Keyboard Enhancement Flags.
///
/// Disable extra kinds of keyboard events.
///
/// Specifically, it pops one level of keyboard enhancement flags.
///
/// See [`PushKeyboardEnhancementFlags`] and
/// <https://sw.kovidgoyal.net/kitty/keyboard-protocol/> for more information.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PopKeyboardEnhancementFlags;

impl ConstEncode for PopKeyboardEnhancementFlags {
    const STR: &'static str = csi!("<1u");
}

/// Set Application Keypad Mode (`DECKPAM`).
///
/// Enable application keypad mode.
///
/// See <https://terminalguide.namepad.de/seq/esc_a_eq/> for
/// terminal support specifics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetApplicationKeypadMode;

impl ConstEncode for SetApplicationKeypadMode {
    const STR: &'static str = esc!("=");
}

/// Reset Application Keypad Mode (`DECKPNM`).
///
/// Disable application keypad mode.
///
/// See <https://terminalguide.namepad.de/seq/esc_a_gt/> for
/// terminal support specifics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResetApplicationKeypadMode;

impl ConstEncode for ResetApplicationKeypadMode {
    const STR: &'static str = esc!(">");
}
