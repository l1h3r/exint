Strict subtraction with a signed integer. Computes `self - rhs`, panicking if overflow occurred.

# Panics

## Overflow behavior

This function will always panic on overflow, regardless of whether overflow checks are enabled.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(3_u24.strict_sub_signed(2_i24), 1_u24);
# }
```

The following panics because of overflow:

```should_panic
# use ::exint::primitive::*;
# ::exint::uint! {
let _ = 1_u24.strict_sub_signed(2_i24);
# }
```

```should_panic
# use ::exint::primitive::*;
# ::exint::uint! {
let _ = u24::MAX.strict_sub_signed(-1_i24);
# }
```
