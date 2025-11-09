Checked negation. Computes `-self`,
returning `None` unless `self == 0`.

Note that negating any positive integer will overflow.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(0_u24.checked_neg(), Some(0_u24));
assert_eq!(1_u24.checked_neg(), None);
# }
```
