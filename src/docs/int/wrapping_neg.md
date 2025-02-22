Wrapping (modular) negation. Computes `-self`,
wrapping around at the boundary of the type.

The only case where such wrapping can occur is when one negates `MIN` on a
signed type (where `MIN` is the negative minimal value for the type); this is a
positive value that is too large to represent in the type. In such a case, this
function returns `MIN` itself.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
assert_eq!(int!(100).wrapping_neg(), int!(-100));
assert_eq!(int!(-100).wrapping_neg(), int!(100));
assert_eq!(int::MIN.wrapping_neg(), int::MIN);
```
