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
if cfg!(target_endian = "big") {
    assert_eq!(u24::from_ne_bytes([0x12, 0x34, 0x56]), 0x123456_u24);
} else {
    assert_eq!(u24::from_ne_bytes([0x56, 0x34, 0x12]), 0x123456_u24);
}
# }
```
