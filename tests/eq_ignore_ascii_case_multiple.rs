use str_utils::EqIgnoreAsciiCaseMultiple;

#[test]
fn eq_ignore_ascii_case_multiple() {
    assert_eq!(None, "12345678".eq_ignore_ascii_case_multiple(&[] as &[&str]));
    assert_eq!(None, "12345678".eq_ignore_ascii_case_multiple(&[""]));
    assert_eq!(Some(0), "12345678".eq_ignore_ascii_case_multiple(&["12345678", "12345670"]));
    assert_eq!(Some(1), "12345678".eq_ignore_ascii_case_multiple(&["12345670", "12345678"]));
    assert_eq!(Some(0), "12345678".eq_ignore_ascii_case_multiple(&["12345678", "12345678"]));
    assert_eq!(None, "12345678".eq_ignore_ascii_case_multiple(&["12345670", "12345670"]));
    assert_eq!(Some(1), "ABcDEFGh".eq_ignore_ascii_case_multiple(&["12345670", "AbcDEfGh"]));
}

#[test]
fn eq_ignore_ascii_case_with_lowercase_multiple() {
    assert_eq!(None, "12345678".eq_ignore_ascii_case_with_lowercase_multiple(&[] as &[&str]));
    assert_eq!(None, "12345678".eq_ignore_ascii_case_with_lowercase_multiple(&[""]));
    assert_eq!(
        Some(0),
        "12345678".eq_ignore_ascii_case_with_lowercase_multiple(&["12345678", "12345670"])
    );
    assert_eq!(
        Some(1),
        "12345678".eq_ignore_ascii_case_with_lowercase_multiple(&["12345670", "12345678"])
    );
    assert_eq!(
        Some(0),
        "12345678".eq_ignore_ascii_case_with_lowercase_multiple(&["12345678", "12345678"])
    );
    assert_eq!(
        None,
        "12345678".eq_ignore_ascii_case_with_lowercase_multiple(&["12345670", "12345670"])
    );
    assert_eq!(
        Some(1),
        "ABcDEFGh".eq_ignore_ascii_case_with_lowercase_multiple(&["12345670", "abcdefgh"])
    );
}

#[test]
fn eq_ignore_ascii_case_with_uppercase_multiple() {
    assert_eq!(None, "12345678".eq_ignore_ascii_case_with_uppercase_multiple(&[] as &[&str]));
    assert_eq!(None, "12345678".eq_ignore_ascii_case_with_uppercase_multiple(&[""]));
    assert_eq!(
        Some(0),
        "12345678".eq_ignore_ascii_case_with_uppercase_multiple(&["12345678", "12345670"])
    );
    assert_eq!(
        Some(1),
        "12345678".eq_ignore_ascii_case_with_uppercase_multiple(&["12345670", "12345678"])
    );
    assert_eq!(
        Some(0),
        "12345678".eq_ignore_ascii_case_with_uppercase_multiple(&["12345678", "12345678"])
    );
    assert_eq!(
        None,
        "12345678".eq_ignore_ascii_case_with_uppercase_multiple(&["12345670", "12345670"])
    );
    assert_eq!(
        Some(1),
        "ABcDEFGh".eq_ignore_ascii_case_with_uppercase_multiple(&["12345670", "ABCDEFGH"])
    );
}
