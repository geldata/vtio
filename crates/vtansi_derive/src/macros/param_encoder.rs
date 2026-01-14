//! Shared parameter encoding logic.
//!
//! This module provides reusable parameter encoding code generation that can
//! be used by both framed (control sequences) and unframed (plain struct)
//! encoding implementations.
//!
//! The key insight is that parameter encoding logic should be identical
//! whether the parameters appear in a control sequence (with intro/final
//! bytes) or as standalone struct values. This module extracts the common
//! encoding logic.

use proc_macro2::TokenStream;
use quote::quote;

use crate::helpers::FieldInfo;
use crate::helpers::{
    metadata::StructFormat, struct_params::Params, type_props::ParamEncoding,
};

/// Generate parameter encoding expressions for a list of fields.
pub fn generate_param_encoding(
    params: &Params,
    encoding: &ParamEncoding,
    sink: &syn::Ident,
    counter: &syn::Ident,
) -> syn::Result<TokenStream> {
    match encoding.format {
        StructFormat::Map => {
            generate_map_encoding(params, encoding, sink, counter)
        }
        StructFormat::Vector => {
            generate_vector_encoding(params, encoding, sink, counter)
        }
    }
}

pub fn generate_map_encoding(
    params: &Params,
    encoding: &ParamEncoding,
    sink: &syn::Ident,
    counter: &syn::Ident,
) -> syn::Result<TokenStream> {
    let fields = &params.fields;
    let offset = encoding.offset;
    let delimiter =
        &syn::Ident::new("__vtansi_delimiter", proc_macro2::Span::mixed_site());
    let skipcount = &syn::Ident::new(
        "__vtansi_skip_count",
        proc_macro2::Span::mixed_site(),
    );
    let mut lines: Vec<TokenStream> = Vec::new();

    let mut encoding_ctx = FieldEncodingContext {
        sink,
        counter,
        skipcount,
        delimiter,
        offset,
        key: None,
        index: 0usize,
    };

    let delimiter_lit = match &encoding.delimiter {
        Some(d) => {
            let d = **d;
            quote! { ::std::slice::from_ref(&#d) }
        }
        None => quote! { &[] },
    };

    let setup = quote! {
        let mut #skipcount = 0usize;
        const #delimiter: &[u8] = #delimiter_lit;
    };

    if !params.has_mux {
        for (i, field) in fields.iter().enumerate() {
            let key = field.ident().to_string();
            encoding_ctx.key = Some(key);
            encoding_ctx.index = i;
            lines.push(generate_simple_field_encoding(field, &encoding_ctx));
        }
    } else {
        let mut param_groups: Vec<Vec<&FieldInfo>> = Vec::new();
        for field in fields {
            if let Some(mux_index) = &field.mux_index {
                let mux_index = match mux_index {
                    syn::Member::Unnamed(idx) => idx.index as usize,
                    _ => panic!(
                        "encoder expected syn::Member::Unnamed, got syn::Member::Named"
                    ),
                };
                param_groups[mux_index].push(field);
            } else {
                param_groups.push(vec![field]);
            }
        }

        // Generate encoding for each muxed parameter
        for (i, fields) in param_groups.iter().enumerate() {
            let key = fields[0].ident().to_string();
            encoding_ctx.key = Some(key);
            encoding_ctx.index = i;
            if fields.len() == 1 {
                lines.push(generate_simple_field_encoding(
                    fields[0],
                    &encoding_ctx,
                ));
            } else {
                // Multiplexing: combine primary field with mux fields using
                // AnsiMuxEncode
                lines.push(generate_multiplexed_field_encoding(
                    fields,
                    &encoding_ctx,
                ));
            }
        }
    }

    Ok(quote! {
        {
            #setup
            #(#lines)*
        }
    })
}

pub fn generate_vector_encoding(
    params: &Params,
    encoding: &ParamEncoding,
    sink: &syn::Ident,
    counter: &syn::Ident,
) -> syn::Result<TokenStream> {
    let fields = &params.fields;
    let offset = encoding.offset;
    let delimiter =
        &syn::Ident::new("__vtansi_delimiter", proc_macro2::Span::mixed_site());
    let skipcount = &syn::Ident::new(
        "__vtansi_skip_count",
        proc_macro2::Span::mixed_site(),
    );
    let mut lines: Vec<TokenStream> = Vec::new();

    let mut encoding_ctx = FieldEncodingContext {
        sink,
        counter,
        skipcount,
        delimiter,
        offset,
        key: None,
        index: 0usize,
    };

    let delimiter_lit = match &encoding.delimiter {
        Some(d) => {
            let d = **d;
            quote! { ::std::slice::from_ref(&#d) }
        }
        None => quote! { &[] },
    };

    let setup = quote! {
        let mut #skipcount = 0usize;
        const #delimiter: &[u8] = #delimiter_lit;
    };

    if !params.has_mux {
        for (i, field) in fields.iter().enumerate() {
            encoding_ctx.index = i;
            lines.push(generate_simple_field_encoding(field, &encoding_ctx));
        }
    } else {
        let mut param_groups: Vec<Vec<&FieldInfo>> = Vec::new();

        for field in fields {
            if let Some(mux_index) = &field.mux_index {
                let mux_index = match mux_index {
                    syn::Member::Unnamed(idx) => idx.index as usize,
                    _ => panic!(
                        "expected syn::Member::Unnamed, got syn::Member::Named"
                    ),
                };
                param_groups[mux_index].push(field);
            } else {
                param_groups.push(vec![field]);
            }
        }

        // Generate encoding for each muxed parameter
        for (i, fields) in param_groups.iter().enumerate() {
            encoding_ctx.index = i;
            if fields.len() == 1 {
                // No multiplexing - simple case
                lines.push(generate_simple_field_encoding(
                    fields[0],
                    &encoding_ctx,
                ));
            } else {
                // Multiplexing: combine primary field with mux fields using
                // AnsiMuxEncode
                lines.push(generate_multiplexed_field_encoding(
                    fields,
                    &encoding_ctx,
                ));
            }
        }
    }

    Ok(quote! {
        {
            #setup
            #(#lines)*
        }
    })
}

struct FieldEncodingContext<'a> {
    sink: &'a syn::Ident,
    counter: &'a syn::Ident,
    skipcount: &'a syn::Ident,
    delimiter: &'a syn::Ident,
    offset: usize,
    key: Option<String>,
    index: usize,
}

/// Generate encoding for a simple (non-multiplexed) field.
///
/// Create encoding code for a field that doesn't use multiplexing.
fn generate_simple_field_encoding(
    field: &FieldInfo,
    ctx: &FieldEncodingContext,
) -> TokenStream {
    let field_name = &field.member;

    let FieldEncodingContext {
        sink,
        counter,
        skipcount,
        delimiter,
        offset,
        key,
        index,
    } = ctx;

    let emit_key = if let Some(key) = key {
        quote! {
            #counter += ::vtansi::write_str_into(#sink, #key)?;
            #counter += ::vtansi::write_str_into(#sink, "=")?;
        }
    } else {
        quote! {}
    };

    let emit_delim = if (*index + offset) > 0 {
        quote! {
            for _ in 0 .. #skipcount + 1 {
                #counter += ::vtansi::write_bytes_into(#sink, #delimiter)?;
            }
        }
    } else {
        quote! {}
    };

    let field_type = &field.inner_ty;

    if field.is_optional {
        quote! {
            if let Some(value) = &self.#field_name {
                #emit_delim
                #emit_key
                #counter += <#field_type as ::vtansi::encode::AnsiEncode>::encode_ansi_into(value, #sink)?;
            } else {
                #skipcount += 1;
            }
        }
    } else {
        quote! {
            #emit_delim
            #emit_key
            #counter += <#field_type as ::vtansi::encode::AnsiEncode>::encode_ansi_into(&self.#field_name, #sink)?;
        }
    }
}

/// Generate encoding for a multiplexed field.
///
/// Create encoding code for a field that combines multiple fields using
/// AnsiMuxEncode.
fn generate_multiplexed_field_encoding(
    mux_fields: &[&FieldInfo],
    ctx: &FieldEncodingContext,
) -> TokenStream {
    let FieldEncodingContext {
        sink,
        counter,
        skipcount,
        delimiter,
        offset,
        key,
        index,
    } = ctx;
    let primary_field = mux_fields[0];
    let mux_fields = &mux_fields[1..];
    let primary_name = &primary_field.ident();
    let is_optional = primary_field.is_optional;
    let mux_value =
        syn::Ident::new("__vtansi_value", proc_macro2::Span::mixed_site());

    let emit_key = if let Some(key) = key {
        quote! {
            #counter += ::vtansi::write_str_into(#sink, #key)?;
            #counter += ::vtansi::write_str_into(#sink, "=")?;
        }
    } else {
        quote! {}
    };

    let emit_delim = if (*index + offset) > 0 {
        quote! {
            #counter += ::vtansi::write_bytes_into(#sink, #delimiter)?;
        }
    } else {
        quote! {}
    };

    // Generate chained mux_encode calls
    let mux_calls = mux_fields.iter().map(|mux_field| {
        let mux_name = &mux_field.ident();
        let mux_ty = &mux_field.ty;
        quote! {
            let #mux_value = <#mux_ty as ::vtansi::encode::AnsiMuxEncode>::mux_encode(&self.#mux_name, Some(&#mux_value))?;
        }
    });

    if is_optional {
        quote! {
            if let Some(value) = &self.#primary_name {
                #emit_delim
                #emit_key
                let #mux_value = <_ as ::vtansi::encode::AnsiMuxEncode>::mux_encode(value, None)?;
                #(#mux_calls)*
                #counter += <_ as ::vtansi::encode::AnsiEncode>::encode_ansi_into(&#mux_value, #sink)?;
            } else {
                #skipcount += 1;
            }
        }
    } else {
        quote! {
            #emit_delim
            #emit_key
            let #mux_value = <_ as ::vtansi::encode::AnsiMuxEncode>::mux_encode(&self.#primary_name, None)?;
            #(#mux_calls)*
            #counter += <_ as ::vtansi::encode::AnsiEncode>::encode_ansi_into(&#mux_value, #sink)?;
        }
    }
}
