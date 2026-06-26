#![cfg(feature = "alloc")]

use str_utils::Replace;

#[test]
fn replace_cow_with_str() {
    let ss = "foo foo".replace_cow("foo", "bar");

    assert_eq!("bar bar", ss);
}

#[test]
fn replace_cow_with_char() {
    let ss = "a-b-c".replace_cow('-', "_");

    assert_eq!("a_b_c", ss);
}

#[test]
fn replace_cow_with_char_slice() {
    let from: &[char] = &['a', 'b'];
    let ss = "abc".replace_cow(from, "x");

    assert_eq!("xxc", ss);
}

#[test]
fn replace_cow_with_char_array() {
    let ss = "a-b.c".replace_cow(['-', '.'], "_");

    assert_eq!("a_b_c", ss);
}

#[test]
fn replace_cow_with_predicate() {
    let ss = "a1b2".replace_cow(|c: char| c.is_ascii_digit(), "#");

    assert_eq!("a#b#", ss);
}

#[test]
fn replacen_cow_limits_replacements() {
    let ss = "foo foo foo".replacen_cow("foo", "bar", 2);

    assert_eq!("bar bar foo", ss);
}
