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
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!(uint!(5).carrying_mul(uint!(2), uint!(0)), (uint!(10), uint!(0)));
assert_eq!(uint!(5).carrying_mul(uint!(2), uint!(10)), (uint!(20), uint!(0)));
assert_eq!(uint!(1000000000).carrying_mul(uint!(10), uint!(0)), (uint!(1410065408), uint!(2)));
assert_eq!(uint!(1000000000).carrying_mul(uint!(10), uint!(10)), (uint!(1410065418), uint!(2)));
assert_eq!(uint::MAX.carrying_mul(uint::MAX, uint::MAX), (uint!(0), uint::MAX));
```
