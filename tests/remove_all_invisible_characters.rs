#![cfg(feature = "std")]

use str_utils::RemoveInvisibleCharacters;

#[test]
fn remove_all_invisible_characters() {
    #[allow(clippy::invisible_characters)]
    let s = "here is a ZERO_WIDTH_SPACE -> ​";

    assert_eq!(s.len(), s.trim().len());

    let ss = s.remove_all_invisible_characters();
    assert_eq!(s.chars().take(s.chars().count() - 1).collect::<String>(), ss);
    assert_eq!(s.len() - 4, ss.trim().len());
}
