/*!
# str Utils

This crate provides some traits to extend `[u8]`, `str` and `Cow<str>`.

## Examples

```rust
use str_utils::*;

assert_eq!(true, "foobar".starts_with_ignore_ascii_case("FoO"));
assert_eq!(Some(1), "photo.jpg".ends_with_ignore_ascii_case_multiple(&[".png", ".jpg", ".gif"]));
assert_eq!(true, "http".eq_ignore_ascii_case_with_uppercase("HTTP")); // faster than `eq_ignore_ascii_case`
assert_eq!(true, "foobar".starts_with_ignore_ascii_case("FoO"));

# #[cfg(feature = "alloc")]
assert_eq!("here is a ZERO_WIDTH_SPACE -> ​".len() - 3, "here is a ZERO_WIDTH_SPACE -> ​".remove_all_invisible_characters().len());

# #[cfg(feature = "alloc")]
assert_eq!(r"foo\% b\_r", r"foo% b_r".escape_ascii_characters(b'\\', b"%_"));

assert!(!"AaBb\u{0130}Zz".is_lowercased());
assert!(!"AaBbZz".is_ascii_lowercased());
assert!(!"aabbßzz".is_uppercased());
assert!(!"aabbzz".is_ascii_uppercased());

# #[cfg(feature = "alloc")]
assert_eq!("aabbi\u{0307}zz", "AaBb\u{0130}Zz".to_lowercase_cow());
# #[cfg(feature = "alloc")]
assert_eq!("aabb\u{0130}zz", "AaBb\u{0130}Zz".to_ascii_lowercase_cow());
# #[cfg(feature = "alloc")]
assert_eq!("AABBSSZZ", "aabbßzz".to_uppercase_cow());
# #[cfg(feature = "alloc")]
assert_eq!("AABBßZZ", "aabbßzz".to_ascii_uppercase_cow());

# #[cfg(feature = "alloc")]
assert_eq!("Line 1 Line 2 Line 2 Line 3 Line 4 Line 5", "Line 1\r\nLine 2\r\nLine 2\rLine 3\nLine 4\nLine 5".replace_newlines_with_space());
```

## No Std

Disable the default features to compile this crate without std.

```toml
[dependencies.str-utils]
version = "*"
default-features = false
```
*/

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]

// TODO eq_ignore_case_multiple
// TODO starts_with_ignore_case_multiple
// TODO ends_with_ignore_case_multiple

#[cfg(feature = "alloc")]
extern crate alloc;

mod ends_with_ignore_ascii_case;
mod ends_with_ignore_ascii_case_multiple;
#[cfg(feature = "alloc")]
mod ends_with_ignore_case;
mod ends_with_multiple;
mod eq_ignore_ascii_case;
mod eq_ignore_ascii_case_multiple;
#[cfg(feature = "unicase")]
mod eq_ignore_case;
mod eq_multiple;
#[cfg(feature = "alloc")]
mod escape_characters;
mod is_ascii_lowercased;
mod is_ascii_uppercased;
mod is_lowercased;
mod is_uppercased;
#[cfg(feature = "alloc")]
mod remove_all_invisible_characters;
#[cfg(feature = "alloc")]
mod replace_newlines_with_space;
mod starts_with_ignore_ascii_case;
mod starts_with_ignore_ascii_case_multiple;
#[cfg(feature = "alloc")]
mod starts_with_ignore_case;
mod starts_with_multiple;
#[cfg(feature = "alloc")]
mod to_lowercase;
#[cfg(feature = "alloc")]
mod to_uppercase;

pub use ends_with_ignore_ascii_case::*;
pub use ends_with_ignore_ascii_case_multiple::*;
#[cfg(feature = "alloc")]
pub use ends_with_ignore_case::*;
pub use ends_with_multiple::*;
pub use eq_ignore_ascii_case::*;
pub use eq_ignore_ascii_case_multiple::*;
#[cfg(feature = "unicase")]
pub use eq_ignore_case::*;
pub use eq_multiple::*;
#[cfg(feature = "alloc")]
pub use escape_characters::*;
pub use is_ascii_lowercased::*;
pub use is_ascii_uppercased::*;
pub use is_lowercased::*;
pub use is_uppercased::*;
#[cfg(feature = "alloc")]
pub use remove_all_invisible_characters::*;
#[cfg(feature = "alloc")]
pub use replace_newlines_with_space::*;
pub use starts_with_ignore_ascii_case::*;
pub use starts_with_ignore_ascii_case_multiple::*;
#[cfg(feature = "alloc")]
pub use starts_with_ignore_case::*;
pub use starts_with_multiple::*;
#[cfg(feature = "alloc")]
pub use to_lowercase::*;
#[cfg(feature = "alloc")]
pub use to_uppercase::*;

#[cfg(feature = "alloc")]
pub(crate) unsafe fn find_substring_position(parent: &str, sub: &str) -> (usize, usize) {
    let parent_start_address = parent.as_ptr() as usize;

    let sub_start_address = sub.as_ptr() as usize;
    let position_len = sub.len();

    let position_start = sub_start_address - parent_start_address;

    (position_start, position_len)
}

#[cfg(feature = "alloc")]
pub(crate) unsafe fn into_substring_in_place(
    mut s: String,
    (start, len): (usize, usize),
) -> String {
    let bytes = s.as_mut_vec();

    bytes.drain(..start);
    bytes.truncate(len);

    s
}

#[cfg(feature = "alloc")]
macro_rules! to_substring_in_place {
    ($parent:ident, $sub:ident) => {{
        let position = $crate::find_substring_position($parent.as_str(), $sub);

        $crate::into_substring_in_place($parent, position)
    }};
}

#[cfg(feature = "alloc")]
pub(crate) use to_substring_in_place;
