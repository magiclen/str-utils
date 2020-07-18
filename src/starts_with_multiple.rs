/// To extend types which implement `AsRef<[u8]>` to have a `starts_with_multiple` method.
pub trait StartsWithMultiple {
    /// Returns `Some(usize)` if one of the given string slices case-sensitively matches a prefix of this string slice.
    fn starts_with_multiple<S: AsRef<[u8]>>(&self, bs: &[S]) -> Option<usize>;
}

impl<T: AsRef<[u8]>> StartsWithMultiple for T {
    #[inline]
    fn starts_with_multiple<S: AsRef<[u8]>>(&self, bs: &[S]) -> Option<usize> {
        let a = self.as_ref();

        bs.iter().position(|b| a.starts_with(b.as_ref()))
    }
}
