Strict Euclidean division. Computes `self.div_euclid(rhs)`, panicking if overflow occurred.

# Panics

This function will panic if `rhs` is zero.

## Overflow behavior

This function will always panic on overflow, regardless of whether overflow checks are enabled.

The only case where such an overflow can occur is when one divides `MIN / -1` on
a signed type (where `MIN` is the negative minimal value for the type); this is
equivalent to `-MIN`, a positive value that is too large to represent in the type.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!((i24::MIN + 1_i24).strict_div_euclid(-1_i24), i24::MAX);
# }
```

The following panics because of overflow:

```should_panic
# use ::exint::primitive::*;
# ::exint::uint! {
let _ = i24::MIN.strict_div_euclid(-1_i24);
# }
```

The following panics because of division by zero:

```should_panic
# use ::exint::primitive::*;
# ::exint::uint! {
let _ = 1_i24.strict_div_euclid(0_i24);
# }
```
