Returns the number of trailing zeros in the binary representation of `self`.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!(uint!(0b00101000).trailing_zeros(), 3);
assert_eq!(uint!(0).trailing_zeros(), 32);
assert_eq!(uint::MAX.trailing_zeros(), 0);
```
