Reverses the byte order of the integer.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
use exint::Wrapping;

let n = Wrapping(0x123456_i24);
let m = n.swap_bytes();

assert_eq!(m, Wrapping(0x563412_i24));
# }
```
