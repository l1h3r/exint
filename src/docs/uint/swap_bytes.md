Reverses the byte order of the integer.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
let n = uint!(0x12345678);
let m = n.reverse_bits();

assert_eq!(m, uint!(0x1E6A2C48));
assert_eq!(uint!(0), uint!(0).reverse_bits());
```
