#![cfg(feature = "unicase")]

use str_utils::EqIgnoreCase;

#[test]
fn eq_ignore_ascii_case_with_lowercase() {
    assert!(!"Maße".eq_ignore_case(""));
    assert!(!"Maße".eq_ignore_case("Mabe"));
    assert!("Maße".eq_ignore_case("MASSE"));
}
