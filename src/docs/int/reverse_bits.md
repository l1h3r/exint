Reverses the order of bits in the integer. The least significant bit becomes the
most significant bit, second least-significant bit becomes second
most-significant bit, etc.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
let n = 0x123456_i24;
let m = n.reverse_bits();

assert_eq!(m, 0x6A2C48_i24);
assert_eq!(0_i24, 0_i24.reverse_bits());
# }
```
