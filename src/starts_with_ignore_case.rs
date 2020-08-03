extern crate cow_utils;

use cow_utils::CowUtils;

/// To extend types which implement `AsRef<str>` to have a `starts_with_ignore_case` method.
pub trait StartsWithIgnoreCase {
    /// Returns `true` if the given string slice case-insensitively (using case-folding) matches a prefix of this string slice.
    ///
    /// NOTE: This method may allocate heap memory.
    fn starts_with_ignore_case<S: AsRef<str>>(&self, b: S) -> bool;
}

impl<T: AsRef<str>> StartsWithIgnoreCase for T {
    #[inline]
    fn starts_with_ignore_case<S: AsRef<str>>(&self, b: S) -> bool {
        let a = self.as_ref();
        let b = b.as_ref();

        if b.is_empty() {
            return true;
        }

        {
            let au = a.cow_to_uppercase();
            let bu = b.cow_to_uppercase();

            let au_length = au.len();
            let bu_length = bu.len();

            if au_length >= bu_length && unsafe { au.get_unchecked(..bu_length) == bu } {
                return true;
            }
        }

        let al = a.cow_to_lowercase();
        let bl = b.cow_to_lowercase();

        let al_length = al.len();
        let bl_length = bl.len();

        if al_length >= bl_length {
            unsafe { al.get_unchecked(..bl_length) == bl }
        } else {
            false
        }
    }
}
