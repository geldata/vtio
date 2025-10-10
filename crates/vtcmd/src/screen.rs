//! Screen control commands.

use vtansi::encode::{Encode, EncodeError};
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

impl Encode for EnableLineWrap {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        write_csi!(buf, "?7h")
    }
}

/// Disable line wrapping.
pub struct DisableLineWrap;

impl Encode for DisableLineWrap {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        write_csi!(buf, "?7l")
    }
}

/// Enter alternate screen buffer.
pub struct EnterAlternateScreen;

impl Encode for EnterAlternateScreen {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        write_csi!(buf, "?1049h")
    }
}

/// Leave alternate screen buffer.
pub struct LeaveAlternateScreen;

impl Encode for LeaveAlternateScreen {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        write_csi!(buf, "?1049l")
    }
}
