//! Window control commands.

use vtansi::{Encode, EncodeError, Write};
use vtansi::{write_csi, write_osc};

/// Set terminal window title.
pub struct SetTitle<'a>(pub &'a str);

impl Encode for SetTitle<'_> {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        // OSC 0 ; title ST
        write_osc!(buf, "0;{}", self.0)
    }
}

impl Write for SetTitle<'_> {
    fn write<W: std::io::Write>(&mut self, writer: &mut W) -> std::io::Result<usize> {
        write_osc!(writer, "0;{}", self.0).map_err(Into::into)
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
        write_csi!(buf, "8;{};{}t", self.rows, self.cols)
    }
}
