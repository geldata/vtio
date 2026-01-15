//! Keyboard-related messages.

mod encoding;
mod event;
mod keycode;
mod mode;
mod modifier;
mod util;

pub use encoding::{bytes_to_events, get_key_event_encoding};
pub use event::{KeyEvent, KeyEventBuilder};
pub use keycode::{KeyCode, MediaKeyCode};
pub use mode::{
    KeyboardEnhancementFlags, KeyboardEnhancementFlagsQuery,
    KeyboardEnhancementFlagsResponse, PopKeyboardEnhancementFlags,
    PushKeyboardEnhancementFlags, ResetApplicationKeypadMode,
    SetApplicationKeypadMode,
};
pub use modifier::{
    KeyEventKind, KeyEventState, KeyModifiers, ModifierKeyCode,
};

use crate::terminal_mode;

terminal_mode!(
    /// Disable Keyboard Input (`KAM`).
    ///
    /// When this mode is active it disables all keyboard input.
    ///
    /// See <https://terminalguide.namepad.de/mode/2/> for
    /// terminal support specifics.
    KeyboardInputDisabledMode, params = ["2"]
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
    CursorKeysMode, private = '?', params = ["1"]
);

terminal_mode!(
    /// Repeat Held Keys.
    ///
    /// Repeat keys while being held down. If enabled a key held down
    /// automatically repeats in an implementation specific interval.
    ///
    /// See <https://terminalguide.namepad.de/mode/p8/> for
    /// terminal support specifics.
    HeldKeysRepeatMode, private = '?', params = ["8"]
);

terminal_mode!(
    /// Application Keypad Mode (`DECNKM`).
    ///
    /// This is a mirror of [`SetApplicationKeypadMode`].
    ///
    /// See <https://terminalguide.namepad.de/mode/p66/> for
    /// terminal support specifics.
    ApplicationKeypadMode, private = '?', params = ["66"]
);

terminal_mode!(
    /// Backspace Sends Delete (`DECBKM`).
    ///
    /// When set the backspace key sends BS, when reset it sends DEL.
    ///
    /// See <https://terminalguide.namepad.de/mode/p67/> for
    /// terminal support specifics.
    BackspaceSendsDeleteMode, private = '?', params = ["67"]
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
    AltKeyHighBitSetMode, private = '?', params = ["1034"]
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
    IgnoreKeypadApplicationModeOnNumlockMode, private = '?', params = ["1035"]
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
    AltKeySendsEscPrefixMode, private = '?', params = ["1036"]
);

terminal_mode!(
    /// Delete Key sends DEL.
    ///
    /// If set use legacy DEL for the delete key instead of an
    /// escape sequence.
    ///
    /// See <https://terminalguide.namepad.de/mode/p1037/> for
    /// terminal support specifics.
    DeleteKeySendsDELMode, private = '?', params = ["1037"]
);

terminal_mode!(
    /// Additional Modifier + Key Sends Esc as Prefix.
    ///
    /// This is similar to [`AltKeySendsEscPrefixMode`], but for
    /// an additionally configured modifier.
    ///
    /// See <https://terminalguide.namepad.de/mode/p1039/> for
    /// terminal support specifics.
    AdditionalModifierKeySendsEscPrefix, private = '?', params = ["1039"]
);
