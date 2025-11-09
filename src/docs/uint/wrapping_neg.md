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
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(0_u24.wrapping_neg(), 0_u24);
assert_eq!(u24::MAX.wrapping_neg(), 1_u24);
assert_eq!(13_u24.wrapping_neg(), (!13_u24) + 1_u24);
assert_eq!(42_u24.wrapping_neg(), !(42_u24 - 1_u24));
# }
```
