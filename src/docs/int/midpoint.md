Calculates the midpoint (average) between `self` and `rhs`.

`midpoint(a, b)` is `(a + b) >> 1` as if it were performed in a
sufficiently-large signed integral type. This implies that the result is always
rounded towards negative infinity and that no overflow will ever occur.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(0_i24.midpoint(4_i24), 2_i24);
assert_eq!(-1_i24.midpoint(2_i24), 0_i24);
assert_eq!(-7_i24.midpoint(0_i24), -3_i24);
assert_eq!(0_i24.midpoint(-7_i24), -3_i24);
assert_eq!(0_i24.midpoint(7_i24), 3_i24);
# }
```
