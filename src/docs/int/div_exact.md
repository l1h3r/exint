Integer division without remainder. Computes `self / rhs`,
returning `None` if `self % rhs != 0`.

# Panics

This function will panic  if `rhs == 0`.

## Overflow behavior

On overflow, this function will panic if overflow checks are enabled (default in
debug mode) and wrap if overflow checks are disabled (default in release mode).

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(64_i24.div_exact(2_i24), Some(32_i24));
assert_eq!(64_i24.div_exact(32_i24), Some(2_i24));
assert_eq!((i24::MIN + 1_i24).div_exact(-1_i24), Some(i24::MAX));
assert_eq!(65_i24.div_exact(2_i24), None);
# }
```

The following panics because of overflow:

```should_panic
# use ::exint::primitive::*;
# ::exint::uint! {
let _ = i24::MIN.div_exact(-1_i24);
# }
```

This will panic:

```should_panic
# use ::exint::primitive::*;
# ::exint::uint! {
let _ = 64_i24.div_exact(0_i24);
# }
```
