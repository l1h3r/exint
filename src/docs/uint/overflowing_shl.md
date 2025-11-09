Shifts self left by `rhs` bits.

Returns a tuple of the shifted version of self along with a boolean indicating
whether the shift value was larger than or equal to the number of bits. If the
shift value is too large, then value is masked (N-1) where N is the number of
bits, and this value is then used to perform the shift.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(0x1_u24.overflowing_shl(4), (0x10_u24, false));
assert_eq!(0x1_u24.overflowing_shl(100), (0x10_u24, true));
assert_eq!(0x10_u24.overflowing_shl(u24::BITS - 1), (0_u24, false));
# }
```
