Wrapping (modular) remainder. Computes `self % rhs`.

Wrapped remainder calculation on unsigned types is just the regular remainder
calculation. There's no way wrapping could ever happen. This function exists, so
that all operations are accounted for in the wrapping operations.

# Panics

This function will panic if `rhs` is zero.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
assert_eq!(uint!(100).wrapping_rem(uint!(10)), uint!(0));
```
