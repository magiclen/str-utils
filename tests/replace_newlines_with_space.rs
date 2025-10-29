#![cfg(feature = "alloc")]

use str_utils::ReplaceNewlinesWithSpace;

#[test]
fn replace_newlines_with_space() {
    let s = "Line 1\r\nLine 2\r\nLine 2\rLine 3\nLine 4\nLine 5";

    let ss = s.replace_newlines_with_space();
    assert_eq!("Line 1 Line 2 Line 2 Line 3 Line 4 Line 5", ss);
}
