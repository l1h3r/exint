Saturating integer division. Computes `self / rhs`,
saturating at the numeric bounds instead of overflowing.

# Panics

This function will panic if `rhs` is zero.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
assert_eq!(uint!(5).saturating_div(uint!(2)), uint!(2));
```
