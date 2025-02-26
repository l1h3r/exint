Returns the smallest power of two greater than or equal to `n`. If the next
power of two is greater than the type's maximum value, the return value is
wrapped to `0`.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint_integer as exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!(uint!(2).wrapping_next_power_of_two(), uint!(2));
assert_eq!(uint!(3).wrapping_next_power_of_two(), uint!(4));
assert_eq!(uint::MAX.wrapping_next_power_of_two(), uint!(0));
```
