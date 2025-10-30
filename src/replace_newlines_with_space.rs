use alloc::{borrow::Cow, vec::Vec};

/// To extend `str` and `Cow<str>` to have `replace_newlines_with_space` method.
///
/// This can be useful when you need to normalize multiline strings into a single line for logging, database storage, or display purposes.
pub trait ReplaceNewlinesWithSpace<'a> {
    /// Returns a `Cow<str>` where all newline sequences are replaced with a space.
    ///
    /// Replaces Windows-style newlines (`\r\n`), old Mac-style (`\r`), and Unix-style (`\n`).
    fn replace_newlines_with_space(self) -> Cow<'a, str>;
}

impl<'a> ReplaceNewlinesWithSpace<'a> for &'a str {
    fn replace_newlines_with_space(self) -> Cow<'a, str> {
        let s = self;
        let bytes = s.as_bytes();
        let length = bytes.len();

        let mut p = 0;

        let first_len = loop {
            if p == length {
                return Cow::from(s);
            }

            let e = bytes[p];

            match e {
                b'\r' => {
                    if p < length - 1 && bytes[p + 1] == b'\n' {
                        break 2; // CRLF
                    } else {
                        break 1; // CR
                    }
                },
                b'\n' => {
                    break 1; // LF
                },
                _ => (),
            }

            p += 1;
        };

        let mut new_v = Vec::with_capacity(bytes.len());

        new_v.extend_from_slice(&bytes[..p]);
        new_v.push(b' ');

        p += first_len;

        let mut start = p;

        loop {
            if p == length {
                break;
            }

            let e = bytes[p];

            match e {
                b'\r' => {
                    new_v.extend_from_slice(&bytes[start..p]);

                    if p < length - 1 && bytes[p + 1] == b'\n' {
                        // CRLF
                        p += 1;
                        start = p + 1;
                    } else {
                        // CR
                        start = p + 1;
                    }

                    new_v.push(b' ');
                },
                b'\n' => {
                    // LF
                    new_v.extend_from_slice(&bytes[start..p]);
                    start = p + 1;

                    new_v.push(b' ');
                },
                _ => (),
            }

            p += 1;
        }

        new_v.extend_from_slice(&bytes[start..p]);

        Cow::from(unsafe { String::from_utf8_unchecked(new_v) })
    }
}

impl<'a> ReplaceNewlinesWithSpace<'a> for Cow<'a, str> {
    #[inline]
    fn replace_newlines_with_space(self) -> Cow<'a, str> {
        match self {
            Cow::Borrowed(s) => s.replace_newlines_with_space(),
            Cow::Owned(s) => {
                match s.replace_newlines_with_space() {
                    Cow::Borrowed(_) => {
                        // it changes nothing
                        // if there were any characters that needed to be replaced, it had to be `Cow::Owned`
                        Cow::Owned(s)
                    },
                    Cow::Owned(s) => Cow::Owned(s),
                }
            },
        }
    }
}
