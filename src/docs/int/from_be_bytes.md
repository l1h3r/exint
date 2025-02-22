Creates a native endian integer value from its representation as a byte array in big endian.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
assert_eq!(int::from_be_bytes([0x12, 0x34, 0x56, 0x78]), int!(0x12345678));
```
