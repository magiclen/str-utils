/// To extend `[u8]` and `str` to have `eq_ignore_ascii_case_multiple`, `eq_ignore_ascii_case_with_lowercase_multiple` and `eq_ignore_ascii_case_with_uppercase_multiple` methods.
pub trait EqIgnoreAsciiCaseMultiple {
    /// Returns `Some(usize)` if one of the given string slices case-insensitively matches this string slice.
    fn eq_ignore_ascii_case_multiple<S: AsRef<[u8]>>(&self, bs: &[S]) -> Option<usize>;

    /// Returns `Some(usize)` if one of the given lowercase string slices case-insensitively matches this string slice.
    fn eq_ignore_ascii_case_with_lowercase_multiple<S: AsRef<[u8]>>(
        &self,
        bs: &[S],
    ) -> Option<usize>;

    /// Returns `Some(usize)` if one of the given uppercase string slices case-insensitively matches this string slice.
    fn eq_ignore_ascii_case_with_uppercase_multiple<S: AsRef<[u8]>>(
        &self,
        bs: &[S],
    ) -> Option<usize>;
}

impl EqIgnoreAsciiCaseMultiple for [u8] {
    #[inline]
    fn eq_ignore_ascii_case_multiple<S: AsRef<[u8]>>(&self, bs: &[S]) -> Option<usize> {
        let a = self;

        bs.iter().position(|b| a.eq_ignore_ascii_case(b.as_ref()))
    }

    #[inline]
    fn eq_ignore_ascii_case_with_lowercase_multiple<S: AsRef<[u8]>>(
        &self,
        bs: &[S],
    ) -> Option<usize> {
        let a = self;
        let a_length = a.len();

        for (i, b) in bs.iter().enumerate() {
            let b = b.as_ref();

            debug_assert!(!b.iter().any(|e| e.is_ascii_uppercase()));

            if a_length != b.len() {
                continue;
            }

            if !a
                .iter()
                .map(|e| e.to_ascii_lowercase())
                .zip(b.iter().copied())
                .any(|(ac, bc)| ac != bc)
            {
                return Some(i);
            }
        }

        None
    }

    #[inline]
    fn eq_ignore_ascii_case_with_uppercase_multiple<S: AsRef<[u8]>>(
        &self,
        bs: &[S],
    ) -> Option<usize> {
        let a = self;
        let a_length = a.len();

        for (i, b) in bs.iter().enumerate() {
            let b = b.as_ref();

            debug_assert!(!b.iter().any(|e| e.is_ascii_lowercase()));

            if a_length != b.len() {
                continue;
            }

            if !a
                .iter()
                .map(|e| e.to_ascii_uppercase())
                .zip(b.iter().copied())
                .any(|(ac, bc)| ac != bc)
            {
                return Some(i);
            }
        }

        None
    }
}

impl EqIgnoreAsciiCaseMultiple for str {
    #[inline]
    fn eq_ignore_ascii_case_multiple<S: AsRef<[u8]>>(&self, bs: &[S]) -> Option<usize> {
        self.as_bytes().eq_ignore_ascii_case_multiple(bs)
    }

    #[inline]
    fn eq_ignore_ascii_case_with_lowercase_multiple<S: AsRef<[u8]>>(
        &self,
        bs: &[S],
    ) -> Option<usize> {
        self.as_bytes().eq_ignore_ascii_case_with_lowercase_multiple(bs)
    }

    #[inline]
    fn eq_ignore_ascii_case_with_uppercase_multiple<S: AsRef<[u8]>>(
        &self,
        bs: &[S],
    ) -> Option<usize> {
        self.as_bytes().eq_ignore_ascii_case_with_uppercase_multiple(bs)
    }
}
