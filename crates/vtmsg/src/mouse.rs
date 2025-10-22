//! Mouse mode control commands.
//!
//! See <https://terminalguide.namepad.de/mouse/> for details.

use crate::terminal_mode;
use vtenc::{ConstEncodedLen, Encode, EncodeError, const_composite, write_csi};

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
    MouseX10Mode,
    "?9"
);

terminal_mode!(
    /// Mouse down+up tracking.
    ///
    /// Send mouse button press and release. Also send scroll wheel
    /// events.
    ///
    /// See <https://terminalguide.namepad.de/mode/p1000/> for
    /// terminal support specifics.
    MouseDownUpTrackingMode,
    "?1000"
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
    MouseHighlightMode,
    "?1001"
);

terminal_mode!(
    /// Mouse click and dragging tracking.
    ///
    /// Send mouse button press and release. Send mouse move events
    /// while a button is pressed. Also send scroll wheel events.
    ///
    /// See <https://terminalguide.namepad.de/mode/p1002/> for
    /// terminal support specifics.
    MouseClickAndDragTrackingMode,
    "?1002"
);

terminal_mode!(
    /// Mouse tracking with movement.
    ///
    /// Send mouse button press and release. Always send mouse move
    /// events. Also send scroll wheel events.
    ///
    /// See <https://terminalguide.namepad.de/mode/p1003/> for
    /// terminal support specifics.
    MouseAnyEventTrackingMode,
    "?1003"
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
    MouseReportMultibyteMode,
    "?1005"
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
    MouseReportSgrMode,
    "?1006"
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
    MouseReportRxvtMode,
    "?1015"
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
    MouseReportFocusMode,
    "?1004"
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
    MouseWheelToCursorKeysMode,
    "?1007"
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
        Self {
            attr_xor,
            char_xor,
        }
    }
}

impl ConstEncodedLen for LinuxMousePointerStyle {
    // CSI (2) + max attr_xor (3) + ";" (1) + max char_xor (3) + "m" (1)
    // = 10
    const ENCODED_LEN: usize = 10;
}

impl Encode for LinuxMousePointerStyle {
    #[inline]
    fn encode<W: std::io::Write>(
        &mut self,
        buf: &mut W,
    ) -> Result<usize, EncodeError> {
        write_csi!(buf; self.attr_xor, ";", self.char_xor, "m")
    }
}
