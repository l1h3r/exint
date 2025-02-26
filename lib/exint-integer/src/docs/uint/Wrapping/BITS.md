The size of this integer type in bits.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
use exint::Wrapping;

assert_eq!(<Wrapping<uint>>::BITS, uint::BITS);
```
