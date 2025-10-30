use str_utils::IsUppercased;

#[test]
fn is_uppercased() {
    let s = "AaBbßZz";

    assert!(!s.is_uppercased());

    let s = "AABBSSZZ";

    assert!(s.is_uppercased());
}
