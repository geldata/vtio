//! Helper functions and utilities for derive macros.

pub mod default_variant;
pub mod field_props;
pub mod metadata;
pub mod repr_type;
pub mod type_props;
pub mod variant_props;

pub use self::default_variant::{find_default_variant, DefaultVariant};
pub use self::field_props::HasFieldProperties;
pub use self::type_props::HasTypeProperties;

use proc_macro2::Span;

/// Return an error indicating that the macro can only be used on enums.
pub fn non_enum_error() -> syn::Error {
    syn::Error::new(Span::call_site(), "This macro only supports enums.")
}

/// Return an error indicating that the macro can only be used on structs.
pub fn non_struct_error() -> syn::Error {
    syn::Error::new(Span::call_site(), "This macro only supports structs with named fields.")
}