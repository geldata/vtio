//! Variant properties and metadata extraction.
//!
//! This module provides utilities for extracting structured metadata from
//! enum variant attributes. The `HasVariantProperties` trait provides a
//! uniform interface for parsing `#[vtansi(...)]` attributes on variants
//! and returning a structured representation.

use std::default::Default;
use syn::Variant;

use super::metadata::{VariantExt, VariantMeta, kw};
use super::occurrence_error;

/// Trait for extracting vtansi variant properties from AST nodes.
///
/// This trait extends `syn::Variant` with a method to extract all vtansi
/// properties in a structured form. Implementations scan the variant's
/// attributes, parse them, and validate their consistency.
pub trait HasVariantProperties {
    /// Extract vtansi properties from this variant.
    ///
    /// This method processes all `#[vtansi(...)]` attributes on the variant
    /// and returns a `VariantProperties` struct containing the parsed
    /// metadata. It also performs validation such as checking for duplicate
    /// attributes.
    ///
    /// # Errors
    ///
    /// Return an error if:
    /// - Attributes cannot be parsed
    /// - Multiple occurrences of the same attribute are found
    /// - Attribute values are invalid
    fn get_variant_properties(&self) -> syn::Result<VariantProperties>;
}

/// Properties extracted from a variant's vtansi attributes.
///
/// This struct represents all the metadata that can be specified on an enum
/// variant using `#[vtansi(...)]` attributes. Each field corresponds to one
/// possible attribute.
#[derive(Clone, Default)]
pub struct VariantProperties {
    /// The `#[vtansi(default)]` keyword, if present.
    ///
    /// When this is `Some`, the variant is marked as the default fallback
    /// for parsing failures. Only one variant in an enum can have this
    /// attribute.
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
                        return Err(occurrence_error(fst_kw, kw, "default"));
                    }

                    default_kw = Some(kw);
                    output.default = Some(kw);
                }
            }
        }

        Ok(output)
    }
}
