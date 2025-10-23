use smallvec::{SmallVec};
use crate::bytevec;

pub type EscapeSequenceParam = bytevec::NonEmptyBytes<32>;
pub type EscapeSequenceParams = SmallVec<[EscapeSequenceParam; 8]>;

impl From<EscapeSequenceParam> for bool {
    fn from(param: EscapeSequenceParam) -> Self {
        param.first() != 0
    }
}

impl From<&EscapeSequenceParam> for bool {
    fn from(param: &EscapeSequenceParam) -> Self {
        param.first() != 0
    }
}

impl From<EscapeSequenceParam> for String {
    fn from(param: EscapeSequenceParam) -> Self {
        String::from_utf8_lossy(&param).into_owned()
    }
}

impl From<&EscapeSequenceParam> for String {
    fn from(param: &EscapeSequenceParam) -> Self {
        String::from_utf8_lossy(param).into_owned()
    }
}

impl From<EscapeSequenceParam> for char {
    fn from(param: EscapeSequenceParam) -> Self {
        param.first() as char
    }
}

impl From<&EscapeSequenceParam> for char {
    fn from(param: &EscapeSequenceParam) -> Self {
        param.first() as char
    }
}

// Macro to implement From<EscapeSequenceParam> for numeric types
macro_rules! impl_from_param_numeric {
    ($($t:ty),+ $(,)?) => {
        $(
            impl From<EscapeSequenceParam> for $t {
                #[allow(clippy::cast_lossless, clippy::cast_possible_wrap)]
                fn from(param: EscapeSequenceParam) -> Self {
                    param.first() as $t
                }
            }

            impl From<&EscapeSequenceParam> for $t {
                #[allow(clippy::cast_lossless, clippy::cast_possible_wrap)]
                fn from(param: &EscapeSequenceParam) -> Self {
                    param.first() as $t
                }
            }
        )+
    };
}

impl_from_param_numeric! {
    u8, i8, u16, i16, u32, i32, u64, i64, usize, isize
}
