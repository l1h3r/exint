Returns the memory representation of this integer as a byte array in
big-endian (network) byte order.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
use exint::Saturating;

assert_eq!(Saturating(0x123456_u24).to_be_bytes(), [0x12, 0x34, 0x56]);
# }
```
