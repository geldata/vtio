//! SS2 and SS3 (Single Shift) sequence parsing.
//!
//! This module provides functions to parse SS2 and SS3 sequences from
//! terminal input into terminal events. SS3 sequences are used for
//! application mode cursor keys and function keys. SS2 sequences are not
//! used for keyboard input in standard terminals (ESC N should be treated
//! as Alt+N instead).

use vt_push_parser::event::SS3;

use crate::event::{KeyCode, TerminalInputEvent};

/// Parse an SS3 sequence into a terminal event.
///
/// SS3 sequences (ESC O x) are used for application mode cursor keys and
/// function keys.
pub(crate) fn parse_ss3<'a>(ss3: SS3) -> Option<TerminalInputEvent<'a>> {
    let ch = ss3.char;

    // Parse SS3 sequences for function/cursor keys
    let key_code = match ch {
        // Cursor keys (application mode)
        b'A' => KeyCode::Up,
        b'B' => KeyCode::Down,
        b'C' => KeyCode::Right,
        b'D' => KeyCode::Left,
        b'H' => KeyCode::Home,
        b'F' => KeyCode::End,
        // Function keys
        b'P' => KeyCode::F(1),
        b'Q' => KeyCode::F(2),
        b'R' => KeyCode::F(3),
        b'S' => KeyCode::F(4),
        // Additional keypad keys in application mode
        b'M' => KeyCode::Enter,     // Keypad Enter
        b'X' => KeyCode::Char('='), // Keypad =
        b'j' => KeyCode::Char('*'), // Keypad *
        b'k' => KeyCode::Char('+'), // Keypad +
        b'l' => KeyCode::Char(','), // Keypad ,
        b'm' => KeyCode::Char('-'), // Keypad -
        b'n' => KeyCode::Char('.'), // Keypad .
        b'o' => KeyCode::Char('/'), // Keypad /
        b'p' => KeyCode::Char('0'), // Keypad 0
        b'q' => KeyCode::Char('1'), // Keypad 1
        b'r' => KeyCode::Char('2'), // Keypad 2
        b's' => KeyCode::Char('3'), // Keypad 3
        b't' => KeyCode::Char('4'), // Keypad 4
        b'u' => KeyCode::Char('5'), // Keypad 5
        b'v' => KeyCode::Char('6'), // Keypad 6
        b'w' => KeyCode::Char('7'), // Keypad 7
        b'x' => KeyCode::Char('8'), // Keypad 8
        b'y' => KeyCode::Char('9'), // Keypad 9
        _ => return None,
    };

    Some(TerminalInputEvent::Key(key_code.into()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::KeyCode;

    #[test]
    fn test_parse_ss3_cursor_keys() {
        // Test cursor keys in application mode
        let ss3_up = SS3 { char: b'A' };
        let ss3_down = SS3 { char: b'B' };
        let ss3_right = SS3 { char: b'C' };
        let ss3_left = SS3 { char: b'D' };

        assert_eq!(
            parse_ss3(ss3_up),
            Some(TerminalInputEvent::Key(KeyCode::Up.into()))
        );
        assert_eq!(
            parse_ss3(ss3_down),
            Some(TerminalInputEvent::Key(KeyCode::Down.into()))
        );
        assert_eq!(
            parse_ss3(ss3_right),
            Some(TerminalInputEvent::Key(KeyCode::Right.into()))
        );
        assert_eq!(
            parse_ss3(ss3_left),
            Some(TerminalInputEvent::Key(KeyCode::Left.into()))
        );
    }

    #[test]
    fn test_parse_ss3_home_end() {
        // Test Home and End keys
        let ss3_home = SS3 { char: b'H' };
        let ss3_end = SS3 { char: b'F' };

        assert_eq!(
            parse_ss3(ss3_home),
            Some(TerminalInputEvent::Key(KeyCode::Home.into()))
        );
        assert_eq!(
            parse_ss3(ss3_end),
            Some(TerminalInputEvent::Key(KeyCode::End.into()))
        );
    }

    #[test]
    fn test_parse_ss3_function_keys() {
        // Test function keys F1-F4
        let ss3_f1 = SS3 { char: b'P' };
        let ss3_f2 = SS3 { char: b'Q' };
        let ss3_f3 = SS3 { char: b'R' };
        let ss3_f4 = SS3 { char: b'S' };

        assert_eq!(
            parse_ss3(ss3_f1),
            Some(TerminalInputEvent::Key(KeyCode::F(1).into()))
        );
        assert_eq!(
            parse_ss3(ss3_f2),
            Some(TerminalInputEvent::Key(KeyCode::F(2).into()))
        );
        assert_eq!(
            parse_ss3(ss3_f3),
            Some(TerminalInputEvent::Key(KeyCode::F(3).into()))
        );
        assert_eq!(
            parse_ss3(ss3_f4),
            Some(TerminalInputEvent::Key(KeyCode::F(4).into()))
        );
    }

    #[test]
    fn test_parse_ss3_keypad_keys() {
        // Test keypad keys in application mode
        let ss3_enter = SS3 { char: b'M' };
        let ss3_plus = SS3 { char: b'k' };
        let ss3_minus = SS3 { char: b'm' };
        let ss3_star = SS3 { char: b'j' };

        assert_eq!(
            parse_ss3(ss3_enter),
            Some(TerminalInputEvent::Key(KeyCode::Enter.into()))
        );
        assert_eq!(
            parse_ss3(ss3_plus),
            Some(TerminalInputEvent::Key(KeyCode::Char('+').into()))
        );
        assert_eq!(
            parse_ss3(ss3_minus),
            Some(TerminalInputEvent::Key(KeyCode::Char('-').into()))
        );
        assert_eq!(
            parse_ss3(ss3_star),
            Some(TerminalInputEvent::Key(KeyCode::Char('*').into()))
        );
    }

    #[test]
    fn test_parse_ss3_unrecognized() {
        // Test unrecognized SS3 sequence
        let ss3_unknown = SS3 { char: b'Z' };
        assert_eq!(parse_ss3(ss3_unknown), None);
    }
}
