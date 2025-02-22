Reverses the byte order of the integer.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
use exint::Strict;

let n = Strict(int!(0x12345678));
let m = n.swap_bytes();

assert_eq!(m, Strict(int!(0x78563412)));
```
