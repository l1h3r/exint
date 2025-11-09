Checked Euclidean modulo. Computes `self.rem_euclid(rhs)`,
returning `None` if `rhs == 0` or the division results in overflow.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(5_i24.checked_rem_euclid(2_i24), Some(1_i24));
assert_eq!(5_i24.checked_rem_euclid(0_i24), None);
assert_eq!(i24::MIN.checked_rem_euclid(-1_i24), None);
# }
```
