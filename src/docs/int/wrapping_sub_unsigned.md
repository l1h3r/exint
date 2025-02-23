Wrapping (modular) subtraction with an unsigned integer. Computes `self - rhs`,
wrapping around at the boundary of the type.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!(int!(0).wrapping_sub_unsigned(uint!(127)), int!(-127));
assert_eq!(int!(-2).wrapping_sub_unsigned(uint::MAX), int!(-1));
```
