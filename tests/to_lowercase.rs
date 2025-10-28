#![cfg(feature = "alloc")]

use str_utils::ToLowercase;

#[test]
fn to_lowercase_cow() {
    let s = "AaBb\u{0130}Zz";

    assert_eq!(s.len(), s.trim().len());

    let ss = s.to_lowercase_cow();

    assert_eq!("aabbi\u{0307}zz", ss);
}

#[test]
fn to_ascii_lowercase_cow() {
    let s = "AaBb\u{0130}Zz";

    assert_eq!(s.len(), s.trim().len());

    let ss = s.to_ascii_lowercase();

    assert_eq!("aabb\u{0130}zz", ss);
}
