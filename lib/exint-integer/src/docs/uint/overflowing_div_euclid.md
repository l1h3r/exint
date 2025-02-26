Calculates the quotient of Euclidean division `self.div_euclid(rhs)`.

Returns a tuple of the divisor along with a boolean indicating whether an
arithmetic overflow would occur. Note that for unsigned integers overflow never
occurs, so the second value is always `false`. Since, for the positive integers,
all common definitions of division are equal, this is exactly equal to
`self.overflowing_div(rhs)`.

# Panics

This function will panic if `rhs` is zero.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint_integer as exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!(uint!(5).overflowing_div_euclid(uint!(2)), (uint!(2), false));
```
