extern crate str_utils;

use str_utils::StartsWithIgnoreAsciiCase;

#[test]
fn starts_with_ignore_ascii_case() {
    assert_eq!(true, "foobar".starts_with_ignore_ascii_case("foo"));
    assert_eq!(true, "foobar".starts_with_ignore_ascii_case("FOO"));
    assert_eq!(true, "foobar".starts_with_ignore_ascii_case("FoO"));
    assert_eq!(true, "foobar".starts_with_ignore_ascii_case("FooBar"));
    assert_eq!(false, "foobar".starts_with_ignore_ascii_case("bar"));
    assert_eq!(false, "foobar".starts_with_ignore_ascii_case(" foobar"));
}

#[test]
fn starts_with_ignore_ascii_case_with_lowercase() {
    assert_eq!(true, "foobar".starts_with_ignore_ascii_case_with_lowercase("foo"));
    assert_eq!(true, "foobar".starts_with_ignore_ascii_case_with_lowercase("foobar"));
    assert_eq!(false, "foobar".starts_with_ignore_ascii_case_with_lowercase("bar"));
    assert_eq!(false, "foobar".starts_with_ignore_ascii_case_with_lowercase(" foobar"));
}

#[test]
fn starts_with_ignore_ascii_case_with_uppercase() {
    assert_eq!(true, "foobar".starts_with_ignore_ascii_case_with_uppercase("FOO"));
    assert_eq!(true, "foobar".starts_with_ignore_ascii_case_with_uppercase("FOOBAR"));
    assert_eq!(false, "foobar".starts_with_ignore_ascii_case_with_uppercase("BAR"));
    assert_eq!(false, "foobar".starts_with_ignore_ascii_case_with_uppercase(" FOOBAR"));
}
