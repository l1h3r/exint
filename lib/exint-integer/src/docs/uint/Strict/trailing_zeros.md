Returns the number of trailing zeros in the binary representation of `self`.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint_integer as exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
use exint::Strict;

assert_eq!(Strict(uint!(0b00101000)).trailing_zeros(), 3);
```
