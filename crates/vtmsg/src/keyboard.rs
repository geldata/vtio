//! Keyboard-related messages.

use bitflags::bitflags;
use vtenc::{ConstEncode, Encode, EncodeError, csi, esc, write_csi};

use crate::terminal_mode;

terminal_mode!(
    #[doc = "Disable keyboard input (KAM)."]
    KeyboardInputDisabledMode,
    "2"
);

terminal_mode!(
    #[doc = "Cursor keys mode (DECCKM)."]
    CursorKeysMode,
    "?1"
);

terminal_mode!(
    #[doc = "Repeat Held Keys. \
            Repeat keys while being held down. If enabled a key held down \
            automatically repeats in an implementation specific interval."]
    HeldKeysRepeatMode,
    "?8"
);

terminal_mode!(
    #[doc = "Application Keypad Mode (DECNKM). \
            This is a mirror of [`SetApplicationKeypadMode`]."]
    ApplicationKeypadMode,
    "?66"
);

terminal_mode!(
    #[doc = "Backspace Sends Delete (DECBKM). \
            When set the backspace key sends BS, when reset it sends DEL."]
    BackspaceSendsDeleteMode,
    "?67"
);

terminal_mode!(
    #[doc = "Alt + Key Sends Character with High Bit Set. \
            Use high bit to signal keypresses with alt modifier held.\n\n\
            When reporting key presses use the high (eighth) bit to indicate \
            alt modifier. (This will collide with non ASCII characters)"]
    AltKeyHighBitSetMode,
    "?1034"
);

terminal_mode!(
    #[doc = "Ignore Keypad Application Mode When Numlock is Active. \
            Use high bit to signal keypresses with alt modifier held.\n\n\
            When reporting key presses use the high (eighth) bit to indicate \
            alt modifier. (This will collide with non ASCII characters)"]
    IgnoreKeypadApplicationModeOnNumlockMode,
    "?1035"
);

terminal_mode!(
    #[doc = "Alt + Key Sends Esc as Prefix.\n\n\
            When this mode is active Alt + Key sends ESC + the key for \
            printable inputs instead of forcing the 8th bit of the character \
            to high."]
    AltKeySendsEscPrefixMode,
    "?1036"
);

terminal_mode!(
    #[doc = "Delete Key sends DEL.\n\n\
            If set use legacy DEL for the delete key instead of an \
            escape sequence."]
    DeleteKeySendsDELMode,
    "?1037"
);

terminal_mode!(
    #[doc = "Additional Modifier + Key Sends Esc as Prefix.\n\n\
            This is similar to [`AltKeySendsEscPrefixMode`], but for \
            an additionally configured modifier."]
    AdditionalModifierKeySendsEscPrefix,
    "?1039"
);


bitflags! {
    /// Represents special flags that tell compatible terminals to add extra information to keyboard events.
    ///
    /// See <https://sw.kovidgoyal.net/kitty/keyboard-protocol/#progressive-enhancement> for more information.
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(transparent))]
    #[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
    pub struct KeyboardEnhancementFlags: u8 {
        /// Represent Escape and modified keys using CSI-u sequences, so they can be unambiguously
        /// read.
        const DISAMBIGUATE_ESCAPE_CODES = 0b0000_0001;
        /// Add extra events with [`KeyEvent.kind`] set to [`KeyEventKind::Repeat`] or
        /// [`KeyEventKind::Release`] when keys are autorepeated or released.
        const REPORT_EVENT_TYPES = 0b0000_0010;
        /// Send [alternate keycodes](https://sw.kovidgoyal.net/kitty/keyboard-protocol/#key-codes)
        /// in addition to the base keycode. The alternate keycode overrides the base keycode in
        /// resulting `KeyEvent`s.
        const REPORT_ALTERNATE_KEYS = 0b0000_0100;
        /// Represent all keyboard events as CSI-u sequences. This is required to get repeat/release
        /// events for plain-text keys.
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

/// A command that enables the [kitty keyboard protocol](https://sw.kovidgoyal.net/kitty/keyboard-protocol/), which adds extra information to keyboard events and removes ambiguity for modifier keys.
///
/// It should be paired with [`PopKeyboardEnhancementFlags`] to restore the state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PushKeyboardEnhancementFlags(pub KeyboardEnhancementFlags);

impl Encode for PushKeyboardEnhancementFlags {
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        write_csi!(buf; ">", self.0.bits(), "u")
    }
}

/// A command that disables extra kinds of keyboard events.
///
/// Specifically, it pops one level of keyboard enhancement flags.
///
/// See [`PushKeyboardEnhancementFlags`] and <https://sw.kovidgoyal.net/kitty/keyboard-protocol/> for more information.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PopKeyboardEnhancementFlags;

impl ConstEncode for PopKeyboardEnhancementFlags {
    const STR: &'static str = csi!("<1u");
}

/// Enable application keypad mode (DECKPAM).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetApplicationKeypadMode;

impl ConstEncode for SetApplicationKeypadMode {
    const STR: &'static str = esc!("=");
}

/// Disable application keypad mode (DECKPNM).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResetApplicationKeypadMode;

impl ConstEncode for ResetApplicationKeypadMode {
    const STR: &'static str = esc!(">");
}
