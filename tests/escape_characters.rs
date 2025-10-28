#![cfg(feature = "std")]

use str_utils::EscapeCharacters;

#[test]
fn escape_characters() {
    let s = r"a%b_c\d";

    assert_eq!(s.len(), s.trim().len());

    let ss = s.escape_characters('\\', &['%', '_']);

    assert_eq!(r"a\%b\_c\\d", ss);
}

#[test]
fn escape_ascii_characters() {
    let s = r"一%二_三\四";

    assert_eq!(s.len(), s.trim().len());

    let ss = s.escape_ascii_characters(b'\\', b"%_");

    assert_eq!(r"一\%二\_三\\四", ss);
}
