//! Screen control commands.

use vtansi::csi;
use vtansi::{Encode, EncodeError, ConstEncode};
use vtansi::write_csi;

/// Scroll up by the specified number of lines.
pub struct ScrollUp(pub u16);

impl Encode for ScrollUp {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        write_csi!(buf, "{}S", self.0)
    }
}

/// Scroll down by the specified number of lines.
pub struct ScrollDown(pub u16);

impl Encode for ScrollDown {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        write_csi!(buf, "{}T", self.0)
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
