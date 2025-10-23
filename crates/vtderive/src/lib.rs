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

    /// Add string terminator for specific sequence types.
    fn string_terminator(mut self, intro: &str) -> Self {
        if matches!(intro, "DCS" | "PM" | "APC") {
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
    /// Final byte that terminates the sequence.
    final_byte: Option<u8>,
    /// Optional data string (for DCS sequences) that appears after the
    /// final byte but before the string terminator.
    data: Option<String>,
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

    if let Expr::Array(arr) = value {
        for elem in arr.elems.iter() {
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

/// Represents a DCS data parameter (either a field or a constant literal).
#[derive(Clone)]
enum DcsDataParam {
    /// A field from the struct
    Field(Box<(String, syn::Type)>),
    /// A constant literal value
    Literal(String),
}

/// Remove helper attributes from struct fields.
///
/// Filter out attributes like `#[literal]` that are used by the macro but
/// should not appear in the generated output.
fn remove_helper_attributes(mut input: ItemStruct) -> ItemStruct {
    if let syn::Fields::Named(ref mut fields) = input.fields {
        for field in fields.named.iter_mut() {
            field.attrs.retain(|attr| !attr.path().is_ident("literal"));
        }
    }
    input
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

                    // Check for #[literal("value")] attribute
                    for attr in &field.attrs {
                        if attr.path().is_ident("literal")
                            && let Ok(list) = attr.meta.require_list()
                            && let Ok(value) = list.parse_args::<syn::LitStr>()
                        {
                            return Some(DcsDataParam::Literal(value.value()));
                        }
                    }

                    // Regular field - check if it's a unit type (these are for const markers)
                    let ty = &field.ty;
                    if is_unit_type(ty) {
                        // Unit type without #[literal] is skipped
                        return None;
                    }

                    Some(DcsDataParam::Field(Box::new((name, field.ty.clone()))))
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

/// Parse the final byte attribute.
///
/// Extract a single byte from a character literal like `finalbyte = 'h'`.
fn parse_finalbyte(value: &Expr, diagnostics: &mut Vec<Diagnostic>) -> Option<u8> {
    parse_char_as_byte(value, "finalbyte", "h", diagnostics)
}

/// Parse the data attribute.
///
/// Extract a string from a string literal like `data = " q"`.
fn parse_data(value: &Expr, diagnostics: &mut Vec<Diagnostic>) -> Option<String> {
    parse_string_literal(value, "data", " q", diagnostics)
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
        final_byte: None,
        data: None,
    };

    for meta in meta_list {
        if let Some((key, value)) = parse_name_value_attr(&meta, diagnostics) {
            match key.as_str() {
                "private" => attrs.private = parse_private(value, diagnostics),
                "params" => attrs.params = parse_params(value, diagnostics),
                "intermediate" => attrs.intermediate = parse_intermediate(value, diagnostics),
                "finalbyte" => attrs.final_byte = parse_finalbyte(value, diagnostics),
                "data" => attrs.data = parse_data(value, diagnostics),
                unknown => {
                    diagnostics.push(
                        meta.span()
                            .error(format!("unknown attribute: {}", unknown))
                            .help(
                                "valid attributes are: private, params, intermediate, finalbyte, data",
                            ),
                    );
                }
            }
        } else if !matches!(meta, Meta::NameValue(_)) {
            diagnostics.push(
                meta.span()
                    .error("expected name-value pairs in attribute")
                    .help("example: #[csi(private = '?', params = [\"6\"], finalbyte = 'h')]"),
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
                const PARAMS: ::vtparser::EscapeSequenceParams = const {
                    ::smallvec::SmallVec::new_const()
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
                            ::vtparser::EscapeSequenceParam::from_const_with_len_unchecked(
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
                    ::vtparser::EscapeSequenceParam::from_u8(0u8)
                }
            });

            quote! {
                const PARAMS: ::vtparser::EscapeSequenceParams = const {
                    // SAFETY: we compute the number above and it is always
                    //         smaller than 8.
                    unsafe {
                        ::smallvec::SmallVec::from_const_with_len_unchecked(
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
    fn generate_param_parsing(
        i: usize,
        name: &str,
        ty: &syn::Type,
        struct_span: proc_macro2::Span,
    ) -> proc_macro2::TokenStream {
        let field_name = syn::Ident::new(name, struct_span);

        quote! {
            let #field_name: #ty = params
                .get(#i)
                .map(|p| <#ty as ::core::convert::From<&::vtparser::EscapeSequenceParam>>::from(p))
                .unwrap_or_default();
        }
    }

    // Generate handler function
    let handler_body = if let Some(params) = var_params {
        // Variable sequence - parse params and call new
        let param_parsing: Vec<_> = params
            .iter()
            .enumerate()
            .map(|(i, (name, ty))| generate_param_parsing(i, name, ty, struct_name.span()))
            .collect();

        let field_names: Vec<_> = params
            .iter()
            .map(|(name, _)| syn::Ident::new(name, struct_name.span()))
            .collect();

        quote! {
            #(#param_parsing)*
            let _seq = #struct_name::new(#(#field_names),*);
        }
    } else {
        // Const sequence - just construct unit struct
        quote! {
            let _seq = #struct_name;
        }
    };

    let handler_fn = quote! {
        fn #handler_name(params: &[::vtparser::EscapeSequenceParam]) {
            #handler_body
        }
    };

    let intro_variant = syn::Ident::new(intro, struct_name.span());
    let struct_name_str = struct_name.to_string();

    quote! {
        #[doc(hidden)]
        #handler_fn

        #[doc(hidden)]
        #[::linkme::distributed_slice(::vtparser::ESCAPE_SEQUENCE_REGISTRY)]
        static #registry_name: ::vtparser::EscapeSequenceMatchEntry =
            ::vtparser::EscapeSequenceMatchEntry {
                name: #struct_name_str,
                intro: ::vtparser::EscapeSequenceIntroducer::#intro_variant,
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
) -> proc_macro2::TokenStream {
    let struct_name = &input.ident;

    // Validate required attributes
    let Some(final_byte) = attrs.final_byte else {
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

    // Extract variable parameters from struct fields
    let var_params = extract_var_params_from_struct(&input);

    // For DCS sequences, also extract data parameters (including const literals)
    // This must be done BEFORE removing helper attributes
    let dcs_data_params = if intro == "DCS" {
        extract_dcs_data_params(&input)
    } else {
        None
    };

    // Now remove helper attributes from the struct before emitting it
    let input = remove_helper_attributes(input.clone());

    // Check if params and var_params are both specified
    // For DCS sequences, params are allowed with fields (params go in header, fields in data)
    // For CSI sequences, params are also allowed with fields (const params followed by variable params)
    if !attrs.params.is_empty() && var_params.is_some() && intro != "DCS" && intro != "CSI" {
        diagnostics.push(struct_name.span().error(
            "cannot specify params attribute for structs with fields"
        ).help(
            "use params for unit structs (const sequences) or add fields for variable sequences",
        ));
        return emit_diagnostics(diagnostics);
    }

    // Check if data attribute is used with variable sequences
    if attrs.data.is_some() && var_params.is_some() {
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
        let registry_entry = generate_registry_entry(
            struct_name,
            intro,
            &prefix_bytes,
            final_byte,
            None, // const sequence
        );

        // Generate the const string for ConstEncode
        let const_str = generate_const_str(
            intro,
            attrs.private,
            &attrs.params,
            &attrs.intermediate,
            final_byte,
            attrs.data.as_deref(),
        );

        let consts_impl = consts.generate_all();

        quote! {
            #input

            impl ::vtparser::EscapeSequence for #struct_name {
                const INTRO: ::vtparser::EscapeSequenceIntroducer =
                    ::vtparser::EscapeSequenceIntroducer::#intro_variant;
                #consts_impl
            }

            impl ::vtenc::encode::ConstEncode for #struct_name {
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
            dcs_data_params: dcs_data_params.as_deref(),
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
) -> String {
    ConstStrBuilder::new(intro)
        .private(private)
        .params(params)
        .intermediate(intermediate)
        .final_byte(final_byte)
        .data(data)
        .string_terminator(intro)
        .build()
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
    } = params;
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
        impl #struct_name {
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
    write_ops.write_private(private);

    // Write const params (for sequences with both params and fields)
    for (i, param) in const_params.iter().enumerate() {
        write_ops.write_separator(i);
        let param_str = String::from_utf8_lossy(param).into_owned();
        write_ops.write_str(&param_str);
    }

    // Write regular params if NOT using DCS data params
    if dcs_data_params.is_none() || !matches!(intro, "DCS") {
        let start_index = const_params.len();
        for (i, (name, _)) in var_params.iter().enumerate() {
            write_ops.write_separator(start_index + i);
            let field_name = syn::Ident::new(name, struct_name.span());
            write_ops.write_field(&field_name);
        }
    }

    write_ops.write_intermediate_bytes(intermediate);
    write_ops.write_char(final_byte as char);

    // Write DCS data params (for DCS sequences with dcs_data_params)
    if let Some(data_params) = dcs_data_params
        && matches!(intro, "DCS")
    {
        for (i, param) in data_params.iter().enumerate() {
            write_ops.write_separator(i);
            match param {
                DcsDataParam::Field(boxed) => {
                    let (field_name, _ty) = &**boxed;
                    let field_ident = syn::Ident::new(field_name, struct_name.span());
                    write_ops.write_field(&field_ident);
                }
                DcsDataParam::Literal(lit) => {
                    write_ops.write_str(lit);
                }
            }
        }
    }

    // Add string terminator for DCS/PM/APC variable sequences
    if matches!(intro, "DCS" | "PM" | "APC") {
        write_ops.write_str("\x1B\\");
    }

    let all_write_ops = write_ops.build();

    // Generate const params (empty for variable sequences)
    let consts = EscapeSequenceConsts::new(private, &[], intermediate, final_byte);
    let consts_impl = consts.generate_all();

    let registry_entry = generate_registry_entry(
        struct_name,
        intro,
        &[],
        final_byte,
        Some(var_params), // variable sequence with params
    );

    quote! {
        #input

        #new_constructor

        impl ::vtparser::EscapeSequence for #struct_name {
            const INTRO: ::vtparser::EscapeSequenceIntroducer =
                ::vtparser::EscapeSequenceIntroducer::#intro_variant;
            #consts_impl
        }

        impl ::vtenc::encode::ConstEncodedLen for #struct_name {
            const ENCODED_LEN: usize = #encoded_len;
        }

        impl ::vtenc::encode::Encode for #struct_name {
            #[inline]
            fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, ::vtenc::encode::EncodeError> {
                let mut __total = 0usize;
                #(#all_write_ops)*
                Ok(__total)
            }
        }

        #registry_entry
    }
}

/// Helper macro to generate attribute macro functions for escape sequences.
///
/// Reduces duplication across similar escape sequence types.
macro_rules! define_escape_sequence_macro {
    (
        $(#[$meta:meta])*
        $name:ident, $intro:expr
    ) => {
        $(#[$meta])*
        #[proc_macro_attribute]
        pub fn $name(attr: TokenStream, item: TokenStream) -> TokenStream {
            let input = parse_macro_input!(item as ItemStruct);
            let meta_list =
                parse_macro_input!(attr with Punctuated::<Meta, Token![,]>::parse_terminated);

            let mut diagnostics = Vec::new();
            let attrs = parse_escape_sequence_attributes(meta_list, &mut diagnostics);

            TokenStream::from(generate_escape_sequence_impl(input, $intro, attrs, &mut diagnostics))
        }
    };
}

define_escape_sequence_macro!(
    /// Attribute macro for CSI (Control Sequence Introducer) sequences.
    ///
    /// # Example
    ///
    /// ```ignore
    /// // Const sequence
    /// #[csi(private='?', params=["6"], finalbyte='h')]
    /// struct DecSetMode;
    ///
    /// // Variable sequence - struct with fields
    /// #[csi(finalbyte='H')]
    /// struct CursorPosition {
    ///     pub row: u16,
    ///     pub col: u16,
    /// }
    ///
    /// // Mixed const and variable params
    /// #[csi(private='?', params=["27"], finalbyte='n')]
    /// struct KeyboardStatusReport {
    ///     pub dialect: u8,
    /// }
    /// ```
    ///
    /// # Attributes
    ///
    /// - `private` (optional): Character literal for private marker byte
    /// - `params` (optional): Array of string literals for const parameters
    /// - `intermediate` (optional): String literal for intermediate bytes
    /// - `finalbyte` (required): Character literal for final byte
    ///
    /// For variable sequences, define the struct with fields. The fields will be used as
    /// parameters in the encoded sequence. A `new` constructor will be generated automatically.
    ///
    /// For mixed sequences with both `params` and fields, const params are emitted first,
    /// followed by variable params from fields (e.g., `CSI ? 27 ; <dialect> n`).
    csi, "CSI"
);

define_escape_sequence_macro!(
    /// Attribute macro for OSC (Operating System Command) sequences.
    ///
    /// # Example
    ///
    /// ```ignore
    /// #[osc(params=["0"], finalbyte=';')]
    /// struct SetWindowTitle;
    /// ```
    osc, "OSC"
);

define_escape_sequence_macro!(
    /// Attribute macro for SS2 (Single Shift 2) sequences.
    ///
    /// # Example
    ///
    /// ```ignore
    /// #[ss2(finalbyte='G')]
    /// struct SingleShift2;
    /// ```
    ss2, "SS2"
);

define_escape_sequence_macro!(
    /// Attribute macro for SS3 (Single Shift 3) sequences.
    ///
    /// # Example
    ///
    /// ```ignore
    /// #[ss3(finalbyte='H')]
    /// struct SingleShift3;
    /// ```
    ss3, "SS3"
);

define_escape_sequence_macro!(
    /// Attribute macro for DCS (Device Control String) sequences.
    ///
    /// Per ECMA-48 §5.6.3, DCS sequences may contain printable data after
    /// the final byte but before the string terminator (ST).
    ///
    /// # Examples
    ///
    /// Basic DCS sequence:
    /// ```ignore
    /// #[dcs(finalbyte = 'q')]
    /// struct RequestStatus;
    /// ```
    ///
    /// DCS sequence with intermediate byte and data (DECRQSS format):
    /// ```ignore
    /// #[dcs(intermediate = "$", finalbyte = 'q', data = " q")]
    /// struct RequestCursorStyle;
    /// ```
    ///
    /// This generates: `ESC P $ q <space> q ESC \`
    dcs, "DCS"
);

define_escape_sequence_macro!(
    /// Attribute macro for PM (Privacy Message) sequences.
    ///
    /// # Example
    ///
    /// ```ignore
    /// #[pm(finalbyte='p')]
    /// struct PrivacyMessage;
    /// ```
    pm, "PM"
);

define_escape_sequence_macro!(
    /// Attribute macro for APC (Application Program Command) sequences.
    ///
    /// # Example
    ///
    /// ```ignore
    /// #[apc(finalbyte='a')]
    /// struct ApplicationCommand;
    /// ```
    apc, "APC"
);

define_escape_sequence_macro!(
    /// Attribute macro for ST (String Terminator) sequences.
    ///
    /// # Example
    ///
    /// ```ignore
    /// #[st(finalbyte='\\')]
    /// struct StringTerminator;
    /// ```
    st, "ST"
);

define_escape_sequence_macro!(
    /// Attribute macro for DECKPAM (DEC Keypad Application Mode) sequences.
    ///
    /// # Example
    ///
    /// ```ignore
    /// #[deckpam(finalbyte='=')]
    /// struct KeypadApplicationMode;
    /// ```
    deckpam, "DECKPAM"
);

define_escape_sequence_macro!(
    /// Attribute macro for DECKPNM (DEC Keypad Numeric Mode) sequences.
    ///
    /// # Example
    ///
    /// ```ignore
    /// #[deckpnm(finalbyte='>')]
    /// struct KeypadNumericMode;
    /// ```
    deckpnm, "DECKPNM"
);

/// Generate implementation for plain ESC sequences.
///
/// Plain ESC sequences don't have an introducer byte after ESC, they go
/// directly to intermediate bytes and final byte.
fn generate_esc_sequence_impl(
    input: ItemStruct,
    attrs: EscapeSequenceAttributes,
    diagnostics: &mut Vec<Diagnostic>,
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
        let Some(final_byte) = attrs.final_byte else {
            diagnostics.push(error_required_attr(
                struct_name.span(),
                "finalbyte",
                "'X' where X is the final byte character (for const sequences)",
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

        quote! {
            #input

            impl ::vtenc::encode::ConstEncode for #struct_name {
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

        quote! {
            #input

            impl ::vtenc::encode::ConstEncodedLen for #struct_name {
                const ENCODED_LEN: usize = 4; // Conservative upper bound: ESC + intermediate + 2-byte charset
            }

            impl ::vtenc::encode::Encode for #struct_name {
                fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, ::vtenc::encode::EncodeError> {
                    ::vtenc::write_esc!(buf; #intermediate_str, #(self.#field_idents),*)
                }
            }
        }
    }
}

/// Attribute macro for plain ESC sequences.
///
/// Generate implementations for escape sequences that start with ESC (\x1B)
/// followed by optional intermediate bytes and a final byte.
///
/// # Example
///
/// ```ignore
/// #[esc(finalbyte = 'G', intermediate = "%")]
/// struct EnableUTF8Mode;
/// ```
///
/// # Attributes
///
/// - `finalbyte` (required): Character literal for the final byte
/// - `intermediate` (optional): String literal for intermediate bytes
#[proc_macro_attribute]
pub fn esc(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let meta_list = parse_macro_input!(attr with Punctuated::<Meta, Token![,]>::parse_terminated);

    let mut diagnostics = Vec::new();
    let attrs = parse_escape_sequence_attributes(meta_list, &mut diagnostics);

    TokenStream::from(generate_esc_sequence_impl(input, attrs, &mut diagnostics))
}

/// Attribute macro for C0 control characters.
///
/// Generate ConstEncode implementation for single-byte C0 control codes.
///
/// # Example
///
/// ```ignore
/// #[c0(code = 0x0E)]
/// struct ShiftOut;
/// ```
///
/// # Attributes
///
/// - `code` (required): Integer literal (0x00-0x1F) for the control code byte
#[proc_macro_attribute]
pub fn c0(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let meta_list = parse_macro_input!(attr with Punctuated::<Meta, Token![,]>::parse_terminated);

    let mut diagnostics = Vec::new();
    let struct_name = &input.ident;

    // Parse the code attribute
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
            }) if path.is_ident("code") => match lit_int.base10_parse::<u8>() {
                Ok(val) if val <= 0x1F => {
                    code = Some(val);
                }
                Ok(_) => {
                    diagnostics.push(
                        lit_int
                            .span()
                            .error("C0 control code must be in range 0x00-0x1F"),
                    );
                }
                Err(e) => {
                    diagnostics.push(lit_int.span().error(format!("invalid integer: {}", e)));
                }
            },
            _ => {
                diagnostics.push(
                    meta.span()
                        .error("unsupported attribute")
                        .help("only 'code' attribute is supported"),
                );
            }
        }
    }

    let Some(code) = code else {
        diagnostics.push(error_required_attr(
            struct_name.span(),
            "code",
            "0x.. where 0x.. is the control code byte (0x00-0x1F)",
        ));
        return TokenStream::from(emit_diagnostics(&mut diagnostics));
    };

    if !diagnostics.is_empty() {
        return TokenStream::from(emit_diagnostics(&mut diagnostics));
    }

    // Generate the const string
    let const_str = format!("{}", code as char);

    let expanded = quote! {
        #input

        impl ::vtenc::encode::ConstEncode for #struct_name {
            const STR: &'static str = #const_str;
        }
    };

    TokenStream::from(expanded)
}

/// Attribute macro for terminal mode control structures.
///
/// Generates three structs for controlling terminal modes:
/// - `Enable{Name}` - Sets the mode (CSI code h)
/// - `Disable{Name}` - Resets the mode (CSI code l)
/// - `Request{Name}` - Requests the mode state (CSI code $p)
///
/// The base struct should have an `enabled: bool` field and will be used
/// for mode state responses (CSI code;value$y).
///
/// # Example
///
/// ```ignore
/// #[terminal_mode(private = '?', params = "6")]
/// pub struct RelativeCursorOriginMode {
///     pub enabled: bool,
/// }
/// ```
///
/// Expands to four structs with appropriate CSI encodings.
///
/// # Attributes
///
/// - `private` (optional): Character literal for private marker byte (e.g., '?')
/// - `params` (required): String literal for the mode parameters (e.g., "6", "1037")
#[proc_macro_attribute]
pub fn terminal_mode(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let meta_list = parse_macro_input!(attr with Punctuated::<Meta, Token![,]>::parse_terminated);

    let mut diagnostics = Vec::new();

    // Parse attributes
    let mut private_char: Option<char> = None;
    let mut params_str: Option<String> = None;

    for meta in meta_list {
        if let Some((key, value)) = parse_name_value_attr(&meta, &mut diagnostics) {
            match key.as_str() {
                "private" => {
                    if let Some(byte) = parse_char_as_byte(value, "private", "?", &mut diagnostics)
                    {
                        private_char = Some(byte as char);
                    }
                }
                "params" => {
                    params_str = parse_string_literal(value, "params", "6", &mut diagnostics);
                }
                _ => {
                    diagnostics.push(
                        meta.span()
                            .error("unknown attribute")
                            .help("supported attributes: private, params"),
                    );
                }
            }
        } else if !matches!(meta, Meta::NameValue(_)) {
            diagnostics.push(
                meta.span()
                    .error("expected name-value attribute")
                    .help("example: private = '?', params = \"6\""),
            );
        }
    }

    // Validate required attributes
    let params = match params_str {
        Some(p) => p,
        None => {
            diagnostics.push(error_required_attr(
                input.span(),
                "params",
                "\"6\" (or other mode parameter)",
            ));
            String::new()
        }
    };

    if !diagnostics.is_empty() {
        return TokenStream::from(emit_diagnostics(&mut diagnostics));
    }

    // Extract struct metadata
    let base_name = &input.ident;
    let vis = &input.vis;
    let attrs = &input.attrs;

    // Build param array for CSI attributes
    let params_array = quote! { [#params] };

    // Build private attribute
    let private_attr = if let Some(ch) = private_char {
        quote! { private = #ch, }
    } else {
        quote! {}
    };

    // Generate the three control structs
    let enable_name = Ident::new(&format!("Enable{}", base_name), base_name.span());
    let disable_name = Ident::new(&format!("Disable{}", base_name), base_name.span());
    let request_name = Ident::new(&format!("Request{}", base_name), base_name.span());

    let expanded = quote! {
        #(#attrs)*
        #[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
        #[::vtderive::csi(#private_attr params = #params_array, finalbyte = 'h')]
        #vis struct #enable_name;

        #(#attrs)*
        #[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
        #[::vtderive::csi(#private_attr params = #params_array, finalbyte = 'l')]
        #vis struct #disable_name;

        #(#attrs)*
        #[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
        #[::vtderive::csi(#private_attr params = #params_array, intermediate = "$", finalbyte = 'p')]
        #vis struct #request_name;

        #(#attrs)*
        #[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
        #[::vtderive::csi(#private_attr intermediate = "$", finalbyte = 'y')]
        #vis struct #base_name {
            pub enabled: bool,
        }
    };

    TokenStream::from(expanded)
}
