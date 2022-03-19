use str_utils::EndsWithIgnoreAsciiCase;

#[test]
fn ends_with_ignore_ascii_case() {
    assert!("foobar".ends_with_ignore_ascii_case("bar"));
    assert!("foobar".ends_with_ignore_ascii_case("BAR"));
    assert!("foobar".ends_with_ignore_ascii_case("BaR"));
    assert!("foobar".ends_with_ignore_ascii_case("FooBar"));
    assert!(!"foobar".ends_with_ignore_ascii_case("foo"));
    assert!(!"foobar".ends_with_ignore_ascii_case(" foobar"));
}

#[test]
fn ends_with_ignore_ascii_case_with_lowercase() {
    assert!("foobar".ends_with_ignore_ascii_case_with_lowercase("bar"));
    assert!("foobar".ends_with_ignore_ascii_case_with_lowercase("foobar"));
    assert!(!"foobar".ends_with_ignore_ascii_case_with_lowercase("foo"));
    assert!(!"foobar".ends_with_ignore_ascii_case_with_lowercase(" foobar"));
}

#[test]
fn ends_with_ignore_ascii_case_with_uppercase() {
    assert!("foobar".ends_with_ignore_ascii_case_with_uppercase("BAR"));
    assert!("foobar".ends_with_ignore_ascii_case_with_uppercase("FOOBAR"));
    assert!(!"foobar".ends_with_ignore_ascii_case_with_uppercase("FOO"));
    assert!(!"foobar".ends_with_ignore_ascii_case_with_uppercase(" FOOBAR"));
}
