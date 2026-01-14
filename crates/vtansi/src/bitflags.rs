//! Support for bitflags types.
//!
//! This module provides a wrapper around the `bitflags!` macro that
//! automatically implements `AnsiEncode` and `TryFromAnsi` traits for
//! bitflags types.
//!
//! # Usage
//!
//! Simply use `vtansi::bitflags!` instead of `bitflags::bitflags!`:
//!
//! ```ignore
//! use vtansi::bitflags;
//! use vtansi::derive::{ToAnsi, FromAnsi};
//!
//! bitflags! {
//!     pub struct MyFlags: u8 {
//!         const FLAG_A = 0x01;
//!         const FLAG_B = 0x02;
//!         const FLAG_C = 0x04;
//!     }
//! }
//!
//! // AnsiEncode and TryFromAnsi are automatically implemented!
//! // Now use MyFlags in structs with ToAnsi/FromAnsi
//! #[derive(ToAnsi, FromAnsi)]
//! struct MyStruct {
//!     flags: MyFlags,
//!     value: u16,
//! }
//! ```
//!
//! For bitflags that require bit transformations during encoding/decoding,
//! add `encode:` and `decode:` closures after the struct definition:
//!
//! ```ignore
//! use vtansi::bitflags;
//!
//! bitflags! {
//!     pub struct MyFlags: u8 {
//!         const FLAG_A = 0x01;
//!         const FLAG_B = 0x02;
//!     }
//!     encode: |bits| bits | 0x40,
//!     decode: |bits| bits & !0x40,
//! }
//! // Custom transformations applied during encode/decode
//! ```

#[macro_export]
macro_rules! bitflags {
    // Version with custom encode/decode transformations
    (
        $(#[$attr:meta])*
        $vis:vis struct $name:ident: $type:ty {
            $($const_items:tt)*
        }
        encode: $encode_expr:expr,
        decode: $decode_expr:expr,
    ) => {
        ::bitflags::bitflags! {
            $(#[$attr])*
            $vis struct $name: $type {
                $($const_items)*
            }
        }

        #[automatically_derived]
        impl $crate::encode::AnsiEncode for $name {
            const ENCODED_LEN: ::core::option::Option<usize> = <
                <$name as $crate::__private::bitflags::Flags>::Bits
                as $crate::encode::AnsiEncode
            >::ENCODED_LEN;

            #[inline]
            fn encode_ansi_into<W: ::std::io::Write + ?::std::marker::Sized>(
                &self,
                sink: &mut W,
            ) -> ::std::result::Result<usize, $crate::encode::EncodeError> {
                let bits = <$name as $crate::__private::bitflags::Flags>::bits(self);
                let transform = $encode_expr;
                let bits = transform(bits);
                $crate::encode::AnsiEncode::encode_ansi_into(&bits, sink)
            }
        }

        #[automatically_derived]
        impl $crate::parse::TryFromAnsi<'_> for $name {
            #[inline]
            fn try_from_ansi(
                bytes: &[u8],
            ) -> ::std::result::Result<Self, $crate::parse::ParseError> {
                type BitsType = <$name as $crate::__private::bitflags::Flags>::Bits;
                let bits =
                    <BitsType as $crate::parse::TryFromAnsi>::try_from_ansi(bytes)?;
                let transform = $decode_expr;
                let bits = transform(bits);
                ::std::result::Result::Ok(<$name as $crate::__private::bitflags::Flags>::from_bits_retain(
                    bits,
                ))
            }
        }
    };

    // Standard version without transformations
    (
        $($input:tt)*
    ) => {
        // First, call the upstream bitflags macro with all input
        ::bitflags::bitflags! {
            $($input)*
        }

        // Then extract the type name and generate trait implementations
        $crate::__bitflags_impl_traits! {
            $($input)*
        }
    };
}

/// Helper macro to extract type name from bitflags input and generate traits
#[doc(hidden)]
#[macro_export]
macro_rules! __bitflags_impl_traits {
    // Match struct definition
    (
        $(#[$_attr:meta])*
        $vis:vis struct $name:ident: $($_rest:tt)*
    ) => {
        $crate::impl_bitflags_traits!($name);
    };

    // Match impl definition
    (
        impl $name:ident: $($_rest:tt)*
    ) => {
        $crate::impl_bitflags_traits!($name);
    };
}

/// Generate `AnsiEncode` and `TryFromAnsi` implementations for a bitflags type.
///
/// This macro is called automatically by the `vtansi::bitflags!` macro, but
/// can also be used directly if you need to implement traits for an existing
/// bitflags type.
///
/// # Example
///
/// ```ignore
/// use bitflags::bitflags;
/// use vtansi::impl_bitflags_traits;
///
/// bitflags! {
///     pub struct StatusFlags: u32 {
///         const READY = 0x01;
///     }
/// }
///
/// impl_bitflags_traits!(StatusFlags);
/// ```
#[macro_export]
macro_rules! impl_bitflags_traits {
    ($type:ty) => {
        #[automatically_derived]
        impl $crate::encode::AnsiEncode for $type {
            const ENCODED_LEN: ::core::option::Option<usize> = <
                <$type as $crate::__private::bitflags::Flags>::Bits
                as $crate::encode::AnsiEncode
            >::ENCODED_LEN;

            #[inline]
            fn encode_ansi_into<W: ::std::io::Write + ?::std::marker::Sized>(
                &self,
                sink: &mut W,
            ) -> ::std::result::Result<usize, $crate::encode::EncodeError> {
                $crate::encode::AnsiEncode::encode_ansi_into(
                    &<$type as $crate::__private::bitflags::Flags>::bits(self),
                    sink
                )
            }
        }

        #[automatically_derived]
        impl $crate::parse::TryFromAnsi<'_> for $type {
            #[inline]
            fn try_from_ansi(
                bytes: &[u8],
            ) -> ::std::result::Result<Self, $crate::parse::ParseError> {
                type BitsType = <$type as $crate::__private::bitflags::Flags>::Bits;
                let bits =
                    <BitsType as $crate::parse::TryFromAnsi>::try_from_ansi(bytes)?;
                ::std::result::Result::Ok(<$type as $crate::__private::bitflags::Flags>::from_bits_retain(
                    bits,
                ))
            }
        }
    };
}

#[cfg(test)]
mod tests {
    // Test using the wrapper macro
    crate::bitflags! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct TestFlags: u8 {
            const FLAG_A = 0x01;
            const FLAG_B = 0x02;
            const FLAG_C = 0x04;
        }
    }

    // Traits are automatically implemented by the wrapper macro

    #[test]
    fn test_bitflags_encode() {
        use crate::encode::AnsiEncode;

        let flags = TestFlags::FLAG_A | TestFlags::FLAG_C;
        let encoded = flags.encode_ansi().unwrap();
        assert_eq!(encoded, b"5"); // 0x01 | 0x04 = 5
    }

    #[test]
    fn test_bitflags_decode() {
        use crate::parse::TryFromAnsi;

        let flags = TestFlags::try_from_ansi(b"5").unwrap();
        assert_eq!(flags, TestFlags::FLAG_A | TestFlags::FLAG_C);
    }

    #[test]
    fn test_bitflags_roundtrip() {
        use crate::encode::AnsiEncode;
        use crate::parse::TryFromAnsi;

        let original = TestFlags::FLAG_B | TestFlags::FLAG_C;
        let encoded = original.encode_ansi().unwrap();
        let decoded = TestFlags::try_from_ansi(&encoded).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn test_bitflags_empty() {
        use crate::encode::AnsiEncode;
        use crate::parse::TryFromAnsi;

        let empty = TestFlags::empty();
        let encoded = empty.encode_ansi().unwrap();
        assert_eq!(encoded, b"0");

        let decoded = TestFlags::try_from_ansi(b"0").unwrap();
        assert_eq!(decoded, TestFlags::empty());
    }

    #[test]
    fn test_bitflags_unknown_bits_retained() {
        use crate::parse::TryFromAnsi;

        // Parse a value with unknown bits (0x80)
        let flags = TestFlags::try_from_ansi(b"133").unwrap(); // 0x80 | 0x04 | 0x01
        assert_eq!(flags.bits(), 133);
        assert!(flags.contains(TestFlags::FLAG_A));
        assert!(flags.contains(TestFlags::FLAG_C));
    }

    // Test the bitflags macro with transformations
    crate::bitflags! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct OffsetFlags: u8 {
            const FLAG_A = 0x01;
            const FLAG_B = 0x02;
            const FLAG_C = 0x04;
        }
        encode: |bits| bits | 0x40,
        decode: |bits| bits & !0x40,
    }

    #[test]
    fn test_bitflags_transform_encode() {
        use crate::encode::AnsiEncode;

        let flags = OffsetFlags::FLAG_A | OffsetFlags::FLAG_C;
        let encoded = flags.encode_ansi().unwrap();
        // 0x01 | 0x04 | 0x40 = 0x45 = 69
        assert_eq!(encoded, b"69");
    }

    #[test]
    fn test_bitflags_transform_decode() {
        use crate::parse::TryFromAnsi;

        // Input has offset included
        let flags = OffsetFlags::try_from_ansi(b"69").unwrap();
        // Should decode to just the flags, with offset stripped
        assert_eq!(flags, OffsetFlags::FLAG_A | OffsetFlags::FLAG_C);
        assert_eq!(flags.bits(), 0x05);
    }

    #[test]
    fn test_bitflags_transform_roundtrip() {
        use crate::encode::AnsiEncode;
        use crate::parse::TryFromAnsi;

        let original = OffsetFlags::FLAG_B | OffsetFlags::FLAG_C;
        let encoded = original.encode_ansi().unwrap();
        let decoded = OffsetFlags::try_from_ansi(&encoded).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn test_bitflags_transform_empty() {
        use crate::encode::AnsiEncode;
        use crate::parse::TryFromAnsi;

        let empty = OffsetFlags::empty();
        let encoded = empty.encode_ansi().unwrap();
        // Just the offset
        assert_eq!(encoded, b"64"); // 0x40 = 64

        let decoded = OffsetFlags::try_from_ansi(b"64").unwrap();
        assert_eq!(decoded, OffsetFlags::empty());
    }
}
