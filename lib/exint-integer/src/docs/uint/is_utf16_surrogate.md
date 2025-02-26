Checks if the value is a Unicode surrogate code point,
which are disallowed values for [`char`].

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint_integer as exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
let low_non_surrogate = uint!(0xA000 u16);
let low_surrogate = uint!(0xD800 u16);
let high_surrogate = uint!(0xDC00 u16);
let high_non_surrogate = uint!(0xE000 u16);

assert!(!low_non_surrogate.is_utf16_surrogate());
assert!(low_surrogate.is_utf16_surrogate());
assert!(high_surrogate.is_utf16_surrogate());
assert!(!high_non_surrogate.is_utf16_surrogate());
```
