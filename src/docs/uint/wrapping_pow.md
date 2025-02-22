Wrapping (modular) exponentiation. Computes `self.pow(exp)`,
wrapping around at the boundary of the type.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
assert_eq!(uint!(3).wrapping_pow(5), uint!(243));
assert_eq!(uint!(3 u8).wrapping_pow(6), uint!(217 u8));
```
