use std::fmt::{self, Display, Write as FmtWrite};
use std::hash::{Hash, Hasher};

use bitflags::bitflags;
use vt_push_parser::event::VTEvent;

use crate::{
    csi,
    encode::{Encode, EncodeError, write_str_into},
    write_into,
};

/// Format terminal events in a terse, human-readable format for test
/// output.
pub trait TerseDisplay {
    /// Format the value in a terse format.
    ///
    /// # Errors
    ///
    /// Return an error if formatting fails.
    fn terse_fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}

bitflags! {
    /// Represents special flags that tell compatible terminals to add extra information to keyboard events.
    ///
    /// See <https://sw.kovidgoyal.net/kitty/keyboard-protocol/#progressive-enhancement> for more information.
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(transparent))]
    #[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
    pub struct KeyboardEnhancementFlags: u8 {
        /// Represent Escape and modified keys using CSI-u sequences, so they can be unambiguously
        /// read.
        const DISAMBIGUATE_ESCAPE_CODES = 0b0000_0001;
        /// Add extra events with [`KeyEvent.kind`] set to [`KeyEventKind::Repeat`] or
        /// [`KeyEventKind::Release`] when keys are autorepeated or released.
        const REPORT_EVENT_TYPES = 0b0000_0010;
        /// Send [alternate keycodes](https://sw.kovidgoyal.net/kitty/keyboard-protocol/#key-codes)
        /// in addition to the base keycode. The alternate keycode overrides the base keycode in
        /// resulting `KeyEvent`s.
        const REPORT_ALTERNATE_KEYS = 0b0000_0100;
        /// Represent all keyboard events as CSI-u sequences. This is required to get repeat/release
        /// events for plain-text keys.
        const REPORT_ALL_KEYS_AS_ESCAPE_CODES = 0b0000_1000;
        /// Send the Unicode codepoint as well as the keycode.
        const REPORT_ASSOCIATED_TEXT = 0b0001_0000;
    }
}

/// A command that enables mouse event capture.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EnableMouseCapture;

impl Encode for EnableMouseCapture {
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        write_str_into(
            buf,
            concat!(
                // Normal tracking: Send mouse X & Y on button press and release
                csi!("?1000h"),
                // Button-event tracking: Report button motion events (dragging)
                csi!("?1002h"),
                // Any-event tracking: Report all motion events
                csi!("?1003h"),
                // RXVT mouse mode: Allows mouse coordinates of >223
                csi!("?1015h"),
                // SGR mouse mode: Allows mouse coordinates of >223, preferred over RXVT mode
                csi!("?1006h"),
            ),
        )
    }
}

/// A command that disables mouse event capture.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisableMouseCapture;

impl Encode for DisableMouseCapture {
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        write_str_into(
            buf,
            concat!(
                // The inverse commands of EnableMouseCapture, in reverse order.
                csi!("?1006l"),
                csi!("?1015l"),
                csi!("?1003l"),
                csi!("?1002l"),
                csi!("?1000l"),
            ),
        )
    }
}

/// A command that enables focus event emission.
///
/// It should be paired with [`DisableFocusChange`] at the end of execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EnableFocusChange;

impl Encode for EnableFocusChange {
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        write_str_into(buf, csi!("?1004h"))
    }
}

/// A command that disables focus event emission.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisableFocusChange;

impl Encode for DisableFocusChange {
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        write_str_into(buf, csi!("?1004l"))
    }
}

/// A command that enables [bracketed paste mode](https://en.wikipedia.org/wiki/Bracketed-paste).
///
/// It should be paired with [`DisableBracketedPaste`] at the end of execution.
///
/// This is not supported in older Windows terminals without
/// [virtual terminal sequences](https://docs.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EnableBracketedPaste;

impl Encode for EnableBracketedPaste {
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        write_str_into(buf, csi!("?2004h"))
    }
}

/// A command that disables bracketed paste mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisableBracketedPaste;

impl Encode for DisableBracketedPaste {
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        write_str_into(buf, csi!("?2004l"))
    }
}

/// A command that enables the [kitty keyboard protocol](https://sw.kovidgoyal.net/kitty/keyboard-protocol/), which adds extra information to keyboard events and removes ambiguity for modifier keys.
///
/// It should be paired with [`PopKeyboardEnhancementFlags`] at the end of execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PushKeyboardEnhancementFlags(pub KeyboardEnhancementFlags);

impl Encode for PushKeyboardEnhancementFlags {
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        write_into!(buf, "{}{}u", csi!(">"), self.0.bits())
    }
}

/// A command that disables extra kinds of keyboard events.
///
/// Specifically, it pops one level of keyboard enhancement flags.
///
/// See [`PushKeyboardEnhancementFlags`] and <https://sw.kovidgoyal.net/kitty/keyboard-protocol/> for more information.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PopKeyboardEnhancementFlags;

impl Encode for PopKeyboardEnhancementFlags {
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        write_str_into(buf, csi!("<1u"))
    }
}

impl Encode for KeyEvent {
    #[allow(clippy::too_many_lines)]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        // Only generate on key press (ignore repeats/releases)
        if self.kind != KeyEventKind::Press {
            return Ok(0);
        }

        let mods = self.modifiers;

        // xterm modifier parameter: 1 + (Shift=1) + (Alt=2) + (Ctrl=4)
        let mod_param = 1
            + i32::from(mods.contains(KeyModifiers::SHIFT))
            + if mods.contains(KeyModifiers::ALT) {
                2
            } else {
                0
            }
            + if mods.contains(KeyModifiers::CONTROL) {
                4
            } else {
                0
            };

        let mut pos = 0;
        let alt_prefix = mods.contains(KeyModifiers::ALT);

        match self.code {
            KeyCode::Char(mut c) => {
                // Shift is represented by the character case itself
                if mods.contains(KeyModifiers::SHIFT) && c.is_ascii_lowercase() {
                    c = c.to_ascii_uppercase();
                }

                // Ctrl+char -> control codes (ASCII)
                if mods.contains(KeyModifiers::CONTROL) {
                    let ctrl = control_code_for(c);
                    if alt_prefix {
                        if buf.is_empty() {
                            return Err(EncodeError::BufferOverflow(2));
                        }
                        buf[pos] = 0x1b;
                        pos += 1;
                    }
                    if pos >= buf.len() {
                        return Err(EncodeError::BufferOverflow(pos + 1));
                    }
                    buf[pos] = ctrl;
                    pos += 1;
                    return Ok(pos);
                }

                if alt_prefix {
                    if buf.is_empty() {
                        return Err(EncodeError::BufferOverflow(1));
                    }
                    buf[pos] = 0x1b;
                    pos += 1;
                }
                let mut tmp = [0u8; 4];
                let s = c.encode_utf8(&mut tmp);
                if pos + s.len() > buf.len() {
                    return Err(EncodeError::BufferOverflow(pos + s.len()));
                }
                buf[pos..pos + s.len()].copy_from_slice(s.as_bytes());
                pos += s.len();
            }

            KeyCode::Enter => {
                // Handle modified Enter key
                if mod_param > 1 {
                    // CSI u format: ESC[13;<mod>u for modified Enter
                    pos += write_into!(&mut buf[pos..], "{}13;{}u", csi!(), mod_param)?;
                } else if alt_prefix {
                    // Alt+Enter: ESC followed by CR
                    if buf.len() < 2 {
                        return Err(EncodeError::BufferOverflow(2));
                    }
                    buf[pos] = 0x1b;
                    buf[pos + 1] = b'\r';
                    pos += 2;
                } else {
                    // Normal Enter: just CR
                    if buf.is_empty() {
                        return Err(EncodeError::BufferOverflow(1));
                    }
                    buf[pos] = b'\r';
                    pos += 1;
                }
            }

            KeyCode::Backspace => {
                if buf.is_empty() {
                    return Err(EncodeError::BufferOverflow(1));
                }
                buf[pos] = 0x7f; // DEL
                pos += 1;
            }

            KeyCode::Tab => {
                if mods.contains(KeyModifiers::SHIFT) {
                    pos += write_str_into(&mut buf[pos..], csi!("Z"))?; // Back-tab
                } else {
                    if buf.is_empty() {
                        return Err(EncodeError::BufferOverflow(1));
                    }
                    buf[pos] = b'\t';
                    pos += 1;
                }
            }

            KeyCode::Esc => {
                if buf.is_empty() {
                    return Err(EncodeError::BufferOverflow(1));
                }
                buf[pos] = 0x1b;
                pos += 1;
            }

            KeyCode::Up
            | KeyCode::Down
            | KeyCode::Right
            | KeyCode::Left
            | KeyCode::Home
            | KeyCode::End => {
                let (final_byte, use_ss3) = match self.code {
                    KeyCode::Up => (b'A', true),
                    KeyCode::Down => (b'B', true),
                    KeyCode::Right => (b'C', true),
                    KeyCode::Left => (b'D', true),
                    KeyCode::Home => (b'H', true),
                    KeyCode::End => (b'F', true),
                    _ => unreachable!(),
                };

                let no_mods = mod_param == 1;
                let application_cursor = false; // Default to normal cursor mode

                if application_cursor && no_mods && use_ss3 {
                    // SS3: ESC O <final>
                    if buf.len() < pos + 3 {
                        return Err(EncodeError::BufferOverflow(pos + 3));
                    }
                    buf[pos] = 0x1b;
                    buf[pos + 1] = b'O';
                    buf[pos + 2] = final_byte;
                    pos += 3;
                } else if no_mods && use_ss3 {
                    // Normal cursor mode: CSI <final>
                    if buf.len() < pos + 3 {
                        return Err(EncodeError::BufferOverflow(pos + 3));
                    }
                    buf[pos] = 0x1b;
                    buf[pos + 1] = b'[';
                    buf[pos + 2] = final_byte;
                    pos += 3;
                } else {
                    // With modifiers: CSI 1;M <final>
                    pos += write_into!(
                        &mut buf[pos..],
                        "{}1;{}{}",
                        csi!(),
                        mod_param,
                        final_byte as char
                    )?;
                }
            }

            KeyCode::Insert => pos += push_tilde_seq(&mut buf[pos..], 2, mod_param)?,
            KeyCode::Delete => pos += push_tilde_seq(&mut buf[pos..], 3, mod_param)?,
            KeyCode::PageUp => pos += push_tilde_seq(&mut buf[pos..], 5, mod_param)?,
            KeyCode::PageDown => pos += push_tilde_seq(&mut buf[pos..], 6, mod_param)?,

            KeyCode::F(n) => {
                // xterm mappings
                if (1..=4).contains(&n) {
                    let letter = match n {
                        1 => b'P',
                        2 => b'Q',
                        3 => b'R',
                        4 => b'S',
                        _ => unreachable!(),
                    };
                    if mod_param == 1 {
                        // SS3 for F1-F4
                        if buf.len() < pos + 3 {
                            return Err(EncodeError::BufferOverflow(pos + 3));
                        }
                        buf[pos] = 0x1b;
                        buf[pos + 1] = b'O';
                        buf[pos + 2] = letter;
                        pos += 3;
                    } else {
                        pos += write_into!(
                            &mut buf[pos..],
                            "{}1;{}{}",
                            csi!(),
                            mod_param,
                            letter as char
                        )?;
                    }
                } else {
                    let code = match n {
                        5 => 15,
                        6 => 17,
                        7 => 18,
                        8 => 19,
                        9 => 20,
                        10 => 21,
                        11 => 23,
                        12 => 24,
                        13 => 25,
                        14 => 26,
                        15 => 28,
                        16 => 29,
                        17 => 31,
                        18 => 32,
                        19 => 33,
                        20 => 34,
                        _ => 0,
                    };
                    if code != 0 {
                        if mod_param == 1 {
                            pos += write_into!(&mut buf[pos..], "{}{}~", csi!(), code)?;
                        } else {
                            pos +=
                                write_into!(&mut buf[pos..], "{}{};{}~", csi!(), code, mod_param)?;
                        }
                    }
                }
            }

            KeyCode::BackTab => {
                pos += write_str_into(&mut buf[pos..], csi!("Z"))?;
            }

            KeyCode::Null
            | KeyCode::CapsLock
            | KeyCode::ScrollLock
            | KeyCode::NumLock
            | KeyCode::PrintScreen
            | KeyCode::Pause
            | KeyCode::Menu
            | KeyCode::KeypadBegin
            | KeyCode::Media(_)
            | KeyCode::Modifier(_) => {
                // No standard sequences for these keys
            }
        }

        Ok(pos)
    }
}

impl Encode for MouseEvent {
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let mods = self.modifiers;

        // Calculate modifier offset for SGR mode
        let mod_offset = if mods.contains(KeyModifiers::SHIFT) {
            4
        } else {
            0
        } + if mods.contains(KeyModifiers::ALT) {
            8
        } else {
            0
        } + if mods.contains(KeyModifiers::CONTROL) {
            16
        } else {
            0
        };

        // Map mouse event kinds to SGR button codes
        let (base_button, final_char) = match self.kind {
            MouseEventKind::Down(button) => {
                let btn_code = match button {
                    MouseButton::Left => 0,
                    MouseButton::Middle => 1,
                    MouseButton::Right => 2,
                };
                (btn_code, b'M')
            }
            MouseEventKind::Up(button) => {
                let btn_code = match button {
                    MouseButton::Left => 0,
                    MouseButton::Middle => 1,
                    MouseButton::Right => 2,
                };
                (btn_code, b'm')
            }
            MouseEventKind::Drag(button) => {
                let btn_code = match button {
                    MouseButton::Left => 0,
                    MouseButton::Middle => 1,
                    MouseButton::Right => 2,
                };
                // Add drag bit (bit 5 = 32)
                (btn_code + 32, b'M')
            }
            MouseEventKind::Moved => {
                // Mouse move without button
                (3 + 32, b'M')
            }
            MouseEventKind::ScrollUp => (1 << 6, b'M'),
            MouseEventKind::ScrollDown => (1 << 6 | 1, b'M'),
            MouseEventKind::ScrollLeft => (1 << 6 | 2, b'M'),
            MouseEventKind::ScrollRight => (1 << 6 | 3, b'M'),
        };

        let button_code = base_button + mod_offset;

        // Convert coordinates (0-based to 1-based for SGR)
        let x = self.column + 1;
        let y = self.row + 1;

        // Generate SGR sequence: ESC[<btn;col;row(M|m)
        write_into!(
            buf,
            "{}<{};{};{}{}",
            csi!(),
            button_code,
            x,
            y,
            final_char as char
        )
    }
}

impl Encode for TerminalInputEvent<'_> {
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        match self {
            TerminalInputEvent::Key(event) => event.encode(buf),
            TerminalInputEvent::Mouse(event) => event.encode(buf),
            TerminalInputEvent::Resize(rows, cols) => {
                write_into!(buf, "{}8;{};{}t", csi!(), rows, cols)
            }
            TerminalInputEvent::Focus(true) => write_str_into(buf, csi!("I")),
            TerminalInputEvent::Focus(false) => write_str_into(buf, csi!("O")),
            TerminalInputEvent::Paste(text) => {
                let text_str =
                    core::str::from_utf8(text).map_err(|_| EncodeError::BufferOverflow(0))?;
                write_into!(buf, "{}200~{}{}201~", csi!(), text_str, csi!())
            }
            #[cfg(unix)]
            TerminalInputEvent::CursorPosition(_, _)
            | TerminalInputEvent::KeyboardEnhancementFlags(_)
            | TerminalInputEvent::PrimaryDeviceAttributes
            | TerminalInputEvent::KeyboardEnhancementFlagsPush(_)
            | TerminalInputEvent::KeyboardEnhancementFlagsPop(_)
            | TerminalInputEvent::KeyboardEnhancementFlagsQuery => {
                // These are responses from the terminal or protocol negotiation
                Ok(0)
            }
            TerminalInputEvent::LowLevel(_) => {
                // Low-level events are not encoded
                Ok(0)
            }
        }
    }
}

/// Map Ctrl+<char> to control code (ASCII).
fn control_code_for(c: char) -> u8 {
    match c {
        '@' | ' ' => 0x00,
        'A'..='Z' => (c as u8 | 0x20) & 0x1f,
        '[' => 0x1b,
        '\\' => 0x1c,
        ']' => 0x1d,
        '^' => 0x1e,
        '_' => 0x1f,
        '?' => 0x7f,
        _ => c as u8 & 0x1f,
    }
}

/// Write a tilde sequence (like for Insert, Delete, `PageUp`, `PageDown`).
fn push_tilde_seq(buf: &mut [u8], base: u8, mod_param: i32) -> Result<usize, EncodeError> {
    if mod_param == 1 {
        write_into!(buf, "{}{}~", csi!(), base)
    } else {
        write_into!(buf, "{}{};{}~", csi!(), base, mod_param)
    }
}

/// Represents an event.
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TerminalInputEvent<'a> {
    /// The terminal gained/lost focus
    Focus(bool),
    /// A single key event with additional pressed modifiers.
    Key(KeyEvent),
    /// A single mouse event with additional pressed modifiers.
    Mouse(MouseEvent),
    /// A string that was pasted into the terminal.
    Paste(&'a [u8]),
    /// An resize event with new dimensions after resize (columns, rows).
    /// **Note** that resize events can occur in batches.
    Resize(u16, u16),
    /// A cursor position (`col`, `row`).
    #[cfg(unix)]
    CursorPosition(u16, u16),
    /// The progressive keyboard enhancement flags enabled by the terminal.
    #[cfg(unix)]
    KeyboardEnhancementFlags(KeyboardEnhancementFlags),
    /// Attributes and architectural class of the terminal.
    #[cfg(unix)]
    PrimaryDeviceAttributes,
    /// Request to push keyboard enhancement flags onto the stack.
    /// Contains the flags to push (or None to push current flags).
    #[cfg(unix)]
    KeyboardEnhancementFlagsPush(Option<KeyboardEnhancementFlags>),
    /// Request to pop keyboard enhancement flags from the stack.
    /// Contains the number of entries to pop (defaults to 1).
    #[cfg(unix)]
    KeyboardEnhancementFlagsPop(u16),
    /// Query for current keyboard enhancement flags.
    #[cfg(unix)]
    KeyboardEnhancementFlagsQuery,
    /// Low-level VT event.
    LowLevel(&'a VTEvent<'a>),
}

impl TerminalInputEvent<'_> {
    /// Convert a borrowed `TerminalInputEvent` to an owned `TerminalInputEventOwned`.
    #[must_use]
    pub fn to_owned(&self) -> TerminalInputEventOwned {
        match self {
            TerminalInputEvent::Focus(focus) => TerminalInputEventOwned::Focus(*focus),
            TerminalInputEvent::Key(key) => TerminalInputEventOwned::Key(key.clone()),
            TerminalInputEvent::Mouse(mouse) => TerminalInputEventOwned::Mouse(*mouse),
            TerminalInputEvent::Paste(paste) => TerminalInputEventOwned::Paste(paste.to_vec()),
            TerminalInputEvent::Resize(cols, rows) => TerminalInputEventOwned::Resize(*cols, *rows),
            #[cfg(unix)]
            TerminalInputEvent::CursorPosition(col, row) => {
                TerminalInputEventOwned::CursorPosition(*col, *row)
            }
            #[cfg(unix)]
            TerminalInputEvent::KeyboardEnhancementFlags(flags) => {
                TerminalInputEventOwned::KeyboardEnhancementFlags(*flags)
            }
            #[cfg(unix)]
            TerminalInputEvent::PrimaryDeviceAttributes => {
                TerminalInputEventOwned::PrimaryDeviceAttributes
            }
            #[cfg(unix)]
            TerminalInputEvent::KeyboardEnhancementFlagsPush(flags) => {
                TerminalInputEventOwned::KeyboardEnhancementFlagsPush(*flags)
            }
            #[cfg(unix)]
            TerminalInputEvent::KeyboardEnhancementFlagsPop(count) => {
                TerminalInputEventOwned::KeyboardEnhancementFlagsPop(*count)
            }
            #[cfg(unix)]
            TerminalInputEvent::KeyboardEnhancementFlagsQuery => {
                TerminalInputEventOwned::KeyboardEnhancementFlagsQuery
            }
            TerminalInputEvent::LowLevel(vt_event) => TerminalInputEventOwned::LowLevel(Box::new(
                vt_push_parser::event::VTEvent::to_owned(vt_event),
            )),
        }
    }
}

/// An owned version of `TerminalEvent`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TerminalInputEventOwned {
    /// The terminal gained/lost focus
    Focus(bool),
    /// A single key event with additional pressed modifiers.
    Key(KeyEvent),
    /// A single mouse event with additional pressed modifiers.
    Mouse(MouseEvent),
    /// A string that was pasted into the terminal.
    Paste(Vec<u8>),
    /// An resize event with new dimensions after resize (columns, rows).
    /// **Note** that resize events can occur in batches.
    Resize(u16, u16),
    /// A cursor position (`col`, `row`).
    #[cfg(unix)]
    CursorPosition(u16, u16),
    /// The progressive keyboard enhancement flags enabled by the terminal.
    #[cfg(unix)]
    KeyboardEnhancementFlags(KeyboardEnhancementFlags),
    /// Attributes and architectural class of the terminal.
    #[cfg(unix)]
    PrimaryDeviceAttributes,
    /// Request to push keyboard enhancement flags onto the stack.
    #[cfg(unix)]
    KeyboardEnhancementFlagsPush(Option<KeyboardEnhancementFlags>),
    /// Request to pop keyboard enhancement flags from the stack.
    #[cfg(unix)]
    KeyboardEnhancementFlagsPop(u16),
    /// Query for current keyboard enhancement flags.
    #[cfg(unix)]
    KeyboardEnhancementFlagsQuery,
    /// Low-level VT event.
    LowLevel(Box<vt_push_parser::event::VTOwnedEvent>),
}

impl TerminalInputEventOwned {
    /// Borrow the owned `TerminalInputEventOwned` as a `TerminalEvent`.
    ///
    /// Note: This method cannot borrow `LowLevel` variants due to lifetime
    /// constraints. Use the owned `VTOwnedEvent` directly for those cases.
    ///
    /// # Panics
    ///
    /// Panic if called on a `LowLevel` variant.
    #[must_use]
    pub fn borrow(&self) -> TerminalInputEvent<'_> {
        match self {
            TerminalInputEventOwned::Focus(focus) => TerminalInputEvent::Focus(*focus),
            TerminalInputEventOwned::Key(key) => TerminalInputEvent::Key(key.clone()),
            TerminalInputEventOwned::Mouse(mouse) => TerminalInputEvent::Mouse(*mouse),
            TerminalInputEventOwned::Paste(paste) => TerminalInputEvent::Paste(paste),
            TerminalInputEventOwned::Resize(cols, rows) => TerminalInputEvent::Resize(*cols, *rows),
            #[cfg(unix)]
            TerminalInputEventOwned::CursorPosition(col, row) => {
                TerminalInputEvent::CursorPosition(*col, *row)
            }
            #[cfg(unix)]
            TerminalInputEventOwned::KeyboardEnhancementFlags(flags) => {
                TerminalInputEvent::KeyboardEnhancementFlags(*flags)
            }
            #[cfg(unix)]
            TerminalInputEventOwned::PrimaryDeviceAttributes => {
                TerminalInputEvent::PrimaryDeviceAttributes
            }
            #[cfg(unix)]
            TerminalInputEventOwned::KeyboardEnhancementFlagsPush(flags) => {
                TerminalInputEvent::KeyboardEnhancementFlagsPush(*flags)
            }
            #[cfg(unix)]
            TerminalInputEventOwned::KeyboardEnhancementFlagsPop(count) => {
                TerminalInputEvent::KeyboardEnhancementFlagsPop(*count)
            }
            #[cfg(unix)]
            TerminalInputEventOwned::KeyboardEnhancementFlagsQuery => {
                TerminalInputEvent::KeyboardEnhancementFlagsQuery
            }
            TerminalInputEventOwned::LowLevel(_) => {
                // We cannot borrow LowLevel events because VTEvent.borrow() returns
                // a temporary value that cannot be referenced in the return type.
                // This variant should be accessed directly from the owned event.
                panic!("Cannot borrow LowLevel variant - access VTOwnedEvent directly")
            }
        }
    }
}

/// An internal event.
///
/// Encapsulates publicly available `Event` with additional internal
/// events that shouldn't be publicly available to the crate users.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum InternalEvent<'a> {
    /// An event.
    Event(TerminalInputEvent<'a>),
    /// Start of bracketed paste.
    BracketedPasteStart,
}

/// Represents a mouse event.
///
/// # Platform-specific Notes
///
/// ## Mouse Buttons
///
/// Some platforms/terminals do not report mouse button for the
/// `MouseEventKind::Up` and `MouseEventKind::Drag` events. `MouseButton::Left`
/// is returned if we don't know which button was used.
///
/// ## Key Modifiers
///
/// Some platforms/terminals does not report all key modifiers
/// combinations for all mouse event types. For example - macOS reports
/// `Ctrl` + left mouse button click as a right mouse button click.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct MouseEvent {
    /// The kind of mouse event that was caused.
    pub kind: MouseEventKind,
    /// The column that the event occurred on.
    pub column: u16,
    /// The row that the event occurred on.
    pub row: u16,
    /// The key modifiers active when the event occurred.
    pub modifiers: KeyModifiers,
}

/// A mouse event kind.
///
/// # Platform-specific Notes
///
/// ## Mouse Buttons
///
/// Some platforms/terminals do not report mouse button for the
/// `MouseEventKind::Up` and `MouseEventKind::Drag` events. `MouseButton::Left`
/// is returned if we don't know which button was used.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub enum MouseEventKind {
    /// Pressed mouse button. Contains the button that was pressed.
    Down(MouseButton),
    /// Released mouse button. Contains the button that was released.
    Up(MouseButton),
    /// Moved the mouse cursor while pressing the contained mouse button.
    Drag(MouseButton),
    /// Moved the mouse cursor while not pressing a mouse button.
    Moved,
    /// Scrolled mouse wheel downwards (towards the user).
    ScrollDown,
    /// Scrolled mouse wheel upwards (away from the user).
    ScrollUp,
    /// Scrolled mouse wheel left (mostly on a laptop touchpad).
    ScrollLeft,
    /// Scrolled mouse wheel right (mostly on a laptop touchpad).
    ScrollRight,
}

/// Represents a mouse button.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub enum MouseButton {
    /// Left mouse button.
    Left,
    /// Right mouse button.
    Right,
    /// Middle mouse button.
    Middle,
}

bitflags! {
    /// Represents key modifiers (shift, control, alt, etc.).
    ///
    /// **Note:** `SUPER`, `HYPER`, and `META` can only be read if
    /// [`KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES`] has been enabled with
    /// [`PushKeyboardEnhancementFlags`].
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(transparent))]
    #[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
    pub struct KeyModifiers: u8 {
        const SHIFT = 0b0000_0001;
        const CONTROL = 0b0000_0010;
        const ALT = 0b0000_0100;
        const SUPER = 0b0000_1000;
        const HYPER = 0b0001_0000;
        const META = 0b0010_0000;
        const NONE = 0b0000_0000;
    }
}

impl TerseDisplay for KeyModifiers {
    fn terse_fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;

        // Output modifiers in conventional order: ctrl-alt-shift-super-hyper-meta
        if self.contains(KeyModifiers::CONTROL) {
            if !first {
                f.write_char('-')?;
            }
            first = false;
            f.write_str("ctrl")?;
        }
        if self.contains(KeyModifiers::ALT) {
            if !first {
                f.write_char('-')?;
            }
            first = false;
            f.write_str("alt")?;
        }
        if self.contains(KeyModifiers::SHIFT) {
            if !first {
                f.write_char('-')?;
            }
            first = false;
            f.write_str("shift")?;
        }
        if self.contains(KeyModifiers::SUPER) {
            if !first {
                f.write_char('-')?;
            }
            first = false;
            f.write_str("super")?;
        }
        if self.contains(KeyModifiers::HYPER) {
            if !first {
                f.write_char('-')?;
            }
            first = false;
            f.write_str("hyper")?;
        }
        if self.contains(KeyModifiers::META) {
            if !first {
                f.write_char('-')?;
            }
            f.write_str("meta")?;
        }

        Ok(())
    }
}

impl Display for KeyModifiers {
    /// Formats the key modifiers using the given formatter.
    ///
    /// The key modifiers are joined by a `+` character.
    ///
    /// # Platform-specific Notes
    ///
    /// On macOS, the control, alt, and super keys is displayed as "Control", "Option", and
    /// "Command" respectively. See
    /// <https://support.apple.com/guide/applestyleguide/welcome/1.0/web>.
    ///
    /// On Windows, the super key is displayed as "Windows" and the control key is displayed as
    /// "Ctrl". See
    /// <https://learn.microsoft.com/en-us/style-guide/a-z-word-list-term-collections/term-collections/keys-keyboard-shortcuts>.
    ///
    /// On other platforms, the super key is referred to as "Super" and the control key is
    /// displayed as "Ctrl".
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        for modifier in self.iter() {
            if !first {
                f.write_str("+")?;
            }

            first = false;
            match modifier {
                KeyModifiers::SHIFT => f.write_str("Shift")?,
                #[cfg(unix)]
                KeyModifiers::CONTROL => f.write_str("Control")?,
                #[cfg(windows)]
                KeyModifiers::CONTROL => f.write_str("Ctrl")?,
                #[cfg(target_os = "macos")]
                KeyModifiers::ALT => f.write_str("Option")?,
                #[cfg(not(target_os = "macos"))]
                KeyModifiers::ALT => f.write_str("Alt")?,
                #[cfg(target_os = "macos")]
                KeyModifiers::SUPER => f.write_str("Command")?,
                #[cfg(target_os = "windows")]
                KeyModifiers::SUPER => f.write_str("Windows")?,
                #[cfg(not(any(target_os = "macos", target_os = "windows")))]
                KeyModifiers::SUPER => f.write_str("Super")?,
                KeyModifiers::HYPER => f.write_str("Hyper")?,
                KeyModifiers::META => f.write_str("Meta")?,
                _ => unreachable!(),
            }
        }
        Ok(())
    }
}

/// Represents a keyboard event kind.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub enum KeyEventKind {
    Press,
    Repeat,
    Release,
}

impl TerseDisplay for KeyEventKind {
    fn terse_fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KeyEventKind::Press => f.write_str("press"),
            KeyEventKind::Repeat => f.write_str("repeat"),
            KeyEventKind::Release => f.write_str("release"),
        }
    }
}

bitflags! {
    /// Represents extra state about the key event.
    ///
    /// **Note:** This state can only be read if
    /// [`KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES`] has been enabled with
    /// [`PushKeyboardEnhancementFlags`].
    #[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(transparent))]
    pub struct KeyEventState: u8 {
        /// The key event origins from the keypad.
        const KEYPAD = 0b0000_0001;
        /// Caps Lock was enabled for this key event.
        ///
        /// **Note:** this is set for the initial press of Caps Lock itself.
        const CAPS_LOCK = 0b0000_0010;
        /// Num Lock was enabled for this key event.
        ///
        /// **Note:** this is set for the initial press of Num Lock itself.
        const NUM_LOCK = 0b0000_0100;
        const NONE = 0b0000_0000;
    }
}

/// Represents a key event.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialOrd, Clone)]
pub struct KeyEvent {
    /// The key itself.
    pub code: KeyCode,
    /// Additional key modifiers.
    pub modifiers: KeyModifiers,
    /// Kind of event.
    ///
    /// Only set if:
    /// - Unix: [`KeyboardEnhancementFlags::REPORT_EVENT_TYPES`] has been enabled with [`PushKeyboardEnhancementFlags`].
    /// - Windows: always
    pub kind: KeyEventKind,
    /// Keyboard state.
    ///
    /// Only set if [`KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES`] has been enabled with
    /// [`PushKeyboardEnhancementFlags`].
    pub state: KeyEventState,
    /// Base layout key for cross-layout shortcut matching.
    ///
    /// This represents the key in the standard PC-101 layout that corresponds
    /// to the physical key pressed. For example, on a Cyrillic keyboard
    /// layout, pressing the physical key that produces 'С' (Cyrillic) would
    /// have `base_layout_key` set to Some('c') (Latin), allowing Ctrl+C
    /// shortcuts to work regardless of keyboard layout.
    ///
    /// Only set if [`KeyboardEnhancementFlags::REPORT_ALTERNATE_KEYS`] has
    /// been enabled with [`PushKeyboardEnhancementFlags`].
    pub base_layout_key: Option<KeyCode>,
    /// Associated text as Unicode codepoints.
    ///
    /// This represents the actual text generated by the key press,
    /// which may differ from the key code when modifiers or keyboard
    /// layouts are involved. For example, `Shift+A` would have key code 'a'
    /// but text "A".
    ///
    /// Only set if [`KeyboardEnhancementFlags::REPORT_ASSOCIATED_TEXT`] has
    /// been enabled with [`PushKeyboardEnhancementFlags`].
    pub text: Option<String>,
}

impl KeyEvent {
    #[must_use]
    pub const fn new(code: KeyCode, modifiers: KeyModifiers) -> KeyEvent {
        KeyEvent {
            code,
            modifiers,
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
            base_layout_key: None,
            text: None,
        }
    }

    #[must_use]
    pub const fn new_with_kind(
        code: KeyCode,
        modifiers: KeyModifiers,
        kind: KeyEventKind,
    ) -> KeyEvent {
        KeyEvent {
            code,
            modifiers,
            kind,
            state: KeyEventState::empty(),
            base_layout_key: None,
            text: None,
        }
    }

    #[must_use]
    pub const fn new_with_kind_and_state(
        code: KeyCode,
        modifiers: KeyModifiers,
        kind: KeyEventKind,
        state: KeyEventState,
    ) -> KeyEvent {
        KeyEvent {
            code,
            modifiers,
            kind,
            state,
            base_layout_key: None,
            text: None,
        }
    }

    #[must_use]
    pub const fn new_with_all(
        code: KeyCode,
        modifiers: KeyModifiers,
        kind: KeyEventKind,
        state: KeyEventState,
        base_layout_key: Option<KeyCode>,
    ) -> KeyEvent {
        KeyEvent {
            code,
            modifiers,
            kind,
            state,
            base_layout_key,
            text: None,
        }
    }

    #[must_use]
    pub const fn new_with_text(
        code: KeyCode,
        modifiers: KeyModifiers,
        kind: KeyEventKind,
        state: KeyEventState,
        base_layout_key: Option<KeyCode>,
        text: Option<String>,
    ) -> KeyEvent {
        KeyEvent {
            code,
            modifiers,
            kind,
            state,
            base_layout_key,
            text,
        }
    }

    // modifies the KeyEvent,
    // so that KeyModifiers::SHIFT is present iff
    // an uppercase char is present.
    fn normalize_case(mut self) -> KeyEvent {
        let KeyCode::Char(c) = self.code else {
            return self;
        };

        if c.is_ascii_uppercase() {
            self.modifiers.insert(KeyModifiers::SHIFT);
        } else if self.modifiers.contains(KeyModifiers::SHIFT) {
            self.code = KeyCode::Char(c.to_ascii_uppercase());
        }

        // Normalize base_layout_key if it's a Char
        if let Some(KeyCode::Char(base_c)) = self.base_layout_key {
            if base_c.is_ascii_uppercase() {
                // Already uppercase, keep it
            } else if self.modifiers.contains(KeyModifiers::SHIFT) {
                self.base_layout_key = Some(KeyCode::Char(base_c.to_ascii_uppercase()));
            }
        }

        self
    }

    /// Returns whether the key event is a press event.
    #[must_use]
    pub fn is_press(&self) -> bool {
        matches!(self.kind, KeyEventKind::Press)
    }

    /// Returns whether the key event is a release event.
    #[must_use]
    pub fn is_release(&self) -> bool {
        matches!(self.kind, KeyEventKind::Release)
    }

    /// Returns whether the key event is a repeat event.
    #[must_use]
    pub fn is_repeat(&self) -> bool {
        matches!(self.kind, KeyEventKind::Repeat)
    }

    /// Return the base layout key for cross-layout shortcut matching.
    ///
    /// This returns the key in the standard PC-101 layout that corresponds
    /// to the physical key pressed, useful for matching shortcuts across
    /// different keyboard layouts.
    #[must_use]
    pub fn base_layout_key(&self) -> Option<KeyCode> {
        self.base_layout_key
    }

    /// Check if this key event matches a key code, considering the base
    /// layout key for cross-layout matching.
    ///
    /// This is useful for shortcut matching where you want Ctrl+C to work
    /// regardless of keyboard layout.
    #[must_use]
    pub fn matches_key(&self, key: KeyCode) -> bool {
        self.code == key || self.base_layout_key == Some(key)
    }

    /// Return the associated text generated by this key event.
    ///
    /// This is the actual text that would be inserted by the key press,
    /// which may differ from the key code when modifiers or keyboard
    /// layouts are involved.
    #[must_use]
    pub fn text(&self) -> Option<&str> {
        self.text.as_deref()
    }
}

impl TerseDisplay for KeyEvent {
    fn terse_fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("key(")?;
        self.kind.terse_fmt(f)?;

        if self.modifiers.is_empty() {
            f.write_char(':')?;
        } else {
            f.write_char(':')?;
            self.modifiers.terse_fmt(f)?;
            f.write_char('-')?;
        }

        self.code.terse_fmt(f)?;

        if !self.state.is_empty() {
            if self.state.contains(KeyEventState::KEYPAD) {
                f.write_str(":keypad")?;
            }
            if self.state.contains(KeyEventState::CAPS_LOCK) {
                f.write_str(":caps_lock")?;
            }
            if self.state.contains(KeyEventState::NUM_LOCK) {
                f.write_str(":num_lock")?;
            }
        }

        if let Some(base_key) = self.base_layout_key {
            f.write_str(":base=")?;
            base_key.terse_fmt(f)?;
        }

        if let Some(text) = &self.text {
            f.write_str(":text=\"")?;
            for c in text.chars() {
                if c == '"' {
                    f.write_str("\\\"")?;
                } else if c == '\\' {
                    f.write_str("\\\\")?;
                } else if c.is_control() {
                    write!(f, "\\u{{{:x}}}", c as u32)?;
                } else {
                    f.write_char(c)?;
                }
            }
            f.write_char('"')?;
        }

        f.write_char(')')
    }
}

impl From<KeyCode> for KeyEvent {
    fn from(code: KeyCode) -> Self {
        KeyEvent {
            code,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
            base_layout_key: None,
            text: None,
        }
    }
}

impl PartialEq for KeyEvent {
    fn eq(&self, other: &KeyEvent) -> bool {
        let KeyEvent {
            code: lhs_code,
            modifiers: lhs_modifiers,
            kind: lhs_kind,
            state: lhs_state,
            base_layout_key: lhs_base_layout_key,
            text: lhs_text,
        } = &self.clone().normalize_case();
        let KeyEvent {
            code: rhs_code,
            modifiers: rhs_modifiers,
            kind: rhs_kind,
            state: rhs_state,
            base_layout_key: rhs_base_layout_key,
            text: rhs_text,
        } = &other.clone().normalize_case();
        (lhs_code == rhs_code)
            && (lhs_modifiers == rhs_modifiers)
            && (lhs_kind == rhs_kind)
            && (lhs_state == rhs_state)
            && (lhs_base_layout_key == rhs_base_layout_key)
            && (lhs_text == rhs_text)
    }
}

impl Eq for KeyEvent {}

impl Hash for KeyEvent {
    fn hash<H: Hasher>(&self, hash_state: &mut H) {
        let KeyEvent {
            code,
            modifiers,
            kind,
            state,
            base_layout_key,
            text,
        } = &self.clone().normalize_case();
        code.hash(hash_state);
        modifiers.hash(hash_state);
        kind.hash(hash_state);
        state.hash(hash_state);
        base_layout_key.hash(hash_state);
        text.hash(hash_state);
    }
}

/// Represents a media key (as part of [`KeyCode::Media`]).
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MediaKeyCode {
    /// Play media key.
    Play,
    /// Pause media key.
    Pause,
    /// Play/Pause media key.
    PlayPause,
    /// Reverse media key.
    Reverse,
    /// Stop media key.
    Stop,
    /// Fast-forward media key.
    FastForward,
    /// Rewind media key.
    Rewind,
    /// Next-track media key.
    TrackNext,
    /// Previous-track media key.
    TrackPrevious,
    /// Record media key.
    Record,
    /// Lower-volume media key.
    LowerVolume,
    /// Raise-volume media key.
    RaiseVolume,
    /// Mute media key.
    MuteVolume,
}

impl Display for MediaKeyCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MediaKeyCode::Play => write!(f, "Play"),
            MediaKeyCode::Pause => write!(f, "Pause"),
            MediaKeyCode::PlayPause => write!(f, "Play/Pause"),
            MediaKeyCode::Reverse => write!(f, "Reverse"),
            MediaKeyCode::Stop => write!(f, "Stop"),
            MediaKeyCode::FastForward => write!(f, "Fast Forward"),
            MediaKeyCode::Rewind => write!(f, "Rewind"),
            MediaKeyCode::TrackNext => write!(f, "Next Track"),
            MediaKeyCode::TrackPrevious => write!(f, "Previous Track"),
            MediaKeyCode::Record => write!(f, "Record"),
            MediaKeyCode::LowerVolume => write!(f, "Lower Volume"),
            MediaKeyCode::RaiseVolume => write!(f, "Raise Volume"),
            MediaKeyCode::MuteVolume => write!(f, "Mute Volume"),
        }
    }
}

/// Represents a modifier key (as part of [`KeyCode::Modifier`]).
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ModifierKeyCode {
    /// Left Shift key.
    LeftShift,
    /// Left Control key. (Control on macOS, Ctrl on other platforms)
    LeftControl,
    /// Left Alt key. (Option on macOS, Alt on other platforms)
    LeftAlt,
    /// Left Super key. (Command on macOS, Windows on Windows, Super on other platforms)
    LeftSuper,
    /// Left Hyper key.
    LeftHyper,
    /// Left Meta key.
    LeftMeta,
    /// Right Shift key.
    RightShift,
    /// Right Control key. (Control on macOS, Ctrl on other platforms)
    RightControl,
    /// Right Alt key. (Option on macOS, Alt on other platforms)
    RightAlt,
    /// Right Super key. (Command on macOS, Windows on Windows, Super on other platforms)
    RightSuper,
    /// Right Hyper key.
    RightHyper,
    /// Right Meta key.
    RightMeta,
    /// Iso Level3 Shift key.
    IsoLevel3Shift,
    /// Iso Level5 Shift key.
    IsoLevel5Shift,
}

impl Display for ModifierKeyCode {
    /// Formats the modifier key using the given formatter.
    ///
    /// # Platform-specific Notes
    ///
    /// On macOS, the control, alt, and super keys are displayed as "Control", "Option", and
    /// "Command" respectively. See
    /// <https://support.apple.com/guide/applestyleguide/welcome/1.0/web>.
    ///
    /// On Windows, the super key is displayed as "Windows" and the control key is displayed as
    /// "Ctrl". See
    /// <https://learn.microsoft.com/en-us/style-guide/a-z-word-list-term-collections/term-collections/keys-keyboard-shortcuts>.
    ///
    /// On other platforms, the super key is referred to as "Super".
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ModifierKeyCode::LeftShift => write!(f, "Left Shift"),
            ModifierKeyCode::LeftHyper => write!(f, "Left Hyper"),
            ModifierKeyCode::LeftMeta => write!(f, "Left Meta"),
            ModifierKeyCode::RightShift => write!(f, "Right Shift"),
            ModifierKeyCode::RightHyper => write!(f, "Right Hyper"),
            ModifierKeyCode::RightMeta => write!(f, "Right Meta"),
            ModifierKeyCode::IsoLevel3Shift => write!(f, "Iso Level 3 Shift"),
            ModifierKeyCode::IsoLevel5Shift => write!(f, "Iso Level 5 Shift"),

            #[cfg(target_os = "macos")]
            ModifierKeyCode::LeftControl => write!(f, "Left Control"),
            #[cfg(not(target_os = "macos"))]
            ModifierKeyCode::LeftControl => write!(f, "Left Ctrl"),

            #[cfg(target_os = "macos")]
            ModifierKeyCode::LeftAlt => write!(f, "Left Option"),
            #[cfg(not(target_os = "macos"))]
            ModifierKeyCode::LeftAlt => write!(f, "Left Alt"),

            #[cfg(target_os = "macos")]
            ModifierKeyCode::LeftSuper => write!(f, "Left Command"),
            #[cfg(target_os = "windows")]
            ModifierKeyCode::LeftSuper => write!(f, "Left Windows"),
            #[cfg(not(any(target_os = "macos", target_os = "windows")))]
            ModifierKeyCode::LeftSuper => write!(f, "Left Super"),

            #[cfg(target_os = "macos")]
            ModifierKeyCode::RightControl => write!(f, "Right Control"),
            #[cfg(not(target_os = "macos"))]
            ModifierKeyCode::RightControl => write!(f, "Right Ctrl"),

            #[cfg(target_os = "macos")]
            ModifierKeyCode::RightAlt => write!(f, "Right Option"),
            #[cfg(not(target_os = "macos"))]
            ModifierKeyCode::RightAlt => write!(f, "Right Alt"),

            #[cfg(target_os = "macos")]
            ModifierKeyCode::RightSuper => write!(f, "Right Command"),
            #[cfg(target_os = "windows")]
            ModifierKeyCode::RightSuper => write!(f, "Right Windows"),
            #[cfg(not(any(target_os = "macos", target_os = "windows")))]
            ModifierKeyCode::RightSuper => write!(f, "Right Super"),
        }
    }
}

/// Represents a key.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum KeyCode {
    /// Backspace key (Delete on macOS, Backspace on other platforms).
    Backspace,
    /// Enter key.
    Enter,
    /// Left arrow key.
    Left,
    /// Right arrow key.
    Right,
    /// Up arrow key.
    Up,
    /// Down arrow key.
    Down,
    /// Home key.
    Home,
    /// End key.
    End,
    /// Page up key.
    PageUp,
    /// Page down key.
    PageDown,
    /// Tab key.
    Tab,
    /// Shift + Tab key.
    BackTab,
    /// Delete key. (Fn+Delete on macOS, Delete on other platforms)
    Delete,
    /// Insert key.
    Insert,
    /// F key.
    ///
    /// `KeyCode::F(1)` represents F1 key, etc.
    F(u8),
    /// A character.
    ///
    /// `KeyCode::Char('c')` represents `c` character, etc.
    Char(char),
    /// Null.
    Null,
    /// Escape key.
    Esc,
    /// Caps Lock key.
    ///
    /// **Note:** this key can only be read if
    /// [`KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES`] has been enabled with
    /// [`PushKeyboardEnhancementFlags`].
    CapsLock,
    /// Scroll Lock key.
    ///
    /// **Note:** this key can only be read if
    /// [`KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES`] has been enabled with
    /// [`PushKeyboardEnhancementFlags`].
    ScrollLock,
    /// Num Lock key.
    ///
    /// **Note:** this key can only be read if
    /// [`KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES`] has been enabled with
    /// [`PushKeyboardEnhancementFlags`].
    NumLock,
    /// Print Screen key.
    ///
    /// **Note:** this key can only be read if
    /// [`KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES`] has been enabled with
    /// [`PushKeyboardEnhancementFlags`].
    PrintScreen,
    /// Pause key.
    ///
    /// **Note:** this key can only be read if
    /// [`KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES`] has been enabled with
    /// [`PushKeyboardEnhancementFlags`].
    Pause,
    /// Menu key.
    ///
    /// **Note:** this key can only be read if
    /// [`KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES`] has been enabled with
    /// [`PushKeyboardEnhancementFlags`].
    Menu,
    /// The "Begin" key (often mapped to the 5 key when Num Lock is turned on).
    ///
    /// **Note:** this key can only be read if
    /// [`KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES`] has been enabled with
    /// [`PushKeyboardEnhancementFlags`].
    KeypadBegin,
    /// A media key.
    ///
    /// **Note:** these keys can only be read if
    /// [`KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES`] has been enabled with
    /// [`PushKeyboardEnhancementFlags`].
    Media(MediaKeyCode),
    /// A modifier key.
    ///
    /// **Note:** these keys can only be read if **both**
    /// [`KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES`] and
    /// [`KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES`] have been enabled with
    /// [`PushKeyboardEnhancementFlags`].
    Modifier(ModifierKeyCode),
}

impl TerseDisplay for KeyCode {
    fn terse_fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KeyCode::Backspace => f.write_str("backspace"),
            KeyCode::Enter => f.write_str("enter"),
            KeyCode::Left => f.write_str("left"),
            KeyCode::Right => f.write_str("right"),
            KeyCode::Up => f.write_str("up"),
            KeyCode::Down => f.write_str("down"),
            KeyCode::Home => f.write_str("home"),
            KeyCode::End => f.write_str("end"),
            KeyCode::PageUp => f.write_str("pageup"),
            KeyCode::PageDown => f.write_str("pagedown"),
            KeyCode::Tab => f.write_str("tab"),
            KeyCode::BackTab => f.write_str("backtab"),
            KeyCode::Delete => f.write_str("delete"),
            KeyCode::Insert => f.write_str("insert"),
            KeyCode::F(n) => write!(f, "f{n}"),
            KeyCode::Char(' ') => f.write_str("space"),
            KeyCode::Char(c) => write!(f, "{c}"),
            KeyCode::Null => f.write_str("null"),
            KeyCode::Esc => f.write_str("esc"),
            KeyCode::CapsLock => f.write_str("capslock"),
            KeyCode::ScrollLock => f.write_str("scrolllock"),
            KeyCode::NumLock => f.write_str("numlock"),
            KeyCode::PrintScreen => f.write_str("printscreen"),
            KeyCode::Pause => f.write_str("pause"),
            KeyCode::Menu => f.write_str("menu"),
            KeyCode::KeypadBegin => f.write_str("keypadbegin"),
            KeyCode::Media(media) => write!(f, "media:{media}"),
            KeyCode::Modifier(modifier) => write!(f, "modifier:{modifier}"),
        }
    }
}

impl KeyCode {
    /// Returns `true` if the key code is the given function key.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vtinput::event::KeyCode;
    /// assert!(KeyCode::F(1).is_function_key(1));
    /// assert!(!KeyCode::F(1).is_function_key(2));
    /// ```
    #[must_use]
    pub fn is_function_key(&self, n: u8) -> bool {
        matches!(self, KeyCode::F(m) if *m == n)
    }

    /// Returns `true` if the key code is the given character.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vtinput::event::KeyCode;
    /// assert!(KeyCode::Char('a').is_char('a'));
    /// assert!(!KeyCode::Char('a').is_char('b'));
    /// assert!(!KeyCode::F(1).is_char('a'));
    /// ```
    #[must_use]
    pub fn is_char(&self, c: char) -> bool {
        matches!(self, KeyCode::Char(m) if *m == c)
    }

    /// Returns the character if the key code is a character key.
    ///
    /// Returns `None` if the key code is not a character key.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vtinput::event::KeyCode;
    /// assert_eq!(KeyCode::Char('a').as_char(), Some('a'));
    /// assert_eq!(KeyCode::F(1).as_char(), None);
    /// ```
    #[must_use]
    pub fn as_char(&self) -> Option<char> {
        match self {
            KeyCode::Char(c) => Some(*c),
            _ => None,
        }
    }

    /// Returns `true` if the key code is the given media key.
    ///
    /// **Note:** this method requires
    /// [`KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES`] to be enabled with
    /// [`PushKeyboardEnhancementFlags`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use vtinput::event::{KeyCode, MediaKeyCode};
    /// assert!(KeyCode::Media(MediaKeyCode::Play).is_media_key(MediaKeyCode::Play));
    /// assert!(!KeyCode::Media(MediaKeyCode::Play).is_media_key(MediaKeyCode::Pause));
    /// ```
    #[must_use]
    pub fn is_media_key(&self, media: MediaKeyCode) -> bool {
        matches!(self, KeyCode::Media(m) if *m == media)
    }

    /// Returns `true` if the key code is the given modifier key.
    ///
    /// **Note:** this method requires both
    /// [`KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES`] and
    /// [`KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES`] to be enabled with
    /// [`PushKeyboardEnhancementFlags`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use vtinput::event::{KeyCode, ModifierKeyCode};
    /// assert!(KeyCode::Modifier(ModifierKeyCode::LeftShift).is_modifier(ModifierKeyCode::LeftShift));
    /// assert!(!KeyCode::Modifier(ModifierKeyCode::LeftShift).is_modifier(ModifierKeyCode::RightShift));
    /// ```
    #[must_use]
    pub fn is_modifier(&self, modifier: ModifierKeyCode) -> bool {
        matches!(self, KeyCode::Modifier(m) if *m == modifier)
    }
}

impl Display for KeyCode {
    /// Formats the `KeyCode` using the given formatter.
    ///
    /// # Platform-specific Notes
    ///
    /// On macOS, the Backspace key is displayed as "Delete", the Delete key is displayed as "Fwd
    /// Del", and the Enter key is displayed as "Return". See
    /// <https://support.apple.com/guide/applestyleguide/welcome/1.0/web>.
    ///
    /// On other platforms, the Backspace key is displayed as "Backspace", the Delete key is
    /// displayed as "Del", and the Enter key is displayed as "Enter".
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // On macOS, the Backspace key is called "Delete" and the Delete key is called "Fwd Del".
            #[cfg(target_os = "macos")]
            KeyCode::Backspace => write!(f, "Delete"),
            #[cfg(target_os = "macos")]
            KeyCode::Delete => write!(f, "Fwd Del"),

            #[cfg(not(target_os = "macos"))]
            KeyCode::Backspace => write!(f, "Backspace"),
            #[cfg(not(target_os = "macos"))]
            KeyCode::Delete => write!(f, "Del"),

            #[cfg(target_os = "macos")]
            KeyCode::Enter => write!(f, "Return"),
            #[cfg(not(target_os = "macos"))]
            KeyCode::Enter => write!(f, "Enter"),
            KeyCode::Left => write!(f, "Left"),
            KeyCode::Right => write!(f, "Right"),
            KeyCode::Up => write!(f, "Up"),
            KeyCode::Down => write!(f, "Down"),
            KeyCode::Home => write!(f, "Home"),
            KeyCode::End => write!(f, "End"),
            KeyCode::PageUp => write!(f, "Page Up"),
            KeyCode::PageDown => write!(f, "Page Down"),
            KeyCode::Tab => write!(f, "Tab"),
            KeyCode::BackTab => write!(f, "Back Tab"),
            KeyCode::Insert => write!(f, "Insert"),
            KeyCode::F(n) => write!(f, "F{n}"),
            KeyCode::Char(c) => match c {
                // special case for non-visible characters
                ' ' => write!(f, "Space"),
                c => write!(f, "{c}"),
            },
            KeyCode::Null => write!(f, "Null"),
            KeyCode::Esc => write!(f, "Esc"),
            KeyCode::CapsLock => write!(f, "Caps Lock"),
            KeyCode::ScrollLock => write!(f, "Scroll Lock"),
            KeyCode::NumLock => write!(f, "Num Lock"),
            KeyCode::PrintScreen => write!(f, "Print Screen"),
            KeyCode::Pause => write!(f, "Pause"),
            KeyCode::Menu => write!(f, "Menu"),
            KeyCode::KeypadBegin => write!(f, "Begin"),
            KeyCode::Media(media) => write!(f, "{media}"),
            KeyCode::Modifier(modifier) => write!(f, "{modifier}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    use super::*;
    use KeyCode::*;
    use MediaKeyCode::*;
    use ModifierKeyCode::*;

    #[test]
    fn test_equality() {
        let lowercase_d_with_shift = KeyEvent::new(KeyCode::Char('d'), KeyModifiers::SHIFT);
        let uppercase_d_with_shift = KeyEvent::new(KeyCode::Char('D'), KeyModifiers::SHIFT);
        let uppercase_d = KeyEvent::new(KeyCode::Char('D'), KeyModifiers::NONE);
        assert_eq!(lowercase_d_with_shift, uppercase_d_with_shift);
        assert_eq!(uppercase_d, uppercase_d_with_shift);
    }

    #[test]
    fn test_hash() {
        let lowercase_d_with_shift_hash = {
            let mut hasher = DefaultHasher::new();
            KeyEvent::new(KeyCode::Char('d'), KeyModifiers::SHIFT).hash(&mut hasher);
            hasher.finish()
        };
        let uppercase_d_with_shift_hash = {
            let mut hasher = DefaultHasher::new();
            KeyEvent::new(KeyCode::Char('D'), KeyModifiers::SHIFT).hash(&mut hasher);
            hasher.finish()
        };
        let uppercase_d_hash = {
            let mut hasher = DefaultHasher::new();
            KeyEvent::new(KeyCode::Char('D'), KeyModifiers::NONE).hash(&mut hasher);
            hasher.finish()
        };
        assert_eq!(lowercase_d_with_shift_hash, uppercase_d_with_shift_hash);
        assert_eq!(uppercase_d_hash, uppercase_d_with_shift_hash);
    }

    #[test]
    fn keycode_display() {
        #[cfg(target_os = "macos")]
        {
            assert_eq!(format!("{Backspace}"), "Delete");
            assert_eq!(format!("{Delete}"), "Fwd Del");
            assert_eq!(format!("{Enter}"), "Return");
        }
        #[cfg(not(target_os = "macos"))]
        {
            assert_eq!(format!("{Backspace}"), "Backspace");
            assert_eq!(format!("{Delete}"), "Del");
            assert_eq!(format!("{Enter}"), "Enter");
        }
        assert_eq!(format!("{Left}"), "Left");
        assert_eq!(format!("{Right}"), "Right");
        assert_eq!(format!("{Up}"), "Up");
        assert_eq!(format!("{Down}"), "Down");
        assert_eq!(format!("{Home}"), "Home");
        assert_eq!(format!("{End}"), "End");
        assert_eq!(format!("{PageUp}"), "Page Up");
        assert_eq!(format!("{PageDown}"), "Page Down");
        assert_eq!(format!("{Tab}"), "Tab");
        assert_eq!(format!("{BackTab}"), "Back Tab");
        assert_eq!(format!("{Insert}"), "Insert");
        assert_eq!(format!("{}", F(1)), "F1");
        assert_eq!(format!("{}", Char('a')), "a");
        assert_eq!(format!("{Null}"), "Null");
        assert_eq!(format!("{Esc}"), "Esc");
        assert_eq!(format!("{CapsLock}"), "Caps Lock");
        assert_eq!(format!("{ScrollLock}"), "Scroll Lock");
        assert_eq!(format!("{NumLock}"), "Num Lock");
        assert_eq!(format!("{PrintScreen}"), "Print Screen");
        assert_eq!(format!("{}", KeyCode::Pause), "Pause");
        assert_eq!(format!("{Menu}"), "Menu");
        assert_eq!(format!("{KeypadBegin}"), "Begin");
    }

    #[test]
    fn media_keycode_display() {
        assert_eq!(format!("{}", Media(Play)), "Play");
        assert_eq!(format!("{}", Media(MediaKeyCode::Pause)), "Pause");
        assert_eq!(format!("{}", Media(PlayPause)), "Play/Pause");
        assert_eq!(format!("{}", Media(Reverse)), "Reverse");
        assert_eq!(format!("{}", Media(Stop)), "Stop");
        assert_eq!(format!("{}", Media(FastForward)), "Fast Forward");
        assert_eq!(format!("{}", Media(Rewind)), "Rewind");
        assert_eq!(format!("{}", Media(TrackNext)), "Next Track");
        assert_eq!(format!("{}", Media(TrackPrevious)), "Previous Track");
        assert_eq!(format!("{}", Media(Record)), "Record");
        assert_eq!(format!("{}", Media(LowerVolume)), "Lower Volume");
        assert_eq!(format!("{}", Media(RaiseVolume)), "Raise Volume");
        assert_eq!(format!("{}", Media(MuteVolume)), "Mute Volume");
    }

    #[test]
    fn modifier_keycode_display() {
        assert_eq!(format!("{}", Modifier(LeftShift)), "Left Shift");
        assert_eq!(format!("{}", Modifier(LeftHyper)), "Left Hyper");
        assert_eq!(format!("{}", Modifier(LeftMeta)), "Left Meta");
        assert_eq!(format!("{}", Modifier(RightShift)), "Right Shift");
        assert_eq!(format!("{}", Modifier(RightHyper)), "Right Hyper");
        assert_eq!(format!("{}", Modifier(RightMeta)), "Right Meta");
        assert_eq!(format!("{}", Modifier(IsoLevel3Shift)), "Iso Level 3 Shift");
        assert_eq!(format!("{}", Modifier(IsoLevel5Shift)), "Iso Level 5 Shift");
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn modifier_keycode_display_macos() {
        assert_eq!(format!("{}", Modifier(LeftControl)), "Left Control");
        assert_eq!(format!("{}", Modifier(LeftAlt)), "Left Option");
        assert_eq!(format!("{}", Modifier(LeftSuper)), "Left Command");
        assert_eq!(format!("{}", Modifier(RightControl)), "Right Control");
        assert_eq!(format!("{}", Modifier(RightAlt)), "Right Option");
        assert_eq!(format!("{}", Modifier(RightSuper)), "Right Command");
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn modifier_keycode_display_windows() {
        assert_eq!(format!("{}", Modifier(LeftControl)), "Left Ctrl");
        assert_eq!(format!("{}", Modifier(LeftAlt)), "Left Alt");
        assert_eq!(format!("{}", Modifier(LeftSuper)), "Left Windows");
        assert_eq!(format!("{}", Modifier(RightControl)), "Right Ctrl");
        assert_eq!(format!("{}", Modifier(RightAlt)), "Right Alt");
        assert_eq!(format!("{}", Modifier(RightSuper)), "Right Windows");
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    #[test]
    fn modifier_keycode_display_other() {
        assert_eq!(format!("{}", Modifier(LeftControl)), "Left Ctrl");
        assert_eq!(format!("{}", Modifier(LeftAlt)), "Left Alt");
        assert_eq!(format!("{}", Modifier(LeftSuper)), "Left Super");
        assert_eq!(format!("{}", Modifier(RightControl)), "Right Ctrl");
        assert_eq!(format!("{}", Modifier(RightAlt)), "Right Alt");
        assert_eq!(format!("{}", Modifier(RightSuper)), "Right Super");
    }

    #[test]
    fn key_modifiers_display() {
        let modifiers = KeyModifiers::SHIFT | KeyModifiers::CONTROL | KeyModifiers::ALT;

        #[cfg(target_os = "macos")]
        assert_eq!(modifiers.to_string(), "Shift+Control+Option");

        #[cfg(target_os = "windows")]
        assert_eq!(modifiers.to_string(), "Shift+Ctrl+Alt");

        #[cfg(not(any(target_os = "macos", target_os = "windows")))]
        assert_eq!(modifiers.to_string(), "Shift+Control+Alt");
    }

    #[test]
    fn test_encode_key_event_char() {
        let mut event = KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE);
        let mut buf = [0u8; 64];
        let len = event.encode(&mut buf).unwrap();
        assert_eq!(&buf[..len], b"a");
    }

    #[test]
    fn test_encode_key_event_ctrl_char() {
        let mut event = KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL);
        let mut buf = [0u8; 64];
        let len = event.encode(&mut buf).unwrap();
        assert_eq!(&buf[..len], &[0x03]); // Ctrl-C
    }

    #[test]
    fn test_encode_key_event_enter() {
        let mut event = KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE);
        let mut buf = [0u8; 64];
        let len = event.encode(&mut buf).unwrap();
        assert_eq!(&buf[..len], b"\r");
    }

    #[test]
    fn test_encode_key_event_arrow() {
        let mut event = KeyEvent::new(KeyCode::Up, KeyModifiers::NONE);
        let mut buf = [0u8; 64];
        let len = event.encode(&mut buf).unwrap();
        assert_eq!(&buf[..len], b"\x1b[A");
    }

    #[test]
    fn test_encode_key_event_arrow_with_modifiers() {
        let mut event = KeyEvent::new(KeyCode::Up, KeyModifiers::SHIFT);
        let mut buf = [0u8; 64];
        let len = event.encode(&mut buf).unwrap();
        assert_eq!(&buf[..len], b"\x1b[1;2A");
    }

    #[test]
    fn test_encode_key_event_f1() {
        let mut event = KeyEvent::new(KeyCode::F(1), KeyModifiers::NONE);
        let mut buf = [0u8; 64];
        let len = event.encode(&mut buf).unwrap();
        assert_eq!(&buf[..len], b"\x1bOP");
    }

    #[test]
    fn test_base_layout_key() {
        // Test basic base layout key storage
        let event = KeyEvent::new_with_all(
            KeyCode::Char('С'), // Cyrillic S
            KeyModifiers::CONTROL,
            KeyEventKind::Press,
            KeyEventState::empty(),
            Some(KeyCode::Char('c')), // Latin c
        );
        assert_eq!(event.base_layout_key(), Some(KeyCode::Char('c')));
    }

    #[test]
    fn test_matches_key_with_base_layout() {
        // Test that matches_key works with base layout key
        let event = KeyEvent::new_with_all(
            KeyCode::Char('С'), // Cyrillic S
            KeyModifiers::CONTROL,
            KeyEventKind::Press,
            KeyEventState::empty(),
            Some(KeyCode::Char('c')), // Latin c
        );

        // Should match the actual key
        assert!(event.matches_key(KeyCode::Char('С')));

        // Should also match the base layout key
        assert!(event.matches_key(KeyCode::Char('c')));

        // Should not match other keys
        assert!(!event.matches_key(KeyCode::Char('d')));
    }

    #[test]
    fn test_matches_key_without_base_layout() {
        // Test that matches_key works without base layout key
        let event = KeyEvent::new(KeyCode::Char('a'), KeyModifiers::CONTROL);

        // Should match the actual key
        assert!(event.matches_key(KeyCode::Char('a')));

        // Should not match other keys
        assert!(!event.matches_key(KeyCode::Char('b')));
    }

    #[test]
    fn test_base_layout_key_equality() {
        // Events with same base layout key should be equal
        let event1 = KeyEvent::new_with_all(
            KeyCode::Char('С'),
            KeyModifiers::CONTROL,
            KeyEventKind::Press,
            KeyEventState::empty(),
            Some(KeyCode::Char('c')),
        );
        let event2 = KeyEvent::new_with_all(
            KeyCode::Char('С'),
            KeyModifiers::CONTROL,
            KeyEventKind::Press,
            KeyEventState::empty(),
            Some(KeyCode::Char('c')),
        );
        assert_eq!(event1, event2);

        // Events with different base layout keys should not be equal
        let event3 = KeyEvent::new_with_all(
            KeyCode::Char('С'),
            KeyModifiers::CONTROL,
            KeyEventKind::Press,
            KeyEventState::empty(),
            Some(KeyCode::Char('d')),
        );
        assert_ne!(event1, event3);
    }

    #[test]
    fn test_base_layout_key_hash() {
        // Events with same base layout key should have same hash
        let hash1 = {
            let mut hasher = DefaultHasher::new();
            KeyEvent::new_with_all(
                KeyCode::Char('С'),
                KeyModifiers::CONTROL,
                KeyEventKind::Press,
                KeyEventState::empty(),
                Some(KeyCode::Char('c')),
            )
            .hash(&mut hasher);
            hasher.finish()
        };
        let hash2 = {
            let mut hasher = DefaultHasher::new();
            KeyEvent::new_with_all(
                KeyCode::Char('С'),
                KeyModifiers::CONTROL,
                KeyEventKind::Press,
                KeyEventState::empty(),
                Some(KeyCode::Char('c')),
            )
            .hash(&mut hasher);
            hasher.finish()
        };
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_encode_key_event_f5() {
        let mut event = KeyEvent::new(KeyCode::F(5), KeyModifiers::NONE);
        let mut buf = [0u8; 64];
        let len = event.encode(&mut buf).unwrap();
        assert_eq!(&buf[..len], b"\x1b[15~");
    }

    #[test]
    fn test_encode_mouse_event_down() {
        let mut event = MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: 0,
            row: 0,
            modifiers: KeyModifiers::NONE,
        };
        let mut buf = [0u8; 64];
        let len = event.encode(&mut buf).unwrap();
        assert_eq!(&buf[..len], b"\x1b[<0;1;1M");
    }

    #[test]
    fn test_encode_mouse_event_up() {
        let mut event = MouseEvent {
            kind: MouseEventKind::Up(MouseButton::Left),
            column: 0,
            row: 0,
            modifiers: KeyModifiers::NONE,
        };
        let mut buf = [0u8; 64];
        let len = event.encode(&mut buf).unwrap();
        assert_eq!(&buf[..len], b"\x1b[<0;1;1m");
    }

    #[test]
    fn test_encode_mouse_event_scroll() {
        let mut event = MouseEvent {
            kind: MouseEventKind::ScrollUp,
            column: 0,
            row: 0,
            modifiers: KeyModifiers::NONE,
        };
        let mut buf = [0u8; 64];
        let len = event.encode(&mut buf).unwrap();
        assert_eq!(&buf[..len], b"\x1b[<64;1;1M");
    }

    #[test]
    fn test_encode_terminal_event_focus() {
        let mut event = TerminalInputEvent::Focus(true);
        let mut buf = [0u8; 64];
        let len = event.encode(&mut buf).unwrap();
        assert_eq!(&buf[..len], b"\x1b[I");

        let mut event = TerminalInputEvent::Focus(false);
        let len = event.encode(&mut buf).unwrap();
        assert_eq!(&buf[..len], b"\x1b[O");
    }

    #[test]
    fn test_encode_terminal_event_resize() {
        let mut event = TerminalInputEvent::Resize(24, 80);
        let mut buf = [0u8; 64];
        let len = event.encode(&mut buf).unwrap();
        assert_eq!(&buf[..len], b"\x1b[8;24;80t");
    }

    #[test]
    fn test_encode_terminal_event_paste() {
        let text = b"hello world";
        let mut event = TerminalInputEvent::Paste(text);
        let mut buf = [0u8; 128];
        let len = event.encode(&mut buf).unwrap();
        assert_eq!(&buf[..len], b"\x1b[200~hello world\x1b[201~");
    }
}
