/// To extend `[u8]` and `str` to have `eq_ignore_ascii_case_with_lowercase` and `eq_ignore_ascii_case_with_uppercase` methods.
///
/// Deprecated: use the standard library's `eq_ignore_ascii_case` instead.
#[deprecated(note = "use [u8]::eq_ignore_ascii_case or str::eq_ignore_ascii_case instead; this \
                     API is not faster than the standard library now")]
pub trait EqIgnoreAsciiCase {
    /// Returns `true` if the given lowercase string slice case-insensitively (only ignoring ASCII case) matches this string slice.
    #[deprecated(note = "use [u8]::eq_ignore_ascii_case or str::eq_ignore_ascii_case instead; \
                         this API is not faster than the standard library now")]
    fn eq_ignore_ascii_case_with_lowercase<S: AsRef<[u8]>>(&self, b: S) -> bool;
    /// Returns `true` if the given uppercase string slice case-insensitively (only ignoring ASCII case) matches this string slice.
    #[deprecated(note = "use [u8]::eq_ignore_ascii_case or str::eq_ignore_ascii_case instead; \
                         this API is not faster than the standard library now")]
    fn eq_ignore_ascii_case_with_uppercase<S: AsRef<[u8]>>(&self, b: S) -> bool;
}

#[allow(deprecated)]
impl EqIgnoreAsciiCase for [u8] {
    #[inline]
    fn eq_ignore_ascii_case_with_lowercase<S: AsRef<[u8]>>(&self, b: S) -> bool {
        let b = b.as_ref();

        debug_assert!(!b.iter().any(|e| e.is_ascii_uppercase()));

        self.eq_ignore_ascii_case(b)
    }

    #[inline]
    fn eq_ignore_ascii_case_with_uppercase<S: AsRef<[u8]>>(&self, b: S) -> bool {
        let b = b.as_ref();

        debug_assert!(!b.iter().any(|e| e.is_ascii_lowercase()));

        self.eq_ignore_ascii_case(b)
    }
}

#[allow(deprecated)]
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
