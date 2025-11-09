Returns the memory representation of this integer as a byte array in
little-endian byte order.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
use exint::Saturating;

assert_eq!(Saturating(0x123456_i24).to_le_bytes(), [0x56, 0x34, 0x12]);
# }
```
