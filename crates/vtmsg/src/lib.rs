//! Terminal command sequences implementing common VT escape codes.
//!
//! This crate provides types that implement the `Encode` trait to render
//! terminal control sequences for cursor movement, screen manipulation,
//! and terminal state queries.
//!
//! # Modules
//!
//! - [`cursor`] - Cursor movement and control commands
//! - [`clear`] - Screen clearing commands
//! - [`screen`] - Screen control commands (scrolling, alternate screen, etc.)
//! - [`window`] - Window control commands (title, resize)
//! - [`mode`] - Mode control commands (bracketed paste, focus reporting, etc.)
//! - [`query`] - Terminal query commands
//!
//! # Example
//!
//! ```
//! use vtmsg::{ClearAll, cursor::MoveTo};
//! use vtenc::Encode;
//!
//! let mut buf = [0u8; 64];
//!
//! // Clear the screen
//! let len = ClearAll.encode(&mut &mut buf[..]).unwrap();
//! // Write buf[..len] to stdout
//!
//! // Move cursor to row 10, column 20
//! let len = MoveTo { row: 10, col: 20 }.encode(&mut &mut buf[..]).unwrap();
//! // Write buf[..len] to stdout
//! ```

#![warn(clippy::pedantic)]

pub mod charset;
pub mod cursor;
pub mod keyboard;
pub mod macros;
pub mod mouse;
pub mod screen;
pub mod scroll;
pub mod terminal;
pub mod window;

#[cfg(test)]
mod tests {
    use super::*;
    use vtenc::Encode;

    #[test]
    fn test_clear_commands() {
        let mut buf = [0u8; 64];

        assert_eq!(ClearAll.encode(&mut &mut buf[..]).unwrap(), 4);
        assert_eq!(&buf[..4], b"\x1B[2J");

        assert_eq!(ClearLine.encode(&mut &mut buf[..]).unwrap(), 4);
        assert_eq!(&buf[..4], b"\x1B[2K");
    }

    #[test]
    fn test_cursor_movement() {
        let mut buf = [0u8; 64];

        let len = MoveTo { row: 10, col: 20 }
            .encode(&mut &mut buf[..])
            .unwrap();
        assert_eq!(&buf[..len], b"\x1B[10;20H");

        let len = MoveUp(5).encode(&mut &mut buf[..]).unwrap();
        assert_eq!(&buf[..len], b"\x1B[5A");

        let len = MoveDown(3).encode(&mut &mut buf[..]).unwrap();
        assert_eq!(&buf[..len], b"\x1B[3B");
    }

    #[test]
    fn test_screen_commands() {
        let mut buf = [0u8; 64];

        assert_eq!(EnterAlternateScreen.encode(&mut &mut buf[..]).unwrap(), 8);
        assert_eq!(&buf[..8], b"\x1B[?1049h");

        assert_eq!(LeaveAlternateScreen.encode(&mut &mut buf[..]).unwrap(), 8);
        assert_eq!(&buf[..8], b"\x1B[?1049l");
    }

    #[test]
    fn test_cursor_visibility() {
        let mut buf = [0u8; 64];

        assert_eq!(ShowCursor.encode(&mut &mut buf[..]).unwrap(), 6);
        assert_eq!(&buf[..6], b"\x1B[?25h");

        assert_eq!(HideCursor.encode(&mut &mut buf[..]).unwrap(), 6);
        assert_eq!(&buf[..6], b"\x1B[?25l");
    }

    #[test]
    fn test_cursor_shape() {
        let mut buf = [0u8; 64];

        let len = SetCursorShape(CursorShape::BlinkingBlock)
            .encode(&mut &mut buf[..])
            .unwrap();
        assert_eq!(&buf[..len], b"\x1B[1 q");

        let len = SetCursorShape(CursorShape::SteadyBar)
            .encode(&mut &mut buf[..])
            .unwrap();
        assert_eq!(&buf[..len], b"\x1B[6 q");
    }

    #[test]
    fn test_title() {
        let mut buf = [0u8; 64];

        let len = SetTitle("Test").encode(&mut &mut buf[..]).unwrap();
        assert_eq!(&buf[..len], b"\x1B]0;Test\x1B\\");
    }

    #[test]
    fn test_request_commands() {
        let mut buf = [0u8; 64];

        assert_eq!(RequestCursorPosition.encode(&mut &mut buf[..]).unwrap(), 4);
        assert_eq!(&buf[..4], b"\x1B[6n");

        assert_eq!(
            RequestDeviceAttributes.encode(&mut &mut buf[..]).unwrap(),
            3
        );
        assert_eq!(&buf[..3], b"\x1B[c");
    }

    #[test]
    fn test_feature_request() {
        let mut buf = [0u8; 64];

        let len = RequestFeature(Feature::BracketedPaste)
            .encode(&mut &mut buf[..])
            .unwrap();
        assert_eq!(&buf[..len], b"\x1B[?2004$p");

        let len = RequestFeature(Feature::InsertMode)
            .encode(&mut &mut buf[..])
            .unwrap();
        assert_eq!(&buf[..len], b"\x1B[4$p");
    }

    #[test]
    fn test_buffer_overflow() {
        let mut buf = [0u8; 2];

        let result = ClearAll.encode(&mut &mut buf[..]);
        assert!(matches!(result, Err(vtenc::EncodeError::BufferOverflow(_))));
    }
}
