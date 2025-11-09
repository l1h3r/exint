Creates a native endian integer value from its memory representation as a byte
array in native endianness.

As the target platform's native endianness is used, portable code likely wants
to use [`from_be_bytes`] or [`from_le_bytes`], as appropriate instead.

[`from_be_bytes`]: Self::from_be_bytes
[`from_le_bytes`]: Self::from_le_bytes

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
use exint::Saturating;

if cfg!(target_endian = "big") {
    assert_eq!(Saturating::<i24>::from_ne_bytes([0x12, 0x34, 0x56]), Saturating(0x123456_i24));
} else {
    assert_eq!(Saturating::<i24>::from_ne_bytes([0x56, 0x34, 0x12]), Saturating(0x123456_i24));
}
# }
```
