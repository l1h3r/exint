Returns the number of ones in the binary representation of `self`.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
use exint::Saturating;

assert_eq!(Saturating(uint!(0b01001100)).count_ones(), 3);
```
