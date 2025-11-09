Strict absolute value. Computes `self.abs()`, panicking if `self == MIN`.

# Panics

## Overflow behavior

This function will always panic on overflow, regardless of whether overflow checks are enabled.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!((-5_i24).strict_abs(), 5_i24);
# }
```

The following panics because of overflow:

```should_panic
# use ::exint::primitive::*;
# ::exint::uint! {
let _ = i24::MIN.strict_abs();
# }
```
