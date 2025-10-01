use vt_push_parser::{
    VT_PARSER_INTEREST_ALL,
    capture::{VTCaptureEvent, VTCapturePushParser, VTInputCapture},
    event::VTEvent,
};

use crate::event::TerminalInputEvent;

struct ParserState {
    // Buffer for incomplete UTF-8 sequences from previous feed (max 4 bytes)
    utf8_buffer: [u8; crate::char::MAX_UTF8_CHAR_BYTES],
    utf8_buffer_len: usize,
    paste_buffer: Vec<u8>,
}

impl ParserState {
    const fn new() -> Self {
        Self {
            utf8_buffer: [0; crate::char::MAX_UTF8_CHAR_BYTES],
            utf8_buffer_len: 0,
            paste_buffer: Vec::new(),
        }
    }
}

pub struct TerminalInputParser {
    seq_parser: VTCapturePushParser<VT_PARSER_INTEREST_ALL>,
    state: ParserState,
}

impl Default for TerminalInputParser {
    fn default() -> Self {
        Self::new()
    }
}

impl TerminalInputParser {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            seq_parser: VTCapturePushParser::new_with_interest::<VT_PARSER_INTEREST_ALL>(),
            state: ParserState::new(),
        }
    }

    /// Decode a buffer of bytes into a series of events.
    pub fn decode_buffer<'a>(input: &'a [u8], mut cb: impl for<'b> FnMut(TerminalInputEvent<'b>)) {
        let mut parser = TerminalInputParser::new();
        parser.feed_with(input, &mut cb);
    }

    // =====================
    // Callback-driven API
    // =====================

    /// Feed bytes into the parser. This is the main entry point for the parser.
    /// It will call the callback with events as they are emitted.
    ///
    /// The callback must be valid for the lifetime of the `feed_with` call.
    ///
    /// The callback may emit any number of events (including zero), depending
    /// on the state of the internal parser.
    #[inline]
    pub fn feed_with<'this, 'input, F: for<'any> FnMut(TerminalInputEvent<'any>)>(
        &'this mut self,
        input: &'input [u8],
        cb: &mut F,
    ) {
        self.seq_parser
            .feed_with(input, |vt_event: VTCaptureEvent<'_>| {
                Self::process_vt_event(&vt_event, &mut self.state, cb)
            });
    }

    fn process_vt_event<F: for<'any> FnMut(TerminalInputEvent<'any>)>(
        vt_event: &VTCaptureEvent,
        state: &mut ParserState,
        cb: &mut F,
    ) -> VTInputCapture {
        match vt_event {
            VTCaptureEvent::VTEvent(VTEvent::Raw(bytes)) => {
                if state.utf8_buffer_len == 0 {
                    state.utf8_buffer_len =
                        crate::char::bytes_to_events(bytes, &mut state.utf8_buffer, cb);
                } else {
                    let buf_len = state.utf8_buffer_len;

                    // Combine buffered bytes with just enough new bytes to
                    // complete a UTF-8 char (max 4 bytes total).
                    let mut temp_buf = [0u8; crate::char::MAX_UTF8_CHAR_BYTES];
                    temp_buf[..buf_len].copy_from_slice(&state.utf8_buffer[..buf_len]);

                    let take = bytes.len().min(crate::char::MAX_UTF8_CHAR_BYTES - buf_len);
                    temp_buf[buf_len..buf_len + take].copy_from_slice(&bytes[..take]);

                    let incomplete_len = crate::char::bytes_to_events(
                        &temp_buf[..buf_len + take],
                        &mut state.utf8_buffer,
                        cb,
                    );

                    state.utf8_buffer_len = if take < bytes.len() && incomplete_len <= take {
                        // Process from start of incomplete sequence (or from take if none)
                        crate::char::bytes_to_events(
                            &bytes[take - incomplete_len..],
                            &mut state.utf8_buffer,
                            cb,
                        )
                    } else {
                        // No more input, or incomplete bytes span buffer boundary
                        incomplete_len
                    };
                }
                VTInputCapture::None
            }
            VTCaptureEvent::VTEvent(vt_event @ VTEvent::Csi(csi)) => {
                if let Some(internal_event) = crate::csi::parse_csi(csi) {
                    match internal_event {
                        crate::event::InternalEvent::Event(event) => {
                            cb(event);
                            VTInputCapture::None
                        }
                        crate::event::InternalEvent::BracketedPasteStart => {
                            state.paste_buffer.clear();
                            VTInputCapture::Terminator(b"\x1b[201~")
                        }
                    }
                } else {
                    // Emit unrecognized CSI as LowLevel event
                    cb(TerminalInputEvent::LowLevel(vt_event));
                    VTInputCapture::None
                }
            }
            VTCaptureEvent::VTEvent(vt_event @ VTEvent::C0(byte)) => {
                cb(crate::c0::parse_c0(*byte).unwrap_or(TerminalInputEvent::LowLevel(vt_event)));
                VTInputCapture::None
            }
            VTCaptureEvent::VTEvent(vt_event @ VTEvent::Esc(esc)) => {
                cb(crate::esc::parse_esc(*esc).unwrap_or(TerminalInputEvent::LowLevel(vt_event)));
                VTInputCapture::None
            }
            VTCaptureEvent::VTEvent(vt_event @ VTEvent::EscInvalid(esc)) => {
                cb(crate::esc::parse_esc_invalid(*esc)
                    .unwrap_or(TerminalInputEvent::LowLevel(vt_event)));
                VTInputCapture::None
            }
            VTCaptureEvent::VTEvent(vt_event @ VTEvent::Ss3(ss3)) => {
                cb(crate::ss::parse_ss3(*ss3).unwrap_or(TerminalInputEvent::LowLevel(vt_event)));
                VTInputCapture::None
            }
            VTCaptureEvent::Capture(data) => {
                state.paste_buffer.extend_from_slice(data);
                VTInputCapture::None
            }
            VTCaptureEvent::CaptureEnd => {
                cb(TerminalInputEvent::Paste(&state.paste_buffer));
                state.paste_buffer.clear();
                VTInputCapture::None
            }
            VTCaptureEvent::VTEvent(vt_event) => {
                cb(TerminalInputEvent::LowLevel(vt_event));
                VTInputCapture::None
            }
        }
    }

    /// Handle idle state by flushing any incomplete escape sequences.
    ///
    /// This method should be called when input has stopped but the parser
    /// may be in the middle of processing an escape sequence. It will emit
    /// any appropriate events for incomplete sequences and reset the parser
    /// to ground state.
    ///
    /// Return `true` if any events were emitted, `false` otherwise.
    pub fn idle<F: FnMut(TerminalInputEvent)>(&mut self, cb: &mut F) -> bool {
        if let Some(vt_event) = self.seq_parser.idle() {
            Self::process_vt_event(&vt_event, &mut self.state, cb);
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::{KeyCode, KeyEvent, KeyModifiers, TerminalInputEvent};

    fn collect_events(input: &[u8]) -> Vec<crate::TerminalInputEventOwned> {
        let mut parser = TerminalInputParser::new();
        let mut events = Vec::new();
        parser.feed_with(input, &mut |event| {
            events.push(event.to_owned());
        });
        parser.idle(&mut |event| {
            events.push(event.to_owned());
        });
        events
    }

    fn collect_events_with_idle(chunks: &[&[u8]]) -> Vec<crate::TerminalInputEventOwned> {
        let mut parser = TerminalInputParser::new();
        let mut events = Vec::new();
        for chunk in chunks {
            parser.feed_with(chunk, &mut |event| {
                events.push(event.to_owned());
            });
            parser.idle(&mut |event| {
                events.push(event.to_owned());
            });
        }
        events
    }

    #[test]
    fn test_basic_text_input() {
        let events = collect_events(b"hello");
        assert_eq!(events.len(), 5);
        for (i, ch) in "hello".chars().enumerate() {
            assert!(matches!(
                &events[i],
                crate::TerminalInputEventOwned::Key(KeyEvent {
                    code: KeyCode::Char(c),
                    modifiers,
                    ..
                }) if *c == ch && modifiers.is_empty()
            ));
        }
    }

    #[test]
    fn test_escape_key_with_idle() {
        // Test that a lone ESC byte followed by idle becomes an Escape key
        // event
        let mut parser = TerminalInputParser::new();
        let mut events = Vec::new();

        parser.feed_with(b"\x1b", &mut |event| {
            events.push(event.to_owned());
        });
        // Without idle, the parser is waiting to see if more bytes follow
        assert_eq!(events.len(), 0);

        parser.idle(&mut |event| {
            events.push(event.to_owned());
        });
        // After idle, the ESC is emitted as an Escape key
        assert_eq!(events.len(), 1);
        assert!(matches!(
            &events[0],
            crate::TerminalInputEventOwned::Key(KeyEvent {
                code: KeyCode::Esc,
                modifiers,
                ..
            }) if modifiers.is_empty()
        ));
    }

    #[test]
    fn test_alt_enter() {
        // ESC followed immediately by Enter should be Alt+Enter
        let events = collect_events(b"\x1b\n");
        assert_eq!(events.len(), 1);
        assert!(matches!(
            &events[0],
            crate::TerminalInputEventOwned::Key(KeyEvent {
                code: KeyCode::Enter,
                modifiers,
                ..
            }) if modifiers.contains(KeyModifiers::ALT)
        ));
    }

    #[test]
    fn test_escape_key_followed_by_text() {
        // ESC with idle, then text - should get separate Escape key and text
        let events = collect_events_with_idle(&[b"\x1b", b"hi\r"]);
        assert_eq!(events.len(), 4);
        assert!(matches!(
            &events[0],
            crate::TerminalInputEventOwned::Key(KeyEvent {
                code: KeyCode::Esc,
                ..
            })
        ));
        assert!(matches!(
            &events[1],
            crate::TerminalInputEventOwned::Key(KeyEvent {
                code: KeyCode::Char('h'),
                ..
            })
        ));
        assert!(matches!(
            &events[2],
            crate::TerminalInputEventOwned::Key(KeyEvent {
                code: KeyCode::Char('i'),
                ..
            })
        ));
        assert!(matches!(
            &events[3],
            crate::TerminalInputEventOwned::Key(KeyEvent {
                code: KeyCode::Enter,
                ..
            })
        ));
    }

    #[test]
    fn test_arrow_key() {
        let events = collect_events(b"\x1b[A");
        assert_eq!(events.len(), 1);
        assert!(matches!(
            &events[0],
            crate::TerminalInputEventOwned::Key(KeyEvent {
                code: KeyCode::Up,
                modifiers,
                ..
            }) if modifiers.is_empty()
        ));
    }

    #[test]
    fn test_emoji_input() {
        let events = collect_events("🤣🛜".as_bytes());
        assert_eq!(events.len(), 2);
        assert!(matches!(
            &events[0],
            crate::TerminalInputEventOwned::Key(KeyEvent {
                code: KeyCode::Char('🤣'),
                ..
            })
        ));
        assert!(matches!(
            &events[1],
            crate::TerminalInputEventOwned::Key(KeyEvent {
                code: KeyCode::Char('🛜'),
                ..
            })
        ));
    }

    #[test]
    fn test_focus_events() {
        let events = collect_events(b"x\x1b[I\x7f");
        assert_eq!(events.len(), 3);
        assert!(matches!(
            &events[0],
            crate::TerminalInputEventOwned::Key(KeyEvent {
                code: KeyCode::Char('x'),
                ..
            })
        ));
        assert!(matches!(
            &events[1],
            crate::TerminalInputEventOwned::Focus(true)
        ));
        assert!(matches!(
            &events[2],
            crate::TerminalInputEventOwned::Key(KeyEvent {
                code: KeyCode::Backspace,
                ..
            })
        ));
    }

    #[test]
    fn test_mouse_events() {
        use crate::event::{MouseEvent, MouseEventKind};

        let events = collect_events(b"\x1b[<35;73;5M\x1b[<35;73;5M");
        assert_eq!(events.len(), 2);
        assert!(matches!(
            &events[0],
            crate::TerminalInputEventOwned::Mouse(MouseEvent {
                kind: MouseEventKind::Moved,
                column: 72,
                row: 4,
                ..
            })
        ));
        assert!(matches!(
            &events[1],
            crate::TerminalInputEventOwned::Mouse(MouseEvent {
                kind: MouseEventKind::Moved,
                column: 72,
                row: 4,
                ..
            })
        ));
    }

    #[test]
    #[cfg(unix)]
    fn test_cursor_position_report() {
        let events = collect_events(b"\x1b[3;1R");
        assert_eq!(events.len(), 1);
        // CSI 3;1R means row 3, column 1 (1-indexed), converted to 0-indexed
        assert!(matches!(
            &events[0],
            crate::TerminalInputEventOwned::CursorPosition(0, 2)
        ));
    }

    #[test]
    #[cfg(unix)]
    fn test_device_attributes() {
        let events = collect_events(b"\x1b[?62;22;52c");
        assert_eq!(events.len(), 1);
        assert!(matches!(
            &events[0],
            crate::TerminalInputEventOwned::PrimaryDeviceAttributes
        ));
    }

    #[test]
    #[cfg(unix)]
    fn test_keyboard_enhancement_flags() {
        let events = collect_events(b"\x1b[?0u");
        assert_eq!(events.len(), 1);
        assert!(matches!(
            &events[0],
            crate::TerminalInputEventOwned::KeyboardEnhancementFlags(_)
        ));
    }

    #[test]
    #[cfg(unix)]
    fn test_keyboard_enhancement_query() {
        let events = collect_events(b"\x1b[?u");
        assert_eq!(events.len(), 1);
        assert!(matches!(
            &events[0],
            crate::TerminalInputEventOwned::KeyboardEnhancementFlagsQuery
        ));
    }

    #[test]
    #[cfg(unix)]
    fn test_keyboard_enhancement_push() {
        let events = collect_events(b"\x1b[>1u");
        assert_eq!(events.len(), 1);
        assert!(matches!(
            &events[0],
            crate::TerminalInputEventOwned::KeyboardEnhancementFlagsPush(_)
        ));
    }

    #[test]
    #[cfg(unix)]
    fn test_keyboard_enhancement_pop() {
        let events = collect_events(b"\x1b[<u");
        assert_eq!(events.len(), 1);
        assert!(matches!(
            &events[0],
            crate::TerminalInputEventOwned::KeyboardEnhancementFlagsPop(1)
        ));
    }

    #[test]
    fn test_low_level_csi_event() {
        // Random CSI that doesn't map to a known event
        let events = collect_events(b"\x1b[12p");
        assert_eq!(events.len(), 1);
        match &events[0] {
            crate::TerminalInputEventOwned::LowLevel(vt_event) => match &**vt_event {
                crate::VTOwnedEvent::Csi(csi) => {
                    assert_eq!(csi.final_byte, b'p');
                }
                _ => panic!("Expected Csi event"),
            },
            _ => panic!("Expected LowLevel event"),
        }
    }

    #[test]
    fn test_osc_sequences() {
        let events = collect_events(b"\x1b]0;test\x07");
        // OSC sequences emit OscStart and OscEnd events
        assert_eq!(events.len(), 2);
        assert!(matches!(
            &events[0],
            crate::TerminalInputEventOwned::LowLevel(vt_event)
                if matches!(**vt_event, crate::VTOwnedEvent::OscStart)
        ));
        assert!(matches!(
            &events[1],
            crate::TerminalInputEventOwned::LowLevel(vt_event)
                if matches!(**vt_event, crate::VTOwnedEvent::OscEnd {
                    used_bel: true,
                    ..
                })
        ));
    }

    #[test]
    fn test_osc_and_keyboard_enhancement() {
        // OSC sequence followed by keyboard enhancement flags
        let events = collect_events(b"\x1b]0;test\x07\x1b[?0u");
        assert_eq!(events.len(), 3);

        // First: OscStart
        assert!(matches!(
            &events[0],
            crate::TerminalInputEventOwned::LowLevel(vt_event)
                if matches!(**vt_event, crate::VTOwnedEvent::OscStart)
        ));

        // Second: OscEnd
        assert!(matches!(
            &events[1],
            crate::TerminalInputEventOwned::LowLevel(vt_event)
                if matches!(**vt_event, crate::VTOwnedEvent::OscEnd {
                    used_bel: true,
                    ..
                })
        ));

        // Third: KeyboardEnhancementFlags
        #[cfg(unix)]
        assert!(matches!(
            &events[2],
            crate::TerminalInputEventOwned::KeyboardEnhancementFlags(_)
        ));
    }

    #[test]
    #[cfg(unix)]
    fn test_complex_sequence_combination() {
        // Test cursor position, device attributes, and OSC sequences
        let events = collect_events(
            b"\x1b[3;1R\x1b[>1;10;0c\x1b]10;rgb:ffff/ffff/ffff\x07\x1b]11;rgb:2828/2c2c/3434\x07",
        );

        // Should receive cursor position, device attributes, and 2 OSC
        // sequences (each as Start+End)
        assert_eq!(events.len(), 6);

        // First: CSI 3;1R (cursor position) - 3;1 means row 3, column 1
        // (0-indexed as col 0, row 2)
        assert!(matches!(
            &events[0],
            crate::TerminalInputEventOwned::CursorPosition(0, 2)
        ));

        // Second: CSI >1;10;0c (device attributes) - becomes LowLevel as
        // it's unrecognized
        match &events[1] {
            crate::TerminalInputEventOwned::LowLevel(vt_event) => match &**vt_event {
                crate::VTOwnedEvent::Csi(csi) => {
                    assert_eq!(csi.private, Some(b'>'));
                    assert_eq!(csi.final_byte, b'c');
                }
                _ => panic!("Expected Csi event"),
            },
            _ => panic!("Expected LowLevel event"),
        }

        // Third and fourth: First OSC sequence (OscStart + OscEnd)
        assert!(matches!(
            &events[2],
            crate::TerminalInputEventOwned::LowLevel(vt_event)
                if matches!(**vt_event, crate::VTOwnedEvent::OscStart)
        ));
        assert!(matches!(
            &events[3],
            crate::TerminalInputEventOwned::LowLevel(vt_event)
                if matches!(**vt_event, crate::VTOwnedEvent::OscEnd {
                    used_bel: true,
                    ..
                })
        ));

        // Fifth and sixth: Second OSC sequence (OscStart + OscEnd)
        assert!(matches!(
            &events[4],
            crate::TerminalInputEventOwned::LowLevel(vt_event)
                if matches!(**vt_event, crate::VTOwnedEvent::OscStart)
        ));
        assert!(matches!(
            &events[5],
            crate::TerminalInputEventOwned::LowLevel(vt_event)
                if matches!(**vt_event, crate::VTOwnedEvent::OscEnd {
                    used_bel: true,
                    ..
                })
        ));
    }

    #[test]
    #[cfg(unix)]
    fn test_vi_startup_sequence() {
        // Typical vi startup sequence with OSC, keyboard enhancement, and
        // device attributes
        let events = collect_events(b"\x1b]11;rgb:2828/2c2c/3434\x07\x1b[?0u\x1b[?62;22;52c");

        // OSC sequence (Start+End), KeyboardEnhancementFlags, and device
        // attributes
        assert_eq!(events.len(), 4);

        // First and second: OSC sequence (OscStart + OscEnd)
        assert!(matches!(
            &events[0],
            crate::TerminalInputEventOwned::LowLevel(vt_event)
                if matches!(**vt_event, crate::VTOwnedEvent::OscStart)
        ));
        assert!(matches!(
            &events[1],
            crate::TerminalInputEventOwned::LowLevel(vt_event)
                if matches!(**vt_event, crate::VTOwnedEvent::OscEnd {
                    used_bel: true,
                    ..
                })
        ));

        // Third: KeyboardEnhancementFlags (CSI ?0u)
        assert!(matches!(
            &events[2],
            crate::TerminalInputEventOwned::KeyboardEnhancementFlags(_)
        ));

        // Fourth: PrimaryDeviceAttributes (CSI ?62;22;52c)
        assert!(matches!(
            &events[3],
            crate::TerminalInputEventOwned::PrimaryDeviceAttributes
        ));
    }

    #[test]
    fn test_bracketed_paste_basic() {
        let events = collect_events(b"\x1b[200~Hello World\x1b[201~");
        assert_eq!(events.len(), 1);
        assert!(matches!(
            &events[0],
            crate::TerminalInputEventOwned::Paste(data) if data == b"Hello World"
        ));
    }

    #[test]
    fn test_bracketed_paste_with_escape_sequences() {
        // Test that escape sequences inside paste are treated as raw content
        let mut parser = TerminalInputParser::new();
        let mut paste_data = None;

        // Paste containing an arrow key sequence - should be treated as raw
        // bytes
        let input = b"\x1b[200~text\x1b[A\x1b[201~";
        parser.feed_with(input, &mut |event| {
            if let TerminalInputEvent::Paste(data) = event {
                paste_data = Some(data.to_vec());
            }
        });

        // The ESC[A should be included as raw content in the paste
        assert_eq!(paste_data, Some(b"text\x1b[A".to_vec()));
    }

    #[test]
    fn test_bracketed_paste_with_csi_sequence() {
        // Test that CSI sequences in paste are treated as raw content
        let mut parser = TerminalInputParser::new();
        let mut paste_data = None;

        // Start paste, add some data including CSI sequence, then end paste
        let input = b"\x1b[200~partial\x1b[A\x1b[201~";
        parser.feed_with(input, &mut |event| {
            if let TerminalInputEvent::Paste(data) = event {
                paste_data = Some(data.to_vec());
            }
        });

        // Should emit paste with the CSI sequence as literal data
        assert_eq!(paste_data, Some(b"partial\x1b[A".to_vec()));
    }

    #[test]
    fn test_bracketed_paste_with_ss3_sequence() {
        // Test that SS3 sequences in paste are treated as raw content
        let mut parser = TerminalInputParser::new();
        let mut paste_data = None;

        // Start paste, add data including SS3 sequence, then end paste
        let input = b"\x1b[200~some text\x1bOH\x1b[201~";
        parser.feed_with(input, &mut |event| {
            if let TerminalInputEvent::Paste(data) = event {
                paste_data = Some(data.to_vec());
            }
        });

        // ESC O H should be included as raw content, not interpreted as Home
        // key
        assert_eq!(paste_data, Some(b"some text\x1bOH".to_vec()));
    }

    #[test]
    fn test_bracketed_paste_with_newlines() {
        let events = collect_events(b"\x1b[200~Line1\nLine2\r\nLine3\tTab\x1b[201~");
        assert_eq!(events.len(), 1);
        assert!(matches!(
            &events[0],
            crate::TerminalInputEventOwned::Paste(data)
                if data == b"Line1\nLine2\r\nLine3\tTab"
        ));
    }

    #[test]
    fn test_bracketed_paste_large_content() {
        let long_content = "A".repeat(10000);
        let mut test_data = b"\x1b[200~".to_vec();
        test_data.extend_from_slice(long_content.as_bytes());
        test_data.extend_from_slice(b"\x1b[201~");

        let events = collect_events(&test_data);
        assert_eq!(events.len(), 1);
        assert!(matches!(
            &events[0],
            crate::TerminalInputEventOwned::Paste(data)
                if data == long_content.as_bytes()
        ));
    }

    #[test]
    fn test_bracketed_paste_multiple_chunks() {
        // Test chunked paste input
        let mut parser = TerminalInputParser::new();
        let mut events = Vec::new();

        parser.feed_with(b"\x1b[200~Chunk1", &mut |event| {
            events.push(event.to_owned());
        });
        parser.feed_with(b"Chunk2\x1b[201~", &mut |event| {
            events.push(event.to_owned());
        });

        assert_eq!(events.len(), 1);
        assert!(matches!(
            &events[0],
            crate::TerminalInputEventOwned::Paste(data)
                if data == b"Chunk1Chunk2"
        ));
    }

    #[test]
    fn test_incomplete_utf8_across_chunks() {
        // Test UTF-8 handling when a multibyte character is split across
        // chunks
        let mut parser = TerminalInputParser::new();
        let mut events = Vec::new();

        // Split the emoji '🤣' (F0 9F A4 A3) across two chunks
        parser.feed_with(&[0xF0, 0x9F], &mut |event| {
            events.push(event.to_owned());
        });
        // Should not emit anything yet
        assert_eq!(events.len(), 0);

        parser.feed_with(&[0xA4, 0xA3], &mut |event| {
            events.push(event.to_owned());
        });
        // Now should emit the complete character
        assert_eq!(events.len(), 1);
        assert!(matches!(
            &events[0],
            crate::TerminalInputEventOwned::Key(KeyEvent {
                code: KeyCode::Char('🤣'),
                ..
            })
        ));
    }

    #[test]
    fn test_idle_resets_incomplete_sequence() {
        // Test that calling idle handles incomplete sequences properly
        let mut parser = TerminalInputParser::new();
        let mut events = Vec::new();

        // Start an OSC sequence but don't complete it
        parser.feed_with(b"\x1b]10;rgb:ffff/ffff/ffff", &mut |event| {
            events.push(event.to_owned());
        });

        // Should have OscStart and potentially OscData
        assert!(!events.is_empty());
        assert!(matches!(
            &events[0],
            crate::TerminalInputEventOwned::LowLevel(vt_event)
                if matches!(**vt_event, crate::VTOwnedEvent::OscStart)
        ));

        events.clear();

        // Call idle - incomplete OSC may produce OscData
        parser.idle(&mut |event| {
            events.push(event.to_owned());
        });

        events.clear();

        // Additional input after idle
        parser.feed_with(b"\x1b[3;1R", &mut |event| {
            events.push(event.to_owned());
        });

        // Should have at least one event
        assert!(!events.is_empty());
    }
}
