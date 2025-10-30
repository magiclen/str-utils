use str_utils::IsLowercased;

#[test]
fn is_lowercased() {
    let s = "AaBb\u{0130}Zz";

    assert!(!s.is_lowercased());

    let s = "aabbi\u{0307}zz";

    assert!(s.is_lowercased());
}
