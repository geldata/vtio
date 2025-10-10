//! Cursor movement and control commands.

use vtansi::encode::{Encode, EncodeError};
use vtansi::{write_csi, write_esc};

/// Move cursor to the specified position (1-indexed).
pub struct MoveTo {
    pub row: u16,
    pub col: u16,
}

impl Encode for MoveTo {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        write_csi!(buf, "{};{}H", self.row, self.col)
    }
}

/// Move cursor up by the specified number of lines.
pub struct MoveUp(pub u16);

impl Encode for MoveUp {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        write_csi!(buf, "{}A", self.0)
    }
}

/// Move cursor down by the specified number of lines.
pub struct MoveDown(pub u16);

impl Encode for MoveDown {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        write_csi!(buf, "{}B", self.0)
    }
}

/// Move cursor left by the specified number of columns.
pub struct MoveLeft(pub u16);

impl Encode for MoveLeft {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        write_csi!(buf, "{}D", self.0)
    }
}

/// Move cursor right by the specified number of columns.
pub struct MoveRight(pub u16);

impl Encode for MoveRight {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        write_csi!(buf, "{}C", self.0)
    }
}

/// Move cursor to the beginning of the line N lines down.
pub struct MoveToNextLine(pub u16);

impl Encode for MoveToNextLine {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        write_csi!(buf, "{}E", self.0)
    }
}

/// Move cursor to the beginning of the line N lines up.
pub struct MoveToPreviousLine(pub u16);

impl Encode for MoveToPreviousLine {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        write_csi!(buf, "{}F", self.0)
    }
}

/// Move cursor to the specified column on the current line.
pub struct MoveToColumn(pub u16);

impl Encode for MoveToColumn {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        write_csi!(buf, "{}G", self.0)
    }
}

/// Hide the cursor.
pub struct HideCursor;

impl Encode for HideCursor {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        write_csi!(buf, "?25l")
    }
}

/// Show the cursor.
pub struct ShowCursor;

impl Encode for ShowCursor {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        write_csi!(buf, "?25h")
    }
}

/// Enable cursor blinking.
pub struct EnableCursorBlinking;

impl Encode for EnableCursorBlinking {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        write_csi!(buf, "?12h")
    }
}

/// Disable cursor blinking.
pub struct DisableCursorBlinking;

impl Encode for DisableCursorBlinking {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        write_csi!(buf, "?12l")
    }
}

/// Cursor shape variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorShape {
    /// Default cursor shape (usually blinking block).
    Default,
    /// Blinking block cursor.
    BlinkingBlock,
    /// Steady (non-blinking) block cursor.
    SteadyBlock,
    /// Blinking underline cursor.
    BlinkingUnderline,
    /// Steady underline cursor.
    SteadyUnderline,
    /// Blinking bar (vertical line) cursor.
    BlinkingBar,
    /// Steady bar cursor.
    SteadyBar,
}

/// Set cursor shape using DECSCUSR.
pub struct SetCursorShape(pub CursorShape);

impl Encode for SetCursorShape {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let code = match self.0 {
            CursorShape::Default => 0,
            CursorShape::BlinkingBlock => 1,
            CursorShape::SteadyBlock => 2,
            CursorShape::BlinkingUnderline => 3,
            CursorShape::SteadyUnderline => 4,
            CursorShape::BlinkingBar => 5,
            CursorShape::SteadyBar => 6,
        };
        write_csi!(buf, "{} q", code)
    }
}

/// Save cursor position (DECSC).
pub struct SaveCursorPosition;

impl Encode for SaveCursorPosition {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        // DECSC: ESC 7 (not a CSI sequence)
        write_esc!(buf, "7")
    }
}

/// Restore cursor position (DECRC).
pub struct RestoreCursorPosition;

impl Encode for RestoreCursorPosition {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        // DECRC: ESC 8 (not a CSI sequence)
        write_esc!(buf, "8")
    }
}
