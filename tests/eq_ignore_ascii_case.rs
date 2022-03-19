use str_utils::EqIgnoreAsciiCase;

#[test]
fn eq_ignore_ascii_case_with_lowercase() {
    assert!(!"12345678".eq_ignore_ascii_case_with_lowercase(""));
    assert!("12345678".eq_ignore_ascii_case_with_lowercase("12345678"));
    assert!("ABcDEFGh".eq_ignore_ascii_case_with_lowercase("abcdefgh"));
    assert!(!"ABcDEFGI".eq_ignore_ascii_case_with_lowercase("abcdefgh"));
}

#[test]
fn eq_ignore_ascii_case_with_uppercase() {
    assert!(!"12345678".eq_ignore_ascii_case_with_uppercase(""));
    assert!("12345678".eq_ignore_ascii_case_with_uppercase("12345678"));
    assert!("ABcDEFGh".eq_ignore_ascii_case_with_uppercase("ABCDEFGH"));
    assert!(!"ABcDEFGI".eq_ignore_ascii_case_with_uppercase("ABCDEFGH"));
}
