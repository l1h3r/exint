Wrapping Euclidean division. Computes `self.div_euclid(rhs)`.

Wrapped division on unsigned types is just normal division. There's no way
wrapping could ever happen. This function exists, so that all operations are
accounted for in the wrapping operations. Since, for the positive integers, all
common definitions of division are equal, this is exactly equal to
`self.wrapping_div(rhs)`.

# Panics

This function will panic if `rhs` is zero.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(100_u24.wrapping_div_euclid(10_u24), 10_u24);
# }
```
