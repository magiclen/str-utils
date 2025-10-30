/// To extend `[u8]` and `str` to have `ends_with_ignore_ascii_case`, `ends_with_ignore_ascii_case_with_lowercase` and `ends_with_ignore_ascii_case_with_uppercase` methods.
pub trait EndsWithIgnoreAsciiCase {
    /// Returns `true` if the given string slice case-insensitively (only ignoring ASCII case) matches a suffix of this string slice.
    fn ends_with_ignore_ascii_case<S: AsRef<[u8]>>(&self, b: S) -> bool;

    /// Returns `true` if the given lowercase string slice case-insensitively (only ignoring ASCII case) matches a suffix of this string slice.
    fn ends_with_ignore_ascii_case_with_lowercase<S: AsRef<[u8]>>(&self, b: S) -> bool;

    /// Returns `true` if the given uppercase string slice case-insensitively (only ignoring ASCII case) matches a suffix of this string slice.
    fn ends_with_ignore_ascii_case_with_uppercase<S: AsRef<[u8]>>(&self, b: S) -> bool;
}

impl EndsWithIgnoreAsciiCase for [u8] {
    #[inline]
    fn ends_with_ignore_ascii_case<S: AsRef<[u8]>>(&self, b: S) -> bool {
        let b = b.as_ref();

        let b_length = b.len();

        if b_length == 0 {
            return true;
        }

        let a = self;

        let a_length = a.len();

        if a_length >= b_length {
            unsafe { a.get_unchecked((a_length - b_length)..) }.eq_ignore_ascii_case(b)
        } else {
            false
        }
    }

    #[inline]
    fn ends_with_ignore_ascii_case_with_lowercase<S: AsRef<[u8]>>(&self, b: S) -> bool {
        let b = b.as_ref();

        debug_assert!(!b.iter().any(|e| e.is_ascii_uppercase()));

        let b_length = b.len();

        if b_length == 0 {
            return true;
        }

        let a = self;

        let a_length = a.len();

        if a_length >= b_length {
            !unsafe { a.get_unchecked((a_length - b_length)..) }
                .iter()
                .map(|e| e.to_ascii_lowercase())
                .zip(b.iter().copied())
                .any(|(ac, bc)| ac != bc)
        } else {
            false
        }
    }

    #[inline]
    fn ends_with_ignore_ascii_case_with_uppercase<S: AsRef<[u8]>>(&self, b: S) -> bool {
        let b = b.as_ref();

        debug_assert!(!b.iter().any(|e| e.is_ascii_lowercase()));

        let b_length = b.len();

        if b_length == 0 {
            return true;
        }

        let a = self;

        let a_length = a.len();

        if a_length >= b_length {
            !unsafe { a.get_unchecked((a_length - b_length)..) }
                .iter()
                .map(|e| e.to_ascii_uppercase())
                .zip(b.iter().copied())
                .any(|(ac, bc)| ac != bc)
        } else {
            false
        }
    }
}

impl EndsWithIgnoreAsciiCase for str {
    #[inline]
    fn ends_with_ignore_ascii_case<S: AsRef<[u8]>>(&self, b: S) -> bool {
        self.as_bytes().ends_with_ignore_ascii_case(b)
    }

    #[inline]
    fn ends_with_ignore_ascii_case_with_lowercase<S: AsRef<[u8]>>(&self, b: S) -> bool {
        self.as_bytes().ends_with_ignore_ascii_case_with_lowercase(b)
    }

    #[inline]
    fn ends_with_ignore_ascii_case_with_uppercase<S: AsRef<[u8]>>(&self, b: S) -> bool {
        self.as_bytes().ends_with_ignore_ascii_case_with_uppercase(b)
    }
}
