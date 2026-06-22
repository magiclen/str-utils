#![cfg(feature = "alloc")]

use str_utils::EndsWithIgnoreCase;

#[test]
fn ends_with_ignore_case() {
    assert!("123 Maße".ends_with_ignore_case("Maße"));
    assert!("123 Maße".ends_with_ignore_case("MASSE"));
    assert!(!"123 Maße".ends_with_ignore_case("messe"));
    assert!(!"123 Maße".ends_with_ignore_case("123"));
    assert!(!"123 Maße".ends_with_ignore_case(" Maße 123"));

    assert!("foobar".ends_with_ignore_case("BAR"));
    assert!(!"foobar".ends_with_ignore_case("FOO"));

    assert!("xß".ends_with_ignore_case("S"));
    assert!(!"xİ".ends_with_ignore_case("i"));
}
