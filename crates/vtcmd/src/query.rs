//! Terminal query commands.

use vtansi::encode::{Encode, EncodeError, write_str_into};
use vtansi::{csi, dcs, osc, write_const_str_into};

/// Request cursor position report (CPR).
pub struct RequestCursorPosition;

impl Encode for RequestCursorPosition {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        write_const_str_into!(buf, csi!("6n"))
    }
}

/// Request terminal size (DECSLPP).
pub struct RequestTerminalSize;

impl Encode for RequestTerminalSize {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        write_const_str_into!(buf, csi!("18t"))
    }
}

/// Request primary device attributes (DA1).
pub struct RequestDeviceAttributes;

impl Encode for RequestDeviceAttributes {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        write_const_str_into!(buf, csi!("c"))
    }
}

/// Request secondary device attributes (DA2).
pub struct RequestSecondaryDeviceAttributes;

impl Encode for RequestSecondaryDeviceAttributes {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        write_const_str_into!(buf, csi!(">c"))
    }
}

/// Request tertiary device attributes (DA3).
pub struct RequestTertiaryDeviceAttributes;

impl Encode for RequestTertiaryDeviceAttributes {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        write_const_str_into!(buf, csi!("=c"))
    }
}

/// Feature report identifier for DECRQM (Request Mode).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum Feature {
    /// Alternate screen buffer (1049).
    AltScreen = 1049,
    /// Bracketed paste mode (2004).
    BracketedPaste = 2004,
    /// X10 mouse mode (9).
    MouseX10 = 9,
    /// Normal mouse tracking (1000).
    MouseNormal = 1000,
    /// Button-event mouse tracking (1002).
    MouseButtonEvent = 1002,
    /// Any-event mouse tracking (1003).
    MouseAnyEvent = 1003,
    /// SGR extended mouse mode (1006).
    MouseSGR = 1006,
    /// RXVT extended mouse mode (1015).
    MouseRXVT = 1015,
    /// Focus reporting (1004).
    FocusReporting = 1004,
    /// Application cursor keys (1).
    ApplicationCursorKeys = 1,
    /// Auto wrap mode (7).
    AutoWrap = 7,
    /// Cursor blinking (12).
    CursorBlinking = 12,
    /// Cursor visible (25).
    CursorVisible = 25,
    /// Linefeed/newline mode (20).
    LinefeedNewline = 20,
    /// Insert mode (4).
    InsertMode = 4,
}

impl Feature {
    /// Check if this is an ANSI mode (vs DEC private mode).
    #[must_use]
    pub const fn is_ansi(self) -> bool {
        matches!(self, Self::InsertMode | Self::LinefeedNewline)
    }
}

/// Request feature status using DECRQM (Request Mode).
pub struct RequestFeature(pub Feature);

impl Encode for RequestFeature {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        if self.0.is_ansi() {
            write_str_into(buf, &csi!("{}$p", self.0 as u16))
        } else {
            write_str_into(buf, &csi!("?{}$p", self.0 as u16))
        }
    }
}

/// Request default foreground color.
pub struct RequestDefaultForeground;

impl Encode for RequestDefaultForeground {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        write_const_str_into!(buf, osc!("10;?"))
    }
}

/// Request default background color.
pub struct RequestDefaultBackground;

impl Encode for RequestDefaultBackground {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        write_const_str_into!(buf, osc!("11;?"))
    }
}

/// Request cursor shape using DECRQSS.
pub struct RequestCursorShape;

impl Encode for RequestCursorShape {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        write_const_str_into!(buf, dcs!("$q q"))
    }
}

/// Request text attributes (SGR) using DECRQSS.
pub struct RequestTextAttributes;

impl Encode for RequestTextAttributes {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        write_const_str_into!(buf, dcs!("$qm"))
    }
}

/// Request scrolling region (top/bottom) using DECRQSS.
pub struct RequestScrollingRegion;

impl Encode for RequestScrollingRegion {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        write_const_str_into!(buf, dcs!("$qr"))
    }
}

/// Request scrolling region (left/right) using DECRQSS.
pub struct RequestScrollingColumns;

impl Encode for RequestScrollingColumns {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        write_const_str_into!(buf, dcs!("$qs"))
    }
}
