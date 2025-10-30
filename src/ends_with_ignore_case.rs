use crate::{ToLowercase, ToUppercase};

/// To extend `str` to have a `ends_with_ignore_case` method.
pub trait EndsWithIgnoreCase {
    /// Returns `true` if the given string slice case-insensitively (using case-folding) matches a suffix of this string slice.
    ///
    /// NOTE: This method may allocate heap memory.
    fn ends_with_ignore_case<S: AsRef<str>>(&self, b: S) -> bool;
}

impl EndsWithIgnoreCase for str {
    #[inline]
    fn ends_with_ignore_case<S: AsRef<str>>(&self, b: S) -> bool {
        let a = self;
        let b = b.as_ref();

        if b.is_empty() {
            return true;
        }

        {
            let au = a.to_uppercase_cow();
            let bu = b.to_uppercase_cow();

            let au_length = au.len();
            let bu_length = bu.len();

            if au_length >= bu_length
                && unsafe { au.get_unchecked((au_length - bu_length)..) == bu }
            {
                return true;
            }
        }

        let al = a.to_lowercase_cow();
        let bl = b.to_lowercase_cow();

        let al_length = al.len();
        let bl_length = bl.len();

        if al_length >= bl_length {
            unsafe { al.get_unchecked((al_length - bl_length)..) == bl }
        } else {
            false
        }
    }
}
