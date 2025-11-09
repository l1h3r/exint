Strict integer subtraction. Computes `self - rhs`, panicking if overflow occurred.

# Panics

## Overflow behavior

This function will always panic on overflow, regardless of whether overflow checks are enabled.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(1_u24.strict_sub(1_u24), 0_u24);
# }
```

The following panics because of overflow:

```should_panic
# use ::exint::primitive::*;
# ::exint::uint! {
let _ = 0_u24.strict_sub(1_u24);
# }
```
