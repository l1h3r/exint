Calculates the quotient of `self` and `rhs`, rounding the result towards
positive infinity.

# Panics

This function will panic if `rhs` is zero or if `self` is `Self::MIN` and
`rhs` is -1. This behavior is not affected by the `overflow-checks` flag.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
let a = int!(8);
let b = int!(3);

assert_eq!(a.div_ceil(b), int!(3));
assert_eq!(a.div_ceil(-b), int!(-2));
assert_eq!((-a).div_ceil(b), int!(-2));
assert_eq!((-a).div_ceil(-b), int!(3));
```
