Returns the number of ones in the binary representation of `self`.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint_integer as exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
use exint::Strict;

assert_eq!(Strict(uint!(0b01001100)).count_ones(), 3);
```
