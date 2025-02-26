Reverses the byte order of the integer.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint_integer as exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
use exint::Saturating;

let n = Saturating(int!(0x12345678));
let m = n.swap_bytes();

assert_eq!(m, Saturating(int!(0x78563412)));
```
