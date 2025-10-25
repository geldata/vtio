//! Type-level properties and metadata extraction.
//!
//! This module provides utilities for extracting metadata from type-level
//! attributes on structs, such as `#[vtctl(...)]` attributes. The extracted
//! properties are used to determine how to generate escape sequence
//! implementations.

use syn::ItemStruct;

use super::metadata::{IntroType, ItemStructExt, TypeMeta};

/// Trait for extracting vtctl type properties from AST nodes.
///
/// This trait provides a uniform interface for extracting type-level
/// metadata from struct AST nodes. Implementations scan the type's
/// attributes and return a structured representation of the vtctl-relevant
/// properties.
pub trait HasTypeProperties {
    /// Extract vtctl properties from this type.
    ///
    /// This method processes the type's attributes and returns a
    /// `TypeProperties` struct containing all relevant metadata for code
    /// generation.
    ///
    /// # Errors
    ///
    /// Return an error if the attributes cannot be parsed or contain
    /// invalid values.
    fn get_type_properties(&self) -> syn::Result<TypeProperties>;
}

/// Properties extracted from a struct's attributes.
///
/// This struct holds metadata parsed from `#[vtctl(...)]` attributes. It is
/// used during code generation to determine the appropriate implementation
/// strategy for escape sequences.
#[derive(Clone)]
pub struct TypeProperties {
    /// The escape sequence introducer type (CSI, ESC, OSC, etc).
    pub intro: Option<IntroType>,

    /// Optional private marker byte (e.g., `?`, `>`, `=`).
    pub private: Option<u8>,

    /// Parameter byte sequences (const params), each parameter is a vector
    /// of bytes.
    pub params: Vec<Vec<u8>>,

    /// Intermediate byte sequence (max 2 bytes).
    pub intermediate: Vec<u8>,

    /// Final byte(s) that terminate the sequence.
    pub final_bytes: Vec<u8>,

    /// Optional data string (for DCS sequences) that appears after the
    /// final byte but before the string terminator.
    pub data: Option<String>,

    /// Optional numeric parameter for OSC sequences (Ps in ESC ] Ps; Pt
    /// ST).
    pub number: Option<String>,

    /// Custom separator between static data and first field (default: ";").
    pub data_sep: String,

    /// Custom separator between parameters/fields (default: ";").
    pub param_sep: String,
}

impl Default for TypeProperties {
    fn default() -> Self {
        Self {
            intro: None,
            private: None,
            params: Vec::new(),
            intermediate: Vec::new(),
            final_bytes: Vec::new(),
            data: None,
            number: None,
            data_sep: ";".to_string(),
            param_sep: ";".to_string(),
        }
    }
}

impl HasTypeProperties for ItemStruct {
    fn get_type_properties(&self) -> syn::Result<TypeProperties> {
        let mut props = TypeProperties::default();

        // Track which attributes we've seen for duplicate detection
        let mut seen_intro = None;
        let mut seen_private = None;
        let mut seen_params = None;
        let mut seen_intermediate = None;
        let mut seen_finalbyte = None;
        let mut seen_data = None;
        let mut seen_number = None;
        let mut seen_data_sep = None;
        let mut seen_param_sep = None;

        for meta in self.get_type_metadata()? {
            match meta {
                TypeMeta::Intro { intro_type } => {
                    if let Some(first) = seen_intro {
                        return Err(duplicate_attr_error(
                            first,
                            intro_type,
                            "intro type",
                        ));
                    }
                    seen_intro = Some(intro_type);
                    props.intro = Some(intro_type);
                }
                TypeMeta::Private { kw, value } => {
                    if let Some(first_kw) = seen_private {
                        return Err(occurrence_error(
                            first_kw,
                            kw,
                            "private",
                        ));
                    }
                    seen_private = Some(kw);
                    props.private = Some(value.value() as u8);
                }
                TypeMeta::Params { kw, value } => {
                    if let Some(first_kw) = seen_params {
                        return Err(occurrence_error(first_kw, kw, "params"));
                    }
                    seen_params = Some(kw);
                    props.params = parse_params_value(&value)?;
                }
                TypeMeta::Intermediate { kw, value } => {
                    if let Some(first_kw) = seen_intermediate {
                        return Err(occurrence_error(
                            first_kw,
                            kw,
                            "intermediate",
                        ));
                    }
                    seen_intermediate = Some(kw);
                    props.intermediate = parse_intermediate_value(&value)?;
                }
                TypeMeta::FinalByte { kw, value } => {
                    if let Some(first_kw) = seen_finalbyte {
                        return Err(occurrence_error(
                            first_kw,
                            kw,
                            "finalbyte",
                        ));
                    }
                    seen_finalbyte = Some(kw);
                    props.final_bytes = parse_finalbyte_value(&value)?;
                }
                TypeMeta::Data { kw, value } => {
                    if let Some(first_kw) = seen_data {
                        return Err(occurrence_error(first_kw, kw, "data"));
                    }
                    seen_data = Some(kw);
                    props.data = Some(value.value());
                }
                TypeMeta::Number { kw, value } => {
                    if let Some(first_kw) = seen_number {
                        return Err(occurrence_error(first_kw, kw, "number"));
                    }
                    seen_number = Some(kw);
                    props.number = Some(value.value());
                }
                TypeMeta::DataSep { kw, value } => {
                    if let Some(first_kw) = seen_data_sep {
                        return Err(occurrence_error(
                            first_kw,
                            kw,
                            "data_sep",
                        ));
                    }
                    seen_data_sep = Some(kw);
                    props.data_sep = value.value();
                }
                TypeMeta::ParamSep { kw, value } => {
                    if let Some(first_kw) = seen_param_sep {
                        return Err(occurrence_error(
                            first_kw,
                            kw,
                            "param_sep",
                        ));
                    }
                    seen_param_sep = Some(kw);
                    props.param_sep = value.value();
                }
            }
        }

        Ok(props)
    }
}

/// Parse params attribute value from an array expression.
fn parse_params_value(value: &syn::Expr) -> syn::Result<Vec<Vec<u8>>> {
    use syn::{Expr, ExprLit, Lit};

    let value = unwrap_group(value);

    if let Expr::Array(arr) = value {
        let mut params = Vec::new();
        for elem in &arr.elems {
            let elem = unwrap_group(elem);
            if let Expr::Lit(ExprLit {
                lit: Lit::Str(s), ..
            }) = elem
            {
                let param: Vec<u8> =
                    s.value().chars().map(|c| c as u8).collect();
                params.push(param);
            } else {
                return Err(syn::Error::new_spanned(
                    elem,
                    "params must be an array of string literals (e.g., \
                     params = [\"6\", \"1\"])",
                ));
            }
        }
        Ok(params)
    } else {
        Err(syn::Error::new_spanned(
            value,
            "params must be an array (e.g., params = [\"6\"])",
        ))
    }
}

/// Parse intermediate attribute value from a char or string expression.
fn parse_intermediate_value(value: &syn::Expr) -> syn::Result<Vec<u8>> {
    use syn::{Expr, ExprLit, Lit};

    let value = unwrap_group(value);

    match value {
        Expr::Lit(ExprLit {
            lit: Lit::Char(ch), ..
        }) => Ok(vec![ch.value() as u8]),
        Expr::Lit(ExprLit {
            lit: Lit::Str(s), ..
        }) => {
            let bytes: Vec<u8> = s.value().chars().map(|c| c as u8).collect();
            if bytes.len() > 2 {
                return Err(syn::Error::new_spanned(
                    s,
                    "intermediate byte sequence cannot exceed 2 bytes",
                ));
            }
            Ok(bytes)
        }
        _ => Err(syn::Error::new_spanned(
            value,
            "intermediate must be a char or string literal (e.g., \
             intermediate = ' ' or intermediate = \"$ \")",
        )),
    }
}

/// Parse finalbyte attribute value from a char or string expression.
fn parse_finalbyte_value(value: &syn::Expr) -> syn::Result<Vec<u8>> {
    use syn::{Expr, ExprLit, Lit};

    let value = unwrap_group(value);

    match value {
        Expr::Lit(ExprLit {
            lit: Lit::Char(ch), ..
        }) => Ok(vec![ch.value() as u8]),
        Expr::Lit(ExprLit {
            lit: Lit::Str(s), ..
        }) => Ok(s.value().chars().map(|c| c as u8).collect()),
        _ => Err(syn::Error::new_spanned(
            value,
            "finalbyte must be a char or string literal (e.g., finalbyte = \
             'h' or finalbyte = \"hl\")",
        )),
    }
}

/// Unwrap Expr::Group to handle macro-expanded tokens.
fn unwrap_group(expr: &syn::Expr) -> &syn::Expr {
    match expr {
        syn::Expr::Group(group) => &group.expr,
        other => other,
    }
}

/// Create an error for duplicate occurrences of an attribute.
fn occurrence_error<T: quote::ToTokens>(
    fst: T,
    snd: T,
    attr: &str,
) -> syn::Error {
    let mut e = syn::Error::new_spanned(
        snd,
        format!("found multiple occurrences of vtctl({})", attr),
    );
    e.combine(syn::Error::new_spanned(fst, "first occurrence here"));
    e
}

/// Create an error for duplicate intro type declarations.
fn duplicate_attr_error(
    fst: IntroType,
    snd: IntroType,
    attr: &str,
) -> syn::Error {
    syn::Error::new(
        proc_macro2::Span::call_site(),
        format!(
            "found multiple occurrences of {} ({:?} and {:?})",
            attr, fst, snd
        ),
    )
}