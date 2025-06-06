/*!
# str Utils

This crate provides some traits to extend types which implement `AsRef<[u8]>` or `AsRef<str>`.

## Examples

```rust
use str_utils::*;

assert_eq!(true, "foobar".starts_with_ignore_ascii_case("FoO"));
assert_eq!(Some(1), "photo.jpg".ends_with_ignore_ascii_case_multiple(&[".png", ".jpg", ".gif"]));
assert_eq!(true, "http".eq_ignore_ascii_case_with_uppercase("HTTP")); // faster than `eq_ignore_ascii_case`
```
*/

#![cfg_attr(not(feature = "std"), no_std)]

// TODO eq_ignore_case_multiple
// TODO starts_with_ignore_case_multiple
// TODO ends_with_ignore_case_multiple

mod ends_with_ignore_ascii_case;
mod ends_with_ignore_ascii_case_multiple;
mod ends_with_ignore_case;
mod ends_with_multiple;
mod eq_ignore_ascii_case;
mod eq_ignore_ascii_case_multiple;
mod eq_ignore_case;
mod eq_multiple;
#[cfg(feature = "std")]
mod remove_all_invisible_characters;
mod starts_with_ignore_ascii_case;
mod starts_with_ignore_ascii_case_multiple;
mod starts_with_ignore_case;
mod starts_with_multiple;

pub use ends_with_ignore_ascii_case::*;
pub use ends_with_ignore_ascii_case_multiple::*;
pub use ends_with_ignore_case::*;
pub use ends_with_multiple::*;
pub use eq_ignore_ascii_case::*;
pub use eq_ignore_ascii_case_multiple::*;
pub use eq_ignore_case::*;
pub use eq_multiple::*;
#[cfg(feature = "std")]
pub use remove_all_invisible_characters::*;
pub use starts_with_ignore_ascii_case::*;
pub use starts_with_ignore_ascii_case_multiple::*;
pub use starts_with_ignore_case::*;
pub use starts_with_multiple::*;
