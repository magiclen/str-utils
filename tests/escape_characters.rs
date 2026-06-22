#![cfg(feature = "alloc")]

use str_utils::EscapeCharacters;

#[test]
fn escape_characters() {
    let s = r"a%b_c\d";

    assert_eq!(s.len(), s.trim().len());

    let ss = s.escape_characters('\\', &['%', '_']);

    assert_eq!(r"a\%b\_c\\d", ss);
}

#[test]
fn escape_characters_escapes_escape_character_with_empty_list() {
    let s = r"a\b";

    let ss = s.escape_characters('\\', &[]);

    assert_eq!(r"a\\b", ss);
}

#[test]
fn escape_ascii_characters() {
    let s = r"一%二_三\四";

    assert_eq!(s.len(), s.trim().len());

    let ss = s.escape_ascii_characters(b'\\', b"%_");

    assert_eq!(r"一\%二\_三\\四", ss);
}

#[test]
fn escape_ascii_characters_escapes_escape_character_with_empty_list() {
    let s = r"a\b";

    let ss = s.escape_ascii_characters(b'\\', b"");

    assert_eq!(r"a\\b", ss);
}

#[test]
#[should_panic(expected = "escape_character must be ASCII")]
fn escape_ascii_characters_rejects_non_ascii_escape_character() {
    let _ = "a%b".escape_ascii_characters(0xFF, b"%");
}