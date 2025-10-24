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

use std::borrow::Cow;

use vtenc::{Encode, EncodeError, IntoSeq, WriteSeq, write_osc};
use vtio_control_derive::VTControl;
use vtio_control_base::EscapeSequenceParam;

const ITERM2_OSC_PREFIX: &str = "1337;";

/// Set a mark at the current cursor position.
///
/// Equivalent to the "Set Mark" command (cmd-shift-M).
/// The mark can be jumped to later with cmd-shift-J.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, VTControl)]
#[vtctl(osc, number = "1337", data = "SetMark")]
pub struct SetMark;

/// Bring iTerm2 window to the foreground.
///
/// Force the terminal to steal focus from other applications.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, VTControl)]
#[vtctl(osc, number = "1337", data = "StealFocus")]
pub struct StealFocus;

/// Clear the scrollback history.
///
/// Erase all content in the scrollback buffer, keeping only the
/// visible screen content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, VTControl)]
#[vtctl(osc, number = "1337", data = "ClearScrollback")]
pub struct ClearScrollback;

/// End a copy-to-clipboard operation.
///
/// Marks the end of text being copied to the pasteboard. Must be
/// preceded by a `CopyToClipboard` command.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, VTControl)]
#[vtctl(osc, number = "1337", data = "EndCopy")]
pub struct EndCopy;

/// Report the cell size in points.
///
/// The terminal responds with:
/// `OSC 1337 ; ReportCellSize=[height];[width];[scale] ST`
///
/// where scale is the pixel-to-point ratio (1.0 for non-retina,
/// 2.0 for retina).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, VTControl)]
#[vtctl(osc, number = "1337", data = "ReportCellSize")]
pub struct ReportCellSize;

/// Push the current touch bar key labels onto a stack.
///
/// Save the current set of function key labels for later restoration
/// with `PopKeyLabels`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, VTControl)]
#[vtctl(osc, number = "1337", data = "PushKeyLabels")]
pub struct PushKeyLabels;

/// Pop touch bar key labels from the stack.
///
/// Restore the most recently pushed set of function key labels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, VTControl)]
#[vtctl(osc, number = "1337", data = "PopKeyLabels")]
pub struct PopKeyLabels;

/// Disinter a buried session.
///
/// Restore a previously buried session to the active state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, VTControl)]
#[vtctl(osc, number = "1337", data = "Disinter")]
pub struct Disinter;

/// Clear captured output.
///
/// Erase the current captured output buffer for this session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, VTControl)]
#[vtctl(osc, number = "1337", data = "ClearCapturedOutput")]
pub struct ClearCapturedOutput;

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

impl Default for CursorShapeValue {
    fn default() -> Self {
        Self::Block
    }
}

impl IntoSeq for CursorShapeValue {
    fn into_seq(&self) -> impl WriteSeq {
        *self as u8
    }
}

impl From<u8> for CursorShapeValue {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Block,
            1 => Self::VerticalBar,
            2 => Self::Underline,
            _ => Self::default(),
        }
    }
}

impl From<&EscapeSequenceParam> for CursorShapeValue {
    fn from(param: &EscapeSequenceParam) -> Self {
        Self::from(param.first())
    }
}

impl From<EscapeSequenceParam> for CursorShapeValue {
    fn from(param: EscapeSequenceParam) -> Self {
        Self::from(&param)
    }
}

/// Set the cursor shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, VTControl)]
#[vtctl(osc, number = "1337", data = "CursorShape", data_sep = "=")]
pub struct CursorShape {
    pub shape: CursorShapeValue,
}

/// Set the current directory path.
///
/// Inform iTerm2 of the current working directory to enable
/// semantic history and other path-based features.
#[derive(Debug, Clone, PartialEq, Eq, Hash, VTControl)]
#[vtctl(osc, number = "1337", data = "CurrentDir", data_sep = "=")]
pub struct CurrentDir<'a> {
    pub path: &'a str,
}

/// Change the session's profile.
///
/// Switch to a different profile by name. The profile must exist
/// in iTerm2's configuration.
#[derive(Debug, Clone, PartialEq, Eq, Hash, VTControl)]
#[vtctl(osc, number = "1337", data = "SetProfile", data_sep = "=")]
pub struct SetProfile<'a> {
    pub profile: &'a str,
}

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

impl Default for Clipboard {
    fn default() -> Self {
        Self::General
    }
}

impl From<&EscapeSequenceParam> for Clipboard {
    fn from(param: &EscapeSequenceParam) -> Self {
        let s = String::from_utf8_lossy(param);
        match &s {
            Cow::Borrowed("rule") => Self::Rule,
            Cow::Borrowed("find") => Self::Find,
            Cow::Borrowed("font") => Self::Font,
            _ => Self::default(),
        }
    }
}

impl From<EscapeSequenceParam> for Clipboard {
    fn from(param: EscapeSequenceParam) -> Self {
        Self::from(&param)
    }
}

/// Start copying text to a clipboard.
///
/// All text received after this command is placed in the specified
/// pasteboard until `EndCopy` is received.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, VTControl)]
#[vtctl(osc, number = "1337", data = "CopyToClipboard", data_sep = "=")]
pub struct CopyToClipboard {
    pub clipboard: Clipboard,
}

/// Set background image from a file path.
///
/// The value should be a base64-encoded filename. An empty string
/// removes the background image. User confirmation is required as
/// a security measure.
#[derive(Debug, Clone, PartialEq, Eq, Hash, VTControl)]
#[vtctl(osc, number = "1337", data = "SetBackgroundImageFile", data_sep = "=")]
pub struct SetBackgroundImageFile<'a> {
    pub base64_path: &'a str,
}

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

impl Default for AttentionMode {
    fn default() -> Self {
        Self::No
    }
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

impl From<&EscapeSequenceParam> for AttentionMode {
    fn from(param: &EscapeSequenceParam) -> Self {
        let s = String::from_utf8_lossy(param);
        match &s {
            Cow::Borrowed("yes") => Self::Yes,
            Cow::Borrowed("once") => Self::Once,
            Cow::Borrowed("no") => Self::No,
            Cow::Borrowed("fireworks") => Self::Fireworks,
            _ => Self::default(),
        }
    }
}

impl From<EscapeSequenceParam> for AttentionMode {
    fn from(param: EscapeSequenceParam) -> Self {
        Self::from(&param)
    }
}

/// Request attention with visual effects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, VTControl)]
#[vtctl(osc, number = "1337", data = "RequestAttention", data_sep = "=")]
pub struct RequestAttention {
    pub mode: AttentionMode,
}

/// Unicode version values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnicodeVersionValue {
    /// Unicode 8 width tables.
    V8 = 8,
    /// Unicode 9 width tables.
    V9 = 9,
}

impl Default for UnicodeVersionValue {
    fn default() -> Self {
        Self::V9
    }
}

impl IntoSeq for UnicodeVersionValue {
    fn into_seq(&self) -> impl WriteSeq {
        *self as u8
    }
}

impl From<u8> for UnicodeVersionValue {
    fn from(value: u8) -> Self {
        match value {
            8 => Self::V8,
            9 => Self::V9,
            _ => Self::default(),
        }
    }
}

impl From<&EscapeSequenceParam> for UnicodeVersionValue {
    fn from(param: &EscapeSequenceParam) -> Self {
        Self::from(param.first())
    }
}

impl From<EscapeSequenceParam> for UnicodeVersionValue {
    fn from(param: EscapeSequenceParam) -> Self {
        Self::from(&param)
    }
}

/// Set Unicode width table version.
///
/// Can also push/pop values on a stack using special string values
/// (use `GenericCommand` for that).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, VTControl)]
#[vtctl(osc, number = "1337", data = "UnicodeVersion", data_sep = "=")]
pub struct UnicodeVersion {
    pub version: UnicodeVersionValue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
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

impl From<&EscapeSequenceParam> for Iterm2Bool {
    fn from(param: &EscapeSequenceParam) -> Self {
        let s = String::from_utf8_lossy(param);
        match &s {
            Cow::Borrowed("yes") => Self(true),
            Cow::Borrowed("no") => Self(false),
            _ => Self::default(),
        }
    }
}

impl From<EscapeSequenceParam> for Iterm2Bool {
    fn from(param: EscapeSequenceParam) -> Self {
        Self::from(&param)
    }
}

/// Enable or disable the cursor guide (highlight cursor line).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, VTControl)]
#[vtctl(osc, number = "1337", data = "HighlightCursorLine", data_sep = "=")]
pub struct HighlightCursorLine {
    pub enabled: Iterm2Bool,
}

/// Copy text to the general clipboard.
#[derive(Debug, Clone, PartialEq, Eq, Hash, VTControl)]
#[vtctl(osc, number = "1337", data = "Copy", data_sep = "=")]
pub struct Copy<'a> {
    pub base64_text: &'a str,
}

/// Report the value of a session variable.
#[derive(Debug, Clone, PartialEq, Eq, Hash, VTControl)]
#[vtctl(osc, number = "1337", data = "ReportVariable", data_sep = "=")]
pub struct ReportVariable<'a> {
    pub base64_name: &'a str,
}

/// Request file upload from the user.
#[derive(Debug, Clone, PartialEq, Eq, Hash, VTControl)]
#[vtctl(osc, number = "1337", data = "RequestUpload", data_sep = "=")]
pub struct RequestUpload<'a> {
    pub format: &'a str,
}

/// Open a URL in the default browser.
#[derive(Debug, Clone, PartialEq, Eq, Hash, VTControl)]
#[vtctl(osc, number = "1337", data = "OpenUrl", data_sep = "=")]
pub struct OpenUrl<'a> {
    pub base64_url: &'a str,
}

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
