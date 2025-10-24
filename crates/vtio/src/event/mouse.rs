//! Mouse mode control commands.
//!
//! See <https://terminalguide.namepad.de/mouse/> for details.

use vtenc::{IntoSeq, WriteSeq, const_composite};
use vtio_control_base::{FinalByte, EscapeSequenceParam};
use vtio_control_derive::{terminal_mode, VTControl};

use crate::event::keyboard::KeyModifiers;

//
// Mouse event modes (mutually exclusive).
//
// These modes control what events are sent and their button encoding.
// The last activated mode wins.
//
// See <https://terminalguide.namepad.de/mouse/#events>
//

terminal_mode!(
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
    MouseX10Mode, private = '?', params = ["9"]
);

terminal_mode!(
    /// Mouse down+up tracking.
    ///
    /// Send mouse button press and release. Also send scroll wheel
    /// events.
    ///
    /// See <https://terminalguide.namepad.de/mode/p1000/> for
    /// terminal support specifics.
    MouseDownUpTrackingMode, private = '?', params = ["1000"]
);

terminal_mode!(
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
    MouseHighlightMode, private = '?', params = ["1001"]
);

terminal_mode!(
    /// Mouse click and dragging tracking.
    ///
    /// Send mouse button press and release. Send mouse move events
    /// while a button is pressed. Also send scroll wheel events.
    ///
    /// See <https://terminalguide.namepad.de/mode/p1002/> for
    /// terminal support specifics.
    MouseClickAndDragTrackingMode, private = '?', params = ["1002"]
);

terminal_mode!(
    /// Mouse tracking with movement.
    ///
    /// Send mouse button press and release. Always send mouse move
    /// events. Also send scroll wheel events.
    ///
    /// See <https://terminalguide.namepad.de/mode/p1003/> for
    /// terminal support specifics.
    MouseAnyEventTrackingMode, private = '?', params = ["1003"]
);

//
// Mouse reporting format modes (mutually exclusive).
//
// These modes control which report encoding is used for mouse events.
// The last activated mode wins.
//
// See <https://terminalguide.namepad.de/mouse/#reporting-format>
//

terminal_mode!(
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
    MouseReportMultibyteMode, private = '?', params = ["1005"]
);

terminal_mode!(
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
    MouseReportSgrMode, private = '?', params = ["1006"]
);

terminal_mode!(
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
    MouseReportRxvtMode, private = '?', params = ["1015"]
);

//
// Additional mouse-related modes.
//

terminal_mode!(
    /// Report focus change.
    ///
    /// When the terminal gains focus emit `ESC [ I`. When the
    /// terminal loses focus emit `ESC [ O`.
    ///
    /// See <https://terminalguide.namepad.de/mode/p1004/> for
    /// terminal support specifics.
    MouseReportFocusMode, private = '?', params = ["1004"]
);

terminal_mode!(
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
    MouseWheelToCursorKeysMode, private = '?', params = ["1007"]
);

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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, VTControl)]
#[vtctl(csi, finalbyte = 'm')]
pub struct LinuxMousePointerStyle {
    /// XOR mask for attribute manipulation.
    pub attr_xor: u8,
    /// XOR mask for character glyph manipulation.
    pub char_xor: u8,
}

/// Terminal coordinates (column and row).
///
/// Both coordinates are 0-based for internal representation but are
/// converted to 1-based when encoding for terminal sequences.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Coordinates {
    /// Column position (0-based).
    pub column: u16,
    /// Row position (0-based).
    pub row: u16,
}

impl Coordinates {
    /// Create new coordinates.
    #[must_use]
    pub const fn new(column: u16, row: u16) -> Self {
        Self { column, row }
    }
}

impl IntoSeq for Coordinates {
    fn into_seq(&self) -> impl WriteSeq {
        // Coordinates are encoded as two separate parameters with a
        // semicolon between them. Convert to 1-based for terminal
        // sequences.
        CoordinatesSeq {
            column: self.column + 1,
            row: self.row + 1,
        }
    }
}

struct CoordinatesSeq {
    column: u16,
    row: u16,
}

impl WriteSeq for CoordinatesSeq {
    fn write_seq<W: std::io::Write + ?Sized>(
        &self,
        buf: &mut W,
    ) -> Result<usize, vtenc::EncodeError> {
        use vtenc::encode::{write_str_into, WriteSeq};
        let mut total = 0;
        total += WriteSeq::write_seq(&self.column, buf)?;
        total += write_str_into(buf, ";")?;
        total += WriteSeq::write_seq(&self.row, buf)?;
        Ok(total)
    }
}

impl From<EscapeSequenceParam> for Coordinates {
    fn from(param: EscapeSequenceParam) -> Self {
        // Coordinates are encoded as separate params, but we receive
        // them as a single param here. Extract first value only.
        let col = param.first() as u16;
        Self {
            column: col.saturating_sub(1),
            row: 0, // Will be set from second param
        }
    }
}

impl From<&EscapeSequenceParam> for Coordinates {
    fn from(param: &EscapeSequenceParam) -> Self {
        Self::from(param.clone())
    }
}

/// Represents a mouse event in SGR format.
///
/// This structure encodes mouse events using the SGR mouse reporting
/// format, which uses sequences like `ESC[<btn;col;row;M` for button
/// press/movement and `ESC[<btn;col;row;m` for button release.
///
/// The button code is computed from the event kind and modifiers
/// during encoding. During parsing, both the kind and modifiers are
/// extracted from the button code.
///
/// # Platform-specific Notes
///
/// ## Mouse Buttons
///
/// Some platforms/terminals do not report mouse button for the
/// `MouseEventKind::Up` and `MouseEventKind::Drag` events.
/// `MouseButton::Left` is returned if we don't know which button was
/// used.
///
/// ## Key Modifiers
///
/// Some platforms/terminals does not report all key modifiers
/// combinations for all mouse event types. For example - macOS reports
/// `Ctrl` + left mouse button click as a right mouse button click.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash, VTControl)]
#[vtctl(csi, intermediate = "<", finalbyte = 'M' | 'm')]
pub struct MouseEvent {
    /// The kind of mouse event that was caused.
    pub kind: MouseEventKind,
    /// The key modifiers active when the event occurred.
    /// Encoded in parameter 0 (button code).
    #[vtctl(paramidx = 0)]
    pub modifiers: KeyModifiers,
    /// The coordinates where the event occurred.
    pub coords: Coordinates,
}

impl FinalByte for MouseEvent {
    fn final_byte(&self) -> u8 {
        self.kind.final_byte()
    }
}

impl MouseEvent {
    /// Get the column where the event occurred.
    #[must_use]
    pub const fn column(&self) -> u16 {
        self.coords.column
    }

    /// Get the row where the event occurred.
    #[must_use]
    pub const fn row(&self) -> u16 {
        self.coords.row
    }

    /// Calculate modifier offset for SGR mode encoding.
    const fn sgr_modifier_offset(self) -> u16 {
        let mut offset = 0;
        if self.modifiers.contains(KeyModifiers::SHIFT) {
            offset += 4;
        }
        if self.modifiers.contains(KeyModifiers::ALT) {
            offset += 8;
        }
        if self.modifiers.contains(KeyModifiers::CONTROL) {
            offset += 16;
        }
        offset
    }

    /// Compute the SGR button code from kind and modifiers.
    fn sgr_button_code(&self) -> u16 {
        let base_button = self.kind.base_button_code();
        let mod_offset = self.sgr_modifier_offset();
        base_button + mod_offset
    }
}

/// A mouse event kind.
///
/// # Platform-specific Notes
///
/// ## Mouse Buttons
///
/// Some platforms/terminals do not report mouse button for the
/// `MouseEventKind::Up` and `MouseEventKind::Drag` events.
/// `MouseButton::Left` is returned if we don't know which button was
/// used.
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

impl MouseEventKind {
    /// Get the base SGR button code (without modifiers).
    const fn base_button_code(self) -> u16 {
        match self {
            MouseEventKind::Down(button) | MouseEventKind::Up(button) => button.code(),
            MouseEventKind::Drag(button) => button.code() + 32, // Add drag bit
            MouseEventKind::Moved => 3 + 32,                    // Mouse move without button
            MouseEventKind::ScrollUp => 1 << 6,
            MouseEventKind::ScrollDown => (1 << 6) | 1,
            MouseEventKind::ScrollLeft => (1 << 6) | 2,
            MouseEventKind::ScrollRight => (1 << 6) | 3,
        }
    }

    /// Get the final byte for SGR encoding ('M' or 'm').
    const fn final_byte(self) -> u8 {
        match self {
            MouseEventKind::Up(_) => b'm',
            _ => b'M',
        }
    }
}

impl IntoSeq for MouseEventKind {
    fn into_seq(&self) -> impl WriteSeq {
        self.base_button_code()
    }
}

impl From<EscapeSequenceParam> for MouseEventKind {
    fn from(param: EscapeSequenceParam) -> Self {
        let code = param.first() as u16;
        // Parse SGR button code (mask out modifier bits)
        let base_code = code & !0x1C; // Remove shift (4), alt (8), ctrl (16) bits
        let is_drag = (code & 32) != 0;

        if base_code >= 64 {
            // Scroll events
            match base_code & 0x03 {
                0 => MouseEventKind::ScrollUp,
                1 => MouseEventKind::ScrollDown,
                2 => MouseEventKind::ScrollLeft,
                _ => MouseEventKind::ScrollRight,
            }
        } else if (base_code & !32) == 3 && is_drag {
            // Mouse move without button (code 3 + drag bit)
            MouseEventKind::Moved
        } else {
            let button = MouseButton::from_code((base_code & 0x03) as u8);
            if is_drag {
                MouseEventKind::Drag(button)
            } else {
                MouseEventKind::Down(button)
            }
        }
    }
}

impl From<&EscapeSequenceParam> for MouseEventKind {
    fn from(param: &EscapeSequenceParam) -> Self {
        Self::from(param.clone())
    }
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

impl MouseButton {
    /// Get the SGR button code.
    const fn code(self) -> u16 {
        match self {
            MouseButton::Left => 0,
            MouseButton::Middle => 1,
            MouseButton::Right => 2,
        }
    }

    /// Create from SGR button code.
    const fn from_code(code: u8) -> Self {
        match code {
            1 => MouseButton::Middle,
            2 => MouseButton::Right,
            _ => MouseButton::Left,
        }
    }
}

impl IntoSeq for MouseButton {
    fn into_seq(&self) -> impl WriteSeq {
        self.code()
    }
}

impl From<u8> for MouseButton {
    fn from(value: u8) -> Self {
        Self::from_code(value)
    }
}

impl From<EscapeSequenceParam> for MouseButton {
    fn from(param: EscapeSequenceParam) -> Self {
        Self::from_code(param.first())
    }
}

impl From<&EscapeSequenceParam> for MouseButton {
    fn from(param: &EscapeSequenceParam) -> Self {
        Self::from(param.clone())
    }
}

impl From<EscapeSequenceParam> for KeyModifiers {
    fn from(param: EscapeSequenceParam) -> Self {
        let code = param.first();
        let mut modifiers = KeyModifiers::NONE;

        // Extract modifier bits from SGR button code
        if (code & 4) != 0 {
            modifiers |= KeyModifiers::SHIFT;
        }
        if (code & 8) != 0 {
            modifiers |= KeyModifiers::ALT;
        }
        if (code & 16) != 0 {
            modifiers |= KeyModifiers::CONTROL;
        }

        modifiers
    }
}

impl From<&EscapeSequenceParam> for KeyModifiers {
    fn from(param: &EscapeSequenceParam) -> Self {
        Self::from(param.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use vtenc::Encode;

    #[test]
    fn test_encode_mouse_event_down() {
        let mut event = MouseEvent::new(
            MouseEventKind::Down(MouseButton::Left),
            KeyModifiers::NONE,
            Coordinates::new(0, 0),
        );
        let mut buf = [0u8; 64];
        let len = event.encode(&mut &mut buf[..]).unwrap();
        assert_eq!(&buf[..len], b"\x1b[<0;1;1M");
    }

    #[test]
    fn test_encode_mouse_event_up() {
        let mut event = MouseEvent::new(
            MouseEventKind::Up(MouseButton::Left),
            KeyModifiers::NONE,
            Coordinates::new(0, 0),
        );
        let mut buf = [0u8; 64];
        let len = event.encode(&mut &mut buf[..]).unwrap();
        assert_eq!(&buf[..len], b"\x1b[<0;1;1m");
    }

    #[test]
    fn test_encode_mouse_event_scroll() {
        let mut event = MouseEvent::new(
            MouseEventKind::ScrollUp,
            KeyModifiers::NONE,
            Coordinates::new(0, 0),
        );
        let mut buf = [0u8; 64];
        let len = event.encode(&mut &mut buf[..]).unwrap();
        assert_eq!(&buf[..len], b"\x1b[<64;1;1M");
    }
}
