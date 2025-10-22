//! Simple example demonstrating usage of escape sequence macros.
//!
//! Run with: cargo run --example simple

use vtderive::{apc, csi, dcs, deckpam, deckpnm, osc, pm, ss2, ss3, st};
use vtparser::EscapeSequence;
use vtenc::encode::{ConstEncode, Encode};

// Const CSI sequences - fixed parameters, zero-cost encoding
#[csi(private = '?', params = ["6"], finalbyte = 'h')]
struct DecSetMode;

fn decsetmode_handler(params: &vtparser::EscapeSequenceParams) {
    println!("DEC Set Mode handler called with {} params", params.len());
}

#[csi(params = ["1"], finalbyte = 'm')]
struct ResetAttributes;

fn resetattributes_handler(params: &vtparser::EscapeSequenceParams) {
    println!("Reset Attributes handler called");
}

#[csi(finalbyte = 'H')]
struct ClearHome;

fn clearhome_handler(params: &vtparser::EscapeSequenceParams) {
    println!("Clear Home handler called");
}

// Variable CSI sequences - runtime parameters, efficient encoding
#[csi(finalbyte = 'H')]
struct CursorPosition {
    pub row: u16,
    pub col: u16,
}

fn cursorposition_handler(params: &vtparser::EscapeSequenceParams) {
    println!("Cursor Position handler called");
}

#[csi(finalbyte = 'A')]
struct CursorUp {
    pub n: u16,
}

fn cursorup_handler(params: &vtparser::EscapeSequenceParams) {
    println!("Cursor Up handler called");
}

#[csi(finalbyte = 'B')]
struct CursorDown {
    pub n: u16,
}

fn cursordown_handler(params: &vtparser::EscapeSequenceParams) {
    println!("Cursor Down handler called");
}

// OSC sequences
#[osc(params = ["0"], finalbyte = ';')]
struct SetWindowTitle;

fn setwindowtitle_handler(params: &vtparser::EscapeSequenceParams) {
    println!("Set Window Title handler called");
}

// DCS sequences
#[dcs(finalbyte = 'q')]
struct RequestStatus;

fn requeststatus_handler(params: &vtparser::EscapeSequenceParams) {
    println!("Request Status handler called");
}

// SS2 sequences
#[ss2(finalbyte = 'G')]
struct SingleShift2;

fn singleshift2_handler(params: &vtparser::EscapeSequenceParams) {
    println!("Single Shift 2 handler called");
}

// SS3 sequences
#[ss3(finalbyte = 'H')]
struct SingleShift3;

fn singleshift3_handler(params: &vtparser::EscapeSequenceParams) {
    println!("Single Shift 3 handler called");
}

// PM sequences
#[pm(finalbyte = 'p')]
struct PrivacyMessage;

fn privacymessage_handler(params: &vtparser::EscapeSequenceParams) {
    println!("Privacy Message handler called");
}

// APC sequences
#[apc(finalbyte = 'a')]
struct ApplicationCommand;

fn applicationcommand_handler(params: &vtparser::EscapeSequenceParams) {
    println!("Application Command handler called");
}

// ST sequences
#[st(finalbyte = '\\')]
struct StringTerminator;

fn stringterminator_handler(params: &vtparser::EscapeSequenceParams) {
    println!("String Terminator handler called");
}

// DECKPAM sequences
#[deckpam(finalbyte = '=')]
struct KeypadApplicationMode;

fn keypadapplicationmode_handler(params: &vtparser::EscapeSequenceParams) {
    println!("Keypad Application Mode handler called");
}

// DECKPNM sequences
#[deckpnm(finalbyte = '>')]
struct KeypadNumericMode;

fn keypadnumericmode_handler(params: &vtparser::EscapeSequenceParams) {
    println!("Keypad Numeric Mode handler called");
}

fn main() {
    println!("Escape sequence macros example");
    println!("==============================\n");

    // Const sequences - zero-cost encoding
    println!("=== Const Sequences ===\n");

    println!("DecSetMode (const with params):");
    println!("  STR: {:?}", DecSetMode::STR);
    println!("  INTRO: {:?}", DecSetMode::INTRO);
    println!("  PRIVATE: {:?}", DecSetMode::PRIVATE);
    println!("  PARAMS: {:?}", DecSetMode::PARAMS);
    println!("  FINAL: {:?}", DecSetMode::FINAL);

    let mut buf = [0u8; 64];
    let written = DecSetMode.encode(&mut &mut buf[..]).unwrap();
    println!("  Encoded: {:?}", &buf[..written]);
    println!();

    println!("ResetAttributes (const with params):");
    println!("  STR: {:?}", ResetAttributes::STR);
    let written = ResetAttributes.encode(&mut &mut buf[..]).unwrap();
    println!("  Encoded: {:?}", &buf[..written]);
    println!();

    println!("ClearHome (const, no params):");
    println!("  STR: {:?}", ClearHome::STR);
    let written = ClearHome.encode(&mut &mut buf[..]).unwrap();
    println!("  Encoded: {:?}", &buf[..written]);
    println!();

    // Variable sequences - runtime parameters
    println!("=== Variable Sequences ===\n");

    println!("CursorPosition (variable):");
    let mut pos = CursorPosition { row: 10, col: 20 };
    println!("  row: {}, col: {}", pos.row, pos.col);
    let written = pos.encode(&mut &mut buf[..]).unwrap();
    println!("  Encoded: {:?}", &buf[..written]);
    println!("  String: {:?}", std::str::from_utf8(&buf[..written]).unwrap());
    println!();

    println!("CursorUp (variable):");
    let mut up = CursorUp { n: 5 };
    println!("  n: {}", up.n);
    let written = up.encode(&mut &mut buf[..]).unwrap();
    println!("  Encoded: {:?}", &buf[..written]);
    println!("  String: {:?}", std::str::from_utf8(&buf[..written]).unwrap());
    println!();

    println!("CursorDown (variable):");
    let mut down = CursorDown { n: 3 };
    println!("  n: {}", down.n);
    let written = down.encode(&mut &mut buf[..]).unwrap();
    println!("  Encoded: {:?}", &buf[..written]);
    println!("  String: {:?}", std::str::from_utf8(&buf[..written]).unwrap());
    println!();

    // Display registry information
    println!("=== Registry ===\n");
    println!("Total entries: {}", vtparser::ESCAPE_SEQUENCE_REGISTRY.len());
    println!("\nRegistered sequences:");
    for entry in vtparser::ESCAPE_SEQUENCE_REGISTRY.iter().take(10) {
        println!(
            "  - {} (intro={:?}, prefix={:?}, final='{}')",
            entry.name,
            entry.intro,
            entry.prefix,
            entry.final_byte as char
        );
    }
    if vtparser::ESCAPE_SEQUENCE_REGISTRY.len() > 10 {
        println!("  ... and {} more", vtparser::ESCAPE_SEQUENCE_REGISTRY.len() - 10);
    }
}
