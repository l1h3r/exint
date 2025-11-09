Strict shift left. Computes `self << rhs`,
panicking if `rhs` is larger than or equal to the number of bits in `self`.

# Panics

## Overflow behavior

This function will always panic on overflow, regardless of whether overflow checks are enabled.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(0x1_u24.strict_shl(4), 0x10_u24);
# }
```

The following panics because of overflow:

```should_panic
# use ::exint::primitive::*;
# ::exint::uint! {
let _ = 0x10_u24.strict_shl(129);
# }
```
