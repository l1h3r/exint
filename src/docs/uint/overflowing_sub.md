Calculates `self` - `rhs`.

Returns a tuple of the subtraction along with a boolean indicating whether an
arithmetic overflow would occur. If an overflow would have occurred then the
wrapped value is returned.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(5_u24.overflowing_sub(2_u24), (3_u24, false));
assert_eq!(0_u24.overflowing_sub(1_u24), (u24::MAX, true));
# }
```
