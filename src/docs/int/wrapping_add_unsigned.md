Wrapping (modular) addition with an unsigned integer. Computes `self + rhs`,
wrapping around at the boundary of the type.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
assert_eq!(int!(100).wrapping_add_unsigned(uint!(27)), int!(127));
assert_eq!(int::MAX.wrapping_add_unsigned(uint!(2)), int::MIN + int!(1));
```
