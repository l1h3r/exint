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
use exint::Strict;

let n = Strict(uint!(0x12345678));
let m = n.reverse_bits();

assert_eq!(m, Strict(uint!(0x1E6A2C48)));
assert_eq!(Strict(uint!(0)), Strict(uint!(0).reverse_bits()));
```
