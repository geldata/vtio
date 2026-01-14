//! Implementation of the `ToAnsi` derive macro.
//!
//! This module generates implementations of the `ToAnsi` trait for both enums
//! and structs.
//!
//! # Enum Support
//!
//! For enums, it supports two encoding strategies:
//!
//! 1. **Primitive representation** - for enums with `#[repr(u8)]` and similar
//!    attributes, encoding as integer values
//! 2. **String-based** - for enums without repr, encoding as string values
//!    via `AsRef<str>`
//!
//! The generated implementations are optimized with `#[inline]` attributes
//! for better performance.
//!
//! # Struct Support
//!
//! For structs, this module supports two modes:
//!
//! 1. **Normal structs** - encode fields as parameters with configurable
//!    format (map or vector) and delimiter
//! 2. **Transparent structs** - with `#[vtansi(transparent)]`, the struct
//!    acts as a newtype wrapper and delegates encoding to its single field.
//!    This is useful for creating type-safe wrappers around primitive types
//!    or types like bitflags that implement `AnsiEncode`.
//!
//! # Transparent Attribute
//!
//! The `#[vtansi(transparent)]` attribute works similarly to
//! `#[serde(transparent)]`:
//!
//! - The struct must have exactly one field
//! - Encoding delegates directly to that field's `AnsiEncode` implementation
//! - Useful for wrapping types like bitflags with custom encoding logic
//!
//! ## Example
//!
//! ```ignore
//! #[derive(ToAnsi)]
//! #[vtansi(transparent)]
//! struct Wrapper(u16);
//!
//! // Encodes as "42", not as a struct with fields
//! let wrapped = Wrapper(42);
//! ```

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields};

use crate::helpers::metadata::FieldLocation;
use crate::helpers::type_props::{HasFormatProperties, ValueProperties};
use crate::helpers::{
    HasTypeProperties, extract_struct_param_info, extract_vec_inner_type,
    generate_doc_imports, insert_lifetime, non_enum_error, non_struct_error,
};
use crate::macros::param_encoder::generate_param_encoding;

/// Generate the implementation of `ToAnsi` for an enum or struct.
///
/// This function dispatches to the appropriate generator based on whether the
/// input is an enum or a struct.
///
/// # Errors
///
/// Return an error if:
/// - The input is neither an enum nor a struct with named fields
/// - The attributes cannot be parsed
/// - The configuration is invalid
pub fn to_ansi_inner(ast: &DeriveInput) -> syn::Result<TokenStream> {
    // Generate doc imports for IDE hover support
    let doc_imports = generate_doc_imports(ast);

    let impl_code = match &ast.data {
        Data::Enum(_) => generate_enum_impl(ast),
        Data::Struct(data) => match &data.fields {
            Fields::Named(_) | Fields::Unnamed(_) => generate_struct_impl(ast),
            Fields::Unit => Err(syn::Error::new_spanned(
                ast,
                "ToAnsi cannot be derived for unit structs",
            )),
        },
        Data::Union(_) => Err(syn::Error::new_spanned(
            ast,
            "ToAnsi cannot be derived for unions",
        )),
    }?;

    Ok(quote! {
        #doc_imports
        #impl_code
    })
}

pub fn generate_struct_impl(ast: &DeriveInput) -> syn::Result<TokenStream> {
    let props = ast.get_type_properties()?;

    if props.transparent {
        generate_transparent_struct_impl(ast, &props)
    } else {
        generate_normal_struct_impl(ast, &props)
    }
}

fn generate_normal_struct_impl(
    ast: &DeriveInput,
    props: &ValueProperties,
) -> syn::Result<TokenStream> {
    let Data::Struct(_) = &ast.data else {
        return Err(non_struct_error());
    };

    let name = &ast.ident;
    let generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Handle normal structs
    let params = extract_struct_param_info(ast, None, FieldLocation::Params)?;
    let sink =
        syn::Ident::new("__vtansi_sink", proc_macro2::Span::mixed_site());
    let counter =
        syn::Ident::new("__vtansi_ctr", proc_macro2::Span::mixed_site());
    let params = generate_param_encoding(
        &params.params,
        &props.get_param_encoding(),
        &sink,
        &counter,
    )?;

    Ok(quote! {
        #[automatically_derived]
        impl #impl_generics ::vtansi::encode::AnsiEncode for #name #ty_generics #where_clause {
            #[inline]
            fn encode_ansi_into<W: ::std::io::Write + ?::std::marker::Sized>(
                &self,
                #sink: &mut W,
            ) -> Result<usize, ::vtansi::EncodeError> {
                let mut #counter = 0usize;
                #params
                ::core::result::Result::Ok(#counter)
            }
        }
    })
}

fn generate_transparent_struct_impl(
    ast: &DeriveInput,
    props: &ValueProperties,
) -> syn::Result<TokenStream> {
    let Data::Struct(data) = &ast.data else {
        return Err(non_struct_error());
    };
    let name = &ast.ident;
    let generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let fields = match &data.fields {
        Fields::Named(fields) => fields.named.iter().collect::<Vec<_>>(),
        Fields::Unnamed(fields) => fields.unnamed.iter().collect::<Vec<_>>(),
        Fields::Unit => Vec::new(),
    };

    if fields.len() != 1 {
        return Err(syn::Error::new_spanned(
            ast,
            format!(
                "transparent structs must have exactly one field, found {}",
                fields.len()
            ),
        ));
    }

    let field = fields[0];
    let field_ty = &field.ty;
    let field_access = match &field.ident {
        Some(ident) => quote! { &self.#ident },
        None => quote! { &self.0 },
    };

    // Check if the field type is Vec<T>
    if let Some(_inner_ty) = extract_vec_inner_type(field_ty) {
        // Vec type requires delimiter attribute
        let delimiter = &props.delimiter.to_literal();

        Ok(quote! {
            #[automatically_derived]
            impl #impl_generics ::vtansi::encode::AnsiEncode for #name #ty_generics #where_clause {
                #[inline]
                fn encode_ansi_into<W: ::std::io::Write + ?::std::marker::Sized>(
                    &self,
                    sink: &mut W,
                ) -> Result<usize, ::vtansi::EncodeError> {
                    let delimiter = #delimiter;
                    let mut counter = 0usize;
                    let mut first = true;

                    for item in #field_access {
                        if !first {
                            counter += ::vtansi::encode::write_bytes_into(
                                sink,
                                &[delimiter],
                            )?;
                        }
                        first = false;
                        counter += ::vtansi::encode::AnsiEncode::encode_ansi_into(item, sink)?;
                    }

                    ::core::result::Result::Ok(counter)
                }
            }
        })
    } else {
        // Non-Vec type: simple delegation
        Ok(quote! {
            #[automatically_derived]
            impl #impl_generics ::vtansi::encode::AnsiEncode for #name #ty_generics #where_clause {
                #[inline]
                fn encode_ansi_into<W: ::std::io::Write + ?::std::marker::Sized>(
                    &self,
                    sink: &mut W,
                ) -> Result<usize, ::vtansi::EncodeError> {
                    ::vtansi::encode::AnsiEncode::encode_ansi_into(#field_access, sink)
                }
            }
        })
    }
}

/// Generate the implementation of `ToAnsi` for an enum.
///
/// This function orchestrates the code generation process by:
/// 1. Extracting type-level properties (e.g., repr type)
/// 2. Delegating to the appropriate generation function based on the repr
///    type
///
/// # Errors
///
/// Return an error if:
/// - The attributes cannot be parsed
fn generate_enum_impl(ast: &DeriveInput) -> syn::Result<TokenStream> {
    let Data::Enum(_) = &ast.data else {
        return Err(non_enum_error());
    };

    let props = ast.get_type_properties()?;
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) =
        ast.generics.split_for_impl();

    let to_ansi_impls = if let Some(repr_type) = &props.repr_type {
        vec![quote! {
            #[automatically_derived]
            impl #impl_generics ::vtansi::encode::AnsiEncode for #name #ty_generics #where_clause {
                const ENCODED_LEN: ::core::option::Option<usize> = ::core::option::Option::Some(
                    <#repr_type as ::vtansi::__private::itoa::Integer>::MAX_STR_LEN
                );

                #[inline]
                fn encode_ansi_into<W: ::std::io::Write + ?::std::marker::Sized>(
                    &self,
                    sink: &mut W,
                ) -> Result<usize, ::vtansi::EncodeError> {
                    ::vtansi::encode::write_int(sink, #repr_type::from(*self))
                }
            }
        }]
    } else {
        let traits = &[(
            quote! { ::core::convert::TryInto<&'static str> },
            quote! {
                <Self as ::core::convert::TryInto<&'static str>>::try_into(*self)
                    .map_err(
                        |_| ::vtansi::encode::EncodeError::Unencodeable(
                            "could not convert value to &str".to_string()
                        )
                    )?
            },
        )];

        // Generate ENCODED_LEN constant based on encoded_len attribute
        let encoded_len_const = if let Some(ref len) = props.encoded_len {
            quote! {
                const ENCODED_LEN: ::core::option::Option<usize> = ::core::option::Option::Some(#len);
            }
        } else {
            quote! {}
        };

        traits.iter().map(
            |(t, m)| {
                let generics = ast.generics.clone();
                let impl_only_generics = ast.generics.clone();
                let mut for_generics: syn::Generics = syn::parse_quote!();
                let lt = insert_lifetime(&mut for_generics);
                let (_, ty_generics, _) = generics.split_for_impl();
                let (impl_generics, _, _) = impl_only_generics.split_for_impl();
                let mut generics = ast.generics.clone();
                let where_clause = generics.make_where_clause();
                where_clause.predicates.push(syn::parse_quote!(
                    for <#lt> #name #ty_generics: #t
                ));
                quote! {
                    #[automatically_derived]
                    impl #impl_generics ::vtansi::encode::AnsiEncode for #name #ty_generics #where_clause {
                        #encoded_len_const

                        #[inline]
                        fn encode_ansi_into<W: ::std::io::Write + ?::std::marker::Sized>(
                            &self,
                            sink: &mut W,
                        ) -> Result<usize, ::vtansi::EncodeError> {
                            ::vtansi::encode::write_str_into(sink, #m)
                        }
                    }
                }
            }
        ).collect()
    };

    Ok(quote! {
        #(#to_ansi_impls)*
    })
}
