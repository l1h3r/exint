Checked integer division without remainder. Computes `self / rhs`.

# Panics

This function will panic  if `rhs == 0` or `self % rhs != 0`.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(64_u24.exact_div(2_u24), 32_u24);
assert_eq!(64_u24.exact_div(32_u24), 2_u24);
# }
```

This will panic:

```should_panic
# use ::exint::primitive::*;
# ::exint::uint! {
let _ = 65_u24.exact_div(2_u24);
# }
```
