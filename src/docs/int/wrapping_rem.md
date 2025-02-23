Wrapping (modular) remainder. Computes `self % rhs`,
wrapping around at the boundary of the type.

Such wrap-around never actually occurs mathematically; implementation artifacts
make `x % y` invalid for `MIN / -1` on a signed type (where `MIN` is the
negative minimal value). In such a case, this function returns `0`.

# Panics

This function will panic if `rhs` is zero.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!(int!(100).wrapping_rem(int!(10)), int!(0));
assert_eq!(int!(-128 i8).wrapping_rem(int!(-1 i8)), int!(0 i8));
```
