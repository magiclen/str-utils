/// To extend `[u8]` and `str` to have a `starts_with_multiple` method.
pub trait StartsWithMultiple {
    /// Returns `Some(usize)` if one of the given string slices case-sensitively matches a prefix of this string slice.
    fn starts_with_multiple<S: AsRef<[u8]>>(&self, bs: &[S]) -> Option<usize>;
}

impl StartsWithMultiple for [u8] {
    #[inline]
    fn starts_with_multiple<S: AsRef<[u8]>>(&self, bs: &[S]) -> Option<usize> {
        let a = self;

        bs.iter().position(|b| a.starts_with(b.as_ref()))
    }
}

impl StartsWithMultiple for str {
    #[inline]
    fn starts_with_multiple<S: AsRef<[u8]>>(&self, bs: &[S]) -> Option<usize> {
        self.as_bytes().starts_with_multiple(bs)
    }
}
