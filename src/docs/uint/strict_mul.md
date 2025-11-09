Strict integer multiplication. Computes `self * rhs`, panicking if overflow occurred.

# Panics

## Overflow behavior

This function will always panic on overflow, regardless of whether overflow checks are enabled.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(5_u24.strict_mul(1_u24), 5_u24);
# }
```

The following panics because of overflow:

```should_panic
# use ::exint::primitive::*;
# ::exint::uint! {
let _ = u24::MAX.strict_mul(2_u24);
# }
```
