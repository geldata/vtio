//! Window control commands.

use vtansi::encode::{Encode, EncodeError, write_str_into};
use vtansi::{csi, osc};

/// Set terminal window title.
pub struct SetTitle<'a>(pub &'a str);

impl Encode for SetTitle<'_> {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        // OSC 0 ; title ST
        write_str_into(buf, &osc!("0;{}", self.0))
    }
}

/// Resize terminal window to specified size.
pub struct SetSize {
    pub rows: u16,
    pub cols: u16,
}

impl Encode for SetSize {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        write_str_into(buf, &csi!("8;{};{}t", self.rows, self.cols))
    }
}
