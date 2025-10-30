#![cfg(feature = "alloc")]

extern crate alloc;

use alloc::borrow::Cow;

use str_utils::Trim;

#[test]
fn trim_cow() {
    let s = "\n 12345 6789 \n";

    let c_borrowed = Cow::Borrowed(s);
    let c_owned: Cow<'static, str> = Cow::Owned(String::from(s));

    let a = "12345 6789";
    assert_eq!(a, c_borrowed.trim_cow());
    assert_eq!(a, c_owned.trim_cow());
}

#[test]
fn trim_start_cow() {
    let s = "\n 12345 6789 \n";

    let c_borrowed = Cow::Borrowed(s);
    let c_owned: Cow<'static, str> = Cow::Owned(String::from(s));

    let a = "12345 6789 \n";
    assert_eq!(a, c_borrowed.trim_start_cow());
    assert_eq!(a, c_owned.trim_start_cow());
}

#[test]
fn trim_end_cow() {
    let s = "\n 12345 6789 \n";

    let c_borrowed = Cow::Borrowed(s);
    let c_owned: Cow<'static, str> = Cow::Owned(String::from(s));

    let a = "\n 12345 6789";
    assert_eq!(a, c_borrowed.trim_end_cow());
    assert_eq!(a, c_owned.trim_end_cow());
}

#[test]
fn trim_ascii() {
    let s = "\n 12345 6789 \n";

    let c_borrowed = Cow::Borrowed(s);
    let c_owned: Cow<'static, str> = Cow::Owned(String::from(s));

    let a = "12345 6789";
    assert_eq!(a, c_borrowed.trim_ascii());
    assert_eq!(a, c_owned.trim_ascii());
}

#[test]
fn trim_ascii_start() {
    let s = "\n 12345 6789 \n";

    let c_borrowed = Cow::Borrowed(s);
    let c_owned: Cow<'static, str> = Cow::Owned(String::from(s));

    let a = "12345 6789 \n";
    assert_eq!(a, c_borrowed.trim_ascii_start());
    assert_eq!(a, c_owned.trim_ascii_start());
}

#[test]
fn trim_ascii_end() {
    let s = "\n 12345 6789";

    let c_borrowed = Cow::Borrowed(s);
    let c_owned: Cow<'static, str> = Cow::Owned(String::from(s));

    let a = "\n 12345 6789";
    assert_eq!(a, c_borrowed.trim_ascii_end());
    assert_eq!(a, c_owned.trim_ascii_end());
}
