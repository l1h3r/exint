Strict Euclidean division. Computes `self.div_euclid(rhs)`.

Strict division on unsigned types is just normal division. There's no way
overflow could ever happen. This function exists so that all operations are
accounted for in the strict operations. Since, for the positive integers, all
common definitions of division are equal, this is exactly equal to
`self.strict_div(rhs)`.

# Panics

This function will panic if `rhs` is zero.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(100_u24.strict_div_euclid(10_u24), 10_u24);
# }
```

The following panics because of division by zero:

```should_panic
# use ::exint::primitive::*;
# ::exint::uint! {
let _ = 1_u24.strict_div_euclid(0_u24);
# }
```
