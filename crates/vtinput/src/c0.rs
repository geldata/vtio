//! C0 control code parsing.
//!
//! This module handles C0 control codes (0x00-0x1F and 0x7F) that come
//! through as `VTEvent::C0` from the VT parser.

use crate::event::{KeyCode, KeyEvent, KeyModifiers, TerminalInputEvent};

/// Parse a C0 control code into a terminal event.
///
/// C0 codes are control characters in the range 0x00-0x1F and 0x7F (DEL).
/// Many of these map to Ctrl+letter combinations.
pub(crate) fn parse_c0(byte: u8) -> Option<TerminalInputEvent<'static>> {
    let key_event = match byte {
        b'\t' => {
            // Tab (0x09)
            KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE)
        }
        b'\r' => {
            // Enter (carriage return, 0x0D)
            KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE)
        }
        b'\x7F' => {
            // Backspace (DEL, 0x7F)
            KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE)
        }
        b'\0' => {
            // Ctrl+Space (0x00)
            KeyEvent::new(KeyCode::Char(' '), KeyModifiers::CONTROL)
        }
        c @ b'\x01'..=b'\x1A' => {
            // Ctrl+A through Ctrl+Z (0x01-0x1A)
            let ch = (c - 0x1 + b'a') as char;
            KeyEvent::new(KeyCode::Char(ch), KeyModifiers::CONTROL)
        }
        c @ b'\x1C'..=b'\x1F' => {
            // Ctrl+4 through Ctrl+7 (Ctrl+\ Ctrl+] Ctrl+^ Ctrl+_)
            // (0x1C-0x1F)
            let ch = (c - 0x1C + b'4') as char;
            KeyEvent::new(KeyCode::Char(ch), KeyModifiers::CONTROL)
        }
        b'\x1B' => {
            // ESC key (0x1B)
            // Note: This typically comes through as VTEvent::Esc, but we
            // handle it here for completeness
            KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE)
        }
        _ => {
            // Unknown or unsupported C0 code
            return None;
        }
    };

    Some(TerminalInputEvent::Key(key_event))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tab() {
        assert_eq!(
            parse_c0(b'\t'),
            Some(TerminalInputEvent::Key(KeyEvent::new(
                KeyCode::Tab,
                KeyModifiers::NONE
            )))
        );
    }

    #[test]
    fn test_enter() {
        assert_eq!(
            parse_c0(b'\r'),
            Some(TerminalInputEvent::Key(KeyEvent::new(
                KeyCode::Enter,
                KeyModifiers::NONE
            )))
        );
    }

    #[test]
    fn test_backspace() {
        assert_eq!(
            parse_c0(b'\x7F'),
            Some(TerminalInputEvent::Key(KeyEvent::new(
                KeyCode::Backspace,
                KeyModifiers::NONE
            )))
        );
    }

    #[test]
    fn test_ctrl_space() {
        assert_eq!(
            parse_c0(b'\0'),
            Some(TerminalInputEvent::Key(KeyEvent::new(
                KeyCode::Char(' '),
                KeyModifiers::CONTROL
            )))
        );
    }

    #[test]
    fn test_ctrl_letters() {
        // Test Ctrl+A
        assert_eq!(
            parse_c0(b'\x01'),
            Some(TerminalInputEvent::Key(KeyEvent::new(
                KeyCode::Char('a'),
                KeyModifiers::CONTROL
            )))
        );

        // Test Ctrl+M (which is also Enter, but we map it to Ctrl+M)
        assert_eq!(
            parse_c0(b'\x0D'),
            Some(TerminalInputEvent::Key(KeyEvent::new(
                KeyCode::Enter,
                KeyModifiers::NONE
            )))
        );

        // Test Ctrl+Z
        assert_eq!(
            parse_c0(b'\x1A'),
            Some(TerminalInputEvent::Key(KeyEvent::new(
                KeyCode::Char('z'),
                KeyModifiers::CONTROL
            )))
        );
    }

    #[test]
    fn test_ctrl_special_chars() {
        // Test Ctrl+\ (0x1C)
        assert_eq!(
            parse_c0(b'\x1C'),
            Some(TerminalInputEvent::Key(KeyEvent::new(
                KeyCode::Char('4'),
                KeyModifiers::CONTROL
            )))
        );

        // Test Ctrl+] (0x1D)
        assert_eq!(
            parse_c0(b'\x1D'),
            Some(TerminalInputEvent::Key(KeyEvent::new(
                KeyCode::Char('5'),
                KeyModifiers::CONTROL
            )))
        );

        // Test Ctrl+^ (0x1E)
        assert_eq!(
            parse_c0(b'\x1E'),
            Some(TerminalInputEvent::Key(KeyEvent::new(
                KeyCode::Char('6'),
                KeyModifiers::CONTROL
            )))
        );

        // Test Ctrl+_ (0x1F)
        assert_eq!(
            parse_c0(b'\x1F'),
            Some(TerminalInputEvent::Key(KeyEvent::new(
                KeyCode::Char('7'),
                KeyModifiers::CONTROL
            )))
        );
    }

    #[test]
    fn test_esc() {
        assert_eq!(
            parse_c0(b'\x1B'),
            Some(TerminalInputEvent::Key(KeyEvent::new(
                KeyCode::Esc,
                KeyModifiers::NONE
            )))
        );
    }

    #[test]
    fn test_unsupported_c0() {
        // All C0 codes in the standard range are now handled
        // 0x0B is Ctrl+K, 0x0C is Ctrl+L - both are valid
        // Test that we handle them correctly
        assert_eq!(
            parse_c0(b'\x0B'),
            Some(TerminalInputEvent::Key(KeyEvent::new(
                KeyCode::Char('k'),
                KeyModifiers::CONTROL
            )))
        );
        assert_eq!(
            parse_c0(b'\x0C'),
            Some(TerminalInputEvent::Key(KeyEvent::new(
                KeyCode::Char('l'),
                KeyModifiers::CONTROL
            )))
        );
    }
}
