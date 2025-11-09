Calculates `self` - `rhs` with a signed `rhs`.

Returns a tuple of the subtraction along with a boolean indicating whether an
arithmetic overflow would occur. If an overflow would have occurred then the
wrapped value is returned.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(1_u24.overflowing_sub_signed(2_i24), (u24::MAX, true));
assert_eq!(1_u24.overflowing_sub_signed(-2_i24), (3_u24, false));
assert_eq!((u24::MAX - 2_u24).overflowing_sub_signed(-4_i24), (1_u24, true));
# }
```
