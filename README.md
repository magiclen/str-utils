str Utils
====================

[![Build Status](https://travis-ci.org/magiclen/str-utils.svg?branch=master)](https://travis-ci.org/magiclen/str-utils)

This crate provides some traits to extend types which implement `AsRef<[u8]>` or `AsRef<str>`.

## Examples

```rust
extern crate str_utils;

use str_utils::*;

assert_eq!(true, "foobar".starts_with_ignore_ascii_case("FoO"));
```

```rust
extern crate str_utils;

use str_utils::*;

assert_eq!(Some(1), "photo.jpg".ends_with_ignore_ascii_case_multiple(&[".png", ".jpg", ".gif"]));
```

```rust
extern crate str_utils;

use str_utils::*;

assert_eq!(true, "http".eq_ignore_ascii_case_with_uppercase("HTTP")); // faster than `eq_ignore_ascii_case`
```

## Crates.io

https://crates.io/crates/str-utils

## Documentation

https://docs.rs/str-utils

## License

[MIT](LICENSE)