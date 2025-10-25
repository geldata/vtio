//! Example demonstrating default variant handling with FromAnsi.
//!
//! This example shows how to use both unit and capturing default variants
//! to handle unrecognized values gracefully.

use vtenc::FromAnsi;
use vtenc::parse::TryFromAnsi;

/// ANSI SGR attribute with unit default variant.
///
/// Unit default variants return a constant value for all unrecognized
/// inputs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromAnsi)]
#[repr(u8)]
enum SgrAttribute {
    Reset = 0,
    Bold = 1,
    Faint = 2,
    Italic = 3,
    Underline = 4,
    #[vtansi(default)]
    Unknown = 255,
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
            255 => Ok(SgrAttribute::Unknown),
            _ => Err(()),
        }
    }
}

/// HTTP status code with capturing default variant.
///
/// Capturing default variants store the actual unrecognized value,
/// allowing you to inspect what was received.
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromAnsi)]
#[repr(u16)]
enum HttpStatus {
    Ok = 200,
    Created = 201,
    NotFound = 404,
    ServerError = 500,
    #[vtansi(default)]
    Other(u16),
}

impl TryFrom<u16> for HttpStatus {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            200 => Ok(HttpStatus::Ok),
            201 => Ok(HttpStatus::Created),
            404 => Ok(HttpStatus::NotFound),
            500 => Ok(HttpStatus::ServerError),
            _ => Err(()),
        }
    }
}

/// Editor mode with string-based unit default.
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromAnsi)]
enum EditorMode {
    Normal,
    Insert,
    Visual,
    #[vtansi(default)]
    Unknown,
}

impl TryFrom<&str> for EditorMode {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "normal" | "n" => Ok(EditorMode::Normal),
            "insert" | "i" => Ok(EditorMode::Insert),
            "visual" | "v" => Ok(EditorMode::Visual),
            _ => Err(format!("unknown mode: {}", s)),
        }
    }
}

/// Command with capturing string default.
///
/// The captured string can be used to implement custom command handling.
#[derive(Debug, Clone, PartialEq, Eq, FromAnsi)]
enum Command {
    Quit,
    Save,
    Load,
    Help,
    #[vtansi(default)]
    Custom(String),
}

impl TryFrom<&str> for Command {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "quit" | "q" | "exit" => Ok(Command::Quit),
            "save" | "w" | "write" => Ok(Command::Save),
            "load" | "r" | "read" => Ok(Command::Load),
            "help" | "h" | "?" => Ok(Command::Help),
            _ => Err(format!("unknown command: {}", s)),
        }
    }
}

fn main() {
    println!("=== Unit Default Variant (repr enum) ===\n");

    // Parse known SGR attributes
    let attrs: &[&[u8]] = &[b"0", b"1", b"3", b"4"];
    for attr_bytes in attrs {
        let attr = SgrAttribute::try_from_ansi(attr_bytes).unwrap();
        println!(
            "SGR {:?} -> {:?}",
            std::str::from_utf8(attr_bytes).unwrap(),
            attr
        );
    }

    // Parse unknown SGR attributes - all become Unknown
    println!("\nUnknown attributes:");
    let unknown_attrs: &[&[u8]] = &[b"5", b"99", b"254"];
    for attr_bytes in unknown_attrs {
        let attr = SgrAttribute::try_from_ansi(attr_bytes).unwrap();
        println!(
            "SGR {:?} -> {:?} {}",
            std::str::from_utf8(attr_bytes).unwrap(),
            attr,
            if attr == SgrAttribute::Unknown {
                "(default)"
            } else {
                ""
            }
        );
    }

    println!("\n=== Capturing Default Variant (repr enum) ===\n");

    // Parse known HTTP status codes
    let statuses: &[&[u8]] = &[b"200", b"201", b"404", b"500"];
    for status_bytes in statuses {
        let status = HttpStatus::try_from_ansi(status_bytes).unwrap();
        println!(
            "HTTP {:?} -> {:?}",
            std::str::from_utf8(status_bytes).unwrap(),
            status
        );
    }

    // Parse unknown HTTP status codes - captured in Other variant
    println!("\nUnknown status codes (captured):");
    let unknown_statuses: &[&[u8]] = &[b"202", b"403", b"503"];
    for status_bytes in unknown_statuses {
        let status = HttpStatus::try_from_ansi(status_bytes).unwrap();
        if let HttpStatus::Other(code) = status {
            println!(
                "HTTP {:?} -> Other({}) (captured)",
                std::str::from_utf8(status_bytes).unwrap(),
                code
            );
        }
    }

    println!("\n=== Unit Default Variant (string enum) ===\n");

    // Parse known editor modes
    let modes: &[&[u8]] = &[b"normal", b"n", b"insert", b"visual"];
    for mode_bytes in modes {
        let mode = EditorMode::try_from_ansi(mode_bytes).unwrap();
        println!(
            "Mode {:?} -> {:?}",
            std::str::from_utf8(mode_bytes).unwrap(),
            mode
        );
    }

    // Parse unknown editor modes
    println!("\nUnknown modes:");
    let unknown_modes: &[&[u8]] = &[b"command", b"replace", b"terminal"];
    for mode_bytes in unknown_modes {
        let mode = EditorMode::try_from_ansi(mode_bytes).unwrap();
        println!(
            "Mode {:?} -> {:?} {}",
            std::str::from_utf8(mode_bytes).unwrap(),
            mode,
            if mode == EditorMode::Unknown {
                "(default)"
            } else {
                ""
            }
        );
    }

    println!("\n=== Capturing Default Variant (string enum) ===\n");

    // Parse known commands
    let commands: &[&[u8]] = &[b"quit", b"q", b"save", b"w", b"help"];
    for cmd_bytes in commands {
        let cmd = Command::try_from_ansi(cmd_bytes).unwrap();
        println!(
            "Command {:?} -> {:?}",
            std::str::from_utf8(cmd_bytes).unwrap(),
            cmd
        );
    }

    // Parse unknown commands - captured as Custom
    println!("\nUnknown commands (captured):");
    let unknown_commands: &[&[u8]] = &[b"echo", b"set option=value", b"custom-command"];
    for cmd_bytes in unknown_commands {
        let cmd = Command::try_from_ansi(cmd_bytes).unwrap();
        if let Command::Custom(ref s) = cmd {
            println!(
                "Command {:?} -> Custom(\"{}\") (captured)",
                std::str::from_utf8(cmd_bytes).unwrap(),
                s
            );
        }
    }

    println!("\n=== Error Handling ===\n");

    // Parse errors still propagate even with defaults
    println!("Invalid numeric input for repr enum:");
    match SgrAttribute::try_from_ansi(b"not-a-number") {
        Ok(_) => println!("  Unexpected success"),
        Err(e) => println!("  Error: {}", e),
    }

    println!("\nInvalid UTF-8 for string enum:");
    match Command::try_from_ansi(&[0xFF, 0xFE]) {
        Ok(_) => println!("  Unexpected success"),
        Err(e) => println!("  Error: {}", e),
    }

    println!("\n=== Use Cases ===\n");

    println!("Use unit defaults when:");
    println!("  - You want a single fallback value for all unrecognized input");
    println!("  - You don't need to know what the unrecognized value was");
    println!("  - You want the simplest error handling");

    println!("\nUse capturing defaults when:");
    println!("  - You need to preserve the unrecognized value");
    println!("  - You want to implement custom handling for unknown values");
    println!("  - You need to log or display what was received");
    println!("  - You're building extensible systems that support custom values");
}
