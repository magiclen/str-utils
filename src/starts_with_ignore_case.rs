use crate::{ToLowercase, ToUppercase};

/// To extend `str` to have a `starts_with_ignore_case` method.
pub trait StartsWithIgnoreCase {
    /// Returns `true` if the given string slice case-insensitively (using case-folding) matches a prefix of this string slice.
    ///
    /// NOTE: This method may allocate heap memory.
    fn starts_with_ignore_case<S: AsRef<str>>(&self, b: S) -> bool;
}

impl StartsWithIgnoreCase for str {
    #[inline]
    fn starts_with_ignore_case<S: AsRef<str>>(&self, b: S) -> bool {
        let a = self;
        let b = b.as_ref();

        if b.is_empty() {
            return true;
        }

        {
            let au = a.to_uppercase_cow();
            let bu = b.to_uppercase_cow();

            if au.as_bytes().starts_with(bu.as_bytes()) {
                return true;
            }
        }

        let al = a.to_lowercase_cow();
        let bl = b.to_lowercase_cow();

        al.as_bytes().starts_with(bl.as_bytes())
    }
}
