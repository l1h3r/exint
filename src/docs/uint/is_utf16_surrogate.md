Checks if the value is a Unicode surrogate code point,
which are disallowed values for [`char`].

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
let low_non_surrogate = 0xA000_u16;
let low_surrogate = 0xD800_u16;
let high_surrogate = 0xDC00_u16;
let high_non_surrogate = 0xE000_u16;

assert!(!low_non_surrogate.is_utf16_surrogate());
assert!(low_surrogate.is_utf16_surrogate());
assert!(high_surrogate.is_utf16_surrogate());
assert!(!high_non_surrogate.is_utf16_surrogate());
# }
```
