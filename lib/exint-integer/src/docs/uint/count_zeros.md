Returns the number of zeros in the binary representation of `self`.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint_integer as exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!(uint!(0).count_zeros(), 32);
assert_eq!(uint::MAX.count_zeros(), 0);
```
