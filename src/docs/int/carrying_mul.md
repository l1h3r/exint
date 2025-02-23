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
assert_eq!(int!(5).carrying_mul(int!(-2), int!(0)), (uint!(4294967286), int!(-1)));
assert_eq!(int!(5).carrying_mul(int!(-2), int!(10)), (uint!(0), int!(0)));
assert_eq!(int!(1000000000).carrying_mul(int!(-10), int!(0)), (uint!(2884901888), int!(-3)));
assert_eq!(int!(1000000000).carrying_mul(int!(-10), int!(10)), (uint!(2884901898), int!(-3)));
assert_eq!(int::MAX.carrying_mul(int::MAX, int::MAX), (int::MAX.unsigned_abs() + uint!(1), int::MAX / int!(2)));
```
