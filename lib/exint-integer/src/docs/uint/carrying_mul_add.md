Calculates the "full multiplication" `self * rhs + carry1 + carry2` without the possibility to overflow.

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
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint_integer as exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!(uint!(5).carrying_mul_add(uint!(2), uint!(0), uint!(0)), (uint!(10), uint!(0)));
assert_eq!(uint!(5).carrying_mul_add(uint!(2), uint!(10), uint!(10)), (uint!(30), uint!(0)));
assert_eq!(uint!(1000000000).carrying_mul_add(uint!(10), uint!(0), uint!(0)), (uint!(1410065408), uint!(2)));
assert_eq!(uint!(1000000000).carrying_mul_add(uint!(10), uint!(10), uint!(10)), (uint!(1410065428), uint!(2)));
assert_eq!(uint::MAX.carrying_mul_add(uint::MAX, uint::MAX, uint::MAX), (uint::MAX, uint::MAX));
```
