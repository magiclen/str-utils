use str_utils::StartsWithIgnoreAsciiCaseMultiple;

#[test]
fn starts_with_ignore_ascii_case_multiple() {
    assert_eq!(Some(0), "foobar".starts_with_ignore_ascii_case_multiple(&["foo", "bar"]));
    assert_eq!(Some(1), "foobar".starts_with_ignore_ascii_case_multiple(&["bar", "FooBar"]));
    assert_eq!(None, "foobar".starts_with_ignore_ascii_case_multiple(&["bar", "oo"]));
}

#[test]
fn starts_with_ignore_ascii_case_with_lowercase_multiple() {
    assert_eq!(
        Some(0),
        "foobar".starts_with_ignore_ascii_case_with_lowercase_multiple(&["foo", "bar"])
    );
    assert_eq!(
        Some(1),
        "foobar".starts_with_ignore_ascii_case_with_lowercase_multiple(&["bar", "foobar"])
    );
    assert_eq!(
        None,
        "foobar".starts_with_ignore_ascii_case_with_lowercase_multiple(&["bar", "oo"])
    );
}

#[test]
fn starts_with_ignore_ascii_case_with_uppercase_multiple() {
    assert_eq!(
        Some(0),
        "foobar".starts_with_ignore_ascii_case_with_uppercase_multiple(&["FOO", "BAR"])
    );
    assert_eq!(
        Some(1),
        "foobar".starts_with_ignore_ascii_case_with_uppercase_multiple(&["BAR", "FOOBAR"])
    );
    assert_eq!(
        None,
        "foobar".starts_with_ignore_ascii_case_with_uppercase_multiple(&["BAR", "OO"])
    );
}
