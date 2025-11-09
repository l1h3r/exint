Strict integer addition. Computes `self + rhs`, panicking if overflow occurred.

# Panics

## Overflow behavior

This function will always panic on overflow, regardless of whether overflow checks are enabled.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!((u24::MAX - 2_u24).strict_add(1_u24), u24::MAX - 1_u24);
# }
```

The following panics because of overflow:

```should_panic
# use ::exint::primitive::*;
# ::exint::uint! {
let _ = (u24::MAX - 2_u24).strict_add(3_u24);
# }
```
