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

use vtio_control_derive::VTControl;

/// A command that marks the beginning of a shell prompt.
///
/// This sequence (OSC 133;A) indicates where a new prompt starts. Terminal
/// emulators can use this to enable features like jumping between prompts.
///
/// # Notes
///
/// - This should be emitted at the very start of drawing the prompt.
/// - Must be paired with `PromptEnd` to mark where the prompt ends.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, VTControl)]
#[osc(number = "133", data = "A")]
pub struct PromptStart;

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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, VTControl)]
#[osc(number = "133", data = "B")]
pub struct PromptEnd;

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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, VTControl)]
#[osc(number = "133", data = "C")]
pub struct CommandStart;

/// A command that marks the end of command output.
///
/// This sequence (OSC 133;D or OSC 133;D;exit_code) indicates where the
/// command output ends. It can optionally include the command's exit code.
/// Terminal emulators can use this to track command execution status and
/// enable features like showing success/failure indicators.
///
/// # Notes
///
/// - This should be emitted after a command finishes execution.
/// - Should follow a `CommandStart` sequence.
/// - The exit code parameter is optional.
///
/// # Example
///
/// ```ignore
/// // Report command completion without exit code
/// let end = CommandEnd::new(None);
///
/// // Report successful command completion
/// let end = CommandEnd::new(Some(0));
///
/// // Report command failure
/// let end = CommandEnd::new(Some(1));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, VTControl)]
#[osc(number = "133", data = "D")]
pub struct CommandEnd {
    pub exit_code: Option<i32>,
}

