Wrapping (modular) subtraction with a signed integer. Computes `self - rhs`,
wrapping around at the boundary of the type.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!(uint!(1).wrapping_sub_signed(int!(2)), uint::MAX);
assert_eq!(uint!(1).wrapping_sub_signed(int!(-2)), uint!(3));
assert_eq!((uint::MAX - uint!(2)).wrapping_sub_signed(int!(-4)), uint!(1));
```
