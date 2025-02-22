Reverses the byte order of the integer.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
let n = int!(0x12345678);
let m = n.reverse_bits();

assert_eq!(m, int!(0x1E6A2C48));
assert_eq!(int!(0), int!(0).reverse_bits());
```
