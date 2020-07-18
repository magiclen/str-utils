/// To extend types which implement `AsRef<[u8]>` to have `starts_with_ignore_ascii_case`, `starts_with_ignore_ascii_case_with_lowercase` and `starts_with_ignore_ascii_case_with_uppercase` methods.
pub trait StartsWithIgnoreAsciiCase {
    /// Returns `true` if the given string slice case-insensitively (only ignoring ASCII case) matches a prefix of this string slice.
    fn starts_with_ignore_ascii_case<S: AsRef<[u8]>>(&self, b: S) -> bool;

    /// Returns `true` if the given lowercase string slice case-insensitively (only ignoring ASCII case) matches a prefix of this string slice.
    fn starts_with_ignore_ascii_case_with_lowercase<S: AsRef<[u8]>>(&self, b: S) -> bool;

    /// Returns `true` if the given uppercase string slice case-insensitively (only ignoring ASCII case) matches a prefix of this string slice.
    fn starts_with_ignore_ascii_case_with_uppercase<S: AsRef<[u8]>>(&self, b: S) -> bool;
}

impl<T: AsRef<[u8]>> StartsWithIgnoreAsciiCase for T {
    #[inline]
    fn starts_with_ignore_ascii_case<S: AsRef<[u8]>>(&self, b: S) -> bool {
        let b = b.as_ref();

        let b_length = b.len();

        if b_length == 0 {
            return true;
        }

        let a = self.as_ref();

        let a_length = a.len();

        if a_length >= b_length {
            unsafe { a.get_unchecked(..b_length) }.eq_ignore_ascii_case(b)
        } else {
            false
        }
    }

    #[inline]
    fn starts_with_ignore_ascii_case_with_lowercase<S: AsRef<[u8]>>(&self, b: S) -> bool {
        let b = b.as_ref();

        debug_assert!(!b.iter().any(|e| e.is_ascii_uppercase()));

        let b_length = b.len();

        if b_length == 0 {
            return true;
        }

        let a = self.as_ref();

        let a_length = a.len();

        if a_length >= b_length {
            !unsafe { a.get_unchecked(..b_length) }
                .iter()
                .map(|e| e.to_ascii_lowercase())
                .zip(b.iter().copied())
                .any(|(ac, bc)| ac != bc)
        } else {
            false
        }
    }

    #[inline]
    fn starts_with_ignore_ascii_case_with_uppercase<S: AsRef<[u8]>>(&self, b: S) -> bool {
        let b = b.as_ref();

        debug_assert!(!b.iter().any(|e| e.is_ascii_lowercase()));

        let b_length = b.len();

        if b_length == 0 {
            return true;
        }

        let a = self.as_ref();

        let a_length = a.len();

        if a_length >= b_length {
            !unsafe { a.get_unchecked(..b_length) }
                .iter()
                .map(|e| e.to_ascii_uppercase())
                .zip(b.iter().copied())
                .any(|(ac, bc)| ac != bc)
        } else {
            false
        }
    }
}
