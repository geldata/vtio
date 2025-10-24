//! Procedural macros for deriving VT escape sequence traits.
//!
//! This crate provides derive macros for automatically implementing traits
//! used to represent terminal control sequences and register them with the
//! global escape sequence registry.

pub use vtio_control_derive_internal::{c0, csi, dcs, esc, terminal_mode, Control};

#[doc(hidden)]
pub mod __internal {
    pub use smallvec;
    pub use vtio_control_base;

    #[cfg(feature = "parser")]
    pub use linkme;
    #[cfg(feature = "parser")]
    pub use vtio_control_registry;
}
