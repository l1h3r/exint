Wrapping (modular) multiplication. Computes `self * rhs`,
wrapping around at the boundary of the type.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
assert_eq!(uint!(10).wrapping_mul(uint!(12)), uint!(120));
assert_eq!(25u8.wrapping_mul(12), 44);
```
