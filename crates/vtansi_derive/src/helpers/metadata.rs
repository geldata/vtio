//! Metadata parsing utilities for vtansi attributes.
//!
//! This module provides the core infrastructure for parsing `#[vtansi(...)]`
//! attributes on enum variants. It defines custom keywords, metadata types,
//! and extension traits for extracting structured metadata from syntax tree
//! nodes.

use syn::{
    parse::{Parse, ParseStream},
    Variant,
};

/// Custom keywords for vtansi attributes.
///
/// These keywords are recognized within `#[vtansi(...)]` attribute lists and
/// are parsed using syn's `custom_keyword!` macro.
pub mod kw {
    use syn::custom_keyword;

    custom_keyword!(default);
}

/// Metadata that can be attached to enum variants.
///
/// This enum represents all possible metadata items that can appear in a
/// `#[vtansi(...)]` attribute on an enum variant. Each variant captures the
/// keyword token for use in error reporting.
pub enum VariantMeta {
    /// `#[vtansi(default)]` - mark a variant as the default fallback.
    ///
    /// When present, this variant will be used when parsing fails to match
    /// any other variant.
    Default { kw: kw::default },
}

impl Parse for VariantMeta {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::default) {
            let kw = input.parse::<kw::default>()?;
            Ok(VariantMeta::Default { kw })
        } else {
            Err(lookahead.error())
        }
    }
}

/// Extension trait for parsing vtansi metadata from variants.
///
/// This trait extends `syn::Variant` with a method to extract all
/// `#[vtansi(...)]` attributes and parse them into structured metadata.
pub trait VariantExt {
    /// Extract all vtansi metadata from a variant's attributes.
    ///
    /// This method scans the variant's attributes, filters for those with
    /// the `vtansi` identifier, and parses their contents into
    /// `VariantMeta` values.
    ///
    /// # Errors
    ///
    /// Return an error if any attribute cannot be parsed according to the
    /// `VariantMeta` grammar.
    fn get_metadata(&self) -> syn::Result<Vec<VariantMeta>>;
}

impl VariantExt for Variant {
    fn get_metadata(&self) -> syn::Result<Vec<VariantMeta>> {
        let mut result = Vec::new();

        for attr in &self.attrs {
            if !attr.path().is_ident("vtansi") {
                continue;
            }

            let meta = attr.parse_args::<VariantMeta>()?;
            result.push(meta);
        }

        Ok(result)
    }
}