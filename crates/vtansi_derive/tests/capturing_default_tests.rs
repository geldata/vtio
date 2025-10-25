//! Tests for capturing default variants with #[vtansi(default)].

use vtansi_derive::FromAnsi;
use vtenc::parse::TryFromAnsi;

#[derive(Debug, Clone, PartialEq, Eq, FromAnsi)]
#[repr(u8)]
enum ColorWithCapture {
    Red = 0,
    Green = 1,
    Blue = 2,
    #[vtansi(default)]
    Other(u8),
}

impl TryFrom<u8> for ColorWithCapture {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ColorWithCapture::Red),
            1 => Ok(ColorWithCapture::Green),
            2 => Ok(ColorWithCapture::Blue),
            _ => Err(()),
        }
    }
}

#[test]
fn test_repr_enum_capturing_valid_values() {
    assert_eq!(
        ColorWithCapture::try_from_ansi(b"0").unwrap(),
        ColorWithCapture::Red
    );
    assert_eq!(
        ColorWithCapture::try_from_ansi(b"1").unwrap(),
        ColorWithCapture::Green
    );
    assert_eq!(
        ColorWithCapture::try_from_ansi(b"2").unwrap(),
        ColorWithCapture::Blue
    );
}

#[test]
fn test_repr_enum_capturing_invalid_values() {
    // Invalid values should be captured in the default variant
    assert_eq!(
        ColorWithCapture::try_from_ansi(b"3").unwrap(),
        ColorWithCapture::Other(3)
    );
    assert_eq!(
        ColorWithCapture::try_from_ansi(b"99").unwrap(),
        ColorWithCapture::Other(99)
    );
    assert_eq!(
        ColorWithCapture::try_from_ansi(b"255").unwrap(),
        ColorWithCapture::Other(255)
    );
}

#[derive(Debug, Clone, PartialEq, Eq, FromAnsi)]
enum ModeWithCapture {
    Normal,
    Insert,
    Visual,
    #[vtansi(default)]
    Custom(String),
}

impl TryFrom<&str> for ModeWithCapture {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "normal" => Ok(ModeWithCapture::Normal),
            "insert" => Ok(ModeWithCapture::Insert),
            "visual" => Ok(ModeWithCapture::Visual),
            _ => Err(format!("unknown mode: {}", s)),
        }
    }
}

#[test]
fn test_string_enum_capturing_valid_values() {
    assert_eq!(
        ModeWithCapture::try_from_ansi(b"normal").unwrap(),
        ModeWithCapture::Normal
    );
    assert_eq!(
        ModeWithCapture::try_from_ansi(b"insert").unwrap(),
        ModeWithCapture::Insert
    );
    assert_eq!(
        ModeWithCapture::try_from_ansi(b"visual").unwrap(),
        ModeWithCapture::Visual
    );
}

#[test]
fn test_string_enum_capturing_invalid_values() {
    // Invalid values should be captured in the default variant
    assert_eq!(
        ModeWithCapture::try_from_ansi(b"command").unwrap(),
        ModeWithCapture::Custom("command".to_string())
    );
    assert_eq!(
        ModeWithCapture::try_from_ansi(b"replace").unwrap(),
        ModeWithCapture::Custom("replace".to_string())
    );
    assert_eq!(
        ModeWithCapture::try_from_ansi(b"anything").unwrap(),
        ModeWithCapture::Custom("anything".to_string())
    );
}

#[derive(Debug, Clone, PartialEq, Eq, FromAnsi)]
#[repr(u16)]
enum StatusCode {
    Ok = 200,
    NotFound = 404,
    ServerError = 500,
    #[vtansi(default)]
    Unknown(u16),
}

impl TryFrom<u16> for StatusCode {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            200 => Ok(StatusCode::Ok),
            404 => Ok(StatusCode::NotFound),
            500 => Ok(StatusCode::ServerError),
            _ => Err(()),
        }
    }
}

#[test]
fn test_u16_capturing_default() {
    assert_eq!(StatusCode::try_from_ansi(b"200").unwrap(), StatusCode::Ok);
    assert_eq!(
        StatusCode::try_from_ansi(b"404").unwrap(),
        StatusCode::NotFound
    );
    assert_eq!(
        StatusCode::try_from_ansi(b"500").unwrap(),
        StatusCode::ServerError
    );

    // Unknown codes are captured
    assert_eq!(
        StatusCode::try_from_ansi(b"201").unwrap(),
        StatusCode::Unknown(201)
    );
    assert_eq!(
        StatusCode::try_from_ansi(b"403").unwrap(),
        StatusCode::Unknown(403)
    );
    assert_eq!(
        StatusCode::try_from_ansi(b"999").unwrap(),
        StatusCode::Unknown(999)
    );
}

#[derive(Debug, Clone, PartialEq, Eq, FromAnsi)]
#[repr(i8)]
enum SignedWithCapture {
    Negative = -1,
    Zero = 0,
    Positive = 1,
    #[vtansi(default)]
    Other(i8),
}

impl TryFrom<i8> for SignedWithCapture {
    type Error = ();

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        match value {
            -1 => Ok(SignedWithCapture::Negative),
            0 => Ok(SignedWithCapture::Zero),
            1 => Ok(SignedWithCapture::Positive),
            _ => Err(()),
        }
    }
}

#[test]
fn test_signed_capturing_default() {
    assert_eq!(
        SignedWithCapture::try_from_ansi(b"-1").unwrap(),
        SignedWithCapture::Negative
    );
    assert_eq!(
        SignedWithCapture::try_from_ansi(b"0").unwrap(),
        SignedWithCapture::Zero
    );
    assert_eq!(
        SignedWithCapture::try_from_ansi(b"1").unwrap(),
        SignedWithCapture::Positive
    );

    // Other values are captured
    assert_eq!(
        SignedWithCapture::try_from_ansi(b"2").unwrap(),
        SignedWithCapture::Other(2)
    );
    assert_eq!(
        SignedWithCapture::try_from_ansi(b"-10").unwrap(),
        SignedWithCapture::Other(-10)
    );
    assert_eq!(
        SignedWithCapture::try_from_ansi(b"127").unwrap(),
        SignedWithCapture::Other(127)
    );
}

#[test]
fn test_parse_errors_still_propagate_with_capturing() {
    // Parse errors should still error out, not be captured
    let result = ColorWithCapture::try_from_ansi(b"not-a-number");
    assert!(result.is_err());

    let result = ColorWithCapture::try_from_ansi(b"");
    assert!(result.is_err());

    // Invalid UTF-8 for string enums should still error
    let result = ModeWithCapture::try_from_ansi(&[0xFF, 0xFE]);
    assert!(result.is_err());
}

#[derive(Debug, Clone, PartialEq, Eq, FromAnsi)]
enum CommandWithCapture {
    Quit,
    Save,
    Load,
    #[vtansi(default)]
    Custom(String),
}

impl TryFrom<&str> for CommandWithCapture {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "quit" | "q" | "exit" => Ok(CommandWithCapture::Quit),
            "save" | "w" | "write" => Ok(CommandWithCapture::Save),
            "load" | "r" | "read" => Ok(CommandWithCapture::Load),
            _ => Err(format!("unknown command: {}", s)),
        }
    }
}

#[test]
fn test_aliases_with_capturing_default() {
    // Test all valid aliases
    assert_eq!(
        CommandWithCapture::try_from_ansi(b"quit").unwrap(),
        CommandWithCapture::Quit
    );
    assert_eq!(
        CommandWithCapture::try_from_ansi(b"q").unwrap(),
        CommandWithCapture::Quit
    );
    assert_eq!(
        CommandWithCapture::try_from_ansi(b"save").unwrap(),
        CommandWithCapture::Save
    );
    assert_eq!(
        CommandWithCapture::try_from_ansi(b"w").unwrap(),
        CommandWithCapture::Save
    );

    // Unknown commands are captured
    assert_eq!(
        CommandWithCapture::try_from_ansi(b"help").unwrap(),
        CommandWithCapture::Custom("help".to_string())
    );
    assert_eq!(
        CommandWithCapture::try_from_ansi(b"custom-command").unwrap(),
        CommandWithCapture::Custom("custom-command".to_string())
    );
}

#[test]
fn test_empty_string_captured() {
    // Empty strings should be captured, not error out
    assert_eq!(
        ModeWithCapture::try_from_ansi(b"").unwrap(),
        ModeWithCapture::Custom("".to_string())
    );
}

#[test]
fn test_whitespace_captured() {
    // Whitespace should be captured
    assert_eq!(
        ModeWithCapture::try_from_ansi(b" ").unwrap(),
        ModeWithCapture::Custom(" ".to_string())
    );
    assert_eq!(
        ModeWithCapture::try_from_ansi(b"  normal  ").unwrap(),
        ModeWithCapture::Custom("  normal  ".to_string())
    );
}

#[test]
fn test_unicode_captured() {
    // Unicode strings should be captured
    assert_eq!(
        ModeWithCapture::try_from_ansi("世界".as_bytes()).unwrap(),
        ModeWithCapture::Custom("世界".to_string())
    );
    assert_eq!(
        ModeWithCapture::try_from_ansi("emoji🎉".as_bytes()).unwrap(),
        ModeWithCapture::Custom("emoji🎉".to_string())
    );
}

#[test]
fn test_special_characters_captured() {
    // Special characters should be captured
    assert_eq!(
        ModeWithCapture::try_from_ansi(b"mode:special").unwrap(),
        ModeWithCapture::Custom("mode:special".to_string())
    );
    assert_eq!(
        ModeWithCapture::try_from_ansi(b"mode/with/slash").unwrap(),
        ModeWithCapture::Custom("mode/with/slash".to_string())
    );
}
