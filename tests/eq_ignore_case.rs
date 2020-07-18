extern crate str_utils;

use str_utils::EqIgnoreCase;

#[test]
fn eq_ignore_ascii_case_with_lowercase() {
    assert_eq!(false, "Maße".eq_ignore_case(""));
    assert_eq!(false, "Maße".eq_ignore_case("Mabe"));
    assert_eq!(true, "Maße".eq_ignore_case("MASSE"));
}
