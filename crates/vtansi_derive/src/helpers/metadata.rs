//! Metadata parsing utilities for vtansi attributes.

use syn::{
    parse::{Parse, ParseStream},
    Variant,
};

/// Custom keywords for vtansi attributes.
pub mod kw {
    use syn::custom_keyword;

    custom_keyword!(default);
}

/// Metadata that can be attached to enum variants.
pub enum VariantMeta {
    /// `#[vtansi(default)]` - marks a variant as the default fallback
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
pub trait VariantExt {
    /// Extract all vtansi metadata from a variant's attributes.
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