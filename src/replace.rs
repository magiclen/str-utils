use alloc::borrow::Cow;

use crate::Pattern;

/// To extend `str` and `Cow<str>` to have `replace_cow` and `replacen_cow` methods.
pub trait Replace<'a> {
    /// Replaces all matches of a pattern with another string, returning a `Cow<str>` to avoid allocation when possible.
    fn replace_cow<P: Pattern>(self, from: P, to: &str) -> Cow<'a, str>;

    /// Replaces at most `count` matches of a pattern with another string, returning a `Cow<str>` to avoid allocation when possible.
    fn replacen_cow<P: Pattern>(self, from: P, to: &str, count: usize) -> Cow<'a, str>;
}

impl<'a> Replace<'a> for &'a str {
    #[inline]
    fn replace_cow<P: Pattern>(self, from: P, to: &str) -> Cow<'a, str> {
        from.replace_from(self, to)
    }

    #[inline]
    fn replacen_cow<P: Pattern>(self, from: P, to: &str, count: usize) -> Cow<'a, str> {
        from.replacen_from(self, to, count)
    }
}

impl<'a> Replace<'a> for Cow<'a, str> {
    #[inline]
    fn replace_cow<P: Pattern>(self, from: P, to: &str) -> Cow<'a, str> {
        match self {
            Cow::Borrowed(s) => s.replace_cow(from, to),
            Cow::Owned(s) => Cow::Owned(cow_into_owned!(s, s.as_str().replace_cow(from, to),)),
        }
    }

    #[inline]
    fn replacen_cow<P: Pattern>(self, from: P, to: &str, count: usize) -> Cow<'a, str> {
        match self {
            Cow::Borrowed(s) => s.replacen_cow(from, to, count),
            Cow::Owned(s) => {
                Cow::Owned(cow_into_owned!(s, s.as_str().replacen_cow(from, to, count),))
            },
        }
    }
}
