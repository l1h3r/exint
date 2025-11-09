Calculates the least nonnegative remainder of `self (mod rhs)`.

This is done as if by the Euclidean division algorithm -- given
`r = self.rem_euclid(rhs)`, the result satisfies
`self = rhs * self.div_euclid(rhs) + r` and `0 <= r < abs(rhs)`.

# Panics

This function will panic if `rhs` is zero or if `self` is `Self::MIN` and `rhs`
is -1. This behavior is not affected by the `overflow-checks` flag.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
let a = 7_i24;
let b = 4_i24;

assert_eq!(a.rem_euclid(b), 3_i24);
assert_eq!((-a).rem_euclid(b), 1_i24);
assert_eq!(a.rem_euclid(-b), 3_i24);
assert_eq!((-a).rem_euclid(-b), 1_i24);
# }
```

This will panic:

```should_panic
# use ::exint::primitive::*;
# ::exint::uint! {
let _ = i24::MIN.rem_euclid(-1_i24);
# }
```
