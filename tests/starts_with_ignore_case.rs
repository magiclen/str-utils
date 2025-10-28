#![cfg(feature = "alloc")]

use str_utils::StartsWithIgnoreCase;

#[test]
fn starts_with_ignore_case() {
    assert!("Maße 123".starts_with_ignore_case("Maße"));
    assert!("Maße 123".starts_with_ignore_case("MASSE"));
    assert!(!"Maße 123".starts_with_ignore_case("messe"));
    assert!(!"Maße 123".starts_with_ignore_case("123"));
    assert!(!"Maße 123".starts_with_ignore_case(" Maße 123"));
}
