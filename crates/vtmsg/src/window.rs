//! Window control commands.

use vtenc::{ConstEncode, Encode, EncodeError, csi, write_csi, write_osc};

/// Set terminal window title.
pub struct SetTitle<'a>(pub &'a str);

impl Encode for SetTitle<'_> {
    #[inline]
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        // OSC 0 ; title ST
        write_osc!(buf; "0;", self.0)
    }
}

/// Resize terminal window to specified size.
pub struct SetSize {
    pub rows: u16,
    pub cols: u16,
}

impl Encode for SetSize {
    #[inline]
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        write_csi!(buf; "8;", self.rows, ";", self.cols, "t")
    }
}

/// Request terminal size (DECSLPP).
pub struct RequestTerminalSize;

impl ConstEncode for RequestTerminalSize {
    const STR: &'static str = csi!("18t");
}
