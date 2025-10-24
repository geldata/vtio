//! Procedural macros for deriving VT escape sequence traits.
//!
//! This crate provides derive macros for automatically implementing traits
//! used to represent terminal control sequences and register them with the
//! global escape sequence registry.

pub use vtio_control_derive_internal::VTControl;

/// Generate terminal mode control sequences.
///
/// This macro generates four control sequence structs for a terminal mode:
/// - `Enable{Name}`: CSI sequence with 'h' final byte to enable the mode
/// - `Disable{Name}`: CSI sequence with 'l' final byte to disable the mode
/// - `Request{Name}`: CSI sequence with '$' intermediate and 'p' final byte
///   to request the mode state
/// - `{Name}`: CSI sequence with '$' intermediate and 'y' final byte
///   representing the mode state response (with `enabled: bool` field)
///
/// # Syntax
///
/// ```ignore
/// terminal_mode!(ModeName, params = ["param_value"]);
/// terminal_mode!(ModeName, private = '?', params = ["param_value"]);
/// ```
///
/// # Parameters
///
/// - `ModeName`: The base identifier for the mode
/// - `private`: Optional private parameter character (e.g., '?')
/// - `params`: Required parameter array for the CSI sequence
///
/// # Example
///
/// ```ignore
/// terminal_mode!(RelativeCursorOriginMode, private = '?', params = ["6"]);
/// ```
///
/// This generates:
/// - `EnableRelativeCursorOriginMode` → CSI ? 6 h
/// - `DisableRelativeCursorOriginMode` → CSI ? 6 l
/// - `RequestRelativeCursorOriginMode` → CSI ? 6 $ p
/// - `RelativeCursorOriginMode` → CSI ? $ y (with `enabled` field)
#[macro_export]
macro_rules! terminal_mode {
    // With private parameter
    ($base_name:ident, private = $private:literal, params = [$($params:literal),* $(,)?]) => {
        $crate::__internal::paste::paste! {
            $crate::__internal::vtio_control_derive_internal::__terminal_mode_impl!(
                [<Enable $base_name>],
                [<Disable $base_name>],
                [<Request $base_name>],
                $base_name,
                $private,
                [$($params),*]
            );
        }
    };
    
    // Without private parameter
    ($base_name:ident, params = [$($params:literal),* $(,)?]) => {
        $crate::__internal::paste::paste! {
            $crate::__internal::vtio_control_derive_internal::__terminal_mode_impl!(
                [<Enable $base_name>],
                [<Disable $base_name>],
                [<Request $base_name>],
                $base_name,
                ,
                [$($params),*]
            );
        }
    };
}

#[doc(hidden)]
pub mod __internal {
    pub use paste;
    pub use smallvec;
    pub use vtio_control_base;
    pub use vtio_control_derive_internal;

    #[cfg(feature = "parser")]
    pub use linkme;
    #[cfg(feature = "parser")]
    pub use vtio_control_registry;
}