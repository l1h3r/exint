Reverses the byte order of the integer.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint_integer as exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
use exint::Strict;

let n = Strict(uint!(0x12345678));
let m = n.swap_bytes();

assert_eq!(m, Strict(uint!(0x78563412)));
```
