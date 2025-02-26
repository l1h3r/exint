Reverses the order of bits in the integer. The least significant bit becomes the
most significant bit, second least-significant bit becomes second
most-significant bit, etc.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint_integer as exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
use exint::Saturating;

let n = Saturating(uint!(0x12345678));
let m = n.reverse_bits();

assert_eq!(m, Saturating(uint!(0x1E6A2C48)));
assert_eq!(Saturating(uint!(0)), Saturating(uint!(0).reverse_bits()));
```
