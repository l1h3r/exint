Shifts the bits to the left by a specified amount, `n`,
wrapping the truncated bits to the end of the resulting integer.

Please note this isn't the same operation as the `<<` shifting operator!

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint_integer as exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
use exint::Saturating;

let n: Saturating<int> = Saturating(int!(0x12003400));
let m: Saturating<int> = Saturating(int!(0x34001200));

assert_eq!(n.rotate_left(16), m);
```
