The size of this integer type in bits.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint_integer as exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
use exint::Saturating;

assert_eq!(<Saturating<uint>>::BITS, uint::BITS);
```
