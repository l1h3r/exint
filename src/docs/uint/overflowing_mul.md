Calculates the multiplication of `self` and `rhs`.

Returns a tuple of the multiplication along with a boolean indicating whether an
arithmetic overflow would occur. If an overflow would have occurred then the
wrapped value is returned.

If you want the *value* of the overflow, rather than just *whether*
an overflow occurred, see [`carrying_mul`].

[`carrying_mul`]: Self::carrying_mul

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(5_u24.overflowing_mul(2_u24), (10_u24, false));
assert_eq!(1000000000_u32.overflowing_mul(10_u32), (1410065408_u32, true));
assert_eq!(u24::MAX.overflowing_mul(2_u24), (u24::MAX - 1_u24, true));
# }
```
