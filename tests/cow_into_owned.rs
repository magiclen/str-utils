#![cfg(feature = "alloc")]

use std::borrow::Cow;

use str_utils::{cow_into_owned, ToLowercase, ToUppercase};

#[test]
fn borrowed_string_case_returns_original_string_and_keeps_allocation() {
    let s = String::from("abc");
    let ptr = s.as_ptr();

    let s = cow_into_owned!(s, s.as_str().to_lowercase_cow());

    assert_eq!("abc", s);
    assert_eq!(ptr, s.as_ptr());
}

#[test]
fn owned_string_case_returns_cow_owned_string() {
    let original = String::from("original");
    let owned = String::from("owned");
    let owned_ptr = owned.as_ptr();
    let cow: Cow<'_, str> = Cow::Owned(owned);

    let s = cow_into_owned!(original, cow);

    assert_eq!("owned", s);
    assert_eq!(owned_ptr, s.as_ptr());
}

#[test]
fn borrowed_slice_case_returns_original_vec_and_keeps_allocation() {
    let v = vec![1, 2, 3];
    let ptr = v.as_ptr();
    let cow: Cow<'_, [u8]> = Cow::Borrowed(v.as_slice());

    let v = cow_into_owned!(v, cow);

    assert_eq!([1, 2, 3], *v);
    assert_eq!(ptr, v.as_ptr());
}

#[test]
fn owned_slice_case_returns_cow_owned_vec() {
    let original = vec![1, 2, 3];
    let owned = vec![4, 5, 6];
    let owned_ptr = owned.as_ptr();
    let cow: Cow<'_, [u8]> = Cow::Owned(owned);

    let v = cow_into_owned!(original, cow);

    assert_eq!([4, 5, 6], *v);
    assert_eq!(owned_ptr, v.as_ptr());
}

#[test]
fn accepts_trailing_comma() {
    let s = String::from("abc");

    let s = cow_into_owned!(s, s.as_str().to_uppercase_cow(),);

    assert_eq!("ABC", s);
}
