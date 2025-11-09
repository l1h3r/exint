Calculates `self` + `rhs` with an unsigned `rhs`.

Returns a tuple of the addition along with a boolean indicating whether an
arithmetic overflow would occur. If an overflow would have occurred then the
wrapped value is returned.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(1_i24.overflowing_add_unsigned(2_u24), (3_i24, false));
assert_eq!(i24::MIN.overflowing_add_unsigned(u24::MAX), (i24::MAX, false));
assert_eq!((i24::MAX - 2_i24).overflowing_add_unsigned(3_u24), (i24::MIN, true));
# }
```
