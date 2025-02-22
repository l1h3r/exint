Wrapping (modular) addition. Computes `self + rhs`,
wrapping around at the boundary of the type.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
assert_eq!(uint!(200).wrapping_add(uint!(55)), uint!(255));
assert_eq!(uint!(200).wrapping_add(uint::MAX), uint!(199));
```
