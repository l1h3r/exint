Saturating addition with an unsigned integer. Computes `self + rhs`,
saturating at the numeric bounds instead of overflowing.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(1_i24.saturating_add_unsigned(2_u24), 3_i24);
assert_eq!(i24::MAX.saturating_add_unsigned(100_u24), i24::MAX);
# }
```
