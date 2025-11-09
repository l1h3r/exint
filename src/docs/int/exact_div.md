Checked integer division without remainder. Computes `self / rhs`.

# Panics

This function will panic  if `rhs == 0`, the division results in overflow,
or `self % rhs != 0`.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(64_i24.exact_div(2_i24), 32_i24);
assert_eq!(64_i24.exact_div(32_i24), 2_i24);
assert_eq!((i24::MIN + 1_i24).exact_div(-1_i24), i24::MAX);
# }
```

The following panics because of overflow:

```should_panic
# use ::exint::primitive::*;
# ::exint::uint! {
let _ = i24::MIN.exact_div(-1_i24);
# }
```

This will panic:

```should_panic
# use ::exint::primitive::*;
# ::exint::uint! {
let _ = 65_i24.exact_div(2_i24);
# }
```
