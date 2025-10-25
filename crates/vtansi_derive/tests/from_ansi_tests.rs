//! Tests for the FromAnsi derive macro.

use vtansi_derive::FromAnsi;
use vtenc::parse::{ParseError, TryFromAnsi};

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromAnsi)]
#[repr(u8)]
enum Color {
    Red = 0,
    Green = 1,
    Blue = 2,
}

impl TryFrom<u8> for Color {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Color::Red),
            1 => Ok(Color::Green),
            2 => Ok(Color::Blue),
            _ => Err(()),
        }
    }
}

#[test]
fn test_repr_u8_enum() {
    assert_eq!(Color::try_from_ansi(b"0").unwrap(), Color::Red);
    assert_eq!(Color::try_from_ansi(b"1").unwrap(), Color::Green);
    assert_eq!(Color::try_from_ansi(b"2").unwrap(), Color::Blue);
}

#[test]
fn test_repr_u8_enum_invalid() {
    let result = Color::try_from_ansi(b"3");
    assert!(result.is_err());
    match result {
        Err(ParseError::InvalidValue(_)) => {}
        _ => panic!("expected InvalidValue error"),
    }
}

#[test]
fn test_repr_u8_enum_invalid_num() {
    let result = Color::try_from_ansi(b"abc");
    assert!(result.is_err());
    match result {
        Err(ParseError::InvalidNum(_)) => {}
        _ => panic!("expected InvalidNum error"),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromAnsi)]
#[repr(u16)]
enum Mode {
    Normal = 100,
    Insert = 200,
    Visual = 300,
}

impl TryFrom<u16> for Mode {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            100 => Ok(Mode::Normal),
            200 => Ok(Mode::Insert),
            300 => Ok(Mode::Visual),
            _ => Err(()),
        }
    }
}

#[test]
fn test_repr_u16_enum() {
    assert_eq!(Mode::try_from_ansi(b"100").unwrap(), Mode::Normal);
    assert_eq!(Mode::try_from_ansi(b"200").unwrap(), Mode::Insert);
    assert_eq!(Mode::try_from_ansi(b"300").unwrap(), Mode::Visual);
}

#[test]
fn test_repr_u16_enum_invalid() {
    let result = Mode::try_from_ansi(b"400");
    assert!(result.is_err());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromAnsi)]
enum TextStyle {
    Plain,
    Bold,
    Italic,
}

impl TryFrom<&str> for TextStyle {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "plain" => Ok(TextStyle::Plain),
            "bold" => Ok(TextStyle::Bold),
            "italic" => Ok(TextStyle::Italic),
            _ => Err(format!("unknown text style: {}", s)),
        }
    }
}

#[test]
fn test_str_enum() {
    assert_eq!(
        TextStyle::try_from_ansi(b"plain").unwrap(),
        TextStyle::Plain
    );
    assert_eq!(TextStyle::try_from_ansi(b"bold").unwrap(), TextStyle::Bold);
    assert_eq!(
        TextStyle::try_from_ansi(b"italic").unwrap(),
        TextStyle::Italic
    );
}

#[test]
fn test_str_enum_invalid() {
    let result = TextStyle::try_from_ansi(b"underline");
    assert!(result.is_err());
    match result {
        Err(ParseError::InvalidValue(_)) => {}
        _ => panic!("expected InvalidValue error"),
    }
}

#[test]
fn test_str_enum_invalid_utf8() {
    let result = TextStyle::try_from_ansi(&[0xFF, 0xFE]);
    assert!(result.is_err());
    match result {
        Err(ParseError::InvalidString(_)) => {}
        _ => panic!("expected InvalidString error"),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromAnsi)]
#[repr(i8)]
enum Direction {
    Up = -1,
    Down = 1,
}

impl TryFrom<i8> for Direction {
    type Error = ();

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        match value {
            -1 => Ok(Direction::Up),
            1 => Ok(Direction::Down),
            _ => Err(()),
        }
    }
}

#[test]
fn test_repr_i8_enum() {
    assert_eq!(Direction::try_from_ansi(b"-1").unwrap(), Direction::Up);
    assert_eq!(Direction::try_from_ansi(b"1").unwrap(), Direction::Down);
}

#[test]
fn test_repr_i8_enum_invalid() {
    let result = Direction::try_from_ansi(b"0");
    assert!(result.is_err());
}

#[test]
fn test_empty_bytes() {
    let result = Color::try_from_ansi(b"");
    assert!(result.is_err());
    match result {
        Err(ParseError::InvalidNum(_)) => {}
        _ => panic!("expected InvalidNum error"),
    }
}

#[test]
fn test_whitespace_handling() {
    // Whitespace should cause parse error for numbers
    let result = Color::try_from_ansi(b" 0");
    assert!(result.is_err());

    let result = Color::try_from_ansi(b"0 ");
    assert!(result.is_err());
}

#[test]
fn test_negative_for_unsigned() {
    let result = Color::try_from_ansi(b"-1");
    assert!(result.is_err());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromAnsi)]
#[repr(u32)]
enum LargeEnum {
    Value1 = 1000000,
    Value2 = 2000000,
}

impl TryFrom<u32> for LargeEnum {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1000000 => Ok(LargeEnum::Value1),
            2000000 => Ok(LargeEnum::Value2),
            _ => Err(()),
        }
    }
}

#[test]
fn test_large_values() {
    assert_eq!(
        LargeEnum::try_from_ansi(b"1000000").unwrap(),
        LargeEnum::Value1
    );
    assert_eq!(
        LargeEnum::try_from_ansi(b"2000000").unwrap(),
        LargeEnum::Value2
    );
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromAnsi)]
enum CaseSensitiveEnum {
    Lower,
    Upper,
}

impl TryFrom<&str> for CaseSensitiveEnum {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "lower" => Ok(CaseSensitiveEnum::Lower),
            "UPPER" => Ok(CaseSensitiveEnum::Upper),
            _ => Err(format!("unknown value: {}", s)),
        }
    }
}

#[test]
fn test_case_sensitivity() {
    assert_eq!(
        CaseSensitiveEnum::try_from_ansi(b"lower").unwrap(),
        CaseSensitiveEnum::Lower
    );
    assert_eq!(
        CaseSensitiveEnum::try_from_ansi(b"UPPER").unwrap(),
        CaseSensitiveEnum::Upper
    );

    // Wrong case should fail
    assert!(CaseSensitiveEnum::try_from_ansi(b"LOWER").is_err());
    assert!(CaseSensitiveEnum::try_from_ansi(b"upper").is_err());
}

#[test]
fn test_unicode_string_enum() {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, FromAnsi)]
    enum UnicodeEnum {
        Hello,
        World,
    }

    impl TryFrom<&str> for UnicodeEnum {
        type Error = String;

        fn try_from(s: &str) -> Result<Self, Self::Error> {
            match s {
                "hello" => Ok(UnicodeEnum::Hello),
                "世界" => Ok(UnicodeEnum::World),
                _ => Err(format!("unknown: {}", s)),
            }
        }
    }

    assert_eq!(
        UnicodeEnum::try_from_ansi(b"hello").unwrap(),
        UnicodeEnum::Hello
    );
    assert_eq!(
        UnicodeEnum::try_from_ansi("世界".as_bytes()).unwrap(),
        UnicodeEnum::World
    );
}
