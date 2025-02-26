Wrapping (modular) multiplication. Computes `self * rhs`,
wrapping around at the boundary of the type.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint_integer as exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!(uint!(10).wrapping_mul(uint!(12)), uint!(120));
assert_eq!(uint!(25 u8).wrapping_mul(uint!(12 u8)), uint!(44 u8));
```
