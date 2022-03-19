use str_utils::StartsWithIgnoreAsciiCase;

#[test]
fn starts_with_ignore_ascii_case() {
    assert!("foobar".starts_with_ignore_ascii_case("foo"));
    assert!("foobar".starts_with_ignore_ascii_case("FOO"));
    assert!("foobar".starts_with_ignore_ascii_case("FoO"));
    assert!("foobar".starts_with_ignore_ascii_case("FooBar"));
    assert!(!"foobar".starts_with_ignore_ascii_case("bar"));
    assert!(!"foobar".starts_with_ignore_ascii_case(" foobar"));
}

#[test]
fn starts_with_ignore_ascii_case_with_lowercase() {
    assert!("foobar".starts_with_ignore_ascii_case_with_lowercase("foo"));
    assert!("foobar".starts_with_ignore_ascii_case_with_lowercase("foobar"));
    assert!(!"foobar".starts_with_ignore_ascii_case_with_lowercase("bar"));
    assert!(!"foobar".starts_with_ignore_ascii_case_with_lowercase(" foobar"));
}

#[test]
fn starts_with_ignore_ascii_case_with_uppercase() {
    assert!("foobar".starts_with_ignore_ascii_case_with_uppercase("FOO"));
    assert!("foobar".starts_with_ignore_ascii_case_with_uppercase("FOOBAR"));
    assert!(!"foobar".starts_with_ignore_ascii_case_with_uppercase("BAR"));
    assert!(!"foobar".starts_with_ignore_ascii_case_with_uppercase(" FOOBAR"));
}
