Strict Euclidean remainder. Computes `self.rem_euclid(rhs)`, panicking if the division results in overflow.

# Panics

This function will panic if `rhs` is zero.

## Overflow behavior

This function will always panic on overflow, regardless of whether overflow checks are enabled.

The only case where such an overflow can occur is `x % y` for `MIN / -1` on a
signed type (where `MIN` is the negative minimal value), which is invalid due to
implementation artifacts.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(5_i24.strict_rem_euclid(2_i24), 1_i24);
# }
```

The following panics because of overflow:

```should_panic
# use ::exint::primitive::*;
# ::exint::uint! {
let _ = i24::MIN.strict_rem_euclid(-1_i24);
# }
```

The following panics because of division by zero:

```should_panic
# use ::exint::primitive::*;
# ::exint::uint! {
let _ = 5_i24.strict_rem_euclid(0_i24);
# }
```
