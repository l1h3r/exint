Returns `true` if and only if `self == 2^k` for some unsigned integer `k`.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert!(16_u24.is_power_of_two());
assert!(!10_u24.is_power_of_two());
# }
```
