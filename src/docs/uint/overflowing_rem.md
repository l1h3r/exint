Calculates the remainder when `self` is divided by `rhs`.

Returns a tuple of the remainder after dividing along with a boolean indicating
whether an arithmetic overflow would occur. Note that for unsigned integers
overflow never occurs, so the second value is always `false`.

# Panics

This function will panic if `rhs` is zero.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!(uint!(5).overflowing_rem(uint!(2)), (uint!(1), false));
```
