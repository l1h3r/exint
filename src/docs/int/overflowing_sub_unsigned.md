Calculates `self` - `rhs` with an unsigned `rhs`.

Returns a tuple of the subtraction along with a boolean indicating whether an
arithmetic overflow would occur. If an overflow would have occurred then the
wrapped value is returned.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(1_i24.overflowing_sub_unsigned(2_u24), (-1_i24, false));
assert_eq!(i24::MAX.overflowing_sub_unsigned(u24::MAX), (i24::MIN, false));
assert_eq!((i24::MIN + 2_i24).overflowing_sub_unsigned(3_u24), (i24::MAX, true));
# }
```
