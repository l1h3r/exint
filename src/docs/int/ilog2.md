Returns the base 2 logarithm of the number, rounded down.

# Panics

This function will panic if `self` is less than or equal to zero.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!(int!(2).ilog2(), 1);
```
