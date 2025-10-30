use str_utils::IsAsciiLowercased;

#[test]
fn is_ascii_lowercased() {
    let s = "AaBbZz";

    assert!(!s.is_ascii_lowercased());

    let s = "aabbzz";

    assert!(s.is_ascii_lowercased());
}
