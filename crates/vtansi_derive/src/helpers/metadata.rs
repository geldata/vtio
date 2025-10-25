//! Metadata parsing utilities for vtansi attributes.
//!
//! This module provides the core infrastructure for parsing `#[vtansi(...)]`
//! attributes on enum variants, structs, and fields. It defines custom
//! keywords, metadata types, and extension traits for extracting structured
//! metadata from syntax tree nodes.

use syn::{
    DeriveInput, Field, LitStr, Token, Variant,
    parse::{Parse, ParseStream},
};

/// Custom keywords for vtansi attributes.
///
/// These keywords are recognized within `#[vtansi(...)]` attribute lists and
/// are parsed using syn's `custom_keyword!` macro.
pub mod kw {
    use syn::custom_keyword;

    custom_keyword!(default);
    custom_keyword!(format);
    custom_keyword!(delimiter);
    custom_keyword!(skip);
}

/// Format style for struct encoding/decoding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StructFormat {
    /// Encode as `key=value` pairs (default).
    KeyValue,
    /// Encode as values only, in field order.
    Value,
}

impl Parse for StructFormat {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let s: LitStr = input.parse()?;
        match s.value().as_str() {
            "key=value" => Ok(StructFormat::KeyValue),
            "value" => Ok(StructFormat::Value),
            _ => Err(syn::Error::new_spanned(
                s,
                "format must be either \"key=value\" or \"value\"",
            )),
        }
    }
}

/// Metadata that can be attached to types (enums or structs).
///
/// This enum represents all possible metadata items that can appear in a
/// `#[vtansi(...)]` attribute on a type definition.
pub enum TypeMeta {
    /// `#[vtansi(format = "key=value")]` - specify struct encoding format.
    Format {
        kw: kw::format,
        format: StructFormat,
    },
    /// `#[vtansi(delimiter = ";")]` - specify field delimiter for structs.
    Delimiter {
        kw: kw::delimiter,
        delimiter: LitStr,
    },
}

impl Parse for TypeMeta {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::format) {
            let kw = input.parse::<kw::format>()?;
            input.parse::<Token![=]>()?;
            let format = input.parse()?;
            Ok(TypeMeta::Format { kw, format })
        } else if lookahead.peek(kw::delimiter) {
            let kw = input.parse::<kw::delimiter>()?;
            input.parse::<Token![=]>()?;
            let delimiter = input.parse()?;
            Ok(TypeMeta::Delimiter { kw, delimiter })
        } else {
            Err(lookahead.error())
        }
    }
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

/// Metadata that can be attached to struct fields.
///
/// This enum represents all possible metadata items that can appear in a
/// `#[vtansi(...)]` attribute on a struct field.
pub enum FieldMeta {
    /// `#[vtansi(skip)]` - skip this field during encoding/decoding.
    Skip { kw: kw::skip },
}

impl Parse for FieldMeta {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::skip) {
            let kw = input.parse::<kw::skip>()?;
            Ok(FieldMeta::Skip { kw })
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

/// Extension trait for parsing vtansi metadata from types.
///
/// This trait extends `syn::DeriveInput` with a method to extract all
/// `#[vtansi(...)]` attributes and parse them into structured metadata.
pub trait DeriveInputExt {
    /// Extract all vtansi metadata from a type's attributes.
    ///
    /// This method scans the type's attributes, filters for those with the
    /// `vtansi` identifier, and parses their contents into `TypeMeta`
    /// values.
    ///
    /// # Errors
    ///
    /// Return an error if any attribute cannot be parsed according to the
    /// `TypeMeta` grammar.
    fn get_type_metadata(&self) -> syn::Result<Vec<TypeMeta>>;
}

impl DeriveInputExt for DeriveInput {
    fn get_type_metadata(&self) -> syn::Result<Vec<TypeMeta>> {
        let mut result = Vec::new();

        for attr in &self.attrs {
            if !attr.path().is_ident("vtansi") {
                continue;
            }

            let meta = attr.parse_args::<TypeMeta>()?;
            result.push(meta);
        }

        Ok(result)
    }
}

/// Extension trait for parsing vtansi metadata from fields.
///
/// This trait extends `syn::Field` with a method to extract all
/// `#[vtansi(...)]` attributes and parse them into structured metadata.
pub trait FieldExt {
    /// Extract all vtansi metadata from a field's attributes.
    ///
    /// This method scans the field's attributes, filters for those with the
    /// `vtansi` identifier, and parses their contents into `FieldMeta`
    /// values.
    ///
    /// # Errors
    ///
    /// Return an error if any attribute cannot be parsed according to the
    /// `FieldMeta` grammar.
    fn get_metadata(&self) -> syn::Result<Vec<FieldMeta>>;
}

impl FieldExt for Field {
    fn get_metadata(&self) -> syn::Result<Vec<FieldMeta>> {
        let mut result = Vec::new();

        for attr in &self.attrs {
            if !attr.path().is_ident("vtansi") {
                continue;
            }

            let meta = attr.parse_args::<FieldMeta>()?;
            result.push(meta);
        }

        Ok(result)
    }
}
