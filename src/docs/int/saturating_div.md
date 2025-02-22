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
assert_eq!(int!(5).saturating_div(int!(2)), int!(2));
assert_eq!(int::MAX.saturating_div(int!(-1)), int::MIN + int!(1));
assert_eq!(int::MIN.saturating_div(int!(-1)), int::MAX);
```
