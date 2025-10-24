//! Mouse mode control commands.
//!
//! See <https://terminalguide.namepad.de/mouse/> for details.

use vtenc::{ConstEncodedLen, Encode, EncodeError, const_composite, write_csi};

use crate::event::keyboard::KeyModifiers;
use vtio_control_derive::terminal_mode;

//
// Mouse event modes (mutually exclusive).
//
// These modes control what events are sent and their button encoding.
// The last activated mode wins.
//
// See <https://terminalguide.namepad.de/mouse/#events>
//

/// Mouse click-only tracking (`X10_MOUSE`).
///
/// Send mouse button press for left, middle, and right mouse
/// buttons.
///
/// Button encoding `btn` does not contain bits for modifiers,
/// but is the button number without moved bits.
///
/// See <https://terminalguide.namepad.de/mode/p9/> for
/// terminal support specifics.
terminal_mode!(MouseX10Mode, private = '?', params = ["9"]);

/// Mouse down+up tracking.
///
/// Send mouse button press and release. Also send scroll wheel
/// events.
///
/// See <https://terminalguide.namepad.de/mode/p1000/> for
/// terminal support specifics.
terminal_mode!(MouseDownUpTrackingMode, private = '?', params = ["1000"]);

/// Mouse highlight mode.
///
/// Like mouse down+up tracking, but shows a text selection.
///
/// Needs a cooperating application to avoid rendering the
/// terminal non-operative. xterm-only.
///
/// Note: This mode will make the terminal unresponsive if not
/// used correctly.
///
/// See <https://terminalguide.namepad.de/mode/p1001/> and
/// <https://terminalguide.namepad.de/mouse/#highlight-tracking>
/// for terminal support specifics.
terminal_mode!(MouseHighlightMode, private = '?', params = ["1001"]);

/// Mouse click and dragging tracking.
///
/// Send mouse button press and release. Send mouse move events
/// while a button is pressed. Also send scroll wheel events.
///
/// See <https://terminalguide.namepad.de/mode/p1002/> for
/// terminal support specifics.
terminal_mode!(MouseClickAndDragTrackingMode, private = '?', params = ["1002"]);

/// Mouse tracking with movement.
///
/// Send mouse button press and release. Always send mouse move
/// events. Also send scroll wheel events.
///
/// See <https://terminalguide.namepad.de/mode/p1003/> for
/// terminal support specifics.
terminal_mode!(MouseAnyEventTrackingMode, private = '?', params = ["1003"]);

//
// Mouse reporting format modes (mutually exclusive).
//
// These modes control which report encoding is used for mouse events.
// The last activated mode wins.
//
// See <https://terminalguide.namepad.de/mouse/#reporting-format>
//

/// Mouse report format multibyte mode.
///
/// Encodes mouse information with variable length byte
/// sequences.
///
/// For values < 96 the format is identical to the default mode.
/// Values above that boundary are encoded as 2 bytes as if
/// encoding codepoint value + 32 as UTF-8. This mode has a
/// range from 1 to 2015.
///
/// See <https://terminalguide.namepad.de/mode/p1005/> for
/// terminal support specifics.
terminal_mode!(MouseReportMultibyteMode, private = '?', params = ["1005"]);

/// Mouse reporting format digits (SGR mode).
///
/// Encodes mouse information with digit sequences.
///
/// Mouse information is reported as `ESC [ < btn ; column ; row M`
/// for button press or movement, and `ESC [ < btn ; column ; row m`
/// for button release. This mode does not have an arbitrary range
/// limit and is the preferred extended coordinate format.
///
/// See <https://terminalguide.namepad.de/mode/p1006/> for
/// terminal support specifics.
terminal_mode!(MouseReportSgrMode, private = '?', params = ["1006"]);

/// Mouse reporting format urxvt.
///
/// Encodes mouse information with digit sequences.
///
/// Mouse information is reported as `ESC [ btn ; column ; row M`.
/// For `btn` the encoded value is offset by the value 32. This
/// mode does not have an arbitrary range limit but is less
/// preferred than SGR mode.
///
/// See <https://terminalguide.namepad.de/mode/p1015/> for
/// terminal support specifics.
terminal_mode!(MouseReportRxvtMode, private = '?', params = ["1015"]);

//
// Additional mouse-related modes.
//

/// Report focus change.
///
/// When the terminal gains focus emit `ESC [ I`. When the
/// terminal loses focus emit `ESC [ O`.
///
/// See <https://terminalguide.namepad.de/mode/p1004/> for
/// terminal support specifics.
terminal_mode!(MouseReportFocusMode, private = '?', params = ["1004"]);

/// Send cursor keys on mouse wheel on alternative screen.
///
/// When the alternate screen is active and the mouse wheel is
/// used send arrow up and down.
///
/// The number of arrow up or arrow down sequences that are
/// transmitted is implementation defined.
///
/// All mouse reporting modes suppress this and report in their
/// specific format instead.
///
/// See <https://terminalguide.namepad.de/mode/p1007/> for
/// terminal support specifics.
terminal_mode!(MouseWheelToCursorKeysMode, private = '?', params = ["1007"]);

const_composite! {
    /// A command that enables mouse event capture.
    ///
    /// This command enables all mouse tracking modes and coordinate
    /// encoding formats for comprehensive mouse support.
    pub struct EnableMouseCapture = [
        EnableMouseDownUpTrackingMode,
        EnableMouseClickAndDragTrackingMode,
        EnableMouseAnyEventTrackingMode,
        EnableMouseReportRxvtMode,
        EnableMouseReportSgrMode,
    ];
}

const_composite! {
    /// A command that disables mouse event capture.
    ///
    /// This command disables all mouse tracking modes and coordinate
    /// encoding formats. The modes are disabled in reverse order of
    /// enablement.
    pub struct DisableMouseCapture = [
        DisableMouseReportSgrMode,
        DisableMouseReportRxvtMode,
        DisableMouseAnyEventTrackingMode,
        DisableMouseClickAndDragTrackingMode,
        DisableMouseDownUpTrackingMode,
    ];
}

/// Linux Mouse Pointer Style (`LINUX_MOUSE_POINTER_STYLE`).
///
/// Select Linux mouse pointer style with control over appearance.
///
/// This sequence allows setting the mouse pointer appearance by toggling
/// attribute bits and character glyph bits in the Linux virtual console.
///
/// The `attr_xor` parameter controls toggling of display attributes
/// similar to the Linux cursor style, but only allows toggling each
/// aspect (not enabling/disabling). Each bit controls one color channel:
///
/// | bit value |          meaning              |
/// |-----------|-------------------------------|
/// |         1 | foreground blue channel       |
/// |         2 | foreground green channel      |
/// |         4 | foreground red channel        |
/// |         8 | foreground brightness channel |
/// |        16 | background blue channel       |
/// |        32 | background green channel      |
/// |        64 | background red channel        |
/// |       128 | background brightness         |
///
/// The `char_xor` parameter allows toggling bits in the glyph index
/// into the terminal's font, effectively changing which character is
/// displayed at the mouse pointer position.
///
/// See <https://terminalguide.namepad.de/seq/csi_sm__p/> for terminal
/// support specifics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LinuxMousePointerStyle {
    /// XOR mask for attribute manipulation.
    pub attr_xor: u8,
    /// XOR mask for character glyph manipulation.
    pub char_xor: u8,
}

impl LinuxMousePointerStyle {
    /// Create a new Linux mouse pointer style with the specified
    /// parameters.
    #[must_use]
    pub const fn new(attr_xor: u8, char_xor: u8) -> Self {
        Self { attr_xor, char_xor }
    }
}

impl ConstEncodedLen for LinuxMousePointerStyle {
    // CSI (2) + max attr_xor (3) + ";" (1) + max char_xor (3) + "m" (1)
    // = 10
    const ENCODED_LEN: usize = 10;
}

impl Encode for LinuxMousePointerStyle {
    #[inline]
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        write_csi!(buf; self.attr_xor, ";", self.char_xor, "m")
    }
}

impl Encode for MouseEvent {
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        let mods = self.modifiers;

        // Calculate modifier offset for SGR mode
        let mod_offset = if mods.contains(KeyModifiers::SHIFT) {
            4
        } else {
            0
        } + if mods.contains(KeyModifiers::ALT) {
            8
        } else {
            0
        } + if mods.contains(KeyModifiers::CONTROL) {
            16
        } else {
            0
        };

        // Map mouse event kinds to SGR button codes
        let (base_button, final_char) = match self.kind {
            MouseEventKind::Down(button) => {
                let btn_code = match button {
                    MouseButton::Left => 0,
                    MouseButton::Middle => 1,
                    MouseButton::Right => 2,
                };
                (btn_code, b'M')
            }
            MouseEventKind::Up(button) => {
                let btn_code = match button {
                    MouseButton::Left => 0,
                    MouseButton::Middle => 1,
                    MouseButton::Right => 2,
                };
                (btn_code, b'm')
            }
            MouseEventKind::Drag(button) => {
                let btn_code = match button {
                    MouseButton::Left => 0,
                    MouseButton::Middle => 1,
                    MouseButton::Right => 2,
                };
                // Add drag bit (bit 5 = 32)
                (btn_code + 32, b'M')
            }
            MouseEventKind::Moved => {
                // Mouse move without button
                (3 + 32, b'M')
            }
            MouseEventKind::ScrollUp => (1 << 6, b'M'),
            MouseEventKind::ScrollDown => (1 << 6 | 1, b'M'),
            MouseEventKind::ScrollLeft => (1 << 6 | 2, b'M'),
            MouseEventKind::ScrollRight => (1 << 6 | 3, b'M'),
        };

        let button_code = base_button + mod_offset;

        // Convert coordinates (0-based to 1-based for SGR)
        let x = self.column + 1;
        let y = self.row + 1;

        // Generate SGR sequence: ESC[<btn;col;row(M|m)
        write_csi!(buf; "<", button_code, ";", x, ";", y, final_char as char)
    }
}

/// Represents a mouse event.
///
/// # Platform-specific Notes
///
/// ## Mouse Buttons
///
/// Some platforms/terminals do not report mouse button for the
/// `MouseEventKind::Up` and `MouseEventKind::Drag` events. `MouseButton::Left`
/// is returned if we don't know which button was used.
///
/// ## Key Modifiers
///
/// Some platforms/terminals does not report all key modifiers
/// combinations for all mouse event types. For example - macOS reports
/// `Ctrl` + left mouse button click as a right mouse button click.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct MouseEvent {
    /// The kind of mouse event that was caused.
    pub kind: MouseEventKind,
    /// The column that the event occurred on.
    pub column: u16,
    /// The row that the event occurred on.
    pub row: u16,
    /// The key modifiers active when the event occurred.
    pub modifiers: KeyModifiers,
}

/// A mouse event kind.
///
/// # Platform-specific Notes
///
/// ## Mouse Buttons
///
/// Some platforms/terminals do not report mouse button for the
/// `MouseEventKind::Up` and `MouseEventKind::Drag` events. `MouseButton::Left`
/// is returned if we don't know which button was used.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub enum MouseEventKind {
    /// Pressed mouse button. Contains the button that was pressed.
    Down(MouseButton),
    /// Released mouse button. Contains the button that was released.
    Up(MouseButton),
    /// Moved the mouse cursor while pressing the contained mouse button.
    Drag(MouseButton),
    /// Moved the mouse cursor while not pressing a mouse button.
    Moved,
    /// Scrolled mouse wheel downwards (towards the user).
    ScrollDown,
    /// Scrolled mouse wheel upwards (away from the user).
    ScrollUp,
    /// Scrolled mouse wheel left (mostly on a laptop touchpad).
    ScrollLeft,
    /// Scrolled mouse wheel right (mostly on a laptop touchpad).
    ScrollRight,
}

/// Represents a mouse button.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub enum MouseButton {
    /// Left mouse button.
    Left,
    /// Right mouse button.
    Right,
    /// Middle mouse button.
    Middle,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_mouse_event_down() {
        let mut event = MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: 0,
            row: 0,
            modifiers: KeyModifiers::NONE,
        };
        let mut buf = [0u8; 64];
        let len = event.encode(&mut &mut buf[..]).unwrap();
        assert_eq!(&buf[..len], b"\x1b[<0;1;1M");
    }

    #[test]
    fn test_encode_mouse_event_up() {
        let mut event = MouseEvent {
            kind: MouseEventKind::Up(MouseButton::Left),
            column: 0,
            row: 0,
            modifiers: KeyModifiers::NONE,
        };
        let mut buf = [0u8; 64];
        let len = event.encode(&mut &mut buf[..]).unwrap();
        assert_eq!(&buf[..len], b"\x1b[<0;1;1m");
    }

    #[test]
    fn test_encode_mouse_event_scroll() {
        let mut event = MouseEvent {
            kind: MouseEventKind::ScrollUp,
            column: 0,
            row: 0,
            modifiers: KeyModifiers::NONE,
        };
        let mut buf = [0u8; 64];
        let len = event.encode(&mut &mut buf[..]).unwrap();
        assert_eq!(&buf[..len], b"\x1b[<64;1;1M");
    }
}
