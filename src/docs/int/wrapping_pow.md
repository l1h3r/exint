Wrapping (modular) exponentiation. Computes `self.pow(exp)`,
wrapping around at the boundary of the type.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
assert_eq!(int!(3).wrapping_pow(4), int!(81));
assert_eq!(int!(3 i8).wrapping_pow(5), int!(-13 i8));
assert_eq!(int!(3 i8).wrapping_pow(6), int!(-39 i8));
```
