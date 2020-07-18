/// To extend types which implement `AsRef<[u8]>` to have a `eq_multiple` method.
pub trait EqMultiple {
    /// Returns `Some(usize)` if one of the given string slices case-sensitively matches this string slice.
    fn eq_multiple<S: AsRef<[u8]>>(&self, bs: &[S]) -> Option<usize>;
}

impl<T: AsRef<[u8]>> EqMultiple for T {
    #[inline]
    fn eq_multiple<S: AsRef<[u8]>>(&self, bs: &[S]) -> Option<usize> {
        let a = self.as_ref();

        bs.iter().position(|b| a == b.as_ref())
    }
}
