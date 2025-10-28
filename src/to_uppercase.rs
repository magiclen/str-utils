use alloc::{borrow::Cow, str};

/// To extend types which implement `AsRef<str>` to have `is_uppercase`, `is_ascii_uppercase`, `to_uppercase_cow` and `to_ascii_uppercase_cow` methods.
pub trait ToUppercase {
    /// Returns `true` if all characters in the string are uppercase according to Unicode.
    fn is_uppercase(&self) -> bool;

    /// Returns `true` if all characters in the string are ASCII uppercase.
    fn is_ascii_uppercase(&self) -> bool;

    /// Converts the string to its uppercase form (Unicode-aware), returning a `Cow<str>` to avoid allocation when possible.
    fn to_uppercase_cow(&self) -> Cow<'_, str>;

    /// Converts the string to its ASCII uppercase form, returning a `Cow<str>` to avoid allocation when possible.
    fn to_ascii_uppercase_cow(&self) -> Cow<'_, str>;
}

impl<T: AsRef<str>> ToUppercase for T {
    #[inline]
    fn is_uppercase(&self) -> bool {
        self.as_ref().chars().all(|c| c.is_uppercase())
    }

    #[inline]
    fn is_ascii_uppercase(&self) -> bool {
        self.as_ref().chars().all(|c| c.is_ascii_uppercase())
    }

    #[inline]
    fn to_uppercase_cow(&self) -> Cow<'_, str> {
        if self.is_uppercase() {
            Cow::from(self.as_ref())
        } else {
            Cow::from(self.as_ref().to_uppercase())
        }
    }

    #[inline]
    fn to_ascii_uppercase_cow(&self) -> Cow<'_, str> {
        if self.is_ascii_uppercase() {
            Cow::from(self.as_ref())
        } else {
            Cow::from(self.as_ref().to_ascii_uppercase())
        }
    }
}
