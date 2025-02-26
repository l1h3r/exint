Returns the number of leading ones in the binary representation of `self`.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint_integer as exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!((!(uint::MAX >> 2u32)).leading_ones(), 2);
assert_eq!(uint!(0).leading_ones(), 0);
assert_eq!(uint::MAX.leading_ones(), 32);
```
