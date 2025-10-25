//! Tests for the #[vtansi(default)] attribute functionality.

use vtenc::parse::{ParseError, TryFromAnsi};
use vtansi_derive::FromAnsi;

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromAnsi)]
#[repr(u8)]
enum ColorWithDefault {
    Red = 0,
    Green = 1,
    Blue = 2,
    #[vtansi(default)]
    Unknown = 255,
}

impl TryFrom<u8> for ColorWithDefault {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ColorWithDefault::Red),
            1 => Ok(ColorWithDefault::Green),
            2 => Ok(ColorWithDefault::Blue),
            255 => Ok(ColorWithDefault::Unknown),
            _ => Err(()),
        }
    }
}

#[test]
fn test_repr_enum_valid_values() {
    assert_eq!(
        ColorWithDefault::try_from_ansi(b"0").unwrap(),
        ColorWithDefault::Red
    );
    assert_eq!(
        ColorWithDefault::try_from_ansi(b"1").unwrap(),
        ColorWithDefault::Green
    );
    assert_eq!(
        ColorWithDefault::try_from_ansi(b"2").unwrap(),
        ColorWithDefault::Blue
    );
}

#[test]
fn test_repr_enum_default_for_invalid() {
    // Invalid values should return the default variant
    assert_eq!(
        ColorWithDefault::try_from_ansi(b"3").unwrap(),
        ColorWithDefault::Unknown
    );
    assert_eq!(
        ColorWithDefault::try_from_ansi(b"99").unwrap(),
        ColorWithDefault::Unknown
    );
    assert_eq!(
        ColorWithDefault::try_from_ansi(b"254").unwrap(),
        ColorWithDefault::Unknown
    );
}

#[test]
fn test_repr_enum_explicit_default_value() {
    // The explicit default value (255) should still work
    assert_eq!(
        ColorWithDefault::try_from_ansi(b"255").unwrap(),
        ColorWithDefault::Unknown
    );
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromAnsi)]
enum ModeWithDefault {
    Normal,
    Insert,
    Visual,
    #[vtansi(default)]
    Unknown,
}

impl TryFrom<&str> for ModeWithDefault {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "normal" => Ok(ModeWithDefault::Normal),
            "insert" => Ok(ModeWithDefault::Insert),
            "visual" => Ok(ModeWithDefault::Visual),
            _ => Err(format!("unknown mode: {}", s)),
        }
    }
}

#[test]
fn test_string_enum_valid_values() {
    assert_eq!(
        ModeWithDefault::try_from_ansi(b"normal").unwrap(),
        ModeWithDefault::Normal
    );
    assert_eq!(
        ModeWithDefault::try_from_ansi(b"insert").unwrap(),
        ModeWithDefault::Insert
    );
    assert_eq!(
        ModeWithDefault::try_from_ansi(b"visual").unwrap(),
        ModeWithDefault::Visual
    );
}

#[test]
fn test_string_enum_default_for_invalid() {
    // Invalid values should return the default variant
    assert_eq!(
        ModeWithDefault::try_from_ansi(b"command").unwrap(),
        ModeWithDefault::Unknown
    );
    assert_eq!(
        ModeWithDefault::try_from_ansi(b"replace").unwrap(),
        ModeWithDefault::Unknown
    );
    assert_eq!(
        ModeWithDefault::try_from_ansi(b"invalid").unwrap(),
        ModeWithDefault::Unknown
    );
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromAnsi)]
#[repr(i8)]
enum SignedWithDefault {
    NegativeOne = -1,
    Zero = 0,
    PositiveOne = 1,
    #[vtansi(default)]
    Other = 127,
}

impl TryFrom<i8> for SignedWithDefault {
    type Error = ();

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        match value {
            -1 => Ok(SignedWithDefault::NegativeOne),
            0 => Ok(SignedWithDefault::Zero),
            1 => Ok(SignedWithDefault::PositiveOne),
            127 => Ok(SignedWithDefault::Other),
            _ => Err(()),
        }
    }
}

#[test]
fn test_signed_enum_default() {
    assert_eq!(
        SignedWithDefault::try_from_ansi(b"-1").unwrap(),
        SignedWithDefault::NegativeOne
    );
    assert_eq!(
        SignedWithDefault::try_from_ansi(b"0").unwrap(),
        SignedWithDefault::Zero
    );
    assert_eq!(
        SignedWithDefault::try_from_ansi(b"1").unwrap(),
        SignedWithDefault::PositiveOne
    );

    // Invalid values get the default
    assert_eq!(
        SignedWithDefault::try_from_ansi(b"2").unwrap(),
        SignedWithDefault::Other
    );
    assert_eq!(
        SignedWithDefault::try_from_ansi(b"-2").unwrap(),
        SignedWithDefault::Other
    );
    assert_eq!(
        SignedWithDefault::try_from_ansi(b"100").unwrap(),
        SignedWithDefault::Other
    );
}

// Test that enums without default still produce errors
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromAnsi)]
#[repr(u8)]
enum ColorNoDefault {
    Red = 0,
    Green = 1,
    Blue = 2,
}

impl TryFrom<u8> for ColorNoDefault {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ColorNoDefault::Red),
            1 => Ok(ColorNoDefault::Green),
            2 => Ok(ColorNoDefault::Blue),
            _ => Err(()),
        }
    }
}

#[test]
fn test_no_default_produces_error() {
    assert_eq!(
        ColorNoDefault::try_from_ansi(b"0").unwrap(),
        ColorNoDefault::Red
    );

    // Without a default, invalid values should error
    let result = ColorNoDefault::try_from_ansi(b"3");
    assert!(result.is_err());
    match result {
        Err(ParseError::InvalidValue(_)) => {}
        _ => panic!("expected InvalidValue error"),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromAnsi)]
#[repr(u16)]
enum LargeEnumWithDefault {
    Small = 1,
    Medium = 100,
    Large = 1000,
    #[vtansi(default)]
    Unknown = 65535,
}

impl TryFrom<u16> for LargeEnumWithDefault {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(LargeEnumWithDefault::Small),
            100 => Ok(LargeEnumWithDefault::Medium),
            1000 => Ok(LargeEnumWithDefault::Large),
            65535 => Ok(LargeEnumWithDefault::Unknown),
            _ => Err(()),
        }
    }
}

#[test]
fn test_large_enum_default() {
    assert_eq!(
        LargeEnumWithDefault::try_from_ansi(b"1").unwrap(),
        LargeEnumWithDefault::Small
    );
    assert_eq!(
        LargeEnumWithDefault::try_from_ansi(b"100").unwrap(),
        LargeEnumWithDefault::Medium
    );
    assert_eq!(
        LargeEnumWithDefault::try_from_ansi(b"1000").unwrap(),
        LargeEnumWithDefault::Large
    );

    // Any invalid value becomes Unknown
    assert_eq!(
        LargeEnumWithDefault::try_from_ansi(b"2").unwrap(),
        LargeEnumWithDefault::Unknown
    );
    assert_eq!(
        LargeEnumWithDefault::try_from_ansi(b"500").unwrap(),
        LargeEnumWithDefault::Unknown
    );
    assert_eq!(
        LargeEnumWithDefault::try_from_ansi(b"65534").unwrap(),
        LargeEnumWithDefault::Unknown
    );
}

#[test]
fn test_parse_errors_still_propagate() {
    // Parse errors (non-numeric input) should still error out
    let result = ColorWithDefault::try_from_ansi(b"not-a-number");
    assert!(result.is_err());
    match result {
        Err(ParseError::InvalidNum(_)) => {}
        _ => panic!("expected InvalidNum error"),
    }

    // Empty input should still error
    let result = ColorWithDefault::try_from_ansi(b"");
    assert!(result.is_err());

    // Invalid UTF-8 for string enums should still error
    let result = ModeWithDefault::try_from_ansi(&[0xFF, 0xFE]);
    assert!(result.is_err());
    match result {
        Err(ParseError::InvalidString(_)) => {}
        _ => panic!("expected InvalidString error"),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromAnsi)]
enum MultipleAliasesWithDefault {
    Active,
    Inactive,
    #[vtansi(default)]
    Unknown,
}

impl TryFrom<&str> for MultipleAliasesWithDefault {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "active" | "on" | "enabled" | "true" => {
                Ok(MultipleAliasesWithDefault::Active)
            }
            "inactive" | "off" | "disabled" | "false" => {
                Ok(MultipleAliasesWithDefault::Inactive)
            }
            _ => Err(format!("unknown state: {}", s)),
        }
    }
}

#[test]
fn test_aliases_with_default() {
    // Test all valid aliases
    assert_eq!(
        MultipleAliasesWithDefault::try_from_ansi(b"active").unwrap(),
        MultipleAliasesWithDefault::Active
    );
    assert_eq!(
        MultipleAliasesWithDefault::try_from_ansi(b"on").unwrap(),
        MultipleAliasesWithDefault::Active
    );
    assert_eq!(
        MultipleAliasesWithDefault::try_from_ansi(b"enabled").unwrap(),
        MultipleAliasesWithDefault::Active
    );

    assert_eq!(
        MultipleAliasesWithDefault::try_from_ansi(b"inactive").unwrap(),
        MultipleAliasesWithDefault::Inactive
    );
    assert_eq!(
        MultipleAliasesWithDefault::try_from_ansi(b"off").unwrap(),
        MultipleAliasesWithDefault::Inactive
    );

    // Unknown values fall back to default
    assert_eq!(
        MultipleAliasesWithDefault::try_from_ansi(b"maybe").unwrap(),
        MultipleAliasesWithDefault::Unknown
    );
    assert_eq!(
        MultipleAliasesWithDefault::try_from_ansi(b"pending").unwrap(),
        MultipleAliasesWithDefault::Unknown
    );
}