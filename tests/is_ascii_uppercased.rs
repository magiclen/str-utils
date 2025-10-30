use str_utils::IsAsciiUppercased;

#[test]
fn is_ascii_uppercased() {
    let s = "AaBbZz";

    assert!(!s.is_ascii_uppercased());

    let s = "AABBZZ";

    assert!(s.is_ascii_uppercased());
}
