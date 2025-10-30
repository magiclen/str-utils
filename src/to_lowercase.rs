use alloc::{borrow::Cow, str};

use crate::{IsAsciiLowercased, IsLowercased};

/// To extend `str` and `Cow<str>` to have `to_lowercase_cow` and `to_ascii_lowercase_cow` methods.
pub trait ToLowercase<'a> {
    /// Converts the string to its lowercase form (Unicode-aware), returning a `Cow<str>` to avoid allocation when possible.
    fn to_lowercase_cow(self) -> Cow<'a, str>;

    /// Converts the string to its ASCII lowercase form, returning a `Cow<str>` to avoid allocation when possible.
    fn to_ascii_lowercase_cow(self) -> Cow<'a, str>;
}

impl<'a> ToLowercase<'a> for &'a str {
    #[inline]
    fn to_lowercase_cow(self) -> Cow<'a, str> {
        if self.is_lowercased() {
            Cow::from(self)
        } else {
            Cow::from(self.to_lowercase())
        }
    }

    #[inline]
    fn to_ascii_lowercase_cow(self) -> Cow<'a, str> {
        if self.is_ascii_lowercased() {
            Cow::from(self)
        } else {
            Cow::from(self.to_ascii_lowercase())
        }
    }
}

impl<'a> ToLowercase<'a> for Cow<'a, str> {
    #[inline]
    fn to_lowercase_cow(self) -> Cow<'a, str> {
        match self {
            Cow::Borrowed(s) => s.to_lowercase_cow(),
            Cow::Owned(s) => {
                match s.to_lowercase_cow() {
                    Cow::Borrowed(_) => {
                        // it changes nothing
                        // if there were any characters that needed to be lowercased, it had to be `Cow::Owned`
                        Cow::Owned(s)
                    },
                    Cow::Owned(s) => Cow::Owned(s),
                }
            },
        }
    }

    #[inline]
    fn to_ascii_lowercase_cow(self) -> Cow<'a, str> {
        match self {
            Cow::Borrowed(s) => s.to_ascii_lowercase_cow(),
            Cow::Owned(s) => {
                match s.to_ascii_lowercase_cow() {
                    Cow::Borrowed(_) => {
                        // it changes nothing
                        // if there were any characters that needed to be lowercased, it had to be `Cow::Owned`
                        Cow::Owned(s)
                    },
                    Cow::Owned(s) => Cow::Owned(s),
                }
            },
        }
    }
}
