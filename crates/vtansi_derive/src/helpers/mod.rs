//! Helper functions and utilities for derive macros.

pub mod default_variant;
pub mod field_props;
pub mod metadata;
pub mod repr_type;
pub mod type_props;
pub mod variant_props;

pub use self::default_variant::{DefaultVariant, find_default_variant};
pub use self::field_props::HasFieldProperties;
pub use self::type_props::HasTypeProperties;

use proc_macro2::Span;

/// Return an error indicating that the macro can only be used on enums.
pub fn non_enum_error() -> syn::Error {
    syn::Error::new(Span::call_site(), "This macro only supports enums.")
}

/// Return an error indicating that the macro can only be used on structs.
pub fn non_struct_error() -> syn::Error {
    syn::Error::new(
        Span::call_site(),
        "This macro only supports structs with named fields.",
    )
}

/// Extract the inner type `T` from `Option<T>`.
///
/// Returns `None` if the type is not an `Option`.
#[allow(clippy::collapsible_if)]
pub fn extract_option_inner_type(ty: &syn::Type) -> Option<&syn::Type> {
    if let syn::Type::Path(type_path) = ty {
        if type_path.qself.is_some() {
            return None;
        }

        let path = &type_path.path;

        // Check for Option with generic argument
        let last_segment = path.segments.last()?;

        // Check if the last segment is "Option"
        if last_segment.ident != "Option" {
            return None;
        }

        // Check that the path is either just "Option" or "std/core::option::Option"
        let is_valid_path = if path.segments.len() == 1 {
            true
        } else if path.segments.len() == 3 {
            let segs: Vec<_> = path.segments.iter().map(|s| s.ident.to_string()).collect();
            (segs[0] == "std" || segs[0] == "core") && segs[1] == "option"
        } else {
            false
        };

        if !is_valid_path {
            return None;
        }

        // Extract the inner type from the angle brackets
        if let syn::PathArguments::AngleBracketed(args) = &last_segment.arguments {
            if args.args.len() == 1 {
                if let syn::GenericArgument::Type(inner_ty) = &args.args[0] {
                    return Some(inner_ty);
                }
            }
        }
    }

    None
}
