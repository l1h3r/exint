Calculates the "full multiplication" `self * rhs + carry + add`.

This returns the low-order (wrapping) bits and the high-order (overflow) bits of
the result as two separate values, in that order.

This cannot overflow, as the double-width result has exactly enough space for
the largest possible result. This is equivalent to how, in decimal,
9 × 9 + 9 + 9 = 81 + 18 = 99 = 9×10⁰ + 9×10¹ = 10² - 1.

Performs "long multiplication" which takes in an extra amount to add, and may
return an additional amount of overflow. This allows for chaining together
multiple multiplications to create "big integers" which represent larger values.

If you don't need the `add` part, then you can use [`carrying_mul`] instead.

[`carrying_mul`]: Self::carrying_mul

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(5_u32.carrying_mul_add(2_u32, 0_u32, 0_u32), (10_u32, 0_u32));
assert_eq!(5_u32.carrying_mul_add(2_u32, 10_u32, 10_u32), (30_u32, 0_u32));
assert_eq!(1000000000_u32.carrying_mul_add(10_u32, 0_u32, 0_u32), (1410065408_u32, 2_u32));
assert_eq!(1000000000_u32.carrying_mul_add(10_u32, 10_u32, 10_u32), (1410065428_u32, 2_u32));
assert_eq!(u32::MAX.carrying_mul_add(u32::MAX, u32::MAX, u32::MAX), (u32::MAX, u32::MAX));
# }
```
