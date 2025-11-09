Shifts the bits to the left by a specified amount, `n`,
wrapping the truncated bits to the end of the resulting integer.

Please note this isn't the same operation as the `<<` shifting operator!

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
let n = 0x120034_u24;
let m = 0x341200_u24;

assert_eq!(n.rotate_left(16), m);
# }
```
