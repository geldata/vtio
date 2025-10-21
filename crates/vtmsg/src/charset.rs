//! Terminal character set control and information messages.

use vtenc::{ConstEncode, Encode, ConstEncodedLen, EncodeError, esc, write_esc};

/// Enable UTF-8 mode.
///
/// Set character set to UTF-8.
///
/// See <https://terminalguide.namepad.de/seq/a_esc_zpercent_cg/> for
/// terminal support specifics and details.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct EnableUTF8Mode;

impl ConstEncode for EnableUTF8Mode {
    const STR: &'static str = esc!("%G");
}

/// Disable UTF-8 mode.
///
/// See <https://terminalguide.namepad.de/seq/a_esc_zpercent_x40_at/> for
/// terminal support specifics and details.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct DisableUTF8Mode;

impl ConstEncode for DisableUTF8Mode {
    const STR: &'static str = esc!("%@");
}

/// Shift Out (SO).
///
/// Invoke G1 character set into GL (left half of character table).
/// Maps G1 character set into the left (GL) character positions.
///
/// See <https://terminalguide.namepad.de/seq/a_c0-n/> for terminal support
/// specifics and details.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct ShiftOut;

impl ConstEncode for ShiftOut {
    const STR: &'static str = "\x0E";
}

/// Shift In (SI).
///
/// Invoke G0 character set into GL (left half of character table).
/// Maps G0 character set into the left (GL) character positions.
///
/// See <https://terminalguide.namepad.de/seq/a_c0-o/> for terminal support
/// specifics and details.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct ShiftIn;

impl ConstEncode for ShiftIn {
    const STR: &'static str = "\x0F";
}

/// Locking Shift 2 (LS2).
///
/// Invoke G2 character set into GL (left half of character table).
///
/// See <https://terminalguide.namepad.de/seq/a_esc_sn/> for terminal support
/// specifics and details.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct LockingShift2;

impl ConstEncode for LockingShift2 {
    const STR: &'static str = esc!("n");
}

/// Locking Shift 3 (LS3).
///
/// Invoke G3 character set into GL (left half of character table).
///
/// See <https://terminalguide.namepad.de/seq/a_esc_so/> for terminal support
/// specifics and details.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct LockingShift3;

impl ConstEncode for LockingShift3 {
    const STR: &'static str = esc!("o");
}

/// Locking Shift 1 Right (LS1R).
///
/// Invoke G1 character set into GR (right half of character table).
///
/// See <https://terminalguide.namepad.de/seq/a_esc_x7e_tilde/> for terminal
/// support specifics and details.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct LockingShift1Right;

impl ConstEncode for LockingShift1Right {
    const STR: &'static str = esc!("~");
}

/// Locking Shift 2 Right (LS2R).
///
/// Invoke G2 character set into GR (right half of character table).
///
/// See <https://terminalguide.namepad.de/seq/a_esc_x7d_right_brace/> for
/// terminal support specifics and details.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct LockingShift2Right;

impl ConstEncode for LockingShift2Right {
    const STR: &'static str = esc!("}");
}

/// Locking Shift 3 Right (LS3R).
///
/// Invoke G3 character set into GR (right half of character table).
///
/// See <https://terminalguide.namepad.de/seq/a_esc_x7c_pipe/> for terminal
/// support specifics and details.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct LockingShift3Right;

impl ConstEncode for LockingShift3Right {
    const STR: &'static str = esc!("|");
}

/// Single Shift 2 (SS2).
///
/// Temporarily invoke G2 character set for the next character only.
///
/// See <https://terminalguide.namepad.de/seq/a_esc_cn/> for terminal
/// support specifics and details.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct SingleShift2;

impl ConstEncode for SingleShift2 {
    const STR: &'static str = esc!("N");
}

/// Single Shift 3 (SS3).
///
/// Temporarily invoke G3 character set for the next character only.
///
/// See <https://terminalguide.namepad.de/seq/a_esc_co/> for terminal
/// support specifics and details.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct SingleShift3;

impl ConstEncode for SingleShift3 {
    const STR: &'static str = esc!("O");
}

/// Character set codes used in charset designation sequences.
///
/// These codes identify specific character sets that can be designated to
/// G0, G1, G2, or G3 charset registers.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub enum CharsetCode {
    /// ASCII character set.
    Ascii,
    /// British character set.
    British,
    /// DEC Special Character and Line Drawing Set.
    DecSpecialGraphic,
    /// DEC Alternate Character Set.
    DecAltChars,
    /// DEC Alternate Graphics.
    DecAltGraphics,
    /// DEC Supplemental character set.
    DecSupp,
    /// Dutch character set.
    Dutch,
    /// Finnish character set.
    Finnish,
    /// Finnish character set (variant 2).
    Finnish2,
    /// French character set.
    French,
    /// French character set (variant 2).
    French2,
    /// French Canadian character set.
    FrenchCanadian,
    /// French Canadian character set (variant 2).
    FrenchCanadian2,
    /// German character set.
    German,
    /// Italian character set.
    Italian,
    /// Norwegian/Danish character set.
    NorwegianDanish,
    /// Norwegian/Danish character set (variant 2).
    NorwegianDanish2,
    /// Norwegian/Danish character set (variant 3).
    NorwegianDanish3,
    /// Spanish character set.
    Spanish,
    /// Swedish character set.
    Swedish,
    /// Swedish character set (variant 2).
    Swedish2,
    /// Swiss character set.
    Swiss,
    /// DEC Technical character set.
    DecTechnical,
    /// DEC Supplemental Graphic character set.
    DecSuppGraphic,
    /// Portuguese character set.
    Portuguese,
    /// Turkish character set.
    Turkish,
    /// Turkish Supplement character set.
    TurkishSupplement,
    /// Hebrew character set.
    Hebrew,
    /// DEC Hebrew Supplement character set.
    DecHebrewSupplement,
    /// Greek character set.
    Greek,
    /// DEC Greek Supplement character set.
    DecGreekSupplement,
    /// IBM code page 437 (Linux console only).
    Cp437,
    /// Latin-1 Supplemental (96-character set).
    Latin1Supplemental,
    /// Greek (bottom part of ISO-8859-7, 96-character set).
    GreekSupplemental,
    /// Hebrew (bottom part of ISO-8859-8, 96-character set).
    HebrewSupplemental,
    /// Latin-Cyrillic (bottom part of ISO-8859-5, 96-character set).
    LatinCyrillic,
    /// Latin-5 (bottom part of ISO-8859-9, 96-character set).
    Latin5,
}

impl CharsetCode {
    /// Get the character code for this charset.
    ///
    /// Note: Some different charsets use the same designator character but
    /// are distinguished by context (94-char vs 96-char designation
    /// sequences). For example, "A" means British in `ESC(A` but Latin-1
    /// Supplemental in `ESC-A`.
    #[allow(clippy::match_same_arms)]
    const fn code(self) -> &'static str {
        match self {
            Self::Ascii => "B",
            Self::British => "A",
            Self::DecSpecialGraphic => "0",
            Self::DecAltChars => "1",
            Self::DecAltGraphics => "2",
            Self::DecSupp => "<",
            Self::Dutch => "4",
            Self::Finnish => "5",
            Self::Finnish2 => "C",
            Self::French => "R",
            Self::French2 => "f",
            Self::FrenchCanadian => "Q",
            Self::FrenchCanadian2 => "9",
            Self::German => "K",
            Self::Italian => "Y",
            Self::NorwegianDanish => "`",
            Self::NorwegianDanish2 => "E",
            Self::NorwegianDanish3 => "6",
            Self::Spanish => "Z",
            Self::Swedish => "7",
            Self::Swedish2 => "H",
            Self::Swiss => "=",
            Self::DecTechnical => ">",
            Self::DecSuppGraphic => "%5",
            Self::Portuguese => "%6",
            Self::Turkish => "%0",
            Self::TurkishSupplement => "%2",
            Self::Hebrew => "%=",
            Self::DecHebrewSupplement => "\"4",
            Self::Greek => "\">",
            Self::DecGreekSupplement => "\"?",
            Self::Cp437 => "U",
            Self::Latin1Supplemental => "A",
            Self::GreekSupplemental => "F",
            Self::HebrewSupplemental => "H",
            Self::LatinCyrillic => "L",
            Self::Latin5 => "M",
        }
    }


}

/// Designate G0 Character Set (94 characters).
///
/// Designate a 94-character set to the G0 character set register.
/// This is part of the ISO-2022 character set designation mechanism.
///
/// See <https://terminalguide.namepad.de/seq/> charset designation section
/// for terminal support specifics and details.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct DesignateG0 {
    pub charset: CharsetCode,
}

impl ConstEncodedLen for DesignateG0 {
    // ESC ( + charset code (1-2 bytes)
    const ENCODED_LEN: usize = 4;
}

impl Encode for DesignateG0 {
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        write_esc!(buf; "(", self.charset.code())
    }
}

/// Designate G1 Character Set (94 characters).
///
/// Designate a 94-character set to the G1 character set register.
/// This is part of the ISO-2022 character set designation mechanism.
///
/// See <https://terminalguide.namepad.de/seq/> charset designation section
/// for terminal support specifics and details.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct DesignateG1 {
    pub charset: CharsetCode,
}

impl ConstEncodedLen for DesignateG1 {
    // ESC ) + charset code (1-2 bytes)
    const ENCODED_LEN: usize = 4;
}

impl Encode for DesignateG1 {
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        write_esc!(buf; ")", self.charset.code())
    }
}

/// Designate G2 Character Set (94 characters).
///
/// Designate a 94-character set to the G2 character set register.
/// This is part of the ISO-2022 character set designation mechanism.
///
/// See <https://terminalguide.namepad.de/seq/> charset designation section
/// for terminal support specifics and details.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct DesignateG2 {
    pub charset: CharsetCode,
}

impl ConstEncodedLen for DesignateG2 {
    // ESC * + charset code (1-2 bytes)
    const ENCODED_LEN: usize = 4;
}

impl Encode for DesignateG2 {
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        write_esc!(buf; "*", self.charset.code())
    }
}

/// Designate G3 Character Set (94 characters).
///
/// Designate a 94-character set to the G3 character set register.
/// This is part of the ISO-2022 character set designation mechanism.
///
/// See <https://terminalguide.namepad.de/seq/> charset designation section
/// for terminal support specifics and details.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct DesignateG3 {
    pub charset: CharsetCode,
}

impl ConstEncodedLen for DesignateG3 {
    // ESC + + charset code (1-2 bytes)
    const ENCODED_LEN: usize = 4;
}

impl Encode for DesignateG3 {
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        write_esc!(buf; "+", self.charset.code())
    }
}

/// Designate G1 Character Set (96 characters).
///
/// Designate a 96-character set to the G1 character set register.
/// This is part of the ISO-2022 character set designation mechanism.
///
/// See <https://terminalguide.namepad.de/seq/> charset designation section
/// for terminal support specifics and details.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct DesignateG1_96 {
    pub charset: CharsetCode,
}

impl ConstEncodedLen for DesignateG1_96 {
    // ESC - + charset code (1 byte for 96-char sets)
    const ENCODED_LEN: usize = 3;
}

impl Encode for DesignateG1_96 {
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        write_esc!(buf; "-", self.charset.code())
    }
}

/// Designate G2 Character Set (96 characters).
///
/// Designate a 96-character set to the G2 character set register.
/// This is part of the ISO-2022 character set designation mechanism.
///
/// See <https://terminalguide.namepad.de/seq/> charset designation section
/// for terminal support specifics and details.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct DesignateG2_96 {
    pub charset: CharsetCode,
}

impl ConstEncodedLen for DesignateG2_96 {
    // ESC . + charset code (1 byte for 96-char sets)
    const ENCODED_LEN: usize = 3;
}

impl Encode for DesignateG2_96 {
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        write_esc!(buf; ".", self.charset.code())
    }
}

/// Designate G3 Character Set (96 characters).
///
/// Designate a 96-character set to the G3 character set register.
/// This is part of the ISO-2022 character set designation mechanism.
///
/// See <https://terminalguide.namepad.de/seq/> charset designation section
/// for terminal support specifics and details.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct DesignateG3_96 {
    pub charset: CharsetCode,
}

impl ConstEncodedLen for DesignateG3_96 {
    // ESC / + charset code (1 byte for 96-char sets)
    const ENCODED_LEN: usize = 3;
}

impl Encode for DesignateG3_96 {
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        write_esc!(buf; "/", self.charset.code())
    }
}
