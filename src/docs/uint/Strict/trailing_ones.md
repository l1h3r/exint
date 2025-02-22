Returns the number of trailing ones in the binary representation of `self`.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
use exint::Strict;

assert_eq!(Strict(uint!(0b01010111)).trailing_ones(), 3);
```
