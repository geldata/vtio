//! Procedural macros for deriving VT escape sequence traits.
//!
//! This crate provides derive macros for automatically implementing traits
//! used to represent terminal control sequences and register them with the
//! global escape sequence registry.

use proc_macro::TokenStream;
use proc_macro2_diagnostics::{Diagnostic, SpanDiagnosticExt};
use quote::quote;
use syn::{
    parse_macro_input, punctuated::Punctuated, spanned::Spanned, Expr, ExprLit, Ident,
    ItemStruct, Lit, Meta, MetaNameValue, Token,
};

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
    /// Optional custom handler function name.
    handler: Option<String>,
}

/// Parse the private marker attribute.
///
/// Extract a single byte from a character literal like `private = '?'`.
fn parse_private(value: &Expr, diagnostics: &mut Vec<Diagnostic>) -> Option<u8> {
    if let Expr::Lit(ExprLit {
        lit: Lit::Char(ch),
        ..
    }) = value
    {
        Some(ch.value() as u8)
    } else {
        diagnostics.push(
            value
                .span()
                .error("private must be a char literal")
                .help("example: private = '?'"),
        );
        None
    }
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
fn extract_var_params_from_struct(input: &ItemStruct) -> Option<Vec<(String, syn::Type)>> {
    match &input.fields {
        syn::Fields::Named(fields) => {
            let params: Vec<_> = fields
                .named
                .iter()
                .map(|field| {
                    let name = field.ident.as_ref().unwrap().to_string();
                    let ty = field.ty.clone();
                    (name, ty)
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
    if let Expr::Lit(ExprLit {
        lit: Lit::Str(s), ..
    }) = value
    {
        let intermediate: Vec<u8> = s.value().chars().map(|c| c as u8).collect();
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
        diagnostics.push(
            value
                .span()
                .error("intermediate must be a string literal")
                .help("example: intermediate = \" \""),
        );
        Vec::new()
    }
}

/// Parse the final byte attribute.
///
/// Extract a single byte from a character literal like `finalbyte = 'h'`.
fn parse_finalbyte(value: &Expr, diagnostics: &mut Vec<Diagnostic>) -> Option<u8> {
    if let Expr::Lit(ExprLit {
        lit: Lit::Char(ch),
        ..
    }) = value
    {
        Some(ch.value() as u8)
    } else {
        diagnostics.push(
            value
                .span()
                .error("finalbyte must be a char literal")
                .help("example: finalbyte = 'h'"),
        );
        None
    }
}

/// Parse the handler function name attribute.
///
/// Extract a function identifier from `handler = function_name`.
fn parse_handler(value: &Expr, diagnostics: &mut Vec<Diagnostic>) -> Option<String> {
    if let Expr::Path(path) = value
        && let Some(ident) = path.path.get_ident()
    {
        return Some(ident.to_string());
    }

    diagnostics.push(
        value
            .span()
            .error("handler must be a function identifier")
            .help("example: handler = handle_sequence"),
    );
    None
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
        handler: None,
    };

    for meta in meta_list {
        match meta {
            Meta::NameValue(MetaNameValue { path, value, .. }) => {
                let Some(key_ident) = path.get_ident() else {
                    diagnostics.push(
                        path.span()
                            .error("expected identifier")
                            .help("attribute key must be a simple identifier"),
                    );
                    continue;
                };

                match key_ident.to_string().as_str() {
                    "private" => attrs.private = parse_private(&value, diagnostics),
                    "params" => attrs.params = parse_params(&value, diagnostics),
                    "intermediate" => attrs.intermediate = parse_intermediate(&value, diagnostics),
                    "finalbyte" => attrs.final_byte = parse_finalbyte(&value, diagnostics),
                    "handler" => attrs.handler = parse_handler(&value, diagnostics),
                    unknown => {
                        diagnostics.push(
                            key_ident
                                .span()
                                .error(format!("unknown attribute: {}", unknown))
                                .help(
                                    "valid attributes are: private, params, intermediate, finalbyte, handler",
                                ),
                        );
                    }
                }
            }
            _ => {
                diagnostics.push(
                    meta.span()
                        .error("expected name-value pairs in attribute")
                        .help("example: #[csi(private = '?', params = [\"6\"], finalbyte = 'h')]"),
                );
            }
        }
    }

    attrs
}

/// Generate the PRIVATE constant.
///
/// Produce `const PRIVATE: Option<u8> = Some(byte)` or `None` depending on
/// whether a private marker was specified.
fn generate_private_const(private: Option<u8>) -> proc_macro2::TokenStream {
    if let Some(byte) = private {
        quote! { const PRIVATE: Option<u8> = Some(#byte); }
    } else {
        quote! { const PRIVATE: Option<u8> = None; }
    }
}

/// Generate the PARAMS constant.
///
/// Produce a `const PARAMS: EscapeSequenceParams` declaration using
/// `SmallVec::from_const` with proper padding. Each param is padded to 32
/// bytes, and the params array is padded to 8 elements.
fn generate_params_const(params: &[Vec<u8>]) -> proc_macro2::TokenStream {
    if params.is_empty() {
        quote! {
            const PARAMS: ::vtparser::EscapeSequenceParams = const {
                ::smallvec::SmallVec::new_const()
            };
        }
    } else {
        let param_inits: Vec<_> = params
            .iter()
            .map(|param| {
                let mut padded = param.clone();
                padded.resize(32, 0);
                let bytes = padded.iter();
                quote! {
                    ::smallvec::SmallVec::from_const([#(#bytes),*])
                }
            })
            .collect();

        let num_params = params.len();
        let padding_params = (0..(8 - num_params)).map(|_| {
            quote! { ::smallvec::SmallVec::new_const() }
        });

        quote! {
            const PARAMS: ::vtparser::EscapeSequenceParams = const {
                ::smallvec::SmallVec::from_const([
                    #(#param_inits,)*
                    #(#padding_params),*
                ])
            };
        }
    }
}

/// Generate the INTERMEDIATE constant.
///
/// Produce `const INTERMEDIATE: [u8; 2] = [byte0, byte1]` with zero
/// padding as needed to fill the 2-element array.
fn generate_intermediate_const(intermediate: &[u8]) -> proc_macro2::TokenStream {
    let value = match intermediate.len() {
        0 => quote! { [0, 0] },
        1 => {
            let byte = intermediate[0];
            quote! { [#byte, 0] }
        }
        _ => {
            let byte0 = intermediate[0];
            let byte1 = intermediate[1];
            quote! { [#byte0, #byte1] }
        }
    };

    quote! { const INTERMEDIATE: [u8; 2] = #value; }
}

/// Generate the FINAL constant.
///
/// Produce `const FINAL: u8 = byte` with the final byte value.
fn generate_final_const(final_byte: u8) -> proc_macro2::TokenStream {
    quote! { const FINAL: u8 = #final_byte; }
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

/// Generate the registry entry for the escape sequence.
fn generate_registry_entry(
    struct_name: &Ident,
    intro: &str,
    prefix_bytes: &[u8],
    final_byte: u8,
    handler: Option<&str>,
) -> proc_macro2::TokenStream {
    let registry_name = syn::Ident::new(
        &format!("__{}_REGISTRY_ENTRY", struct_name.to_string().to_uppercase()),
        struct_name.span(),
    );

    let handler_fn = if let Some(h) = handler {
        let handler_ident = syn::Ident::new(h, struct_name.span());
        quote! { #handler_ident }
    } else {
        let default_handler = syn::Ident::new(
            &format!("{}_handler", struct_name.to_string().to_lowercase()),
            struct_name.span(),
        );
        quote! { #default_handler }
    };

    let intro_variant = syn::Ident::new(intro, struct_name.span());
    let struct_name_str = struct_name.to_string();

    quote! {
        #[::linkme::distributed_slice(::vtparser::ESCAPE_SEQUENCE_REGISTRY)]
        static #registry_name: ::vtparser::EscapeSequenceMatchEntry =
            ::vtparser::EscapeSequenceMatchEntry {
                name: #struct_name_str,
                intro: ::vtparser::EscapeSequenceIntroducer::#intro_variant,
                prefix: &[#(#prefix_bytes),*],
                final_byte: #final_byte,
                handler: #handler_fn,
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
        diagnostics.push(
            struct_name
                .span()
                .error("finalbyte attribute is required")
                .help("add finalbyte = 'X' where X is the final byte character"),
        );
        let tokens: proc_macro2::TokenStream = diagnostics
            .drain(..)
            .map(|d| d.emit_as_item_tokens())
            .collect();
        return tokens;
    };

    if !diagnostics.is_empty() {
        let tokens: proc_macro2::TokenStream = diagnostics
            .drain(..)
            .map(|d| d.emit_as_item_tokens())
            .collect();
        return tokens;
    }

    // Extract variable parameters from struct fields
    let var_params = extract_var_params_from_struct(&input);
    
    // Check if params and var_params are both specified
    if !attrs.params.is_empty() && var_params.is_some() {
        diagnostics.push(
            struct_name
                .span()
                .error("cannot specify params attribute for structs with fields")
                .help("use params for unit structs (const sequences) or add fields for variable sequences"),
        );
        let tokens: proc_macro2::TokenStream = diagnostics
            .drain(..)
            .map(|d| d.emit_as_item_tokens())
            .collect();
        return tokens;
    }

    let intro_variant = syn::Ident::new(intro, struct_name.span());
    let is_const = var_params.is_none();

    if is_const {
        // Generate const sequence
        let private_const = generate_private_const(attrs.private);
        let params_const = generate_params_const(&attrs.params);
        let intermediate_const = generate_intermediate_const(&attrs.intermediate);
        let final_const = generate_final_const(final_byte);

        let prefix_bytes = generate_prefix_bytes(attrs.private, &attrs.params);
        let registry_entry = generate_registry_entry(
            struct_name,
            intro,
            &prefix_bytes,
            final_byte,
            attrs.handler.as_deref(),
        );

        // Generate the const string for ConstEncode
        let const_str = generate_const_str(intro, attrs.private, &attrs.params, &attrs.intermediate, final_byte);

        quote! {
            #input

            impl ::vtparser::EscapeSequence for #struct_name {
                const INTRO: ::vtparser::EscapeSequenceIntroducer =
                    ::vtparser::EscapeSequenceIntroducer::#intro_variant;
                #private_const
                #params_const
                #intermediate_const
                #final_const
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
            var_params: &var_params,
            intermediate: &attrs.intermediate,
            final_byte,
            handler: attrs.handler.as_deref(),
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
) -> String {
    let mut result = String::from("\x1B");
    
    // Add introducer
    match intro {
        "CSI" => result.push('['),
        "OSC" => result.push(']'),
        "SS2" => result.push('N'),
        "SS3" => result.push('O'),
        "DCS" => result.push('P'),
        "PM" => result.push('^'),
        "APC" => result.push('_'),
        "ST" => result.push('\\'),
        "DECKPAM" => result.push('='),
        "DECKPNM" => result.push('>'),
        _ => {}
    }
    
    // Add private marker
    if let Some(byte) = private {
        result.push(byte as char);
    }
    
    // Add params
    for (i, param) in params.iter().enumerate() {
        if i > 0 {
            result.push(';');
        }
        result.push_str(&String::from_utf8_lossy(param));
    }
    
    // Add intermediate bytes
    for &byte in intermediate {
        if byte != 0 {
            result.push(byte as char);
        }
    }
    
    // Add final byte
    result.push(final_byte as char);
    
    result
}

/// Parameters for generating a variable sequence.
struct VariableSequenceParams<'a> {
    input: &'a ItemStruct,
    struct_name: &'a Ident,
    intro: &'a str,
    intro_variant: &'a Ident,
    private: Option<u8>,
    var_params: &'a [(String, syn::Type)],
    intermediate: &'a [u8],
    final_byte: u8,
    handler: Option<&'a str>,
}

/// Generate a variable (non-const) sequence implementation.
fn generate_variable_sequence(params: VariableSequenceParams<'_>) -> proc_macro2::TokenStream {
    let VariableSequenceParams {
        input,
        struct_name,
        intro,
        intro_variant,
        private,
        var_params,
        intermediate,
        final_byte,
        handler,
    } = params;
    // Struct is already defined by the user, we just need to generate the impls

    // Calculate encoded length (upper bound)
    let intro_len = 2; // ESC + introducer
    let private_len = if private.is_some() { 1 } else { 0 };
    let intermediate_len = intermediate.iter().filter(|&&b| b != 0).count();
    let final_len = 1;
    
    // For integers, use max digits (u8=3, u16=5, u32=10, u64=20, etc.)
    let total_param_len: usize = var_params
        .iter()
        .map(|(_, ty)| {
            // Parse type to determine max length
            let ty_str = quote!(#ty).to_string();
            match ty_str.trim() {
                "u8" | "i8" => 3,
                "u16" | "i16" => 5,
                "u32" | "i32" => 10,
                "u64" | "i64" => 20,
                "usize" | "isize" => 20,
                "bool" => 1,
                "char" => 4,
                _ => 20, // conservative default
            }
        })
        .sum::<usize>() + if var_params.len() > 1 { var_params.len() - 1 } else { 0 };
    
    let encoded_len = intro_len + private_len + total_param_len + intermediate_len + final_len;

    // Generate encode implementation
    let intro_str = match intro {
        "CSI" => "\\x1B[",
        "OSC" => "\\x1B]",
        "SS2" => "\\x1BN",
        "SS3" => "\\x1BO",
        "DCS" => "\\x1BP",
        "PM" => "\\x1B^",
        "APC" => "\\x1B_",
        "ST" => "\\x1B\\\\",
        "DECKPAM" => "\\x1B=",
        "DECKPNM" => "\\x1B>",
        _ => "\\x1B",
    };

    let write_intro = quote! {
        __total += ::vtenc::encode::write_str_into(buf, #intro_str)?;
    };

    let write_private = if let Some(byte) = private {
        let ch = byte as char;
        quote! {
            __total += ::vtenc::encode::WriteSeq::write_seq(&(#ch), buf)?;
        }
    } else {
        quote! {}
    };

    let write_params: Vec<_> = var_params
        .iter()
        .enumerate()
        .map(|(i, (name, _))| {
            let field_name = syn::Ident::new(name, struct_name.span());
            let separator = if i > 0 {
                quote! { __total += ::vtenc::encode::write_str_into(buf, ";")?; }
            } else {
                quote! {}
            };
            quote! {
                #separator
                __total += ::vtenc::encode::WriteSeq::write_seq(&self.#field_name, buf)?;
            }
        })
        .collect();

    let write_intermediate: Vec<_> = intermediate
        .iter()
        .filter(|&&b| b != 0)
        .map(|&byte| {
            let ch = byte as char;
            quote! {
                __total += ::vtenc::encode::WriteSeq::write_seq(&(#ch), buf)?;
            }
        })
        .collect();

    let write_final = {
        let ch = final_byte as char;
        quote! {
            __total += ::vtenc::encode::WriteSeq::write_seq(&(#ch), buf)?;
        }
    };

    // Generate const params (empty for variable sequences)
    let private_const = generate_private_const(private);
    let params_const = generate_params_const(&[]);
    let intermediate_const = generate_intermediate_const(intermediate);
    let final_const = generate_final_const(final_byte);

    // Generate registry entry with empty prefix
    let registry_entry = generate_registry_entry(
        struct_name,
        intro,
        &[],
        final_byte,
        handler,
    );

    quote! {
        #input

        impl ::vtparser::EscapeSequence for #struct_name {
            const INTRO: ::vtparser::EscapeSequenceIntroducer =
                ::vtparser::EscapeSequenceIntroducer::#intro_variant;
            #private_const
            #params_const
            #intermediate_const
            #final_const
        }

        impl ::vtenc::encode::ConstEncodedLen for #struct_name {
            const ENCODED_LEN: usize = #encoded_len;
        }

        impl ::vtenc::encode::Encode for #struct_name {
            #[inline]
            fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, ::vtenc::encode::EncodeError> {
                let mut __total = 0usize;
                #write_intro
                #write_private
                #(#write_params)*
                #(#write_intermediate)*
                #write_final
                Ok(__total)
            }
        }

        #registry_entry
    }
}

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
/// ```
///
/// # Attributes
///
/// - `private` (optional): Character literal for private marker byte
/// - `params` (optional): Array of string literals for const parameters (only for unit structs)
/// - `intermediate` (optional): String literal for intermediate bytes
/// - `finalbyte` (required): Character literal for final byte
/// - `handler` (optional): Function identifier for custom handler
///
/// For variable sequences, define the struct with fields. The fields will be used as
/// parameters in the encoded sequence.
#[proc_macro_attribute]
pub fn csi(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let meta_list =
        parse_macro_input!(attr with Punctuated::<Meta, Token![,]>::parse_terminated);

    let mut diagnostics = Vec::new();
    let attrs = parse_escape_sequence_attributes(meta_list, &mut diagnostics);

    TokenStream::from(generate_escape_sequence_impl(input, "CSI", attrs, &mut diagnostics))
}

/// Attribute macro for OSC (Operating System Command) sequences.
///
/// # Example
///
/// ```ignore
/// #[osc(params=["0"], finalbyte=';')]
/// struct SetWindowTitle;
/// ```
#[proc_macro_attribute]
pub fn osc(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let meta_list =
        parse_macro_input!(attr with Punctuated::<Meta, Token![,]>::parse_terminated);

    let mut diagnostics = Vec::new();
    let attrs = parse_escape_sequence_attributes(meta_list, &mut diagnostics);

    TokenStream::from(generate_escape_sequence_impl(input, "OSC", attrs, &mut diagnostics))
}

/// Attribute macro for SS2 (Single Shift 2) sequences.
///
/// # Example
///
/// ```ignore
/// #[ss2(finalbyte='G')]
/// struct SingleShift2;
/// ```
#[proc_macro_attribute]
pub fn ss2(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let meta_list =
        parse_macro_input!(attr with Punctuated::<Meta, Token![,]>::parse_terminated);

    let mut diagnostics = Vec::new();
    let attrs = parse_escape_sequence_attributes(meta_list, &mut diagnostics);

    TokenStream::from(generate_escape_sequence_impl(input, "SS2", attrs, &mut diagnostics))
}

/// Attribute macro for SS3 (Single Shift 3) sequences.
///
/// # Example
///
/// ```ignore
/// #[ss3(finalbyte='H')]
/// struct SingleShift3;
/// ```
#[proc_macro_attribute]
pub fn ss3(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let meta_list =
        parse_macro_input!(attr with Punctuated::<Meta, Token![,]>::parse_terminated);

    let mut diagnostics = Vec::new();
    let attrs = parse_escape_sequence_attributes(meta_list, &mut diagnostics);

    TokenStream::from(generate_escape_sequence_impl(input, "SS3", attrs, &mut diagnostics))
}

/// Attribute macro for DCS (Device Control String) sequences.
///
/// # Example
///
/// ```ignore
/// #[dcs(finalbyte = 'q')]
/// struct RequestStatus;
/// ```
#[proc_macro_attribute]
pub fn dcs(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let meta_list =
        parse_macro_input!(attr with Punctuated::<Meta, Token![,]>::parse_terminated);

    let mut diagnostics = Vec::new();
    let attrs = parse_escape_sequence_attributes(meta_list, &mut diagnostics);

    TokenStream::from(generate_escape_sequence_impl(input, "DCS", attrs, &mut diagnostics))
}

/// Attribute macro for PM (Privacy Message) sequences.
///
/// # Example
///
/// ```ignore
/// #[pm(finalbyte='p')]
/// struct PrivacyMessage;
/// ```
#[proc_macro_attribute]
pub fn pm(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let meta_list =
        parse_macro_input!(attr with Punctuated::<Meta, Token![,]>::parse_terminated);

    let mut diagnostics = Vec::new();
    let attrs = parse_escape_sequence_attributes(meta_list, &mut diagnostics);

    TokenStream::from(generate_escape_sequence_impl(input, "PM", attrs, &mut diagnostics))
}

/// Attribute macro for APC (Application Program Command) sequences.
///
/// # Example
///
/// ```ignore
/// #[apc(finalbyte='a')]
/// struct ApplicationCommand;
/// ```
#[proc_macro_attribute]
pub fn apc(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let meta_list =
        parse_macro_input!(attr with Punctuated::<Meta, Token![,]>::parse_terminated);

    let mut diagnostics = Vec::new();
    let attrs = parse_escape_sequence_attributes(meta_list, &mut diagnostics);

    TokenStream::from(generate_escape_sequence_impl(input, "APC", attrs, &mut diagnostics))
}

/// Attribute macro for ST (String Terminator) sequences.
///
/// # Example
///
/// ```ignore
/// #[st(finalbyte='\\')]
/// struct StringTerminator;
/// ```
#[proc_macro_attribute]
pub fn st(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let meta_list =
        parse_macro_input!(attr with Punctuated::<Meta, Token![,]>::parse_terminated);

    let mut diagnostics = Vec::new();
    let attrs = parse_escape_sequence_attributes(meta_list, &mut diagnostics);

    TokenStream::from(generate_escape_sequence_impl(input, "ST", attrs, &mut diagnostics))
}

/// Attribute macro for DECKPAM (DEC Keypad Application Mode) sequences.
///
/// # Example
///
/// ```ignore
/// #[deckpam(finalbyte='=')]
/// struct KeypadApplicationMode;
/// ```
#[proc_macro_attribute]
pub fn deckpam(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let meta_list =
        parse_macro_input!(attr with Punctuated::<Meta, Token![,]>::parse_terminated);

    let mut diagnostics = Vec::new();
    let attrs = parse_escape_sequence_attributes(meta_list, &mut diagnostics);

    TokenStream::from(generate_escape_sequence_impl(input, "DECKPAM", attrs, &mut diagnostics))
}

/// Attribute macro for DECKPNM (DEC Keypad Numeric Mode) sequences.
///
/// # Example
///
/// ```ignore
/// #[deckpnm(finalbyte='>')]
/// struct KeypadNumericMode;
/// ```
#[proc_macro_attribute]
pub fn deckpnm(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let meta_list =
        parse_macro_input!(attr with Punctuated::<Meta, Token![,]>::parse_terminated);

    let mut diagnostics = Vec::new();
    let attrs = parse_escape_sequence_attributes(meta_list, &mut diagnostics);

    TokenStream::from(generate_escape_sequence_impl(input, "DECKPNM", attrs, &mut diagnostics))
}
