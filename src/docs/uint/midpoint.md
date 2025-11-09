Calculates the midpoint (average) between `self` and `rhs`.

`midpoint(a, b)` is `(a + b) >> 1` as if it were performed in a
sufficiently-large signed integral type. This implies that the result is always
rounded towards negative infinity and that no overflow will ever occur.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(0_u24.midpoint(4_u24), 2_u24);
assert_eq!(1_u24.midpoint(4_u24), 2_u24);
# }
```
