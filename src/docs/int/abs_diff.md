Computes the absolute difference between `self` and `rhs`.

This function always returns the correct answer without overflow or panics by
returning an unsigned integer.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(100_i24.abs_diff(80_i24), 20_u24);
assert_eq!(100_i24.abs_diff(110_i24), 10_u24);
assert_eq!((-100_i24).abs_diff(80_i24), 180_u24);
assert_eq!((-100_i24).abs_diff(-120_i24), 20_u24);
assert_eq!(i24::MIN.abs_diff(i24::MAX), u24::MAX);
# }
```
