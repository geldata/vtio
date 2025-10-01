//! Convert raw UTF8 stream to a Key events

use crate::event::{KeyCode, KeyEvent, KeyModifiers, TerminalInputEvent};

pub const MAX_UTF8_CHAR_BYTES: usize = 4;

#[inline]
fn ascii_to_event<F: for<'a> FnMut(TerminalInputEvent<'a>)>(byte: u8, cb: &mut F) {
    // Note: Most C0 control codes (0x00-0x1F, 0x7F) are handled by the VT
    // parser and come through as VTEvent::C0. However, the VT parser
    // treats \t, \r, and \n as "whitespace control characters" and sends
    // them as Raw events, so we must handle them here.
    match byte {
        b'\t' => {
            // Tab
            let key_event = KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE);
            cb(TerminalInputEvent::Key(key_event));
            return;
        }
        b'\r' => {
            // Enter (carriage return)
            let key_event = KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE);
            cb(TerminalInputEvent::Key(key_event));
            return;
        }
        _ => {}
    }

    // Regular ASCII character
    let ch = byte as char;
    let modifiers = if ch.is_ascii_uppercase() {
        KeyModifiers::SHIFT
    } else {
        KeyModifiers::NONE
    };
    let key_event = KeyEvent::new(KeyCode::Char(ch), modifiers);
    cb(TerminalInputEvent::Key(key_event));
}

#[inline]
fn len_utf8(byte0: u8) -> Option<usize> {
    if byte0 & 0xE0 == 0xC0 {
        Some(2)
    } else if byte0 & 0xF0 == 0xE0 {
        Some(3)
    } else if byte0 & 0xF8 == 0xF0 {
        Some(4)
    } else {
        // Invalid start byte
        None
    }
}

#[inline]
fn validate_utf8_continuation_byte(byte: u8) -> bool {
    byte & !0b0011_1111 == 0b1000_0000
}

/// Validate continuation bytes for a multi-byte UTF-8 sequence.
#[inline]
fn validate_utf8_continuation_bytes(bytes: &[u8]) -> bool {
    for byte in bytes {
        if !validate_utf8_continuation_byte(*byte) {
            return false;
        }
    }

    true
}

#[inline]
fn utf8_to_event<F: for<'a> FnMut(TerminalInputEvent<'a>)>(bytes: &[u8], cb: &mut F) -> bool {
    match std::str::from_utf8(bytes) {
        Ok(s) => {
            // SAFETY: we know there's at least one char
            let ch = unsafe { s.chars().next().unwrap_unchecked() };
            let modifiers = if ch.is_uppercase() {
                KeyModifiers::SHIFT
            } else {
                KeyModifiers::NONE
            };
            let key_event = KeyEvent::new(KeyCode::Char(ch), modifiers);
            cb(TerminalInputEvent::Key(key_event));
            true
        }
        Err(_) => false, // Invalid UTF-8 (overlong encoding, surrogates, etc.)
    }
}

/// Process raw bytes and emit Key events for valid UTF-8 characters.
/// Invalid UTF-8 sequences are silently skipped.  If the buffer ends
/// with a potentially incomplete UTF-8 sequence, returns the number
/// of bytes in the incomplete sequence.
#[inline]
pub fn bytes_to_events<F: for<'a> FnMut(TerminalInputEvent<'a>)>(
    bytes: &[u8],
    utf8_buffer: &mut [u8; MAX_UTF8_CHAR_BYTES],
    cb: &mut F,
) -> usize {
    let mut pos = 0;
    while pos < bytes.len() {
        let byte = bytes[pos];

        // Fast path for ASCII (most common case)
        if byte < 0x80 {
            pos += 1;
            ascii_to_event(byte, cb);
            continue;
        }

        // Multi-byte UTF-8 character
        let Some(char_len) = len_utf8(byte) else {
            // Invalid start byte - skip it
            pos += 1;
            continue;
        };

        if pos + char_len > bytes.len() {
            // Incomplete sequence at end, validate and save it
            if validate_utf8_continuation_bytes(&bytes[pos + 1..]) {
                let remaining = &bytes[pos..];
                let len = remaining.len();
                utf8_buffer[..len].copy_from_slice(&remaining[..len]);
                return len;
            }
            // Invalid continunation byte, skip
            pos += 1;
            continue;
        }

        // Process the multi-byte character
        if utf8_to_event(&bytes[pos..pos + char_len], cb) {
            pos += char_len;
        } else {
            // Invalid UTF-8 - skip the start byte
            pos += 1;
        }
    }

    0
}
