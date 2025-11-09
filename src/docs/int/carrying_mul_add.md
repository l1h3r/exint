Calculates the "full multiplication" `self * rhs + carry + add` without the possibility to overflow.

This returns the low-order (wrapping) bits and the high-order (overflow) bits of
the result as two separate values, in that order.

Performs "long multiplication" which takes in an extra amount to add, and may
return an additional amount of overflow. This allows for chaining together
multiple multiplications to create "big integers" which represent larger values.

If you don't need either `carry`, then you can use [`widening_mul`] instead,
and if you only need one `carry`, then you can use [`carrying_mul`] instead.

[`widening_mul`]: Self::widening_mul
[`carrying_mul`]: Self::carrying_mul

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(5_i32.carrying_mul_add(-2_i32, 0_i32, 0_i32), (4294967286_u32, -1_i32));
assert_eq!(5_i32.carrying_mul_add(-2_i32, 10_i32, 10_i32), (10_u32, 0_i32));
assert_eq!(1000000000_i32.carrying_mul_add(-10_i32, 0_i32, 0_i32), (2884901888_u32, -3_i32));
assert_eq!(1000000000_i32.carrying_mul_add(-10_i32, 10_i32, 10_i32), (2884901908_u32, -3_i32));
assert_eq!(i32::MAX.carrying_mul_add(i32::MAX, i32::MAX, i32::MAX), (u32::MAX, i32::MAX / 2_i32));
# }
```
