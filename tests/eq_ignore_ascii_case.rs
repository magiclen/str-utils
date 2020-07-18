extern crate str_utils;

use str_utils::EqIgnoreAsciiCase;

#[test]
fn eq_ignore_ascii_case_with_lowercase() {
    assert_eq!(false, "12345678".eq_ignore_ascii_case_with_lowercase(""));
    assert_eq!(true, "12345678".eq_ignore_ascii_case_with_lowercase("12345678"));
    assert_eq!(true, "ABcDEFGh".eq_ignore_ascii_case_with_lowercase("abcdefgh"));
    assert_eq!(false, "ABcDEFGI".eq_ignore_ascii_case_with_lowercase("abcdefgh"));
}

#[test]
fn eq_ignore_ascii_case_with_uppercase() {
    assert_eq!(false, "12345678".eq_ignore_ascii_case_with_uppercase(""));
    assert_eq!(true, "12345678".eq_ignore_ascii_case_with_uppercase("12345678"));
    assert_eq!(true, "ABcDEFGh".eq_ignore_ascii_case_with_uppercase("ABCDEFGH"));
    assert_eq!(false, "ABcDEFGI".eq_ignore_ascii_case_with_uppercase("ABCDEFGH"));
}
