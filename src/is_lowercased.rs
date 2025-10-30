/// To extend `str` to have `is_lowercased` method.
pub trait IsLowercased {
    /// Returns `true` if all characters in the string are not uppercase according to Unicode.
    fn is_lowercased(&self) -> bool;
}

impl IsLowercased for str {
    #[inline]
    fn is_lowercased(&self) -> bool {
        self.chars().all(|c| !c.is_uppercase())
    }
}
