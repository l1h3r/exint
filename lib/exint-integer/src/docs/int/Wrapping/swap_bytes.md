Reverses the byte order of the integer.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
use exint::Wrapping;

let n = Wrapping(int!(0x12345678));
let m = n.swap_bytes();

assert_eq!(m, Wrapping(int!(0x78563412)));
```
