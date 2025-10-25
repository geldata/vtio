//! Helper functions and utilities for derive macros.

pub mod default_variant;
pub mod repr_type;

pub use self::default_variant::{find_default_variant, DefaultVariant};
pub use self::repr_type::extract_repr_type;

use proc_macro2::Span;

/// Return an error indicating that the macro can only be used on enums.
pub fn non_enum_error() -> syn::Error {
    syn::Error::new(Span::call_site(), "This macro only supports enums.")
}