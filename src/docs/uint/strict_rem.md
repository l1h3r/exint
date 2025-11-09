Strict integer remainder. Computes `self % rhs`.

Strict remainder calculation on unsigned types is just the regular remainder
calculation. There's no way overflow could ever happen. This function exists so
that all operations are accounted for in the strict operations.

# Panics

This function will panic if `rhs` is zero.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(100_u24.strict_rem(10_u24), 0_u24);
# }
```

The following panics because of division by zero:

```should_panic
# use ::exint::primitive::*;
# ::exint::uint! {
let _ = 5_u24.strict_rem(0_u24);
# }
```
