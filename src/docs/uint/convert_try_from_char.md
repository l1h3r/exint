Tries to convert a [`char`] into a [`uint`].

[`uint`]: crate::types::uint

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
let a = 'ÿ'; // U+00FF
let b = 'Ā'; // U+0100
let c = '🥷'; // U+1F977
let d = char::MAX; // U+10FFFF

assert_eq!(u8::try_from(a), Ok(0xFF_u8));
assert_eq!(u8::try_from(b).is_err(), true);
assert_eq!(u16::try_from(b), Ok(0x0100_u16));
assert_eq!(u16::try_from(c).is_err(), true);
assert_eq!(u24::try_from(c), Ok(0x1F977_u24));
assert_eq!(u24::try_from(d), Ok(0x10FFFF_u24));
# }
```
