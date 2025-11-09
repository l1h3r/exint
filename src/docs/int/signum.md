Returns a number representing sign of `self`.

- `0` if the number is zero
- `1` if the number is positive
- `-1` if the number is negative

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(10_i24.signum(), 1_i24);
assert_eq!(0_i24.signum(), 0_i24);
assert_eq!((-10_i24).signum(), -1_i24);
# }
```
