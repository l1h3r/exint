Strict negation. Computes `-self`, panicking unless `self == 0`.

Note that negating any positive integer will overflow.

# Panics

## Overflow behavior

This function will always panic on overflow, regardless of whether overflow checks are enabled.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(0_u24.strict_neg(), 0_u24);
# }
```

The following panics because of overflow:

```should_panic
# use ::exint::primitive::*;
# ::exint::uint! {
let _ = 1_u24.strict_neg();
# }
```
