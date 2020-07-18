extern crate unicase;

use unicase::UniCase;

/**
To extend types which implement `AsRef<str>` to have a `eq_ignore_case` method.

However, it is not recommended to use this trait. Try to wrap your type inside `UniCase` by yourself instead.

```rust
extern crate unicase;
use unicase::UniCase;

let a = UniCase::new("Maße");
let b = UniCase::new("MASSE");

assert_eq!(a, b);
```
*/
pub trait EqIgnoreCase {
    /// Returns `true` if the given string slice case-insensitively (using case-folding) matches this string slice.
    fn eq_ignore_case<S: AsRef<str>>(&self, b: S) -> bool;
}

impl<T: AsRef<str>> EqIgnoreCase for T {
    #[inline]
    fn eq_ignore_case<S: AsRef<str>>(&self, b: S) -> bool {
        let a = self.as_ref();
        let b = b.as_ref();

        if a.is_ascii() {
            if b.is_ascii() {
                a.eq_ignore_ascii_case(b)
            } else {
                let a = UniCase::ascii(a);
                let b = UniCase::unicode(b);

                a == b
            }
        } else if b.is_ascii() {
            let a = UniCase::unicode(a);
            let b = UniCase::ascii(b);

            a == b
        } else {
            let a = UniCase::unicode(a);
            let b = UniCase::unicode(b);

            a == b
        }
    }
}
