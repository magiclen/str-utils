use alloc::borrow::Cow;

use crate::{IsAsciiUppercased, IsUppercased};

/// To extend `str` and `Cow<str>` to have `to_uppercase_cow` and `to_ascii_uppercase_cow` methods.
pub trait ToUppercase<'a> {
    /// Converts the string to its uppercase form (Unicode-aware), returning a `Cow<str>` to avoid allocation when possible.
    fn to_uppercase_cow(self) -> Cow<'a, str>;

    /// Converts the string to its ASCII uppercase form, returning a `Cow<str>` to avoid allocation when possible.
    fn to_ascii_uppercase_cow(self) -> Cow<'a, str>;
}

impl<'a> ToUppercase<'a> for &'a str {
    #[inline]
    fn to_uppercase_cow(self) -> Cow<'a, str> {
        if self.is_uppercased() {
            Cow::Borrowed(self)
        } else {
            Cow::Owned(self.to_uppercase())
        }
    }

    #[inline]
    fn to_ascii_uppercase_cow(self) -> Cow<'a, str> {
        if self.is_ascii_uppercased() {
            Cow::Borrowed(self)
        } else {
            Cow::Owned(self.to_ascii_uppercase())
        }
    }
}

impl<'a> ToUppercase<'a> for Cow<'a, str> {
    #[inline]
    fn to_uppercase_cow(self) -> Cow<'a, str> {
        match self {
            Cow::Borrowed(s) => s.to_uppercase_cow(),
            Cow::Owned(s) => Cow::Owned(cow_into_owned!(s, s.as_str().to_uppercase_cow())),
        }
    }

    #[inline]
    fn to_ascii_uppercase_cow(self) -> Cow<'a, str> {
        match self {
            Cow::Borrowed(s) => s.to_ascii_uppercase_cow(),
            Cow::Owned(s) => Cow::Owned(cow_into_owned!(s, s.as_str().to_ascii_uppercase_cow(),)),
        }
    }
}
