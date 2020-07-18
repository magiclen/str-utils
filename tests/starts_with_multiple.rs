extern crate str_utils;

use str_utils::StartsWithMultiple;

#[test]
fn starts_with_multiple() {
    assert_eq!(Some(0), "foobar".starts_with_multiple(&["foo", "bar"]));
    assert_eq!(None, "foobar".starts_with_multiple(&["bar", "FooBar"]));
    assert_eq!(None, "foobar".starts_with_multiple(&["bar", "oo"]));
}
