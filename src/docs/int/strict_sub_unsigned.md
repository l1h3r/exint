Strict subtraction with an unsigned integer. Computes `self - rhs`, panicking if overflow occurred.

# Panics

## Overflow behavior

This function will always panic on overflow, regardless of whether overflow checks are enabled.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(1_i24.strict_sub_unsigned(2_u24), -1_i24);
# }
```

The following panics because of overflow:

```should_panic
# use ::exint::primitive::*;
# ::exint::uint! {
let _ = (i24::MIN + 2_i24).strict_sub_unsigned(3_u24);
# }
```
