/// To extend `[u8]` and `str` to have `is_ascii_uppercased` method.
pub trait IsAsciiUppercased {
    /// Returns `true` if all characters in the string are not lowercase according to Unicode.
    fn is_ascii_uppercased(&self) -> bool;
}

impl IsAsciiUppercased for [u8] {
    #[inline]
    fn is_ascii_uppercased(&self) -> bool {
        self.iter().all(|c| !c.is_ascii_lowercase())
    }
}

impl IsAsciiUppercased for str {
    #[inline]
    fn is_ascii_uppercased(&self) -> bool {
        self.as_bytes().is_ascii_uppercased()
    }
}
