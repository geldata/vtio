//! ESC and Alt key parsing.
//!
//! This module handles ESC key events and Alt modifier combinations.
//! When ESC is followed by another key, the Alt modifier is added to that
//! key.

use vt_push_parser::event::{Esc, EscInvalid};

use crate::event::{KeyCode, KeyEvent, KeyModifiers, TerminalInputEvent};

pub(crate) fn parse_esc<'a>(esc: Esc) -> Option<TerminalInputEvent<'a>> {
    // Check if there are intermediates - if so, we don't recognize this
    // sequence
    if !esc.intermediates.is_empty() {
        return None;
    }

    // Delegate to parse_esc_key for the actual parsing
    parse_esc_key(esc.final_byte)
}

pub(crate) fn parse_esc_invalid<'a>(esc: EscInvalid) -> Option<TerminalInputEvent<'a>> {
    match esc {
        EscInvalid::One(byte) => parse_esc_key(byte),
        _ => None,
    }
}

/// Parse an ESC sequence into a terminal event.
///
/// ESC sequences can be:
/// - A standalone ESC key
/// - An Alt+key combination (ESC followed by a character)
///
/// Note: This function handles generic ESC sequences. SS3 (ESC O) and CSI
/// (ESC [) sequences are handled by their respective parsers.
/// Parse an `ESC` event into a terminal event with Alt modifier.
pub(crate) fn parse_esc_key(byte: u8) -> Option<TerminalInputEvent<'static>> {
    let base_event = match byte {
        // ESC (0x1B) is Ctrl+[, so Alt+ESC is Alt+Ctrl+[
        b'\x1B' => KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE),
        b'\t' => KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE),
        b'\r' | b'\n' => KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE),
        b'\x7F' => KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE),
        b'\0' => KeyEvent::new(KeyCode::Char(' '), KeyModifiers::CONTROL),
        c @ b'\x01'..=b'\x1A' => {
            // Ctrl+A through Ctrl+Z
            let ch = (c - 0x1 + b'a') as char;
            KeyEvent::new(KeyCode::Char(ch), KeyModifiers::CONTROL)
        }
        c @ b'\x1C'..=b'\x1F' => {
            // Ctrl+4 through Ctrl+7
            let ch = (c - 0x1C + b'4') as char;
            KeyEvent::new(KeyCode::Char(ch), KeyModifiers::CONTROL)
        }
        c if c.is_ascii() && !c.is_ascii_control() => {
            // Regular ASCII character
            let ch = c as char;
            let modifiers = if ch.is_ascii_uppercase() {
                KeyModifiers::SHIFT
            } else {
                KeyModifiers::NONE
            };
            KeyEvent::new(KeyCode::Char(ch), modifiers)
        }
        _ => return None,
    };

    // Add Alt modifier to the base event
    let mut alt_event = base_event;
    alt_event.modifiers |= KeyModifiers::ALT;

    Some(TerminalInputEvent::Key(alt_event))
}

#[cfg(test)]
mod tests_alt_key {
    use super::*;

    #[test]
    fn test_esc_key_enter_lf() {
        // ESC LF should produce Alt+Enter
        assert_eq!(
            parse_esc_key(b'\n'),
            Some(TerminalInputEvent::Key(KeyEvent::new(
                KeyCode::Enter,
                KeyModifiers::ALT
            )))
        );
    }

    #[test]
    fn test_esc_key_enter_cr() {
        // ESC CR should produce Alt+Enter
        assert_eq!(
            parse_esc_key(b'\r'),
            Some(TerminalInputEvent::Key(KeyEvent::new(
                KeyCode::Enter,
                KeyModifiers::ALT
            )))
        );
    }

    #[test]
    fn test_esc_key_lowercase() {
        // ESC a should produce Alt+a
        assert_eq!(
            parse_esc_key(b'a'),
            Some(TerminalInputEvent::Key(KeyEvent::new(
                KeyCode::Char('a'),
                KeyModifiers::ALT
            )))
        );
    }

    #[test]
    fn test_esc_key_tab() {
        // ESC Tab should produce Alt+Tab
        assert_eq!(
            parse_esc_key(b'\t'),
            Some(TerminalInputEvent::Key(KeyEvent::new(
                KeyCode::Tab,
                KeyModifiers::ALT
            )))
        );
    }

    #[test]
    fn test_esc_key_esc() {
        // ESC ESC should produce Alt+ESC
        assert_eq!(
            parse_esc_key(b'\x1B'),
            Some(TerminalInputEvent::Key(KeyEvent::new(
                KeyCode::Esc,
                KeyModifiers::ALT
            )))
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::KeyCode;
    use vt_push_parser::event::VTIntermediate;

    #[test]
    fn test_esc_delegates_to_alt_key() {
        // Verify that parse_esc delegates to parse_esc_key
        let esc_event = Esc {
            intermediates: VTIntermediate::empty(),
            final_byte: b'\n',
            private: None,
        };
        assert_eq!(
            parse_esc(esc_event),
            Some(TerminalInputEvent::Key(KeyEvent::new(
                KeyCode::Enter,
                KeyModifiers::ALT
            )))
        );
    }

    #[test]
    fn test_esc_esc() {
        // ESC ESC should produce Alt+ESC (since Ctrl+[ = ESC)
        let esc_event = Esc {
            intermediates: VTIntermediate::empty(),
            final_byte: b'\x1B',
            private: None,
        };
        assert_eq!(
            parse_esc(esc_event),
            Some(TerminalInputEvent::Key(KeyEvent::new(
                KeyCode::Esc,
                KeyModifiers::ALT
            )))
        );
    }

    #[test]
    fn test_alt_lowercase() {
        // ESC a should produce Alt+a
        let esc_event = Esc {
            intermediates: VTIntermediate::empty(),
            final_byte: b'a',
            private: None,
        };
        assert_eq!(
            parse_esc(esc_event),
            Some(TerminalInputEvent::Key(KeyEvent::new(
                KeyCode::Char('a'),
                KeyModifiers::ALT
            )))
        );
    }

    #[test]
    fn test_alt_uppercase() {
        // ESC A should produce Alt+Shift+A
        let esc_event = Esc {
            intermediates: VTIntermediate::empty(),
            final_byte: b'A',
            private: None,
        };
        assert_eq!(
            parse_esc(esc_event),
            Some(TerminalInputEvent::Key(KeyEvent::new(
                KeyCode::Char('A'),
                KeyModifiers::ALT | KeyModifiers::SHIFT
            )))
        );
    }

    #[test]
    fn test_alt_ctrl() {
        // ESC Ctrl+A (0x01) should produce Alt+Ctrl+a
        let esc_event = Esc {
            intermediates: VTIntermediate::empty(),
            final_byte: b'\x01',
            private: None,
        };
        assert_eq!(
            parse_esc(esc_event),
            Some(TerminalInputEvent::Key(KeyEvent::new(
                KeyCode::Char('a'),
                KeyModifiers::ALT | KeyModifiers::CONTROL
            )))
        );
    }

    #[test]
    fn test_alt_tab() {
        // ESC Tab should produce Alt+Tab
        let esc_event = Esc {
            intermediates: VTIntermediate::empty(),
            final_byte: b'\t',
            private: None,
        };
        assert_eq!(
            parse_esc(esc_event),
            Some(TerminalInputEvent::Key(KeyEvent::new(
                KeyCode::Tab,
                KeyModifiers::ALT
            )))
        );
    }

    #[test]
    fn test_alt_enter_cr() {
        // ESC CR should produce Alt+Enter
        let esc_event = Esc {
            intermediates: VTIntermediate::empty(),
            final_byte: b'\r',
            private: None,
        };
        assert_eq!(
            parse_esc(esc_event),
            Some(TerminalInputEvent::Key(KeyEvent::new(
                KeyCode::Enter,
                KeyModifiers::ALT
            )))
        );
    }

    #[test]
    fn test_alt_enter_lf() {
        // ESC LF should also produce Alt+Enter
        let esc_event = Esc {
            intermediates: VTIntermediate::empty(),
            final_byte: b'\n',
            private: None,
        };
        assert_eq!(
            parse_esc(esc_event),
            Some(TerminalInputEvent::Key(KeyEvent::new(
                KeyCode::Enter,
                KeyModifiers::ALT
            )))
        );
    }

    #[test]
    fn test_alt_backspace() {
        // ESC Backspace should produce Alt+Backspace
        let esc_event = Esc {
            intermediates: VTIntermediate::empty(),
            final_byte: b'\x7F',
            private: None,
        };
        assert_eq!(
            parse_esc(esc_event),
            Some(TerminalInputEvent::Key(KeyEvent::new(
                KeyCode::Backspace,
                KeyModifiers::ALT
            )))
        );
    }

    #[test]
    fn test_esc_with_intermediates() {
        // ESC with intermediates should return None
        let esc_event = Esc {
            intermediates: VTIntermediate::one(b' '),
            final_byte: b'A',
            private: None,
        };
        assert_eq!(parse_esc(esc_event), None);
    }
}
