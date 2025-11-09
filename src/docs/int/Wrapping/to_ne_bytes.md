Returns the memory representation of this integer as a byte array in native byte order.

As the target platform's native endianness is used, portable code should use
[`to_be_bytes`] or [`to_le_bytes`], as appropriate, instead.

[`to_be_bytes`]: Self::to_be_bytes
[`to_le_bytes`]: Self::to_le_bytes

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
use exint::Wrapping;

if cfg!(target_endian = "big") {
    assert_eq!(Wrapping(0x123456_i24).to_ne_bytes(), [0x12, 0x34, 0x56]);
} else {
    assert_eq!(Wrapping(0x123456_i24).to_ne_bytes(), [0x56, 0x34, 0x12]);
}
# }
```
