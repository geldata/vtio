//! Screen and line erase commands.

use vtenc::{ConstEncode, ConstEncodedLen, Encode, EncodeError, format_csi, format_esc, write_csi};

/// Erase Display Below (`ED`).
///
/// Erase from cursor position (inclusive) to the end of the screen.
///
/// This is the same as `CSI 0 J` or `CSI J`.
///
/// Erases all cells from the cursor position to the end of the screen,
/// including the cell at the cursor position. This includes all cells on
/// the current line from the cursor to the end, and all cells on all lines
/// below the cursor line.
///
/// The erased cells are replaced with spaces using the current SGR
/// attributes (background color, etc.).
///
/// Does not move the cursor.
///
/// See <https://terminalguide.namepad.de/seq/csi_cj/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct EraseDisplayBelow;

impl ConstEncode for EraseDisplayBelow {
    const STR: &'static str = format_csi!("J");
}

/// Erase Display Above (`ED`).
///
/// Erase from the beginning of the screen to cursor position (inclusive).
///
/// Erases all cells from the beginning of the screen to the cursor position,
/// including the cell at the cursor position. This includes all cells on all
/// lines above the cursor line, and all cells from the beginning of the
/// current line to the cursor position.
///
/// The erased cells are replaced with spaces using the current SGR
/// attributes (background color, etc.).
///
/// Does not move the cursor.
///
/// See <https://terminalguide.namepad.de/seq/csi_cj/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct EraseDisplayAbove;

impl ConstEncode for EraseDisplayAbove {
    const STR: &'static str = format_csi!("1J");
}

/// Erase Display Complete (`ED`).
///
/// Erase the entire screen.
///
/// Erases all cells on the screen. The cursor position does not change.
///
/// Cells are cleared by replacing their contents with spaces, using the
/// current SGR attributes for the background color.
///
/// Note: This does not clear the scrollback buffer. Use
/// [`EraseDisplayScrollback`] for that. In some terminals (e.g., xterm with
/// specific options), this may scroll the screen content into the scrollback
/// buffer before clearing.
///
/// See <https://terminalguide.namepad.de/seq/csi_cj/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct EraseDisplayComplete;

impl ConstEncode for EraseDisplayComplete {
    const STR: &'static str = format_csi!("2J");
}

/// Erase Display Scroll-back (`ED`).
///
/// Erase the scrollback buffer.
///
/// This is an extended command that clears the terminal's scrollback buffer
/// (the off-screen history of previously displayed content). The visible
/// screen content is not affected.
///
/// This sequence is not supported by all terminals (notably not by urxvt).
///
/// Does not move the cursor.
///
/// See <https://terminalguide.namepad.de/seq/csi_cj/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct EraseDisplayScrollback;

impl ConstEncode for EraseDisplayScrollback {
    const STR: &'static str = format_csi!("3J");
}

/// Erase Line Right (`EL`).
///
/// Erase from cursor position (inclusive) to the end of the line.
///
/// Erases all cells from the cursor position to the end of the current line,
/// including the cell at the cursor position. The cursor position does not
/// change.
///
/// Cells are cleared by replacing their contents with spaces, using the
/// current SGR attributes for the background color.
///
/// See <https://terminalguide.namepad.de/seq/csi_ck/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct EraseLineRight;

impl ConstEncode for EraseLineRight {
    const STR: &'static str = format_csi!("K");
}

/// Erase Line Left (`EL`).
///
/// Erase from the beginning of the line to cursor position (inclusive).
///
/// Erases all cells from the beginning of the current line to the cursor
/// position, including the cell at the cursor position. The cursor position
/// does not change.
///
/// Cells are cleared by replacing their contents with spaces, using the
/// current SGR attributes for the background color.
///
/// See <https://terminalguide.namepad.de/seq/csi_ck/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct EraseLineLeft;

impl ConstEncode for EraseLineLeft {
    const STR: &'static str = format_csi!("1K");
}

/// Erase Line Complete (`EL`).
///
/// Erase the entire line.
///
/// This is the same as `CSI 2 K`.
///
/// Erases all cells on the current line. The cursor position does not
/// change.
///
/// Cells are cleared by replacing their contents with spaces, using the
/// current SGR attributes for the background color.
///
/// See <https://terminalguide.namepad.de/seq/csi_ck/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct EraseLineComplete;

impl ConstEncode for EraseLineComplete {
    const STR: &'static str = format_csi!("2K");
}

/// Selective Erase Display Below (`DECSED`).
///
/// Erase from cursor position (inclusive) to the end of the screen,
/// preserving protected cells.
///
/// This is the same as `CSI ? 0 J` or `CSI ? J`.
///
/// Like [`EraseDisplayBelow`], but does not erase cells marked with
/// protected state. Protected cells retain their content while unprotected
/// cells are replaced with spaces using the current SGR attributes.
///
/// The cursor position does not change.
///
/// See <https://terminalguide.namepad.de/seq/csi_cj__p/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct SelectiveEraseDisplayBelow;

impl ConstEncode for SelectiveEraseDisplayBelow {
    const STR: &'static str = format_csi!("?J");
}

/// Selective Erase Display Above (`DECSED`).
///
/// Erase from the beginning of the screen to cursor position (inclusive),
/// preserving protected cells.
///
/// This is the same as `CSI ? 1 J`.
///
/// Like [`EraseDisplayAbove`], but does not erase cells marked with
/// protected state. Protected cells retain their content while unprotected
/// cells are replaced with spaces using the current SGR attributes.
///
/// The cursor position does not change.
///
/// See <https://terminalguide.namepad.de/seq/csi_cj__p/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct SelectiveEraseDisplayAbove;

impl ConstEncode for SelectiveEraseDisplayAbove {
    const STR: &'static str = format_csi!("?1J");
}

/// Selective Erase Display Complete (`DECSED`).
///
/// Erase the entire screen, preserving protected cells.
///
/// Like [`EraseDisplayComplete`], but does not erase cells marked with
/// protected state. Protected cells retain their content while unprotected
/// cells are replaced with spaces using the current SGR attributes.
///
/// The cursor position does not change.
///
/// See <https://terminalguide.namepad.de/seq/csi_cj__p/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct SelectiveEraseDisplayComplete;

impl ConstEncode for SelectiveEraseDisplayComplete {
    const STR: &'static str = format_csi!("?2J");
}

/// Selective Erase Line Right (`DECSEL`).
///
/// Erase from cursor position (inclusive) to the end of the line,
/// preserving protected cells.
///
/// Like [`EraseLineRight`], but does not erase cells marked with protected
/// state. Protected cells retain their content while unprotected cells are
/// replaced with spaces using the current SGR attributes.
///
/// The cursor position does not change.
///
/// See <https://terminalguide.namepad.de/seq/csi_ck__p/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct SelectiveEraseLineRight;

impl ConstEncode for SelectiveEraseLineRight {
    const STR: &'static str = format_csi!("?K");
}

/// Selective Erase Line Left (`DECSEL`).
///
/// Erase from the beginning of the line to cursor position (inclusive),
/// preserving protected cells.
///
/// Like [`EraseLineLeft`], but does not erase cells marked with protected
/// state. Protected cells retain their content while unprotected cells are
/// replaced with spaces using the current SGR attributes.
///
/// The cursor position does not change.
///
/// See <https://terminalguide.namepad.de/seq/csi_ck__p/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct SelectiveEraseLineLeft;

impl ConstEncode for SelectiveEraseLineLeft {
    const STR: &'static str = format_csi!("?1K");
}

/// Selective Erase Line Complete (`DECSEL`).
///
/// Erase the entire line, preserving protected cells.
///
/// Like [`EraseLineComplete`], but does not erase cells marked with
/// protected state. Protected cells retain their content while unprotected
/// cells are replaced with spaces using the current SGR attributes.
///
/// The cursor position does not change.
///
/// See <https://terminalguide.namepad.de/seq/csi_ck__p/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct SelectiveEraseLineComplete;

impl ConstEncode for SelectiveEraseLineComplete {
    const STR: &'static str = format_csi!("?2K");
}

/// Insert Line (`IL`).
///
/// Insert `amount` lines at the current cursor row.
///
/// This is the same as `CSI Ps L`.
///
/// The contents of the line at the current cursor row and below (to the
/// bottom-most line in the scrolling region) are shifted down by `amount`
/// lines. The contents of the `amount` bottom-most lines in the scroll
/// region are lost.
///
/// If the current cursor position is outside of the current scroll region,
/// it does nothing.
///
/// If `amount` is greater than the remaining number of lines in the
/// scrolling region, it is adjusted down.
///
/// In left and right margin mode, the margins are respected; lines are only
/// scrolled in the scroll region.
///
/// All cleared space is colored according to the current SGR state.
///
/// This unsets the pending wrap state without wrapping.
///
/// Moves the cursor to the left margin.
///
/// See <https://terminalguide.namepad.de/seq/csi_cl/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct InsertLine(pub u16);

impl ConstEncodedLen for InsertLine {
    // CSI (2) + max u16 digits (5) + "L" (1) = 8
    const ENCODED_LEN: usize = 8;
}

impl Encode for InsertLine {
    #[inline]
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        write_csi!(buf; self.0, "L")
    }
}

/// Delete Line (`DL`).
///
/// Remove `amount` lines from the current cursor row down.
///
/// This is the same as `CSI Ps M`.
///
/// The remaining lines to the bottom margin are shifted up and space from
/// the bottom margin up is filled with empty lines.
///
/// If the current cursor position is outside of the current scroll region,
/// it does nothing.
///
/// If `amount` is greater than the remaining number of lines in the
/// scrolling region, it is adjusted down.
///
/// In left and right margin mode, the margins are respected; lines are only
/// scrolled in the scroll region.
///
/// All cleared space is colored according to the current SGR state.
///
/// This unsets the pending wrap state without wrapping.
///
/// Moves the cursor to the left margin.
///
/// See <https://terminalguide.namepad.de/seq/csi_cm/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct DeleteLine(pub u16);

impl ConstEncodedLen for DeleteLine {
    // CSI (2) + max u16 digits (5) + "M" (1) = 8
    const ENCODED_LEN: usize = 8;
}

impl Encode for DeleteLine {
    #[inline]
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        write_csi!(buf; self.0, "M")
    }
}

/// Delete Character (`DCH`).
///
/// Remove `amount` characters from the current cursor position to the right.
///
/// This is the same as `CSI Ps P`.
///
/// The remaining characters are shifted to the left and space from the right
/// margin is filled with spaces.
///
/// If the current cursor column is not between the left and right margin, it
/// does nothing.
///
/// If `amount` is greater than the remaining number of characters in the
/// scrolling region, it is adjusted down.
///
/// In left and right margin mode, the margins are respected; characters are
/// only scrolled in the scroll region.
///
/// All newly cleared space starting from the right margin is colored
/// according to the current SGR state.
///
/// Does not change the cursor position.
///
/// This unsets the pending wrap state without wrapping.
///
/// See <https://terminalguide.namepad.de/seq/csi_cp/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct DeleteCharacter(pub u16);

impl ConstEncodedLen for DeleteCharacter {
    // CSI (2) + max u16 digits (5) + "P" (1) = 8
    const ENCODED_LEN: usize = 8;
}

impl Encode for DeleteCharacter {
    #[inline]
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        write_csi!(buf; self.0, "P")
    }
}

/// Insert Column (`DECIC`).
///
/// Insert `amount` columns at the current cursor column.
///
/// This is the same as `CSI Ps ' }`.
///
/// Inserts `amount` columns (over the whole height of the current scrolling
/// region) from the current cursor column. The contents of the column at the
/// current cursor column and the columns to its right (to the right-most
/// column in the scrolling region) are shifted right by `amount` columns.
/// The contents of the `amount` right-most columns in the scroll region are
/// lost.
///
/// If the current cursor position is outside of the current scroll region,
/// it does nothing.
///
/// If `amount` is greater than the remaining number of columns in the
/// scrolling region, it is adjusted down.
///
/// In left and right margin mode, the margins are respected; columns are
/// only scrolled in the scroll region.
///
/// All cleared space is colored according to the current SGR state.
///
/// The cursor position is not changed.
///
/// See <https://terminalguide.namepad.de/seq/csi_x7d_right_brace_t_tick/>
/// for terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct InsertColumn(pub u16);

impl ConstEncodedLen for InsertColumn {
    // CSI (2) + max u16 digits (5) + "'" (1) + "}" (1) = 9
    const ENCODED_LEN: usize = 9;
}

impl Encode for InsertColumn {
    #[inline]
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        write_csi!(buf; self.0, "\'}")
    }
}

/// Delete Column (`DECDC`).
///
/// Remove `amount` columns from the current cursor column to the right.
///
/// This is the same as `CSI Ps ' ~`.
///
/// Removes `amount` columns (over the whole height of the current scrolling
/// region) from the current cursor column to the right. The remaining
/// columns to the right are shifted left and space from the right margin is
/// filled with empty cells.
///
/// If the current cursor position is outside of the current scroll region,
/// it does nothing.
///
/// If `amount` is greater than the remaining number of columns in the
/// scrolling region, it is adjusted down.
///
/// In left and right margin mode, the margins are respected; columns are
/// only scrolled in the scroll region.
///
/// All cleared space is colored according to the current SGR state.
///
/// The cursor position is not changed.
///
/// See <https://terminalguide.namepad.de/seq/csi_x7e_tilde_t_tick/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct DeleteColumn(pub u16);

impl ConstEncodedLen for DeleteColumn {
    // CSI (2) + max u16 digits (5) + "'" (1) + "~" (1) = 9
    const ENCODED_LEN: usize = 9;
}

impl Encode for DeleteColumn {
    #[inline]
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        write_csi!(buf; self.0, "\'~")
    }
}

/// Fill Screen with E (`DECALN`).
///
/// Fill the entire screen with the character 'E'.
///
/// This command is primarily used for screen alignment testing. It fills
/// the entire screen with the letter 'E' and moves the cursor to the top
/// left corner (1,1).
///
/// See <https://terminalguide.namepad.de/seq/a_esc_zhash_a8/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct FillScreenWithE;

impl ConstEncode for FillScreenWithE {
    const STR: &'static str = format_esc!("#8");
}

/// Set Double Height Line Top Half (`DECDHL`).
///
/// Display double width and double height text (top half).
///
/// Sets a per-line attribute that allows displaying double height text.
/// For proper text display, two consecutive lines with identical text
/// content need to be output. The first line needs to be set with this
/// sequence, and the second line needs to be set with
/// [`SetDoubleHeightLineBottomHalf`].
///
/// If the line mode is switched from single width to this mode, the
/// number of columns is halved. If the cursor was in the second half of
/// the row, the cursor is moved to the new right-most column. The
/// columns no longer visible keep their contents and are revealed when
/// [`SetSingleWidthLine`] is set for the line.
///
/// In left and right margin mode, this control is ignored.
///
/// See <https://terminalguide.namepad.de/seq/a_esc_zhash_a3/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct SetDoubleHeightLineTopHalf;

impl ConstEncode for SetDoubleHeightLineTopHalf {
    const STR: &'static str = format_esc!("#3");
}

/// Set Double Height Line Bottom Half (`DECDHL`).
///
/// Display double width and double height text (bottom half).
///
/// Sets a per-line attribute that allows displaying double height text.
/// For proper text display, two consecutive lines with identical text
/// content need to be output. The first line needs to be set with
/// [`SetDoubleHeightLineTopHalf`], and the second line needs to be set
/// with this sequence.
///
/// If the line mode is switched from single width to this mode, the
/// number of columns is halved. If the cursor was in the second half of
/// the row, the cursor is moved to the new right-most column. The
/// columns no longer visible keep their contents and are revealed when
/// [`SetSingleWidthLine`] is set for the line.
///
/// In left and right margin mode, this control is ignored.
///
/// See <https://terminalguide.namepad.de/seq/a_esc_zhash_a4/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct SetDoubleHeightLineBottomHalf;

impl ConstEncode for SetDoubleHeightLineBottomHalf {
    const STR: &'static str = format_esc!("#4");
}

/// Set Single Width Line (`DECSWL`).
///
/// Reset a line to normal single width and single height display mode.
///
/// This undoes the effect of [`SetDoubleHeightLineTopHalf`],
/// [`SetDoubleHeightLineBottomHalf`], and [`SetDoubleWidthLine`]. The
/// displayable columns are restored and the previously hidden text is
/// revealed.
///
/// See <https://terminalguide.namepad.de/seq/a_esc_zhash_a5/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct SetSingleWidthLine;

impl ConstEncode for SetSingleWidthLine {
    const STR: &'static str = format_esc!("#5");
}

/// Set Double Width Line (`DECDWL`).
///
/// Display double width and single height text.
///
/// Sets a per-line attribute that allows displaying double width text.
/// Text is displayed using double the normal amount of cell spaces per
/// character.
///
/// If the line mode is switched from single width to this mode, the
/// number of columns is halved. If the cursor was in the second half of
/// the row, the cursor is moved to the new right-most column. The
/// columns no longer visible keep their contents and are revealed when
/// [`SetSingleWidthLine`] is set for the line.
///
/// In left and right margin mode, this control is ignored.
///
/// See <https://terminalguide.namepad.de/seq/a_esc_zhash_a6/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct SetDoubleWidthLine;

impl ConstEncode for SetDoubleWidthLine {
    const STR: &'static str = format_esc!("#6");
}
