Creates a native endian integer value from its representation as a byte array in big endian.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
use exint::Wrapping;

assert_eq!(Wrapping::<i24>::from_be_bytes([0x12, 0x34, 0x56]), Wrapping(0x123456_i24));
# }
```
