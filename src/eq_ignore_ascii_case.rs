/// To extend types which implement `AsRef<[u8]>` to have `eq_ignore_ascii_case_with_lowercase` and `eq_ignore_ascii_case_with_uppercase` methods.
///
/// These methods are a little faster than `eq_ignore_ascii_case`.
pub trait EqIgnoreAsciiCase {
    /// Returns `true` if the given lowercase string slice case-insensitively (only ignoring ASCII case) matches this string slice.
    fn eq_ignore_ascii_case_with_lowercase<S: AsRef<[u8]>>(&self, b: S) -> bool;
    /// Returns `true` if the given uppercase string slice case-insensitively (only ignoring ASCII case) matches this string slice.
    fn eq_ignore_ascii_case_with_uppercase<S: AsRef<[u8]>>(&self, b: S) -> bool;
}

impl<T: AsRef<[u8]>> EqIgnoreAsciiCase for T {
    #[inline]
    fn eq_ignore_ascii_case_with_lowercase<S: AsRef<[u8]>>(&self, b: S) -> bool {
        let b = b.as_ref();

        debug_assert!(!b.iter().any(|e| e.is_ascii_uppercase()));

        let a = self.as_ref();

        if a.len() != b.len() {
            return false;
        }

        !a.iter().map(|e| e.to_ascii_lowercase()).zip(b.iter().copied()).any(|(ac, bc)| ac != bc)
    }

    #[inline]
    fn eq_ignore_ascii_case_with_uppercase<S: AsRef<[u8]>>(&self, b: S) -> bool {
        let b = b.as_ref();

        debug_assert!(!b.iter().any(|e| e.is_ascii_lowercase()));

        let a = self.as_ref();

        if a.len() != b.len() {
            return false;
        }

        !a.iter().map(|e| e.to_ascii_uppercase()).zip(b.iter().copied()).any(|(ac, bc)| ac != bc)
    }
}
