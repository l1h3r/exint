Wrapping Euclidean division. Computes `self.div_euclid(rhs)`,
wrapping around at the boundary of the type.

Wrapping will only occur in `MIN / -1` on a signed type (where `MIN` is the
negative minimal value for the type). This is equivalent to `-MIN`, a positive
value that is too large to represent in the type. In this case, this method
returns `MIN` itself.

# Panics

This function will panic if `rhs` is zero.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
assert_eq!(int!(100).wrapping_div_euclid(int!(10)), int!(10));
assert_eq!(int!(-128 i8).wrapping_div_euclid(int!(-1 i8)), int!(-128 i8));
```
