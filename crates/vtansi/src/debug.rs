//! Terse display traits for terminal events.
//!
//! These traits provide a concise, human-readable format for terminal events,
//! suitable for test output and debugging.

use std::{any::type_name, fmt};

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

/// Format terminal events in a terse format with type name prefix.
///
/// This trait is automatically implemented for all types that implement
/// [`TerseDisplay`], wrapping the output with the type name.
pub trait TerseDebug {
    /// Format the value in a terse format with type name prefix.
    ///
    /// # Errors
    ///
    /// Return an error if formatting fails.
    fn terse_fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}

impl<T: TerseDisplay> TerseDebug for T {
    fn terse_fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Full type name: "my_crate::foo::Bar"
        let full = type_name::<Self>();
        // Just the last path segment: "Bar"
        let short = full.rsplit("::").next().unwrap_or(full);
        write!(f, "{short} ( ")?;
        <Self as TerseDisplay>::terse_fmt(self, f)?;
        write!(f, " )")
    }
}
