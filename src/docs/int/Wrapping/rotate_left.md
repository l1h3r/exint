Shifts the bits to the left by a specified amount, `n`,
wrapping the truncated bits to the end of the resulting integer.

Please note this isn't the same operation as the `<<` shifting operator!

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
use exint::Wrapping;

let n: Wrapping<int> = Wrapping(int!(0x12003400));
let m: Wrapping<int> = Wrapping(int!(0x34001200));

assert_eq!(n.rotate_left(16), m);
```
