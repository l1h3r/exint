Calculates `self` + `rhs`.

Returns a tuple of the addition along with a boolean indicating whether an
arithmetic overflow would occur. If an overflow would have occurred then the
wrapped value is returned.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(5_u24.overflowing_add(2_u24), (7_u24, false));
assert_eq!(u24::MAX.overflowing_add(1_u24), (0_u24, true));
# }
```
