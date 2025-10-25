//! Tests for the ToAnsi derive macro.

use vtansi_derive::ToAnsi;
use vtenc::encode::AnsiEncode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ToAnsi)]
#[repr(u8)]
enum Color {
    Red = 0,
    Green = 1,
    Blue = 2,
}

#[test]
fn test_repr_u8_enum() {
    let mut buf = Vec::new();
    Color::Red.encode_ansi_into(&mut buf).unwrap();
    assert_eq!(buf, b"0");

    let mut buf = Vec::new();
    Color::Green.encode_ansi_into(&mut buf).unwrap();
    assert_eq!(buf, b"1");

    let mut buf = Vec::new();
    Color::Blue.encode_ansi_into(&mut buf).unwrap();
    assert_eq!(buf, b"2");
}

#[test]
fn test_repr_u8_enum_to_vec() {
    assert_eq!(Color::Red.encode_ansi().unwrap(), b"0");
    assert_eq!(Color::Green.encode_ansi().unwrap(), b"1");
    assert_eq!(Color::Blue.encode_ansi().unwrap(), b"2");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ToAnsi)]
#[repr(u16)]
enum Mode {
    Normal = 100,
    Insert = 200,
    Visual = 300,
}

#[test]
fn test_repr_u16_enum() {
    let mut buf = Vec::new();
    Mode::Normal.encode_ansi_into(&mut buf).unwrap();
    assert_eq!(buf, b"100");

    let mut buf = Vec::new();
    Mode::Insert.encode_ansi_into(&mut buf).unwrap();
    assert_eq!(buf, b"200");

    let mut buf = Vec::new();
    Mode::Visual.encode_ansi_into(&mut buf).unwrap();
    assert_eq!(buf, b"300");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ToAnsi)]
enum TextStyle {
    Plain,
    Bold,
    Italic,
}

impl AsRef<str> for TextStyle {
    fn as_ref(&self) -> &str {
        match self {
            TextStyle::Plain => "plain",
            TextStyle::Bold => "bold",
            TextStyle::Italic => "italic",
        }
    }
}

#[test]
fn test_str_enum() {
    let mut buf = Vec::new();
    TextStyle::Plain.encode_ansi_into(&mut buf).unwrap();
    assert_eq!(buf, b"plain");

    let mut buf = Vec::new();
    TextStyle::Bold.encode_ansi_into(&mut buf).unwrap();
    assert_eq!(buf, b"bold");

    let mut buf = Vec::new();
    TextStyle::Italic.encode_ansi_into(&mut buf).unwrap();
    assert_eq!(buf, b"italic");
}

#[test]
fn test_str_enum_to_vec() {
    assert_eq!(TextStyle::Plain.encode_ansi().unwrap(), b"plain");
    assert_eq!(TextStyle::Bold.encode_ansi().unwrap(), b"bold");
    assert_eq!(TextStyle::Italic.encode_ansi().unwrap(), b"italic");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ToAnsi)]
#[repr(i8)]
enum Direction {
    Up = -1,
    Down = 1,
}

#[test]
fn test_repr_i8_enum() {
    let mut buf = Vec::new();
    Direction::Up.encode_ansi_into(&mut buf).unwrap();
    assert_eq!(buf, b"-1");

    let mut buf = Vec::new();
    Direction::Down.encode_ansi_into(&mut buf).unwrap();
    assert_eq!(buf, b"1");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ToAnsi)]
#[repr(u32)]
enum LargeEnum {
    Value1 = 1000000,
    Value2 = 2000000,
}

#[test]
fn test_large_values() {
    assert_eq!(LargeEnum::Value1.encode_ansi().unwrap(), b"1000000");
    assert_eq!(LargeEnum::Value2.encode_ansi().unwrap(), b"2000000");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ToAnsi)]
#[repr(usize)]
enum UsizeEnum {
    First = 0,
    Second = 1,
}

#[test]
fn test_usize_repr() {
    assert_eq!(UsizeEnum::First.encode_ansi().unwrap(), b"0");
    assert_eq!(UsizeEnum::Second.encode_ansi().unwrap(), b"1");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ToAnsi)]
enum CaseSensitiveEnum {
    Lower,
    Upper,
}

impl AsRef<str> for CaseSensitiveEnum {
    fn as_ref(&self) -> &str {
        match self {
            CaseSensitiveEnum::Lower => "lower",
            CaseSensitiveEnum::Upper => "UPPER",
        }
    }
}

#[test]
fn test_case_sensitivity() {
    assert_eq!(CaseSensitiveEnum::Lower.encode_ansi().unwrap(), b"lower");
    assert_eq!(CaseSensitiveEnum::Upper.encode_ansi().unwrap(), b"UPPER");
}

#[test]
fn test_unicode_string_enum() {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, ToAnsi)]
    enum UnicodeEnum {
        Hello,
        World,
    }

    impl AsRef<str> for UnicodeEnum {
        fn as_ref(&self) -> &str {
            match self {
                UnicodeEnum::Hello => "hello",
                UnicodeEnum::World => "世界",
            }
        }
    }

    assert_eq!(UnicodeEnum::Hello.encode_ansi().unwrap(), b"hello");
    assert_eq!(UnicodeEnum::World.encode_ansi().unwrap(), "世界".as_bytes());
}

#[test]
fn test_encode_to_slice() {
    let mut buf = [0u8; 10];
    let n = Color::Red.encode_ansi_into_slice(&mut buf).unwrap();
    assert_eq!(n, 1);
    assert_eq!(&buf[..n], b"0");

    let mut buf = [0u8; 10];
    let n = TextStyle::Bold.encode_ansi_into_slice(&mut buf).unwrap();
    assert_eq!(n, 4);
    assert_eq!(&buf[..n], b"bold");
}

#[test]
fn test_batch_encoding() {
    let colors = [Color::Red, Color::Green, Color::Blue];
    let mut buf = Vec::new();

    for color in &colors {
        color.encode_ansi_into(&mut buf).unwrap();
        buf.push(b';');
    }
    buf.pop(); // Remove trailing semicolon

    assert_eq!(buf, b"0;1;2");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ToAnsi)]
#[repr(i16)]
enum SignedEnum {
    NegativeHundred = -100,
    Zero = 0,
    PositiveHundred = 100,
}

#[test]
fn test_signed_enum() {
    assert_eq!(SignedEnum::NegativeHundred.encode_ansi().unwrap(), b"-100");
    assert_eq!(SignedEnum::Zero.encode_ansi().unwrap(), b"0");
    assert_eq!(SignedEnum::PositiveHundred.encode_ansi().unwrap(), b"100");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ToAnsi)]
#[repr(u32)]
enum SparseEnum {
    A = 1,
    B = 10,
    C = 100,
    D = 1000,
}

#[test]
fn test_sparse_discriminants() {
    assert_eq!(SparseEnum::A.encode_ansi().unwrap(), b"1");
    assert_eq!(SparseEnum::B.encode_ansi().unwrap(), b"10");
    assert_eq!(SparseEnum::C.encode_ansi().unwrap(), b"100");
    assert_eq!(SparseEnum::D.encode_ansi().unwrap(), b"1000");
}
