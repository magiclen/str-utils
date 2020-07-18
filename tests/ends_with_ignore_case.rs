extern crate str_utils;

use str_utils::EndsWithIgnoreCase;

#[test]
fn ends_with_ignore_case() {
    assert_eq!(true, "123 Maße".ends_with_ignore_case("Maße"));
    assert_eq!(true, "123 Maße".ends_with_ignore_case("MASSE"));
    assert_eq!(false, "123 Maße".ends_with_ignore_case("messe"));
    assert_eq!(false, "123 Maße".ends_with_ignore_case("123"));
    assert_eq!(false, "123 Maße".ends_with_ignore_case(" Maße 123"));
}
