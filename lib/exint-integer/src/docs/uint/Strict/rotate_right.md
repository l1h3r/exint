Shifts the bits to the right by a specified amount, `n`,
wrapping the truncated bits to the beginning of the resulting integer.

Please note this isn't the same operation as the `>>` shifting operator!

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint_integer as exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
use exint::Strict;

let n: Strict<uint> = Strict(uint!(0x34001200));
let m: Strict<uint> = Strict(uint!(0x12003400));

assert_eq!(n.rotate_right(16), m);
```
