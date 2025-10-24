//! Procedural macros for deriving VT escape sequence traits.
//!
//! This crate provides derive macros for automatically implementing traits
//! used to represent terminal control sequences and register them with the
//! global escape sequence registry.

use proc_macro::TokenStream;
use proc_macro2_diagnostics::{Diagnostic, SpanDiagnosticExt};
use quote::quote;
use syn::{
    Expr, ExprLit, Ident, ItemStruct, Lit, Meta, MetaNameValue, Token, parse_macro_input,
    punctuated::Punctuated, spanned::Spanned,
};

/// Emit diagnostics as TokenStream and return early.
///
/// Helper to reduce repetition when emitting diagnostic errors.
fn emit_diagnostics(diagnostics: &mut Vec<Diagnostic>) -> proc_macro2::TokenStream {
    diagnostics
        .drain(..)
        .map(|d| d.emit_as_item_tokens())
        .collect()
}

/// Create a "required attribute" diagnostic error.
///
/// Helper to reduce repetition for missing required attribute errors.
fn error_required_attr(span: proc_macro2::Span, attr_name: &str, example: &str) -> Diagnostic {
    span.error(format!("{} attribute is required", attr_name))
        .help(format!("add {} = {}", attr_name, example))
}

/// Create an "unsupported attribute" diagnostic error.
///
/// Helper to reduce repetition for unsupported attribute errors.
#[allow(dead_code)]
fn error_unsupported_attr(span: proc_macro2::Span, valid_attrs: &str) -> Diagnostic {
    span.error("unsupported attribute")
        .help(format!("valid attributes are: {}", valid_attrs))
}

/// Get type string representation from a syn::Type.
///
/// Helper to avoid repeating the quote + to_string + trim pattern.
fn type_to_string(ty: &syn::Type) -> String {
    quote!(#ty).to_string().replace(" ", "")
}

/// Check if a type is a unit type `()`.
///
/// Helper to reduce repetition when filtering unit type fields.
fn is_unit_type(ty: &syn::Type) -> bool {
    type_to_string(ty) == "()"
}

/// Check if a type is Option<T>.
///
/// Helper to detect optional fields for conditional encoding.
fn is_option_type(ty: &syn::Type) -> bool {
    let ty_str = type_to_string(ty);
    ty_str.starts_with("Option<") || ty_str.starts_with("std::option::Option<")
}

/// Extract the inner type from Option<T>.
///
/// Returns the inner type T if the type is Option<T>, otherwise returns the type itself.
#[allow(dead_code)]
fn extract_option_inner_type(ty: &syn::Type) -> Option<syn::Type> {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == "Option" {
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first() {
                        return Some(inner_ty.clone());
                    }
                }
            }
        }
    }
    None
}

/// Filter intermediate bytes to exclude zero padding.
///
/// Helper to reduce repetition when processing intermediate byte sequences.
fn filter_intermediate_bytes(intermediate: &[u8]) -> impl Iterator<Item = &u8> {
    intermediate.iter().filter(|&&b| b != 0)
}

/// Extract field identifier from a syn::Field.
///
/// Helper to reduce repetition when extracting field names.
fn field_ident(field: &syn::Field) -> &syn::Ident {
    field.ident.as_ref().expect("field must have an identifier")
}

/// Builder for generating write operation token streams.
///
/// Consolidates repetitive write operation generation patterns.
struct WriteOperationBuilder {
    operations: Vec<proc_macro2::TokenStream>,
}

impl WriteOperationBuilder {
    /// Create a new write operation builder.
    fn new() -> Self {
        Self {
            operations: Vec::new(),
        }
    }

    /// Add a string write operation.
    fn write_str(&mut self, s: &str) -> &mut Self {
        self.operations.push(generate_write_str(s));
        self
    }

    /// Add a character write operation.
    fn write_char(&mut self, ch: char) -> &mut Self {
        self.operations.push(generate_write_char(ch));
        self
    }

    /// Add a field write operation.
    fn write_field(&mut self, field_name: &syn::Ident) -> &mut Self {
        self.operations.push(generate_write_field(field_name));
        self
    }

    /// Add a raw token stream operation.
    fn add_raw(&mut self, op: proc_macro2::TokenStream) -> &mut Self {
        self.operations.push(op);
        self
    }

    /// Add a separator if the index is greater than 0.
    fn write_separator(&mut self, index: usize) -> &mut Self {
        if index > 0 {
            self.operations.push(generate_separator(index));
        }
        self
    }

    /// Add intermediate bytes, filtering out zero padding.
    fn write_intermediate_bytes(&mut self, intermediate: &[u8]) -> &mut Self {
        for &byte in filter_intermediate_bytes(intermediate) {
            self.write_char(byte as char);
        }
        self
    }

    /// Add optional private byte.
    fn write_private(&mut self, private: Option<u8>) -> &mut Self {
        if let Some(byte) = private {
            self.write_char(byte as char);
        }
        self
    }

    /// Build the final token stream.
    fn build(self) -> Vec<proc_macro2::TokenStream> {
        self.operations
    }
}

/// Builder for constructing const escape sequence strings.
///
/// Consolidates repetitive string building patterns.
struct ConstStrBuilder {
    result: String,
}

impl ConstStrBuilder {
    /// Create a new const string builder with an introducer.
    fn new(intro: &str) -> Self {
        Self {
            result: get_intro_str(intro).to_string(),
        }
    }

    /// Add optional private marker byte.
    fn private(mut self, private: Option<u8>) -> Self {
        if let Some(byte) = private {
            self.result.push(byte as char);
        }
        self
    }

    /// Add parameter sequences.
    fn params(mut self, params: &[Vec<u8>]) -> Self {
        for (i, param) in params.iter().enumerate() {
            if i > 0 {
                self.result.push(';');
            }
            self.result.push_str(&String::from_utf8_lossy(param));
        }
        self
    }

    /// Add intermediate bytes, filtering out zero padding.
    fn intermediate(mut self, intermediate: &[u8]) -> Self {
        for &byte in filter_intermediate_bytes(intermediate) {
            self.result.push(byte as char);
        }
        self
    }

    /// Add final byte.
    fn final_byte(mut self, final_byte: u8) -> Self {
        self.result.push(final_byte as char);
        self
    }

    /// Add optional data string (for DCS sequences).
    fn data(mut self, data: Option<&str>) -> Self {
        if let Some(data_str) = data {
            self.result.push_str(data_str);
        }
        self
    }

    /// Add OSC number parameter (Ps in ESC ] Ps; Pt ST).
    fn osc_number(mut self, number: Option<&str>) -> Self {
        if let Some(num) = number {
            self.result.push_str(num);
            self.result.push(';');
        }
        self
    }

    /// Add string terminator for specific sequence types.
    fn string_terminator(mut self, intro: &str) -> Self {
        if matches!(intro, "DCS" | "OSC" | "PM" | "APC") {
            self.result.push_str("\x1B\\");
        }
        self
    }

    /// Build the final const string.
    fn build(self) -> String {
        self.result
    }
}

/// Generate a separator token stream for indexed items.
///
/// Returns a semicolon write statement if index > 0, empty otherwise.
fn generate_separator(index: usize) -> proc_macro2::TokenStream {
    if index > 0 {
        quote! { __total += ::vtenc::encode::write_str_into(buf, ";")?; }
    } else {
        quote! {}
    }
}

/// Generate a write statement for a string literal.
///
/// Helper to generate quote! blocks for writing strings to buffers.
fn generate_write_str(s: &str) -> proc_macro2::TokenStream {
    quote! {
        __total += ::vtenc::encode::write_str_into(buf, #s)?;
    }
}

/// Generate a write statement for a character.
///
/// Helper to generate quote! blocks for writing characters to buffers.
fn generate_write_char(ch: char) -> proc_macro2::TokenStream {
    quote! {
        __total += ::vtenc::encode::WriteSeq::write_seq(&(#ch), buf)?;
    }
}

/// Generate a write statement for a field.
///
/// Helper to generate quote! blocks for writing struct fields to buffers.
fn generate_write_field(field_name: &syn::Ident) -> proc_macro2::TokenStream {
    quote! {
        __total += ::vtenc::encode::WriteSeq::write_seq(&self.#field_name, buf)?;
    }
}

/// Parse a string literal from an attribute value.
///
/// Generic helper to extract string literals with proper error handling.
fn parse_string_literal(
    value: &Expr,
    attr_name: &str,
    example: &str,
    diagnostics: &mut Vec<Diagnostic>,
) -> Option<String> {
    // Unwrap Expr::Group to handle macro-expanded tokens
    let value = match value {
        Expr::Group(group) => &*group.expr,
        other => other,
    };

    if let Expr::Lit(ExprLit {
        lit: Lit::Str(s), ..
    }) = value
    {
        Some(s.value())
    } else {
        diagnostics.push(
            value
                .span()
                .error(format!("{} must be a string literal", attr_name))
                .help(format!("example: {} = \"{}\"", attr_name, example)),
        );
        None
    }
}

/// Parse a name-value attribute pair.
///
/// Helper to extract key-value pairs from Meta::NameValue with unified error handling.
fn parse_name_value_attr<'a>(
    meta: &'a Meta,
    diagnostics: &mut Vec<Diagnostic>,
) -> Option<(String, &'a Expr)> {
    if let Meta::NameValue(MetaNameValue { path, value, .. }) = meta {
        if let Some(ident) = path.get_ident() {
            return Some((ident.to_string(), value));
        } else {
            diagnostics.push(
                path.span()
                    .error("expected identifier")
                    .help("attribute key must be a simple identifier"),
            );
        }
    }
    None
}

/// Parsed escape sequence attribute values.
///
/// This structure holds the intermediate parsed representation of all escape
/// sequence attributes before they are validated and converted to code
/// generation tokens.
struct EscapeSequenceAttributes {
    /// Optional private marker byte (e.g., `?`, `>`, `=`).
    private: Option<u8>,
    /// Parameter byte sequences (const params), each parameter is a vector of bytes.
    params: Vec<Vec<u8>>,
    /// Intermediate byte sequence (max 2 bytes).
    intermediate: Vec<u8>,
    /// Final byte(s) that terminate the sequence. Can be single or multiple.
    final_bytes: Vec<u8>,
    /// Optional data string (for DCS sequences) that appears after the
    /// final byte but before the string terminator.
    data: Option<String>,
    /// Optional numeric parameter for OSC sequences (Ps in ESC ] Ps; Pt ST).
    number: Option<String>,
    /// Custom separator between static data and first field (default: ";").
    data_sep: Option<String>,
    /// Custom separator between parameters/fields (default: ";").
    param_sep: Option<String>,
}

/// Parse a character literal attribute and convert to u8.
///
/// Generic helper to extract a byte from a character literal.
fn parse_char_as_byte(
    value: &Expr,
    attr_name: &str,
    example: &str,
    diagnostics: &mut Vec<Diagnostic>,
) -> Option<u8> {
    // Unwrap Expr::Group to handle macro-expanded tokens
    let value = match value {
        Expr::Group(group) => &*group.expr,
        other => other,
    };

    if let Expr::Lit(ExprLit {
        lit: Lit::Char(ch), ..
    }) = value
    {
        Some(ch.value() as u8)
    } else {
        diagnostics.push(
            value
                .span()
                .error(format!("{} must be a char literal", attr_name))
                .help(format!("example: {} = '{}'", attr_name, example)),
        );
        None
    }
}

/// Parse the private marker attribute.
///
/// Extract a single byte from a character literal like `private = '?'`.
fn parse_private(value: &Expr, diagnostics: &mut Vec<Diagnostic>) -> Option<u8> {
    parse_char_as_byte(value, "private", "?", diagnostics)
}

/// Parse the params array attribute for const params.
///
/// Extract a vector of byte sequences from an array of string literals like
/// `params = ["6", "1"]`. Each string is converted to a vector of bytes.
fn parse_params(value: &Expr, diagnostics: &mut Vec<Diagnostic>) -> Vec<Vec<u8>> {
    let mut params = Vec::new();

    // Unwrap Expr::Group to handle macro-expanded tokens
    let value = match value {
        Expr::Group(group) => &*group.expr,
        other => other,
    };

    if let Expr::Array(arr) = value {
        for elem in arr.elems.iter() {
            // Unwrap Expr::Group for array elements too
            let elem = match elem {
                Expr::Group(group) => &*group.expr,
                other => other,
            };

            if let Expr::Lit(ExprLit {
                lit: Lit::Str(s), ..
            }) = elem
            {
                let param: Vec<u8> = s.value().chars().map(|c| c as u8).collect();
                params.push(param);
            } else {
                diagnostics.push(
                    elem.span()
                        .error("params must be an array of string literals")
                        .help("example: params = [\"6\", \"1\"]"),
                );
            }
        }
    } else {
        diagnostics.push(
            value
                .span()
                .error("params must be an array")
                .help("example: params = [\"6\"]"),
        );
    }

    params
}

/// Extract variable parameters from struct fields.
///
/// If the struct has named fields, extract them as variable parameters.
/// Returns None if the struct has no fields (unit struct).
/// Filters out unit type fields (those with type `()`).
fn extract_var_params_from_struct(input: &ItemStruct) -> Option<Vec<(String, syn::Type)>> {
    match &input.fields {
        syn::Fields::Named(fields) => {
            let params: Vec<_> = fields
                .named
                .iter()
                .filter_map(|field| {
                    let name = field_ident(field).to_string();
                    let ty = field.ty.clone();

                    // Skip unit type fields
                    if is_unit_type(&ty) {
                        return None;
                    }

                    Some((name, ty))
                })
                .collect();
            if params.is_empty() {
                None
            } else {
                Some(params)
            }
        }
        syn::Fields::Unnamed(_) | syn::Fields::Unit => None,
    }
}

/// Represents a DCS/OSC data parameter (either a field or a constant literal).
#[derive(Clone)]
enum DcsDataParam {
    /// A required field from the struct
    Field(Box<(String, syn::Type)>),
    /// An optional field from the struct (Option<T>)
    OptionalField(Box<(String, syn::Type)>),
    /// A constant literal value
    Literal(String),
}

/// Represents a positional parameter for OSC/DCS sequences.
#[derive(Clone)]
enum PositionalParam {
    /// A required positional parameter
    Required(String, syn::Type),
    /// An optional positional parameter (must come after all required ones)
    Optional(String, syn::Type),
}

/// Extract paramidx attribute from a field if present.
///
/// Returns the parameter index if the field has a `#[vtctl(paramidx = N)]`
/// attribute, indicating it should be parsed from parameter N but not
/// encoded as a separate parameter.
fn get_paramidx_attr(field: &syn::Field) -> Option<usize> {
    for attr in &field.attrs {
        if attr.path().is_ident("vtctl") {
            if let Ok(list) = attr.meta.require_list() {
                if let Ok(meta_list) = list.parse_args_with(
                    Punctuated::<Meta, Token![,]>::parse_terminated
                ) {
                    for meta in meta_list {
                        if let Meta::NameValue(nv) = meta {
                            if nv.path.is_ident("paramidx") {
                                if let Expr::Lit(ExprLit {
                                    lit: Lit::Int(int_lit), ..
                                }) = &nv.value
                                {
                                    if let Ok(idx) = int_lit.base10_parse::<usize>() {
                                        return Some(idx);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

/// Remove helper attributes from struct fields.
///
/// Filter out attributes like `#[vtctl(...)]` that are used by the macro but
/// should not appear in the generated output.
fn remove_helper_attributes(mut input: ItemStruct) -> ItemStruct {
    if let syn::Fields::Named(ref mut fields) = input.fields {
        for field in fields.named.iter_mut() {
            field.attrs.retain(|attr| !attr.path().is_ident("vtctl"));
        }
    }
    input
}

/// Check if a field has the `#[vtctl(positional)]` attribute.
fn is_positional_field(field: &syn::Field) -> bool {
    field.attrs.iter().any(|attr| {
        if attr.path().is_ident("vtctl") {
            if let Ok(list) = attr.meta.require_list() {
                if let Ok(nested) = list.parse_args::<syn::Ident>() {
                    return nested == "positional";
                }
            }
        }
        false
    })
}

/// Extract positional parameters from struct fields.
///
/// Validates that optional positionals come after required ones and returns
/// an ordered list of positional parameters.
fn extract_positional_params(
    input: &ItemStruct,
    diagnostics: &mut Vec<Diagnostic>,
) -> Option<Vec<PositionalParam>> {
    match &input.fields {
        syn::Fields::Named(fields) => {
            let mut params = Vec::new();
            let mut seen_optional = false;

            for field in &fields.named {
                if !is_positional_field(field) {
                    continue;
                }

                let name = field_ident(field).to_string();
                let ty = &field.ty;

                if is_unit_type(ty) {
                    diagnostics.push(
                        field.span()
                            .error("positional parameter cannot be unit type"),
                    );
                    continue;
                }

                let is_optional = is_option_type(ty);

                if is_optional {
                    seen_optional = true;
                    let inner_ty = extract_option_inner_type(ty).unwrap_or_else(|| ty.clone());
                    params.push(PositionalParam::Optional(name, inner_ty));
                } else {
                    if seen_optional {
                        diagnostics.push(
                            field.span()
                                .error("required positional parameter must come before optional ones")
                                .help("reorder fields so that all required positionals come first"),
                        );
                    }
                    params.push(PositionalParam::Required(name, ty.clone()));
                }
            }

            if params.is_empty() {
                None
            } else {
                Some(params)
            }
        }
        syn::Fields::Unnamed(_) | syn::Fields::Unit => None,
    }
}

/// Extract DCS data parameters from struct fields, including literal annotations.
fn extract_dcs_data_params(input: &ItemStruct) -> Option<Vec<DcsDataParam>> {
    match &input.fields {
        syn::Fields::Named(fields) => {
            let params: Vec<_> = fields
                .named
                .iter()
                .filter_map(|field| {
                    let name = field_ident(field).to_string();

                    // Check for #[vtctl(literal = "value")] attribute
                    for attr in &field.attrs {
                        if attr.path().is_ident("vtctl") {
                            if let Ok(list) = attr.meta.require_list() {
                                if let Ok(nested) = list.parse_args_with(
                                    syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated
                                ) {
                                    for meta in nested {
                                        if let syn::Meta::NameValue(syn::MetaNameValue {
                                            path,
                                            value: syn::Expr::Lit(syn::ExprLit {
                                                lit: syn::Lit::Str(ref lit_str), ..
                                            }), ..
                                        }) = meta {
                                            if path.is_ident("literal") {
                                                return Some(DcsDataParam::Literal(lit_str.value()));
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Regular field - check if it's a unit type (these are for const markers)
                    let ty = &field.ty;
                    if is_unit_type(ty) {
                        // Unit type without #[vtctl(literal = ...)] is skipped
                        return None;
                    }

                    // Check if this is an Option<T> field
                    if is_option_type(ty) {
                        Some(DcsDataParam::OptionalField(Box::new((name, field.ty.clone()))))
                    } else {
                        Some(DcsDataParam::Field(Box::new((name, field.ty.clone()))))
                    }
                })
                .collect();
            if params.is_empty() {
                None
            } else {
                Some(params)
            }
        }
        syn::Fields::Unnamed(_) | syn::Fields::Unit => None,
    }
}

/// Parse the intermediate bytes attribute.
///
/// Extract up to 2 bytes from a string literal like `intermediate = " "`.
/// Validates that the string has at most 2 characters.
fn parse_intermediate(value: &Expr, diagnostics: &mut Vec<Diagnostic>) -> Vec<u8> {
    if let Some(s) = parse_string_literal(value, "intermediate", " ", diagnostics) {
        let intermediate: Vec<u8> = s.chars().map(|c| c as u8).collect();
        if intermediate.len() > 2 {
            diagnostics.push(
                value
                    .span()
                    .error("intermediate must have at most 2 characters")
                    .help("intermediate bytes are limited to 2 characters"),
            );
        }
        intermediate
    } else {
        Vec::new()
    }
}

/// Parse the final byte(s) attribute.
///
/// Extract one or more bytes from a character literal like `finalbyte = 'h'`
/// or an array of character literals like `finalbyte = ['M', 'm']`.
fn parse_finalbytes(value: &Expr, diagnostics: &mut Vec<Diagnostic>) -> Vec<u8> {
    // Unwrap Expr::Group to handle macro-expanded tokens
    let value = match value {
        Expr::Group(group) => &*group.expr,
        other => other,
    };

    // Check if it's an array
    if let Expr::Array(arr) = value {
        let mut bytes = Vec::new();
        for elem in arr.elems.iter() {
            let elem = match elem {
                Expr::Group(group) => &*group.expr,
                other => other,
            };

            if let Expr::Lit(ExprLit {
                lit: Lit::Char(ch), ..
            }) = elem
            {
                bytes.push(ch.value() as u8);
            } else {
                diagnostics.push(
                    elem.span()
                        .error("finalbyte array must contain character literals")
                        .help("example: finalbyte = ['M', 'm']"),
                );
            }
        }
        bytes
    } else if let Some(byte) = parse_char_as_byte(value, "finalbyte", "h", diagnostics) {
        // Single character
        vec![byte]
    } else {
        Vec::new()
    }
}

/// Parse the data attribute.
///
/// Extract a string from a string literal like `data = " q"`.
fn parse_data(value: &Expr, diagnostics: &mut Vec<Diagnostic>) -> Option<String> {
    parse_string_literal(value, "data", " q", diagnostics)
}

/// Parse the number attribute.
///
/// Extract a string from a string literal like `number = "133"`.
fn parse_number(value: &Expr, diagnostics: &mut Vec<Diagnostic>) -> Option<String> {
    parse_string_literal(value, "number", "133", diagnostics)
}

/// Parse a separator string attribute.
///
/// Extract a separator string from an attribute like `data_sep = "="`.
fn parse_separator(value: &Expr, attr_name: &str, diagnostics: &mut Vec<Diagnostic>) -> Option<String> {
    parse_string_literal(value, attr_name, "=", diagnostics)
}

/// Parse all escape sequence attributes from the macro input.
///
/// Process the punctuated list of meta items from the attribute macro,
/// dispatching to specialized parsers for each attribute type. Collects all
/// diagnostics encountered during parsing.
fn parse_escape_sequence_attributes(
    meta_list: Punctuated<Meta, Token![,]>,
    diagnostics: &mut Vec<Diagnostic>,
) -> EscapeSequenceAttributes {
    let mut attrs = EscapeSequenceAttributes {
        private: None,
        params: Vec::new(),
        intermediate: Vec::new(),
        final_bytes: Vec::new(),
        data: None,
        number: None,
        data_sep: None,
        param_sep: None,
    };

    for meta in meta_list {
        if let Some((key, value)) = parse_name_value_attr(&meta, diagnostics) {
            match key.as_str() {
                "private" => attrs.private = parse_private(value, diagnostics),
                "params" => attrs.params = parse_params(value, diagnostics),
                "intermediate" => attrs.intermediate = parse_intermediate(value, diagnostics),
                "finalbyte" => attrs.final_bytes = parse_finalbytes(value, diagnostics),
                "data" => attrs.data = parse_data(value, diagnostics),
                "number" => attrs.number = parse_number(value, diagnostics),
                "data_sep" => attrs.data_sep = parse_separator(value, "data_sep", diagnostics),
                "param_sep" => attrs.param_sep = parse_separator(value, "param_sep", diagnostics),
                unknown => {
                    diagnostics.push(
                        meta.span()
                            .error(format!("unknown attribute: {}", unknown))
                            .help(
                                "valid attributes are: private, params, intermediate, finalbyte, data, number, data_sep, param_sep",
                            ),
                    );
                }
            }
        } else if !matches!(meta, Meta::NameValue(_)) {
            diagnostics.push(
                meta.span()
                    .error("expected name-value pairs in attribute")
                    .help("example: #[vtctl(csi, private = '?', params = [\"6\"], finalbyte = 'h')]"),
            );
        }
    }

    attrs
}

/// Builder for generating EscapeSequence trait const declarations.
///
/// Unifies generation of PRIVATE, PARAMS, INTERMEDIATE, and FINAL constants.
struct EscapeSequenceConsts<'a> {
    private: Option<u8>,
    params: &'a [Vec<u8>],
    intermediate: &'a [u8],
    final_byte: u8,
}

impl<'a> EscapeSequenceConsts<'a> {
    /// Create new const generator.
    fn new(
        private: Option<u8>,
        params: &'a [Vec<u8>],
        intermediate: &'a [u8],
        final_byte: u8,
    ) -> Self {
        Self {
            private,
            params,
            intermediate,
            final_byte,
        }
    }

    /// Generate all EscapeSequence trait consts.
    fn generate_all(&self) -> proc_macro2::TokenStream {
        let private = self.generate_private();
        let params = self.generate_params();
        let intermediate = self.generate_intermediate();
        let final_byte = self.generate_final();

        quote! {
            #private
            #params
            #intermediate
            #final_byte
        }
    }

    /// Generate the PRIVATE constant.
    fn generate_private(&self) -> proc_macro2::TokenStream {
        if let Some(byte) = self.private {
            quote! { const PRIVATE: Option<u8> = Some(#byte); }
        } else {
            quote! { const PRIVATE: Option<u8> = None; }
        }
    }

    /// Generate the PARAMS constant with SmallVec padding.
    fn generate_params(&self) -> proc_macro2::TokenStream {
        if self.params.is_empty() {
            quote! {
                const PARAMS: ::vtio_control_derive::__internal::vtio_control_base::EscapeSequenceParams = const {
                    ::vtio_control_derive::__internal::smallvec::SmallVec::new_const()
                };
            }
        } else {
            let param_inits: Vec<_> = self
                .params
                .iter()
                .map(|param| {
                    let param_len = param.len();
                    let mut padded = param.clone();
                    padded.resize(32, 0);
                    let bytes = padded.iter();
                    quote! {
                        // SAFETY: we compute the number above and it is always
                        //         smaller than 32.
                        unsafe {
                            ::vtio_control_derive::__internal::vtio_control_base::EscapeSequenceParam::from_const_with_len_unchecked(
                                [#(#bytes),*],
                                #param_len,
                            )
                        }
                    }
                })
                .collect();

            let num_params = self.params.len();
            let padding_params = (0..(8 - num_params)).map(|_| {
                quote! {
                    ::vtio_control_derive::__internal::vtio_control_base::EscapeSequenceParam::from_u8(0u8)
                }
            });

            quote! {
                const PARAMS: ::vtio_control_derive::__internal::vtio_control_base::EscapeSequenceParams = const {
                    // SAFETY: we compute the number above and it is always
                    //         smaller than 8.
                    unsafe {
                        ::vtio_control_derive::__internal::smallvec::SmallVec::from_const_with_len_unchecked(
                            [
                                #(#param_inits,)*
                                #(#padding_params,)*
                            ],
                            #num_params,
                        )
                    }
                };
            }
        }
    }

    /// Generate the INTERMEDIATE constant with zero padding.
    fn generate_intermediate(&self) -> proc_macro2::TokenStream {
        let value = match self.intermediate.len() {
            0 => quote! { [0, 0] },
            1 => {
                let byte = self.intermediate[0];
                quote! { [#byte, 0] }
            }
            _ => {
                let byte0 = self.intermediate[0];
                let byte1 = self.intermediate[1];
                quote! { [#byte0, #byte1] }
            }
        };

        quote! { const INTERMEDIATE: [u8; 2] = #value; }
    }

    /// Generate the FINAL constant.
    fn generate_final(&self) -> proc_macro2::TokenStream {
        let final_byte = self.final_byte;
        quote! { const FINAL: u8 = #final_byte; }
    }
}

/// Generate the prefix bytes for the registry entry.
///
/// Combine private marker and params into a single byte sequence.
fn generate_prefix_bytes(private: Option<u8>, params: &[Vec<u8>]) -> Vec<u8> {
    let mut prefix = Vec::new();

    if let Some(byte) = private {
        prefix.push(byte);
    }

    for (i, param) in params.iter().enumerate() {
        if i > 0 {
            prefix.push(b';');
        }
        prefix.extend_from_slice(param);
    }

    prefix
}

/// Generate a registry entry and handler for an escape sequence.
fn generate_registry_entry(
    struct_name: &Ident,
    generics: &syn::Generics,
    intro: &str,
    prefix_bytes: &[u8],
    final_byte: u8,
    var_params: Option<&[(String, syn::Type)]>,
) -> proc_macro2::TokenStream {
    let registry_name = syn::Ident::new(
        &format!(
            "__{}_REGISTRY_ENTRY",
            struct_name.to_string().to_uppercase()
        ),
        struct_name.span(),
    );

    let handler_name = syn::Ident::new(
        &format!("{}_handler", struct_name.to_string().to_lowercase()),
        struct_name.span(),
    );

    // Helper function to generate parameter parsing code for a field.
    //
    // Convert parameter to the target type using From<EscapeSequenceParam>.
    // If paramidx is Some(idx), parse from that specific parameter index.
    fn generate_param_parsing(
        i: usize,
        name: &str,
        ty: &syn::Type,
        struct_span: proc_macro2::Span,
        paramidx: Option<usize>,
    ) -> proc_macro2::TokenStream {
        let field_name = syn::Ident::new(name, struct_span);
        let param_index = paramidx.unwrap_or(i);

        // Check if this is an optional type
        if is_option_type(ty) {
            // For Option<T>, parse as Some(value) if parameter exists, None otherwise
            let inner_ty = extract_option_inner_type(ty).unwrap_or_else(|| ty.clone());
            quote! {
                let #field_name: #ty = params
                    .get(#param_index)
                    .map(|p| <#inner_ty as ::core::convert::From<&::vtio_control_derive::__internal::vtio_control_base::EscapeSequenceParam>>::from(p));
            }
        } else {
            // For non-optional types, use unwrap_or_default
            quote! {
                let #field_name: #ty = params
                    .get(#param_index)
                    .map(|p| <#ty as ::core::convert::From<&::vtio_control_derive::__internal::vtio_control_base::EscapeSequenceParam>>::from(p))
                    .unwrap_or_default();
            }
        }
    }

    // Split generics for use in handler
    let (_, ty_generics, _) = generics.split_for_impl();
    
    // Generate handler function
    let handler_body = if let Some(params) = var_params {
        // Variable sequence - parse params and call new
        let param_parsing: Vec<_> = params
            .iter()
            .enumerate()
            .map(|(i, (name, ty))| generate_param_parsing(i, name, ty, struct_name.span(), None))
            .collect();

        let field_names: Vec<_> = params
            .iter()
            .map(|(name, _)| syn::Ident::new(name, struct_name.span()))
            .collect();

        quote! {
            #(#param_parsing)*
            let _seq = #struct_name #ty_generics::new(#(#field_names),*);
        }
    } else {
        // Const sequence - just construct unit struct
        quote! {
            let _seq = #struct_name #ty_generics;
        }
    };

    let handler_fn = quote! {
        fn #handler_name(params: &[::vtio_control_derive::__internal::vtio_control_base::EscapeSequenceParam]) {
            #handler_body
        }
    };

    let intro_variant = syn::Ident::new(intro, struct_name.span());
    let struct_name_str = struct_name.to_string();

    quote! {
        #[doc(hidden)]
        #[cfg(feature = "parser")]
        #handler_fn

        #[doc(hidden)]
        #[cfg(feature = "parser")]
        #[::vtio_control_derive::__internal::linkme::distributed_slice(::vtio_control_derive::__internal::vtio_control_registry::ESCAPE_SEQUENCE_REGISTRY)]
        static #registry_name: ::vtio_control_derive::__internal::vtio_control_registry::EscapeSequenceMatchEntry =
            ::vtio_control_derive::__internal::vtio_control_registry::EscapeSequenceMatchEntry {
                name: #struct_name_str,
                intro: ::vtio_control_derive::__internal::vtio_control_base::EscapeSequenceIntroducer::#intro_variant,
                prefix: &[#(#prefix_bytes),*],
                final_byte: #final_byte,
                handler: #handler_name,
            };
    }
}

/// Generate an escape sequence implementation.
fn generate_escape_sequence_impl(
    input: ItemStruct,
    intro: &str,
    attrs: EscapeSequenceAttributes,
    diagnostics: &mut Vec<Diagnostic>,
    emit_struct: bool,
) -> proc_macro2::TokenStream {
    let struct_name = &input.ident;

    // Validate required attributes
    // For OSC, PM, and APC sequences, finalbyte is optional (they use data instead)
    let has_multiple_final_bytes = attrs.final_bytes.len() > 1;
    let final_byte = if matches!(intro, "OSC" | "PM" | "APC") {
        attrs.final_bytes.first().copied().unwrap_or(0) // Use 0 as placeholder, won't be used
    } else {
        if attrs.final_bytes.is_empty() {
            diagnostics.push(error_required_attr(
                struct_name.span(),
                "finalbyte",
                "'X' where X is the final byte character",
            ));
            return emit_diagnostics(diagnostics);
        }
        attrs.final_bytes[0]
    };

    if !diagnostics.is_empty() {
        return emit_diagnostics(diagnostics);
    }

    // Extract variable parameters from struct fields
    let var_params = extract_var_params_from_struct(&input);
    
    // Extract paramidx attributes for each field
    let field_paramidx: Vec<Option<usize>> = if let syn::Fields::Named(ref fields) = input.fields {
        fields.named.iter().map(|f| get_paramidx_attr(f)).collect()
    } else {
        Vec::new()
    };

    // For CSI sequences, validate that optional parameters come after required ones
    if intro == "CSI" {
        if let Some(ref params) = var_params {
            let mut seen_optional = false;
            for (name, ty) in params {
                let is_optional = is_option_type(ty);
                if is_optional {
                    seen_optional = true;
                } else if seen_optional {
                    diagnostics.push(
                        struct_name.span()
                            .error(format!("required CSI parameter '{}' must come before optional parameters", name))
                            .help("reorder fields so that all required parameters come first"),
                    );
                    return emit_diagnostics(diagnostics);
                }
            }
        }
    }

    // For DCS and OSC sequences, also extract data parameters (including const literals)
    // This must be done BEFORE removing helper attributes
    let dcs_data_params = if intro == "DCS" || intro == "OSC" {
        extract_dcs_data_params(&input)
    } else {
        None
    };

    // Extract positional parameters for OSC/DCS sequences
    let positional_params = if intro == "DCS" || intro == "OSC" {
        extract_positional_params(&input, diagnostics)
    } else {
        None
    };

    // Now remove helper attributes from the struct before emitting it
    let input = remove_helper_attributes(input.clone());

    // Check if params and var_params are both specified
    // For DCS and OSC sequences, params are allowed with fields (params go in header, fields in data)
    // For CSI sequences, params are also allowed with fields (const params followed by variable params)
    if !attrs.params.is_empty() && var_params.is_some() && intro != "DCS" && intro != "OSC" && intro != "CSI" {
        diagnostics.push(struct_name.span().error(
            "cannot specify params attribute for structs with fields"
        ).help(
            "use params for unit structs (const sequences) or add fields for variable sequences",
        ));
        return emit_diagnostics(diagnostics);
    }

    // Check if data attribute is used with variable sequences
    // For OSC/PM/APC, data attribute with fields is allowed (using dcs_data_params)
    if attrs.data.is_some() && var_params.is_some() && !matches!(intro, "OSC" | "PM" | "APC") {
        diagnostics.push(
            struct_name
                .span()
                .error("cannot specify data attribute for structs with fields")
                .help("data attribute is only valid for unit structs (const sequences)"),
        );
        return emit_diagnostics(diagnostics);
    }

    let intro_variant = syn::Ident::new(intro, struct_name.span());
    let is_const = var_params.is_none();

    if is_const {
        // Generate const sequence
        let consts = EscapeSequenceConsts::new(
            attrs.private,
            &attrs.params,
            &attrs.intermediate,
            final_byte,
        );

        let prefix_bytes = generate_prefix_bytes(attrs.private, &attrs.params);
        let has_lifetimes = !input.generics.lifetimes().collect::<Vec<_>>().is_empty();
        let registry_entry = if has_lifetimes {
            quote! {}
        } else if has_multiple_final_bytes {
            // Generate multiple registry entries, one for each final byte
            let entries: Vec<_> = attrs.final_bytes.iter().enumerate().map(|(idx, &fb)| {
                let registry_name_str = format!("__{}_REGISTRY_ENTRY_{}", struct_name.to_string().to_uppercase(), idx);
                let registry_name = syn::Ident::new(&registry_name_str, struct_name.span());
                let handler_name = syn::Ident::new(
                    &format!("{}_handler", struct_name.to_string().to_lowercase()),
                    struct_name.span(),
                );
                let intro_variant = syn::Ident::new(intro, struct_name.span());
                let struct_name_str = struct_name.to_string();

                quote! {
                    #[doc(hidden)]
                    #[cfg(feature = "parser")]
                    #[::vtio_control_derive::__internal::linkme::distributed_slice(::vtio_control_derive::__internal::vtio_control_registry::ESCAPE_SEQUENCE_REGISTRY)]
                    static #registry_name: ::vtio_control_derive::__internal::vtio_control_registry::EscapeSequenceMatchEntry =
                        ::vtio_control_derive::__internal::vtio_control_registry::EscapeSequenceMatchEntry {
                            name: #struct_name_str,
                            intro: ::vtio_control_derive::__internal::vtio_control_base::EscapeSequenceIntroducer::#intro_variant,
                            prefix: &[#(#prefix_bytes),*],
                            final_byte: #fb,
                            handler: #handler_name,
                        };
                }
            }).collect();
            quote! { #(#entries)* }
        } else {
            generate_registry_entry(
                struct_name,
                &input.generics,
                intro,
                &prefix_bytes,
                final_byte,
                None,
            )
        };

        // Generate the const string for ConstEncode
        let const_str = generate_const_str(
            intro,
            attrs.private,
            &attrs.params,
            &attrs.intermediate,
            final_byte,
            attrs.data.as_deref(),
            attrs.number.as_deref(),
        );

        let consts_impl = consts.generate_all();

        let struct_def = if emit_struct {
            quote! { #input }
        } else {
            quote! {}
        };

        let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

        quote! {
            #struct_def

            impl #impl_generics ::vtio_control_derive::__internal::vtio_control_base::EscapeSequence for #struct_name #ty_generics #where_clause {
                const INTRO: ::vtio_control_derive::__internal::vtio_control_base::EscapeSequenceIntroducer =
                    ::vtio_control_derive::__internal::vtio_control_base::EscapeSequenceIntroducer::#intro_variant;
                #consts_impl
            }

            impl #impl_generics ::vtio_control_derive::__internal::vtio_control_base::ConstEncode for #struct_name #ty_generics #where_clause {
                const STR: &'static str = #const_str;
            }

            #registry_entry
        }
    } else {
        // Generate variable sequence
        let var_params = var_params.unwrap();
        generate_variable_sequence(VariableSequenceParams {
            input: &input,
            struct_name,
            intro,
            intro_variant: &intro_variant,
            private: attrs.private,
            const_params: &attrs.params,
            var_params: &var_params,
            intermediate: &attrs.intermediate,
            final_byte,
            final_bytes: &attrs.final_bytes,
            dcs_data_params: dcs_data_params.as_deref(),
            positional_params: positional_params.as_deref(),
            osc_number: attrs.number.as_deref(),
            osc_data: attrs.data.as_deref(),
            data_sep: attrs.data_sep.as_deref(),
            param_sep: attrs.param_sep.as_deref(),
            emit_struct: false,
            field_paramidx: &field_paramidx,
        })
    }
}

/// Generate a const string for the escape sequence.
fn generate_const_str(
    intro: &str,
    private: Option<u8>,
    params: &[Vec<u8>],
    intermediate: &[u8],
    final_byte: u8,
    data: Option<&str>,
    number: Option<&str>,
) -> String {
    if intro == "OSC" {
        // OSC format: ESC ] number ; data ST
        ConstStrBuilder::new(intro)
            .osc_number(number)
            .data(data)
            .string_terminator(intro)
            .build()
    } else {
        ConstStrBuilder::new(intro)
            .private(private)
            .params(params)
            .intermediate(intermediate)
            .final_byte(final_byte)
            .data(data)
            .string_terminator(intro)
            .build()
    }
}

/// Parameters for generating a variable sequence.
/// Get introducer string for an escape sequence type.
///
/// Map escape sequence type names to their introducer byte sequences.
fn get_intro_str(intro: &str) -> &'static str {
    match intro {
        "CSI" => "\x1B[",
        "OSC" => "\x1B]",
        "SS2" => "\x1BN",
        "SS3" => "\x1BO",
        "DCS" => "\x1BP",
        "PM" => "\x1B^",
        "APC" => "\x1B_",
        "ST" => "\x1B\\",
        "DECKPAM" => "\x1B=",
        "DECKPNM" => "\x1B>",
        _ => "\x1B",
    }
}

struct VariableSequenceParams<'a> {
    input: &'a ItemStruct,
    struct_name: &'a Ident,
    intro: &'a str,
    intro_variant: &'a Ident,
    private: Option<u8>,
    const_params: &'a [Vec<u8>],
    var_params: &'a [(String, syn::Type)],
    intermediate: &'a [u8],
    final_byte: u8,
    dcs_data_params: Option<&'a [DcsDataParam]>,
    positional_params: Option<&'a [PositionalParam]>,
    osc_number: Option<&'a str>,
    osc_data: Option<&'a str>,
    data_sep: Option<&'a str>,
    param_sep: Option<&'a str>,
    emit_struct: bool,
    final_bytes: &'a [u8],
    field_paramidx: &'a [Option<usize>],
}

/// Generate a variable (non-const) sequence implementation.
fn generate_variable_sequence(params: VariableSequenceParams<'_>) -> proc_macro2::TokenStream {
    let VariableSequenceParams {
        input,
        struct_name,
        intro,
        intro_variant,
        private,
        const_params,
        var_params,
        intermediate,
        final_byte,
        dcs_data_params,
        positional_params,
        osc_number,
        osc_data,
        data_sep,
        param_sep,
        emit_struct,
        final_bytes,
        field_paramidx,
    } = params;
    
    // Split generics for impl blocks
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    // Struct is already defined by the user, we just need to generate the impls

    // Generate new constructor
    // For constructor params, only include non-unit fields
    let field_names: Vec<_> = var_params
        .iter()
        .map(|(name, _)| syn::Ident::new(name, struct_name.span()))
        .collect();

    let field_types: Vec<_> = var_params.iter().map(|(_, ty)| ty).collect();

    // For initialization, include ALL fields from the input struct
    let all_field_inits: Vec<_> = if let syn::Fields::Named(ref fields) = input.fields {
        fields
            .named
            .iter()
            .map(|field| {
                let field_name = field_ident(field);
                let ty = &field.ty;

                if is_unit_type(ty) {
                    // Unit type - initialize with ()
                    quote! { #field_name: () }
                } else {
                    // Regular field - use parameter
                    quote! { #field_name }
                }
            })
            .collect()
    } else {
        vec![]
    };

    let new_constructor = quote! {
        impl #impl_generics #struct_name #ty_generics #where_clause {
            #[allow(clippy::too_many_arguments)]
            #[inline]
            pub fn new(#(#field_names: #field_types),*) -> Self {
                Self {
                    #(#all_field_inits),*
                }
            }
        }
    };

    // Calculate encoded length (upper bound)
    let intro_len = 2; // ESC + introducer
    let private_len = if private.is_some() { 1 } else { 0 };
    let intermediate_len = filter_intermediate_bytes(intermediate).count();
    let final_len = 1;

    // Helper function to calculate max encoded length for a type
    fn type_max_encoded_len(ty: &syn::Type) -> usize {
        let ty_str = type_to_string(ty);
        match ty_str.as_str() {
            "u8" | "i8" => 3,
            "u16" | "i16" => 5,
            "u32" | "i32" => 10,
            "u64" | "i64" => 20,
            "usize" | "isize" => 20,
            "bool" => 1,
            "char" => 4,
            "String" => 100, // variable length string
            _ => 20,         // conservative default
        }
    }

    // For integers, use max digits (u8=3, u16=5, u32=10, u64=20, etc.)
    // Calculate const params length
    let const_param_len: usize = const_params
        .iter()
        .map(|p| p.len())
        .sum::<usize>()
        + if !const_params.is_empty() {
            const_params.len() - 1
        } else {
            0
        };

    let total_param_len: usize = var_params
        .iter()
        .map(|(_, ty)| type_max_encoded_len(ty))
        .sum::<usize>()
        + if var_params.len() > 1 {
            var_params.len() - 1
        } else {
            0
        };

    // Add separator between const and var params if both exist
    let separator_len = if !const_params.is_empty() && !var_params.is_empty() {
        1
    } else {
        0
    };

    let encoded_len = intro_len + private_len + const_param_len + separator_len + total_param_len + intermediate_len + final_len;

    // Generate encode implementation
    let intro_str = get_intro_str(intro);

    // Build write operations using WriteOperationBuilder
    let mut write_ops = WriteOperationBuilder::new();

    write_ops.write_str(intro_str);

    // For OSC, write number parameter first
    if intro == "OSC" {
        if let Some(num) = osc_number {
            write_ops.write_str(num);
            write_ops.write_str(";");
        }
        // If OSC has static data prefix, write it now
        if let Some(data_str) = osc_data {
            write_ops.write_str(data_str);
        }
    } else {
        write_ops.write_private(private);
    }

    // Write const params (for sequences with both params and fields)
    for (i, param) in const_params.iter().enumerate() {
        write_ops.write_separator(i);
        let param_str = String::from_utf8_lossy(param).into_owned();
        write_ops.write_str(&param_str);
    }

    // Write regular params if NOT using DCS/OSC data params
    if dcs_data_params.is_none() || !matches!(intro, "DCS" | "OSC") {
        let start_index = const_params.len();
        let mut encoded_param_count = start_index;
        
        for (i, (name, ty)) in var_params.iter().enumerate() {
            let field_name = syn::Ident::new(name, struct_name.span());
            
            // Skip fields that have paramidx attribute (they're folded into other params)
            if field_paramidx.get(i).and_then(|&x| x).is_some() {
                continue;
            }
            
            // Check if this is an optional parameter
            if is_option_type(ty) {
                // Generate conditional write for optional parameters
                let sep = if encoded_param_count > 0 { ";" } else { "" };
                let write_op = quote! {
                    if let Some(ref value) = self.#field_name {
                        __total += ::vtenc::encode::write_str_into(buf, #sep)?;
                        __total += ::vtenc::encode::WriteSeq::write_seq(value, buf)?;
                    }
                };
                write_ops.add_raw(write_op);
            } else {
                // Regular required parameter
                write_ops.write_separator(encoded_param_count);
                write_ops.write_field(&field_name);
                encoded_param_count += 1;
            }
        }
    }

    // For OSC, write number before data params
    if intro == "OSC" {
        // OSC sequences don't use intermediate or finalbyte, they use number
        // This will be handled by osc_number in variable sequence generation
    } else {
        write_ops.write_intermediate_bytes(intermediate);

        // For PM/APC, don't write finalbyte (they only have data + ST)
        if !matches!(intro, "PM" | "APC") {
            // Check if we have multiple final bytes - if so, call trait method
            if final_bytes.len() > 1 {
                write_ops.add_raw(quote! {
                    __total += ::vtenc::encode::WriteSeq::write_seq(
                        &(::vtio_control_derive::__internal::vtio_control_base::DynamicFinalByte::final_byte(self) as char),
                        buf
                    )?;
                });
            } else {
                write_ops.write_char(final_byte as char);
            }
        }
    }

    // Write positional parameters (for OSC/DCS sequences with #[vtctl(positional)])
    if let Some(positionals) = positional_params
        && matches!(intro, "DCS" | "OSC")
    {
        for (i, param) in positionals.iter().enumerate() {
            // For OSC with static data prefix, add separator before first field
            // For DCS or OSC without static data, use normal index-based separator
            let needs_separator = if intro == "OSC" && osc_data.is_some() {
                true // Always add separator since static data was written
            } else {
                i > 0 // Normal behavior: separator after first element
            };

            // Determine which separator to use
            let separator = if needs_separator && i == 0 && data_sep.is_some() {
                data_sep.unwrap() // Use custom data separator for first field after static data
            } else {
                param_sep.unwrap_or(";") // Use param separator (or default ";") for subsequent fields
            };

            match param {
                PositionalParam::Required(field_name, _ty) => {
                    if needs_separator {
                        write_ops.write_str(separator);
                    }
                    let field_ident = syn::Ident::new(field_name, struct_name.span());
                    write_ops.write_field(&field_ident);
                }
                PositionalParam::Optional(field_name, _ty) => {
                    // Generate conditional write for optional positional parameters
                    let field_ident = syn::Ident::new(field_name, struct_name.span());
                    let sep_lit = separator;
                    let write_op = quote! {
                        if let Some(ref value) = self.#field_ident {
                            __total += ::vtenc::encode::write_str_into(buf, #sep_lit)?;
                            __total += ::vtenc::encode::WriteSeq::write_seq(value, buf)?;
                        }
                    };
                    write_ops.add_raw(write_op);
                }
            }
        }
    }
    // Write DCS/OSC data params (for DCS/OSC sequences with dcs_data_params)
    else if let Some(data_params) = dcs_data_params
        && matches!(intro, "DCS" | "OSC")
    {
        for (i, param) in data_params.iter().enumerate() {
            // For OSC with static data prefix, add separator before first field
            // For DCS or OSC without static data, use normal index-based separator
            let needs_separator = if intro == "OSC" && osc_data.is_some() {
                true // Always add separator since static data was written
            } else {
                i > 0 // Normal behavior: separator after first element
            };

            // Determine which separator to use
            let separator = if needs_separator && i == 0 && data_sep.is_some() {
                data_sep.unwrap() // Use custom data separator for first field after static data
            } else {
                param_sep.unwrap_or(";") // Use param separator (or default ";") for subsequent fields
            };

            match param {
                DcsDataParam::Field(boxed) => {
                    if needs_separator {
                        write_ops.write_str(separator);
                    }
                    let (field_name, _ty) = &**boxed;
                    let field_ident = syn::Ident::new(field_name, struct_name.span());
                    write_ops.write_field(&field_ident);
                }
                DcsDataParam::OptionalField(boxed) => {
                    // Generate conditional write for optional fields
                    let (field_name, _ty) = &**boxed;
                    let field_ident = syn::Ident::new(field_name, struct_name.span());
                    let sep_lit = separator;
                    let write_op = quote! {
                        if let Some(ref value) = self.#field_ident {
                            __total += ::vtenc::encode::write_str_into(buf, #sep_lit)?;
                            __total += ::vtenc::encode::WriteSeq::write_seq(value, buf)?;
                        }
                    };
                    write_ops.add_raw(write_op);
                }
                DcsDataParam::Literal(lit) => {
                    if needs_separator {
                        write_ops.write_str(separator);
                    }
                    write_ops.write_str(lit);
                }
            }
        }
    }

    // Add string terminator for DCS/OSC/PM/APC variable sequences
    if matches!(intro, "DCS" | "OSC" | "PM" | "APC") {
        write_ops.write_str("\x1B\\");
    }

    let all_write_ops = write_ops.build();

    // Generate const params (empty for variable sequences)
    let consts = EscapeSequenceConsts::new(private, &[], intermediate, final_byte);
    let consts_impl = consts.generate_all();

    // For sequences with positional parameters or lifetime parameters,
    // don't generate registry entry since positionals are in the data section
    // and lifetimes make it impossible to construct without borrowed data
    let has_lifetimes = !input.generics.lifetimes().collect::<Vec<_>>().is_empty();
    let registry_entry = if positional_params.is_some() || has_lifetimes {
        quote! {}
    } else {
        generate_registry_entry(
            struct_name,
            &input.generics,
            intro,
            &[],
            final_byte,
            Some(var_params),
        )
    };

    let struct_def = if emit_struct {
        quote! { #input }
    } else {
        quote! {}
    };

    // Always generate constructor for variable sequences
    let constructor = quote! { #new_constructor };

    quote! {
        #struct_def

        #constructor

        impl #impl_generics ::vtio_control_derive::__internal::vtio_control_base::EscapeSequence for #struct_name #ty_generics #where_clause {
            const INTRO: ::vtio_control_derive::__internal::vtio_control_base::EscapeSequenceIntroducer =
                ::vtio_control_derive::__internal::vtio_control_base::EscapeSequenceIntroducer::#intro_variant;
            #consts_impl
        }

        impl #impl_generics ::vtio_control_derive::__internal::vtio_control_base::ConstEncodedLen for #struct_name #ty_generics #where_clause {
            const ENCODED_LEN: usize = #encoded_len;
        }

        impl #impl_generics ::vtio_control_derive::__internal::vtio_control_base::Encode for #struct_name #ty_generics #where_clause {
            #[inline]
            fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, ::vtio_control_derive::__internal::vtio_control_base::EncodeError> {
                let mut __total = 0usize;
                #(#all_write_ops)*
                Ok(__total)
            }
        }

        #registry_entry
    }
}

/// Derive macro generating an implementation of a VT control function
/// (aka control sequence).
///
/// This macro examines the attributes on a struct to determine which type
/// of control sequence to generate (CSI, DCS, OSC, ESC, etc.).
///
/// # Example
///
/// ```ignore
/// #[derive(VTControl)]
/// #[vtctl(csi, private = '?', finalbyte = 'n')]
/// pub struct UdkStatusReport {
///     pub status: u8,
/// }
/// ```
///
/// # Supported Sequence Types
///
/// All sequence types are specified using the `#[vtctl(type, ...)]` attribute:
///
/// - `csi`: Control Sequence Introducer sequences
/// - `dcs`: Device Control String sequences
/// - `osc`: Operating System Command sequences
/// - `esc`: Plain ESC sequences
/// - `ss2`: Single Shift 2 sequences
/// - `ss3`: Single Shift 3 sequences
/// - `pm`: Privacy Message sequences
/// - `apc`: Application Program Command sequences
/// - `st`: String Terminator sequences
/// - `deckpam`: DEC Keypad Application Mode sequences
/// - `deckpnm`: DEC Keypad Numeric Mode sequences
/// - `c0`: C0 control character sequences
///
/// # Field-Level Attributes
///
/// - `#[vtctl(positional)]`: Mark a field as a positional parameter for OSC/DCS
///   sequences. Optional parameters must come after required ones.
/// - `#[vtctl(literal = "value")]`: Mark a field as a literal string constant in
///   DCS data parameters.
/// - `#[vtctl(data_sep = "=")]`: Custom separator between static data and first
///   field (default: ";").
/// - `#[vtctl(param_sep = "|")]`: Custom separator between parameters/fields
///   (default: ";").
#[proc_macro_derive(VTControl, attributes(vtctl))]
pub fn derive_control(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    let mut diagnostics = Vec::new();

    // Extract struct data
    let struct_data = match input.data {
        syn::Data::Struct(ref data) => data,
        _ => {
            diagnostics.push(
                input
                    .span()
                    .error("Control can only be derived for structs"),
            );
            return TokenStream::from(emit_diagnostics(&mut diagnostics));
        }
    };

    // Convert DeriveInput to ItemStruct for compatibility with existing code
    let item_struct = ItemStruct {
        attrs: input.attrs.clone(),
        vis: input.vis.clone(),
        struct_token: syn::token::Struct {
            span: input.ident.span(),
        },
        ident: input.ident.clone(),
        generics: input.generics.clone(),
        fields: struct_data.fields.clone(),
        semi_token: match struct_data.fields {
            syn::Fields::Named(_) => None,
            _ => Some(syn::token::Semi {
                spans: [input.ident.span()],
            }),
        },
    };

    // Find control sequence attribute
    let mut control_attr: Option<(String, Vec<Meta>)> = None;

    for attr in &input.attrs {
        if let Ok(meta) = attr.meta.require_list() {
            let path_str = meta
                .path
                .get_ident()
                .map(|i| i.to_string())
                .unwrap_or_default();

            if path_str == "vtctl" {
                if control_attr.is_some() {
                    diagnostics.push(
                        attr.span()
                            .error("multiple vtctl attributes found")
                            .help("only one #[vtctl(...)] attribute is allowed"),
                    );
                    return TokenStream::from(emit_diagnostics(&mut diagnostics));
                }

                // Parse the nested meta items
                let nested: Punctuated<Meta, Token![,]> =
                    match meta.parse_args_with(Punctuated::parse_terminated) {
                        Ok(n) => n,
                        Err(e) => {
                            diagnostics.push(
                                attr.span()
                                    .error(format!("failed to parse attribute: {}", e)),
                            );
                            return TokenStream::from(emit_diagnostics(&mut diagnostics));
                        }
                    };

                let mut nested_vec: Vec<Meta> = nested.into_iter().collect();

                // First element should be the sequence type identifier
                if nested_vec.is_empty() {
                    diagnostics.push(
                        attr.span()
                            .error("vtctl attribute requires a sequence type")
                            .help("example: #[vtctl(csi, finalbyte = 'H')] or #[vtctl(osc, number = \"133\", data = \"D\")]"),
                    );
                    return TokenStream::from(emit_diagnostics(&mut diagnostics));
                }

                let first_meta = nested_vec.remove(0);
                let sequence_type = match first_meta {
                    Meta::Path(ref path) => {
                        path.get_ident()
                            .map(|i| i.to_string())
                            .unwrap_or_default()
                    }
                    _ => {
                        diagnostics.push(
                            first_meta.span()
                                .error("expected sequence type identifier")
                                .help("valid types: csi, dcs, osc, esc, ss2, ss3, pm, apc, st, deckpam, deckpnm, c0"),
                        );
                        return TokenStream::from(emit_diagnostics(&mut diagnostics));
                    }
                };

                match sequence_type.as_str() {
                    "csi" | "dcs" | "osc" | "esc" | "ss2" | "ss3" | "pm" | "apc" | "st"
                    | "deckpam" | "deckpnm" | "c0" => {
                        control_attr = Some((sequence_type, nested_vec));
                    }
                    _ => {
                        diagnostics.push(
                            first_meta.span()
                                .error(format!("unknown sequence type: {}", sequence_type))
                                .help("valid types: csi, dcs, osc, esc, ss2, ss3, pm, apc, st, deckpam, deckpnm, c0"),
                        );
                        return TokenStream::from(emit_diagnostics(&mut diagnostics));
                    }
                }
            }
        }
    }

    // Generate implementation based on attribute
    match control_attr {
        Some((ref attr_name, meta_list)) if attr_name == "csi" => {
            let attrs = parse_escape_sequence_attributes(
                meta_list.into_iter().collect(),
                &mut diagnostics,
            );
            TokenStream::from(generate_escape_sequence_impl(
                item_struct,
                "CSI",
                attrs,
                &mut diagnostics,
                false,
            ))
        }
        Some((ref attr_name, meta_list)) if attr_name == "dcs" => {
            let attrs = parse_escape_sequence_attributes(
                meta_list.into_iter().collect(),
                &mut diagnostics,
            );
            TokenStream::from(generate_escape_sequence_impl(
                item_struct,
                "DCS",
                attrs,
                &mut diagnostics,
                false,
            ))
        }
        Some((ref attr_name, meta_list)) if attr_name == "osc" => {
            let attrs = parse_escape_sequence_attributes(
                meta_list.into_iter().collect(),
                &mut diagnostics,
            );
            TokenStream::from(generate_escape_sequence_impl(
                item_struct,
                "OSC",
                attrs,
                &mut diagnostics,
                false,
            ))
        }
        Some((ref attr_name, meta_list)) if attr_name == "ss2" => {
            let attrs = parse_escape_sequence_attributes(
                meta_list.into_iter().collect(),
                &mut diagnostics,
            );
            TokenStream::from(generate_escape_sequence_impl(
                item_struct,
                "SS2",
                attrs,
                &mut diagnostics,
                false,
            ))
        }
        Some((ref attr_name, meta_list)) if attr_name == "ss3" => {
            let attrs = parse_escape_sequence_attributes(
                meta_list.into_iter().collect(),
                &mut diagnostics,
            );
            TokenStream::from(generate_escape_sequence_impl(
                item_struct,
                "SS3",
                attrs,
                &mut diagnostics,
                false,
            ))
        }
        Some((ref attr_name, meta_list)) if attr_name == "pm" => {
            let attrs = parse_escape_sequence_attributes(
                meta_list.into_iter().collect(),
                &mut diagnostics,
            );
            TokenStream::from(generate_escape_sequence_impl(
                item_struct,
                "PM",
                attrs,
                &mut diagnostics,
                false,
            ))
        }
        Some((ref attr_name, meta_list)) if attr_name == "apc" => {
            let attrs = parse_escape_sequence_attributes(
                meta_list.into_iter().collect(),
                &mut diagnostics,
            );
            TokenStream::from(generate_escape_sequence_impl(
                item_struct,
                "APC",
                attrs,
                &mut diagnostics,
                false,
            ))
        }
        Some((ref attr_name, meta_list)) if attr_name == "st" => {
            let attrs = parse_escape_sequence_attributes(
                meta_list.into_iter().collect(),
                &mut diagnostics,
            );
            TokenStream::from(generate_escape_sequence_impl(
                item_struct,
                "ST",
                attrs,
                &mut diagnostics,
                false,
            ))
        }
        Some((ref attr_name, meta_list)) if attr_name == "deckpam" => {
            let attrs = parse_escape_sequence_attributes(
                meta_list.into_iter().collect(),
                &mut diagnostics,
            );
            TokenStream::from(generate_escape_sequence_impl(
                item_struct,
                "DECKPAM",
                attrs,
                &mut diagnostics,
                false,
            ))
        }
        Some((ref attr_name, meta_list)) if attr_name == "deckpnm" => {
            let attrs = parse_escape_sequence_attributes(
                meta_list.into_iter().collect(),
                &mut diagnostics,
            );
            TokenStream::from(generate_escape_sequence_impl(
                item_struct,
                "DECKPNM",
                attrs,
                &mut diagnostics,
                false,
            ))
        }
        Some((ref attr_name, meta_list)) if attr_name == "esc" => {
            let attrs = parse_escape_sequence_attributes(
                meta_list.into_iter().collect(),
                &mut diagnostics,
            );
            TokenStream::from(generate_esc_sequence_impl(item_struct, attrs, &mut diagnostics, false))
        }
        Some((ref attr_name, meta_list)) if attr_name == "c0" => {
            let struct_name = &item_struct.ident;
            let mut code: Option<u8> = None;

            for meta in meta_list {
                match meta {
                    Meta::NameValue(MetaNameValue {
                        path,
                        value:
                            Expr::Lit(ExprLit {
                                lit: Lit::Int(lit_int),
                                ..
                            }),
                        ..
                    }) if path.is_ident("code") => {
                        code = match lit_int.base10_parse::<u8>() {
                            Ok(val) if val <= 0x1F => Some(val),
                            Ok(_) => {
                                diagnostics.push(
                                    lit_int
                                        .span()
                                        .error("C0 code must be in range 0x00-0x1F"),
                                );
                                return TokenStream::from(emit_diagnostics(&mut diagnostics));
                            }
                            Err(e) => {
                                diagnostics.push(lit_int.span().error(format!(
                                    "invalid C0 code value: {}",
                                    e
                                )));
                                return TokenStream::from(emit_diagnostics(&mut diagnostics));
                            }
                        };
                    }
                    _ => {
                        diagnostics.push(
                            meta.span()
                                .error("unsupported attribute")
                                .help("valid attributes are: code = <integer>"),
                        );
                        return TokenStream::from(emit_diagnostics(&mut diagnostics));
                    }
                }
            }

            let code = match code {
                Some(c) => c,
                None => {
                    diagnostics.push(error_required_attr(
                        input.ident.span(),
                        "code",
                        "0x0E",
                    ));
                    return TokenStream::from(emit_diagnostics(&mut diagnostics));
                }
            };

            let const_str = format!("{}", code as char);
            let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
            let expanded = quote! {
                impl #impl_generics ::vtio_control_derive::__internal::vtio_control_base::ConstEncode for #struct_name #ty_generics #where_clause {
                    const STR: &'static str = #const_str;
                }
            };

            TokenStream::from(expanded)
        }
        Some(_) => {
            unreachable!("unknown control sequence attribute");
        }
        None => {
            diagnostics.push(
                input
                    .span()
                    .error("no control sequence attribute found")
                    .help("add #[vtctl(type, ...)] where type is one of: csi, dcs, osc, esc, ss2, ss3, pm, apc, st, deckpam, deckpnm, c0\nexample: #[vtctl(csi, finalbyte = 'H')] or #[vtctl(osc, number = \"133\", data = \"D\")]"),
            );
            TokenStream::from(emit_diagnostics(&mut diagnostics))
        }
    }
}

/// Generate implementation for plain ESC sequences.
///
/// Plain ESC sequences don't have an introducer byte after ESC, they go
/// directly to intermediate bytes and final byte.
fn generate_esc_sequence_impl(
    input: ItemStruct,
    attrs: EscapeSequenceAttributes,
    diagnostics: &mut Vec<Diagnostic>,
    emit_struct: bool,
) -> proc_macro2::TokenStream {
    let struct_name = &input.ident;

    // Extract variable parameters from struct fields
    let var_params = extract_var_params_from_struct(&input);

    // Check if params and var_params are both specified
    if !attrs.params.is_empty() && var_params.is_some() {
        diagnostics.push(struct_name.span().error(
            "cannot specify params attribute for structs with fields"
        ).help(
            "use params for unit structs (const sequences) or add fields for variable sequences",
        ));
        return emit_diagnostics(diagnostics);
    }

    let is_const = var_params.is_none();

    if is_const {
        // For const sequences, finalbyte is required
        let Some(final_byte) = attrs.final_bytes.first().copied() else {
            diagnostics.push(error_required_attr(
                struct_name.span(),
                "finalbyte",
                "'X' where X is the final byte character",
            ));
            return emit_diagnostics(diagnostics);
        };

        if !diagnostics.is_empty() {
            return emit_diagnostics(diagnostics);
        }

        // Generate const sequence
        // Build the const string: ESC + intermediate + final_byte
        let mut const_str = String::from("\x1B");

        // Add intermediate bytes
        for &byte in &attrs.intermediate {
            if byte != 0 {
                const_str.push(byte as char);
            }
        }

        // Add final byte
        const_str.push(final_byte as char);

        let struct_def = if emit_struct {
            quote! { #input }
        } else {
            quote! {}
        };

        let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

        quote! {
            #struct_def

            impl #impl_generics ::vtio_control_derive::__internal::vtio_control_base::ConstEncode for #struct_name #ty_generics #where_clause {
                const STR: &'static str = #const_str;
            }
        }
    } else {
        // Generate variable sequence
        // For variable sequences, finalbyte is optional - fields provide the variable content
        if !diagnostics.is_empty() {
            return emit_diagnostics(diagnostics);
        }

        let var_params = var_params.unwrap();
        let intermediate = &attrs.intermediate;

        // Build intermediate string for write_esc macro
        let intermediate_str: String = filter_intermediate_bytes(intermediate)
            .map(|&b| b as char)
            .collect();

        // Generate field references for write_esc macro
        let field_idents: Vec<_> = var_params
            .iter()
            .map(|(field_name, _)| syn::Ident::new(field_name, struct_name.span()))
            .collect();

        let struct_def = if emit_struct {
            quote! { #input }
        } else {
            quote! {}
        };

        let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

        quote! {
            #struct_def

            impl #impl_generics ::vtio_control_derive::__internal::vtio_control_base::ConstEncodedLen for #struct_name #ty_generics #where_clause {
                const ENCODED_LEN: usize = 4; // Conservative upper bound: ESC + intermediate + 2-byte charset
            }

            impl #impl_generics ::vtio_control_derive::__internal::vtio_control_base::Encode for #struct_name #ty_generics #where_clause {
                fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, ::vtio_control_derive::__internal::vtio_control_base::EncodeError> {
                    ::vtio_control_derive::__internal::vtio_control_base::write_esc!(buf; #intermediate_str, #(self.#field_idents),*)
                }
            }
        }
    }
}
