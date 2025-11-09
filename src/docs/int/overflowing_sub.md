Calculates `self` - `rhs`.

Returns a tuple of the subtraction along with a boolean indicating whether an
arithmetic overflow would occur. If an overflow would have occurred then the
wrapped value is returned.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(5_i24.overflowing_sub(2_i24), (3_i24, false));
assert_eq!(i24::MIN.overflowing_sub(1_i24), (i24::MAX, true));
# }
```
