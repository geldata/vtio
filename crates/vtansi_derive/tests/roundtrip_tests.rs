//! Roundtrip tests verifying FromAnsi and ToAnsi work together correctly.

use vtenc::encode::AnsiEncode;
use vtenc::parse::TryFromAnsi;
use vtansi_derive::{FromAnsi, ToAnsi};

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromAnsi, ToAnsi)]
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
fn test_repr_u8_roundtrip() {
    // Encode then decode
    let original = Color::Red;
    let encoded = original.encode_ansi().unwrap();
    let decoded = Color::try_from_ansi(&encoded).unwrap();
    assert_eq!(original, decoded);

    let original = Color::Green;
    let encoded = original.encode_ansi().unwrap();
    let decoded = Color::try_from_ansi(&encoded).unwrap();
    assert_eq!(original, decoded);

    let original = Color::Blue;
    let encoded = original.encode_ansi().unwrap();
    let decoded = Color::try_from_ansi(&encoded).unwrap();
    assert_eq!(original, decoded);
}

#[test]
fn test_repr_u8_decode_then_encode() {
    // Decode then encode
    let bytes = b"0";
    let decoded = Color::try_from_ansi(bytes).unwrap();
    let encoded = decoded.encode_ansi().unwrap();
    assert_eq!(bytes, encoded.as_slice());

    let bytes = b"1";
    let decoded = Color::try_from_ansi(bytes).unwrap();
    let encoded = decoded.encode_ansi().unwrap();
    assert_eq!(bytes, encoded.as_slice());

    let bytes = b"2";
    let decoded = Color::try_from_ansi(bytes).unwrap();
    let encoded = decoded.encode_ansi().unwrap();
    assert_eq!(bytes, encoded.as_slice());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromAnsi, ToAnsi)]
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
fn test_repr_u16_roundtrip() {
    let original = Mode::Normal;
    let encoded = original.encode_ansi().unwrap();
    let decoded = Mode::try_from_ansi(&encoded).unwrap();
    assert_eq!(original, decoded);

    let original = Mode::Insert;
    let encoded = original.encode_ansi().unwrap();
    let decoded = Mode::try_from_ansi(&encoded).unwrap();
    assert_eq!(original, decoded);

    let original = Mode::Visual;
    let encoded = original.encode_ansi().unwrap();
    let decoded = Mode::try_from_ansi(&encoded).unwrap();
    assert_eq!(original, decoded);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromAnsi, ToAnsi)]
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
fn test_signed_repr_roundtrip() {
    let original = Direction::Up;
    let encoded = original.encode_ansi().unwrap();
    let decoded = Direction::try_from_ansi(&encoded).unwrap();
    assert_eq!(original, decoded);

    let original = Direction::Down;
    let encoded = original.encode_ansi().unwrap();
    let decoded = Direction::try_from_ansi(&encoded).unwrap();
    assert_eq!(original, decoded);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromAnsi, ToAnsi)]
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
fn test_string_enum_roundtrip() {
    let original = TextStyle::Plain;
    let encoded = original.encode_ansi().unwrap();
    let decoded = TextStyle::try_from_ansi(&encoded).unwrap();
    assert_eq!(original, decoded);

    let original = TextStyle::Bold;
    let encoded = original.encode_ansi().unwrap();
    let decoded = TextStyle::try_from_ansi(&encoded).unwrap();
    assert_eq!(original, decoded);

    let original = TextStyle::Italic;
    let encoded = original.encode_ansi().unwrap();
    let decoded = TextStyle::try_from_ansi(&encoded).unwrap();
    assert_eq!(original, decoded);
}

#[test]
fn test_string_enum_decode_then_encode() {
    let bytes = b"plain";
    let decoded = TextStyle::try_from_ansi(bytes).unwrap();
    let encoded = decoded.encode_ansi().unwrap();
    assert_eq!(bytes, encoded.as_slice());

    let bytes = b"bold";
    let decoded = TextStyle::try_from_ansi(bytes).unwrap();
    let encoded = decoded.encode_ansi().unwrap();
    assert_eq!(bytes, encoded.as_slice());

    let bytes = b"italic";
    let decoded = TextStyle::try_from_ansi(bytes).unwrap();
    let encoded = decoded.encode_ansi().unwrap();
    assert_eq!(bytes, encoded.as_slice());
}

#[test]
fn test_batch_roundtrip() {
    let colors = [Color::Red, Color::Green, Color::Blue];

    for color in &colors {
        let encoded = color.encode_ansi().unwrap();
        let decoded = Color::try_from_ansi(&encoded).unwrap();
        assert_eq!(*color, decoded);
    }

    let styles = [TextStyle::Plain, TextStyle::Bold, TextStyle::Italic];

    for style in &styles {
        let encoded = style.encode_ansi().unwrap();
        let decoded = TextStyle::try_from_ansi(&encoded).unwrap();
        assert_eq!(*style, decoded);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromAnsi, ToAnsi)]
#[repr(u32)]
enum SparseEnum {
    A = 1,
    B = 10,
    C = 100,
    D = 1000,
}

impl TryFrom<u32> for SparseEnum {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(SparseEnum::A),
            10 => Ok(SparseEnum::B),
            100 => Ok(SparseEnum::C),
            1000 => Ok(SparseEnum::D),
            _ => Err(()),
        }
    }
}

#[test]
fn test_sparse_enum_roundtrip() {
    let values = [
        SparseEnum::A,
        SparseEnum::B,
        SparseEnum::C,
        SparseEnum::D,
    ];

    for value in &values {
        let encoded = value.encode_ansi().unwrap();
        let decoded = SparseEnum::try_from_ansi(&encoded).unwrap();
        assert_eq!(*value, decoded);
    }
}