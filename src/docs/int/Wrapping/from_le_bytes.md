Creates a native endian integer value from its representation as a byte array in little endian.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
use exint::Wrapping;

assert_eq!(<Wrapping<int>>::from_le_bytes([0x78, 0x56, 0x34, 0x12]), Wrapping(int!(0x12345678)));
```
