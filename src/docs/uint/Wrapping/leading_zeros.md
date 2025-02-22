

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
use exint::Wrapping;

assert_eq!(Wrapping(uint!(0)).leading_zeros(), 32);
```
