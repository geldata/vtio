//! Mode control commands.

/// Generate terminal mode control structures (enable, disable, request,
/// response).
#[macro_export]
macro_rules! terminal_mode {
    ($(#[$meta:meta])* $name:ident, $code:literal) => {
        ::paste::paste! {
            $(#[$meta])*
            #[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
            pub struct [<Enable $name>];

            impl vtenc::ConstEncode for [<Enable $name>] {
                const STR: &'static str = vtenc::csi!($code, "h");
            }

            $(#[$meta])*
            #[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
            pub struct [<Disable $name>];

            impl vtenc::ConstEncode for [<Disable $name>] {
                const STR: &'static str = vtenc::csi!($code, "l");
            }

            $(#[$meta])*
            #[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
            pub struct [<Request $name>];

            impl vtenc::ConstEncode for [<Request $name>] {
                const STR: &'static str = vtenc::csi!($code, "$p");
            }

            $(#[$meta])*
            #[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
            pub struct $name(pub bool);

            impl vtenc::Encode for $name {
                #[inline]
                fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, vtenc::EncodeError> {
                    vtenc::write_csi!(buf; $code, ";", self.0, "$y")
                }
            }
        }
    };
}
