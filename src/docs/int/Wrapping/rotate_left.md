Shifts the bits to the left by a specified amount, `n`,
wrapping the truncated bits to the end of the resulting integer.

Please note this isn't the same operation as the `<<` shifting operator!

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
use exint::Wrapping;

let n: Wrapping<i24> = Wrapping(0x120034_i24);
let m: Wrapping<i24> = Wrapping(0x341200_i24);

assert_eq!(n.rotate_left(16), m);
# }
```
