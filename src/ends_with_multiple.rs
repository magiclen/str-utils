/// To extend `[u8]` and `str` to have a `ends_with_multiple` method.
pub trait EndsWithMultiple {
    /// Returns `Some(usize)` if one of the given string slices case-sensitively matches a suffix of this string slice.
    fn ends_with_multiple<S: AsRef<[u8]>>(&self, bs: &[S]) -> Option<usize>;
}

impl EndsWithMultiple for [u8] {
    #[inline]
    fn ends_with_multiple<S: AsRef<[u8]>>(&self, bs: &[S]) -> Option<usize> {
        let a = self;

        bs.iter().position(|b| a.ends_with(b.as_ref()))
    }
}

impl EndsWithMultiple for str {
    #[inline]
    fn ends_with_multiple<S: AsRef<[u8]>>(&self, bs: &[S]) -> Option<usize> {
        self.as_bytes().ends_with_multiple(bs)
    }
}
