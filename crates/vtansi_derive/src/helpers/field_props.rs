//! Field properties and metadata extraction.
//!
//! This module provides utilities for extracting structured metadata from
//! struct field attributes. The `HasFieldProperties` trait provides a
//! uniform interface for parsing `#[vtansi(...)]` attributes on fields and
//! returning a structured representation.

use std::default::Default;
use syn::Field;

use super::metadata::{FieldExt, FieldMeta, kw};

/// Trait for extracting vtansi field properties from AST nodes.
///
/// This trait extends `syn::Field` with a method to extract all vtansi
/// properties in a structured form. Implementations scan the field's
/// attributes, parse them, and validate their consistency.
pub trait HasFieldProperties {
    /// Extract vtansi properties from this field.
    ///
    /// This method processes all `#[vtansi(...)]` attributes on the field
    /// and returns a `FieldProperties` struct containing the parsed
    /// metadata.
    ///
    /// # Errors
    ///
    /// Return an error if:
    /// - Attributes cannot be parsed
    /// - Multiple occurrences of the same attribute are found
    /// - Attribute values are invalid
    fn get_field_properties(&self) -> syn::Result<FieldProperties>;
}

/// Properties extracted from a field's vtansi attributes.
///
/// This struct represents all the metadata that can be specified on a struct
/// field using `#[vtansi(...)]` attributes. Each field corresponds to one
/// possible attribute.
#[derive(Clone, Default)]
pub struct FieldProperties {
    /// The `#[vtansi(skip)]` keyword, if present.
    ///
    /// When this is `Some`, the field will be skipped during encoding and
    /// decoding.
    pub skip: Option<kw::skip>,
}

impl HasFieldProperties for Field {
    fn get_field_properties(&self) -> syn::Result<FieldProperties> {
        let mut output = FieldProperties::default();

        let mut skip_kw: Option<kw::skip> = None;

        for meta in self.get_metadata()? {
            match meta {
                FieldMeta::Skip { kw } => {
                    if let Some(fst_kw) = skip_kw {
                        let mut err = syn::Error::new_spanned(
                            kw,
                            "Found multiple occurrences of vtansi(skip)",
                        );
                        err.combine(syn::Error::new_spanned(fst_kw, "first one here"));
                        return Err(err);
                    }

                    skip_kw = Some(kw);
                    output.skip = Some(kw);
                }
            }
        }

        Ok(output)
    }
}
