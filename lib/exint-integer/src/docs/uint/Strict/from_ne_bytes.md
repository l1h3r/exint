Creates a native endian integer value from its memory representation as a byte
array in native endianness.

As the target platform's native endianness is used, portable code likely wants
to use [`from_be_bytes`] or [`from_le_bytes`], as appropriate instead.

[`from_be_bytes`]: Self::from_be_bytes
[`from_le_bytes`]: Self::from_le_bytes

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint_integer as exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
use exint::Strict;

if cfg!(target_endian = "big") {
    assert_eq!(<Strict<uint>>::from_ne_bytes([0x12, 0x34, 0x56, 0x78]), Strict(uint!(0x12345678)));
} else {
    assert_eq!(<Strict<uint>>::from_ne_bytes([0x78, 0x56, 0x34, 0x12]), Strict(uint!(0x12345678)));
}
```
