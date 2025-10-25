//! Metadata parsing utilities for vtctl attributes.
//!
//! This module provides the core infrastructure for parsing `#[vtctl(...)]`
//! attributes on structs and fields. It defines custom keywords, metadata
//! types, and extension traits for extracting structured metadata from
//! syntax tree nodes.

use syn::{
    Expr, Field, ItemStruct, LitChar, LitStr, Meta, MetaNameValue, Token,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

/// Custom keywords for vtctl attributes.
///
/// These keywords are recognized within `#[vtctl(...)]` attribute lists and
/// are parsed using syn's `custom_keyword!` macro.
pub mod kw {
    use syn::custom_keyword;

    custom_keyword!(csi);
    custom_keyword!(esc);
    custom_keyword!(osc);
    custom_keyword!(dcs);
    custom_keyword!(apc);
    custom_keyword!(pm);
    custom_keyword!(sos);
    custom_keyword!(private);
    custom_keyword!(params);
    custom_keyword!(intermediate);
    custom_keyword!(finalbyte);
    custom_keyword!(data);
    custom_keyword!(number);
    custom_keyword!(data_sep);
    custom_keyword!(param_sep);
    custom_keyword!(paramidx);
}

/// Escape sequence introducer type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntroType {
    /// CSI (Control Sequence Introducer) - ESC [
    Csi,
    /// ESC - Single escape
    Esc,
    /// OSC (Operating System Command) - ESC ]
    Osc,
    /// DCS (Device Control String) - ESC P
    Dcs,
    /// APC (Application Program Command) - ESC _
    Apc,
    /// PM (Privacy Message) - ESC ^
    Pm,
    /// SOS (Start of String) - ESC X
    Sos,
}

impl Parse for IntroType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::csi) {
            input.parse::<kw::csi>()?;
            Ok(IntroType::Csi)
        } else if lookahead.peek(kw::esc) {
            input.parse::<kw::esc>()?;
            Ok(IntroType::Esc)
        } else if lookahead.peek(kw::osc) {
            input.parse::<kw::osc>()?;
            Ok(IntroType::Osc)
        } else if lookahead.peek(kw::dcs) {
            input.parse::<kw::dcs>()?;
            Ok(IntroType::Dcs)
        } else if lookahead.peek(kw::apc) {
            input.parse::<kw::apc>()?;
            Ok(IntroType::Apc)
        } else if lookahead.peek(kw::pm) {
            input.parse::<kw::pm>()?;
            Ok(IntroType::Pm)
        } else if lookahead.peek(kw::sos) {
            input.parse::<kw::sos>()?;
            Ok(IntroType::Sos)
        } else {
            Err(lookahead.error())
        }
    }
}

/// Metadata that can be attached to types (structs).
///
/// This enum represents all possible metadata items that can appear in a
/// `#[vtctl(...)]` attribute on a struct definition.
pub enum TypeMeta {
    /// Sequence introducer type (csi, esc, osc, etc).
    Intro {
        intro_type: IntroType,
    },
    /// `#[vtctl(private = '?')]` - private marker byte.
    Private {
        kw: kw::private,
        value: LitChar,
    },
    /// `#[vtctl(params = ["6", "1"])]` - constant parameter sequences.
    Params {
        kw: kw::params,
        value: Expr,
    },
    /// `#[vtctl(intermediate = ' ')]` - intermediate byte sequence.
    Intermediate {
        kw: kw::intermediate,
        value: Expr,
    },
    /// `#[vtctl(finalbyte = 'h')]` - final byte(s).
    FinalByte {
        kw: kw::finalbyte,
        value: Expr,
    },
    /// `#[vtctl(data = "...")]` - static data string.
    Data {
        kw: kw::data,
        value: LitStr,
    },
    /// `#[vtctl(number = "1")]` - OSC numeric parameter.
    Number {
        kw: kw::number,
        value: LitStr,
    },
    /// `#[vtctl(data_sep = ";")]` - separator between static data and
    /// first field.
    DataSep {
        kw: kw::data_sep,
        value: LitStr,
    },
    /// `#[vtctl(param_sep = ";")]` - separator between parameters/fields.
    ParamSep {
        kw: kw::param_sep,
        value: LitStr,
    },
}

impl Parse for TypeMeta {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        // Check for intro types first (no assignment)
        if lookahead.peek(kw::csi) {
            let intro_type = input.parse()?;
            return Ok(TypeMeta::Intro { intro_type });
        } else if lookahead.peek(kw::esc) {
            let intro_type = input.parse()?;
            return Ok(TypeMeta::Intro { intro_type });
        } else if lookahead.peek(kw::osc) {
            let intro_type = input.parse()?;
            return Ok(TypeMeta::Intro { intro_type });
        } else if lookahead.peek(kw::dcs) {
            let intro_type = input.parse()?;
            return Ok(TypeMeta::Intro { intro_type });
        } else if lookahead.peek(kw::apc) {
            let intro_type = input.parse()?;
            return Ok(TypeMeta::Intro { intro_type });
        } else if lookahead.peek(kw::pm) {
            let intro_type = input.parse()?;
            return Ok(TypeMeta::Intro { intro_type });
        } else if lookahead.peek(kw::sos) {
            let intro_type = input.parse()?;
            return Ok(TypeMeta::Intro { intro_type });
        } else if lookahead.peek(kw::private) {
            let kw = input.parse::<kw::private>()?;
            input.parse::<Token![=]>()?;
            let value = input.parse()?;
            Ok(TypeMeta::Private { kw, value })
        } else if lookahead.peek(kw::params) {
            let kw = input.parse::<kw::params>()?;
            input.parse::<Token![=]>()?;
            let value = input.parse()?;
            Ok(TypeMeta::Params { kw, value })
        } else if lookahead.peek(kw::intermediate) {
            let kw = input.parse::<kw::intermediate>()?;
            input.parse::<Token![=]>()?;
            let value = input.parse()?;
            Ok(TypeMeta::Intermediate { kw, value })
        } else if lookahead.peek(kw::finalbyte) {
            let kw = input.parse::<kw::finalbyte>()?;
            input.parse::<Token![=]>()?;
            let value = input.parse()?;
            Ok(TypeMeta::FinalByte { kw, value })
        } else if lookahead.peek(kw::data) {
            let kw = input.parse::<kw::data>()?;
            input.parse::<Token![=]>()?;
            let value = input.parse()?;
            Ok(TypeMeta::Data { kw, value })
        } else if lookahead.peek(kw::number) {
            let kw = input.parse::<kw::number>()?;
            input.parse::<Token![=]>()?;
            let value = input.parse()?;
            Ok(TypeMeta::Number { kw, value })
        } else if lookahead.peek(kw::data_sep) {
            let kw = input.parse::<kw::data_sep>()?;
            input.parse::<Token![=]>()?;
            let value = input.parse()?;
            Ok(TypeMeta::DataSep { kw, value })
        } else if lookahead.peek(kw::param_sep) {
            let kw = input.parse::<kw::param_sep>()?;
            input.parse::<Token![=]>()?;
            let value = input.parse()?;
            Ok(TypeMeta::ParamSep { kw, value })
        } else {
            Err(lookahead.error())
        }
    }
}

/// Wrapper for parsing multiple comma-separated TypeMeta items.
pub struct TypeMetaList {
    pub items: Vec<TypeMeta>,
}

impl Parse for TypeMetaList {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let items =
            Punctuated::<TypeMeta, Token![,]>::parse_terminated(input)?;
        Ok(TypeMetaList {
            items: items.into_iter().collect(),
        })
    }
}

/// Metadata that can be attached to struct fields.
///
/// This enum represents all possible metadata items that can appear in a
/// `#[paramidx(...)]` or other attribute on a struct field.
pub enum FieldMeta {
    /// `#[paramidx(0)]` - positional parameter index.
    ParamIdx {
        kw: kw::paramidx,
        index: syn::LitInt,
    },
}

impl Parse for FieldMeta {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::paramidx) {
            let kw = input.parse::<kw::paramidx>()?;
            input.parse::<Token![=]>()?;
            let index = input.parse()?;
            Ok(FieldMeta::ParamIdx { kw, index })
        } else {
            Err(lookahead.error())
        }
    }
}

/// Extension trait for parsing vtctl metadata from types.
///
/// This trait extends `syn::ItemStruct` with a method to extract all
/// `#[vtctl(...)]` attributes and parse them into structured metadata.
pub trait ItemStructExt {
    /// Extract all vtctl metadata from a struct's attributes.
    ///
    /// This method scans the struct's attributes, filters for those with
    /// the `vtctl` identifier, and parses their contents into `TypeMeta`
    /// values.
    ///
    /// # Errors
    ///
    /// Return an error if any attribute cannot be parsed according to the
    /// `TypeMeta` grammar.
    fn get_type_metadata(&self) -> syn::Result<Vec<TypeMeta>>;
}

impl ItemStructExt for ItemStruct {
    fn get_type_metadata(&self) -> syn::Result<Vec<TypeMeta>> {
        let mut result = Vec::new();

        for attr in &self.attrs {
            if !attr.path().is_ident("vtctl") {
                continue;
            }

            let meta_list = attr.parse_args::<TypeMetaList>()?;
            result.extend(meta_list.items);
        }

        Ok(result)
    }
}

/// Extension trait for parsing field metadata from fields.
///
/// This trait extends `syn::Field` with methods to extract attribute
/// metadata.
pub trait FieldExt {
    /// Extract paramidx metadata from a field's attributes.
    ///
    /// This method scans the field's attributes for `#[paramidx(...)]`
    /// and returns the parsed index if found.
    ///
    /// # Errors
    ///
    /// Return an error if the attribute cannot be parsed.
    fn get_paramidx(&self) -> syn::Result<Option<usize>>;
}

impl FieldExt for Field {
    fn get_paramidx(&self) -> syn::Result<Option<usize>> {
        for attr in &self.attrs {
            if !attr.path().is_ident("paramidx") {
                continue;
            }

            let meta = attr.parse_args::<FieldMeta>()?;
            let FieldMeta::ParamIdx { index, .. } = meta;
            return Ok(Some(index.base10_parse()?));
        }

        Ok(None)
    }
}

/// Helper function to parse name-value metadata.
///
/// Extract the key and value from a Meta::NameValue item.
pub fn parse_name_value(meta: &Meta) -> syn::Result<(String, &Expr)> {
    if let Meta::NameValue(MetaNameValue { path, value, .. }) = meta {
        if let Some(ident) = path.get_ident() {
            Ok((ident.to_string(), value))
        } else {
            Err(syn::Error::new_spanned(
                path,
                "expected identifier for attribute key",
            ))
        }
    } else {
        Err(syn::Error::new_spanned(
            meta,
            "expected name-value pair in attribute",
        ))
    }
}