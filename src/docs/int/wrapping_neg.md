Wrapping (modular) negation. Computes `-self`,
wrapping around at the boundary of the type.

The only case where such wrapping can occur is when one negates `MIN` on a
signed type (where `MIN` is the negative minimal value for the type); this is a
positive value that is too large to represent in the type. In such a case, this
function returns `MIN` itself.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(100_i24.wrapping_neg(), -100_i24);
assert_eq!((-100_i24).wrapping_neg(), 100_i24);
assert_eq!(i24::MIN.wrapping_neg(), i24::MIN);
# }
```
