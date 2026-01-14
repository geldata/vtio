//! Keyboard modifier types and utilities.

use std::fmt::{self, Display, Write};

use super::util::parse_colon_separated;
use vtansi::bitflags;
use vtansi::{AnsiEncode, EncodeError, TryFromAnsi};

use crate::TerseDisplay;

/// Represents a keyboard event kind.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub enum KeyEventKind {
    Press,
    Repeat,
    Release,
}

impl TerseDisplay for KeyEventKind {
    fn terse_fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KeyEventKind::Press => f.write_str("press"),
            KeyEventKind::Repeat => f.write_str("repeat"),
            KeyEventKind::Release => f.write_str("release"),
        }
    }
}

bitflags! {
    /// Represents extra state about the key event.
    ///
    /// **Note:** This state can only be read if
    /// [`KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES`] has been enabled with
    /// [`PushKeyboardEnhancementFlags`].
    #[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(transparent))]
    pub struct KeyEventState: u8 {
        /// The key event origins from the keypad.
        const KEYPAD = 0b0000_0001;
        /// Caps Lock was enabled for this key event.
        ///
        /// **Note:** this is set for the initial press of Caps Lock itself.
        const CAPS_LOCK = 0b0000_0010;
        /// Num Lock was enabled for this key event.
        ///
        /// **Note:** this is set for the initial press of Num Lock itself.
        const NUM_LOCK = 0b0000_0100;
        const NONE = 0b0000_0000;
    }
}

// Platform-specific modifier key names
// See: https://support.apple.com/guide/applestyleguide/welcome/1.0/web (macOS)
// See: https://learn.microsoft.com/en-us/style-guide/a-z-word-list-term-collections/term-collections/keys-keyboard-shortcuts (Windows)

#[cfg(target_os = "macos")]
pub(super) const CONTROL_NAME: &str = "Control";
#[cfg(not(target_os = "macos"))]
pub(super) const CONTROL_NAME: &str = "Ctrl";

#[cfg(target_os = "macos")]
pub(super) const ALT_NAME: &str = "Option";
#[cfg(not(target_os = "macos"))]
pub(super) const ALT_NAME: &str = "Alt";

#[cfg(target_os = "macos")]
pub(super) const SUPER_NAME: &str = "Command";
#[cfg(target_os = "windows")]
pub(super) const SUPER_NAME: &str = "Windows";
#[cfg(not(any(target_os = "macos", target_os = "windows")))]
pub(super) const SUPER_NAME: &str = "Super";

bitflags! {
    /// Represents key modifiers (shift, control, alt, etc.).
    ///
    /// **Note:** `SUPER`, `HYPER`, and `META` can only be read if
    /// [`KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES`] has been enabled with
    /// [`PushKeyboardEnhancementFlags`].
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(transparent))]
    #[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
    pub struct KeyModifiers: u8 {
        const SHIFT = 0b0000_0001;
        const CONTROL = 0b0000_0010;
        const ALT = 0b0000_0100;
        const SUPER = 0b0000_1000;
        const HYPER = 0b0001_0000;
        const META = 0b0010_0000;
        const NONE = 0b0000_0000;
    }
}

impl Default for KeyModifiers {
    fn default() -> Self {
        Self::NONE
    }
}

/// Mapping between `KeyModifiers` flags and xterm parameter bits.
///
/// The xterm encoding uses different bit positions than [`KeyModifiers`]:
///
/// | Modifier | `KeyModifiers` | xterm bits |
/// |----------|----------------|------------|
/// | Shift    | bit 0 (1)      | bit 0 (1)  |
/// | Control  | bit 1 (2)      | bit 2 (4)  |
/// | Alt      | bit 2 (4)      | bit 1 (2)  |
/// | Meta     | bit 5 (32)     | bit 3 (8)  |
const XTERM_MODIFIER_BITS: &[(KeyModifiers, u8)] = &[
    (KeyModifiers::SHIFT, 1),
    (KeyModifiers::ALT, 2),
    (KeyModifiers::CONTROL, 4),
    (KeyModifiers::META, 8),
];

/// Kitty keyboard protocol modifier bit mapping.
///
/// Kitty uses different bit positions than our internal `KeyModifiers`:
/// - Kitty: shift=1, alt=2, ctrl=4, super=8, hyper=16, meta=32
/// - Ours:  SHIFT=1, CONTROL=2, ALT=4, SUPER=8, HYPER=16, META=32
///
/// So Kitty's alt (bit 1) maps to our ALT (bit 2), and
/// Kitty's ctrl (bit 2) maps to our CONTROL (bit 1).
const KITTY_MODIFIER_BITS: &[(u8, KeyModifiers)] = &[
    (1, KeyModifiers::SHIFT),   // Kitty bit 0 -> SHIFT
    (2, KeyModifiers::ALT),     // Kitty bit 1 -> ALT (not CONTROL!)
    (4, KeyModifiers::CONTROL), // Kitty bit 2 -> CONTROL (not ALT!)
    (8, KeyModifiers::SUPER),   // Kitty bit 3 -> SUPER
    (16, KeyModifiers::HYPER),  // Kitty bit 4 -> HYPER
    (32, KeyModifiers::META),   // Kitty bit 5 -> META
];

/// Parse modifiers and event type from the Kitty keyboard protocol format.
///
/// The format is `modifiers:event-type` where:
/// - `modifiers` is `1 + actual_modifier_bits`
/// - `event-type` is 1 (press), 2 (repeat), or 3 (release)
///
/// Returns (`KeyModifiers`, `KeyEventKind`, `KeyEventState`) tuple.
pub(crate) fn parse_csi_u_modifiers(
    bytes: &[u8],
) -> (KeyModifiers, KeyEventKind, KeyEventState) {
    let mut parts = parse_colon_separated(bytes);

    // Parse modifier value (1 + bits)
    let mod_value = parts.next().flatten().unwrap_or(1);
    #[allow(clippy::cast_possible_truncation)]
    let mod_bits = mod_value.saturating_sub(1) as u8;

    // Map Kitty modifier bits to our KeyModifiers
    let mut modifiers = KeyModifiers::NONE;
    for (kitty_bit, our_modifier) in KITTY_MODIFIER_BITS {
        if mod_bits & kitty_bit != 0 {
            modifiers |= *our_modifier;
        }
    }

    // caps_lock (64) and num_lock (128) go to KeyEventState
    let mut state = KeyEventState::NONE;
    if mod_bits & 0x40 != 0 {
        state |= KeyEventState::CAPS_LOCK;
    }
    if mod_bits & 0x80 != 0 {
        state |= KeyEventState::NUM_LOCK;
    }

    // Parse event type
    let event_type = parts.next().flatten().unwrap_or(1);
    let kind = match event_type {
        2 => KeyEventKind::Repeat,
        3 => KeyEventKind::Release,
        _ => KeyEventKind::Press,
    };

    (modifiers, kind, state)
}

impl KeyModifiers {
    /// Encode modifiers as an xterm-style parameter.
    ///
    /// The final parameter is `1 + bits`, so unmodified keys have param=1.
    /// This format is used in CSI sequences like `CSI 1;{param} A` for modified
    /// arrow keys.
    ///
    /// See `XTERM_MODIFIER_BITS` for the bit mapping.
    #[inline]
    #[must_use]
    pub fn to_xterm_param(self) -> u8 {
        let mut bits = 0u8;
        for (modifier, xterm_bit) in XTERM_MODIFIER_BITS {
            if self.contains(*modifier) {
                bits |= xterm_bit;
            }
        }
        1 + bits
    }

    /// Decode modifiers from an xterm-style parameter.
    ///
    /// See `XTERM_MODIFIER_BITS` for the bit mapping.
    #[inline]
    #[must_use]
    pub fn from_xterm_param(param: u8) -> Self {
        let bits = param.saturating_sub(1);
        let mut mods = Self::empty();
        for (modifier, xterm_bit) in XTERM_MODIFIER_BITS {
            if bits & xterm_bit != 0 {
                mods |= *modifier;
            }
        }
        mods
    }
}

impl TerseDisplay for KeyModifiers {
    fn terse_fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Output modifiers in conventional order: ctrl-alt-shift-super-hyper-meta
        const MODIFIERS: &[(KeyModifiers, &str)] = &[
            (KeyModifiers::CONTROL, "ctrl"),
            (KeyModifiers::ALT, "alt"),
            (KeyModifiers::SHIFT, "shift"),
            (KeyModifiers::SUPER, "super"),
            (KeyModifiers::HYPER, "hyper"),
            (KeyModifiers::META, "meta"),
        ];

        let mut first = true;
        for (modifier, name) in MODIFIERS {
            if self.contains(*modifier) {
                if !first {
                    f.write_char('-')?;
                }
                first = false;
                f.write_str(name)?;
            }
        }
        Ok(())
    }
}

impl Display for KeyModifiers {
    /// Formats the key modifiers using the given formatter.
    ///
    /// The key modifiers are joined by a `+` character.
    ///
    /// # Platform-specific Notes
    ///
    /// On macOS, the control, alt, and super keys is displayed as "Control",
    /// "Option", and "Command" respectively. See
    /// <https://support.apple.com/guide/applestyleguide/welcome/1.0/web>.
    ///
    /// On Windows, the super key is displayed as "Windows" and the control key
    /// is displayed as "Ctrl". See
    /// <https://learn.microsoft.com/en-us/style-guide/a-z-word-list-term-collections/term-collections/keys-keyboard-shortcuts>.
    ///
    /// On other platforms, the super key is referred to as "Super" and the
    /// control key is displayed as "Ctrl".
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        for modifier in self.iter() {
            if !first {
                f.write_str("+")?;
            }

            first = false;
            match modifier {
                KeyModifiers::SHIFT => f.write_str("Shift")?,
                KeyModifiers::CONTROL => f.write_str(CONTROL_NAME)?,
                KeyModifiers::ALT => f.write_str(ALT_NAME)?,
                KeyModifiers::SUPER => f.write_str(SUPER_NAME)?,
                KeyModifiers::HYPER => f.write_str("Hyper")?,
                KeyModifiers::META => f.write_str("Meta")?,
                _ => unreachable!(),
            }
        }
        Ok(())
    }
}

/// Represents a modifier key (as part of [`KeyCode::Modifier`](super::KeyCode::Modifier)).
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ModifierKeyCode {
    /// Left Shift key.
    LeftShift,
    /// Left Control key. (Control on macOS, Ctrl on other platforms)
    LeftControl,
    /// Left Alt key. (Option on macOS, Alt on other platforms)
    LeftAlt,
    /// Left Super key. (Command on macOS, Windows on Windows, Super on other platforms)
    LeftSuper,
    /// Left Hyper key.
    LeftHyper,
    /// Left Meta key.
    LeftMeta,
    /// Right Shift key.
    RightShift,
    /// Right Control key. (Control on macOS, Ctrl on other platforms)
    RightControl,
    /// Right Alt key. (Option on macOS, Alt on other platforms)
    RightAlt,
    /// Right Super key. (Command on macOS, Windows on Windows, Super on other platforms)
    RightSuper,
    /// Right Hyper key.
    RightHyper,
    /// Right Meta key.
    RightMeta,
    /// Iso Level3 Shift key.
    IsoLevel3Shift,
    /// Iso Level5 Shift key.
    IsoLevel5Shift,
}

impl From<ModifierKeyCode> for KeyModifiers {
    fn from(value: ModifierKeyCode) -> Self {
        match value {
            ModifierKeyCode::LeftShift | ModifierKeyCode::RightShift => {
                KeyModifiers::SHIFT
            }
            ModifierKeyCode::LeftControl | ModifierKeyCode::RightControl => {
                KeyModifiers::CONTROL
            }
            ModifierKeyCode::LeftAlt | ModifierKeyCode::RightAlt => {
                KeyModifiers::ALT
            }
            ModifierKeyCode::LeftSuper | ModifierKeyCode::RightSuper => {
                KeyModifiers::SUPER
            }
            ModifierKeyCode::LeftHyper | ModifierKeyCode::RightHyper => {
                KeyModifiers::HYPER
            }
            ModifierKeyCode::LeftMeta | ModifierKeyCode::RightMeta => {
                KeyModifiers::META
            }
            _ => KeyModifiers::NONE,
        }
    }
}

impl Display for ModifierKeyCode {
    /// Formats the modifier key using the given formatter.
    ///
    /// # Platform-specific Notes
    ///
    /// On macOS, the control, alt, and super keys are displayed as "Control",
    /// "Option", and "Command" respectively. See
    /// <https://support.apple.com/guide/applestyleguide/welcome/1.0/web>.
    ///
    /// On Windows, the super key is displayed as "Windows" and the control key
    /// is displayed as "Ctrl". See
    /// <https://learn.microsoft.com/en-us/style-guide/a-z-word-list-term-collections/term-collections/keys-keyboard-shortcuts>.
    ///
    /// On other platforms, the super key is referred to as "Super".
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // For modifiers with platform-specific names, we need write! for interpolation.
        // For others, f.write_str is more efficient.
        match self {
            ModifierKeyCode::LeftShift => f.write_str("Left Shift"),
            ModifierKeyCode::LeftControl => write!(f, "Left {CONTROL_NAME}"),
            ModifierKeyCode::LeftAlt => write!(f, "Left {ALT_NAME}"),
            ModifierKeyCode::LeftSuper => write!(f, "Left {SUPER_NAME}"),
            ModifierKeyCode::LeftHyper => f.write_str("Left Hyper"),
            ModifierKeyCode::LeftMeta => f.write_str("Left Meta"),
            ModifierKeyCode::RightShift => f.write_str("Right Shift"),
            ModifierKeyCode::RightControl => write!(f, "Right {CONTROL_NAME}"),
            ModifierKeyCode::RightAlt => write!(f, "Right {ALT_NAME}"),
            ModifierKeyCode::RightSuper => write!(f, "Right {SUPER_NAME}"),
            ModifierKeyCode::RightHyper => f.write_str("Right Hyper"),
            ModifierKeyCode::RightMeta => f.write_str("Right Meta"),
            ModifierKeyCode::IsoLevel3Shift => f.write_str("Iso Level 3 Shift"),
            ModifierKeyCode::IsoLevel5Shift => f.write_str("Iso Level 5 Shift"),
        }
    }
}

/// CSI-encoded key modifiers wrapper.
///
/// This wrapper handles the conversion between [`KeyModifiers`] and the
/// xterm-style parameter encoding used in CSI sequences.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Hash)]
pub struct CsiKeyModifiers(pub KeyModifiers);

impl From<CsiKeyModifiers> for KeyModifiers {
    fn from(value: CsiKeyModifiers) -> Self {
        value.0
    }
}

impl<'a> TryFromAnsi<'a> for CsiKeyModifiers {
    fn try_from_ansi(bytes: &'a [u8]) -> Result<Self, vtansi::ParseError> {
        let param = <u8 as TryFromAnsi>::try_from_ansi(bytes)?;
        Ok(Self(KeyModifiers::from_xterm_param(param)))
    }
}

impl AnsiEncode for CsiKeyModifiers {
    fn encode_ansi_into<W: std::io::Write + ?Sized>(
        &self,
        sink: &mut W,
    ) -> Result<usize, EncodeError> {
        <u8 as AnsiEncode>::encode_ansi_into(&self.0.to_xterm_param(), sink)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ModifierKeyCode::*;

    #[test]
    fn modifier_keycode_display() {
        assert_eq!(format!("{LeftShift}"), "Left Shift");
        assert_eq!(format!("{LeftHyper}"), "Left Hyper");
        assert_eq!(format!("{LeftMeta}"), "Left Meta");
        assert_eq!(format!("{RightShift}"), "Right Shift");
        assert_eq!(format!("{RightHyper}"), "Right Hyper");
        assert_eq!(format!("{RightMeta}"), "Right Meta");
        assert_eq!(format!("{IsoLevel3Shift}"), "Iso Level 3 Shift");
        assert_eq!(format!("{IsoLevel5Shift}"), "Iso Level 5 Shift");
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn modifier_keycode_display_macos() {
        assert_eq!(format!("{LeftControl}"), "Left Control");
        assert_eq!(format!("{LeftAlt}"), "Left Option");
        assert_eq!(format!("{LeftSuper}"), "Left Command");
        assert_eq!(format!("{RightControl}"), "Right Control");
        assert_eq!(format!("{RightAlt}"), "Right Option");
        assert_eq!(format!("{RightSuper}"), "Right Command");
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn modifier_keycode_display_windows() {
        assert_eq!(format!("{LeftControl}"), "Left Ctrl");
        assert_eq!(format!("{LeftAlt}"), "Left Alt");
        assert_eq!(format!("{LeftSuper}"), "Left Windows");
        assert_eq!(format!("{RightControl}"), "Right Ctrl");
        assert_eq!(format!("{RightAlt}"), "Right Alt");
        assert_eq!(format!("{RightSuper}"), "Right Windows");
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    #[test]
    fn modifier_keycode_display_other() {
        assert_eq!(format!("{LeftControl}"), "Left Ctrl");
        assert_eq!(format!("{LeftAlt}"), "Left Alt");
        assert_eq!(format!("{LeftSuper}"), "Left Super");
        assert_eq!(format!("{RightControl}"), "Right Ctrl");
        assert_eq!(format!("{RightAlt}"), "Right Alt");
        assert_eq!(format!("{RightSuper}"), "Right Super");
    }

    #[test]
    fn key_modifiers_display() {
        let modifiers =
            KeyModifiers::SHIFT | KeyModifiers::CONTROL | KeyModifiers::ALT;

        #[cfg(target_os = "macos")]
        assert_eq!(modifiers.to_string(), "Shift+Control+Option");

        #[cfg(target_os = "windows")]
        assert_eq!(modifiers.to_string(), "Shift+Ctrl+Alt");

        #[cfg(not(any(target_os = "macos", target_os = "windows")))]
        assert_eq!(modifiers.to_string(), "Shift+Ctrl+Alt");
    }

    #[test]
    fn xterm_param_round_trip() {
        let mods =
            KeyModifiers::SHIFT | KeyModifiers::CONTROL | KeyModifiers::ALT;
        let param = mods.to_xterm_param();
        let decoded = KeyModifiers::from_xterm_param(param);
        assert_eq!(mods, decoded);
    }

    #[test]
    fn xterm_param_empty() {
        let mods = KeyModifiers::NONE;
        assert_eq!(mods.to_xterm_param(), 1);
        assert_eq!(KeyModifiers::from_xterm_param(1), KeyModifiers::NONE);
    }
}
