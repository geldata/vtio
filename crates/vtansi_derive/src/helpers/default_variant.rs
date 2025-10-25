//! Logic for finding and representing default variants in enums.

use syn::{Fields, Ident};

/// Represents a variant marked with `#[vtansi(default)]`.
pub enum DefaultVariant {
    /// A unit variant that returns a constant value.
    Unit(Ident),
    /// A tuple variant with one field that captures the unrecognized value.
    Capturing(Ident),
}

/// Find the variant marked with `#[vtansi(default)]`, if any.
///
/// Returns the variant identifier and whether it's a capturing variant
/// (tuple variant with one field).
///
/// # Errors
///
/// Returns an error if:
/// - Multiple variants are marked with `#[vtansi(default)]`
/// - The default variant is not a unit or single-field tuple variant
pub fn find_default_variant(data: &syn::DataEnum) -> syn::Result<Option<DefaultVariant>> {
    let mut default_variant = None;

    for variant in &data.variants {
        // Check for vtansi(default) attribute
        let has_default = variant.attrs.iter().any(|attr| {
            if !attr.path().is_ident("vtansi") {
                return false;
            }

            // Parse the attribute as a meta list
            if let Ok(meta) = attr.parse_args::<Ident>() {
                meta == "default"
            } else {
                false
            }
        });

        if !has_default {
            continue;
        }

        if default_variant.is_some() {
            return Err(syn::Error::new_spanned(
                variant,
                "Only one variant can be marked with #[vtansi(default)]",
            ));
        }

        // Determine if it's a capturing variant
        let dv = match &variant.fields {
            Fields::Unit => DefaultVariant::Unit(variant.ident.clone()),
            Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                DefaultVariant::Capturing(variant.ident.clone())
            }
            _ => {
                return Err(syn::Error::new_spanned(
                    variant,
                    "Default variant must be either a unit variant or a \
                     tuple variant with exactly one field",
                ));
            }
        };

        default_variant = Some(dv);
    }

    Ok(default_variant)
}