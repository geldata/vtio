//! Test that the derive feature properly re-exports FromAnsi macro.

#![cfg(feature = "derive")]

use vtenc::encode::AnsiEncode;
use vtenc::parse::TryFromAnsi;
use vtenc::{FromAnsi, ToAnsi};

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromAnsi, ToAnsi)]
#[repr(u8)]
enum TestColor {
    Red = 0,
    Green = 1,
    Blue = 2,
}

impl TryFrom<u8> for TestColor {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(TestColor::Red),
            1 => Ok(TestColor::Green),
            2 => Ok(TestColor::Blue),
            _ => Err(()),
        }
    }
}

#[test]
fn test_derive_reexport() {
    assert_eq!(TestColor::try_from_ansi(b"0").unwrap(), TestColor::Red);
    assert_eq!(TestColor::try_from_ansi(b"1").unwrap(), TestColor::Green);
    assert_eq!(TestColor::try_from_ansi(b"2").unwrap(), TestColor::Blue);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromAnsi, ToAnsi)]
enum TestMode {
    Normal,
    Insert,
}

impl TryFrom<&str> for TestMode {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "normal" => Ok(TestMode::Normal),
            "insert" => Ok(TestMode::Insert),
            _ => Err(format!("unknown mode: {}", s)),
        }
    }
}

impl AsRef<str> for TestMode {
    fn as_ref(&self) -> &str {
        match self {
            TestMode::Normal => "normal",
            TestMode::Insert => "insert",
        }
    }
}

#[test]
fn test_string_based_derive() {
    assert_eq!(
        TestMode::try_from_ansi(b"normal").unwrap(),
        TestMode::Normal
    );
    assert_eq!(
        TestMode::try_from_ansi(b"insert").unwrap(),
        TestMode::Insert
    );
}

#[test]
fn test_to_ansi_repr_enum() {
    assert_eq!(TestColor::Red.encode_ansi().unwrap(), b"0");
    assert_eq!(TestColor::Green.encode_ansi().unwrap(), b"1");
    assert_eq!(TestColor::Blue.encode_ansi().unwrap(), b"2");
}

#[test]
fn test_to_ansi_string_enum() {
    assert_eq!(TestMode::Normal.encode_ansi().unwrap(), b"normal");
    assert_eq!(TestMode::Insert.encode_ansi().unwrap(), b"insert");
}

#[test]
fn test_roundtrip() {
    // Test repr enum roundtrip
    let color = TestColor::Green;
    let encoded = color.encode_ansi().unwrap();
    let decoded = TestColor::try_from_ansi(&encoded).unwrap();
    assert_eq!(color, decoded);

    // Test string enum roundtrip
    let mode = TestMode::Insert;
    let encoded = mode.encode_ansi().unwrap();
    let decoded = TestMode::try_from_ansi(&encoded).unwrap();
    assert_eq!(mode, decoded);
}
