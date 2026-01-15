use std::fmt::{self, Display};
use std::hash::Hash;

use super::modifier::ModifierKeyCode;
use vtansi::TerseDisplay;

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
        f.write_str(match self {
            MediaKeyCode::Play => "Play",
            MediaKeyCode::Pause => "Pause",
            MediaKeyCode::PlayPause => "Play/Pause",
            MediaKeyCode::Reverse => "Reverse",
            MediaKeyCode::Stop => "Stop",
            MediaKeyCode::FastForward => "Fast Forward",
            MediaKeyCode::Rewind => "Rewind",
            MediaKeyCode::TrackNext => "Next Track",
            MediaKeyCode::TrackPrevious => "Previous Track",
            MediaKeyCode::Record => "Record",
            MediaKeyCode::LowerVolume => "Lower Volume",
            MediaKeyCode::RaiseVolume => "Raise Volume",
            MediaKeyCode::MuteVolume => "Mute Volume",
        })
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
    /// [`KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES`](super::KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES) has been enabled with
    /// [`PushKeyboardEnhancementFlags`](super::PushKeyboardEnhancementFlags).
    CapsLock,
    /// Scroll Lock key.
    ///
    /// **Note:** this key can only be read if
    /// [`KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES`](super::KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES) has been enabled with
    /// [`PushKeyboardEnhancementFlags`](super::PushKeyboardEnhancementFlags).
    ScrollLock,
    /// Num Lock key.
    ///
    /// **Note:** this key can only be read if
    /// [`KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES`](super::KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES) has been enabled with
    /// [`PushKeyboardEnhancementFlags`](super::PushKeyboardEnhancementFlags).
    NumLock,
    /// Print Screen key.
    ///
    /// **Note:** this key can only be read if
    /// [`KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES`](super::KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES) has been enabled with
    /// [`PushKeyboardEnhancementFlags`](super::PushKeyboardEnhancementFlags).
    PrintScreen,
    /// Pause key.
    ///
    /// **Note:** this key can only be read if
    /// [`KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES`](super::KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES) has been enabled with
    /// [`PushKeyboardEnhancementFlags`](super::PushKeyboardEnhancementFlags).
    Pause,
    /// Menu key.
    ///
    /// **Note:** this key can only be read if
    /// [`KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES`](super::KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES) has been enabled with
    /// [`PushKeyboardEnhancementFlags`](super::PushKeyboardEnhancementFlags).
    Menu,
    /// The "Begin" key (often mapped to the 5 key when Num Lock is turned on).
    ///
    /// **Note:** this key can only be read if
    /// [`KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES`](super::KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES) has been enabled with
    /// [`PushKeyboardEnhancementFlags`](super::PushKeyboardEnhancementFlags).
    KeypadBegin,
    /// A media key.
    ///
    /// **Note:** these keys can only be read if
    /// [`KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES`](super::KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES) has been enabled with
    /// [`PushKeyboardEnhancementFlags`](super::PushKeyboardEnhancementFlags).
    Media(MediaKeyCode),
    /// A modifier key.
    ///
    /// **Note:** these keys can only be read if **both**
    /// [`KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES`](super::KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES) and
    /// [`KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES`](super::KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES) have been enabled with
    /// [`PushKeyboardEnhancementFlags`](super::PushKeyboardEnhancementFlags).
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
            KeyCode::Char('\0') => Ok(()), // Empty for composition events
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
    /// use vtio::event::keyboard::KeyCode;
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
    /// use vtio::event::keyboard::KeyCode;
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
    /// use vtio::event::keyboard::KeyCode;
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
    /// [`KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES`](super::KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES) to be enabled with
    /// [`PushKeyboardEnhancementFlags`](super::PushKeyboardEnhancementFlags).
    ///
    /// # Examples
    ///
    /// ```
    /// use vtio::event::keyboard::{KeyCode, MediaKeyCode};
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
    /// [`KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES`](super::KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES) and
    /// [`KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES`](super::KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES) to be enabled with
    /// [`PushKeyboardEnhancementFlags`](super::PushKeyboardEnhancementFlags).
    ///
    /// # Examples
    ///
    /// ```
    /// use vtio::event::keyboard::{KeyCode, ModifierKeyCode};
    /// assert!(KeyCode::Modifier(ModifierKeyCode::LeftShift).is_modifier(ModifierKeyCode::LeftShift));
    /// assert!(!KeyCode::Modifier(ModifierKeyCode::LeftShift).is_modifier(ModifierKeyCode::RightShift));
    /// ```
    #[must_use]
    pub fn is_modifier(&self, modifier: ModifierKeyCode) -> bool {
        matches!(self, KeyCode::Modifier(m) if *m == modifier)
    }
}

// Platform-specific key names
// On macOS, Backspace is "Delete", Delete is "Fwd Del", Enter is "Return"
#[cfg(target_os = "macos")]
const BACKSPACE_NAME: &str = "Delete";
#[cfg(not(target_os = "macos"))]
const BACKSPACE_NAME: &str = "Backspace";

#[cfg(target_os = "macos")]
const DELETE_NAME: &str = "Fwd Del";
#[cfg(not(target_os = "macos"))]
const DELETE_NAME: &str = "Del";

#[cfg(target_os = "macos")]
const ENTER_NAME: &str = "Return";
#[cfg(not(target_os = "macos"))]
const ENTER_NAME: &str = "Enter";

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
            KeyCode::Backspace => f.write_str(BACKSPACE_NAME),
            KeyCode::Delete => f.write_str(DELETE_NAME),
            KeyCode::Enter => f.write_str(ENTER_NAME),
            KeyCode::Left => f.write_str("Left"),
            KeyCode::Right => f.write_str("Right"),
            KeyCode::Up => f.write_str("Up"),
            KeyCode::Down => f.write_str("Down"),
            KeyCode::Home => f.write_str("Home"),
            KeyCode::End => f.write_str("End"),
            KeyCode::PageUp => f.write_str("Page Up"),
            KeyCode::PageDown => f.write_str("Page Down"),
            KeyCode::Tab => f.write_str("Tab"),
            KeyCode::BackTab => f.write_str("Back Tab"),
            KeyCode::Insert => f.write_str("Insert"),
            KeyCode::F(n) => write!(f, "F{n}"),
            KeyCode::Char(' ') => f.write_str("Space"),
            KeyCode::Char(c) => write!(f, "{c}"),
            KeyCode::Null => f.write_str("Null"),
            KeyCode::Esc => f.write_str("Esc"),
            KeyCode::CapsLock => f.write_str("Caps Lock"),
            KeyCode::ScrollLock => f.write_str("Scroll Lock"),
            KeyCode::NumLock => f.write_str("Num Lock"),
            KeyCode::PrintScreen => f.write_str("Print Screen"),
            KeyCode::Pause => f.write_str("Pause"),
            KeyCode::Menu => f.write_str("Menu"),
            KeyCode::KeypadBegin => f.write_str("Begin"),
            KeyCode::Media(media) => write!(f, "{media}"),
            KeyCode::Modifier(modifier) => write!(f, "{modifier}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use KeyCode::*;
    use MediaKeyCode::*;

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
}
