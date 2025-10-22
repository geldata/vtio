//! Cursor movement and control commands.

use bitflags::bitflags;
use crate::terminal_mode;
use vtenc::{ConstEncode, ConstEncodedLen, Encode, EncodeError, csi, dcs, esc, write_csi};

terminal_mode!(
    /// Cursor Origin Mode (`DECOM`).
    ///
    /// If set, the origin of the coordinate system is relative to the
    /// current scroll region.
    ///
    /// The origin is used by cursor positioning commands such as
    /// [`CursorPosition`], [`CursorVerticalAbsolute`], [`CursorHorizontalAbsolute`], and
    /// cursor position reports.
    ///
    /// When this mode is set, certain sequences will force the cursor to be
    /// in the scrolling region, including carriage return, next line,
    /// cursor next/previous line operations.
    ///
    /// If set, the cursor is moved to the top left of the current scroll
    /// region.
    ///
    /// See <https://terminalguide.namepad.de/mode/p6/> for
    /// terminal support specifics.
    RelativeCursorOriginMode,
    "?6"
);

terminal_mode!(
    /// Cursor Blinking (`ATT610_BLINK`).
    ///
    /// If set, the cursor is blinking.
    ///
    /// This mode interacts with the blinking part of the Select Cursor Style
    /// (`DECSCUSR`) setting. In xterm, this mode is synchronized with the
    /// blinking part of the cursor style. In urxvt, this mode is additive to
    /// the cursor style setting.
    ///
    /// See also [`SetCursorStyle`] for a more widely supported alternative.
    ///
    /// See <https://terminalguide.namepad.de/mode/p12/> for
    /// terminal support specifics.
    CursorBlinking,
    "?12"
);

terminal_mode!(
    /// Cursor Visibility Mode (`DECTCEM`).
    ///
    /// Set visibility of the cursor.
    ///
    /// If set, the cursor is visible. If reset, the cursor is hidden.
    ///
    /// See <https://terminalguide.namepad.de/mode/p25/> for
    /// terminal support specifics.
    CursorVisibility,
    "?25"
);

/// Save cursor (`DECSC`).
///
/// Save cursor position and other state.
///
/// The primary and alternate screen have distinct save state.
///
/// The following state is saved:
///   * the state of [`RelativeCursorOriginMode`]
///     (but not its saved state for restore mode);
///   * the current attributes;
///   * If newly printed characters are protected
///     (like start protected area or select character protection attribute);
///   * the current cursor position, relative to the
///     origin set via cursor origin;
///   * pending wrap state;
///   * GL and GR character sets;
///   * G0, G1, G2, G3 character sets.
///
/// One saved state is kept per screen (main / alternative).
/// If for the current screen state was already saved it is overwritten.
///
/// The state can be restored using [`RestoreCursor`].
///
/// See <https://terminalguide.namepad.de/seq/a_esc_a7/> for
/// terminal support specifics.
pub struct SaveCursor;

impl ConstEncode for SaveCursor {
    const STR: &'static str = esc!("7");
}

/// Restore cursor (`DECRC`).
///
/// Restore cursor position and other state.
///
/// The primary and alternate screen have distinct save state.
///
/// The following state is restored:
///   * the state of [`RelativeCursorOriginMode`]
///     (but not its saved state for restore mode);
///   * the current attributes;
///   * If newly printed characters are protected
///     (like start protected area or select character protection attribute);
///   * the current cursor position, relative to the
///     origin set via cursor origin;
///   * pending wrap state;
///   * GL and GR character sets;
///   * G0, G1, G2, G3 character sets.
///
/// If no [`SaveCursor`] was done previously values are reset to their
/// hard reset values.
///
/// See <https://terminalguide.namepad.de/seq/a_esc_a8/> for
/// terminal support specifics.
pub struct RestoreCursor;

impl ConstEncode for RestoreCursor {
    const STR: &'static str = esc!("8");
}

/// Backspace (`BS`).
///
/// Move the cursor one position to the left.
///
/// If the cursor is on the left-most column, the behavior is implementation
/// dependent (may stay in place or wrap to previous line).
///
/// This unsets the pending wrap state without wrapping.
///
/// See <https://terminalguide.namepad.de/seq/c_bs/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Backspace;

impl ConstEncode for Backspace {
    const STR: &'static str = "\x08";
}

/// Horizontal Tab (`TAB`).
///
/// Move the cursor to the next tab stop.
///
/// If there are no more tab stops, the cursor is moved to the right-most
/// column.
///
/// Tab stops can be set using [`HorizontalTabSet`].
///
/// See <https://terminalguide.namepad.de/seq/c_tab/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct HorizontalTab;

impl ConstEncode for HorizontalTab {
    const STR: &'static str = "\t";
}

/// Line Feed (`LF`).
///
/// Move the cursor to the next line.
///
/// The behavior depends on the Line Feed mode:
///   * If Line Feed mode is not set: move the cursor down one line
///     (like [`Index`])
///   * If Line Feed mode is set: move the cursor down one line and to the
///     left-most column (like [`NextLine`])
///
/// See <https://terminalguide.namepad.de/seq/c_lf/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct LineFeed;

impl ConstEncode for LineFeed {
    const STR: &'static str = "\n";
}

/// Vertical Tab (`VT`).
///
/// Move the cursor down one line (same as [`LineFeed`]).
///
/// See <https://terminalguide.namepad.de/seq/c_vt/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct VerticalTab;

impl ConstEncode for VerticalTab {
    const STR: &'static str = "\x0B";
}

/// Form Feed (`FF`).
///
/// Move the cursor down one line (same as [`LineFeed`]).
///
/// See <https://terminalguide.namepad.de/seq/c_ff/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct FormFeed;

impl ConstEncode for FormFeed {
    const STR: &'static str = "\x0C";
}

/// Carriage Return (`CR`).
///
/// Move the cursor to the left-most column.
///
/// This unsets the pending wrap state without wrapping.
///
/// If left and right margin mode is set and a left margin set and the cursor
/// is on or right of the left margin column it is moved to the left margin. If
/// the cursor is left of the left margin the cursor is moved to the left-most
/// column of the screen.
///
/// If a left margin is set and [`RelativeCursorOriginMode`] is set the cursor
/// will always move to the left margin column.
///
/// See <https://terminalguide.namepad.de/seq/c_cr/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct CarriageReturn;

impl ConstEncode for CarriageReturn {
    const STR: &'static str = "\r";
}

/// Set Cursor Position (`CUP`).
///
/// Move cursor to the position indicated by `row` and `column`.
///
/// If `column` is 0, it is adjusted to 1. If `column` is greater than the
/// right-most column it is adjusted to the right-most column.
///
/// If `row` is 0, it is adjusted to 1. If `row` is greater than the
/// bottom-most row it is adjusted to the bottom-most row.
///
/// `column` = 1 is the left-most column. `row` = 1 is the top-most row.
///
/// This unsets the pending wrap state without wrapping.
///
/// If cursor origin mode is set the cursor row will be moved relative to the
/// top margin row and adjusted to be above or at bottom-most row in the
/// current scroll region.
///
/// If origin mode is set and left and right margin mode is set the cursor
/// will be moved relative to the left margin column and adjusted to be on or
/// left of the right margin column.
///
/// See <https://terminalguide.namepad.de/seq/csi_ch/> for
/// terminal support specifics.
pub struct CursorPosition {
    pub row: u16,
    pub col: u16,
}

impl ConstEncodedLen for CursorPosition {
    // CSI (2) + max u16 digits (5) + ";" (1) + max u16 digits (5) + "H" (1) = 14
    const ENCODED_LEN: usize = 14;
}

impl Encode for CursorPosition {
    #[inline]
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        write_csi!(buf; self.row, ";", self.col, "H")
    }
}

/// Back Index (`DECBI`).
///
/// If the cursor is not on the left-most column of the scroll region this is
/// the same as [`CursorLeft`] with `amount = 1`.
///
/// If the cursor is on the left-most column of the scroll region and on a row
/// that is inside the scroll region, a new blank left-most column of the
/// scroll region is inserted. The previous content of the scroll region are
/// shifted to the right. The right-most column of the scroll region is
/// discarded. If the cell movement splits a multi cell character that
/// character cleared, by replacing it by spaces, keeping its attributes.
///
/// If the cursor is on the left-most column of the scroll region and on a row
/// that is outside the scroll region, nothing is changed.
///
/// The cleared space is colored according to the current SGR state.
///
/// Does not change the cursor position.
///
/// See <https://terminalguide.namepad.de/seq/a_esc_a6/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct BackIndex;

impl ConstEncode for BackIndex {
    const STR: &'static str = esc!("6");
}

/// Forward Index (`DECFI`).
///
/// If the cursor is not on the right-most column of the scroll region this is
/// the same as [`CursorRight`] with `amount = 1`.
///
/// If the cursor is on the right-most column of the scroll region and on a row
/// that is inside the scroll region, the whole left-most column of the scroll
/// region is deleted. The remaining characters are shifted to the left and
/// space from the right margin is filled with spaces. If the cell movement
/// splits a multi cell character that character is cleared, by replacing it by
/// spaces, keeping its attributes.
///
/// If the cursor is on the right-most column of the scroll region and on a row
/// that is outside the scroll region, nothing is changed.
///
/// The cleared space is colored according to the current SGR state.
///
/// Does not change the cursor position.
///
/// See <https://terminalguide.namepad.de/seq/a_esc_a9/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct ForwardIndex;

impl ConstEncode for ForwardIndex {
    const STR: &'static str = esc!("9");
}

/// Index (`IND`).
///
/// Move the cursor to the next line in the scrolling region,
/// possibly scrolling.
///
/// If the cursor is outside of the scrolling region:
///   * move the cursor one line down if it is not on the
///     bottom-most line of the screen.
///
/// If the cursor is inside the scrolling region:
///   * if the cursor is on the bottom-most line of the scrolling region:
///     - invoke [`ScrollUp`] with `amount=1`
///   * if the cursor is not on the bottom-most line of the scrolling region:
///     - move the cursor one line down
///
/// This unsets the pending wrap state without wrapping.
///
/// See <https://terminalguide.namepad.de/seq/a_esc_cd/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Index;

impl ConstEncode for Index {
    const STR: &'static str = esc!("D");
}

/// Next Line (`NEL`).
///
/// Send [`CarriageReturn`] and [`Index`].
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct NextLine;

impl ConstEncode for NextLine {
    const STR: &'static str = esc!("E");
}

/// Horizontal Tab Set (`HTS`).
///
/// Mark current column as tab stop column.
///
/// See <https://terminalguide.namepad.de/seq/a_esc_ch/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct HorizontalTabSet;

impl ConstEncode for HorizontalTabSet {
    const STR: &'static str = esc!("H");
}

/// Reverse Index (`RI`).
///
/// Move the cursor to the previous line in the scrolling region,
/// possibly scrolling.
///
/// If the cursor is outside of the scrolling region:
///   * move the cursor one line up if it is not on the
///     top-most line of the screen.
///
/// If the cursor is inside the scrolling region:
///   * if the cursor is on the top-most line of the scrolling region:
///     - invoke [`ScrollDown`] with `amount=1`
///   * if the cursor is not on the top-most line of the scrolling region:
///     - move the cursor one line up
///
/// This unsets the pending wrap state without wrapping.
///
/// See <https://terminalguide.namepad.de/seq/a_esc_cm/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct ReverseIndex;

impl ConstEncode for ReverseIndex {
    const STR: &'static str = esc!("M");
}

/// Cursor Up (`CUU`).
///
/// Move cursor up by the specified `amount` of lines.
///
/// If `amount` is greater than the maximum move distance then it is
/// internally adjusted to the maximum. If `amount` is `0`, adjust it to `1`.
///
/// This unsets the pending wrap state without wrapping.
///
/// If the current scroll region is set and the cursor is on or below top-most
/// line of it then the cursor may move up only until it reaches the top-most
/// line of current scroll region.
///
/// If the current scroll region is not set or the cursor is above top-most
/// line of current scroll region it may move up until the top of the screen
/// (excluding scroll-back buffer).
///
/// See <https://terminalguide.namepad.de/seq/csi_ca/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct CursorUp(pub u16);

impl ConstEncodedLen for CursorUp {
    // CSI (2) + max u16 digits (5) + "A" (1) = 8
    const ENCODED_LEN: usize = 8;
}

impl Encode for CursorUp {
    #[inline]
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        write_csi!(buf; self.0, "A")
    }
}

/// Cursor Down (`CUD`).
///
/// Move cursor down by the specified `amount` of lines.
///
/// If `amount` is greater than the maximum move distance then it is
/// internally adjusted to the maximum. This sequence will not scroll the
/// screen or scroll region. If `amount` is `0`, adjust it to `1`.
///
/// This unsets the pending wrap state without wrapping.
///
/// If the current scroll region is set and the cursor is on or above
/// bottom-most line of it then the cursor may move down only until it reaches
/// the bottom-most line of current scroll region.
///
/// If the current scroll region is not set or the cursor is below bottom-most
/// line of current scroll region it may move down until the bottom of the
/// screen.
///
/// See <https://terminalguide.namepad.de/seq/csi_cb/> for
/// terminal support specifics.
pub struct CursorDown(pub u16);

impl ConstEncodedLen for CursorDown {
    // CSI (2) + max u16 digits (5) + "B" (1) = 8
    const ENCODED_LEN: usize = 8;
}

impl Encode for CursorDown {
    #[inline]
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        write_csi!(buf; self.0, "B")
    }
}

/// Cursor Left (`CUB`).
///
/// Move the cursor to the left `amount` cells.
///
/// If `amount` is 0, adjust it to 1.
///
/// This unsets the pending wrap state without wrapping.
///
/// If not both of reverse wrap mode and wraparound mode are set:
///   * Move the cursor `amount` cells left. If it would cross the left-most
///     column of the scrolling region, stop at the left-most column of the
///     scrolling region. If the cursor would move left of the left-most
///     column of the screen, move to the left most column of the screen.
///
/// Else:
///   * If the pending wrap state is set, reduce `amount` by one.
///   * If the cursor is left of the left-most column of the scrolling region:
///     - Move the cursor left `amount` of cells with the following rules:
///     - Each time the cursor is advanced past the left screen edge, continue
///       on the right-most column of the scrolling region on the line above.
///       If that would be before the top-most line of the screen resume on
///       the bottom most line of the screen (ignoring the top and bottom
///       margins of the scrolling region).
///   * If the cursor is on or right of the left-most column of the scrolling
///     region:
///     - Move the cursor left `amount` of cells with the following rules:
///     - Each time the cursor is advanced past the left-most column of the
///       scrolling region, continue on the right-most column of the scrolling
///       region on the line above. If that would be before the top-most line
///       of the screen resume on the bottom most line of the screen (ignoring
///       the top and bottom margins of the scrolling region).
///
/// See <https://terminalguide.namepad.de/seq/csi_cd/> for
/// terminal support specifics.
pub struct CursorLeft(pub u16);

impl ConstEncodedLen for CursorLeft {
    // CSI (2) + max u16 digits (5) + "D" (1) = 8
    const ENCODED_LEN: usize = 8;
}

impl Encode for CursorLeft {
    #[inline]
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        write_csi!(buf; self.0, "D")
    }
}

/// Cursor Right (`CUF`).
///
/// Move the cursor right `amount` columns.
///
/// If `amount` is greater than the maximum move distance then it is
/// internally adjusted to the maximum. This sequence will not scroll the
/// screen or scroll region. If `amount` is 0, adjust it to 1.
///
/// This unsets the pending wrap state without wrapping.
///
/// If left and right margin mode is set and a right margin is set and the
/// cursor is on or left of the right-most column of it then the cursor may
/// move right only until it reaches the right-most column of current scroll
/// region.
///
/// If left and right margin mode is not set or a right margin is not set or
/// the cursor is right of right-most column of current scroll region it may
/// move right until the right-most column of the screen.
///
/// See <https://terminalguide.namepad.de/seq/csi_cc/> for
/// terminal support specifics.
pub struct CursorRight(pub u16);

impl ConstEncodedLen for CursorRight {
    // CSI (2) + max u16 digits (5) + "C" (1) = 8
    const ENCODED_LEN: usize = 8;
}

impl Encode for CursorRight {
    #[inline]
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        write_csi!(buf; self.0, "C")
    }
}

/// Cursor Next Line (`CNL`).
///
/// Move `amount` lines down and to the beginning of the line.
///
/// If `amount` is 0, it is adjusted to 1.
///
/// This is a composition of cursor down with the given `amount` parameter
/// and carriage return.
///
/// See <https://terminalguide.namepad.de/seq/csi_ce/> for
/// terminal support specifics.
pub struct CursorNextLine(pub u16);

impl ConstEncodedLen for CursorNextLine {
    // CSI (2) + max u16 digits (5) + "E" (1) = 8
    const ENCODED_LEN: usize = 8;
}

impl Encode for CursorNextLine {
    #[inline]
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        write_csi!(buf; self.0, "E")
    }
}

/// Cursor Previous Line (`CPL`).
///
/// Move `amount` lines up and to the beginning of the line.
///
/// If `amount` is 0, it is adjusted to 1.
///
/// This is a composition of cursor up with the given `amount` parameter
/// and carriage return.
///
/// See <https://terminalguide.namepad.de/seq/csi_cf/> for
/// terminal support specifics.
pub struct CursorPreviousLine(pub u16);

impl ConstEncodedLen for CursorPreviousLine {
    // CSI (2) + max u16 digits (5) + "F" (1) = 8
    const ENCODED_LEN: usize = 8;
}

impl Encode for CursorPreviousLine {
    #[inline]
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        write_csi!(buf; self.0, "F")
    }
}

/// Cursor Horizontal Absolute (`CHA`).
///
/// Move the cursor to column `col` on the current line.
///
/// If `col` is 0, it is adjusted to 1. If `col` is greater than the
/// right-most column it is adjusted to the right-most column.
///
/// `col` = 1 is the left-most column.
///
/// This unsets the pending wrap state without wrapping.
///
/// See <https://terminalguide.namepad.de/seq/csi_cg/> for
/// terminal support specifics.
pub struct CursorHorizontalAbsolute(pub u16);

impl ConstEncodedLen for CursorHorizontalAbsolute {
    // CSI (2) + max u16 digits (5) + "G" (1) = 8
    const ENCODED_LEN: usize = 8;
}

impl Encode for CursorHorizontalAbsolute {
    #[inline]
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        write_csi!(buf; self.0, "G")
    }
}

/// Cursor Horizontal Forward Tabulation (`CHT`).
///
/// Invoke horizontal tab `amount` times.
///
/// Move cursor to the `amount`-th next tab stop.
///
/// Repeat the following procedure `amount` times:
///
/// Move the cursor right until it reaches a column marked as tab stop
/// (that is not the column the cursor started on) or the right-most
/// column of the screen.
///
/// If cursor origin is set and after this move the cursor is right of the
/// right-most column of the scrolling region, move the cursor to the
/// right-most column of the scrolling region.
///
/// See <https://terminalguide.namepad.de/seq/csi_ci/> for
/// terminal support specifics.
pub struct CursorHorizontalForwardTab(pub u16);

impl ConstEncodedLen for CursorHorizontalForwardTab {
    // CSI (2) + max u16 digits (5) + "I" (1) = 8
    const ENCODED_LEN: usize = 8;
}

impl Encode for CursorHorizontalForwardTab {
    #[inline]
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        write_csi!(buf; self.0, "I")
    }
}

/// Cursor Horizontal Backward Tabulation (`CBT`).
///
/// Move cursor to the `amount`-th previous tab stop.
///
/// Repeat the following procedure `amount` times:
///
/// Move the cursor left until it reaches a column marked as tab stop
/// (that is not the column the cursor started on) or the left-most
/// column of the screen.
///
/// If cursor origin is set and after this move the cursor is left of the
/// left-most column of the scrolling region, move the cursor to the
/// left-most column of the scrolling region.
///
/// See <https://terminalguide.namepad.de/seq/csi_cz/> for
/// terminal support specifics.
pub struct CursorHorizontalBackwardTab(pub u16);

impl ConstEncodedLen for CursorHorizontalBackwardTab {
    // CSI (2) + max u16 digits (5) + "Z" (1) = 8
    const ENCODED_LEN: usize = 8;
}

impl Encode for CursorHorizontalBackwardTab {
    #[inline]
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        write_csi!(buf; self.0, "Z")
    }
}

/// Cursor Horizontal Position Relative (`HPR`).
///
/// Move cursor right by the specified `amount` of columns.
///
/// If `amount` is greater than the maximum move distance then it is
/// internally adjusted to the maximum. This sequence will not scroll the
/// screen or scroll region. If `amount` is 0, adjust it to 1.
///
/// This unsets the pending wrap state without wrapping.
///
/// If left and right margin mode is set and a right margin is set and the
/// cursor is on or left of the right-most column of it then the cursor may
/// move right only until it reaches the right-most column of current scroll
/// region.
///
/// If left and right margin mode is not set or a right margin is not set or
/// the cursor is right of right-most column of current scroll region it may
/// move right until the right-most column of the screen.
///
/// See <https://terminalguide.namepad.de/seq/csi_ca/> for
/// terminal support specifics.
pub struct CursorHorizontalRelative(pub u16);

impl ConstEncodedLen for CursorHorizontalRelative {
    // CSI (2) + max u16 digits (5) + "a" (1) = 8
    const ENCODED_LEN: usize = 8;
}

impl Encode for CursorHorizontalRelative {
    #[inline]
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        write_csi!(buf; self.0, "a")
    }
}

/// Cursor Vertical Position Absolute (`VPA`).
///
/// Move the cursor to row `row` on the current column.
///
/// If `row` is 0, it is adjusted to 1. If `row` is greater than the
/// bottom-most row it is adjusted to the bottom-most row.
///
/// `row` = 1 is the top-most row.
///
/// This unsets the pending wrap state without wrapping.
///
/// If cursor origin mode is set the cursor row will be moved relative to the
/// top margin row and adjusted to be above or at bottom-most row in the
/// current scroll region.
///
/// See <https://terminalguide.namepad.de/seq/csi_cd/> for
/// terminal support specifics.
pub struct CursorVerticalAbsolute(pub u16);

impl ConstEncodedLen for CursorVerticalAbsolute {
    // CSI (2) + max u16 digits (5) + "d" (1) = 8
    const ENCODED_LEN: usize = 8;
}

impl Encode for CursorVerticalAbsolute {
    #[inline]
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        write_csi!(buf; self.0, "d")
    }
}

/// Vertical Position Relative (`VPR`).
///
/// Move cursor down by the specified `amount` of lines.
///
/// If `amount` is greater than the maximum move distance then it is
/// internally adjusted to the maximum. This sequence will not scroll the
/// screen or scroll region. If `amount` is 0, adjust it to 1.
///
/// This unsets the pending wrap state without wrapping.
///
/// If the current scroll region is set and the cursor is on or above
/// bottom-most line of it then the cursor may move down only until it reaches
/// the bottom-most line of current scroll region.
///
/// If the current scroll region is not set or the cursor is below bottom-most
/// line of current scroll region it may move down until the bottom of the
/// screen.
///
/// See <https://terminalguide.namepad.de/seq/csi_ce/> for
/// terminal support specifics.
pub struct CursorVerticalRelative(pub u16);

impl ConstEncodedLen for CursorVerticalRelative {
    // CSI (2) + max u16 digits (5) + "e" (1) = 8
    const ENCODED_LEN: usize = 8;
}

impl Encode for CursorVerticalRelative {
    #[inline]
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        write_csi!(buf; self.0, "e")
    }
}

/// Cursor style variants for `DECSCUSR`.
///
/// These control the visual appearance of the cursor.
///
/// See <https://terminalguide.namepad.de/seq/csi_cq/> for
/// terminal support specifics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorStyle {
    /// Default cursor style (usually blinking block).
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

/// Select Cursor Style (`DECSCUSR`).
///
/// Set the cursor style (shape and blinking).
///
/// The cursor style is set using values 0-6:
///   * 0 - Default cursor style (usually blinking block)
///   * 1 - Blinking block
///   * 2 - Steady block
///   * 3 - Blinking underline
///   * 4 - Steady underline
///   * 5 - Blinking bar
///   * 6 - Steady bar
///
/// See <https://terminalguide.namepad.de/seq/csi_sq_t_space/> for
/// terminal support specifics.
pub struct SetCursorStyle(pub CursorStyle);

impl ConstEncodedLen for SetCursorStyle {
    // CSI (2) + max single digit (1) + " q" (2) = 5
    const ENCODED_LEN: usize = 5;
}

impl Encode for SetCursorStyle {
    #[inline]
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        let code = match self.0 {
            CursorStyle::Default => 0,
            CursorStyle::BlinkingBlock => 1,
            CursorStyle::SteadyBlock => 2,
            CursorStyle::BlinkingUnderline => 3,
            CursorStyle::SteadyUnderline => 4,
            CursorStyle::BlinkingBar => 5,
            CursorStyle::SteadyBar => 6,
        };
        write_csi!(buf; code, " q")
    }
}

/// Request Cursor Style (`DECRQSS`).
///
/// Request the current cursor style.
///
/// The terminal will respond with a DCS sequence containing the current
/// cursor style setting.
///
/// See <https://terminalguide.namepad.de/seq/dcs-dollar-q-space-q/> for
/// terminal support specifics.
pub struct RequestCursorStyle;

impl ConstEncode for RequestCursorStyle {
    const STR: &'static str = dcs!("$q q");
}

bitflags! {
    /// Flags for Linux cursor style.
    ///
    /// These flags control the cursor appearance and behavior in the Linux
    /// virtual console.
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(transparent))]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct LinuxCursorStyleFlags: u8 {
        /// Enable foreground and background change.
        ///
        /// When enabled the cursor changes the shown attributes of the
        /// cell it is on. Some drivers force size = 1 (none) internally
        /// if this is set.
        const ENABLE_FG_BG_CHANGE = 16;

        /// Ensure original background and cursor background differ.
        ///
        /// If the original and cursor background would be identical
        /// invert all background color channels (but not brightness).
        const ENSURE_BG_DIFFERS = 32;

        /// Ensure cursor foreground and background differ.
        ///
        /// If the cursor background and foreground would be identical
        /// invert all foreground color channels (but not brightness).
        const ENSURE_FG_BG_DIFFER = 64;
    }
}

/// Linux Cursor Style shape values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum LinuxCursorShape {
    /// Default (depending on driver: off, underline or block).
    Default = 0,
    /// No cursor.
    None = 1,
    /// Underline cursor.
    Underline = 2,
    /// Lower third cursor.
    LowerThird = 3,
    /// Lower half cursor.
    LowerHalf = 4,
    /// Two thirds cursor.
    TwoThirds = 5,
    /// Block cursor.
    Block = 6,
}

/// Linux Cursor Style.
///
/// Select Linux cursor style with fine-grained control over appearance.
///
/// This sequence allows setting the cursor shape, flags for attribute
/// changes, and XOR/OR masks for foreground and background color
/// manipulation.
///
/// The `shape` parameter combines the size (0-6) with optional flags
/// (16, 32, 64).
///
/// The `xor` and `or` parameters define changes to foreground and
/// background of the cell where the cursor is shown when the
/// `ENABLE_FG_BG_CHANGE` flag is set. Each bit controls one color channel:
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
/// The effective change for each bit depends on its value in both
/// parameters:
///
/// | or bit | xor bit |   change  |
/// |--------|---------|-----------|
/// |    0   |    0    | no change |
/// |    1   |    0    | enable    |
/// |    0   |    1    | toggle    |
/// |    1   |    1    | disable   |
///
/// See <https://terminalguide.namepad.de/seq/csi_sc__p/> for terminal
/// support specifics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LinuxCursorStyle {
    /// Cursor shape.
    pub shape: LinuxCursorShape,
    /// Cursor style flags.
    pub flags: LinuxCursorStyleFlags,
    /// XOR mask for color channel manipulation.
    pub xor: u8,
    /// OR mask for color channel manipulation.
    pub or: u8,
}

impl LinuxCursorStyle {
    /// Create a new Linux cursor style with the specified shape.
    ///
    /// Flags, xor, and or values are initialized to 0.
    #[must_use]
    pub const fn new(shape: LinuxCursorShape) -> Self {
        Self {
            shape,
            flags: LinuxCursorStyleFlags::empty(),
            xor: 0,
            or: 0,
        }
    }

    /// Create a new Linux cursor style with shape and flags.
    ///
    /// xor and or values are initialized to 0.
    #[must_use]
    pub const fn with_flags(
        shape: LinuxCursorShape,
        flags: LinuxCursorStyleFlags,
    ) -> Self {
        Self {
            shape,
            flags,
            xor: 0,
            or: 0,
        }
    }

    /// Create a new Linux cursor style with all parameters.
    #[must_use]
    pub const fn with_colors(
        shape: LinuxCursorShape,
        flags: LinuxCursorStyleFlags,
        xor: u8,
        or: u8,
    ) -> Self {
        Self {
            shape,
            flags,
            xor,
            or,
        }
    }

    fn shape_value(self) -> u8 {
        (self.shape as u8) | self.flags.bits()
    }
}

impl ConstEncodedLen for LinuxCursorStyle {
    // CSI (2) + max shape (3) + ";" (1) + max xor (3) + ";" (1) + max or
    // (3) + " q" (2) = 15
    const ENCODED_LEN: usize = 15;
}

impl Encode for LinuxCursorStyle {
    #[inline]
    fn encode<W: std::io::Write>(
        &mut self,
        buf: &mut W,
    ) -> Result<usize, EncodeError> {
        write_csi!(buf; self.shape_value(), ";", self.xor, ";", self.or, " q")
    }
}


/// Request Cursor Position Report (`CPR`).
///
/// Request the current cursor position.
///
/// The terminal replies with:
///
/// `CSI <row> ; <column> R`
///
/// If [`RelativeCursorOriginMode`] is set, the cursor position is reported
/// relative to the top left corner of the scroll area. Otherwise, it is
/// reported relative to the top left corner of the screen.
///
/// The response uses [`CursorPositionReport`].
///
/// See <https://terminalguide.namepad.de/seq/csi_sn-6/> for
/// terminal support specifics.
pub struct RequestCursorPosition;

impl ConstEncode for RequestCursorPosition {
    const STR: &'static str = csi!("6n");
}

/// Cursor Position Report (`CPR`).
///
/// Response from the terminal to [`RequestCursorPosition`].
///
/// Contains the current cursor position as `row` and `col`.
///
/// The position may be relative to the scroll area if
/// [`RelativeCursorOriginMode`] is set, or relative to the screen otherwise.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CursorPositionReport {
    pub row: u16,
    pub col: u16,
}

impl ConstEncodedLen for CursorPositionReport {
    // CSI (2) + max u16 digits (5) + ";" (1) + max u16 digits (5) + "R" (1) = 14
    const ENCODED_LEN: usize = 14;
}

impl Encode for CursorPositionReport {
    #[inline]
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        write_csi!(buf; self.row, ";", self.col, "R")
    }
}
