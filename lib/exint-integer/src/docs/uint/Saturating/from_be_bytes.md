Creates a native endian integer value from its representation as a byte array in big endian.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint_integer as exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
use exint::Saturating;

assert_eq!(<Saturating<uint>>::from_be_bytes([0x12, 0x34, 0x56, 0x78]), Saturating(uint!(0x12345678)));
```
