//! Terminal character set control and information messages.

use vtio_control_derive::{c0, esc};
use vtenc::WriteSeq;

/// Enable UTF-8 mode.
///
/// Set character set to UTF-8.
///
/// See <https://terminalguide.namepad.de/seq/a_esc_zpercent_cg/> for
/// terminal support specifics and details.
#[esc(finalbyte = 'G', intermediate = "%")]
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct EnableUTF8Mode;

/// Disable UTF-8 mode.
///
/// See <https://terminalguide.namepad.de/seq/a_esc_zpercent_x40_at/> for
/// terminal support specifics and details.
#[esc(finalbyte = '@', intermediate = "%")]
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct DisableUTF8Mode;

/// Shift Out (SO).
///
/// Invoke G1 character set into GL (left half of character table).
/// Maps G1 character set into the left (GL) character positions.
///
/// See <https://terminalguide.namepad.de/seq/a_c0-n/> for terminal support
/// specifics and details.
#[c0(code = 0x0E)]
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct ShiftOut;

/// Shift In (SI).
///
/// Invoke G0 character set into GL (left half of character table).
/// Maps G0 character set into the left (GL) character positions.
///
/// See <https://terminalguide.namepad.de/seq/a_c0-o/> for terminal support
/// specifics and details.
#[c0(code = 0x0F)]
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct ShiftIn;

/// Locking Shift 2 (LS2).
///
/// Invoke G2 character set into GL (left half of character table).
///
/// See <https://terminalguide.namepad.de/seq/a_esc_sn/> for terminal support
/// specifics and details.
#[esc(finalbyte = 'n')]
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct LockingShift2;

/// Locking Shift 3 (LS3).
///
/// Invoke G3 character set into GL (left half of character table).
///
/// See <https://terminalguide.namepad.de/seq/a_esc_so/> for terminal support
/// specifics and details.
#[esc(finalbyte = 'o')]
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct LockingShift3;

/// Locking Shift 1 Right (LS1R).
///
/// Invoke G1 character set into GR (right half of character table).
///
/// See <https://terminalguide.namepad.de/seq/a_esc_x7e_tilde/> for terminal
/// support specifics and details.
#[esc(finalbyte = '~')]
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct LockingShift1Right;

/// Locking Shift 2 Right (LS2R).
///
/// Invoke G2 character set into GR (right half of character table).
///
/// See <https://terminalguide.namepad.de/seq/a_esc_x7d_right_brace/> for
/// terminal support specifics and details.
#[esc(finalbyte = '}')]
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct LockingShift2Right;

/// Locking Shift 3 Right (LS3R).
///
/// Invoke G3 character set into GR (right half of character table).
///
/// See <https://terminalguide.namepad.de/seq/a_esc_x7c_pipe/> for terminal
/// support specifics and details.
#[esc(finalbyte = '|')]
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct LockingShift3Right;

/// Single Shift 2 (SS2).
///
/// Temporarily invoke G2 character set for the next character only.
///
/// See <https://terminalguide.namepad.de/seq/a_esc_cn/> for terminal
/// support specifics and details.
#[esc(finalbyte = 'N')]
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct SingleShift2;

/// Single Shift 3 (SS3).
///
/// Temporarily invoke G3 character set for the next character only.
///
/// See <https://terminalguide.namepad.de/seq/a_esc_co/> for terminal
/// support specifics and details.
#[esc(finalbyte = 'O')]
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct SingleShift3;

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

impl vtenc::IntoSeq for CharsetCode {
    fn into_seq(&self) -> impl WriteSeq {
        self.code()
    }
}

/// Designate G0 Character Set (94 characters).
///
/// Designate a 94-character set to the G0 character set register.
/// This is part of the ISO-2022 character set designation mechanism.
///
/// See <https://terminalguide.namepad.de/seq/> charset designation section
/// for terminal support specifics and details.
#[esc(intermediate = "(")]
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct DesignateG0 {
    pub charset: CharsetCode,
}

/// Designate G1 Character Set (94 characters).
///
/// Designate a 94-character set to the G1 character set register.
/// This is part of the ISO-2022 character set designation mechanism.
///
/// See <https://terminalguide.namepad.de/seq/> charset designation section
/// for terminal support specifics and details.
#[esc(intermediate = ")")]
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct DesignateG1 {
    pub charset: CharsetCode,
}

/// Designate G2 Character Set (94 characters).
///
/// Designate a 94-character set to the G2 character set register.
/// This is part of the ISO-2022 character set designation mechanism.
///
/// See <https://terminalguide.namepad.de/seq/> charset designation section
/// for terminal support specifics and details.
#[esc(intermediate = "*")]
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct DesignateG2 {
    pub charset: CharsetCode,
}

/// Designate G3 Character Set (94 characters).
///
/// Designate a 94-character set to the G3 character set register.
/// This is part of the ISO-2022 character set designation mechanism.
///
/// See <https://terminalguide.namepad.de/seq/> charset designation section
/// for terminal support specifics and details.
#[esc(intermediate = "+")]
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct DesignateG3 {
    pub charset: CharsetCode,
}

/// Designate G1 Character Set (96 characters).
///
/// Designate a 96-character set to the G1 character set register.
/// This is part of the ISO-2022 character set designation mechanism.
///
/// See <https://terminalguide.namepad.de/seq/> charset designation section
/// for terminal support specifics and details.
#[esc(intermediate = "-")]
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct DesignateG1_96 {
    pub charset: CharsetCode,
}

/// Designate G2 Character Set (96 characters).
///
/// Designate a 96-character set to the G2 character set register.
/// This is part of the ISO-2022 character set designation mechanism.
///
/// See <https://terminalguide.namepad.de/seq/> charset designation section
/// for terminal support specifics and details.
#[esc(intermediate = ".")]
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct DesignateG2_96 {
    pub charset: CharsetCode,
}

/// Designate G3 Character Set (96 characters).
///
/// Designate a 96-character set to the G3 character set register.
/// This is part of the ISO-2022 character set designation mechanism.
///
/// See <https://terminalguide.namepad.de/seq/> charset designation section
/// for terminal support specifics and details.
#[esc(intermediate = "/")]
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct DesignateG3_96 {
    pub charset: CharsetCode,
}

#[cfg(test)]
mod tests {
    use super::*;
    use vtenc::{ConstEncode, Encode};

    #[test]
    fn test_c0_control_codes() {
        assert_eq!(ShiftOut::STR, "\x0E");
        assert_eq!(ShiftIn::STR, "\x0F");
    }

    #[test]
    fn test_const_esc_sequences() {
        assert_eq!(EnableUTF8Mode::STR, "\x1B%G");
        assert_eq!(DisableUTF8Mode::STR, "\x1B%@");
        assert_eq!(LockingShift2::STR, "\x1Bn");
        assert_eq!(LockingShift3::STR, "\x1Bo");
        assert_eq!(SingleShift2::STR, "\x1BN");
        assert_eq!(SingleShift3::STR, "\x1BO");
        assert_eq!(LockingShift1Right::STR, "\x1B~");
        assert_eq!(LockingShift2Right::STR, "\x1B}");
        assert_eq!(LockingShift3Right::STR, "\x1B|");
    }

    #[test]
    fn test_variable_esc_sequences() {
        let mut buf = Vec::new();
        let mut msg = DesignateG0 {
            charset: CharsetCode::Ascii,
        };
        msg.encode(&mut buf).unwrap();
        assert_eq!(buf, b"\x1B(B");

        let mut buf = Vec::new();
        let mut msg = DesignateG1 {
            charset: CharsetCode::DecSpecialGraphic,
        };
        msg.encode(&mut buf).unwrap();
        assert_eq!(buf, b"\x1B)0");

        let mut buf = Vec::new();
        let mut msg = DesignateG0 {
            charset: CharsetCode::DecSuppGraphic,
        };
        msg.encode(&mut buf).unwrap();
        assert_eq!(buf, b"\x1B(%5");

        let mut buf = Vec::new();
        let mut msg = DesignateG2_96 {
            charset: CharsetCode::Latin1Supplemental,
        };
        msg.encode(&mut buf).unwrap();
        assert_eq!(buf, b"\x1B.A");
    }
}
