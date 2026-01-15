//! Mouse mode control commands.
//!
//! See <https://terminalguide.namepad.de/mouse/> for details.

use vtansi::{ParseError, ansi_composite};

use crate::{event::keyboard::KeyModifiers, terminal_mode};

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

ansi_composite! {
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

ansi_composite! {
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
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, vtansi::derive::AnsiOutput,
)]
#[vtansi(csi, finalbyte = 'm')]
pub struct LinuxMousePointerStyle {
    /// XOR mask for attribute manipulation.
    pub attr_xor: u8,
    /// XOR mask for character glyph manipulation.
    pub char_xor: u8,
}

/// Terminal coordinates (column and row).
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    vtansi::derive::ToAnsi,
    vtansi::derive::FromAnsi,
)]
#[vtansi(format = "vector")]
pub struct Coordinates {
    /// Column position (1-based).
    pub column: u16,
    /// Row position (1-based).
    pub row: u16,
}

impl Coordinates {
    /// Create new coordinates.
    #[must_use]
    pub const fn new(column: u16, row: u16) -> Self {
        Self { column, row }
    }
}

#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, vtansi::derive::ToAnsi)]
#[vtansi(transparent)]
pub struct MouseKeyModifiers(pub(crate) KeyModifiers);

/// Extract modifier keys from a mouse button code.
///
/// The button code contains modifier bits:
/// - bit 2 (4): Shift
/// - bit 3 (8): Alt
/// - bit 4 (16): Ctrl
#[must_use]
pub const fn modifiers_from_button_code(btn_code: u16) -> KeyModifiers {
    let mut bits = KeyModifiers::NONE.bits();
    if (btn_code & 4) != 0 {
        bits |= KeyModifiers::SHIFT.bits();
    }
    if (btn_code & 8) != 0 {
        bits |= KeyModifiers::ALT.bits();
    }
    if (btn_code & 16) != 0 {
        bits |= KeyModifiers::CONTROL.bits();
    }
    KeyModifiers::from_bits_retain(bits)
}

impl ::std::ops::Deref for MouseKeyModifiers {
    type Target = KeyModifiers;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<KeyModifiers> for MouseKeyModifiers {
    fn from(modifiers: KeyModifiers) -> Self {
        Self(modifiers)
    }
}

impl vtansi::AnsiMuxEncode for MouseKeyModifiers {
    type BaseType = u16;

    fn mux_encode(
        &self,
        base: Option<&Self::BaseType>,
    ) -> Result<Self::BaseType, vtansi::EncodeError> {
        Ok((if let Some(base) = base { *base } else { 0 })
            | u16::from(self.bits()))
    }
}

impl<'a> vtansi::TryFromAnsi<'a> for MouseKeyModifiers {
    fn try_from_ansi(bytes: &'a [u8]) -> Result<Self, vtansi::ParseError> {
        let code = <u16 as vtansi::TryFromAnsi>::try_from_ansi(bytes)?;
        Ok(Self(modifiers_from_button_code(code)))
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
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, vtansi::derive::AnsiInput,
)]
#[vtansi(csi, private = '<', finalbyte = 'M' | 'm')]
pub struct MouseEvent {
    /// The kind of mouse event that was caused.
    pub kind: MouseEventKind,
    /// The key modifiers active when the event occurred.
    /// Encoded in parameter 0 (button code).
    #[vtansi(muxwith = "kind")]
    pub modifiers: MouseKeyModifiers,
    /// The coordinates where the event occurred.
    #[vtansi(flatten)]
    pub coords: Coordinates,
}

impl vtansi::AnsiFinalByte for MouseEvent {
    fn ansi_final_byte(&self) -> u8 {
        self.kind.final_byte()
    }
}

impl MouseEvent {
    /// Get the column where the event occurred (0-based).
    #[must_use]
    pub const fn column(&self) -> u16 {
        self.coords.column.saturating_sub(1)
    }

    /// Get the row where the event occurred (0-based).
    #[must_use]
    pub const fn row(&self) -> u16 {
        self.coords.row.saturating_sub(1)
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
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
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
    /// Get the final byte for SGR encoding ('M' or 'm').
    const fn final_byte(self) -> u8 {
        match self {
            MouseEventKind::Up(_) => b'm',
            _ => b'M',
        }
    }

    /// Parse a mouse event kind from a button code.
    ///
    /// The `is_release` parameter indicates whether this is a button release
    /// event (used in SGR format where release is indicated by final byte 'm').
    /// In the default format, release is indicated by button code 3.
    ///
    /// Returns `None` if the button code is invalid.
    /// Converts a button code to a `MouseEventKind`.
    ///
    /// # Errors
    ///
    /// Returns an error if the button code is invalid or unrecognized.
    pub fn from_button_code(
        btn_code: u16,
        is_release: bool,
    ) -> Result<Self, ParseError> {
        // Remove modifier bits (4, 8, 16)
        let base_code = btn_code & !0x1C;
        let is_drag = (btn_code & 32) != 0;

        let event_kind = if base_code >= 64 {
            // Scroll events (bit 6 set)
            match base_code & 0x03 {
                0 => MouseEventKind::ScrollUp,
                1 => MouseEventKind::ScrollDown,
                2 => MouseEventKind::ScrollLeft,
                3 => MouseEventKind::ScrollRight,
                code => {
                    return Err(vtansi::ParseError::InvalidValue(format!(
                        "unrecognized mouse button code: {code}"
                    )));
                }
            }
        } else if (base_code & !32) == 3 {
            // Button code 3: "moved" if drag bit set, otherwise release
            if is_drag {
                MouseEventKind::Moved
            } else {
                // In default format, we don't know which button was released
                MouseEventKind::Up(MouseButton::Left)
            }
        } else {
            let button = match base_code & 0x03 {
                0 => MouseButton::Left,
                1 => MouseButton::Middle,
                2 => MouseButton::Right,
                code => {
                    return Err(vtansi::ParseError::InvalidValue(format!(
                        "unrecognized mouse button code: {code}"
                    )));
                }
            };
            if is_release {
                MouseEventKind::Up(button)
            } else if is_drag {
                MouseEventKind::Drag(button)
            } else {
                MouseEventKind::Down(button)
            }
        };

        Ok(event_kind)
    }
}

/// Convert the base SGR button code (without modifiers) into u16.
impl From<&MouseEventKind> for u16 {
    #[inline]
    fn from(value: &MouseEventKind) -> Self {
        match value {
            MouseEventKind::Down(button) | MouseEventKind::Up(button) => {
                (*button).into()
            }
            MouseEventKind::Drag(button) => 32u16 + u16::from(*button), // Add drag bit
            MouseEventKind::Moved => 3 + 32, // Mouse move without button
            MouseEventKind::ScrollUp => 1 << 6,
            MouseEventKind::ScrollDown => (1 << 6) | 1,
            MouseEventKind::ScrollLeft => (1 << 6) | 2,
            MouseEventKind::ScrollRight => (1 << 6) | 3,
        }
    }
}

impl From<MouseEventKind> for u16 {
    #[inline]
    fn from(value: MouseEventKind) -> Self {
        u16::from(&value)
    }
}

impl vtansi::AnsiMuxEncode for MouseEventKind {
    type BaseType = u16;

    #[inline]
    fn mux_encode(
        &self,
        base: Option<&Self::BaseType>,
    ) -> Result<Self::BaseType, vtansi::EncodeError> {
        let other = if let Some(base) = base { *base } else { 0 };
        Ok(Self::BaseType::from(self) | other)
    }
}

impl vtansi::AnsiEncode for MouseEventKind {
    const ENCODED_LEN: Option<usize> = <u16 as vtansi::AnsiEncode>::ENCODED_LEN;

    #[inline]
    fn encode_ansi_into<W: std::io::Write + ?Sized>(
        &self,
        sink: &mut W,
    ) -> Result<usize, vtansi::EncodeError> {
        <_ as vtansi::AnsiEncode>::encode_ansi_into(&u16::from(self), sink)
    }
}

impl<'a> vtansi::TryFromAnsi<'a> for MouseEventKind {
    fn try_from_ansi(bytes: &'a [u8]) -> Result<Self, vtansi::ParseError> {
        let code = <u16 as vtansi::TryFromAnsi>::try_from_ansi(bytes)?;
        // SGR format: release is indicated by final byte 'm', not button code
        // So we pass is_release=false here; the final byte handling is done elsewhere
        Self::from_button_code(code, false)
    }
}

/// Represents a mouse button.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    vtansi::derive::ToAnsi,
    vtansi::derive::FromAnsi,
)]
#[repr(u16)]
pub enum MouseButton {
    /// Left mouse button.
    Left = 0,
    /// Right mouse button.
    Right = 1,
    /// Middle mouse button.
    Middle = 2,
    /// Nth mouse button.
    #[num_enum(catch_all)]
    Nth(u16),
}

/// Track Mouse.
///
/// This sequence is used with mouse highlight mode to communicate the
/// selection start and allowed rows.
///
/// If cmd is 0 then the highlighting is aborted and the terminal uses non
/// highlighting mouse handling as in mouse down+up tracking.
///
/// If cmd is non-zero then start-column and start-row specify the selection
/// start and first-row specifies the first allowed row for the selection and
/// last-row specifies the first row that the selection may not enter into.
///
/// See <https://terminalguide.namepad.de/seq/csi_ct_5param/> for terminal
/// support specifics.
#[derive(
    Debug,
    PartialOrd,
    PartialEq,
    Eq,
    Clone,
    Copy,
    Hash,
    vtansi::derive::AnsiOutput,
)]
#[vtansi(csi, finalbyte = 'T', disambiguate)]
pub struct TrackMouse {
    cmd: u8,
    start_column: u16,
    start_row: u16,
    first_row: u16,
    last_row: u16,
}

impl TrackMouse {
    /// Create a new `TrackMouse` sequence.
    ///
    /// # Arguments
    ///
    /// * `cmd` - Command byte (0 to abort highlighting, non-zero to start)
    /// * `start_column` - Starting column for selection
    /// * `start_row` - Starting row for selection
    /// * `first_row` - First allowed row for selection
    /// * `last_row` - First row that selection may not enter
    #[must_use]
    pub const fn new(
        cmd: u8,
        start_column: u16,
        start_row: u16,
        first_row: u16,
        last_row: u16,
    ) -> Self {
        Self {
            cmd,
            start_column,
            start_row,
            first_row,
            last_row,
        }
    }

    /// Get the command byte.
    #[must_use]
    pub const fn cmd(&self) -> u8 {
        self.cmd
    }

    /// Get the start column.
    #[must_use]
    pub const fn start_column(&self) -> u16 {
        self.start_column
    }

    /// Get the start row.
    #[must_use]
    pub const fn start_row(&self) -> u16 {
        self.start_row
    }

    /// Get the first allowed row.
    #[must_use]
    pub const fn first_row(&self) -> u16 {
        self.first_row
    }

    /// Get the last row (first row selection may not enter).
    #[must_use]
    pub const fn last_row(&self) -> u16 {
        self.last_row
    }
}

/// Parse default mouse reporting format bytes into a `MouseEvent`.
///
/// The default format uses 3 raw bytes after `CSI M`:
/// - `btn`: 32 + `button_code` (with modifier bits)
/// - `col`: 32 + column (1-based)
/// - `row`: 32 + row (1-based)
///
/// # Errors
///
/// Returns an error if:
/// - The byte slice has fewer than 3 bytes
/// - The button code is invalid
pub fn parse_default_mouse_bytes(
    bytes: &[u8],
) -> Result<MouseEvent, ParseError> {
    if bytes.len() < 3 {
        return Err(vtansi::ParseError::InvalidValue(
            "invalid mouse event byte sequence".to_string(),
        ));
    }
    // Default mouse protocol encodes each value as a single byte with offset 32

    let btn_code = u16::from(bytes[0].saturating_sub(32));
    let column = u16::from(bytes[1].saturating_sub(32));
    let row = u16::from(bytes[2].saturating_sub(32));

    // Default format doesn't have separate release indication, it uses button code 3
    let kind = MouseEventKind::from_button_code(btn_code, false)?;
    let modifiers = MouseKeyModifiers(modifiers_from_button_code(btn_code));

    Ok(MouseEvent {
        kind,
        modifiers,
        coords: Coordinates { column, row },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use vtansi::AnsiEncode;

    #[test]
    fn test_encode_mouse_event_down() {
        let event = MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            modifiers: MouseKeyModifiers(KeyModifiers::NONE),
            coords: Coordinates::new(1, 1),
        };
        let result = event.encode_ansi().unwrap();
        assert_eq!(result, b"\x1b[<0;1;1M");
    }

    #[test]
    fn test_encode_mouse_event_up() {
        let event = MouseEvent {
            kind: MouseEventKind::Up(MouseButton::Left),
            modifiers: MouseKeyModifiers(KeyModifiers::NONE),
            coords: Coordinates::new(1, 1),
        };
        let mut buf = [0u8; 64];
        let len = event.encode_ansi_into(&mut &mut buf[..]).unwrap();
        assert_eq!(&buf[..len], b"\x1b[<0;1;1m");
    }

    #[test]
    fn test_encode_mouse_event_scroll() {
        let event = MouseEvent {
            kind: MouseEventKind::ScrollUp,
            modifiers: MouseKeyModifiers(KeyModifiers::NONE),
            coords: Coordinates::new(1, 1),
        };
        let mut buf = [0u8; 64];
        let len = event.encode_ansi_into(&mut &mut buf[..]).unwrap();
        assert_eq!(&buf[..len], b"\x1b[<64;1;1M");
    }

    #[test]
    fn test_encode_enable_mouse_capture() {
        let mut buf = Vec::new();
        EnableMouseCapture.encode_ansi_into(&mut buf).unwrap();
        // Should encode multiple mode enable sequences:
        // ESC[?1000h (down/up tracking)
        // ESC[?1002h (click and drag)
        // ESC[?1003h (any event)
        // ESC[?1015h (urxvt format)
        // ESC[?1006h (SGR format)
        let expected =
            b"\x1b[?1000h\x1b[?1002h\x1b[?1003h\x1b[?1015h\x1b[?1006h";
        assert_eq!(
            buf,
            expected,
            "EnableMouseCapture should encode to: {:?}, got: {:?}",
            String::from_utf8_lossy(expected),
            String::from_utf8_lossy(&buf)
        );
    }

    #[test]
    fn test_encode_disable_mouse_capture() {
        let mut buf = Vec::new();
        DisableMouseCapture.encode_ansi_into(&mut buf).unwrap();
        // Should encode mode disable sequences in reverse order:
        // ESC[?1006l (SGR format)
        // ESC[?1015l (urxvt format)
        // ESC[?1003l (any event)
        // ESC[?1002l (click and drag)
        // ESC[?1000l (down/up tracking)
        let expected =
            b"\x1b[?1006l\x1b[?1015l\x1b[?1003l\x1b[?1002l\x1b[?1000l";
        assert_eq!(
            buf,
            expected,
            "DisableMouseCapture should encode to: {:?}, got: {:?}",
            String::from_utf8_lossy(expected),
            String::from_utf8_lossy(&buf)
        );
    }

    #[test]
    fn test_parse_default_mouse_left_click() {
        // Left button click at column 10, row 5
        // btn = 32 + 0 (left button) = 32 = 0x20
        // col = 32 + 10 = 42 = 0x2A
        // row = 32 + 5 = 37 = 0x25
        let event = parse_default_mouse_bytes(&[0x20, 0x2A, 0x25]).unwrap();
        assert!(matches!(
            event,
            MouseEvent {
                kind: MouseEventKind::Down(MouseButton::Left),
                ..
            }
        ));
        assert_eq!(event.column(), 9); // 0-based
        assert_eq!(event.row(), 4); // 0-based
    }

    #[test]
    fn test_parse_default_mouse_right_click() {
        // Right button click at column 20, row 15
        // btn = 32 + 2 (right button) = 34 = 0x22
        // col = 32 + 20 = 52 = 0x34
        // row = 32 + 15 = 47 = 0x2F
        let event = parse_default_mouse_bytes(&[0x22, 0x34, 0x2F]).unwrap();
        assert!(matches!(
            event,
            MouseEvent {
                kind: MouseEventKind::Down(MouseButton::Right),
                ..
            }
        ));
        assert_eq!(event.column(), 19);
        assert_eq!(event.row(), 14);
    }

    #[test]
    fn test_parse_default_mouse_middle_click() {
        // Middle button click
        // btn = 32 + 1 (middle button) = 33 = 0x21
        let event = parse_default_mouse_bytes(&[0x21, 0x34, 0x2F]).unwrap();
        assert!(matches!(
            event,
            MouseEvent {
                kind: MouseEventKind::Down(MouseButton::Middle),
                ..
            }
        ));
    }

    #[test]
    fn test_parse_default_mouse_release() {
        // Button release (code 3)
        // btn = 32 + 3 = 35 = 0x23
        let event = parse_default_mouse_bytes(&[0x23, 0x34, 0x2F]).unwrap();
        assert!(matches!(
            event,
            MouseEvent {
                kind: MouseEventKind::Up(_),
                ..
            }
        ));
    }

    #[test]
    fn test_parse_default_mouse_scroll_up() {
        // Scroll up (code 64)
        // btn = 32 + 64 = 96 = 0x60
        let event = parse_default_mouse_bytes(&[0x60, 0x34, 0x2F]).unwrap();
        assert!(matches!(
            event,
            MouseEvent {
                kind: MouseEventKind::ScrollUp,
                ..
            }
        ));
    }

    #[test]
    fn test_parse_default_mouse_scroll_down() {
        // Scroll down (code 65)
        // btn = 32 + 65 = 97 = 0x61
        let event = parse_default_mouse_bytes(&[0x61, 0x34, 0x2F]).unwrap();
        assert!(matches!(
            event,
            MouseEvent {
                kind: MouseEventKind::ScrollDown,
                ..
            }
        ));
    }

    #[test]
    fn test_parse_default_mouse_scroll_left() {
        // Scroll left (code 66)
        // btn = 32 + 66 = 98 = 0x62
        let event = parse_default_mouse_bytes(&[0x62, 0x34, 0x2F]).unwrap();
        assert!(matches!(
            event,
            MouseEvent {
                kind: MouseEventKind::ScrollLeft,
                ..
            }
        ));
    }

    #[test]
    fn test_parse_default_mouse_scroll_right() {
        // Scroll right (code 67)
        // btn = 32 + 67 = 99 = 0x63
        let event = parse_default_mouse_bytes(&[0x63, 0x34, 0x2F]).unwrap();
        assert!(matches!(
            event,
            MouseEvent {
                kind: MouseEventKind::ScrollRight,
                ..
            }
        ));
    }

    #[test]
    fn test_parse_default_mouse_drag() {
        // Drag with left button (code 0 + 32 drag bit = 32)
        // btn = 32 + 32 = 64 = 0x40
        let event = parse_default_mouse_bytes(&[0x40, 0x34, 0x2F]).unwrap();
        assert!(matches!(
            event,
            MouseEvent {
                kind: MouseEventKind::Drag(MouseButton::Left),
                ..
            }
        ));
    }

    #[test]
    fn test_parse_default_mouse_moved() {
        // Moved (code 3 + 32 drag bit = 35)
        // btn = 32 + 35 = 67 = 0x43
        let event = parse_default_mouse_bytes(&[0x43, 0x34, 0x2F]).unwrap();
        assert!(matches!(
            event,
            MouseEvent {
                kind: MouseEventKind::Moved,
                ..
            }
        ));
    }

    #[test]
    fn test_parse_default_mouse_with_ctrl() {
        // Ctrl+click (ctrl bit = 16)
        // btn = 32 + 0 (left) + 16 (ctrl) = 48 = 0x30
        let event = parse_default_mouse_bytes(&[0x30, 0x2A, 0x25]).unwrap();
        assert!(matches!(
            event,
            MouseEvent {
                kind: MouseEventKind::Down(MouseButton::Left),
                ..
            }
        ));
        assert!(event.modifiers.contains(KeyModifiers::CONTROL));
    }

    #[test]
    fn test_parse_default_mouse_with_shift() {
        // Shift+click (shift bit = 4)
        // btn = 32 + 0 (left) + 4 (shift) = 36 = 0x24
        let event = parse_default_mouse_bytes(&[0x24, 0x2A, 0x25]).unwrap();
        assert!(matches!(
            event,
            MouseEvent {
                kind: MouseEventKind::Down(MouseButton::Left),
                ..
            }
        ));
        assert!(event.modifiers.contains(KeyModifiers::SHIFT));
    }

    #[test]
    fn test_parse_default_mouse_with_alt() {
        // Alt+click (alt bit = 8)
        // btn = 32 + 0 (left) + 8 (alt) = 40 = 0x28
        let event = parse_default_mouse_bytes(&[0x28, 0x2A, 0x25]).unwrap();
        assert!(matches!(
            event,
            MouseEvent {
                kind: MouseEventKind::Down(MouseButton::Left),
                ..
            }
        ));
        assert!(event.modifiers.contains(KeyModifiers::ALT));
    }

    #[test]
    fn test_parse_default_mouse_insufficient_bytes() {
        assert!(parse_default_mouse_bytes(&[]).is_err());
        assert!(parse_default_mouse_bytes(&[0x20]).is_err());
        assert!(parse_default_mouse_bytes(&[0x20, 0x2A]).is_err());
    }
}
