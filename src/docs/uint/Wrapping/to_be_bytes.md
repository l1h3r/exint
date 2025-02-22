Returns the memory representation of this integer as a byte array in
big-endian (network) byte order.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
use exint::Wrapping;

assert_eq!(Wrapping(uint!(0x12345678)).to_be_bytes(), [0x12, 0x34, 0x56, 0x78]);
```
