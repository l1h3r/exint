Calculates the quotient of Euclidean division `self.div_euclid(rhs)`.

Returns a tuple of the divisor along with a boolean indicating whether an
arithmetic overflow would occur. If an overflow would occur then `self` is
returned.

# Panics

This function will panic if `rhs` is zero.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!(int!(5).overflowing_div_euclid(int!(2)), (int!(2), false));
assert_eq!(int::MIN.overflowing_div_euclid(int!(-1)), (int::MIN, true));
```
