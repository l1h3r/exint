The largest value that can be represented by this integer type.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint_integer as exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
use exint::Wrapping;

assert_eq!(<Wrapping<uint>>::MAX, Wrapping(uint::MAX));
```
