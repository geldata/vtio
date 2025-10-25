//! Derive macros for `FromAnsi` and `ToAnsi` traits.
//!
//! This crate provides derive macros for the `FromAnsi` and `ToAnsi` traits
//! from the `vtenc` crate. These can be used on enums that either have a
//! primitive integer representation or implement conversion traits for
//! strings.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

/// Derive macro for `FromAnsi` trait.
///
/// This macro can be applied to enums that either:
/// 1. Have a primitive integer representation (e.g., `#[repr(u8)]`)
/// 2. Implement `std::convert::TryFrom<&str>`
///
/// # Examples
///
/// For enums with primitive representations:
///
/// ```ignore
/// #[derive(FromAnsi)]
/// #[repr(u8)]
/// enum Color {
///     Red = 0,
///     Green = 1,
///     Blue = 2,
/// }
/// ```
///
/// For enums implementing `TryFrom<&str>`:
///
/// ```ignore
/// #[derive(FromAnsi)]
/// enum Mode {
///     Normal,
///     Insert,
/// }
///
/// impl TryFrom<&str> for Mode {
///     type Error = String;
///     fn try_from(s: &str) -> Result<Self, Self::Error> {
///         // implementation
///     }
/// }
/// ```
#[proc_macro_derive(FromAnsi)]
pub fn derive_from_ansi(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Check if this is an enum
    let Data::Enum(_) = &input.data else {
        return syn::Error::new_spanned(
            &input,
            "FromAnsi can only be derived for enums",
        )
        .to_compile_error()
        .into();
    };

    // Check for repr attribute and extract the type
    let repr_type = input.attrs.iter().find_map(|attr| {
        if !attr.path().is_ident("repr") {
            return None;
        }
        
        // Parse the repr attribute to get the primitive type
        let Ok(meta) = attr.parse_args::<syn::Ident>() else {
            return None;
        };
        
        let type_str = meta.to_string();
        if matches!(
            type_str.as_str(),
            "u8" | "i8" | "u16" | "i16" | "u32" | "i32" | "u64" | "i64" | "usize" | "isize"
        ) {
            Some(meta)
        } else {
            None
        }
    });

    let expanded = if let Some(repr_type) = repr_type {
        // Generate implementation using the primitive representation
        quote! {
            impl #impl_generics ::vtenc::parse::TryFromAnsi<'_> for #name #ty_generics #where_clause {
                fn try_from_ansi(bytes: &[u8]) -> ::core::result::Result<Self, ::vtenc::parse::ParseError> {
                    use ::core::convert::TryFrom;
                    
                    // Parse as the repr type
                    let num = <#repr_type as ::vtenc::parse::TryFromAnsi>::try_from_ansi(bytes)?;
                    
                    // Convert to enum using TryFrom
                    Self::try_from(num).map_err(|_| {
                        ::vtenc::parse::ParseError::InvalidValue(
                            ::std::format!("invalid enum discriminant: {}", num)
                        )
                    })
                }
            }
        }
    } else {
        // Generate implementation using TryFrom<&str>
        quote! {
            impl #impl_generics ::vtenc::parse::TryFromAnsi<'_> for #name #ty_generics #where_clause {
                fn try_from_ansi(bytes: &[u8]) -> ::core::result::Result<Self, ::vtenc::parse::ParseError> {
                    use ::core::convert::TryFrom;
                    
                    // Parse as &str
                    let s = <&str as ::vtenc::parse::TryFromAnsi>::try_from_ansi(bytes)?;
                    
                    // Convert to enum using TryFrom<&str>
                    Self::try_from(s).map_err(|_| {
                        ::vtenc::parse::ParseError::InvalidValue(
                            ::std::format!("invalid enum value: {}", s)
                        )
                    })
                }
            }
        }
    };

    TokenStream::from(expanded)
}

/// Derive macro for `ToAnsi` trait.
///
/// This macro can be applied to enums that either:
/// 1. Have a primitive integer representation (e.g., `#[repr(u8)]`)
/// 2. Implement `std::convert::Into<&'static str>` or `AsRef<str>`
///
/// # Examples
///
/// For enums with primitive representations:
///
/// ```ignore
/// #[derive(ToAnsi)]
/// #[repr(u8)]
/// enum Color {
///     Red = 0,
///     Green = 1,
///     Blue = 2,
/// }
/// ```
///
/// For enums implementing `AsRef<str>`:
///
/// ```ignore
/// #[derive(ToAnsi)]
/// enum Mode {
///     Normal,
///     Insert,
/// }
///
/// impl AsRef<str> for Mode {
///     fn as_ref(&self) -> &str {
///         // implementation
///     }
/// }
/// ```
#[proc_macro_derive(ToAnsi)]
pub fn derive_to_ansi(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Check if this is an enum
    let Data::Enum(_) = &input.data else {
        return syn::Error::new_spanned(
            &input,
            "ToAnsi can only be derived for enums",
        )
        .to_compile_error()
        .into();
    };

    // Check for repr attribute and extract the type
    let repr_type = input.attrs.iter().find_map(|attr| {
        if !attr.path().is_ident("repr") {
            return None;
        }
        
        // Parse the repr attribute to get the primitive type
        let Ok(meta) = attr.parse_args::<syn::Ident>() else {
            return None;
        };
        
        let type_str = meta.to_string();
        if matches!(
            type_str.as_str(),
            "u8" | "i8" | "u16" | "i16" | "u32" | "i32" | "u64" | "i64" | "usize" | "isize"
        ) {
            Some(meta)
        } else {
            None
        }
    });

    let expanded = if let Some(repr_type) = repr_type {
        // Generate implementation using the primitive representation
        quote! {
            impl #impl_generics ::vtenc::encode::ToAnsi for #name #ty_generics #where_clause {
                fn to_ansi(&self) -> impl ::vtenc::encode::AnsiEncode {
                    // Convert enum to its repr type
                    *self as #repr_type
                }
            }
        }
    } else {
        // Generate implementation using AsRef<str>
        quote! {
            impl #impl_generics ::vtenc::encode::ToAnsi for #name #ty_generics #where_clause {
                fn to_ansi(&self) -> impl ::vtenc::encode::AnsiEncode {
                    // Convert to string slice using AsRef<str>
                    <Self as ::core::convert::AsRef<str>>::as_ref(self)
                }
            }
        }
    };

    TokenStream::from(expanded)
}