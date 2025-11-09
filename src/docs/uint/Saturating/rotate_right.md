Shifts the bits to the right by a specified amount, `n`,
wrapping the truncated bits to the beginning of the resulting integer.

Please note this isn't the same operation as the `>>` shifting operator!

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
use exint::Saturating;

let n: Saturating<u24> = Saturating(0x341200_u24);
let m: Saturating<u24> = Saturating(0x120034_u24);

assert_eq!(n.rotate_right(16), m);
# }
```
