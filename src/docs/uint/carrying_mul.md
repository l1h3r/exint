Calculates the "full multiplication" `self * rhs + carry` without the possibility to overflow.

This returns the low-order (wrapping) bits and the high-order (overflow) bits of
the result as two separate values, in that order.

Performs "long multiplication" which takes in an extra amount to add, and may
return an additional amount of overflow. This allows for chaining together
multiple multiplications to create "big integers" which represent larger values.

If you also need to add a value, then use [`carrying_mul_add`].

[`carrying_mul_add`]: Self::carrying_mul_add

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(5_u32.carrying_mul(2_u32, 0_u32), (10_u32, 0_u32));
assert_eq!(5_u32.carrying_mul(2_u32, 10_u32), (20_u32, 0_u32));
assert_eq!(1000000000_u32.carrying_mul(10_u32, 0_u32), (1410065408_u32, 2_u32));
assert_eq!(1000000000_u32.carrying_mul(10_u32, 10_u32), (1410065418_u32, 2_u32));
assert_eq!(u32::MAX.carrying_mul(u32::MAX, u32::MAX), (0_u32, u32::MAX));
# }
```
