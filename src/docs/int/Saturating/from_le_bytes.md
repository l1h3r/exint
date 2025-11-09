Creates a native endian integer value from its representation as a byte array in little endian.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
use exint::Saturating;

assert_eq!(Saturating::<i24>::from_le_bytes([0x56, 0x34, 0x12]), Saturating(0x123456_i24));
# }
```
