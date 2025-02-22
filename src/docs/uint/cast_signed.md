Returns the bit pattern of `self` reinterpreted as a signed integer of the same size.

This produces the same result as an `as` cast, but ensures that the bit-width
remains the same.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
assert_eq!(uint::MAX.cast_signed(), int!(-1));
```
