/// To extend types which implement `AsRef<[u8]>` to have a `ends_with_multiple` method.
pub trait EndsWithMultiple {
    /// Returns `Some(usize)` if one of the given string slices case-sensitively matches a suffix of this string slice.
    fn ends_with_multiple<S: AsRef<[u8]>>(&self, bs: &[S]) -> Option<usize>;
}

impl<T: AsRef<[u8]>> EndsWithMultiple for T {
    #[inline]
    fn ends_with_multiple<S: AsRef<[u8]>>(&self, bs: &[S]) -> Option<usize> {
        let a = self.as_ref();

        bs.iter().position(|b| a.ends_with(b.as_ref()))
    }
}
