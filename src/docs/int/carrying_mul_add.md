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
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
assert_eq!(int!(5).carrying_mul_add(int!(-2), int!(0), int!(0)), (uint!(4294967286), int!(-1)));
assert_eq!(int!(5).carrying_mul_add(int!(-2), int!(10), int!(10)), (uint!(10), int!(0)));
assert_eq!(int!(1000000000).carrying_mul_add(int!(-10), int!(0), int!(0)), (uint!(2884901888), int!(-3)));
assert_eq!(int!(1000000000).carrying_mul_add(int!(-10), int!(10), int!(10)), (uint!(2884901908), int!(-3)));
assert_eq!(int::MAX.carrying_mul_add(int::MAX, int::MAX, int::MAX), (uint::MAX, int::MAX / int!(2)));
```
