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

pub trait TerseDebug {
    /// Format the value in a terse format.
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
