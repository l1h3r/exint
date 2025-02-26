Returns the smallest power of two greater than or equal to `self`.

If the next power of two is greater than the type's maximum value,
`None` is returned, otherwise the power of two is wrapped in `Some`.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!(uint!(2).checked_next_power_of_two(), Some(uint!(2)));
assert_eq!(uint!(3).checked_next_power_of_two(), Some(uint!(4)));
assert_eq!(uint::MAX.checked_next_power_of_two(), None);
```
