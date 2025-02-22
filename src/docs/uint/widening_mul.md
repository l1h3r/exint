Calculates the complete product `self * rhs` without the possibility to overflow.

This returns the low-order (wrapping) bits and the high-order (overflow) bits of
the result as two separate values, in that order.

If you also need to add a carry to the wide result, then you want
[`carrying_mul`] instead.

[`carrying_mul`]: Self::carrying_mul

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
assert_eq!(uint!(5).widening_mul(uint!(2)), (uint!(10), uint!(0)));
assert_eq!(uint!(1000000000).widening_mul(uint!(10)), (uint!(1410065408), uint!(2)));
```
