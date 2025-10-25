//! Example demonstrating FromAnsi and ToAnsi derive macros with ANSI SGR
//! codes.
//!
//! This example shows how to use the FromAnsi and ToAnsi derive macros to
//! parse and encode ANSI Select Graphic Rendition (SGR) codes from escape
//! sequences.

use vtenc::encode::AnsiEncode;
use vtenc::parse::TryFromAnsi;
use vtenc::{FromAnsi, ToAnsi};

/// ANSI SGR color codes (foreground colors 30-37).
///
/// Includes a default variant for unrecognized color codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromAnsi, ToAnsi)]
#[repr(u8)]
enum SgrColor {
    Black = 30,
    Red = 31,
    Green = 32,
    Yellow = 33,
    Blue = 34,
    Magenta = 35,
    Cyan = 36,
    White = 37,
    #[vtansi(default)]
    Unknown = 255,
}

impl TryFrom<u8> for SgrColor {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            30 => Ok(SgrColor::Black),
            31 => Ok(SgrColor::Red),
            32 => Ok(SgrColor::Green),
            33 => Ok(SgrColor::Yellow),
            34 => Ok(SgrColor::Blue),
            35 => Ok(SgrColor::Magenta),
            36 => Ok(SgrColor::Cyan),
            37 => Ok(SgrColor::White),
            255 => Ok(SgrColor::Unknown),
            _ => Err(()),
        }
    }
}

/// Text decoration modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromAnsi, ToAnsi)]
#[repr(u8)]
enum TextDecoration {
    Reset = 0,
    Bold = 1,
    Dim = 2,
    Italic = 3,
    Underline = 4,
    Blink = 5,
    Reverse = 7,
    Hidden = 8,
    Strikethrough = 9,
}

impl TryFrom<u8> for TextDecoration {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(TextDecoration::Reset),
            1 => Ok(TextDecoration::Bold),
            2 => Ok(TextDecoration::Dim),
            3 => Ok(TextDecoration::Italic),
            4 => Ok(TextDecoration::Underline),
            5 => Ok(TextDecoration::Blink),
            7 => Ok(TextDecoration::Reverse),
            8 => Ok(TextDecoration::Hidden),
            9 => Ok(TextDecoration::Strikethrough),
            _ => Err(()),
        }
    }
}

/// Terminal cursor style (for DECSCUSR sequence).
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromAnsi, ToAnsi)]
enum CursorStyle {
    Default,
    BlinkingBlock,
    SteadyBlock,
    BlinkingUnderline,
    SteadyUnderline,
    BlinkingBar,
    SteadyBar,
}

impl TryFrom<&str> for CursorStyle {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "0" | "default" => Ok(CursorStyle::Default),
            "1" | "blinking-block" => Ok(CursorStyle::BlinkingBlock),
            "2" | "steady-block" => Ok(CursorStyle::SteadyBlock),
            "3" | "blinking-underline" => Ok(CursorStyle::BlinkingUnderline),
            "4" | "steady-underline" => Ok(CursorStyle::SteadyUnderline),
            "5" | "blinking-bar" => Ok(CursorStyle::BlinkingBar),
            "6" | "steady-bar" => Ok(CursorStyle::SteadyBar),
            _ => Err(format!("unknown cursor style: {}", s)),
        }
    }
}

impl AsRef<str> for CursorStyle {
    fn as_ref(&self) -> &str {
        match self {
            CursorStyle::Default => "default",
            CursorStyle::BlinkingBlock => "blinking-block",
            CursorStyle::SteadyBlock => "steady-block",
            CursorStyle::BlinkingUnderline => "blinking-underline",
            CursorStyle::SteadyUnderline => "steady-underline",
            CursorStyle::BlinkingBar => "blinking-bar",
            CursorStyle::SteadyBar => "steady-bar",
        }
    }
}

fn main() {
    println!("=== ANSI SGR Color Codes ===\n");

    // Parse color codes
    let colors = [b"31", b"32", b"33", b"34"];
    for color_bytes in &colors {
        match SgrColor::try_from_ansi(*color_bytes) {
            Ok(color) => println!("Parsed color code {:?} -> {:?}",
                std::str::from_utf8(*color_bytes).unwrap(), color),
            Err(e) => println!("Failed to parse: {}", e),
        }
    }

    println!("\n=== Text Decoration Codes ===\n");

    // Parse decoration codes
    let decorations = [b"0", b"1", b"3", b"4", b"9"];
    for dec_bytes in &decorations {
        match TextDecoration::try_from_ansi(*dec_bytes) {
            Ok(dec) => println!("Parsed decoration code {:?} -> {:?}",
                std::str::from_utf8(*dec_bytes).unwrap(), dec),
            Err(e) => println!("Failed to parse: {}", e),
        }
    }

    println!("\n=== Cursor Styles ===\n");

    // Parse cursor styles (both numeric and string representations)
    let styles: &[&[u8]] = &[
        b"0",
        b"default",
        b"1",
        b"blinking-block",
        b"steady-bar",
    ];
    for style_bytes in styles {
        match CursorStyle::try_from_ansi(style_bytes) {
            Ok(style) => println!("Parsed cursor style {:?} -> {:?}",
                std::str::from_utf8(style_bytes).unwrap(), style),
            Err(e) => println!("Failed to parse: {}", e),
        }
    }

    println!("\n=== Error Handling ===\n");

    // Demonstrate error cases
    match SgrColor::try_from_ansi(b"99") {
        Ok(_) => println!("Unexpected success"),
        Err(e) => println!("Invalid color code 99: {}", e),
    }

    match TextDecoration::try_from_ansi(b"not-a-number") {
        Ok(_) => println!("Unexpected success"),
        Err(e) => println!("Invalid decoration: {}", e),
    }

    match CursorStyle::try_from_ansi(b"invalid-style") {
        Ok(_) => println!("Unexpected success"),
        Err(e) => println!("Invalid cursor style: {}", e),
    }

    println!("\n=== Default Variant Handling ===\n");

    // Demonstrate default variant handling with unrecognized color codes
    println!("Testing unrecognized color codes with default variant:");

    let unrecognized_codes: &[&[u8]] = &[b"99", b"0", b"50"];
    for code in unrecognized_codes {
        match SgrColor::try_from_ansi(code) {
            Ok(color) => println!(
                "  Code {:?} -> {:?}{}",
                std::str::from_utf8(code).unwrap(),
                color,
                if color == SgrColor::Unknown {
                    " (fallback to default)"
                } else {
                    ""
                }
            ),
            Err(e) => println!("  Failed to parse: {}", e),
        }
    }

    println!("\n=== Encoding (ToAnsi) ===\n");

    // Demonstrate encoding with ToAnsi
    let color = SgrColor::Red;
    let encoded = color.encode_ansi().unwrap();
    println!(
        "Encoded {:?} -> {:?}",
        color,
        std::str::from_utf8(&encoded).unwrap()
    );

    let decoration = TextDecoration::Bold;
    let encoded = decoration.encode_ansi().unwrap();
    println!(
        "Encoded {:?} -> {:?}",
        decoration,
        std::str::from_utf8(&encoded).unwrap()
    );

    let cursor = CursorStyle::SteadyBar;
    let encoded = cursor.encode_ansi().unwrap();
    println!(
        "Encoded {:?} -> {:?}",
        cursor,
        std::str::from_utf8(&encoded).unwrap()
    );

    println!("\n=== Roundtrip Tests ===\n");

    // Demonstrate roundtrip: encode then decode
    let original_color = SgrColor::Green;
    let encoded = original_color.encode_ansi().unwrap();
    let decoded = SgrColor::try_from_ansi(&encoded).unwrap();
    println!(
        "Roundtrip: {:?} -> {:?} -> {:?} ({})",
        original_color,
        std::str::from_utf8(&encoded).unwrap(),
        decoded,
        if original_color == decoded {
            "✓"
        } else {
            "✗"
        }
    );

    let original_dec = TextDecoration::Italic;
    let encoded = original_dec.encode_ansi().unwrap();
    let decoded = TextDecoration::try_from_ansi(&encoded).unwrap();
    println!(
        "Roundtrip: {:?} -> {:?} -> {:?} ({})",
        original_dec,
        std::str::from_utf8(&encoded).unwrap(),
        decoded,
        if original_dec == decoded { "✓" } else { "✗" }
    );

    let original_cursor = CursorStyle::BlinkingBlock;
    let encoded = original_cursor.encode_ansi().unwrap();
    let decoded = CursorStyle::try_from_ansi(&encoded).unwrap();
    println!(
        "Roundtrip: {:?} -> {:?} -> {:?} ({})",
        original_cursor,
        std::str::from_utf8(&encoded).unwrap(),
        decoded,
        if original_cursor == decoded {
            "✓"
        } else {
            "✗"
        }
    );
}
