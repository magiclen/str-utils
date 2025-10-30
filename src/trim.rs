use alloc::borrow::Cow;

use crate::to_substring_in_place;

/// To extend `Cow<str>` to have `trim_cow`, `trim_start_cow`, `trim_end_cow`, `trim_ascii_cow`, `trim_ascii_start_cow`, `trim_ascii_end_cow` methods.
pub trait Trim<'a> {
    /// Trims both leading and trailing Unicode whitespace characters.
    fn trim_cow(self) -> Cow<'a, str>;

    /// Trims leading (left) Unicode whitespace characters.
    fn trim_start_cow(self) -> Cow<'a, str>;

    /// Trims trailing (right) Unicode whitespace characters.
    fn trim_end_cow(self) -> Cow<'a, str>;

    /// Trims both leading and trailing **ASCII** whitespace characters.
    fn trim_ascii_cow(self) -> Cow<'a, str>;

    /// Trims leading (left) **ASCII** whitespace characters.
    fn trim_ascii_start_cow(self) -> Cow<'a, str>;

    /// Trims trailing (right) **ASCII** whitespace characters.
    fn trim_ascii_end_cow(self) -> Cow<'a, str>;
}

impl<'a> Trim<'a> for Cow<'a, str> {
    #[inline]
    fn trim_cow(self) -> Cow<'a, str> {
        match self {
            Cow::Borrowed(s) => Cow::Borrowed(s.trim()),
            Cow::Owned(s) => {
                let ss = s.trim();

                Cow::Owned(unsafe { to_substring_in_place!(s, ss) })
            },
        }
    }

    #[inline]
    fn trim_start_cow(self) -> Cow<'a, str> {
        match self {
            Cow::Borrowed(s) => Cow::Borrowed(s.trim_start()),
            Cow::Owned(s) => {
                let ss = s.trim_start();

                Cow::Owned(unsafe { to_substring_in_place!(s, ss) })
            },
        }
    }

    #[inline]
    fn trim_end_cow(self) -> Cow<'a, str> {
        match self {
            Cow::Borrowed(s) => Cow::Borrowed(s.trim_end()),
            Cow::Owned(s) => {
                let ss = s.trim_end();

                Cow::Owned(unsafe { to_substring_in_place!(s, ss) })
            },
        }
    }

    #[inline]
    fn trim_ascii_cow(self) -> Cow<'a, str> {
        match self {
            Cow::Borrowed(s) => Cow::Borrowed(s.trim_ascii()),
            Cow::Owned(s) => {
                let ss = s.trim_ascii();

                Cow::Owned(unsafe { to_substring_in_place!(s, ss) })
            },
        }
    }

    #[inline]
    fn trim_ascii_start_cow(self) -> Cow<'a, str> {
        match self {
            Cow::Borrowed(s) => Cow::Borrowed(s.trim_ascii_start()),
            Cow::Owned(s) => {
                let ss = s.trim_ascii_start();

                Cow::Owned(unsafe { to_substring_in_place!(s, ss) })
            },
        }
    }

    #[inline]
    fn trim_ascii_end_cow(self) -> Cow<'a, str> {
        match self {
            Cow::Borrowed(s) => Cow::Borrowed(s.trim_ascii_end()),
            Cow::Owned(s) => {
                let ss = s.trim_ascii_end();

                Cow::Owned(unsafe { to_substring_in_place!(s, ss) })
            },
        }
    }
}
