//! CSI (Control Sequence Introducer) sequence parsing.
//!
//! This module provides functions to parse CSI sequences from terminal
//! input into terminal events.

use vt_push_parser::event::{CSI, ParamBuf};

use crate::event::{
    InternalEvent, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers, MediaKeyCode,
    ModifierKeyCode, MouseButton, MouseEvent, MouseEventKind, TerminalInputEvent,
};

/// Parse a CSI sequence into a terminal event.
///
/// This function handles all CSI sequences that represent keyboard, mouse,
/// and other terminal events.
#[allow(clippy::too_many_lines)]
pub(crate) fn parse_csi<'a>(csi: &CSI<'a>) -> Option<InternalEvent<'a>> {
    // Handle sequences with no parameters first
    if csi.params.is_empty() && csi.intermediates.is_empty() && csi.private.is_none() {
        return match csi.final_byte {
            b'D' => Some(InternalEvent::Event(TerminalInputEvent::Key(
                KeyCode::Left.into(),
            ))),
            b'C' => Some(InternalEvent::Event(TerminalInputEvent::Key(
                KeyCode::Right.into(),
            ))),
            b'A' => Some(InternalEvent::Event(TerminalInputEvent::Key(
                KeyCode::Up.into(),
            ))),
            b'B' => Some(InternalEvent::Event(TerminalInputEvent::Key(
                KeyCode::Down.into(),
            ))),
            b'H' => Some(InternalEvent::Event(TerminalInputEvent::Key(
                KeyCode::Home.into(),
            ))),
            b'F' => Some(InternalEvent::Event(TerminalInputEvent::Key(
                KeyCode::End.into(),
            ))),
            b'Z' => Some(InternalEvent::Event(TerminalInputEvent::Key(
                KeyEvent::new_with_kind(KeyCode::BackTab, KeyModifiers::SHIFT, KeyEventKind::Press),
            ))),
            b'M' => parse_csi_normal_mouse(csi.params),
            b'I' => Some(InternalEvent::Event(TerminalInputEvent::Focus(true))),
            b'O' => Some(InternalEvent::Event(TerminalInputEvent::Focus(false))),
            // P, Q, and S for compatibility with Kitty keyboard protocol
            b'P' => Some(InternalEvent::Event(TerminalInputEvent::Key(
                KeyCode::F(1).into(),
            ))),
            b'Q' => Some(InternalEvent::Event(TerminalInputEvent::Key(
                KeyCode::F(2).into(),
            ))),
            b'S' => Some(InternalEvent::Event(TerminalInputEvent::Key(
                KeyCode::F(4).into(),
            ))),
            _ => None,
        };
    }

    // Handle private marker sequences
    if let Some(b'<') = csi.private {
        // SGR mouse uses final byte 'M' or 'm', keyboard protocol uses 'u'
        if csi.final_byte == b'u' {
            #[cfg(unix)]
            {
                // CSI < number u - Pop keyboard enhancement flags
                let count = if csi.params.is_empty() {
                    1
                } else {
                    csi.params.try_parse::<u16>(0)?
                };
                return Some(InternalEvent::Event(
                    TerminalInputEvent::KeyboardEnhancementFlagsPop(count),
                ));
            }
            #[cfg(not(unix))]
            return None;
        }
        return parse_csi_sgr_mouse(csi);
    }

    if let Some(b'?') = csi.private {
        return match csi.final_byte {
            #[cfg(unix)]
            b'u' => {
                if csi.params.is_empty() {
                    // CSI ? u - Query keyboard enhancement flags
                    Some(InternalEvent::Event(
                        TerminalInputEvent::KeyboardEnhancementFlagsQuery,
                    ))
                } else {
                    // CSI ? flags u - Response with keyboard enhancement flags
                    parse_csi_keyboard_enhancement_flags(csi.params)
                }
            }
            #[cfg(unix)]
            b'c' => Some(InternalEvent::Event(
                TerminalInputEvent::PrimaryDeviceAttributes,
            )),
            _ => None,
        };
    }

    #[cfg(unix)]
    if let Some(b'>') = csi.private {
        return match csi.final_byte {
            b'u' => {
                // CSI > flags u - Push keyboard enhancement flags
                let flags = if csi.params.is_empty() {
                    None
                } else {
                    Some(parse_keyboard_enhancement_flags_from_params(csi.params)?)
                };
                Some(InternalEvent::Event(
                    TerminalInputEvent::KeyboardEnhancementFlagsPush(flags),
                ))
            }
            _ => None,
        };
    }

    // Handle sequences with intermediates
    if csi.intermediates.has(b' ') && csi.final_byte == b'u' {
        // CSI SP u is used for some special sequences
        return None;
    }

    // Handle numbered sequences
    match csi.final_byte {
        b'M' => parse_csi_rxvt_mouse(csi),
        b'~' => parse_csi_special_key_or_bracketed_paste(csi),
        b'u' => parse_csi_u_encoded_key_code(csi),
        #[cfg(unix)]
        b'R' => parse_csi_cursor_position(csi),
        b'A' | b'B' | b'C' | b'D' | b'F' | b'H' | b'P' | b'Q' | b'S' => {
            parse_csi_modifier_key_code(csi)
        }
        _ => None,
    }
}

/// Parse modifier mask from parameter.
///
/// The modifier mask is encoded as a number where:
/// - 1 = no modifiers
/// - 2 = Shift
/// - 3 = Alt
/// - 4 = Shift + Alt
/// - 5 = Control
/// - etc.
fn parse_modifiers(mask: u8) -> KeyModifiers {
    let modifier_mask = mask.saturating_sub(1);
    let mut modifiers = KeyModifiers::empty();
    if modifier_mask & 1 != 0 {
        modifiers |= KeyModifiers::SHIFT;
    }
    if modifier_mask & 2 != 0 {
        modifiers |= KeyModifiers::ALT;
    }
    if modifier_mask & 4 != 0 {
        modifiers |= KeyModifiers::CONTROL;
    }
    if modifier_mask & 8 != 0 {
        modifiers |= KeyModifiers::SUPER;
    }
    if modifier_mask & 16 != 0 {
        modifiers |= KeyModifiers::HYPER;
    }
    if modifier_mask & 32 != 0 {
        modifiers |= KeyModifiers::META;
    }
    modifiers
}

/// Parse modifier mask to key event state (Caps Lock, Num Lock).
fn parse_modifiers_to_state(mask: u8) -> KeyEventState {
    let modifier_mask = mask.saturating_sub(1);
    let mut state = KeyEventState::empty();
    if modifier_mask & 64 != 0 {
        state |= KeyEventState::CAPS_LOCK;
    }
    if modifier_mask & 128 != 0 {
        state |= KeyEventState::NUM_LOCK;
    }
    state
}

/// Parse key event kind from code.
fn parse_key_event_kind(kind: u8) -> KeyEventKind {
    match kind {
        2 => KeyEventKind::Repeat,
        3 => KeyEventKind::Release,
        _ => KeyEventKind::Press,
    }
}

/// Parse modifier and kind from a sub-parameter (colon-separated).
///
/// Format: `modifier_mask:kind_code`
fn parse_modifier_and_kind(param: &[u8]) -> Option<(u8, u8)> {
    let s = std::str::from_utf8(param).ok()?;
    let mut parts = s.split(':');
    let modifier_mask = parts.next()?.parse::<u8>().ok()?;
    let kind_code = parts.next().and_then(|s| s.parse::<u8>().ok()).unwrap_or(1);
    Some((modifier_mask, kind_code))
}

/// Parse CSI sequences with modifier key codes.
///
/// Format: `CSI modifier ; keycode` or `CSI keycode`
/// Examples: `CSI 1 ; 2 A` (Shift+Up), `CSI 5 A` (Ctrl+Up)
fn parse_csi_modifier_key_code<'a>(csi: &CSI<'a>) -> Option<InternalEvent<'a>> {
    let (modifiers, kind) = if csi.params.len() >= 2 {
        let modifier_param = csi.params.get(1)?;
        if let Some((modifier_mask, kind_code)) = parse_modifier_and_kind(modifier_param) {
            (
                parse_modifiers(modifier_mask),
                parse_key_event_kind(kind_code),
            )
        } else {
            // Legacy format: just a single digit for modifier
            let modifier_mask = csi.params.try_parse::<u8>(1)?;
            (parse_modifiers(modifier_mask), KeyEventKind::Press)
        }
    } else {
        (KeyModifiers::NONE, KeyEventKind::Press)
    };

    let keycode = match csi.final_byte {
        b'A' => KeyCode::Up,
        b'B' => KeyCode::Down,
        b'C' => KeyCode::Right,
        b'D' => KeyCode::Left,
        b'F' => KeyCode::End,
        b'H' => KeyCode::Home,
        b'P' => KeyCode::F(1),
        b'Q' => KeyCode::F(2),
        b'S' => KeyCode::F(4),
        _ => return None,
    };

    Some(InternalEvent::Event(TerminalInputEvent::Key(
        KeyEvent::new_with_kind(keycode, modifiers, kind),
    )))
}

/// Parse CSI special key codes or bracketed paste markers (ending with ~).
///
/// Format: `CSI number ; modifier ; kind ~`
/// Examples: `CSI 1 ~` (Home), `CSI 2 ; 2 ~` (Shift+Insert)
/// Bracketed paste: `CSI 200 ~` (start), `CSI 201 ~` (end)
fn parse_csi_special_key_or_bracketed_paste<'a>(csi: &CSI<'a>) -> Option<InternalEvent<'a>> {
    let first = csi.params.try_parse::<u16>(0)?;

    // Handle bracketed paste markers
    if first == 200 {
        return Some(InternalEvent::BracketedPasteStart);
    }

    // Delegate to special key code parser for regular keys
    parse_csi_special_key_code(csi.params)
}

/// Parse CSI special key codes (ending with ~).
///
/// Format: `CSI number ; modifier ; kind ~`
/// Examples: `CSI 1 ~` (Home), `CSI 2 ; 2 ~` (Shift+Insert)
fn parse_csi_special_key_code(params: ParamBuf<'_>) -> Option<InternalEvent<'_>> {
    let first = params.try_parse::<u8>(0)?;

    let (modifiers, kind, state) = if params.len() >= 2 {
        let modifier_param = params.get(1)?;
        if let Some((modifier_mask, kind_code)) = parse_modifier_and_kind(modifier_param) {
            (
                parse_modifiers(modifier_mask),
                parse_key_event_kind(kind_code),
                parse_modifiers_to_state(modifier_mask),
            )
        } else {
            (KeyModifiers::NONE, KeyEventKind::Press, KeyEventState::NONE)
        }
    } else {
        (KeyModifiers::NONE, KeyEventKind::Press, KeyEventState::NONE)
    };

    let keycode = match first {
        1 | 7 => KeyCode::Home,
        2 => KeyCode::Insert,
        3 => KeyCode::Delete,
        4 | 8 => KeyCode::End,
        5 => KeyCode::PageUp,
        6 => KeyCode::PageDown,
        v @ 11..=15 => KeyCode::F(v - 10),
        v @ 17..=21 => KeyCode::F(v - 11),
        v @ 23..=26 => KeyCode::F(v - 12),
        v @ 28..=29 => KeyCode::F(v - 15),
        v @ 31..=34 => KeyCode::F(v - 17),
        _ => return None,
    };

    Some(InternalEvent::Event(TerminalInputEvent::Key(
        KeyEvent::new_with_kind_and_state(keycode, modifiers, kind, state),
    )))
}

/// Translate functional key code from Kitty keyboard protocol.
///
/// These are special codepoints used by the Kitty keyboard protocol for
/// keys that don't have standard Unicode representations.
fn translate_functional_key_code(codepoint: u32) -> Option<(KeyCode, KeyEventState)> {
    if let Some(keycode) = match codepoint {
        57399 => Some(KeyCode::Char('0')),
        57400 => Some(KeyCode::Char('1')),
        57401 => Some(KeyCode::Char('2')),
        57402 => Some(KeyCode::Char('3')),
        57403 => Some(KeyCode::Char('4')),
        57404 => Some(KeyCode::Char('5')),
        57405 => Some(KeyCode::Char('6')),
        57406 => Some(KeyCode::Char('7')),
        57407 => Some(KeyCode::Char('8')),
        57408 => Some(KeyCode::Char('9')),
        57409 => Some(KeyCode::Char('.')),
        57410 => Some(KeyCode::Char('/')),
        57411 => Some(KeyCode::Char('*')),
        57412 => Some(KeyCode::Char('-')),
        57413 => Some(KeyCode::Char('+')),
        57414 => Some(KeyCode::Enter),
        57415 => Some(KeyCode::Char('=')),
        57416 => Some(KeyCode::Char(',')),
        57417 => Some(KeyCode::Left),
        57418 => Some(KeyCode::Right),
        57419 => Some(KeyCode::Up),
        57420 => Some(KeyCode::Down),
        57421 => Some(KeyCode::PageUp),
        57422 => Some(KeyCode::PageDown),
        57423 => Some(KeyCode::Home),
        57424 => Some(KeyCode::End),
        57425 => Some(KeyCode::Insert),
        57426 => Some(KeyCode::Delete),
        57427 => Some(KeyCode::KeypadBegin),
        _ => None,
    } {
        return Some((keycode, KeyEventState::KEYPAD));
    }

    if let Some(keycode) = match codepoint {
        57358 => Some(KeyCode::CapsLock),
        57359 => Some(KeyCode::ScrollLock),
        57360 => Some(KeyCode::NumLock),
        57361 => Some(KeyCode::PrintScreen),
        57362 => Some(KeyCode::Pause),
        57363 => Some(KeyCode::Menu),
        57376 => Some(KeyCode::F(13)),
        57377 => Some(KeyCode::F(14)),
        57378 => Some(KeyCode::F(15)),
        57379 => Some(KeyCode::F(16)),
        57380 => Some(KeyCode::F(17)),
        57381 => Some(KeyCode::F(18)),
        57382 => Some(KeyCode::F(19)),
        57383 => Some(KeyCode::F(20)),
        57384 => Some(KeyCode::F(21)),
        57385 => Some(KeyCode::F(22)),
        57386 => Some(KeyCode::F(23)),
        57387 => Some(KeyCode::F(24)),
        57388 => Some(KeyCode::F(25)),
        57389 => Some(KeyCode::F(26)),
        57390 => Some(KeyCode::F(27)),
        57391 => Some(KeyCode::F(28)),
        57392 => Some(KeyCode::F(29)),
        57393 => Some(KeyCode::F(30)),
        57394 => Some(KeyCode::F(31)),
        57395 => Some(KeyCode::F(32)),
        57396 => Some(KeyCode::F(33)),
        57397 => Some(KeyCode::F(34)),
        57398 => Some(KeyCode::F(35)),
        57428 => Some(KeyCode::Media(MediaKeyCode::Play)),
        57429 => Some(KeyCode::Media(MediaKeyCode::Pause)),
        57430 => Some(KeyCode::Media(MediaKeyCode::PlayPause)),
        57431 => Some(KeyCode::Media(MediaKeyCode::Reverse)),
        57432 => Some(KeyCode::Media(MediaKeyCode::Stop)),
        57433 => Some(KeyCode::Media(MediaKeyCode::FastForward)),
        57434 => Some(KeyCode::Media(MediaKeyCode::Rewind)),
        57435 => Some(KeyCode::Media(MediaKeyCode::TrackNext)),
        57436 => Some(KeyCode::Media(MediaKeyCode::TrackPrevious)),
        57437 => Some(KeyCode::Media(MediaKeyCode::Record)),
        57438 => Some(KeyCode::Media(MediaKeyCode::LowerVolume)),
        57439 => Some(KeyCode::Media(MediaKeyCode::RaiseVolume)),
        57440 => Some(KeyCode::Media(MediaKeyCode::MuteVolume)),
        57441 => Some(KeyCode::Modifier(ModifierKeyCode::LeftShift)),
        57442 => Some(KeyCode::Modifier(ModifierKeyCode::LeftControl)),
        57443 => Some(KeyCode::Modifier(ModifierKeyCode::LeftAlt)),
        57444 => Some(KeyCode::Modifier(ModifierKeyCode::LeftSuper)),
        57445 => Some(KeyCode::Modifier(ModifierKeyCode::LeftHyper)),
        57446 => Some(KeyCode::Modifier(ModifierKeyCode::LeftMeta)),
        57447 => Some(KeyCode::Modifier(ModifierKeyCode::RightShift)),
        57448 => Some(KeyCode::Modifier(ModifierKeyCode::RightControl)),
        57449 => Some(KeyCode::Modifier(ModifierKeyCode::RightAlt)),
        57450 => Some(KeyCode::Modifier(ModifierKeyCode::RightSuper)),
        57451 => Some(KeyCode::Modifier(ModifierKeyCode::RightHyper)),
        57452 => Some(KeyCode::Modifier(ModifierKeyCode::RightMeta)),
        57453 => Some(KeyCode::Modifier(ModifierKeyCode::IsoLevel3Shift)),
        57454 => Some(KeyCode::Modifier(ModifierKeyCode::IsoLevel5Shift)),
        _ => None,
    } {
        return Some((keycode, KeyEventState::empty()));
    }

    None
}

/// Parse CSI u encoded key code (Kitty keyboard protocol and CSI u).
///
/// Format: `CSI codepoint ; modifiers u` or
///         `CSI codepoint:shifted:base-layout ; modifiers:kind ; text u`
#[allow(clippy::too_many_lines)]
fn parse_csi_u_encoded_key_code<'a>(csi: &CSI<'a>) -> Option<InternalEvent<'a>> {
    let first_param = csi.params.get(0)?;
    let s = std::str::from_utf8(first_param).ok()?;
    let mut codepoints = s.split(':');

    let codepoint = codepoints.next()?.parse::<u32>().ok()?;

    let (mut modifiers, kind, state_from_modifiers) = if csi.params.len() >= 2 {
        let modifier_param = csi.params.get(1)?;
        if let Some((modifier_mask, kind_code)) = parse_modifier_and_kind(modifier_param) {
            (
                parse_modifiers(modifier_mask),
                parse_key_event_kind(kind_code),
                parse_modifiers_to_state(modifier_mask),
            )
        } else {
            (KeyModifiers::NONE, KeyEventKind::Press, KeyEventState::NONE)
        }
    } else {
        (KeyModifiers::NONE, KeyEventKind::Press, KeyEventState::NONE)
    };

    // Parse text-as-codepoints from the third parameter
    // Format: CSI key-code ; modifiers ; text-codepoints u
    // where text-codepoints is a colon-separated list of Unicode codepoints
    // Note: Empty parameters still count, so `CSI 97;2;;65u` has 4 parameters:
    //   params[0] = "97", params[1] = "2", params[2] = "", params[3] = "65"
    let text = if csi.params.len() >= 4 {
        let text_param = csi.params.get(3)?;
        let text_str = std::str::from_utf8(text_param).ok()?;
        if text_str.is_empty() {
            None
        } else {
            let mut result = String::new();
            for cp_str in text_str.split(':') {
                if !cp_str.is_empty()
                    && let Ok(cp) = cp_str.parse::<u32>()
                    && let Some(c) = char::from_u32(cp)
                {
                    result.push(c);
                }
            }
            if result.is_empty() {
                None
            } else {
                Some(result)
            }
        }
    } else {
        None
    };

    let (mut keycode, state_from_keycode) = {
        if let Some((special_key_code, state)) = translate_functional_key_code(codepoint) {
            (special_key_code, state)
        } else if let Some(c) = char::from_u32(codepoint) {
            (
                match c {
                    '\x1B' => KeyCode::Esc,
                    '\r' => KeyCode::Enter,
                    '\t' => {
                        if modifiers.contains(KeyModifiers::SHIFT) {
                            KeyCode::BackTab
                        } else {
                            KeyCode::Tab
                        }
                    }
                    '\x7F' => KeyCode::Backspace,
                    _ => KeyCode::Char(c),
                },
                KeyEventState::empty(),
            )
        } else {
            return None;
        }
    };

    // Handle modifier key codes
    if let KeyCode::Modifier(modifier_keycode) = keycode {
        match modifier_keycode {
            ModifierKeyCode::LeftAlt | ModifierKeyCode::RightAlt => {
                modifiers.set(KeyModifiers::ALT, true);
            }
            ModifierKeyCode::LeftControl | ModifierKeyCode::RightControl => {
                modifiers.set(KeyModifiers::CONTROL, true);
            }
            ModifierKeyCode::LeftShift | ModifierKeyCode::RightShift => {
                modifiers.set(KeyModifiers::SHIFT, true);
            }
            ModifierKeyCode::LeftSuper | ModifierKeyCode::RightSuper => {
                modifiers.set(KeyModifiers::SUPER, true);
            }
            ModifierKeyCode::LeftHyper | ModifierKeyCode::RightHyper => {
                modifiers.set(KeyModifiers::HYPER, true);
            }
            ModifierKeyCode::LeftMeta | ModifierKeyCode::RightMeta => {
                modifiers.set(KeyModifiers::META, true);
            }
            _ => {}
        }
    }

    // Parse alternate keys: shifted key and/or base layout key
    // Format: codepoint:shifted:base-layout
    // - If only one alternate is present, it's the shifted key
    // - If base layout only (no shifted), format is codepoint::base-layout
    let shifted_key = codepoints.next();
    let base_layout_key_str = codepoints.next();

    // Handle shifted character from alternate key codes
    if modifiers.contains(KeyModifiers::SHIFT)
        && let Some(shifted_str) = shifted_key
        && !shifted_str.is_empty()
        && let Some(shifted_c) = shifted_str.parse::<u32>().ok().and_then(char::from_u32)
    {
        keycode = KeyCode::Char(shifted_c);
        modifiers.set(KeyModifiers::SHIFT, false);
    }

    // Parse base layout key for cross-layout shortcut matching
    let base_layout_key = if let Some(base_str) = base_layout_key_str {
        if base_str.is_empty() {
            None
        } else {
            base_str.parse::<u32>().ok().and_then(|cp| {
                if let Some((special_key_code, _)) = translate_functional_key_code(cp) {
                    Some(special_key_code)
                } else {
                    char::from_u32(cp).map(|c| match c {
                        '\x1B' => KeyCode::Esc,
                        '\r' => KeyCode::Enter,
                        '\t' => KeyCode::Tab,
                        '\x7F' => KeyCode::Backspace,
                        _ => KeyCode::Char(c),
                    })
                }
            })
        }
    } else if shifted_key.is_some() && shifted_key != Some("") {
        // If we have a non-empty shifted key and no base layout key,
        // the shifted key might actually be the base layout (when shift is not pressed)
        // This handles the case: codepoint:base-layout (single alternate)
        None
    } else {
        None
    };

    Some(InternalEvent::Event(TerminalInputEvent::Key(
        KeyEvent::new_with_text(
            keycode,
            modifiers,
            kind,
            state_from_keycode | state_from_modifiers,
            base_layout_key,
            text,
        ),
    )))
}

/// Parse CSI cursor position response.
///
/// Format: `CSI row ; col R`
#[cfg(unix)]
fn parse_csi_cursor_position<'a>(csi: &CSI<'a>) -> Option<InternalEvent<'a>> {
    let y = csi.params.try_parse::<u16>(0)?.saturating_sub(1);
    let x = csi.params.try_parse::<u16>(1)?.saturating_sub(1);
    Some(InternalEvent::Event(TerminalInputEvent::CursorPosition(
        x, y,
    )))
}

/// Parse CSI keyboard enhancement flags response.
///
/// Format: `CSI ? flags u`
#[cfg(unix)]
fn parse_keyboard_enhancement_flags_from_params(
    params: ParamBuf<'_>,
) -> Option<crate::event::KeyboardEnhancementFlags> {
    use crate::event::KeyboardEnhancementFlags;

    let bits = params.try_parse::<u8>(0)?;
    let mut flags = KeyboardEnhancementFlags::empty();

    if bits & 1 != 0 {
        flags |= KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES;
    }
    if bits & 2 != 0 {
        flags |= KeyboardEnhancementFlags::REPORT_EVENT_TYPES;
    }
    if bits & 4 != 0 {
        flags |= KeyboardEnhancementFlags::REPORT_ALTERNATE_KEYS;
    }
    if bits & 8 != 0 {
        flags |= KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES;
    }
    if bits & 16 != 0 {
        flags |= KeyboardEnhancementFlags::REPORT_ASSOCIATED_TEXT;
    }

    Some(flags)
}

#[cfg(unix)]
fn parse_csi_keyboard_enhancement_flags(params: ParamBuf<'_>) -> Option<InternalEvent<'_>> {
    let flags = parse_keyboard_enhancement_flags_from_params(params)?;
    Some(InternalEvent::Event(
        TerminalInputEvent::KeyboardEnhancementFlags(flags),
    ))
}

/// Parse mouse button and modifiers from control byte.
///
/// This is used by both rxvt and normal mouse encodings.
fn parse_mouse_cb(cb: u8) -> Option<(MouseEventKind, KeyModifiers)> {
    let button_number = (cb & 0b0000_0011) | ((cb & 0b1100_0000) >> 4);
    let dragging = cb & 0b0010_0000 == 0b0010_0000;

    let kind = match (button_number, dragging) {
        (0, false) => MouseEventKind::Down(MouseButton::Left),
        (1, false) => MouseEventKind::Down(MouseButton::Middle),
        (2, false) => MouseEventKind::Down(MouseButton::Right),
        (0, true) => MouseEventKind::Drag(MouseButton::Left),
        (1, true) => MouseEventKind::Drag(MouseButton::Middle),
        (2, true) => MouseEventKind::Drag(MouseButton::Right),
        (3, false) => MouseEventKind::Up(MouseButton::Left),
        (3..=5, true) => MouseEventKind::Moved,
        (4, false) => MouseEventKind::ScrollUp,
        (5, false) => MouseEventKind::ScrollDown,
        (6, false) => MouseEventKind::ScrollLeft,
        (7, false) => MouseEventKind::ScrollRight,
        _ => return None,
    };

    let mut modifiers = KeyModifiers::empty();

    if cb & 0b0000_0100 == 0b0000_0100 {
        modifiers |= KeyModifiers::SHIFT;
    }
    if cb & 0b0000_1000 == 0b0000_1000 {
        modifiers |= KeyModifiers::ALT;
    }
    if cb & 0b0001_0000 == 0b0001_0000 {
        modifiers |= KeyModifiers::CONTROL;
    }

    Some((kind, modifiers))
}

/// Parse CSI rxvt mouse event.
///
/// Format: `CSI Cb ; Cx ; Cy M`
fn parse_csi_rxvt_mouse<'a>(csi: &CSI<'a>) -> Option<InternalEvent<'a>> {
    let cb = csi.params.try_parse::<u8>(0)?.checked_sub(32)?;
    let (kind, modifiers) = parse_mouse_cb(cb)?;

    let column = csi.params.try_parse::<u16>(1)?.saturating_sub(1);
    let row = csi.params.try_parse::<u16>(2)?.saturating_sub(1);

    Some(InternalEvent::Event(TerminalInputEvent::Mouse(
        MouseEvent {
            kind,
            column,
            row,
            modifiers,
        },
    )))
}

/// Parse CSI normal mouse event.
///
/// Format: `CSI M CB Cx Cy` (6 bytes total, with raw bytes for CB, Cx, Cy)
fn parse_csi_normal_mouse(_params: ParamBuf<'_>) -> Option<InternalEvent<'_>> {
    // Normal mouse encoding requires exactly 3 bytes after CSI M
    // This is handled by the VTPushParser as raw bytes, not as params
    // So this function won't be called in the typical case
    None
}

/// Parse CSI SGR mouse event.
///
/// Format: `CSI < Cb ; Cx ; Cy M` or `CSI < Cb ; Cx ; Cy m`
fn parse_csi_sgr_mouse<'a>(csi: &CSI<'a>) -> Option<InternalEvent<'a>> {
    if csi.final_byte != b'M' && csi.final_byte != b'm' {
        return None;
    }

    let cb = csi.params.try_parse::<u8>(0)?;
    let (kind, modifiers) = parse_mouse_cb(cb)?;

    let column = csi.params.try_parse::<u16>(1)?.saturating_sub(1);
    let row = csi.params.try_parse::<u16>(2)?.saturating_sub(1);

    // Lowercase 'm' indicates button release in SGR mode
    let kind = if csi.final_byte == b'm' {
        match kind {
            MouseEventKind::Down(button) => MouseEventKind::Up(button),
            other => other,
        }
    } else {
        kind
    };

    Some(InternalEvent::Event(TerminalInputEvent::Mouse(
        MouseEvent {
            kind,
            column,
            row,
            modifiers,
        },
    )))
}
