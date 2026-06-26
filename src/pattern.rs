use alloc::{borrow::Cow, string::String};

mod sealed {
    pub trait Sealed {}

    impl Sealed for char {}
    impl Sealed for &str {}
    impl Sealed for &&str {}
    impl Sealed for &[char] {}

    impl<const N: usize> Sealed for [char; N] {}
    impl<const N: usize> Sealed for &[char; N] {}

    impl<F> Sealed for F where F: FnMut(char) -> bool {}
}

#[inline]
fn char_eq_str(c: char, s: &str) -> bool {
    let mut buffer = [0; 4];

    c.encode_utf8(&mut buffer) == s
}

fn replace_chars<'a, F>(s: &'a str, mut from: F, to: &str, mut count: usize) -> Cow<'a, str>
where
    F: FnMut(char) -> bool, {
    if count == 0 {
        return Cow::Borrowed(s);
    }

    let mut new_s: Option<String> = None;
    let mut start = 0;

    for (p, c) in s.char_indices() {
        if count == 0 {
            break;
        }

        if from(c) {
            let next = p + c.len_utf8();

            if char_eq_str(c, to) {
                count -= 1;

                continue;
            }

            if let Some(new_s) = new_s.as_mut() {
                new_s.push_str(&s[start..p]);
                new_s.push_str(to);
            } else {
                let mut owned = String::with_capacity(s.len());

                owned.push_str(&s[..p]);
                owned.push_str(to);

                new_s = Some(owned);
            }

            start = next;
            count -= 1;
        }
    }

    match new_s {
        Some(mut new_s) => {
            new_s.push_str(&s[start..]);

            Cow::Owned(new_s)
        },
        None => Cow::Borrowed(s),
    }
}

#[inline]
fn replace_str<'a>(s: &'a str, from: &str, to: &str) -> Cow<'a, str> {
    if from == to || !s.contains(from) {
        Cow::Borrowed(s)
    } else {
        Cow::Owned(s.replace(from, to))
    }
}

#[inline]
fn replacen_str<'a>(s: &'a str, from: &str, to: &str, count: usize) -> Cow<'a, str> {
    if count == 0 || from == to || !s.contains(from) {
        Cow::Borrowed(s)
    } else {
        Cow::Owned(s.replacen(from, to, count))
    }
}

/// A stable pattern type that can be used by `replace_cow` and `replacen_cow`.
///
/// This trait is local to this crate because the standard library `Pattern` trait is still unstable to name in public APIs.
pub trait Pattern: sealed::Sealed {
    /// Returns a `Cow<str>` after replacing all matches in the given string.
    #[doc(hidden)]
    fn replace_from<'a>(self, s: &'a str, to: &str) -> Cow<'a, str>;

    /// Returns a `Cow<str>` after replacing at most `count` matches in the given string.
    #[doc(hidden)]
    fn replacen_from<'a>(self, s: &'a str, to: &str, count: usize) -> Cow<'a, str>;
}

impl Pattern for char {
    #[inline]
    fn replace_from<'a>(self, s: &'a str, to: &str) -> Cow<'a, str> {
        replace_chars(s, |c| c == self, to, usize::MAX)
    }

    #[inline]
    fn replacen_from<'a>(self, s: &'a str, to: &str, count: usize) -> Cow<'a, str> {
        replace_chars(s, |c| c == self, to, count)
    }
}

impl Pattern for &str {
    #[inline]
    fn replace_from<'a>(self, s: &'a str, to: &str) -> Cow<'a, str> {
        replace_str(s, self, to)
    }

    #[inline]
    fn replacen_from<'a>(self, s: &'a str, to: &str, count: usize) -> Cow<'a, str> {
        replacen_str(s, self, to, count)
    }
}

impl Pattern for &&str {
    #[inline]
    fn replace_from<'a>(self, s: &'a str, to: &str) -> Cow<'a, str> {
        (*self).replace_from(s, to)
    }

    #[inline]
    fn replacen_from<'a>(self, s: &'a str, to: &str, count: usize) -> Cow<'a, str> {
        (*self).replacen_from(s, to, count)
    }
}

impl Pattern for &[char] {
    #[inline]
    fn replace_from<'a>(self, s: &'a str, to: &str) -> Cow<'a, str> {
        replace_chars(s, |c| self.iter().any(|from| c.eq(from)), to, usize::MAX)
    }

    #[inline]
    fn replacen_from<'a>(self, s: &'a str, to: &str, count: usize) -> Cow<'a, str> {
        replace_chars(s, |c| self.iter().any(|from| c.eq(from)), to, count)
    }
}

impl<const N: usize> Pattern for [char; N] {
    #[inline]
    fn replace_from<'a>(self, s: &'a str, to: &str) -> Cow<'a, str> {
        replace_chars(s, |c| self.iter().any(|from| c.eq(from)), to, usize::MAX)
    }

    #[inline]
    fn replacen_from<'a>(self, s: &'a str, to: &str, count: usize) -> Cow<'a, str> {
        replace_chars(s, |c| self.iter().any(|from| c.eq(from)), to, count)
    }
}

impl<const N: usize> Pattern for &[char; N] {
    #[inline]
    fn replace_from<'a>(self, s: &'a str, to: &str) -> Cow<'a, str> {
        replace_chars(s, |c| self.iter().any(|from| c.eq(from)), to, usize::MAX)
    }

    #[inline]
    fn replacen_from<'a>(self, s: &'a str, to: &str, count: usize) -> Cow<'a, str> {
        replace_chars(s, |c| self.iter().any(|from| c.eq(from)), to, count)
    }
}

impl<F> Pattern for F
where
    F: FnMut(char) -> bool,
{
    #[inline]
    fn replace_from<'a>(self, s: &'a str, to: &str) -> Cow<'a, str> {
        replace_chars(s, self, to, usize::MAX)
    }

    #[inline]
    fn replacen_from<'a>(self, s: &'a str, to: &str, count: usize) -> Cow<'a, str> {
        replace_chars(s, self, to, count)
    }
}
