use alloc::{borrow::Cow, str, vec::Vec};

/// To extend `str` and `Cow<str>` to have `escape_characters` and `escape_ascii_characters` method.
///
/// Typical use cases include preparing strings for SQL `LIKE` queries or other contexts where certain characters need to be escaped.
pub trait EscapeCharacters<'a> {
    /// Escapes all occurrences of the specified characters within a string.
    ///
    /// This function scans the input string for any character that matches one of
    /// the `escaped_characters` or the `escape_character` itself, and prefixes
    /// those characters with the provided `escape_character`.
    fn escape_characters(self, escape_character: char, escaped_characters: &[char])
        -> Cow<'a, str>;

    /// Escapes ASCII characters within a UTF-8 string.
    ///
    /// Similar to [`EscapeCharacters::escape_characters`], but operates directly on bytes instead of Unicode scalar values.
    /// This version is optimized for ASCII-only escaping and avoids unnecessary Unicode conversions.
    fn escape_ascii_characters(
        self,
        escape_character: u8,
        escaped_characters: &[u8],
    ) -> Cow<'a, str>;
}

impl<'a> EscapeCharacters<'a> for &'a str {
    fn escape_characters(
        self,
        escape_character: char,
        escaped_characters: &[char],
    ) -> Cow<'a, str> {
        let s = self;

        if escaped_characters.is_empty() {
            return Cow::Borrowed(s);
        }

        let mut p = 0;

        let mut chars = s.chars();

        let need_escape = |c: char| {
            c == escape_character
                || escaped_characters.iter().any(|escaped_character| c.eq(escaped_character))
        };

        let first_c = loop {
            let c = if let Some(c) = chars.next() {
                c
            } else {
                return Cow::Borrowed(s);
            };

            if need_escape(c) {
                break c;
            }

            p += c.len_utf8();
        };

        let mut new_s = String::with_capacity(s.len() + 1);

        new_s.push_str(unsafe { str::from_utf8_unchecked(&s.as_bytes()[0..p]) });
        new_s.push(escape_character);
        new_s.push(first_c);

        for c in chars {
            if need_escape(c) {
                new_s.push(escape_character);
            }

            new_s.push(c);
        }

        Cow::Owned(new_s)
    }

    fn escape_ascii_characters(
        self,
        escape_character: u8,
        escaped_characters: &[u8],
    ) -> Cow<'a, str> {
        let s = self;

        if escaped_characters.is_empty() {
            return Cow::Borrowed(s);
        }

        let bytes = s.as_bytes();

        let length = bytes.len();

        let mut p = 0;

        let need_escape = |b: u8| {
            b == escape_character
                || escaped_characters.iter().any(|escaped_character| b.eq(escaped_character))
        };

        loop {
            if p == length {
                return Cow::Borrowed(s);
            }

            let e = bytes[p];

            let width = unsafe { utf8_width::get_width_assume_valid(e) };

            if width == 1 && need_escape(e) {
                break;
            }

            p += width;
        }

        let mut new_v = Vec::with_capacity(bytes.len() + 1);

        new_v.extend_from_slice(&bytes[..p]);
        new_v.push(escape_character);

        let mut start = p;

        p += 1;

        loop {
            if p == length {
                break;
            }

            let e = bytes[p];

            let width = unsafe { utf8_width::get_width_assume_valid(e) };

            if width == 1 && need_escape(e) {
                new_v.extend_from_slice(&bytes[start..p]);
                start = p + 1;

                new_v.push(escape_character);
                new_v.push(e);
            }

            p += width;
        }

        new_v.extend_from_slice(&bytes[start..p]);

        Cow::Owned(unsafe { String::from_utf8_unchecked(new_v) })
    }
}

impl<'a> EscapeCharacters<'a> for Cow<'a, str> {
    #[inline]
    fn escape_characters(
        self,
        escape_character: char,
        escaped_characters: &[char],
    ) -> Cow<'a, str> {
        match self {
            Cow::Borrowed(s) => s.escape_characters(escape_character, escaped_characters),
            Cow::Owned(s) => {
                match s.escape_characters(escape_character, escaped_characters) {
                    Cow::Borrowed(_) => {
                        // it changes nothing
                        // if there were any characters that needed to be escaped, it had to be `Cow::Owned`
                        Cow::Owned(s)
                    },
                    Cow::Owned(s) => Cow::Owned(s),
                }
            },
        }
    }

    #[inline]
    fn escape_ascii_characters(
        self,
        escape_character: u8,
        escaped_characters: &[u8],
    ) -> Cow<'a, str> {
        match self {
            Cow::Borrowed(s) => s.escape_ascii_characters(escape_character, escaped_characters),
            Cow::Owned(s) => {
                match s.escape_ascii_characters(escape_character, escaped_characters) {
                    Cow::Borrowed(_) => {
                        // it changes nothing
                        // if there were any characters that needed to be escaped, it had to be `Cow::Owned`
                        Cow::Owned(s)
                    },
                    Cow::Owned(s) => Cow::Owned(s),
                }
            },
        }
    }
}
