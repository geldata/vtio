//! # Shell Integration
//!
//! The `shell` module provides functionality for shell integration sequences.
//!
//! Shell integration sequences (OSC 133) enable terminal emulators to track
//! shell prompts, command input, and command output. This allows features
//! like:
//! - Jumping between prompts
//! - Selecting command output
//! - Tracking command execution status
//! - Recording command history with context
//!
//! These sequences are supported by modern terminal emulators including
//! `iTerm2`, `VSCode`, `WezTerm`, and others.

use std::io;
use vtenc::{format_osc, write_osc, ConstEncode, EncodeError, ConstEncodedLen, Encode};

/// A command that marks the beginning of a shell prompt.
///
/// This sequence (OSC 133;A) indicates where a new prompt starts. Terminal
/// emulators can use this to enable features like jumping between prompts.
///
/// # Notes
///
/// - This should be emitted at the very start of drawing the prompt.
/// - Must be paired with `PromptEnd` to mark where the prompt ends.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PromptStart;

impl ConstEncode for PromptStart {
    const STR: &'static str = format_osc!("133;A");
}

/// A command that marks the end of a shell prompt and the beginning of user
/// input.
///
/// This sequence (OSC 133;B) indicates where the prompt ends and user input
/// begins. Terminal emulators can use this to distinguish between the
/// prompt and the user's command.
///
/// # Notes
///
/// - This should be emitted right before accepting user input.
/// - Should follow a `PromptStart` sequence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PromptEnd;

impl ConstEncode for PromptEnd {
    const STR: &'static str = format_osc!("133;B");
}

/// A command that marks the start of command execution and output.
///
/// This sequence (OSC 133;C) indicates where the command output begins.
/// Terminal emulators can use this to enable features like selecting
/// command output or distinguishing input from output.
///
/// # Notes
///
/// - This should be emitted right before executing a command.
/// - Should follow a `PromptEnd` sequence.
/// - Must be paired with `CommandEnd` to mark where output ends.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CommandStart;

impl ConstEncode for CommandStart {
    const STR: &'static str = format_osc!("133;C");
}

/// A command that marks the end of command output.
///
/// This sequence (OSC 133;D) indicates where the command output ends. It
/// can optionally include the command's exit code. Terminal emulators can
/// use this to track command execution status and enable features like
/// showing success/failure indicators.
///
/// # Notes
///
/// - This should be emitted after a command finishes execution.
/// - Should follow a `CommandStart` sequence.
/// - The exit code parameter is optional.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CommandEnd {
    exit_code: Option<i32>,
}

impl CommandEnd {
    /// Create a command end marker without an exit code.
    #[must_use]
    pub const fn new() -> Self {
        Self { exit_code: None }
    }

    /// Create a command end marker with an exit code.
    ///
    /// # Arguments
    ///
    /// * `code` - The exit code of the command (typically 0 for success).
    #[must_use]
    pub const fn with_exit_code(code: i32) -> Self {
        Self {
            exit_code: Some(code),
        }
    }
}

impl Default for CommandEnd {
    fn default() -> Self {
        Self::new()
    }
}

impl ConstEncodedLen for CommandEnd {
    const ENCODED_LEN: usize = 32; // "\x1b]133;D;-2147483648\x1b\\"
}

impl Encode for CommandEnd {
    fn encode<W: io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        if let Some(code) = self.exit_code {
            write_osc!(buf; "133;D;", code)
        } else {
            write_osc!(buf; "133;D")
        }
    }
}
