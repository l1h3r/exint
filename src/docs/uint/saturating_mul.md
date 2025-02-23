Saturating integer multiplication. Computes `self * rhs`,
saturating at the numeric bounds instead of overflowing.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!(uint!(2).saturating_mul(uint!(10)), uint!(20));
assert_eq!((uint::MAX).saturating_mul(uint!(10)), uint::MAX);
```
