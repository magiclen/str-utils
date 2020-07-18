extern crate str_utils;

use str_utils::EqMultiple;

#[test]
fn eq_multiple() {
    assert_eq!(None, "12345678".eq_multiple(&[] as &[&str]));
    assert_eq!(None, "12345678".eq_multiple(&[""]));
    assert_eq!(Some(0), "12345678".eq_multiple(&["12345678", "12345670"]));
    assert_eq!(Some(1), "12345678".eq_multiple(&["12345670", "12345678"]));
    assert_eq!(Some(0), "12345678".eq_multiple(&["12345678", "12345678"]));
    assert_eq!(None, "12345678".eq_multiple(&["12345670", "12345670"]));
}
