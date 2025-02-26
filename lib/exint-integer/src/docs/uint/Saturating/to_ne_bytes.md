Returns the memory representation of this integer as a byte array in native byte order.

As the target platform's native endianness is used, portable code should use
[`to_be_bytes`] or [`to_le_bytes`], as appropriate, instead.

[`to_be_bytes`]: Self::to_be_bytes
[`to_le_bytes`]: Self::to_le_bytes

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint_integer as exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
use exint::Saturating;

if cfg!(target_endian = "big") {
    assert_eq!(Saturating(uint!(0x12345678)).to_ne_bytes(), [0x12, 0x34, 0x56, 0x78]);
} else {
    assert_eq!(Saturating(uint!(0x12345678)).to_ne_bytes(), [0x78, 0x56, 0x34, 0x12]);
}
```
