Checked Euclidean modulo. Computes `self.rem_euclid(rhs)`,
returning `None` if `rhs == 0`.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(5_u24.checked_rem_euclid(2_u24), Some(1_u24));
assert_eq!(5_u24.checked_rem_euclid(0_u24), None);
# }
```
