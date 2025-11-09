Returns the bit pattern of `self` reinterpreted as an unsigned integer of the same size.

This produces the same result as an `as` cast, but ensures that the bit-width
remains the same.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(-1_i24.cast_unsigned(), u24::MAX);
# }
```
