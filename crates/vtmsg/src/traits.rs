use std::fmt;

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
