Calculates the quotient of `self` and `rhs`, rounding the result towards negative infinity.

This is the same as performing `self / rhs` for all unsigned integers.

# Panics

This function will panic if `rhs` is zero.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
assert_eq!(uint!(7).div_floor(uint!(4)), uint!(1));
```
