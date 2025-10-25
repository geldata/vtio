//! Comprehensive integration test demonstrating both FromAnsi and ToAnsi
//! working together in realistic scenarios.

use vtenc::encode::AnsiEncode;
use vtenc::parse::TryFromAnsi;
use vtansi_derive::{FromAnsi, ToAnsi};

/// ANSI SGR (Select Graphic Rendition) attributes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromAnsi, ToAnsi)]
#[repr(u8)]
enum SgrAttribute {
    Reset = 0,
    Bold = 1,
    Faint = 2,
    Italic = 3,
    Underline = 4,
    SlowBlink = 5,
    RapidBlink = 6,
    Reverse = 7,
    Conceal = 8,
    CrossedOut = 9,
}

impl TryFrom<u8> for SgrAttribute {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(SgrAttribute::Reset),
            1 => Ok(SgrAttribute::Bold),
            2 => Ok(SgrAttribute::Faint),
            3 => Ok(SgrAttribute::Italic),
            4 => Ok(SgrAttribute::Underline),
            5 => Ok(SgrAttribute::SlowBlink),
            6 => Ok(SgrAttribute::RapidBlink),
            7 => Ok(SgrAttribute::Reverse),
            8 => Ok(SgrAttribute::Conceal),
            9 => Ok(SgrAttribute::CrossedOut),
            _ => Err(()),
        }
    }
}

/// Terminal bell types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromAnsi, ToAnsi)]
enum BellType {
    Audible,
    Visual,
    None,
}

impl TryFrom<&str> for BellType {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "audible" | "beep" | "sound" => Ok(BellType::Audible),
            "visual" | "flash" => Ok(BellType::Visual),
            "none" | "off" => Ok(BellType::None),
            _ => Err(format!("unknown bell type: {}", s)),
        }
    }
}

impl AsRef<str> for BellType {
    fn as_ref(&self) -> &str {
        match self {
            BellType::Audible => "audible",
            BellType::Visual => "visual",
            BellType::None => "none",
        }
    }
}

/// Cursor shape for terminal emulators.
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromAnsi, ToAnsi)]
#[repr(u32)]
enum CursorShape {
    Block = 0,
    Underline = 1,
    Bar = 2,
}

impl TryFrom<u32> for CursorShape {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(CursorShape::Block),
            1 => Ok(CursorShape::Underline),
            2 => Ok(CursorShape::Bar),
            _ => Err(()),
        }
    }
}

#[test]
fn test_sgr_attributes_comprehensive() {
    let attributes = [
        SgrAttribute::Reset,
        SgrAttribute::Bold,
        SgrAttribute::Italic,
        SgrAttribute::Underline,
        SgrAttribute::Reverse,
    ];

    for attr in &attributes {
        // Encode to bytes
        let encoded = attr.encode_ansi().unwrap();

        // Decode back
        let decoded = SgrAttribute::try_from_ansi(&encoded).unwrap();

        // Verify roundtrip
        assert_eq!(*attr, decoded);

        // Verify the encoded format is correct
        let expected = format!("{}", *attr as u8);
        assert_eq!(encoded, expected.as_bytes());
    }
}

#[test]
fn test_bell_type_comprehensive() {
    let types = [BellType::Audible, BellType::Visual, BellType::None];

    for bell_type in &types {
        // Encode to bytes
        let encoded = bell_type.encode_ansi().unwrap();

        // Decode back
        let decoded = BellType::try_from_ansi(&encoded).unwrap();

        // Verify roundtrip
        assert_eq!(*bell_type, decoded);

        // Verify the encoded format matches AsRef<str>
        assert_eq!(encoded, bell_type.as_ref().as_bytes());
    }
}

#[test]
fn test_multiple_parse_styles() {
    // Test that string-based enum accepts multiple representations
    assert_eq!(
        BellType::try_from_ansi(b"audible").unwrap(),
        BellType::Audible
    );
    assert_eq!(
        BellType::try_from_ansi(b"beep").unwrap(),
        BellType::Audible
    );
    assert_eq!(
        BellType::try_from_ansi(b"sound").unwrap(),
        BellType::Audible
    );

    // But encoding always produces canonical form
    assert_eq!(
        BellType::Audible.encode_ansi().unwrap(),
        b"audible"
    );
}

#[test]
fn test_sequence_building() {
    // Simulate building an ANSI escape sequence with multiple parameters
    let mut sequence = Vec::new();

    // Add SGR attributes
    SgrAttribute::Bold.encode_ansi_into(&mut sequence).unwrap();
    sequence.push(b';');
    SgrAttribute::Italic.encode_ansi_into(&mut sequence).unwrap();
    sequence.push(b';');
    SgrAttribute::Underline
        .encode_ansi_into(&mut sequence)
        .unwrap();

    assert_eq!(sequence, b"1;3;4");

    // Parse them back
    let parts: Vec<&[u8]> = sequence.split(|&b| b == b';').collect();
    assert_eq!(parts.len(), 3);

    let parsed: Vec<SgrAttribute> = parts
        .iter()
        .map(|part| SgrAttribute::try_from_ansi(part).unwrap())
        .collect();

    assert_eq!(parsed[0], SgrAttribute::Bold);
    assert_eq!(parsed[1], SgrAttribute::Italic);
    assert_eq!(parsed[2], SgrAttribute::Underline);
}

#[test]
fn test_cursor_shape_u32() {
    // Test that u32 repr works correctly
    let shapes = [
        CursorShape::Block,
        CursorShape::Underline,
        CursorShape::Bar,
    ];

    for shape in &shapes {
        let encoded = shape.encode_ansi().unwrap();
        let decoded = CursorShape::try_from_ansi(&encoded).unwrap();
        assert_eq!(*shape, decoded);
    }
}

#[test]
fn test_mixed_types_in_sequence() {
    // Create a sequence with different enum types
    let mut buf = Vec::new();

    // SGR attribute (u8 repr)
    SgrAttribute::Bold.encode_ansi_into(&mut buf).unwrap();
    buf.extend_from_slice(b" ");

    // Bell type (string-based)
    BellType::Visual.encode_ansi_into(&mut buf).unwrap();
    buf.extend_from_slice(b" ");

    // Cursor shape (u32 repr)
    CursorShape::Bar.encode_ansi_into(&mut buf).unwrap();

    assert_eq!(buf, b"1 visual 2");

    // Parse them back
    let parts: Vec<&[u8]> = buf.split(|&b| b == b' ').collect();

    let attr = SgrAttribute::try_from_ansi(parts[0]).unwrap();
    let bell = BellType::try_from_ansi(parts[1]).unwrap();
    let cursor = CursorShape::try_from_ansi(parts[2]).unwrap();

    assert_eq!(attr, SgrAttribute::Bold);
    assert_eq!(bell, BellType::Visual);
    assert_eq!(cursor, CursorShape::Bar);
}

#[test]
fn test_all_sgr_attributes_roundtrip() {
    // Test all possible SGR attributes
    for i in 0..=9u8 {
        if let Ok(attr) = SgrAttribute::try_from(i) {
            let encoded = attr.encode_ansi().unwrap();
            let decoded = SgrAttribute::try_from_ansi(&encoded).unwrap();
            assert_eq!(attr, decoded);

            // Verify encoding matches discriminant
            assert_eq!(encoded, format!("{}", i).as_bytes());
        }
    }
}

#[test]
fn test_error_handling_with_invalid_data() {
    // Invalid SGR attribute
    let result = SgrAttribute::try_from_ansi(b"99");
    assert!(result.is_err());

    // Invalid bell type
    let result = BellType::try_from_ansi(b"invalid");
    assert!(result.is_err());

    // Invalid cursor shape
    let result = CursorShape::try_from_ansi(b"999");
    assert!(result.is_err());

    // Invalid UTF-8 for string-based enum
    let result = BellType::try_from_ansi(&[0xFF, 0xFE]);
    assert!(result.is_err());

    // Invalid number format
    let result = SgrAttribute::try_from_ansi(b"not-a-number");
    assert!(result.is_err());
}

#[test]
fn test_encode_to_fixed_buffer() {
    // Test encoding to a fixed-size buffer
    let mut buf = [0u8; 10];

    let n = SgrAttribute::Bold
        .encode_ansi_into_slice(&mut buf)
        .unwrap();
    assert_eq!(n, 1);
    assert_eq!(&buf[..n], b"1");

    let n = BellType::Visual
        .encode_ansi_into_slice(&mut buf)
        .unwrap();
    assert_eq!(n, 6);
    assert_eq!(&buf[..n], b"visual");

    let n = CursorShape::Underline
        .encode_ansi_into_slice(&mut buf)
        .unwrap();
    assert_eq!(n, 1);
    assert_eq!(&buf[..n], b"1");
}

#[test]
fn test_batch_operations() {
    // Encode multiple values at once
    let attrs = [
        SgrAttribute::Reset,
        SgrAttribute::Bold,
        SgrAttribute::Italic,
    ];

    let encoded: Vec<Vec<u8>> = attrs
        .iter()
        .map(|attr| attr.encode_ansi().unwrap())
        .collect();

    assert_eq!(encoded[0], b"0");
    assert_eq!(encoded[1], b"1");
    assert_eq!(encoded[2], b"3");

    // Decode them back
    let decoded: Vec<SgrAttribute> = encoded
        .iter()
        .map(|bytes| SgrAttribute::try_from_ansi(bytes).unwrap())
        .collect();

    assert_eq!(decoded, attrs);
}

#[test]
fn test_canonical_encoding() {
    // Even if parsing accepts multiple forms, encoding should produce
    // canonical form
    let bell1 = BellType::try_from_ansi(b"beep").unwrap();
    let bell2 = BellType::try_from_ansi(b"sound").unwrap();
    let bell3 = BellType::try_from_ansi(b"audible").unwrap();

    // All parse to the same value
    assert_eq!(bell1, bell2);
    assert_eq!(bell2, bell3);

    // And all encode to the same canonical form
    assert_eq!(bell1.encode_ansi().unwrap(), b"audible");
    assert_eq!(bell2.encode_ansi().unwrap(), b"audible");
    assert_eq!(bell3.encode_ansi().unwrap(), b"audible");
}