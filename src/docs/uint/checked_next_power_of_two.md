Returns the smallest power of two greater than or equal to `self`.

If the next power of two is greater than the type's maximum value,
`None` is returned, otherwise the power of two is wrapped in `Some`.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(2_u24.checked_next_power_of_two(), Some(2_u24));
assert_eq!(3_u24.checked_next_power_of_two(), Some(4_u24));
assert_eq!(u24::MAX.checked_next_power_of_two(), None);
# }
```
