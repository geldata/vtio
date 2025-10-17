//! Screen control commands.

use vtenc::csi;
use vtenc::write_csi;
use vtenc::{ConstEncode, Encode, EncodeError};

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
