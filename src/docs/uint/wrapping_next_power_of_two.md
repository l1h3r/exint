Returns the smallest power of two greater than or equal to `n`. If the next
power of two is greater than the type's maximum value, the return value is
wrapped to `0`.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(2_u24.wrapping_next_power_of_two(), 2_u24);
assert_eq!(3_u24.wrapping_next_power_of_two(), 4_u24);
assert_eq!(u24::MAX.wrapping_next_power_of_two(), 0_u24);
# }
```
