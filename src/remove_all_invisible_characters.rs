use alloc::{borrow::Cow, str};

use crate::to_substring_in_place;

/// To extend `str` and `Cow<str>` to have `remove_all_invisible_characters` method.
pub trait RemoveInvisibleCharacters<'a> {
    /// Removes all invisible or non-printable characters from a given string.
    ///
    /// This function filters out a comprehensive set of Unicode characters that are typically
    /// invisible or used for control or formatting purposes. This includes:
    ///
    /// - ASCII control characters (U+0000 to U+001F and U+007F), exclude `\t` and `\n`
    /// - Zero-width characters and format controls:
    ///   - U+200B (Zero Width Space)
    ///   - U+200C (Zero Width Non-Joiner)
    ///   - U+200D (Zero Width Joiner)
    ///   - U+200E to U+200F (Directional marks)
    ///   - U+202A to U+202E (Directional formatting)
    ///   - U+2060 to U+2064 (Word Joiner and Invisible Math Symbols)
    ///   - U+2066 to U+2069 (Bidi Isolates)
    ///   - U+FEFF (Byte Order Mark / Zero Width No-Break Space)
    ///
    /// These characters can interfere with text rendering, parsing, and display,
    /// and are often used in text-based attacks (e.g., for spoofing).
    fn remove_all_invisible_characters(self) -> Cow<'a, str>;
}

impl<'a> RemoveInvisibleCharacters<'a> for &'a str {
    fn remove_all_invisible_characters(self) -> Cow<'a, str> {
        let s = self;
        let bytes = s.as_bytes();

        let length = bytes.len();

        let mut p = 0;

        let check_character_whether_to_remove = |p: usize, e: u8, width: usize| -> bool {
            match width {
                1 => {
                    match e {
                        // ascii controls, only remain \t, \n
                        0..=8 | 11..=13 | 14..=31 | 127 => return true,
                        _ => (),
                    }
                },
                3 => match e {
                    0xE2 => match bytes[p + 1] {
                        // zero width characters and bidirectional controls
                        0x80 => match bytes[p + 2] {
                            0x8B..=0x8F | 0xAA..=0xAE => return true,
                            _ => (),
                        },
                        // word joiner, invisible times/separator/plus and isolate characters
                        0x81 => match bytes[p + 2] {
                            0xA0 | 0xA2..=0xA4 | 0xA6..=0xA9 => return true,
                            _ => (),
                        },
                        _ => (),
                    },
                    // zero width character
                    0xEF => {
                        if bytes[p + 1] == 0xBB && bytes[p + 2] == 0xBF {
                            return true;
                        }
                    },
                    _ => (),
                },
                _ => (),
            }

            false
        };

        let width = loop {
            if p == length {
                return Cow::from(s);
            }

            let e = bytes[p];

            let width = unsafe { utf8_width::get_width_assume_valid(e) };

            if check_character_whether_to_remove(p, e, width) {
                break width;
            } else {
                p += width;
            }
        };

        let heading_normal_characters_end_index = p;

        p += width;

        // there are four situations which can use a string slice:
        // 1. <invisible_characters>
        // 2. <normal_characters><invisible_characters>
        // 3. <invisible_characters><normal_characters>
        // 4. <invisible_characters><normal_characters><invisible_characters>

        // continue to find more invisible characters
        let width = loop {
            if p == length {
                // situation 1 or situation 2

                return Cow::from(unsafe {
                    str::from_utf8_unchecked(&bytes[..heading_normal_characters_end_index])
                });
            }

            let e = bytes[p];

            let width = unsafe { utf8_width::get_width_assume_valid(e) };

            if check_character_whether_to_remove(p, e, width) {
                p += width;
            } else {
                break width;
            }
        };

        let following_invisible_characters_end_index = p;

        p += width;

        // continue to find more normal characters
        let width = loop {
            if p == length {
                // situation 3

                return Cow::from(unsafe {
                    str::from_utf8_unchecked(&bytes[following_invisible_characters_end_index..])
                });
            }

            let e = bytes[p];

            let width = unsafe { utf8_width::get_width_assume_valid(e) };

            if check_character_whether_to_remove(p, e, width) {
                break width;
            } else {
                p += width;
            }
        };

        let following_normal_characters_end_index = p;

        p += width;

        // continue to find more invisible characters
        let width = loop {
            if p == length {
                // situation 4

                return Cow::from(unsafe {
                    str::from_utf8_unchecked(
                        &bytes[following_invisible_characters_end_index
                            ..following_normal_characters_end_index],
                    )
                });
            }

            let e = bytes[p];

            let width = unsafe { utf8_width::get_width_assume_valid(e) };

            if check_character_whether_to_remove(p, e, width) {
                p += width;
            } else {
                break width;
            }
        };

        let mut new_v = bytes
            [following_invisible_characters_end_index..following_normal_characters_end_index]
            .to_vec();

        let mut start = p;

        p += width;

        loop {
            if p == length {
                break;
            }

            let e = bytes[p];

            let width = unsafe { utf8_width::get_width_assume_valid(e) };

            if check_character_whether_to_remove(p, e, width) {
                new_v.extend_from_slice(&bytes[start..p]);

                start = p + width;
            }

            p += width;
        }

        new_v.extend_from_slice(&bytes[start..p]);

        Cow::from(unsafe { String::from_utf8_unchecked(new_v) })
    }
}

impl<'a> RemoveInvisibleCharacters<'a> for Cow<'a, str> {
    #[inline]
    fn remove_all_invisible_characters(self) -> Cow<'a, str> {
        match self {
            Cow::Borrowed(s) => s.remove_all_invisible_characters(),
            Cow::Owned(s) => match s.remove_all_invisible_characters() {
                Cow::Borrowed(ss) => Cow::Owned(unsafe { to_substring_in_place!(s, ss) }),
                Cow::Owned(s) => Cow::Owned(s),
            },
        }
    }
}
