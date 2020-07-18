extern crate str_utils;

use str_utils::StartsWithIgnoreCase;

#[test]
fn starts_with_ignore_case() {
    assert_eq!(true, "Maße 123".starts_with_ignore_case("Maße"));
    assert_eq!(true, "Maße 123".starts_with_ignore_case("MASSE"));
    assert_eq!(false, "Maße 123".starts_with_ignore_case("messe"));
    assert_eq!(false, "Maße 123".starts_with_ignore_case("123"));
    assert_eq!(false, "Maße 123".starts_with_ignore_case(" Maße 123"));
}
