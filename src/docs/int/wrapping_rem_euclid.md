Wrapping Euclidean remainder. Computes `self.rem_euclid(rhs)`,
wrapping around at the boundary of the type.

Wrapping will only occur in `MIN % -1` on a signed type (where `MIN` is the
negative minimal value for the type). In this case, this method returns 0.

# Panics

This function will panic if `rhs` is zero.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
assert_eq!(int!(100).wrapping_rem_euclid(int!(10)), int!(0));
assert_eq!((-128i8).wrapping_rem_euclid(-1), 0);
```
