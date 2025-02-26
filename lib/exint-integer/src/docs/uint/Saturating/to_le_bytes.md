Returns the memory representation of this integer as a byte array in
little-endian byte order.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint_integer as exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
use exint::Saturating;

assert_eq!(Saturating(uint!(0x12345678)).to_le_bytes(), [0x78, 0x56, 0x34, 0x12]);
```
