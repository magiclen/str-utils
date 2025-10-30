/// To extend `[u8]` and `str` to have a `eq_multiple` method.
pub trait EqMultiple {
    /// Returns `Some(usize)` if one of the given string slices case-sensitively matches this string slice.
    fn eq_multiple<S: AsRef<[u8]>>(&self, bs: &[S]) -> Option<usize>;
}

impl EqMultiple for [u8] {
    #[inline]
    fn eq_multiple<S: AsRef<[u8]>>(&self, bs: &[S]) -> Option<usize> {
        let a = self;

        bs.iter().position(|b| a == b.as_ref())
    }
}

impl EqMultiple for str {
    #[inline]
    fn eq_multiple<S: AsRef<[u8]>>(&self, bs: &[S]) -> Option<usize> {
        self.as_bytes().eq_multiple(bs)
    }
}
