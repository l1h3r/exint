Wrapping (modular) negation. Computes `-self`,
wrapping around at the boundary of the type.

Since unsigned types do not have negative equivalents all applications of this
function will wrap (except for `-0`). For values smaller than the corresponding
signed type's maximum the result is the same as casting the corresponding signed
value. Any larger values are equivalent to `MAX + 1 - (val - MAX - 1)` where
`MAX` is the corresponding signed type's maximum.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
assert_eq!(uint!(0).wrapping_neg(), uint!(0));
assert_eq!(uint::MAX.wrapping_neg(), uint!(1));
assert_eq!(uint!(13).wrapping_neg(), (!uint!(13)) + uint!(1));
assert_eq!(uint!(42).wrapping_neg(), !(uint!(42) - uint!(1)));
```
