/// To extend `[u8]` and `str` to have `is_ascii_lowercased` method.
pub trait IsAsciiLowercased {
    /// Returns `true` if all characters in the string are not uppercase according to Unicode.
    fn is_ascii_lowercased(&self) -> bool;
}

impl IsAsciiLowercased for [u8] {
    #[inline]
    fn is_ascii_lowercased(&self) -> bool {
        self.iter().all(|c| !c.is_ascii_uppercase())
    }
}

impl IsAsciiLowercased for str {
    #[inline]
    fn is_ascii_lowercased(&self) -> bool {
        self.as_bytes().is_ascii_lowercased()
    }
}
