//! Logic for finding and representing default variants in enums.

use syn::{DataEnum, Fields, Ident};

use super::variant_props::HasVariantProperties;

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
/// - Multiple variants are marked with `#[vtansi(default)]` (caught by
///   variant properties parsing)
/// - The default variant is not a unit or single-field tuple variant
pub fn find_default_variant(data: &DataEnum) -> syn::Result<Option<DefaultVariant>> {
    let mut default_variant = None;
    let mut default_ident: Option<&Ident> = None;

    for variant in &data.variants {
        let props = variant.get_variant_properties()?;

        if props.default.is_none() {
            continue;
        }

        // Check for duplicate default variants
        if let Some(first_ident) = default_ident {
            return Err(syn::Error::new_spanned(
                variant,
                format!(
                    "Only one variant can be marked with #[vtansi(default)]. \
                     First default variant was '{}'",
                    first_ident
                ),
            ));
        }
        default_ident = Some(&variant.ident);

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