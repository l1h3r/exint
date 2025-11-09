Computes the absolute value of `self` without any wrapping or panicking.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(100_i24.unsigned_abs(), 100_u24);
assert_eq!((-100_i24).unsigned_abs(), 100_u24);
assert_eq!((-128_i24).unsigned_abs(), 128_u24);
# }
```
