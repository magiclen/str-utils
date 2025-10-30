/// To extend `str` to have `is_uppercased` method.
pub trait IsUppercased {
    /// Returns `true` if all characters in the string are not lowercase according to Unicode.
    fn is_uppercased(&self) -> bool;
}

impl IsUppercased for str {
    #[inline]
    fn is_uppercased(&self) -> bool {
        self.chars().all(|c| !c.is_lowercase())
    }
}
