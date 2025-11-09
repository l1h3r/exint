Calculates the multiplication of `self` and `rhs`.

Returns a tuple of the multiplication along with a boolean indicating whether an
arithmetic overflow would occur. If an overflow would have occurred then the
wrapped value is returned.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(5_i24.overflowing_mul(2_i24), (10_i24, false));
assert_eq!(1000000000_i32.overflowing_mul(10_i32), (1410065408_i32, true));
# }
```
