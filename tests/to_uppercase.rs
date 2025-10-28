#![cfg(feature = "alloc")]

use str_utils::ToUppercase;

#[test]
fn to_uppercase_cow() {
    let s = "AaBbßZz";

    assert_eq!(s.len(), s.trim().len());

    let ss = s.to_uppercase_cow();

    assert_eq!("AABBSSZZ", ss);
}

#[test]
fn to_ascii_uppercase_cow() {
    let s = "AaBbßZz";

    assert_eq!(s.len(), s.trim().len());

    let ss = s.to_ascii_uppercase();

    assert_eq!("AABBßZZ", ss);
}
