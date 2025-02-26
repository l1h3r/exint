Returns the bit pattern of `self` reinterpreted as an unsigned integer of the same size.

This produces the same result as an `as` cast, but ensures that the bit-width
remains the same.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint_integer as exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!(int!(-1).cast_unsigned(), uint::MAX);
```
