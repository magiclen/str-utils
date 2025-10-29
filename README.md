str Utils
====================

[![CI](https://github.com/magiclen/str-utils/actions/workflows/ci.yml/badge.svg)](https://github.com/magiclen/str-utils/actions/workflows/ci.yml)

This crate provides some traits to extend types which implement `AsRef<[u8]>` or `AsRef<str>`.

## Examples

```rust
use str_utils::*;

assert_eq!(true, "foobar".starts_with_ignore_ascii_case("FoO"));
assert_eq!(Some(1), "photo.jpg".ends_with_ignore_ascii_case_multiple(&[".png", ".jpg", ".gif"]));
assert_eq!(true, "http".eq_ignore_ascii_case_with_uppercase("HTTP")); // faster than `eq_ignore_ascii_case`
assert_eq!(true, "foobar".starts_with_ignore_ascii_case("FoO"));

assert_eq!("here is a ZERO_WIDTH_SPACE -> ​".len() - 3, "here is a ZERO_WIDTH_SPACE -> ​".remove_all_invisible_characters().len());

assert_eq!(r"foo\% b\_r", r"foo% b_r".escape_ascii_characters(b'\\', b"%_"));

assert_eq!("aabbi\u{0307}zz", "AaBb\u{0130}Zz".to_lowercase_cow());
assert_eq!("aabb\u{0130}zz", "AaBb\u{0130}Zz".to_ascii_lowercase_cow());
assert_eq!("AABBSSZZ", "aabbßzz".to_uppercase_cow());
assert_eq!("AABBßZZ", "aabbßzz".to_ascii_uppercase_cow());

assert_eq!("Line 1 Line 2 Line 2 Line 3 Line 4 Line 5", "Line 1\r\nLine 2\r\nLine 2\rLine 3\nLine 4\nLine 5".replace_newlines_with_space());
```

## No Std

Disable the default features to compile this crate without std.

```toml
[dependencies.str-utils]
version = "*"
default-features = false
```

## Crates.io

https://crates.io/crates/str-utils

## Documentation

https://docs.rs/str-utils

## License

[MIT](LICENSE)