Calculates the complete product `self * rhs` without the possibility to overflow.

This returns the low-order (wrapping) bits and the high-order (overflow) bits of
the result as two separate values, in that order.

If you also need to add a carry to the wide result, then you want
[`carrying_mul`] instead.

[`carrying_mul`]: Self::carrying_mul

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(5_u32.widening_mul(2_u32), (10_u32, 0_u32));
assert_eq!(1000000000_u32.widening_mul(10_u32), (1410065408_u32, 2_u32));
# }
```
