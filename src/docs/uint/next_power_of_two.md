Returns the smallest power of two greater than or equal to `self`.

When return value overflows (i.e., `self > (1 << (N-1))` for type `uN`), it
panics in debug mode and the return value is wrapped to 0 in release mode (the
only situation in which this method can return 0).

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(2_u24.next_power_of_two(), 2_u24);
assert_eq!(3_u24.next_power_of_two(), 4_u24);
assert_eq!(0_u24.next_power_of_two(), 1_u24);
# }
```
