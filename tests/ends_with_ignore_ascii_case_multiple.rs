extern crate str_utils;

use str_utils::EndsWithIgnoreAsciiCaseMultiple;

#[test]
fn ends_with_ignore_ascii_case_multiple() {
    assert_eq!(Some(1), "foobar".ends_with_ignore_ascii_case_multiple(&["foo", "bar"]));
    assert_eq!(Some(1), "foobar".ends_with_ignore_ascii_case_multiple(&["foo", "FooBar"]));
    assert_eq!(None, "foobar".ends_with_ignore_ascii_case_multiple(&["foo", "ba"]));
}

#[test]
fn ends_with_ignore_ascii_case_with_lowercase_multiple() {
    assert_eq!(
        Some(1),
        "foobar".ends_with_ignore_ascii_case_with_lowercase_multiple(&["foo", "bar"])
    );
    assert_eq!(
        Some(1),
        "foobar".ends_with_ignore_ascii_case_with_lowercase_multiple(&["foo", "foobar"])
    );
    assert_eq!(None, "foobar".ends_with_ignore_ascii_case_with_lowercase_multiple(&["foo", "ba"]));
}

#[test]
fn ends_with_ignore_ascii_case_with_uppercase_multiple() {
    assert_eq!(
        Some(1),
        "foobar".ends_with_ignore_ascii_case_with_uppercase_multiple(&["FOO", "BAR"])
    );
    assert_eq!(
        Some(1),
        "foobar".ends_with_ignore_ascii_case_with_uppercase_multiple(&["FOO", "FOOBAR"])
    );
    assert_eq!(None, "foobar".ends_with_ignore_ascii_case_with_uppercase_multiple(&["FOO", "BA"]));
}
