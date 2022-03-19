use str_utils::EndsWithIgnoreCase;

#[test]
fn ends_with_ignore_case() {
    assert!("123 Maße".ends_with_ignore_case("Maße"));
    assert!("123 Maße".ends_with_ignore_case("MASSE"));
    assert!(!"123 Maße".ends_with_ignore_case("messe"));
    assert!(!"123 Maße".ends_with_ignore_case("123"));
    assert!(!"123 Maße".ends_with_ignore_case(" Maße 123"));
}
