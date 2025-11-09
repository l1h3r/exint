Returns the memory representation of this integer as a byte array in
big-endian (network) byte order.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(0x123456_i24.to_be_bytes(), [0x12, 0x34, 0x56]);
# }
```
