Returns the logarithm of the number with respect to an arbitrary base, rounded down.

This method might not be optimized owing to implementation details; `ilog2` can
produce results more efficiently for base 2, and `ilog10` can produce results
more efficiently for base 10.

# Panics

This function will panic if `self` is less than or equal to zero, or if `base`
is less than 2.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint_integer as exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!(int!(5).ilog(int!(5)), 1);
```
