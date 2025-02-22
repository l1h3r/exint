Returns the base 10 logarithm of the number, rounded down.

# Panics

This function will panic if `self` is zero.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
assert_eq!(uint!(10).ilog10(), 1);
```
