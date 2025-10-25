//! Integration tests demonstrating real-world usage patterns.

use vtenc::parse::{ParseError, TryFromAnsi};
use vtansi_derive::FromAnsi;

// Test that the macro works with various integer sizes
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromAnsi)]
#[repr(usize)]
enum UsizeEnum {
    First = 0,
    Second = 1,
}

impl TryFrom<usize> for UsizeEnum {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(UsizeEnum::First),
            1 => Ok(UsizeEnum::Second),
            _ => Err(()),
        }
    }
}

#[test]
fn test_usize_repr() {
    assert_eq!(UsizeEnum::try_from_ansi(b"0").unwrap(), UsizeEnum::First);
    assert_eq!(
        UsizeEnum::try_from_ansi(b"1").unwrap(),
        UsizeEnum::Second
    );
}

// Test multiple enums in the same module
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromAnsi)]
#[repr(u8)]
enum AnsiAttribute {
    Reset = 0,
    Bold = 1,
    Faint = 2,
}

impl TryFrom<u8> for AnsiAttribute {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(AnsiAttribute::Reset),
            1 => Ok(AnsiAttribute::Bold),
            2 => Ok(AnsiAttribute::Faint),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromAnsi)]
#[repr(u8)]
enum AnsiColor {
    Black = 30,
    Red = 31,
    Green = 32,
}

impl TryFrom<u8> for AnsiColor {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            30 => Ok(AnsiColor::Black),
            31 => Ok(AnsiColor::Red),
            32 => Ok(AnsiColor::Green),
            _ => Err(()),
        }
    }
}

#[test]
fn test_multiple_enums() {
    assert_eq!(
        AnsiAttribute::try_from_ansi(b"0").unwrap(),
        AnsiAttribute::Reset
    );
    assert_eq!(
        AnsiColor::try_from_ansi(b"31").unwrap(),
        AnsiColor::Red
    );
}

// Test string-based enum with multiple valid representations
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromAnsi)]
enum BooleanValue {
    True,
    False,
}

impl TryFrom<&str> for BooleanValue {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "true" | "yes" | "1" | "on" => Ok(BooleanValue::True),
            "false" | "no" | "0" | "off" => Ok(BooleanValue::False),
            _ => Err(format!("invalid boolean: {}", s)),
        }
    }
}

#[test]
fn test_flexible_string_parsing() {
    assert_eq!(
        BooleanValue::try_from_ansi(b"true").unwrap(),
        BooleanValue::True
    );
    assert_eq!(
        BooleanValue::try_from_ansi(b"yes").unwrap(),
        BooleanValue::True
    );
    assert_eq!(
        BooleanValue::try_from_ansi(b"1").unwrap(),
        BooleanValue::True
    );
    assert_eq!(
        BooleanValue::try_from_ansi(b"false").unwrap(),
        BooleanValue::False
    );
    assert_eq!(
        BooleanValue::try_from_ansi(b"no").unwrap(),
        BooleanValue::False
    );
    assert_eq!(
        BooleanValue::try_from_ansi(b"0").unwrap(),
        BooleanValue::False
    );
}

// Test that error messages are propagated correctly
#[test]
fn test_error_propagation() {
    let result = BooleanValue::try_from_ansi(b"maybe");
    assert!(result.is_err());
    match result {
        Err(ParseError::InvalidValue(msg)) => {
            assert!(msg.contains("maybe"));
        }
        _ => panic!("expected InvalidValue error"),
    }
}

// Test enum with non-contiguous discriminants
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromAnsi)]
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
fn test_sparse_discriminants() {
    assert_eq!(SparseEnum::try_from_ansi(b"1").unwrap(), SparseEnum::A);
    assert_eq!(SparseEnum::try_from_ansi(b"10").unwrap(), SparseEnum::B);
    assert_eq!(SparseEnum::try_from_ansi(b"100").unwrap(), SparseEnum::C);
    assert_eq!(
        SparseEnum::try_from_ansi(b"1000").unwrap(),
        SparseEnum::D
    );

    // Values between discriminants should fail
    assert!(SparseEnum::try_from_ansi(b"5").is_err());
    assert!(SparseEnum::try_from_ansi(b"50").is_err());
    assert!(SparseEnum::try_from_ansi(b"500").is_err());
}

// Test with signed integers
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromAnsi)]
#[repr(i16)]
enum SignedEnum {
    NegativeHundred = -100,
    Zero = 0,
    PositiveHundred = 100,
}

impl TryFrom<i16> for SignedEnum {
    type Error = ();

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        match value {
            -100 => Ok(SignedEnum::NegativeHundred),
            0 => Ok(SignedEnum::Zero),
            100 => Ok(SignedEnum::PositiveHundred),
            _ => Err(()),
        }
    }
}

#[test]
fn test_signed_enum() {
    assert_eq!(
        SignedEnum::try_from_ansi(b"-100").unwrap(),
        SignedEnum::NegativeHundred
    );
    assert_eq!(
        SignedEnum::try_from_ansi(b"0").unwrap(),
        SignedEnum::Zero
    );
    assert_eq!(
        SignedEnum::try_from_ansi(b"100").unwrap(),
        SignedEnum::PositiveHundred
    );
}

// Test batch parsing of multiple values
#[test]
fn test_batch_parsing() {
    let color_codes: &[&[u8]] = &[b"30", b"31", b"32"];

    let colors: Result<Vec<AnsiColor>, ParseError> = color_codes
        .iter()
        .map(|bytes| AnsiColor::try_from_ansi(bytes))
        .collect();

    let colors = colors.unwrap();
    assert_eq!(colors.len(), 3);
    assert_eq!(colors[0], AnsiColor::Black);
    assert_eq!(colors[1], AnsiColor::Red);
    assert_eq!(colors[2], AnsiColor::Green);
}

// Test that invalid UTF-8 is handled correctly for string enums
#[test]
fn test_invalid_utf8_string_enum() {
    let invalid_utf8 = &[0xFF, 0xFE, 0xFD];
    let result = BooleanValue::try_from_ansi(invalid_utf8);
    assert!(result.is_err());
    match result {
        Err(ParseError::InvalidString(_)) => {}
        _ => panic!("expected InvalidString error"),
    }
}