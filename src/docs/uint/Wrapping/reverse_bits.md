Reverses the order of bits in the integer. The least significant bit becomes the
most significant bit, second least-significant bit becomes second
most-significant bit, etc.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
use exint::Wrapping;

let n = Wrapping(0x123456_u24);
let m = n.reverse_bits();

assert_eq!(m, Wrapping(0x6A2C48_u24));
assert_eq!(Wrapping(0_u24), Wrapping(0_u24.reverse_bits()));
# }
```
