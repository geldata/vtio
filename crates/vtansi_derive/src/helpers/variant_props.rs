//! Variant properties and metadata extraction.

use std::default::Default;
use syn::Variant;

use super::metadata::{kw, VariantExt, VariantMeta};

/// Trait for extracting vtansi variant properties from AST nodes.
pub trait HasVariantProperties {
    /// Extract vtansi properties from this variant.
    fn get_variant_properties(&self) -> syn::Result<VariantProperties>;
}

/// Properties extracted from a variant's vtansi attributes.
#[derive(Clone, Default)]
pub struct VariantProperties {
    /// The `#[vtansi(default)]` keyword, if present.
    pub default: Option<kw::default>,
}

impl HasVariantProperties for Variant {
    fn get_variant_properties(&self) -> syn::Result<VariantProperties> {
        let mut output = VariantProperties::default();

        let mut default_kw: Option<kw::default> = None;

        for meta in self.get_metadata()? {
            match meta {
                VariantMeta::Default { kw } => {
                    if let Some(fst_kw) = default_kw {
                        let mut err = syn::Error::new_spanned(
                            kw,
                            "Found multiple occurrences of vtansi(default)",
                        );
                        err.combine(syn::Error::new_spanned(fst_kw, "first one here"));
                        return Err(err);
                    }

                    default_kw = Some(kw);
                    output.default = Some(kw);
                }
            }
        }

        Ok(output)
    }
}