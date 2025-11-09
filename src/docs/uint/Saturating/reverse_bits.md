Reverses the order of bits in the integer. The least significant bit becomes the
most significant bit, second least-significant bit becomes second
most-significant bit, etc.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
use exint::Saturating;

let n = Saturating(0x123456_u24);
let m = n.reverse_bits();

assert_eq!(m, Saturating(0x6A2C48_u24));
assert_eq!(Saturating(0_u24), Saturating(0_u24.reverse_bits()));
# }
```
