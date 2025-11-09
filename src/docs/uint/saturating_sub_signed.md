Saturating integer subtraction. Computes `self` - `rhs`,
saturating at the numeric bounds instead of overflowing.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(1_u24.saturating_sub_signed(2_i24), 0_u24);
assert_eq!(1_u24.saturating_sub_signed(-2_i24), 3_u24);
assert_eq!((u24::MAX - 2_u24).saturating_sub_signed(-4_i24), u24::MAX);
# }
```
