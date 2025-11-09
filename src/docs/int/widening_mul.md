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
assert_eq!(5_i32.widening_mul(-2_i32), (4294967286_u32, -1_i32));
assert_eq!(1000000000_i32.widening_mul(-10_i32), (2884901888_u32, -3_i32));
# }
```
