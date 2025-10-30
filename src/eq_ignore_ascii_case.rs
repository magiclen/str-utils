/// To extend `[u8]` and `str` to have `eq_ignore_ascii_case_with_lowercase` and `eq_ignore_ascii_case_with_uppercase` methods.
///
/// These methods are slightly faster than `eq_ignore_ascii_case`.
pub trait EqIgnoreAsciiCase {
    /// Returns `true` if the given lowercase string slice case-insensitively (only ignoring ASCII case) matches this string slice.
    fn eq_ignore_ascii_case_with_lowercase<S: AsRef<[u8]>>(&self, b: S) -> bool;
    /// Returns `true` if the given uppercase string slice case-insensitively (only ignoring ASCII case) matches this string slice.
    fn eq_ignore_ascii_case_with_uppercase<S: AsRef<[u8]>>(&self, b: S) -> bool;
}

impl EqIgnoreAsciiCase for [u8] {
    #[inline]
    fn eq_ignore_ascii_case_with_lowercase<S: AsRef<[u8]>>(&self, b: S) -> bool {
        let b = b.as_ref();

        debug_assert!(!b.iter().any(|e| e.is_ascii_uppercase()));

        let a = self;

        if a.len() != b.len() {
            return false;
        }

        !a.iter().map(|e| e.to_ascii_lowercase()).zip(b.iter().copied()).any(|(ac, bc)| ac != bc)
    }

    #[inline]
    fn eq_ignore_ascii_case_with_uppercase<S: AsRef<[u8]>>(&self, b: S) -> bool {
        let b = b.as_ref();

        debug_assert!(!b.iter().any(|e| e.is_ascii_lowercase()));

        let a = self;

        if a.len() != b.len() {
            return false;
        }

        !a.iter().map(|e| e.to_ascii_uppercase()).zip(b.iter().copied()).any(|(ac, bc)| ac != bc)
    }
}

impl EqIgnoreAsciiCase for str {
    #[inline]
    fn eq_ignore_ascii_case_with_lowercase<S: AsRef<[u8]>>(&self, b: S) -> bool {
        self.as_bytes().eq_ignore_ascii_case_with_lowercase(b)
    }

    #[inline]
    fn eq_ignore_ascii_case_with_uppercase<S: AsRef<[u8]>>(&self, b: S) -> bool {
        self.as_bytes().eq_ignore_ascii_case_with_uppercase(b)
    }
}
