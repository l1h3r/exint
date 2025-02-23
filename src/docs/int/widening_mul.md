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
# #[macro_use] extern crate exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!(int!(5).widening_mul(int!(-2)), (uint!(4294967286), int!(-1)));
assert_eq!(int!(1000000000).widening_mul(int!(-10)), (uint!(2884901888), int!(-3)));
```
