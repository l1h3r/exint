Returns the smallest power of two greater than or equal to `self`.

When return value overflows (i.e., `self > (1 << (N-1))` for type `uN`), it
panics in debug mode and the return value is wrapped to 0 in release mode (the
only situation in which this method can return 0).

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
assert_eq!(uint!(2).next_power_of_two(), uint!(2));
assert_eq!(uint!(3).next_power_of_two(), uint!(4));
assert_eq!(uint!(0).next_power_of_two(), uint!(1));
```
