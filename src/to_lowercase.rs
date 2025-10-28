use alloc::{borrow::Cow, str};

/// To extend types which implement `AsRef<str>` to have `is_lowercase`, `is_ascii_lowercase`, `to_lowercase_cow` and `to_ascii_lowercase_cow` methods.
pub trait ToLowercase {
    /// Returns `true` if all characters in the string are lowercase according to Unicode.
    fn is_lowercase(&self) -> bool;

    /// Returns `true` if all characters in the string are ASCII lowercase.
    fn is_ascii_lowercase(&self) -> bool;

    /// Converts the string to its lowercase form (Unicode-aware), returning a `Cow<str>` to avoid allocation when possible.
    fn to_lowercase_cow(&self) -> Cow<'_, str>;

    /// Converts the string to its ASCII lowercase form, returning a `Cow<str>` to avoid allocation when possible.
    fn to_ascii_lowercase_cow(&self) -> Cow<'_, str>;
}

impl<T: AsRef<str>> ToLowercase for T {
    #[inline]
    fn is_lowercase(&self) -> bool {
        self.as_ref().chars().all(|c| c.is_lowercase())
    }

    #[inline]
    fn is_ascii_lowercase(&self) -> bool {
        self.as_ref().chars().all(|c| c.is_ascii_lowercase())
    }

    #[inline]
    fn to_lowercase_cow(&self) -> Cow<'_, str> {
        if self.is_lowercase() {
            Cow::from(self.as_ref())
        } else {
            Cow::from(self.as_ref().to_lowercase())
        }
    }

    #[inline]
    fn to_ascii_lowercase_cow(&self) -> Cow<'_, str> {
        if self.is_ascii_lowercase() {
            Cow::from(self.as_ref())
        } else {
            Cow::from(self.as_ref().to_ascii_lowercase())
        }
    }
}
