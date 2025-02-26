Calculates the remainder when `self` is divided by `rhs`.

Returns a tuple of the remainder after dividing along with a boolean indicating
whether an arithmetic overflow would occur. If an overflow would occur then 0 is
returned.

# Panics

This function will panic if `rhs` is zero.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint_integer as exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!(int!(5).overflowing_rem(int!(2)), (int!(1), false));
assert_eq!(int::MIN.overflowing_rem(int!(-1)), (int!(0), true));
```
