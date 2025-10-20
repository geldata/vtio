//! Keyboard-related messages.

use bitflags::bitflags;

use vtenc::{ConstEncode, Encode, EncodeError, csi, esc, write_csi};

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
pub struct EnableApplicationKeypad;

impl ConstEncode for EnableApplicationKeypad {
    const STR: &'static str = esc!("=");
}

/// Disable application keypad mode (DECKPNM).
pub struct DisableApplicationKeypad;

impl ConstEncode for DisableApplicationKeypad {
    const STR: &'static str = esc!(">");
}
