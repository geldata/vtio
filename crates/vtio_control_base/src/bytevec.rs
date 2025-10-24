use smallvec::SmallVec;
use std::ops::Deref;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NonEmptyBytes<const N: usize>(SmallVec<[u8; N]>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EmptyError;

impl<const N: usize> TryFrom<SmallVec<[u8; N]>> for NonEmptyBytes<N> {
    type Error = EmptyError;

    fn try_from(v: SmallVec<[u8; N]>) -> Result<Self, Self::Error> {
        if v.is_empty() {
            Err(EmptyError)
        } else {
            Ok(Self(v))
        }
    }
}

impl<const N: usize> From<NonEmptyBytes<N>> for SmallVec<[u8; N]> {
    fn from(ne: NonEmptyBytes<N>) -> Self {
        ne.0
    }
}

impl<const N: usize> NonEmptyBytes<N> {
    pub const fn from_u8(byte: u8) -> Self
    {
        let mut arr = [0u8; N];
        arr[0] = byte;
        // SAFETY: 1 <= N
        unsafe {
            Self(SmallVec::<[u8; N]>::from_const_with_len_unchecked(arr, 1))
        }
    }

    pub fn try_from_slice(s: &[u8]) -> Result<Self, EmptyError> {
        if s.is_empty() {
            return Err(EmptyError);
        }
        Ok(Self(SmallVec::<[u8; N]>::from_slice(s)))
    }

    pub const unsafe fn from_const_with_len_unchecked(value: [u8; N], len: usize) -> Self {
        unsafe {
            Self(SmallVec::<[u8; N]>::from_const_with_len_unchecked(value, len))
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }

    #[inline]
    pub fn first(&self) -> u8 {
        self.0[0]
    }
}

impl<const N: usize> Deref for NonEmptyBytes<N> {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

impl<const N: usize> AsRef<[u8]> for NonEmptyBytes<N> {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.0.as_slice()
    }
}
