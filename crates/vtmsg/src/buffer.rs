//! Buffer control/information messages.

use vtenc::{ConstEncode, Encode, EncodeError, csi, dcs, write_csi};

/// Scroll up by the specified number of lines.
pub struct ScrollUp(pub u16);

impl Encode for ScrollUp {
    #[inline]
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        write_csi!(buf; self.0, "S")
    }
}

/// Scroll down by the specified number of lines.
pub struct ScrollDown(pub u16);

impl Encode for ScrollDown {
    #[inline]
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        write_csi!(buf; self.0, "T")
    }
}

/// Enable line wrapping.
pub struct EnableLineWrap;

impl ConstEncode for EnableLineWrap {
    const STR: &'static str = csi!("?7h");
}

/// Disable line wrapping.
pub struct DisableLineWrap;

impl ConstEncode for DisableLineWrap {
    const STR: &'static str = csi!("?7l");
}

/// Enter alternate screen buffer.
pub struct EnterAlternateScreen;

impl ConstEncode for EnterAlternateScreen {
    const STR: &'static str = csi!("?1049h");
}

/// Leave alternate screen buffer.
pub struct LeaveAlternateScreen;

impl ConstEncode for LeaveAlternateScreen {
    const STR: &'static str = csi!("?1049l");
}

/// Request scrolling region (top/bottom) using DECRQSS.
pub struct RequestScrollingRegion;

impl ConstEncode for RequestScrollingRegion {
    const STR: &'static str = dcs!("$qr");
}

/// Request scrolling region (left/right) using DECRQSS.
pub struct RequestScrollingColumns;

impl ConstEncode for RequestScrollingColumns {
    const STR: &'static str = dcs!("$qs");
}
