/// To extend `[u8]` and `str` to have `starts_with_ignore_ascii_case_multiple`, `starts_with_ignore_ascii_case_with_lowercase_multiple` and `starts_with_ignore_ascii_case_with_uppercase_multiple` methods.
pub trait StartsWithIgnoreAsciiCaseMultiple {
    /// Returns `Some(usize)` if one of the given string slices case-insensitively (only ignoring ASCII case) matches a prefix of this string slice.
    fn starts_with_ignore_ascii_case_multiple<S: AsRef<[u8]>>(&self, bs: &[S]) -> Option<usize>;

    /// Returns `Some(usize)` if one of the given lowercase string slices case-insensitively (only ignoring ASCII case) matches a prefix of this string slice.
    fn starts_with_ignore_ascii_case_with_lowercase_multiple<S: AsRef<[u8]>>(
        &self,
        bs: &[S],
    ) -> Option<usize>;

    /// Returns `Some(usize)` if one of the given uppercase string slices case-insensitively (only ignoring ASCII case) matches a prefix of this string slice.
    fn starts_with_ignore_ascii_case_with_uppercase_multiple<S: AsRef<[u8]>>(
        &self,
        bs: &[S],
    ) -> Option<usize>;
}

impl StartsWithIgnoreAsciiCaseMultiple for [u8] {
    #[inline]
    fn starts_with_ignore_ascii_case_multiple<S: AsRef<[u8]>>(&self, bs: &[S]) -> Option<usize> {
        let a = self;
        let a_length = a.len();

        for (i, b) in bs.iter().enumerate() {
            let b = b.as_ref();

            let b_length = b.len();

            if a_length < b_length {
                continue;
            }

            if unsafe { a.get_unchecked(..b_length) }.eq_ignore_ascii_case(b) {
                return Some(i);
            }
        }

        None
    }

    #[inline]
    fn starts_with_ignore_ascii_case_with_lowercase_multiple<S: AsRef<[u8]>>(
        &self,
        bs: &[S],
    ) -> Option<usize> {
        let a = self;
        let a_length = a.len();

        for (i, b) in bs.iter().enumerate() {
            let b = b.as_ref();

            debug_assert!(!b.iter().any(|e| e.is_ascii_uppercase()));

            let b_length = b.len();

            if a_length < b_length {
                continue;
            }

            if !unsafe { a.get_unchecked(..b_length) }
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
    fn starts_with_ignore_ascii_case_with_uppercase_multiple<S: AsRef<[u8]>>(
        &self,
        bs: &[S],
    ) -> Option<usize> {
        let a = self;
        let a_length = a.len();

        for (i, b) in bs.iter().enumerate() {
            let b = b.as_ref();

            debug_assert!(!b.iter().any(|e| e.is_ascii_lowercase()));

            let b_length = b.len();

            if a_length < b_length {
                continue;
            }

            if !unsafe { a.get_unchecked(..b_length) }
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

impl StartsWithIgnoreAsciiCaseMultiple for str {
    #[inline]
    fn starts_with_ignore_ascii_case_multiple<S: AsRef<[u8]>>(&self, bs: &[S]) -> Option<usize> {
        self.as_bytes().starts_with_ignore_ascii_case_multiple(bs)
    }

    #[inline]
    fn starts_with_ignore_ascii_case_with_lowercase_multiple<S: AsRef<[u8]>>(
        &self,
        bs: &[S],
    ) -> Option<usize> {
        self.as_bytes().starts_with_ignore_ascii_case_with_lowercase_multiple(bs)
    }

    #[inline]
    fn starts_with_ignore_ascii_case_with_uppercase_multiple<S: AsRef<[u8]>>(
        &self,
        bs: &[S],
    ) -> Option<usize> {
        self.as_bytes().starts_with_ignore_ascii_case_with_uppercase_multiple(bs)
    }
}
