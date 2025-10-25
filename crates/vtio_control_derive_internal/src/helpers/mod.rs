//! Helper functions and utilities for derive macros.

pub mod field_props;
pub mod metadata;
pub mod type_props;

pub use self::field_props::{
    extract_option_inner_type_cloned, is_option_type, is_unit_type,
};

use proc_macro2::Span;
use quote::ToTokens;

/// Return an error indicating that the macro can only be used on structs.
pub fn non_struct_error() -> syn::Error {
    syn::Error::new(
        Span::call_site(),
        "This macro only supports structs with named fields.",
    )
}

/// Return an error for duplicate occurrences of an attribute.
///
/// Create an error message indicating that an attribute was specified
/// multiple times, with the second occurrence as the primary error and the
/// first occurrence as a note.
pub fn occurrence_error<T: ToTokens>(fst: T, snd: T, attr: &str) -> syn::Error {
    let mut e = syn::Error::new_spanned(
        snd,
        format!("found multiple occurrences of vtctl({})", attr),
    );
    e.combine(syn::Error::new_spanned(fst, "first occurrence here"));
    e
}

/// Return an error indicating a required attribute is missing.
pub fn required_attr_error(span: Span, attr_name: &str, example: &str) -> syn::Error {
    syn::Error::new(
        span,
        format!(
            "{} attribute is required (example: {} = {})",
            attr_name, attr_name, example
        ),
    )
}

/// Return an error indicating an unsupported attribute was used.
pub fn unsupported_attr_error(span: Span, valid_attrs: &str) -> syn::Error {
    syn::Error::new(
        span,
        format!("unsupported attribute (valid attributes are: {})", valid_attrs),
    )
}

/// Get type string representation from a syn::Type.
///
/// Helper to avoid repeating the quote + to_string + trim pattern.
pub fn type_to_string(ty: &syn::Type) -> String {
    quote::quote!(#ty).to_string().replace(" ", "")
}

/// Filter intermediate bytes to exclude zero padding.
///
/// Helper to reduce repetition when processing intermediate byte sequences.
pub fn filter_intermediate_bytes(intermediate: &[u8]) -> impl Iterator<Item = &u8> {
    intermediate.iter().filter(|&&b| b != 0)
}

/// Extract field identifier from a syn::Field.
///
/// Helper to reduce repetition when extracting field names.
pub fn field_ident(field: &syn::Field) -> &syn::Ident {
    field.ident.as_ref().expect("field must have an identifier")
}