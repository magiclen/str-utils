#![cfg(feature = "alloc")]

use std::borrow::Cow;

use str_utils::RemoveInvisibleCharacters;

#[test]
fn remove_all_invisible_characters() {
    #[allow(clippy::octal_escapes)]
    let s = "\0\01234";
    let ss = s.remove_all_invisible_characters();
    assert!(matches!(ss, Cow::Borrowed(_)));
    assert_eq!("1234", ss);

    #[allow(clippy::octal_escapes)]
    let s = "1234\0\0";
    let ss = s.remove_all_invisible_characters();
    assert!(matches!(ss, Cow::Borrowed(_)));
    assert_eq!("1234", ss);

    #[allow(clippy::octal_escapes)]
    let s = "\0\01234\0\0";
    let ss = s.remove_all_invisible_characters();
    assert!(matches!(ss, Cow::Borrowed(_)));
    assert_eq!("1234", ss);

    #[allow(clippy::octal_escapes)]
    let s = "\0\012\0\034\0\0";
    let ss = s.remove_all_invisible_characters();
    assert!(matches!(ss, Cow::Owned(_)));
    assert_eq!("1234", ss);

    #[allow(clippy::invisible_characters)]
    let s = "here is a ZERO_WIDTH_SPACE -> ​";

    assert_eq!(s.len(), s.trim().len());

    let ss = s.remove_all_invisible_characters();
    assert_eq!(s.chars().take(s.chars().count() - 1).collect::<String>(), ss);
    assert_eq!(s.len() - 4, ss.trim().len());
}
