Returns the number of trailing ones in the binary representation of `self`.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
assert_eq!(uint!(0b01010111).trailing_ones(), 3);
assert_eq!(uint!(0).trailing_ones(), 0);
assert_eq!(uint::MAX.trailing_ones(), 32);
```
