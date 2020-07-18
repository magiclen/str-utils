extern crate str_utils;

use str_utils::EndsWithIgnoreAsciiCase;

#[test]
fn ends_with_ignore_ascii_case() {
    assert_eq!(true, "foobar".ends_with_ignore_ascii_case("bar"));
    assert_eq!(true, "foobar".ends_with_ignore_ascii_case("BAR"));
    assert_eq!(true, "foobar".ends_with_ignore_ascii_case("BaR"));
    assert_eq!(true, "foobar".ends_with_ignore_ascii_case("FooBar"));
    assert_eq!(false, "foobar".ends_with_ignore_ascii_case("foo"));
    assert_eq!(false, "foobar".ends_with_ignore_ascii_case(" foobar"));
}

#[test]
fn ends_with_ignore_ascii_case_with_lowercase() {
    assert_eq!(true, "foobar".ends_with_ignore_ascii_case_with_lowercase("bar"));
    assert_eq!(true, "foobar".ends_with_ignore_ascii_case_with_lowercase("foobar"));
    assert_eq!(false, "foobar".ends_with_ignore_ascii_case_with_lowercase("foo"));
    assert_eq!(false, "foobar".ends_with_ignore_ascii_case_with_lowercase(" foobar"));
}

#[test]
fn ends_with_ignore_ascii_case_with_uppercase() {
    assert_eq!(true, "foobar".ends_with_ignore_ascii_case_with_uppercase("BAR"));
    assert_eq!(true, "foobar".ends_with_ignore_ascii_case_with_uppercase("FOOBAR"));
    assert_eq!(false, "foobar".ends_with_ignore_ascii_case_with_uppercase("FOO"));
    assert_eq!(false, "foobar".ends_with_ignore_ascii_case_with_uppercase(" FOOBAR"));
}
