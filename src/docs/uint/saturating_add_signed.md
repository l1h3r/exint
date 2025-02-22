Saturating addition with a signed integer. Computes `self + rhs`,
saturating at the numeric bounds instead of overflowing.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
assert_eq!(uint!(1).saturating_add_signed(int!(2)), uint!(3));
assert_eq!(uint!(1).saturating_add_signed(int!(-2)), uint!(0));
assert_eq!((uint::MAX - uint!(2)).saturating_add_signed(int!(4)), uint::MAX);
```
