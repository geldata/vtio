//! iTerm2 proprietary escape sequences (OSC 1337).
//!
//! iTerm2 defines a set of proprietary escape sequences under OSC 1337
//! for terminal control and communication. These sequences follow the
//! pattern:
//!
//! ```text
//! ESC ] 1337 ; [command] ST
//! ```
//!
//! Where `[command]` can be:
//! - A simple key (e.g., `SetMark`)
//! - A key=value pair (e.g., `CursorShape=1`)
//! - Multiple key=value pairs separated by semicolons (e.g.,
//!   `Block=id=foo;attr=start`)
//!
//! This module provides type-safe wrappers for known sequences and a
//! generic mechanism for encoding arbitrary key=value pairs.

use vtenc::{Encode, EncodeError, IntoSeq, WriteSeq, write_osc};

const ITERM2_OSC_PREFIX: &str = "1337;";

/// Generate a simple iTerm2 command type with no parameters.
///
/// Creates a zero-sized type that implements `ITerm2Command` by writing
/// a fixed command string.
macro_rules! simple_command {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $name;

        impl $name {
            /// The command key string.
            pub const KEY: &'static str = std::stringify!($cmd);
        }

        impl vtenc::Encode for $name {
            fn encode<W: std::io::Write + ?Sized>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
                vtenc::write_osc!(buf; $crate::iterm::ITERM2_OSC_PREFIX, Self::KEY)
            }
        }
    };
}

/// Generate an iTerm2 command type with a single typed parameter.
///
/// Creates a type that implements `ITerm2Command` by writing
/// `key=value` where the key is fixed and the value is of a given type.
macro_rules! iterm2_param_command {
    ($(#[$meta:meta])* $name:ident { $value_field:ident: $type:ty }) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct $name {
            pub $value_field: $type,
        }

        impl $name {
            /// The command key string.
            pub const KEY: &'static str = std::stringify!($name);

            /// Create a new command instance.
            pub fn new<T>($value_field: T) -> Self
                where
                    T: Into<$type>
            {
                Self { $value_field: $value_field.into() }
            }
        }

        impl Encode for $name {
            fn encode<W: std::io::Write + ?Sized>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
                vtenc::write_osc!(buf; $crate::iterm::ITERM2_OSC_PREFIX, Self::KEY, "=", self.$value_field)
            }
        }
    };
}

/// Generate an iTerm2 command type with a single string parameter.
///
/// Creates a type that implements `ITerm2Command` by writing
/// `key=value` where the key is fixed and the value is a `str`.
macro_rules! iterm2_string_param_command {
    ($(#[$meta:meta])* $name:ident { $value_field:ident }) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct $name<'a> {
            pub $value_field: &'a str,
        }

        impl<'a> $name<'a> {
            /// The command key string.
            pub const KEY: &'static str = std::stringify!($name);

            /// Create a new command instance.
            pub fn new($value_field: &'a str) -> Self {
                Self { $value_field }
            }
        }

        impl Encode for $name<'_> {
            fn encode<W: std::io::Write + ?Sized>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
                vtenc::write_osc!(buf; $crate::iterm::ITERM2_OSC_PREFIX, Self::KEY, "=", self.$value_field)
            }
        }
    };
}

// Simple commands (no parameters)

simple_command!(
    /// Set a mark at the current cursor position.
    ///
    /// Equivalent to the "Set Mark" command (cmd-shift-M).
    /// The mark can be jumped to later with cmd-shift-J.
    SetMark
);

simple_command!(
    /// Bring iTerm2 window to the foreground.
    ///
    /// Force the terminal to steal focus from other applications.
    StealFocus
);

simple_command!(
    /// Clear the scrollback history.
    ///
    /// Erase all content in the scrollback buffer, keeping only the
    /// visible screen content.
    ClearScrollback
);

simple_command!(
    /// End a copy-to-clipboard operation.
    ///
    /// Marks the end of text being copied to the pasteboard. Must be
    /// preceded by a `CopyToClipboard` command.
    EndCopy
);

simple_command!(
    /// Report the cell size in points.
    ///
    /// The terminal responds with:
    /// `OSC 1337 ; ReportCellSize=[height];[width];[scale] ST`
    ///
    /// where scale is the pixel-to-point ratio (1.0 for non-retina,
    /// 2.0 for retina).
    ReportCellSize
);

simple_command!(
    /// Push the current touch bar key labels onto a stack.
    ///
    /// Save the current set of function key labels for later restoration
    /// with `PopKeyLabels`.
    PushKeyLabels
);

simple_command!(
    /// Pop touch bar key labels from the stack.
    ///
    /// Restore the most recently pushed set of function key labels.
    PopKeyLabels
);

simple_command!(
    /// Disinter a buried session.
    ///
    /// Restore a previously buried session to the active state.
    Disinter
);

simple_command!(
    /// Clear captured output.
    ///
    /// Erase the current captured output buffer for this session.
    ClearCapturedOutput
);

// Single parameter commands

/// Cursor shape values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CursorShapeValue {
    /// Block cursor.
    Block = 0,
    /// Vertical bar cursor.
    VerticalBar = 1,
    /// Underline cursor.
    Underline = 2,
}

impl IntoSeq for CursorShapeValue {
    fn into_seq(&self) -> impl WriteSeq {
        *self as u8
    }
}

iterm2_param_command!(
    /// Set the cursor shape.
    CursorShape { shape: CursorShapeValue }
);

iterm2_string_param_command!(
    /// Set the current directory path.
    ///
    /// Inform iTerm2 of the current working directory to enable
    /// semantic history and other path-based features.
    CurrentDir { path }
);

iterm2_string_param_command!(
    /// Change the session's profile.
    ///
    /// Switch to a different profile by name. The profile must exist
    /// in iTerm2's configuration.
    SetProfile { profile }
);

/// Attention request modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Clipboard {
    /// General clipboard
    General,
    /// Rule clipboard
    Rule,
    /// Find clipboard
    Find,
    /// Font clipboard
    Font,
}

impl IntoSeq for Clipboard {
    fn into_seq(&self) -> impl WriteSeq {
        match self {
            Self::General => "",
            Self::Rule => "rule",
            Self::Find => "find",
            Self::Font => "font",
        }
    }
}

iterm2_param_command!(
    /// Start copying text to a clipboard.
    ///
    /// All text received after this command is placed in the specified
    /// pasteboard until `EndCopy` is received.
    CopyToClipboard { clipboard: Clipboard }
);

iterm2_string_param_command!(
    /// Set background image from a file path.
    ///
    /// The value should be a base64-encoded filename. An empty string
    /// removes the background image. User confirmation is required as
    /// a security measure.
    SetBackgroundImageFile { base64_path }
);

/// Attention request modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AttentionMode {
    /// Bounce dock icon indefinitely.
    Yes,
    /// Bounce dock icon once.
    Once,
    /// Cancel previous request.
    No,
    /// Display fireworks at cursor location.
    Fireworks,
}

impl IntoSeq for AttentionMode {
    fn into_seq(&self) -> impl WriteSeq {
        match self {
            Self::Yes => "yes",
            Self::Once => "once",
            Self::No => "no",
            Self::Fireworks => "fireworks",
        }
    }
}

iterm2_param_command!(
    /// Request attention with visual effects.
    RequestAttention { mode: AttentionMode }
);

/// Unicode version values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnicodeVersionValue {
    /// Unicode 8 width tables.
    V8 = 8,
    /// Unicode 9 width tables.
    V9 = 9,
}

impl IntoSeq for UnicodeVersionValue {
    fn into_seq(&self) -> impl WriteSeq {
        *self as u8
    }
}

iterm2_param_command!(
    /// Set Unicode width table version.
    ///
    /// Can also push/pop values on a stack using special string values
    /// (use `GenericCommand` for that).
    UnicodeVersion { version: UnicodeVersionValue }
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Iterm2Bool(bool);

impl From<bool> for Iterm2Bool {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl IntoSeq for Iterm2Bool {
    fn into_seq(&self) -> impl WriteSeq {
        if self.0 { "yes" } else { "no" }
    }
}

iterm2_param_command!(
    /// Enable or disable the cursor guide (highlight cursor line).
    HighlightCursorLine { enabled: Iterm2Bool }
);

iterm2_string_param_command!(
    /// Copy text to the general clipboard.
    Copy { base64_text }
);

iterm2_string_param_command!(
    /// Report the value of a session variable.
    ReportVariable { base64_name }
);

iterm2_string_param_command!(
    /// Request file upload from the user.
    RequestUpload { format }
);

iterm2_string_param_command!(
    /// Open a URL in the default browser.
    OpenUrl { base64_url }
);

/// Generic command for arbitrary key=value pairs.
///
/// Use this for unrecognized or custom iTerm2 commands that follow
/// the key=value pattern. Multiple pairs can be separated by semicolons.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GenericCommand {
    pairs: Vec<KeyValue>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum KeyValue {
    Key(String),
    KeyValue(String, String),
}

impl GenericCommand {
    /// Create a new generic command with no pairs.
    #[must_use]
    pub fn new() -> Self {
        Self { pairs: Vec::new() }
    }

    /// Add a key without a value.
    #[must_use]
    pub fn with_key(mut self, key: impl Into<String>) -> Self {
        self.pairs.push(KeyValue::Key(key.into()));
        self
    }

    /// Add a key=value pair.
    #[must_use]
    pub fn with_pair(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.pairs
            .push(KeyValue::KeyValue(key.into(), value.into()));
        self
    }

    /// Add a key without a value (mutable).
    pub fn add_key(&mut self, key: impl Into<String>) {
        self.pairs.push(KeyValue::Key(key.into()));
    }

    /// Add a key=value pair (mutable).
    pub fn add_pair(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.pairs
            .push(KeyValue::KeyValue(key.into(), value.into()));
    }
}

impl Default for GenericCommand {
    fn default() -> Self {
        Self::new()
    }
}

impl Encode for GenericCommand {
    fn encode<W: std::io::Write + ?Sized>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        let values = self
            .pairs
            .iter()
            .map(|pair| match pair {
                KeyValue::Key(key) => key.to_owned(),
                KeyValue::KeyValue(key, value) => format!("{key}={value}"),
            })
            .collect::<Vec<String>>()
            .join(";");
        write_osc!(buf; ITERM2_OSC_PREFIX, values)
    }
}

/// Annotation with optional parameters.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AddAnnotation<'a> {
    pub message: &'a str,
    pub length: Option<u32>,
    pub x_coord: Option<u32>,
    pub y_coord: Option<u32>,
    pub hidden: bool,
}

impl<'a> AddAnnotation<'a> {
    /// Create a simple annotation with just a message.
    #[must_use]
    pub fn new(message: &'a str) -> Self {
        Self {
            message,
            length: None,
            x_coord: None,
            y_coord: None,
            hidden: false,
        }
    }

    #[must_use]
    pub fn key(&self) -> &'static str {
        if self.hidden {
            "AddHiddenAnnotation"
        } else {
            "AddAnnotation"
        }
    }

    /// Set the length of cells to annotate.
    #[must_use]
    pub fn with_length(mut self, length: u32) -> Self {
        self.length = Some(length);
        self
    }

    /// Set the coordinates for the annotation.
    #[must_use]
    pub fn with_coords(mut self, x: u32, y: u32) -> Self {
        self.x_coord = Some(x);
        self.y_coord = Some(y);
        self
    }
}

impl Encode for AddAnnotation<'_> {
    fn encode<W: std::io::Write + ?Sized>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        if let Some(len) = self.length {
            if let (Some(x), Some(y)) = (self.x_coord, self.y_coord) {
                write_osc!(
                    buf;
                    ITERM2_OSC_PREFIX,
                    self.key(),
                    len,
                    "|",
                    self.message,
                    "|",
                    x,
                    "|",
                    y
                )
            } else {
                write_osc!(
                    buf;
                    ITERM2_OSC_PREFIX,
                    self.key(),
                    len,
                    "|",
                    self.message
                )
            }
        } else {
            write_osc!(
                buf;
                ITERM2_OSC_PREFIX,
                self.key(),
                self.message
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_commands() {
        let mut buf = Vec::new();

        let mut seq = ITerm2Sequence::new(SetMark);
        seq.encode(&mut buf).unwrap();
        assert_eq!(&buf[..], b"\x1B]1337;SetMark\x1B\\");

        buf.clear();
        let mut seq = ITerm2Sequence::new(StealFocus);
        seq.encode(&mut buf).unwrap();
        assert_eq!(&buf[..], b"\x1B]1337;StealFocus\x1B\\");
    }

    #[test]
    fn test_int_param() {
        let mut buf = Vec::new();

        let mut seq = ITerm2Sequence::new(CursorShape::new(CursorShapeValue::VerticalBar));
        seq.encode(&mut buf).unwrap();
        assert_eq!(&buf[..], b"\x1B]1337;CursorShape=1\x1B\\");
    }

    #[test]
    fn test_string_param() {
        let mut buf = Vec::new();

        let mut seq = ITerm2Sequence::new(CurrentDir::new("/home/user"));
        seq.encode(&mut buf).unwrap();
        assert_eq!(&buf[..], b"\x1B]1337;CurrentDir=/home/user\x1B\\");
    }

    #[test]
    fn test_boolean_param() {
        let mut buf = Vec::new();

        let mut seq = ITerm2Sequence::new(HighlightCursorLine::new(true));
        seq.encode(&mut buf).unwrap();
        assert_eq!(&buf[..], b"\x1B]1337;HighlightCursorLine=yes\x1B\\");

        buf.clear();
        let mut seq = ITerm2Sequence::new(HighlightCursorLine::new(false));
        seq.encode(&mut buf).unwrap();
        assert_eq!(&buf[..], b"\x1B]1337;HighlightCursorLine=no\x1B\\");
    }

    #[test]
    fn test_generic_command() {
        let mut buf = Vec::new();

        let cmd = GenericCommand::new()
            .with_pair("key1", "value1")
            .with_pair("key2", "value2");
        let mut seq = ITerm2Sequence::new(cmd);
        seq.encode(&mut buf).unwrap();
        assert_eq!(&buf[..], b"\x1B]1337;key1=value1;key2=value2\x1B\\");
    }

    #[test]
    fn test_generic_command_with_key_only() {
        let mut buf = Vec::new();

        let cmd = GenericCommand::new().with_key("SomeCommand");
        let mut seq = ITerm2Sequence::new(cmd);
        seq.encode(&mut buf).unwrap();
        assert_eq!(&buf[..], b"\x1B]1337;SomeCommand\x1B\\");
    }

    #[test]
    fn test_annotation() {
        let mut buf = Vec::new();

        let ann = AddAnnotation::new("test message");
        let mut seq = ITerm2Sequence::new(ann);
        seq.encode(&mut buf).unwrap();
        assert_eq!(&buf[..], b"\x1B]1337;AddAnnotation=test message\x1B\\");

        buf.clear();
        let ann = AddAnnotation::new("test").with_length(10);
        let mut seq = ITerm2Sequence::new(ann);
        seq.encode(&mut buf).unwrap();
        assert_eq!(&buf[..], b"\x1B]1337;AddAnnotation=10|test\x1B\\");
    }
}
