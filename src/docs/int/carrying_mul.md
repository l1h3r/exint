Calculates the "full multiplication" `self * rhs + carry` without the possibility to overflow.

This returns the low-order (wrapping) bits and the high-order (overflow) bits of
the result as two separate values, in that order.

Performs "long multiplication" which takes in an extra amount to add, and may
return an additional amount of overflow. This allows for chaining together
multiple multiplications to create "big integers" which represent larger values.

If you don't need the `carry`, then you can use [`widening_mul`] instead.

[`widening_mul`]: Self::widening_mul

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(5_i32.carrying_mul(-2_i32, 0_i32), (4294967286_u32, -1_i32));
assert_eq!(5_i32.carrying_mul(-2_i32, 10_i32), (0_u32, 0_i32));
assert_eq!(1000000000_i32.carrying_mul(-10_i32, 0_i32), (2884901888_u32, -3_i32));
assert_eq!(1000000000_i32.carrying_mul(-10_i32, 10_i32), (2884901898_u32, -3_i32));
assert_eq!(i32::MAX.carrying_mul(i32::MAX, i32::MAX), (i32::MAX.unsigned_abs() + 1_u32, i32::MAX / 2_i32));
# }
```
