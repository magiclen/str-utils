use str_utils::EndsWithMultiple;

#[test]
fn ends_with_multiple() {
    assert_eq!(Some(1), "foobar".ends_with_multiple(&["foo", "bar"]));
    assert_eq!(None, "foobar".ends_with_multiple(&["foo", "FooBar"]));
    assert_eq!(None, "foobar".ends_with_multiple(&["foo", "ba"]));
}
