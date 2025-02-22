Returns the square root of the number, rounded down.

# Panics

This function will panic if `self` is negative.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
assert_eq!(int!(10).isqrt(), int!(3));
```
