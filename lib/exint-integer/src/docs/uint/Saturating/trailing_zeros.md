Returns the number of trailing zeros in the binary representation of `self`.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
use exint::Saturating;

assert_eq!(Saturating(uint!(0b00101000)).trailing_zeros(), 3);
```
